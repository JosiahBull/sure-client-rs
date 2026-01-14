use crate::error::ApiResult;
use crate::models::usage::UsageResponse;
use reqwest::Method;

use super::SureClient;

impl SureClient {
    /// Get API usage information
    ///
    /// Returns usage statistics for API key authentication or a message for OAuth authentication.
    ///
    /// # Returns
    /// Usage response containing either API key usage information or OAuth authentication message.
    ///
    /// # Errors
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    /// use sure_client_rs::models::usage::UsageResponse;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = client.get_usage().await?;
    ///
    /// match response {
    ///     UsageResponse::ApiKey(usage) => {
    ///         println!("API Key: {}", usage.api_key.name);
    ///         println!("Current count: {}", usage.rate_limit.current_count);
    ///         if let Some(remaining) = usage.rate_limit.remaining {
    ///             println!("Remaining requests: {}", remaining);
    ///         }
    ///     }
    ///     UsageResponse::OAuth(info) => {
    ///         println!("OAuth: {}", info.message);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub async fn get_usage(&self) -> ApiResult<UsageResponse> {
        self.execute_request(Method::GET, "/api/v1/usage", None, None)
            .await
    }
}
