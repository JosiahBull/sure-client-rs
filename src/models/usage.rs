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

impl std::fmt::Display for RateLimitTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RateLimitTier::Standard => write!(f, "standard"),
            RateLimitTier::Premium => write!(f, "premium"),
            RateLimitTier::Enterprise => write!(f, "enterprise"),
            RateLimitTier::Noop => write!(f, "noop"),
            RateLimitTier::Unknown => write!(f, "unknown"),
        }
    }
}

/// Error returned when parsing a `RateLimitTier` from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseRateLimitTierError(String);

impl std::fmt::Display for ParseRateLimitTierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid rate limit tier: {}", self.0)
    }
}

impl std::error::Error for ParseRateLimitTierError {}

impl std::str::FromStr for RateLimitTier {
    type Err = ParseRateLimitTierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(RateLimitTier::Standard),
            "premium" => Ok(RateLimitTier::Premium),
            "enterprise" => Ok(RateLimitTier::Enterprise),
            "noop" => Ok(RateLimitTier::Noop),
            _ => Ok(RateLimitTier::Unknown),
        }
    }
}

impl TryFrom<&str> for RateLimitTier {
    type Error = ParseRateLimitTierError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for RateLimitTier {
    type Error = ParseRateLimitTierError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// API key information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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

impl std::fmt::Display for AuthenticationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthenticationMethod::OAuth => write!(f, "oauth"),
        }
    }
}

/// Error returned when parsing an `AuthenticationMethod` from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseAuthenticationMethodError(String);

impl std::fmt::Display for ParseAuthenticationMethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid authentication method: {}", self.0)
    }
}

impl std::error::Error for ParseAuthenticationMethodError {}

impl std::str::FromStr for AuthenticationMethod {
    type Err = ParseAuthenticationMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "oauth" => Ok(AuthenticationMethod::OAuth),
            _ => Err(ParseAuthenticationMethodError(s.to_string())),
        }
    }
}

impl TryFrom<&str> for AuthenticationMethod {
    type Error = ParseAuthenticationMethodError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for AuthenticationMethod {
    type Error = ParseAuthenticationMethodError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Usage response for OAuth authentication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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
