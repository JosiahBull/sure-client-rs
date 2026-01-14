use crate::types::AccountId;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// The kind of an account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AccountKind {
    /// A depository account, such as a checking or savings account.
    #[serde(alias = "depository")]
    Depository,
    /// A credit card account.
    #[serde(alias = "credit_card")]
    CreditCard,
    /// An investment account, such as a brokerage or retirement account.
    #[serde(alias = "investment")]
    Investment,
    /// A property asset, such as real estate or a vehicle.
    #[serde(alias = "property")]
    Property,
    /// A loan or debt account, such as a mortgage or student loan.
    #[serde(alias = "loan")]
    Loan,
    /// Any other type of asset not covered by other kinds.
    #[serde(alias = "other_asset")]
    OtherAsset,
    /// Any other type of liability not covered by other kinds.
    #[serde(alias = "other_liability")]
    OtherLiability,
}

impl std::fmt::Display for AccountKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AccountKind::Depository => "Depository",
            AccountKind::CreditCard => "CreditCard",
            AccountKind::Investment => "Investment",
            AccountKind::Property => "Property",
            AccountKind::Loan => "Loan",
            AccountKind::OtherAsset => "OtherAsset",
            AccountKind::OtherLiability => "OtherLiability",
        };
        write!(f, "{}", s)
    }
}

/// Error returned when parsing an `AccountKind` from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseAccountKindError(String);

impl std::fmt::Display for ParseAccountKindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid account kind: {}", self.0)
    }
}

impl std::error::Error for ParseAccountKindError {}

impl std::str::FromStr for AccountKind {
    type Err = ParseAccountKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Depository" => Ok(AccountKind::Depository),
            "CreditCard" => Ok(AccountKind::CreditCard),
            "Investment" => Ok(AccountKind::Investment),
            "Property" => Ok(AccountKind::Property),
            "Loan" => Ok(AccountKind::Loan),
            "OtherAsset" => Ok(AccountKind::OtherAsset),
            "OtherLiability" => Ok(AccountKind::OtherLiability),
            _ => Err(ParseAccountKindError(s.to_string())),
        }
    }
}

impl TryFrom<&str> for AccountKind {
    type Error = ParseAccountKindError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Account information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Account {
    /// Unique identifier
    pub id: AccountId,
    /// Account name
    pub name: String,
    /// Formatted balance (e.g. "$1,234.56")
    pub balance: String,
    /// Currency code (e.g. "USD")
    pub currency: String,
    /// Account classification (e.g. "asset", "liability")
    pub classification: String,
    /// Account kind
    #[serde(rename = "account_type")]
    pub kind: AccountKind,
}

/// Detailed account information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct AccountDetail {
    /// Unique identifier
    pub id: AccountId,
    /// Account name
    pub name: String,
    /// Formatted balance (e.g. "$1,234.56")
    pub balance: String,
    /// Currency code (e.g. "USD")
    pub currency: String,
    /// Account classification (e.g. "asset", "liability")
    pub classification: String,
    /// Account kind
    #[serde(rename = "account_type")]
    pub kind: AccountKind,
    /// Account subtype (e.g. "checking", "savings")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    /// Name of the financial institution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    /// Domain of the financial institution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_domain: Option<String>,
    /// Additional notes about the account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Whether the account is active
    pub is_active: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Collection of accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct AccountCollection {
    /// List of accounts
    pub accounts: Vec<Account>,
}

/// Request to create a new account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct CreateAccountRequest {
    /// Account data
    pub account: CreateAccountData,
}

/// Data for creating a new account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct CreateAccountData {
    /// Account name
    pub name: String,
    /// Account kind
    #[serde(rename = "account_type")]
    pub kind: AccountKind,
    /// Initial account balance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<Decimal>,
    /// Currency code (defaults to family currency if not provided)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Account subtype (e.g. "checking", "savings")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    /// Name of the financial institution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    /// Domain of the financial institution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_domain: Option<String>,
    /// Additional notes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Type-specific attributes (varies by kind)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accountable_attributes: Option<serde_json::Value>,
}

/// Request to update an existing account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct UpdateAccountRequest {
    /// Account data
    pub account: UpdateAccountData,
}

/// Data for updating an account
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct UpdateAccountData {
    /// Account name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updates the current balance of the account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<Decimal>,
    /// Account subtype
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    /// Name of the financial institution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    /// Domain of the financial institution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_domain: Option<String>,
    /// Additional notes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Type-specific attributes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accountable_attributes: Option<serde_json::Value>,
}
