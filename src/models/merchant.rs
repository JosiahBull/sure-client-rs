use crate::types::MerchantId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Basic merchant information (used in transactions)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Merchant {
    /// Unique identifier
    pub id: MerchantId,
    /// Merchant name
    pub name: String,
}

/// Detailed merchant information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct MerchantDetail {
    /// Unique identifier
    pub id: MerchantId,
    /// Merchant name
    pub name: String,
    /// Color for UI display (hex code)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Collection of merchants with pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct MerchantCollection {
    /// List of merchants
    pub merchants: Vec<MerchantDetail>,
}

/// Request to create a new merchant
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct CreateMerchantRequest {
    /// Merchant data
    pub merchant: CreateMerchantData,
}

/// Data for creating a new merchant
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct CreateMerchantData {
    /// Merchant name
    pub name: String,
    /// Merchant color (hex code)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Request to update an existing merchant
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct UpdateMerchantRequest {
    /// Merchant data
    pub merchant: UpdateMerchantData,
}

/// Data for updating a merchant
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct UpdateMerchantData {
    /// Merchant name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Merchant color (hex code)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}
