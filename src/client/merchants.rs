use bon::bon;
use reqwest::Method;

use crate::ApiError;
use crate::error::ApiResult;
use crate::models::{DeleteResponse, PaginatedResponse};
use crate::models::merchant::{
    CreateMerchantRequest, MerchantCollection, MerchantDetail, UpdateMerchantRequest,
};
use crate::types::MerchantId;
use std::collections::HashMap;

use super::SureClient;

const MAX_PER_PAGE: u32 = 100;

#[bon]
impl SureClient {
    /// List merchants
    ///
    /// Retrieves a paginated list of merchants.
    ///
    /// # Arguments
    /// * `page` - Page number (default: 1)
    /// * `per_page` - Items per page (default: 25, max: 100)
    ///
    /// # Returns
    /// A paginated response containing merchants and pagination metadata.
    ///
    /// # Errors
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// // Use defaults (page 1, per_page 25)
    /// let response = client.get_merchants().call().await?;
    ///
    /// for merchant in response.items.merchants {
    ///     println!("Merchant: {}", merchant.name);
    /// }
    ///
    /// // Or customize parameters using the builder
    /// let response = client.get_merchants().page(2).per_page(50).call().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[builder]
    pub async fn get_merchants(
        &self,
        #[builder(default = 1)] page: u32,
        #[builder(default = 25)] per_page: u32,
    ) -> ApiResult<PaginatedResponse<MerchantCollection>> {
        if per_page > MAX_PER_PAGE {
            return Err(ApiError::InvalidParameter(format!(
                "per_page cannot exceed {MAX_PER_PAGE}",
            )));
        }

        let mut query_params = HashMap::new();

        query_params.insert("page", page.to_string());
        query_params.insert("per_page", per_page.to_string());

        self.execute_request(Method::GET, "/api/v1/merchants", Some(&query_params), None)
            .await
    }

    /// Get a specific merchant by ID
    ///
    /// Retrieves detailed information about a single merchant.
    ///
    /// # Arguments
    /// * `id` - The merchant ID to retrieve
    ///
    /// # Returns
    /// Detailed merchant information.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the merchant doesn't exist.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken, MerchantId};
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let merchant_id = MerchantId::new(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap());
    /// let merchant = client.get_merchant(&merchant_id).await?;
    ///
    /// println!("Merchant: {}", merchant.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_merchant(&self, id: &MerchantId) -> ApiResult<MerchantDetail> {
        self.execute_request(
            Method::GET,
            &format!("/api/v1/merchants/{}", id),
            None,
            None,
        )
        .await
    }
}

impl SureClient {
    /// Create a new merchant
    ///
    /// Creates a new merchant with the specified details.
    ///
    /// # Arguments
    /// * `request` - The merchant creation request containing all required fields
    ///
    /// # Returns
    /// The newly created merchant with full details.
    ///
    /// # Errors
    /// Returns `ApiError::ValidationError` if required fields are missing or invalid.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    /// use sure_client_rs::models::merchant::{CreateMerchantRequest, CreateMerchantData};
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let request = CreateMerchantRequest {
    ///     merchant: CreateMerchantData {
    ///         name: "Starbucks".to_string(),
    ///         color: Some("#00704A".to_string()),
    ///     },
    /// };
    ///
    /// let merchant = client.create_merchant(&request).await?;
    /// println!("Created merchant: {}", merchant.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_merchant(
        &self,
        request: &CreateMerchantRequest,
    ) -> ApiResult<MerchantDetail> {
        self.execute_request(
            Method::POST,
            "/api/v1/merchants",
            None,
            Some(serde_json::to_string(request)?),
        )
        .await
    }

    /// Update a merchant
    ///
    /// Updates an existing merchant with new values. Only fields provided in the
    /// request will be updated.
    ///
    /// # Arguments
    /// * `id` - The merchant ID to update
    /// * `request` - The merchant update request containing fields to update
    ///
    /// # Returns
    /// The updated merchant.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the merchant doesn't exist.
    /// Returns `ApiError::ValidationError` if the provided values are invalid.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken, MerchantId};
    /// use sure_client_rs::models::merchant::{UpdateMerchantRequest, UpdateMerchantData};
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let merchant_id = MerchantId::new(Uuid::new_v4());
    ///
    /// let request = UpdateMerchantRequest {
    ///     merchant: UpdateMerchantData {
    ///         name: Some("Updated Merchant Name".to_string()),
    ///         color: Some("#FF0000".to_string()),
    ///     },
    /// };
    ///
    /// let merchant = client.update_merchant(&merchant_id, &request).await?;
    /// println!("Updated merchant: {}", merchant.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_merchant(
        &self,
        id: &MerchantId,
        request: &UpdateMerchantRequest,
    ) -> ApiResult<MerchantDetail> {
        self.execute_request(
            Method::PATCH,
            &format!("/api/v1/merchants/{}", id),
            None,
            Some(serde_json::to_string(request)?),
        )
        .await
    }

    /// Delete a merchant
    ///
    /// Permanently deletes a merchant.
    ///
    /// # Arguments
    /// * `id` - The merchant ID to delete
    ///
    /// # Returns
    /// A confirmation message.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the merchant doesn't exist.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken, MerchantId};
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let merchant_id = MerchantId::new(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap());
    /// let response = client.delete_merchant(&merchant_id).await?;
    ///
    /// println!("Deleted: {}", response.message);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_merchant(&self, id: &MerchantId) -> ApiResult<DeleteResponse> {
        self.execute_request(
            Method::DELETE,
            &format!("/api/v1/merchants/{}", id),
            None,
            None,
        )
        .await
    }
}
