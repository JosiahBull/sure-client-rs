//! Integration tests for category endpoints
//!
//! These tests require a running Sure API server and valid credentials.
//! Set SURE_BASE_URL and SURE_TOKEN environment variables in the .env file.

use chrono::Utc;
use sure_client_rs::models::category::Classification;
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
        .classification(Classification::Expense)
        .color("#FF5733".to_string())
        .lucide_icon("shopping-cart".to_string())
        .call()
        .await
        .expect("Failed to create category");

    assert_eq!(created.name, format!("Test Category {}", timestamp));
    assert_eq!(created.classification, Classification::Expense);
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
    assert_eq!(fetched.classification, created.classification);
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

async fn test_list_categories_by_classification() {
    let client = create_test_client();

    // Get expense categories
    let expense_categories = client
        .get_categories()
        .page(1)
        .per_page(50)
        .classification(Classification::Expense)
        .call()
        .await
        .expect("Failed to list expense categories");

    // Verify all are expense categories
    for category in &expense_categories.items.categories {
        assert_eq!(
            category.classification,
            Classification::Expense,
            "Category {} should be expense type",
            category.id
        );
    }
    println!(
        "✓ Listed {} expense categories",
        expense_categories.items.categories.len()
    );

    // Get income categories
    let income_categories = client
        .get_categories()
        .page(1)
        .per_page(50)
        .classification(Classification::Income)
        .call()
        .await
        .expect("Failed to list income categories");

    // Verify all are income categories
    for category in &income_categories.items.categories {
        assert_eq!(
            category.classification,
            Classification::Income,
            "Category {} should be income type",
            category.id
        );
    }
    println!(
        "✓ Listed {} income categories",
        income_categories.items.categories.len()
    );
}

#[tokio::test]

async fn test_create_income_category() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create an income category
    let created = client
        .create_category()
        .name(format!("Test Income Category {}", timestamp))
        .classification(Classification::Income)
        .color("#00FF00".to_string())
        .lucide_icon("dollar-sign".to_string())
        .call()
        .await
        .expect("Failed to create income category");

    assert_eq!(created.classification, Classification::Income);
    assert_eq!(created.name, format!("Test Income Category {}", timestamp));
    println!("✓ Created income category: {}", created.name);

    // Cleanup
    client
        .delete_category(&created.id)
        .await
        .expect("Failed to delete category");
    println!("✓ Cleaned up income category");
}

#[tokio::test]

async fn test_category_with_parent() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create parent category
    let parent = client
        .create_category()
        .name(format!("Parent Category {}", timestamp))
        .classification(Classification::Expense)
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
        .classification(Classification::Expense)
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

#[tokio::test]

async fn test_update_category_classification() {
    let client = create_test_client();
    let timestamp = Utc::now().timestamp();

    // Create an expense category
    let created = client
        .create_category()
        .name(format!("Classification Test {}", timestamp))
        .classification(Classification::Expense)
        .color("#0000FF".to_string())
        .lucide_icon("credit-card".to_string())
        .call()
        .await
        .expect("Failed to create category");

    assert_eq!(created.classification, Classification::Expense);
    println!("✓ Created category as expense");

    // Update to income
    let updated = client
        .update_category()
        .id(&created.id)
        .classification(Classification::Income)
        .call()
        .await
        .expect("Failed to update category classification");

    assert_eq!(updated.classification, Classification::Income);
    println!("✓ Updated category to income classification");

    // Cleanup
    client
        .delete_category(&created.id)
        .await
        .expect("Failed to delete category");
    println!("✓ Cleaned up test category");
}
