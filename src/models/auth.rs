use crate::serde::duration_from_secs;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

/// Token type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenType {
    /// Bearer token
    Bearer,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bearer => write!(f, "Bearer"),
        }
    }
}

/// Error returned when parsing a `TokenType` from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseTokenTypeError(String);

impl std::fmt::Display for ParseTokenTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid token type: {}", self.0)
    }
}

impl std::error::Error for ParseTokenTypeError {}

impl std::str::FromStr for TokenType {
    type Err = ParseTokenTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Bearer" => Ok(Self::Bearer),
            _ => Err(ParseTokenTypeError(s.to_string())),
        }
    }
}

impl TryFrom<&str> for TokenType {
    type Error = ParseTokenTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for TokenType {
    type Error = ParseTokenTypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Base authentication token response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct AuthTokenResponse {
    /// Access token
    pub access_token: String,
    /// Refresh token
    pub refresh_token: String,
    /// Token type (Bearer)
    pub token_type: TokenType,
    /// Token expiration time
    #[serde(with = "duration_from_secs")]
    pub expires_in: Duration,
    /// Unix timestamp of token creation
    #[serde(with = "duration_from_secs")]
    pub created_at: Duration,
}

/// User information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct User {
    /// User ID
    pub id: Uuid,
    /// Email address
    pub email: String,
    /// First name
    pub first_name: String,
    /// Last name
    pub last_name: String,
}

/// Sign up response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct AuthSignupResponse {
    /// Access token
    pub access_token: String,
    /// Refresh token
    pub refresh_token: String,
    /// Token type (Bearer)
    pub token_type: TokenType,
    /// Token expiration time
    #[serde(with = "duration_from_secs")]
    pub expires_in: Duration,
    /// Unix timestamp of token creation
    #[serde(with = "duration_from_secs")]
    pub created_at: Duration,
    /// User information
    pub user: User,
}

/// Login response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct AuthLoginResponse {
    /// Access token
    pub access_token: String,
    /// Refresh token
    pub refresh_token: String,
    /// Token type (Bearer)
    pub token_type: TokenType,
    /// Token expiration time
    #[serde(with = "duration_from_secs")]
    pub expires_in: Duration,
    /// Unix timestamp of token creation
    #[serde(with = "duration_from_secs")]
    pub created_at: Duration,
    /// User information
    pub user: User,
}

/// Device information for authentication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DeviceInfo {
    /// Device identifier
    pub device_id: String,
    /// Device name
    pub device_name: String,
    /// Device type (e.g., "ios", "android", "web")
    pub device_type: String,
    /// OS version
    pub os_version: String,
    /// App version
    pub app_version: String,
}

/// Sign up request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct SignupRequest {
    /// User information
    pub user: SignupUserData,
    /// Invite code (required if invite codes are enabled)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_code: Option<String>,
    /// Device information
    pub device: DeviceInfo,
}

/// User data for sign up
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct SignupUserData {
    /// Email address
    pub email: String,
    /// Password (must be at least 8 characters with uppercase, lowercase, number, and special character)
    pub password: String,
    /// First name
    pub first_name: String,
    /// Last name
    pub last_name: String,
}

/// Login request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct LoginRequest {
    /// Email address
    pub email: String,
    /// Password
    pub password: String,
    /// OTP code (required if user has MFA enabled)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub otp_code: Option<String>,
    /// Device information
    pub device: DeviceInfo,
}

/// Refresh token request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct RefreshTokenRequest {
    /// Refresh token
    pub refresh_token: String,
    /// Device information
    pub device: RefreshDeviceInfo,
}

/// Device information for refresh request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct RefreshDeviceInfo {
    /// Device identifier
    pub device_id: String,
}
