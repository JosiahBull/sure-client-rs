mod accounts;
mod auth;
mod categories;
mod chats;
mod core;
mod merchants;
mod sync;
mod transactions;
mod usage;

use crate::types::Auth;

/// The main Sure API client
///
/// This client provides access to all Sure API endpoints. It handles authentication,
/// request execution, and error handling.
///
/// The API supports two authentication methods:
/// - Bearer token (JWT) via Authorization header
/// - API key via X-Api-Key header
///
/// # Example with API Key
/// ```no_run
/// use sure_client_rs::{SureClient, Auth};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = SureClient::new(
///     reqwest::Client::new(),
///     Auth::api_key("your_api_key"),
///     "https://api.sure.app".to_string(),
/// );
///
/// let categories = client.get_categories().call().await?;
/// # Ok(())
/// # }
/// ```
///
/// # Example with Bearer Token
/// ```no_run
/// use sure_client_rs::{SureClient, Auth};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = SureClient::new(
///     reqwest::Client::new(),
///     Auth::bearer("your_jwt_token"),
///     "https://api.sure.app".to_string(),
/// );
///
/// let categories = client.get_categories().call().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct SureClient {
    /// The HTTP client for making requests
    pub(crate) client: reqwest::Client,
    /// Authentication credentials (Bearer token or API key)
    pub(crate) auth: Auth,
    /// Base URL for the API (defaults to production)
    pub(crate) base_url: String,
}

impl SureClient {
    /// Create a new Sure API client
    ///
    /// # Arguments
    /// * `client` - A configured reqwest::Client for making HTTP requests
    /// * `auth` - Authentication method (Bearer token or API key)
    /// * `base_url` - base url to target
    ///
    /// # Example with API Key
    /// ```no_run
    /// use sure_client_rs::{SureClient, Auth};
    ///
    /// let client = SureClient::new(
    ///     reqwest::Client::new(),
    ///     Auth::api_key("your_api_key"),
    ///     "https://api.sure.app".to_string()
    /// );
    /// ```
    ///
    /// # Example with Bearer Token
    /// ```no_run
    /// use sure_client_rs::{SureClient, Auth};
    ///
    /// let client = SureClient::new(
    ///     reqwest::Client::new(),
    ///     Auth::bearer("your_token"),
    ///     "https://api.sure.app".to_string()
    /// );
    /// ```
    pub fn new<T: Into<Auth>>(
        client: reqwest::Client,
        auth: T,
        base_url: String,
    ) -> Self {
        Self {
            client,
            auth: auth.into(),
            base_url,
        }
    }
}
