//! Integration tests for the valuations API.
//!
//! These tests require a running Sure API server and valid credentials.
//! Set SURE_BASE_URL and SURE_TOKEN environment variables in the .env file.
//!
//! Beyond exercising the create/get/update endpoints directly, the
//! `posting_valuations_then_triggering_sync_settles_balance` test guards
//! against the race condition that originally surfaced in the
//! sure-akahu-sync-tool's `sync_property` flow: rapidly posting several
//! valuations causes Sure's `sync_later` jobs to be deduplicated by Sidekiq
//! per family, so the single SyncJob that ultimately runs may not see the
//! latest reconciliation. The fix on the sync-tool side is an explicit
//! `trigger_sync()` after the backfill — this test pins down both the
//! original bug shape and the fix.

#![allow(
    clippy::tests_outside_test_module,
    clippy::unwrap_used,
    clippy::too_many_lines,
    reason = "Integration tests are correctly placed outside cfg(test) modules"
)]

use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use sure_client_rs::models::account::{
    AccountableAttributes, Address, PropertyAttributes, PropertySubtype,
};
use sure_client_rs::{AccountId, Auth, SureClient};

/// Helper to construct a client from the SURE_BASE_URL / SURE_TOKEN env vars.
fn create_test_client() -> SureClient {
    dotenvy::dotenv().ok();

    let base_url = std::env::var("SURE_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string())
        .parse()
        .unwrap();
    let token = std::env::var("SURE_TOKEN").expect("SURE_TOKEN must be set in .env file");

    SureClient::new(reqwest::Client::new(), Auth::api_key(token), base_url)
}

/// Creates a fresh single-family-home Property account suitable for
/// valuation tests. Returns the AccountId.
async fn create_test_property(client: &SureClient, label: &str) -> AccountId {
    let attributes = AccountableAttributes::Property(PropertyAttributes {
        subtype: Some(PropertySubtype::SingleFamilyHome),
        year_built: None,
        area_value: None,
        area_unit: None,
        locked_attributes: None,
        address_attributes: Some(Address {
            line1: Some("3A Amstel Lane".to_string()),
            line2: None,
            locality: Some("Christchurch".to_string()),
            region: None,
            postal_code: None,
            country: Some("New Zealand".to_string()),
        }),
    });

    let timestamp = Utc::now().timestamp_micros();
    let account = client
        .create_account()
        .name(format!("{label} {timestamp}"))
        .balance(Decimal::new(77_000_000, 2))
        .attributes(attributes)
        .currency(iso_currency::Currency::NZD)
        .call()
        .await
        .expect("Failed to create test property account");

    account.id
}

/// Polls `account.balance` until it equals `expected` or `attempts` runs out.
/// Sure's balance materialiser runs asynchronously inside Sidekiq, so callers
/// have to wait briefly after triggering work.
async fn await_balance(
    client: &SureClient,
    id: &AccountId,
    expected: Decimal,
    attempts: u32,
) -> Decimal {
    let mut last = Decimal::ZERO;
    for _ in 0..attempts {
        let acct = client
            .get_account(id)
            .await
            .expect("Failed to fetch account while polling balance");
        last = acct.balance;
        if last == expected {
            return last;
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
    last
}

#[tokio::test]
async fn create_valuation_persists_with_amount_and_date() {
    let client = create_test_client();
    let account_id = create_test_property(&client, "Valuation Create").await;

    let date = NaiveDate::from_ymd_opt(2026, 1, 12).expect("valid date literal");
    let amount = Decimal::new(77_063_910, 2);

    let valuation = client
        .create_valuation()
        .account_id(account_id)
        .amount(amount)
        .date(date)
        .notes("Created by integration test".to_string())
        .call()
        .await
        .expect("create_valuation should succeed");

    assert_eq!(valuation.date, date);
    assert_eq!(valuation.amount, amount);
    assert_eq!(
        valuation.notes.as_deref(),
        Some("Created by integration test")
    );
    println!(
        "✓ Created valuation {} amount={} date={}",
        valuation.id, valuation.amount, valuation.date
    );
}

#[tokio::test]
async fn get_valuation_round_trips_created_entry() {
    let client = create_test_client();
    let account_id = create_test_property(&client, "Valuation Get").await;

    let date = NaiveDate::from_ymd_opt(2026, 2, 12).expect("valid date literal");
    let amount = Decimal::new(77_128_321, 2);

    let created = client
        .create_valuation()
        .account_id(account_id)
        .amount(amount)
        .date(date)
        .call()
        .await
        .expect("create_valuation should succeed");

    let fetched = client
        .get_valuation(&created.id)
        .await
        .expect("get_valuation should succeed");

    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.amount, amount);
    assert_eq!(fetched.date, date);
}

#[tokio::test]
async fn update_valuation_overwrites_amount_and_date() {
    let client = create_test_client();
    let account_id = create_test_property(&client, "Valuation Update").await;

    let original_date = NaiveDate::from_ymd_opt(2026, 3, 12).expect("valid date literal");
    let new_date = NaiveDate::from_ymd_opt(2026, 3, 15).expect("valid date literal");
    let original_amount = Decimal::new(77_192_810, 2);
    let new_amount = Decimal::new(77_200_000, 2);

    let created = client
        .create_valuation()
        .account_id(account_id)
        .amount(original_amount)
        .date(original_date)
        .call()
        .await
        .expect("create_valuation should succeed");

    let updated = client
        .update_valuation()
        .id(&created.id)
        .amount(new_amount)
        .date(new_date)
        .notes("Edited via integration test".to_string())
        .call()
        .await
        .expect("update_valuation should succeed");

    // Sure rotates the entry id when reconciliation date changes, so we just
    // assert the response reflects what we asked for.
    assert_eq!(updated.amount, new_amount);
    assert_eq!(updated.date, new_date);
    assert_eq!(
        updated.notes.as_deref(),
        Some("Edited via integration test")
    );
}

/// End-to-end regression test for the bug that bit `sync_property` in
/// sure-akahu-sync-tool: posting several past-dated valuations in rapid
/// succession causes Sure's deduped SyncJob to materialise balances for
/// only the early reconciliations, leaving `account.balance` stuck on a
/// stale value. The fix is an explicit `trigger_sync()` after the
/// backfill — this test asserts the post-trigger balance.
#[tokio::test]
async fn posting_valuations_then_triggering_sync_settles_balance() {
    let client = create_test_client();
    let account_id = create_test_property(&client, "Valuation Settles").await;

    // Mimic the sync tool's monthly backfill: four valuations dated the
    // 12th of consecutive months, each a tiny notch above the last,
    // posted as fast as the API will accept.
    let backfill = [
        (
            NaiveDate::from_ymd_opt(2026, 1, 12).expect("date literal"),
            Decimal::new(77_063_910, 2),
        ),
        (
            NaiveDate::from_ymd_opt(2026, 2, 12).expect("date literal"),
            Decimal::new(77_128_321, 2),
        ),
        (
            NaiveDate::from_ymd_opt(2026, 3, 12).expect("date literal"),
            Decimal::new(77_192_810, 2),
        ),
        (
            NaiveDate::from_ymd_opt(2026, 4, 12).expect("date literal"),
            Decimal::new(77_257_413, 2),
        ),
    ];

    for (date, amount) in &backfill {
        client
            .create_valuation()
            .account_id(account_id)
            .amount(*amount)
            .date(*date)
            .call()
            .await
            .expect("create_valuation should succeed");
    }

    // Without trigger_sync, Sidekiq dedup may have collapsed all four
    // sync_later calls into a single SyncJob that ran before all four
    // reconciliations were committed. Force an explicit family sync.
    client
        .trigger_sync()
        .await
        .expect("trigger_sync should succeed");

    // Latest reconciliation is April 12 → expect that to drive the balance.
    let expected = Decimal::new(77_257_413, 2);
    let observed = await_balance(&client, &account_id, expected, 30).await;
    assert_eq!(
        observed, expected,
        "account.balance should settle on the latest reconciliation after trigger_sync"
    );
    println!("✓ Balance settled to {observed} after trigger_sync");
}
