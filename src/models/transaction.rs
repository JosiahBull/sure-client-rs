use crate::types::{AccountId, CategoryId, MerchantId, TagId, TransactionId};
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Account information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Account {
    /// Unique identifier
    pub id: AccountId,
    /// Account name
    pub name: String,
    /// Formatted balance (e.g. "$1,234.56")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    /// Currency code (e.g. "USD")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Account classification (e.g. "asset", "liability")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    /// Accountable type (e.g. "depository", "investment", "credit_card")
    pub account_type: String,
}

/// Category information (basic version for transaction references)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Category {
    /// Unique identifier
    pub id: CategoryId,
    /// Category name
    pub name: String,
    /// Classification (income or expense)
    pub classification: String,
    /// Color for UI display (hex code)
    pub color: String,
    /// Icon identifier
    pub icon: String,
}

/// Merchant information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Merchant {
    /// Unique identifier
    pub id: MerchantId,
    /// Merchant name
    pub name: String,
}

/// Tag information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tag {
    /// Unique identifier
    pub id: TagId,
    /// Tag name
    pub name: String,
    /// Color for UI display (hex code)
    pub color: String,
}

/// Transfer information (for money transfers between accounts)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transfer {
    /// Unique identifier
    pub id: TransactionId,
    /// Transfer amount
    // #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: String,
    /// Currency code (e.g., "USD", "EUR")
    pub currency: String,
    /// The other account involved in the transfer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_account: Option<Account>,
}

/// Transaction nature/type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionNature {
    /// Income transaction
    #[serde(alias = "inflow")]
    Income,
    /// Expense transaction
    #[serde(alias = "outflow")]
    Expense,
}

/// Transaction filter type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    /// Income transactions
    Income,
    /// Expense transactions
    Expense,
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Income => "income",
            Self::Expense => "expense",
        };
        write!(f, "{}", s)
    }
}

/// Complete transaction information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transaction {
    /// Unique identifier
    pub id: TransactionId,
    /// Transaction date
    #[serde(with = "crate::serde::naive_date")]
    pub date: NaiveDate,
    /// Transaction amount
    // #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: String,
    /// Currency code (e.g., "USD", "EUR")
    pub currency: String,
    /// Transaction name/description
    pub name: String,
    /// Additional notes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Classification (income/expense)
    pub classification: String,
    /// Associated account
    pub account: Account,
    /// Associated category
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    /// Associated merchant
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant: Option<Merchant>,
    /// Associated tags
    pub tags: Vec<Tag>,
    /// Associated transfer (if this is a transfer)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer: Option<Transfer>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Collection of transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransactionCollection {
    /// List of transactions
    pub transactions: Vec<Transaction>,
}

/// Request body for creating a transaction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateTransactionRequest {
    /// Transaction data
    pub transaction: CreateTransactionData,
}

/// Transaction data for creation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateTransactionData {
    /// Account ID (required)
    pub account_id: AccountId,
    /// Transaction date (required)
    #[serde(with = "crate::serde::naive_date")]
    pub date: NaiveDate,
    /// Transaction amount (required)
    pub amount: Decimal,
    /// Transaction name/description (required)
    pub name: String,
    /// Additional notes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Currency code (defaults to family currency)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Category ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<CategoryId>,
    /// Merchant ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<MerchantId>,
    /// Transaction nature (determines sign)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nature: Option<TransactionNature>,
    /// Tag IDs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<TagId>>,
}

/// Request body for updating a transaction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateTransactionRequest {
    /// Transaction data
    pub transaction: UpdateTransactionData,
}

/// Transaction data for updates
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateTransactionData {
    /// Transaction date
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::serde::naive_date_option")]
    pub date: Option<NaiveDate>,
    /// Transaction amount
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Decimal>,
    /// Transaction name/description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Additional notes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Currency code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Category ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<CategoryId>,
    /// Merchant ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<MerchantId>,
    /// Transaction nature
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nature: Option<TransactionNature>,
    /// Tag IDs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<TagId>>,
}
