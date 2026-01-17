use crate::{serde::deserialize_flexible_decimal, types::AccountId};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use url::Url;

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
            Self::Depository => "Depository",
            Self::CreditCard => "CreditCard",
            Self::Investment => "Investment",
            Self::Property => "Property",
            Self::Loan => "Loan",
            Self::OtherAsset => "OtherAsset",
            Self::OtherLiability => "OtherLiability",
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
            "Depository" => Ok(Self::Depository),
            "CreditCard" => Ok(Self::CreditCard),
            "Investment" => Ok(Self::Investment),
            "Property" => Ok(Self::Property),
            "Loan" => Ok(Self::Loan),
            "OtherAsset" => Ok(Self::OtherAsset),
            "OtherLiability" => Ok(Self::OtherLiability),
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
    /// Unformatted balance
    #[serde(deserialize_with = "deserialize_flexible_decimal")]
    pub balance: Decimal,
    /// Currency code (e.g. "USD")
    pub currency: iso_currency::Currency,
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
    /// Unformatted balance
    #[serde(deserialize_with = "deserialize_flexible_decimal")]
    pub balance: Decimal,
    /// Currency code (e.g. "USD")
    pub currency: iso_currency::Currency,
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
pub(crate) struct CreateAccountRequest {
    /// Account data
    pub account: CreateAccountData,
}

/// Data for creating a new account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct CreateAccountData {
    /// Account name
    pub name: String,
    /// Account kind
    #[serde(rename = "accountable_type")]
    pub kind: AccountKind,
    /// Initial account balance
    pub balance: Decimal,
    /// Currency code (defaults to family currency if not provided)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<iso_currency::Currency>,
    /// Name of the financial institution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    /// Domain of the financial institution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_domain: Option<Url>,
    /// Additional notes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Type-specific attributes (required, must match the account kind)
    pub accountable_attributes: AccountableAttributes,
}

/// Request to update an existing account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct UpdateAccountRequest {
    /// Account data
    pub account: UpdateAccountData,
}

/// Data for updating an account
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct UpdateAccountData {
    /// Account name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updates the current balance of the account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<Decimal>,
    /// Name of the financial institution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    /// Domain of the financial institution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_domain: Option<Url>,
    /// Additional notes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Type-specific attributes (optional, must match the account kind if provided)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accountable_attributes: Option<AccountableAttributes>,
}

// ==================== Type-specific Account Attributes ====================

/// Subtype for depository accounts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DepositorySubtype {
    /// Checking account
    Checking,
    /// Savings account
    Savings,
    /// Health Savings Account
    Hsa,
    /// Certificate of Deposit
    Cd,
    /// Money market account
    MoneyMarket,
}

/// Attributes for depository (cash) accounts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DepositoryAttributes {
    /// Account subtype (e.g., checking, savings)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<DepositorySubtype>,
    /// Attributes that should not be overwritten by syncs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked_attributes: Option<JsonValue>,
}

/// Subtype for investment accounts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvestmentSubtype {
    /// Standard brokerage account
    Brokerage,
    /// Pension account
    Pension,
    /// General retirement account
    Retirement,
    /// 401(k) retirement plan
    #[serde(rename = "401k")]
    FourZeroOneK,
    /// Roth 401(k) retirement plan
    #[serde(rename = "roth_401k")]
    RothFourZeroOneK,
    /// 403(b) retirement plan
    #[serde(rename = "403b")]
    FourZeroThreeB,
    /// Thrift Savings Plan
    Tsp,
    /// 529 education savings plan
    #[serde(rename = "529_plan")]
    FiveTwoNinePlan,
    /// Health Savings Account
    Hsa,
    /// Mutual fund account
    MutualFund,
    /// Traditional IRA
    Ira,
    /// Roth IRA
    RothIra,
    /// Angel investment account
    Angel,
}

/// Attributes for investment accounts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct InvestmentAttributes {
    /// Account subtype
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<InvestmentSubtype>,
    /// Attributes that should not be overwritten by syncs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked_attributes: Option<JsonValue>,
}

/// Attributes for cryptocurrency accounts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct CryptoAttributes {
    /// Account subtype (no predefined values)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    /// Attributes that should not be overwritten by syncs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked_attributes: Option<JsonValue>,
}

/// Subtype for property assets
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PropertySubtype {
    /// Single family home
    SingleFamilyHome,
    /// Multi-family home
    MultiFamilyHome,
    /// Condominium
    Condominium,
    /// Townhouse
    Townhouse,
    /// Investment property
    InvestmentProperty,
    /// Second home
    SecondHome,
}

/// Address information for property
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Address {
    /// Address line 1
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// City or locality
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// State or region
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Postal code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Country
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// Attributes for property assets
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct PropertyAttributes {
    /// Property subtype
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<PropertySubtype>,
    /// Year the property was built
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year_built: Option<i32>,
    /// Property area value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_value: Option<i32>,
    /// Property area unit (default: "sqft")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_unit: Option<String>,
    /// Attributes that should not be overwritten by syncs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked_attributes: Option<JsonValue>,
    /// Property address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_attributes: Option<Address>,
}

/// Attributes for vehicle assets
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct VehicleAttributes {
    /// Vehicle year
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    /// Vehicle make (e.g., Toyota)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    /// Vehicle model (e.g., Camry)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Vehicle mileage value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mileage_value: Option<i32>,
    /// Vehicle mileage unit (default: "mi")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mileage_unit: Option<String>,
    /// Vehicle subtype (no predefined values)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    /// Attributes that should not be overwritten by syncs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked_attributes: Option<JsonValue>,
}

/// Attributes for other asset types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct OtherAssetAttributes {
    /// Account subtype (no predefined values)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    /// Attributes that should not be overwritten by syncs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked_attributes: Option<JsonValue>,
}

/// Attributes for credit card liabilities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct CreditCardAttributes {
    /// Credit card subtype (only "credit_card" is predefined)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    /// Available credit amount
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_credit: Option<Decimal>,
    /// Minimum payment amount
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_payment: Option<Decimal>,
    /// Annual Percentage Rate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apr: Option<Decimal>,
    /// Card expiration date
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<DateTime<Utc>>,
    /// Annual fee amount
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annual_fee: Option<Decimal>,
    /// Attributes that should not be overwritten by syncs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked_attributes: Option<JsonValue>,
}

/// Subtype for loan liabilities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoanSubtype {
    /// Mortgage loan
    Mortgage,
    /// Student loan
    Student,
    /// Auto loan
    Auto,
    /// Other loan type
    Other,
}

/// Rate type for loans
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoanRateType {
    /// Fixed interest rate
    Fixed,
    /// Variable interest rate
    Variable,
}

/// Attributes for loan liabilities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct LoanAttributes {
    /// Loan subtype
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<LoanSubtype>,
    /// Interest rate type (fixed or variable)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_type: Option<LoanRateType>,
    /// Interest rate percentage
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interest_rate: Option<Decimal>,
    /// Loan term in months
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term_months: Option<i32>,
    /// Initial loan balance (deprecated - use first valuation instead)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_balance: Option<Decimal>,
    /// Attributes that should not be overwritten by syncs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked_attributes: Option<JsonValue>,
}

/// Attributes for other liability types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct OtherLiabilityAttributes {
    /// Account subtype (no predefined values)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    /// Attributes that should not be overwritten by syncs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked_attributes: Option<JsonValue>,
}

/// Type-specific attributes for different account kinds.
///
/// The enum variant must match the `AccountKind` used when creating the account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountableAttributes {
    /// Depository account attributes
    Depository(DepositoryAttributes),
    /// Investment account attributes
    Investment(InvestmentAttributes),
    /// Cryptocurrency account attributes
    Crypto(CryptoAttributes),
    /// Property asset attributes
    Property(PropertyAttributes),
    /// Vehicle asset attributes (note: API uses "Property" kind with vehicle data)
    Vehicle(VehicleAttributes),
    /// Other asset attributes
    OtherAsset(OtherAssetAttributes),
    /// Credit card liability attributes
    CreditCard(CreditCardAttributes),
    /// Loan liability attributes
    Loan(LoanAttributes),
    /// Other liability attributes
    OtherLiability(OtherLiabilityAttributes),
}

impl AccountableAttributes {
    /// Returns the `AccountKind` that corresponds to these attributes.
    pub const fn kind(&self) -> AccountKind {
        match self {
            Self::Depository(_) => AccountKind::Depository,
            Self::Investment(_) => AccountKind::Investment,
            Self::Crypto(_) => AccountKind::Property, // Crypto uses Property kind
            Self::Property(_) => AccountKind::Property,
            Self::Vehicle(_) => AccountKind::Property, // Vehicle uses Property kind
            Self::OtherAsset(_) => AccountKind::OtherAsset,
            Self::CreditCard(_) => AccountKind::CreditCard,
            Self::Loan(_) => AccountKind::Loan,
            Self::OtherLiability(_) => AccountKind::OtherLiability,
        }
    }
}
