use crate::error::ApiResult;
use crate::models::auth::{
    AuthLoginResponse, AuthSignupResponse, AuthTokenResponse, DeviceInfo, LoginRequest,
    RefreshDeviceInfo, RefreshTokenRequest, SignupRequest, SignupUserData,
};
use bon::bon;
use reqwest::Method;

use super::SureClient;

#[bon]
impl SureClient {
    /// Sign up a new user
    ///
    /// Creates a new user account with the provided credentials.
    ///
    /// # Arguments
    /// * `user` - User data (email, password, first_name, last_name)
    /// * `device` - Device information
    /// * `invite_code` - Invite code (required if invite codes are enabled)
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
    /// use sure_client_rs::models::auth::{SignupUserData, DeviceInfo};
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = client.signup()
    ///     .user(SignupUserData {
    ///         email: "user@example.com".to_string(),
    ///         password: "SecureP@ssw0rd".to_string(),
    ///         first_name: "John".to_string(),
    ///         last_name: "Doe".to_string(),
    ///     })
    ///     .device(DeviceInfo {
    ///         device_id: "device123".to_string(),
    ///         device_name: "My Device".to_string(),
    ///         device_type: "web".to_string(),
    ///         os_version: "macOS 12.0".to_string(),
    ///         app_version: "1.0.0".to_string(),
    ///     })
    ///     .call()
    ///     .await?;
    ///
    /// println!("User created: {}", response.user.email);
    /// # Ok(())
    /// # }
    /// ```
    #[builder]
    pub async fn signup(
        &self,
        user: SignupUserData,
        device: DeviceInfo,
        invite_code: Option<String>,
    ) -> ApiResult<AuthSignupResponse> {
        let request = SignupRequest {
            user,
            invite_code,
            device,
        };

        self.execute_request(
            Method::POST,
            "/api/v1/auth/signup",
            None,
            Some(serde_json::to_string(&request)?),
        )
        .await
    }

    /// Log in a user
    ///
    /// Authenticates a user with email and password.
    ///
    /// # Arguments
    /// * `email` - Email address
    /// * `password` - Password
    /// * `device` - Device information
    /// * `otp_code` - OTP code (required if user has MFA enabled)
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
    /// use sure_client_rs::models::auth::DeviceInfo;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = client.login()
    ///     .email("user@example.com".to_string())
    ///     .password("SecureP@ssw0rd".to_string())
    ///     .device(DeviceInfo {
    ///         device_id: "device123".to_string(),
    ///         device_name: "My Device".to_string(),
    ///         device_type: "web".to_string(),
    ///         os_version: "macOS 12.0".to_string(),
    ///         app_version: "1.0.0".to_string(),
    ///     })
    ///     .call()
    ///     .await?;
    ///
    /// println!("Logged in as: {}", response.user.email);
    /// # Ok(())
    /// # }
    /// ```
    #[builder]
    pub async fn login(
        &self,
        email: String,
        password: String,
        device: DeviceInfo,
        otp_code: Option<String>,
    ) -> ApiResult<AuthLoginResponse> {
        let request = LoginRequest {
            email,
            password,
            otp_code,
            device,
        };

        self.execute_request(
            Method::POST,
            "/api/v1/auth/login",
            None,
            Some(serde_json::to_string(&request)?),
        )
        .await
    }

    /// Refresh an access token
    ///
    /// Refreshes an expired access token using a refresh token.
    ///
    /// # Arguments
    /// * `refresh_token` - Refresh token
    /// * `device` - Device information
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
    /// use sure_client_rs::models::auth::RefreshDeviceInfo;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = client.refresh_token()
    ///     .refresh_token("refresh_token_here".to_string())
    ///     .device(RefreshDeviceInfo {
    ///         device_id: "device123".to_string(),
    ///     })
    ///     .call()
    ///     .await?;
    ///
    /// println!("Token refreshed, expires in: {}s", response.expires_in.as_secs());
    /// # Ok(())
    /// # }
    /// ```
    #[builder]
    pub async fn refresh_token(
        &self,
        refresh_token: String,
        device: RefreshDeviceInfo,
    ) -> ApiResult<AuthTokenResponse> {
        let request = RefreshTokenRequest {
            refresh_token,
            device,
        };

        self.execute_request(
            Method::POST,
            "/api/v1/auth/refresh",
            None,
            Some(serde_json::to_string(&request)?),
        )
        .await
    }
}
