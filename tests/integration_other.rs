//! Integration tests for sync, usage, and chat endpoints
//!
//! These tests require a running Sure API server and valid credentials.
//! Set SURE_BASE_URL and SURE_TOKEN environment variables in the .env file.
//!
//! Note: Auth tests (signup, login) are excluded as they require special setup
//! and may interfere with existing user accounts.

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

// ============================================================================
// Usage Endpoint Tests
// ============================================================================

#[tokio::test]
async fn test_get_usage() {
    let client = create_test_client();

    let usage = client.get_usage().await.expect("Failed to get usage info");

    match usage {
        sure_client_rs::models::usage::UsageResponse::ApiKey(api_key_usage) => {
            println!("✓ API Key Usage:");
            println!("  - Name: {}", api_key_usage.api_key.name);
            println!("  - Scopes: {}", api_key_usage.api_key.scopes.join(", "));
            println!("  - Rate Limit Tier: {:?}", api_key_usage.rate_limit.tier);
            if let (Some(remaining), Some(limit)) = (
                api_key_usage.rate_limit.remaining,
                api_key_usage.rate_limit.limit,
            ) {
                println!("  - Remaining Requests: {}/{}", remaining, limit);
            } else {
                println!("  - Rate Limit: Not applicable");
            }
        }
        sure_client_rs::models::usage::UsageResponse::OAuth(oauth_usage) => {
            println!("✓ OAuth Usage:");
            println!("  - Method: {:?}", oauth_usage.authentication_method);
            println!("  - Message: {}", oauth_usage.message);
        }
    }
}

// ============================================================================
// Sync Endpoint Tests
// ============================================================================

#[tokio::test]
async fn test_trigger_sync() {
    let client = create_test_client();

    let sync_response = client.trigger_sync().await.expect("Failed to trigger sync");

    println!("✓ Triggered sync:");
    println!("  - ID: {}", sync_response.id);
    println!("  - Status: {:?}", sync_response.status);
    println!("  - Type: {}", sync_response.syncable_type);
    println!("  - Message: {}", sync_response.message);
}
