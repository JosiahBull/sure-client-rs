use thiserror::Error;

/// The main error type for the Sure API client
#[derive(Debug, Error)]
pub enum ApiError {
    // API-level errors
    /// Bad request error (400)
    #[error("Bad request: {message} (status: {status})")]
    BadRequest {
        /// The error message from the API
        message: String,
        /// The HTTP status code
        status: reqwest::StatusCode,
    },

    /// Unauthorized error (401)
    #[error("Unauthorized: {message}")]
    Unauthorized {
        /// The error message from the API
        message: String,
    },

    /// Forbidden error (403)
    #[error("Forbidden: {message}")]
    Forbidden {
        /// The error message from the API
        message: String,
    },

    /// Not found error (404)
    #[error("Not found: {message}")]
    NotFound {
        /// The error message from the API
        message: String,
    },

    /// Unprocessable entity error (422)
    #[error("Validation error: {message}")]
    ValidationError {
        /// The error message from the API
        message: String,
    },

    /// Rate limit error (429)
    #[error("Rate limited: {message}")]
    RateLimited {
        /// The error message from the API
        message: String,
    },

    /// Internal server error (500)
    #[error("Internal server error: {message}")]
    InternalServerError {
        /// The error message from the API
        message: String,
    },

    /// Generic API error
    #[error("API error {status}: {message}")]
    ApiError {
        /// The HTTP status code
        status: reqwest::StatusCode,
        /// The error message from the API
        message: String,
    },

    // Client-level errors
    /// Invalid parameter error (client-side validation)
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Network error
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// Invalid header value
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    /// URL parse error
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// JSON deserialization error
    #[error("JSON deserialization error: {error}. Source: {source_string}")]
    JsonDeserialization {
        /// The underlying serde error
        error: serde_json::Error,
        /// The source string that failed to deserialize
        source_string: String,
    },

    /// JSON serialization error
    #[error("JSON serialization error: {0}")]
    JsonSerialization(#[from] serde_json::Error),
}

/// Result type alias for the Sure API client
pub type ApiResult<T> = std::result::Result<T, ApiError>;
