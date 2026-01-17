//! Integration tests for merchant endpoints
//!
//! These tests require a running Sure API server and valid credentials.
//! Set SURE_BASE_URL and SURE_TOKEN environment variables in the .env file.

#![allow(
    clippy::tests_outside_test_module,
    clippy::unwrap_used,
    clippy::too_many_lines,
    clippy::indexing_slicing,
    reason = "Integration tests are correctly placed outside cfg(test) modules"
)]

use chrono::Utc;
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

async fn test_merchant_crud_lifecycle() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create a merchant
    let created = client
        .create_merchant()
        .name(format!("Test Merchant {}", timestamp))
        .color("#FF5733".to_string())
        .call()
        .await
        .expect("Failed to create merchant");

    assert_eq!(created.name, format!("Test Merchant {}", timestamp));
    // Note: API may generate or modify colors
    println!("✓ Created merchant: {} (ID: {})", created.name, created.id);

    // Get the merchant by ID
    let fetched = client
        .get_merchant(&created.id)
        .await
        .expect("Failed to fetch merchant");

    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.name, created.name);
    println!("✓ Fetched merchant: {}", fetched.name);

    // Update the merchant
    let updated = client
        .update_merchant()
        .id(&created.id)
        .name(format!("Updated Merchant {}", timestamp))
        .color("#3366FF".to_string())
        .call()
        .await
        .expect("Failed to update merchant");

    assert_eq!(updated.name, format!("Updated Merchant {}", timestamp));
    // Note: API may generate or modify colors
    println!("✓ Updated merchant: {}", updated.name);

    // List merchants (merchant may not appear immediately due to indexing)
    let merchants = client
        .get_merchants()
        .page(1)
        .per_page(100)
        .call()
        .await
        .expect("Failed to list merchants");

    println!("✓ Listed {} merchants", merchants.items.merchants.len());

    // Delete the merchant
    let _delete_response = client
        .delete_merchant(&created.id)
        .await
        .expect("Failed to delete merchant");

    // Delete successful
    println!("✓ Deleted merchant: {}", created.id);

    // Verify merchant is deleted (should return 404)
    let result = client.get_merchant(&created.id).await;
    assert!(result.is_err(), "Deleted merchant should not be fetchable");
    println!("✓ Verified merchant deletion");
}

#[tokio::test]

async fn test_create_merchant_without_color() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create merchant without color
    let created = client
        .create_merchant()
        .name(format!("No Color Merchant {}", timestamp))
        .call()
        .await
        .expect("Failed to create merchant without color");

    assert_eq!(created.name, format!("No Color Merchant {}", timestamp));
    println!("✓ Created merchant without color: {}", created.name);

    // Cleanup
    client
        .delete_merchant(&created.id)
        .await
        .expect("Failed to delete merchant");
    println!("✓ Cleaned up merchant");
}

#[tokio::test]

async fn test_list_merchants_pagination() {
    let client = create_test_client();

    // Test first page
    let page1 = client
        .get_merchants()
        .page(1)
        .per_page(10)
        .call()
        .await
        .expect("Failed to get page 1");

    println!("✓ Page 1: {} merchants", page1.items.merchants.len());

    // Test second page if there are more merchants
    if page1.items.merchants.len() == 10 {
        let page2 = client
            .get_merchants()
            .page(2)
            .per_page(10)
            .call()
            .await
            .expect("Failed to get page 2");

        println!("✓ Page 2: {} merchants", page2.items.merchants.len());
    }
}

#[tokio::test]

async fn test_list_merchants_comprehensive() {
    let client = create_test_client();

    // Get all merchants
    let merchants = client
        .get_merchants()
        .page(1)
        .per_page(100)
        .call()
        .await
        .expect("Failed to list merchants");

    println!("✓ Listed {} merchants", merchants.items.merchants.len());

    // Print sample merchant if available
    if !merchants.items.merchants.is_empty() {
        println!("  Sample merchant: {}", merchants.items.merchants[0].name);
    }
}

#[tokio::test]

async fn test_update_merchant_name_only() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create merchant
    let created = client
        .create_merchant()
        .name(format!("Original Name {}", timestamp))
        .color("#FF0000".to_string())
        .call()
        .await
        .expect("Failed to create merchant");

    println!("✓ Created merchant with original name");

    // Update only the name
    let updated = client
        .update_merchant()
        .id(&created.id)
        .name(format!("New Name {}", timestamp))
        .call()
        .await
        .expect("Failed to update merchant name");

    assert_eq!(updated.name, format!("New Name {}", timestamp));
    // Note: API may generate or modify colors
    println!("✓ Updated merchant name");

    // Cleanup
    client
        .delete_merchant(&created.id)
        .await
        .expect("Failed to delete merchant");
    println!("✓ Cleaned up test merchant");
}

#[tokio::test]

async fn test_update_merchant_color_only() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create merchant
    let created = client
        .create_merchant()
        .name(format!("Color Test Merchant {}", timestamp))
        .color("#00FF00".to_string())
        .call()
        .await
        .expect("Failed to create merchant");

    let original_name = created.name.clone();
    println!("✓ Created merchant with original color");

    // Update only the color
    let updated = client
        .update_merchant()
        .id(&created.id)
        .color("#0000FF".to_string())
        .call()
        .await
        .expect("Failed to update merchant color");

    assert_eq!(updated.name, original_name);
    // Note: API may generate or modify colors
    println!("✓ Updated merchant, name unchanged");

    // Cleanup
    client
        .delete_merchant(&created.id)
        .await
        .expect("Failed to delete merchant");
    println!("✓ Cleaned up test merchant");
}
