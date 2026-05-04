use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::models::account::Account;
use crate::serde::deserialize_flexible_decimal;
use crate::types::{AccountId, ValuationId};

/// The kind of valuation entry. Most user-driven valuations are `reconciliation`
/// entries; the others are anchor entries that mark the opening or current balance
/// of an account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValuationKind {
    /// A user-driven correction or periodic re-valuation of an account's balance.
    Reconciliation,
    /// Anchor for the account's opening balance.
    OpeningAnchor,
    /// Anchor for the account's current balance.
    CurrentAnchor,
}

/// A valuation entry representing a point-in-time balance for an account.
///
/// Valuations are commonly used to record property re-valuations, investment
/// portfolio snapshots, and other manual balance updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Valuation {
    /// Unique identifier (the underlying entry id).
    pub id: ValuationId,
    /// Date the valuation applies to.
    pub date: NaiveDate,
    /// The valuation amount. The API returns this formatted (e.g. `"$770,639.10"`),
    /// so it is parsed via the flexible decimal deserialiser to give a `Decimal`.
    #[serde(deserialize_with = "deserialize_flexible_decimal")]
    pub amount: Decimal,
    /// Currency code.
    pub currency: iso_currency::Currency,
    /// Optional notes attached to the valuation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Kind of valuation (reconciliation, opening anchor, current anchor).
    pub kind: ValuationKind,
    /// Account this valuation belongs to.
    pub account: Account,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Last update timestamp.
    pub updated_at: DateTime<Utc>,
}

/// Request body for creating a valuation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct CreateValuationRequest {
    /// Valuation data.
    pub valuation: CreateValuationData,
}

/// Data for creating a valuation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct CreateValuationData {
    /// Account ID (required).
    pub account_id: AccountId,
    /// Valuation amount (required).
    pub amount: Decimal,
    /// Valuation date (required), serialised as `YYYY-MM-DD`.
    pub date: NaiveDate,
    /// Optional notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Request body for updating a valuation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct UpdateValuationRequest {
    /// Valuation data.
    pub valuation: UpdateValuationData,
}

/// Data for updating a valuation. The API requires both `amount` and `date` to be
/// supplied together when editing the underlying reconciliation; `notes` may be
/// updated independently.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct UpdateValuationData {
    /// New valuation amount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Decimal>,
    /// New valuation date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    /// Updated notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
