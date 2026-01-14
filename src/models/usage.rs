use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Rate limit tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RateLimitTier {
    /// Standard tier
    Standard,
    /// Premium tier
    Premium,
    /// Enterprise tier
    Enterprise,
    /// No operation (testing/development)
    Noop,
    /// Unknown tier
    #[serde(other)]
    Unknown,
}

/// API key information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiKeyInfo {
    /// API key name
    pub name: String,
    /// API key scopes
    pub scopes: Vec<String>,
    /// Last used timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<DateTime<Utc>>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Rate limit information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RateLimitInfo {
    /// Rate limit tier
    pub tier: RateLimitTier,
    /// Rate limit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Current count
    pub current_count: i64,
    /// Remaining requests
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
    /// Reset time in seconds
    pub reset_in_seconds: i64,
    /// Reset timestamp
    pub reset_at: DateTime<Utc>,
}

/// Usage response for API key authentication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageApiKeyResponse {
    /// API key information
    pub api_key: ApiKeyInfo,
    /// Rate limit information
    pub rate_limit: RateLimitInfo,
}

/// Authentication method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthenticationMethod {
    /// OAuth authentication
    OAuth,
}

/// Usage response for OAuth authentication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageOAuthResponse {
    /// Authentication method
    pub authentication_method: AuthenticationMethod,
    /// Response message
    pub message: String,
}

/// Usage response (can be either API key or OAuth)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsageResponse {
    /// API key authentication response
    ApiKey(UsageApiKeyResponse),
    /// OAuth authentication response
    OAuth(UsageOAuthResponse),
}
