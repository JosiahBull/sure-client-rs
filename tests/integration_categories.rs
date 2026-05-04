//! Integration tests for category endpoints
//!
//! These tests require a running Sure API server and valid credentials.
//! Set SURE_BASE_URL and SURE_TOKEN environment variables in the .env file.

#![allow(
    clippy::tests_outside_test_module,
    clippy::unwrap_used,
    clippy::too_many_lines,
    clippy::clone_on_copy,
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
async fn test_category_crud_lifecycle() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create a category
    let created = client
        .create_category()
        .name(format!("Test Category {}", timestamp))
        .color("#FF5733".to_string())
        .lucide_icon("shopping-cart".to_string())
        .call()
        .await
        .expect("Failed to create category");

    assert_eq!(created.name, format!("Test Category {}", timestamp));
    assert_eq!(created.color, "#FF5733");
    assert_eq!(created.icon, "shopping-cart");
    println!("✓ Created category: {} (ID: {})", created.name, created.id);

    // Get the category by ID
    let fetched = client
        .get_category(&created.id)
        .await
        .expect("Failed to fetch category");

    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.name, created.name);
    println!("✓ Fetched category: {}", fetched.name);

    // Update the category
    let updated = client
        .update_category()
        .id(&created.id)
        .name(format!("Updated Category {}", timestamp))
        .color("#3366FF".to_string())
        .lucide_icon("tag".to_string())
        .call()
        .await
        .expect("Failed to update category");

    assert_eq!(updated.name, format!("Updated Category {}", timestamp));
    assert_eq!(updated.color, "#3366FF");
    assert_eq!(updated.icon, "tag");
    println!("✓ Updated category: {}", updated.name);

    // List categories and verify our category is in the list
    let categories = client
        .get_categories()
        .page(1)
        .per_page(100)
        .call()
        .await
        .expect("Failed to list categories");

    let found = categories
        .items
        .categories
        .iter()
        .any(|c| c.id == created.id);
    assert!(found, "Created category should appear in list");
    println!("✓ Listed {} categories", categories.items.categories.len());

    // Delete the category
    let _delete_response = client
        .delete_category(&created.id)
        .await
        .expect("Failed to delete category");

    // Delete successful
    println!("✓ Deleted category: {}", created.id);

    // Verify category is deleted (should return 404)
    let result = client.get_category(&created.id).await;
    assert!(result.is_err(), "Deleted category should not be fetchable");
    println!("✓ Verified category deletion");
}

#[tokio::test]
async fn test_create_category_minimal() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create with just the required fields
    let created = client
        .create_category()
        .name(format!("Minimal Category {}", timestamp))
        .color("#00FF00".to_string())
        .call()
        .await
        .expect("Failed to create minimal category");

    assert_eq!(created.name, format!("Minimal Category {}", timestamp));
    println!("✓ Created minimal category: {}", created.name);

    // Cleanup
    client
        .delete_category(&created.id)
        .await
        .expect("Failed to delete category");
    println!("✓ Cleaned up minimal category");
}

#[tokio::test]
async fn test_category_with_parent() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create parent category
    let parent = client
        .create_category()
        .name(format!("Parent Category {}", timestamp))
        .color("#FF0000".to_string())
        .lucide_icon("folder".to_string())
        .call()
        .await
        .expect("Failed to create parent category");

    println!("✓ Created parent category: {}", parent.name);

    // Create child category
    let child = client
        .create_category()
        .name(format!("Child Category {}", timestamp))
        .color("#FFA500".to_string())
        .lucide_icon("file".to_string())
        .parent_id(parent.id.clone())
        .call()
        .await
        .expect("Failed to create child category");

    assert_eq!(child.name, format!("Child Category {}", timestamp));
    assert!(child.parent.is_some());
    if let Some(parent_ref) = &child.parent {
        assert_eq!(parent_ref.id, parent.id);
        assert_eq!(parent_ref.name, parent.name);
    }
    println!("✓ Created child category: {}", child.name);

    // Verify parent has subcategories
    let parent_detail = client
        .get_category(&parent.id)
        .await
        .expect("Failed to fetch parent category");

    assert!(
        parent_detail.subcategories_count > 0,
        "Parent should have at least one subcategory"
    );
    println!(
        "✓ Parent has {} subcategories",
        parent_detail.subcategories_count
    );

    // Cleanup (delete child first, then parent)
    client
        .delete_category(&child.id)
        .await
        .expect("Failed to delete child category");
    client
        .delete_category(&parent.id)
        .await
        .expect("Failed to delete parent category");
    println!("✓ Cleaned up parent and child categories");
}

#[tokio::test]
async fn test_list_categories_pagination() {
    let client = create_test_client();

    // Test first page
    let page1 = client
        .get_categories()
        .page(1)
        .per_page(10)
        .call()
        .await
        .expect("Failed to get page 1");

    println!("✓ Page 1: {} categories", page1.items.categories.len());

    // Test second page if there are more categories
    if page1.items.categories.len() == 10 {
        let page2 = client
            .get_categories()
            .page(2)
            .per_page(10)
            .call()
            .await
            .expect("Failed to get page 2");

        println!("✓ Page 2: {} categories", page2.items.categories.len());
    }
}
