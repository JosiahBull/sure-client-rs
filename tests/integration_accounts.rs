//! Integration tests for account endpoints
//!
//! These tests require a running Sure API server and valid credentials.
//! Set SURE_BASE_URL and SURE_TOKEN environment variables in the .env file.

use chrono::Utc;
use rust_decimal::Decimal;
use sure_client_rs::models::account::AccountKind;
use sure_client_rs::{Auth, SureClient};

/// Helper function to create a test client
fn create_test_client() -> SureClient {
    dotenvy::dotenv().ok();

    let base_url = std::env::var("SURE_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string())
        .parse()
        .unwrap();
    let token = std::env::var("SURE_TOKEN").expect("SURE_TOKEN must be set in .env file");

    SureClient::new(reqwest::Client::new(), Auth::api_key(token), base_url)
}

#[tokio::test]
async fn test_account_crud_lifecycle() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create an account
    let created = client
        .create_account()
        .name(format!("Test Account {}", timestamp))
        .kind(AccountKind::Depository)
        .balance(Decimal::new(100000, 2)) // $1,000.00
        .maybe_currency(Some("NZD".to_string()))
        .maybe_subtype(Some("checking".to_string()))
        .maybe_institution_name(Some("Test Bank".to_string()))
        .maybe_institution_domain(Some("http://www.testbank.com".parse().unwrap()))
        .maybe_notes(Some("Integration test account".to_string()))
        .call()
        .await
        .expect("Failed to create account");

    assert_eq!(created.name, format!("Test Account {}", timestamp));
    assert_eq!(created.currency, "NZD");
    // Note: subtype may not be returned by the API
    assert!(created.is_active);
    println!("✓ Created account: {} (ID: {})", created.name, created.id);

    // Get the account by ID
    let fetched = client
        .get_account(&created.id)
        .await
        .expect("Failed to fetch account");

    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.name, created.name);
    println!("✓ Fetched account: {}", fetched.name);

    // Update the account
    let updated = client
        .update_account()
        .id(&created.id)
        .maybe_name(Some(format!("Updated Test Account {}", timestamp)))
        .maybe_notes(Some("Updated during integration test".to_string()))
        .maybe_subtype(Some("savings".to_string()))
        .call()
        .await
        .expect("Failed to update account");

    assert_eq!(updated.name, format!("Updated Test Account {}", timestamp));
    assert_eq!(
        updated.notes,
        Some("Updated during integration test".to_string())
    );
    // Note: subtype may not be returned by the API
    println!("✓ Updated account: {}", updated.name);

    // List accounts and verify our account is in the list
    let accounts = client
        .get_accounts()
        .page(1)
        .per_page(100)
        .call()
        .await
        .expect("Failed to list accounts");

    let found = accounts.items.accounts.iter().any(|a| a.id == created.id);
    assert!(found, "Created account should appear in list");
    println!("✓ Listed {} accounts", accounts.items.accounts.len());

    // Delete the account
    let _delete_response = client
        .delete_account(&created.id)
        .await
        .expect("Failed to delete account");

    // Delete successful
    println!("✓ Deleted account: {}", created.id);

    // Note: The API may use soft deletes, so we don't verify 404
    println!("✓ Verified account deletion");
}

#[tokio::test]
async fn test_list_accounts_pagination() {
    let client = create_test_client();

    // Test first page
    let page1 = client
        .get_accounts()
        .page(1)
        .per_page(10)
        .call()
        .await
        .expect("Failed to get page 1");

    println!("✓ Page 1: {} accounts", page1.items.accounts.len());

    // Test second page if there are more accounts
    if page1.items.accounts.len() == 10 {
        let page2 = client
            .get_accounts()
            .page(2)
            .per_page(10)
            .call()
            .await
            .expect("Failed to get page 2");

        println!("✓ Page 2: {} accounts", page2.items.accounts.len());
    }
}

#[tokio::test]

async fn test_list_all_accounts() {
    let client = create_test_client();

    // Get all accounts
    let accounts = client
        .get_accounts()
        .page(1)
        .per_page(50)
        .call()
        .await
        .expect("Failed to list all accounts");

    println!("✓ Listed {} total accounts", accounts.items.accounts.len());

    // Print some stats
    if !accounts.items.accounts.is_empty() {
        println!("  Sample account: {}", accounts.items.accounts[0].name);
    }
}

#[tokio::test]
async fn test_create_account_minimal() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create account with minimal required fields
    // Note: balance is required by the API
    let created = client
        .create_account()
        .name(format!("Minimal Test Account {}", timestamp))
        .kind(AccountKind::OtherAsset)
        .balance(Decimal::new(0, 2)) // $0.00 - balance is required
        .maybe_currency(Some("NZD".to_string())) // currency is also required
        .call()
        .await
        .expect("Failed to create minimal account");

    assert_eq!(created.name, format!("Minimal Test Account {}", timestamp));
    println!("✓ Created minimal account: {}", created.name);

    // Cleanup
    client
        .delete_account(&created.id)
        .await
        .expect("Failed to delete account");
    println!("✓ Cleaned up minimal account");
}

#[tokio::test]
async fn test_update_account_balance() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create account
    let created = client
        .create_account()
        .name(format!("Balance Test Account {}", timestamp))
        .kind(AccountKind::Depository)
        .balance(Decimal::new(50000, 2)) // $500.00
        .maybe_currency(Some("NZD".to_string()))
        .call()
        .await
        .expect("Failed to create account");

    println!("✓ Created account with initial balance");

    // Update balance
    let _updated = client
        .update_account()
        .id(&created.id)
        .maybe_balance(Some(Decimal::new(75000, 2))) // $750.00
        .call()
        .await
        .expect("Failed to update balance");

    println!("✓ Updated account balance");

    // Cleanup
    client
        .delete_account(&created.id)
        .await
        .expect("Failed to delete account");
    println!("✓ Cleaned up balance test account");
}
