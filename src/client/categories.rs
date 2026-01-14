use bon::bon;
use reqwest::Method;

use crate::ApiError;
use crate::error::ApiResult;
use crate::models::category::{
    CategoryCollection, CategoryDetail, Classification, CreateCategoryData, CreateCategoryRequest,
    UpdateCategoryData, UpdateCategoryRequest,
};
use crate::models::{DeleteResponse, PaginatedResponse};
use crate::types::CategoryId;
use std::collections::HashMap;

use super::SureClient;

const MAX_PER_PAGE: u32 = 100;

#[bon]
impl SureClient {
    /// List categories with optional filters
    ///
    /// Retrieves a paginated list of categories. Results can be filtered by classification,
    /// parent category, or limited to root categories only.
    ///
    /// # Arguments
    /// * `page` - Page number (default: 1)
    /// * `per_page` - Items per page (default: 25, max: 100)
    /// * `classification` - Filter by classification (income or expense)
    /// * `roots_only` - Return only root categories (default: false)
    /// * `parent_id` - Filter by parent category ID
    ///
    /// # Returns
    /// A paginated response containing categories and pagination metadata.
    ///
    /// # Errors
    /// Returns `ApiError::Unauthorized` if the bearer token is invalid or expired.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    #[builder]
    pub async fn get_categories(
        &self,
        #[builder(default = 1)] page: u32,
        #[builder(default = 25)] per_page: u32,
        #[builder(default = false)] roots_only: bool,
        classification: Option<Classification>,
        parent_id: Option<&CategoryId>,
    ) -> ApiResult<PaginatedResponse<CategoryCollection>> {
        let mut query_params = HashMap::new();

        if per_page > MAX_PER_PAGE {
            return Err(ApiError::InvalidParameter(format!(
                "per_page cannot exceed {MAX_PER_PAGE}",
            )));
        }

        query_params.insert("page", page.to_string());
        query_params.insert("per_page", per_page.to_string());
        query_params.insert("roots_only", roots_only.to_string());

        if let Some(classification) = classification {
            query_params.insert("classification", classification.to_string());
        }

        if let Some(parent_id) = parent_id {
            query_params.insert("parent_id", parent_id.to_string());
        }

        self.execute_request(Method::GET, "/api/v1/categories", Some(&query_params), None)
            .await
    }

    /// Get a specific category by ID
    ///
    /// Retrieves detailed information about a single category, including parent
    /// and subcategory information.
    ///
    /// # Arguments
    /// * `id` - The category ID to retrieve
    ///
    /// # Returns
    /// Detailed category information including parent and subcategory count.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the category doesn't exist.
    /// Returns `ApiError::Unauthorized` if the bearer token is invalid or expired.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    pub async fn get_category(&self, id: &CategoryId) -> ApiResult<CategoryDetail> {
        self.execute_request(
            Method::GET,
            &format!("/api/v1/categories/{}", id),
            None,
            None,
        )
        .await
    }
}

#[bon]
impl SureClient {
    /// Create a new category
    ///
    /// Creates a new category with the specified details.
    ///
    /// # Arguments
    /// * `request` - The category creation request containing all required fields
    ///
    /// # Returns
    /// The newly created category with full details.
    ///
    /// # Errors
    /// Returns `ApiError::ValidationError` if required fields are missing or invalid.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    /// use sure_client_rs::models::category::Classification;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let category = client.create_category()
    ///     .name("Groceries".to_string())
    ///     .classification(Classification::Expense)
    ///     .color("#FF5733".to_string())
    ///     .lucide_icon("shopping-cart".to_string())
    ///     .call()
    ///     .await?;
    ///
    /// println!("Created category: {}", category.name);
    /// # Ok(())
    /// # }
    /// ```
    #[builder]
    pub async fn create_category(
        &self,
        name: String,
        classification: Classification,
        color: String,
        lucide_icon: Option<String>,
        parent_id: Option<CategoryId>,
    ) -> ApiResult<CategoryDetail> {
        let request = CreateCategoryRequest {
            category: CreateCategoryData {
                name,
                classification,
                color,
                lucide_icon,
                parent_id,
            },
        };

        self.execute_request(
            Method::POST,
            "/api/v1/categories",
            None,
            Some(serde_json::to_string(&request)?),
        )
        .await
    }

    /// Update a category
    ///
    /// Updates an existing category with new values. Only fields provided in the
    /// request will be updated.
    ///
    /// # Arguments
    /// * `id` - The category ID to update
    /// * `request` - The category update request containing fields to update
    ///
    /// # Returns
    /// The updated category.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the category doesn't exist.
    /// Returns `ApiError::ValidationError` if the provided values are invalid.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken, CategoryId};
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let category_id = CategoryId::new(Uuid::new_v4());
    ///
    /// let category = client.update_category()
    ///     .id(&category_id)
    ///     .name("Updated Category Name".to_string())
    ///     .color("#00FF00".to_string())
    ///     .call()
    ///     .await?;
    ///
    /// println!("Updated category: {}", category.name);
    /// # Ok(())
    /// # }
    /// ```
    #[builder]
    pub async fn update_category(
        &self,
        id: &CategoryId,
        name: Option<String>,
        classification: Option<Classification>,
        color: Option<String>,
        lucide_icon: Option<String>,
        parent_id: Option<CategoryId>,
    ) -> ApiResult<CategoryDetail> {
        let request = UpdateCategoryRequest {
            category: UpdateCategoryData {
                name,
                classification,
                color,
                lucide_icon,
                parent_id,
            },
        };

        self.execute_request(
            Method::PATCH,
            &format!("/api/v1/categories/{}", id),
            None,
            Some(serde_json::to_string(&request)?),
        )
        .await
    }

    /// Delete a category
    ///
    /// Permanently deletes a category.
    ///
    /// # Arguments
    /// * `id` - The category ID to delete
    ///
    /// # Returns
    /// A confirmation message.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the category doesn't exist.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken, CategoryId};
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let category_id = CategoryId::new(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap());
    /// let response = client.delete_category(&category_id).await?;
    ///
    /// println!("Deleted: {}", response.message);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_category(&self, id: &CategoryId) -> ApiResult<DeleteResponse> {
        self.execute_request(
            Method::DELETE,
            &format!("/api/v1/categories/{}", id),
            None,
            None,
        )
        .await
    }
}
