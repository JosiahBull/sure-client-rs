//! Integration tests for transaction endpoints
//!
//! These tests require a running Sure API server and valid credentials.
//! Set SURE_BASE_URL and SURE_TOKEN environment variables in the .env file.

use chrono::{DateTime, TimeZone, Utc};
use rust_decimal::Decimal;
use sure_client_rs::models::account::{
    AccountDetail, AccountableAttributes, DepositoryAttributes, DepositorySubtype,
};
use sure_client_rs::models::transaction::TransactionNature;
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

async fn test_transaction_crud_lifecycle() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // First, create a test account for transactions
    let account: AccountDetail = client
        .create_account()
        .name(format!("Transaction Test Account {}", timestamp))
        .attributes(AccountableAttributes::Depository(DepositoryAttributes {
            subtype: Some(DepositorySubtype::Savings),
            locked_attributes: None,
        }))
        .balance(Decimal::new(100000, 2))
        .currency(iso_currency::Currency::NZD)
        .notes("For transaction testing".to_string())
        .call()
        .await
        .expect("Failed to create test account");

    println!("✓ Created test account: {}", account.name);

    // Create a transaction
    let transaction_date: DateTime<Utc> = Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();

    let created = client
        .create_transaction()
        .account_id(account.id.clone())
        .date(transaction_date)
        .amount(Decimal::new(4250, 2)) // $42.50
        .name(format!("Test Transaction {}", timestamp))
        .notes("Integration test transaction".to_string())
        .currency(iso_currency::Currency::NZD)
        .nature(TransactionNature::Expense)
        .call()
        .await
        .expect("Failed to create transaction");

    assert_eq!(created.name, format!("Test Transaction {}", timestamp));
    assert_eq!(created.classification, "expense");
    println!(
        "✓ Created transaction: {} (ID: {})",
        created.name, created.id
    );

    // Get the transaction by ID
    let fetched = client
        .get_transaction(&created.id)
        .await
        .expect("Failed to fetch transaction");

    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.name, created.name);
    println!("✓ Fetched transaction: {}", fetched.name);

    // Update the transaction
    let updated = client
        .update_transaction()
        .id(&created.id)
        .name(format!("Updated Transaction {}", timestamp))
        .notes("Updated during integration test".to_string())
        .amount(Decimal::new(5000, 2)) // $50.00
        .call()
        .await
        .expect("Failed to update transaction");

    assert_eq!(updated.name, format!("Updated Transaction {}", timestamp));
    assert_eq!(
        updated.notes,
        Some("Updated during integration test".to_string())
    );
    println!("✓ Updated transaction: {}", updated.name);

    // List transactions and verify our transaction is in the list
    // Filter by account to ensure we find the transaction
    let transactions = client
        .get_transactions()
        .page(1)
        .per_page(100)
        .account_id(&account.id)
        .call()
        .await
        .expect("Failed to list transactions");

    let found = transactions
        .items
        .transactions
        .iter()
        .any(|t| t.id == created.id);
    assert!(found, "Created transaction should appear in list");
    println!(
        "✓ Listed {} transactions for account",
        transactions.items.transactions.len()
    );

    // Delete the transaction
    let _delete_response = client
        .delete_transaction(&created.id)
        .await
        .expect("Failed to delete transaction");

    // Delete successful
    println!("✓ Deleted transaction: {}", created.id);

    // Cleanup: delete the test account
    client
        .delete_account(&account.id)
        .await
        .expect("Failed to delete test account");
    println!("✓ Cleaned up test account");
}

#[tokio::test]

async fn test_list_transactions_with_filters() {
    let client = create_test_client();

    // Test with date range filter
    let start_date: DateTime<Utc> = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end_date: DateTime<Utc> = Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap();

    let transactions = client
        .get_transactions()
        .page(1)
        .per_page(50)
        .start_date(&start_date)
        .end_date(&end_date)
        .call()
        .await
        .expect("Failed to list transactions with date filter");

    println!(
        "✓ Filtered {} transactions by date range",
        transactions.items.transactions.len()
    );

    // Verify all transactions are within the date range
    for transaction in &transactions.items.transactions {
        assert!(
            transaction.date >= start_date && transaction.date <= end_date,
            "Transaction date should be within range"
        );
    }
}

#[tokio::test]

async fn test_list_transactions_by_account() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create a test account
    let attributes = AccountableAttributes::Depository(DepositoryAttributes {
        subtype: Some(DepositorySubtype::Checking),
        locked_attributes: None,
    });

    let account = client
        .create_account()
        .name(format!("Filter Test Account {}", timestamp))
        .balance(Decimal::new(100000, 2))
        .attributes(attributes)
        .currency(iso_currency::Currency::NZD)
        .call()
        .await
        .expect("Failed to create test account");

    println!("✓ Created test account: {}", account.name);

    // Create a transaction for this account
    let transaction_date: DateTime<Utc> = Utc.with_ymd_and_hms(2024, 6, 15, 12, 0, 0).unwrap();

    let created_transaction = client
        .create_transaction()
        .account_id(account.id.clone())
        .date(transaction_date)
        .amount(Decimal::new(2500, 2))
        .name(format!("Filter Test Transaction {}", timestamp))
        .currency(iso_currency::Currency::NZD)
        .nature(TransactionNature::Expense)
        .call()
        .await
        .expect("Failed to create transaction");

    println!("✓ Created test transaction");

    // List transactions filtered by account
    let transactions = client
        .get_transactions()
        .page(1)
        .per_page(100)
        .account_ids(&[account.id.clone()])
        .call()
        .await
        .expect("Failed to list transactions by account");

    // Verify all transactions belong to this account
    for transaction in &transactions.items.transactions {
        assert_eq!(
            transaction.account.id, account.id,
            "Transaction should belong to the specified account"
        );
    }

    println!(
        "✓ Filtered {} transactions for account",
        transactions.items.transactions.len()
    );

    // Cleanup
    client
        .delete_transaction(&created_transaction.id)
        .await
        .expect("Failed to delete transaction");
    client
        .delete_account(&account.id)
        .await
        .expect("Failed to delete account");
    println!("✓ Cleaned up test data");
}

#[tokio::test]

async fn test_transaction_with_income_nature() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create test account
    let attributes = AccountableAttributes::Depository(DepositoryAttributes {
        subtype: Some(DepositorySubtype::Checking),
        locked_attributes: None,
    });

    let account = client
        .create_account()
        .name(format!("Income Test Account {}", timestamp))
        .balance(Decimal::new(0, 2))
        .attributes(attributes)
        .currency(iso_currency::Currency::NZD)
        .call()
        .await
        .expect("Failed to create test account");

    // Create an income transaction
    let transaction_date: DateTime<Utc> = Utc.with_ymd_and_hms(2024, 3, 1, 9, 0, 0).unwrap();

    let created = client
        .create_transaction()
        .account_id(account.id.clone())
        .date(transaction_date)
        .amount(Decimal::new(150000, 2)) // $1,500.00
        .name(format!("Salary Payment {}", timestamp))
        .notes("Test income transaction".to_string())
        .currency(iso_currency::Currency::NZD)
        .nature(TransactionNature::Income)
        .call()
        .await
        .expect("Failed to create income transaction");

    assert_eq!(created.classification, "income");
    println!("✓ Created income transaction: {}", created.name);

    // Cleanup
    client
        .delete_transaction(&created.id)
        .await
        .expect("Failed to delete transaction");
    client
        .delete_account(&account.id)
        .await
        .expect("Failed to delete account");
    println!("✓ Cleaned up test data");
}

#[tokio::test]

async fn test_list_transactions_pagination() {
    let client = create_test_client();

    // Test first page
    let page1 = client
        .get_transactions()
        .page(1)
        .per_page(10)
        .call()
        .await
        .expect("Failed to get page 1");

    println!("✓ Page 1: {} transactions", page1.items.transactions.len());

    // Test second page if there are more transactions
    if page1.items.transactions.len() == 10 {
        let page2 = client
            .get_transactions()
            .page(2)
            .per_page(10)
            .call()
            .await
            .expect("Failed to get page 2");

        println!("✓ Page 2: {} transactions", page2.items.transactions.len());
    }
}
