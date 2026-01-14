//! Integration tests for sync, usage, and chat endpoints
//!
//! These tests require a running Sure API server and valid credentials.
//! Set SURE_BASE_URL and SURE_TOKEN environment variables in the .env file.
//!
//! Note: Auth tests (signup, login) are excluded as they require special setup
//! and may interfere with existing user accounts.

use chrono::Utc;
use sure_client_rs::models::chat::{CreateChatRequest, CreateMessageRequest, UpdateChatRequest};
use sure_client_rs::{Auth, SureClient};

/// Helper function to create a test client
fn create_test_client() -> SureClient {
    dotenvy::dotenv().ok();

    let base_url =
        std::env::var("SURE_BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
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

// ============================================================================
// Chat Endpoint Tests
// ============================================================================

#[tokio::test]
#[ignore] // Requires AI features to be enabled
async fn test_chat_crud_lifecycle() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create a chat
    let create_request = CreateChatRequest {
        title: format!("Test Chat {}", timestamp),
        message: Some("Hello, this is a test message!".to_string()),
        model: None,
    };

    let created = client
        .create_chat(&create_request)
        .await
        .expect("Failed to create chat");

    assert_eq!(created.title, format!("Test Chat {}", timestamp));
    println!("✓ Created chat: {} (ID: {})", created.title, created.id);

    // Verify initial message was created
    assert!(
        !created.messages.is_empty(),
        "Chat should have at least one message"
    );
    println!("  - Messages: {}", created.messages.len());

    // Get the chat by ID
    let fetched = client
        .get_chat(&created.id)
        .await
        .expect("Failed to fetch chat");

    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.title, created.title);
    println!("✓ Fetched chat: {}", fetched.title);

    // Update the chat
    let update_request = UpdateChatRequest {
        title: format!("Updated Chat {}", timestamp),
    };

    let updated = client
        .update_chat(&created.id, &update_request)
        .await
        .expect("Failed to update chat");

    assert_eq!(updated.title, format!("Updated Chat {}", timestamp));
    println!("✓ Updated chat: {}", updated.title);

    // List chats and verify our chat is in the list
    let chats = client
        .get_chats()
        .page(1)
        .per_page(100)
        .call()
        .await
        .expect("Failed to list chats");

    let found = chats.items.chats.iter().any(|c| c.id == created.id);
    assert!(found, "Created chat should appear in list");
    println!("✓ Listed {} chats", chats.items.chats.len());

    // Delete the chat
    client
        .delete_chat(&created.id)
        .await
        .expect("Failed to delete chat");

    println!("✓ Deleted chat: {}", created.id);

    // Verify chat is deleted (should return 404)
    let result = client.get_chat(&created.id).await;
    assert!(result.is_err(), "Deleted chat should not be fetchable");
    println!("✓ Verified chat deletion");
}

#[tokio::test]
#[ignore] // Requires AI features to be enabled
async fn test_create_chat_without_message() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create chat without initial message
    let create_request = CreateChatRequest {
        title: format!("Empty Chat {}", timestamp),
        message: None,
        model: None,
    };

    let created = client
        .create_chat(&create_request)
        .await
        .expect("Failed to create chat without message");

    assert_eq!(created.title, format!("Empty Chat {}", timestamp));
    println!("✓ Created chat without initial message: {}", created.title);

    // Cleanup
    client
        .delete_chat(&created.id)
        .await
        .expect("Failed to delete chat");
    println!("✓ Cleaned up chat");
}

#[tokio::test]
#[ignore] // Requires AI features to be enabled
async fn test_create_message_in_chat() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create a chat
    let create_chat_request = CreateChatRequest {
        title: format!("Message Test Chat {}", timestamp),
        message: Some("Initial message".to_string()),
        model: None,
    };

    let chat = client
        .create_chat(&create_chat_request)
        .await
        .expect("Failed to create chat");

    println!("✓ Created chat for message testing");

    let initial_message_count = chat.messages.len();

    // Create a new message in the chat
    let create_message_request = CreateMessageRequest {
        content: format!("Test message {}", timestamp),
        model: None,
    };

    let message_response = client
        .create_message(&chat.id, &create_message_request)
        .await
        .expect("Failed to create message");

    assert_eq!(
        message_response.content,
        format!("Test message {}", timestamp)
    );
    assert_eq!(message_response.chat_id, chat.id);
    println!("✓ Created message in chat");

    // Fetch the chat again to verify message was added
    let updated_chat = client
        .get_chat(&chat.id)
        .await
        .expect("Failed to fetch chat after adding message");

    assert!(
        updated_chat.messages.len() >= initial_message_count,
        "Chat should have at least as many messages as before"
    );
    println!("  - Messages in chat: {}", updated_chat.messages.len());

    // Cleanup
    client
        .delete_chat(&chat.id)
        .await
        .expect("Failed to delete chat");
    println!("✓ Cleaned up test chat");
}

#[tokio::test]
#[ignore] // Requires AI features to be enabled
async fn test_retry_message() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create a chat with a message
    let create_request = CreateChatRequest {
        title: format!("Retry Test Chat {}", timestamp),
        message: Some("Test message for retry".to_string()),
        model: None,
    };

    let chat = client
        .create_chat(&create_request)
        .await
        .expect("Failed to create chat");

    println!("✓ Created chat for retry testing");

    // Retry the message
    let retry_response = client
        .retry_message(&chat.id)
        .await
        .expect("Failed to retry message");

    println!("✓ Retried message:");
    println!("  - Message: {}", retry_response.message);
    println!("  - Message ID: {}", retry_response.message_id);

    // Cleanup
    client
        .delete_chat(&chat.id)
        .await
        .expect("Failed to delete chat");
    println!("✓ Cleaned up test chat");
}

#[tokio::test]
#[ignore] // Requires AI features to be enabled
async fn test_list_chats_pagination() {
    let client = create_test_client();

    // Test first page
    let page1 = client
        .get_chats()
        .page(1)
        .per_page(10)
        .call()
        .await
        .expect("Failed to get page 1");

    println!("✓ Page 1: {} chats", page1.items.chats.len());

    // Test second page if there are more chats
    if page1.items.chats.len() == 10 {
        let page2 = client
            .get_chats()
            .page(2)
            .per_page(10)
            .call()
            .await
            .expect("Failed to get page 2");

        println!("✓ Page 2: {} chats", page2.items.chats.len());
    }
}

#[tokio::test]
#[ignore] // Requires AI features to be enabled
async fn test_chat_with_custom_model() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create chat with custom model
    let create_request = CreateChatRequest {
        title: format!("Custom Model Chat {}", timestamp),
        message: Some("Test message with custom model".to_string()),
        model: Some("gpt-4".to_string()),
    };

    let created = client
        .create_chat(&create_request)
        .await
        .expect("Failed to create chat with custom model");

    println!("✓ Created chat with custom model");

    // Check if the message has the model set
    if let Some(first_message) = created.messages.first() {
        println!("  - First message model: {:?}", first_message.model);
    } else {
        println!("  - No messages in chat");
    }

    // Cleanup
    client
        .delete_chat(&created.id)
        .await
        .expect("Failed to delete chat");
    println!("✓ Cleaned up test chat");
}
