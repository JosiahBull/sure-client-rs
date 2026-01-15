use crate::types::{AccountId, CategoryId, MerchantId, TagId, TransactionId};
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Account information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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
    pub currency: Option<iso_currency::Currency>,
    /// Account classification (e.g. "asset", "liability")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    /// Accountable type (e.g. "depository", "investment", "credit_card")
    pub account_type: String,
}

/// Category information (basic version for transaction references)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Merchant {
    /// Unique identifier
    pub id: MerchantId,
    /// Merchant name
    pub name: String,
}

/// Tag information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Transfer {
    /// Unique identifier
    pub id: TransactionId,
    /// Transfer amount
    // #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: String,
    /// Currency code (e.g., "USD", "EUR")
    pub currency: iso_currency::Currency,
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

impl std::fmt::Display for TransactionNature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Income => write!(f, "income"),
            Self::Expense => write!(f, "expense"),
        }
    }
}

/// Error returned when parsing a `TransactionNature` from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseTransactionNatureError(String);

impl std::fmt::Display for ParseTransactionNatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid transaction nature: {}", self.0)
    }
}

impl std::error::Error for ParseTransactionNatureError {}

impl std::str::FromStr for TransactionNature {
    type Err = ParseTransactionNatureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "income" | "inflow" => Ok(Self::Income),
            "expense" | "outflow" => Ok(Self::Expense),
            _ => Err(ParseTransactionNatureError(s.to_string())),
        }
    }
}

impl TryFrom<&str> for TransactionNature {
    type Error = ParseTransactionNatureError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for TransactionNature {
    type Error = ParseTransactionNatureError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
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

/// Error returned when parsing a `TransactionType` from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseTransactionTypeError(String);

impl std::fmt::Display for ParseTransactionTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid transaction type: {}", self.0)
    }
}

impl std::error::Error for ParseTransactionTypeError {}

impl std::str::FromStr for TransactionType {
    type Err = ParseTransactionTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "income" => Ok(Self::Income),
            "expense" => Ok(Self::Expense),
            _ => Err(ParseTransactionTypeError(s.to_string())),
        }
    }
}

impl TryFrom<&str> for TransactionType {
    type Error = ParseTransactionTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for TransactionType {
    type Error = ParseTransactionTypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Complete transaction information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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
    pub currency: iso_currency::Currency,
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
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct TransactionCollection {
    /// List of transactions
    pub transactions: Vec<Transaction>,
}

/// Request body for creating a transaction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct CreateTransactionRequest {
    /// Transaction data
    pub transaction: CreateTransactionData,
}

/// Transaction data for creation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct CreateTransactionData {
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
    /// Currency code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<iso_currency::Currency>,
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
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct UpdateTransactionRequest {
    /// Transaction data
    pub transaction: UpdateTransactionData,
}

/// Transaction data for updates
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct UpdateTransactionData {
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
    pub currency: Option<iso_currency::Currency>,
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
