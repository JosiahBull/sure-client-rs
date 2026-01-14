use crate::error::ApiResult;
use crate::models::auth::{
    AuthLoginResponse, AuthSignupResponse, AuthTokenResponse, LoginRequest, RefreshTokenRequest,
    SignupRequest,
};
use reqwest::Method;

use super::SureClient;

impl SureClient {
    /// Sign up a new user
    ///
    /// Creates a new user account with the provided credentials.
    ///
    /// # Arguments
    /// * `request` - The signup request containing user and device information
    ///
    /// # Returns
    /// Authentication response with access token and user information.
    ///
    /// # Errors
    /// Returns `ApiError::Forbidden` if invite code is required or invalid.
    /// Returns `ApiError::ValidationError` if validation fails.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    /// use sure_client_rs::models::auth::{SignupRequest, SignupUserData, DeviceInfo};
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let request = SignupRequest {
    ///     user: SignupUserData {
    ///         email: "user@example.com".to_string(),
    ///         password: "SecureP@ssw0rd".to_string(),
    ///         first_name: "John".to_string(),
    ///         last_name: "Doe".to_string(),
    ///     },
    ///     invite_code: None,
    ///     device: DeviceInfo {
    ///         device_id: "device123".to_string(),
    ///         device_name: "My Device".to_string(),
    ///         device_type: "web".to_string(),
    ///         os_version: "macOS 12.0".to_string(),
    ///         app_version: "1.0.0".to_string(),
    ///     },
    /// };
    ///
    /// let response = client.signup(&request).await?;
    /// println!("User created: {}", response.user.email);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn signup(&self, request: &SignupRequest) -> ApiResult<AuthSignupResponse> {
        self.execute_request(
            Method::POST,
            "/api/v1/auth/signup",
            None,
            Some(serde_json::to_string(request)?),
        )
        .await
    }

    /// Log in a user
    ///
    /// Authenticates a user with email and password.
    ///
    /// # Arguments
    /// * `request` - The login request containing credentials and device information
    ///
    /// # Returns
    /// Authentication response with access token and user information.
    ///
    /// # Errors
    /// Returns `ApiError::Unauthorized` if credentials are invalid or MFA is required.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    /// use sure_client_rs::models::auth::{LoginRequest, DeviceInfo};
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let request = LoginRequest {
    ///     email: "user@example.com".to_string(),
    ///     password: "SecureP@ssw0rd".to_string(),
    ///     otp_code: None,
    ///     device: DeviceInfo {
    ///         device_id: "device123".to_string(),
    ///         device_name: "My Device".to_string(),
    ///         device_type: "web".to_string(),
    ///         os_version: "macOS 12.0".to_string(),
    ///         app_version: "1.0.0".to_string(),
    ///     },
    /// };
    ///
    /// let response = client.login(&request).await?;
    /// println!("Logged in as: {}", response.user.email);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn login(&self, request: &LoginRequest) -> ApiResult<AuthLoginResponse> {
        self.execute_request(
            Method::POST,
            "/api/v1/auth/login",
            None,
            Some(serde_json::to_string(request)?),
        )
        .await
    }

    /// Refresh an access token
    ///
    /// Refreshes an expired access token using a refresh token.
    ///
    /// # Arguments
    /// * `request` - The refresh request containing the refresh token and device ID
    ///
    /// # Returns
    /// New authentication tokens.
    ///
    /// # Errors
    /// Returns `ApiError::BadRequest` if refresh token is missing.
    /// Returns `ApiError::Unauthorized` if refresh token is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    /// use sure_client_rs::models::auth::{RefreshTokenRequest, RefreshDeviceInfo};
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let request = RefreshTokenRequest {
    ///     refresh_token: "refresh_token_here".to_string(),
    ///     device: RefreshDeviceInfo {
    ///         device_id: "device123".to_string(),
    ///     },
    /// };
    ///
    /// let response = client.refresh_token(&request).await?;
    /// println!("Token refreshed, expires in: {}s", response.expires_in.as_secs());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn refresh_token(
        &self,
        request: &RefreshTokenRequest,
    ) -> ApiResult<AuthTokenResponse> {
        self.execute_request(
            Method::POST,
            "/api/v1/auth/refresh",
            None,
            Some(serde_json::to_string(request)?),
        )
        .await
    }
}
