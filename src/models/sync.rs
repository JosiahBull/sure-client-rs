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
