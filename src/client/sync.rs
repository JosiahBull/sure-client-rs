use crate::error::ApiResult;
use crate::models::sync::SyncResponse;
use reqwest::Method;

use super::SureClient;

impl SureClient {
    /// Trigger a family sync
    ///
    /// Triggers a sync operation that will apply all active rules, sync all accounts,
    /// and auto-match transfers.
    ///
    /// # Returns
    /// Sync response with status information.
    ///
    /// # Errors
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Forbidden` if the API key has insufficient scope.
    /// Returns `ApiError::InternalServerError` if the sync fails to start.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = client.trigger_sync().await?;
    /// println!("Sync queued: {}", response.message);
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub async fn trigger_sync(&self) -> ApiResult<SyncResponse> {
        self.execute_request(Method::POST, "/api/v1/sync", None, None)
            .await
    }
}
