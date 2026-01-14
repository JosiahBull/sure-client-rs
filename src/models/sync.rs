use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Sync status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SyncStatus {
    /// Pending
    Pending,
    /// Syncing
    Syncing,
    /// Completed
    Completed,
    /// Failed
    Failed,
}

impl std::fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncStatus::Pending => write!(f, "pending"),
            SyncStatus::Syncing => write!(f, "syncing"),
            SyncStatus::Completed => write!(f, "completed"),
            SyncStatus::Failed => write!(f, "failed"),
        }
    }
}

/// Error returned when parsing a `SyncStatus` from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseSyncStatusError(String);

impl std::fmt::Display for ParseSyncStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid sync status: {}", self.0)
    }
}

impl std::error::Error for ParseSyncStatusError {}

impl std::str::FromStr for SyncStatus {
    type Err = ParseSyncStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(SyncStatus::Pending),
            "syncing" => Ok(SyncStatus::Syncing),
            "completed" => Ok(SyncStatus::Completed),
            "failed" => Ok(SyncStatus::Failed),
            _ => Err(ParseSyncStatusError(s.to_string())),
        }
    }
}

impl TryFrom<&str> for SyncStatus {
    type Error = ParseSyncStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for SyncStatus {
    type Error = ParseSyncStatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Sync response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct SyncResponse {
    /// Sync ID
    pub id: Uuid,
    /// Sync status
    pub status: SyncStatus,
    /// Syncable type
    pub syncable_type: String,
    /// Syncable ID
    pub syncable_id: Uuid,
    /// Syncing start timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub syncing_at: Option<DateTime<Utc>>,
    /// Completion timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    /// Window start date
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::serde::naive_date_option")]
    pub window_start_date: Option<NaiveDate>,
    /// Window end date
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::serde::naive_date_option")]
    pub window_end_date: Option<NaiveDate>,
    /// Response message
    pub message: String,
}
