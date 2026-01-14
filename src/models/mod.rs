pub mod account;
pub mod auth;
pub mod category;
pub mod chat;
pub mod merchant;
pub mod sync;
pub mod transaction;
pub mod usage;

use serde::{Deserialize, Serialize};

/// Pagination information for paginated responses
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Pagination {
    /// Current page number (1-based)
    pub page: u32,
    /// Number of items per page
    pub per_page: u32,
    /// Total number of items across all pages
    pub total_count: u32,
    /// Total number of pages
    pub total_pages: u32,
}

/// Generic paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct PaginatedResponse<T> {
    /// The items in this page
    #[serde(flatten)]
    pub items: T,
    /// Pagination metadata
    pub pagination: Pagination,
}

/// Response for successful deletion operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DeleteResponse {
    /// Confirmation message
    pub message: String,
}

/// Error response from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct ErrorResponse {
    /// Error type/code
    pub error: String,
    /// Optional error message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Optional error details (can be array or object)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}
