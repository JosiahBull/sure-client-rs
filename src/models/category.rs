use crate::types::CategoryId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Classification of a category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Classification {
    /// Income category
    Income,
    /// Expense category
    Expense,
}

impl fmt::Display for Classification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Income => write!(f, "income"),
            Self::Expense => write!(f, "expense"),
        }
    }
}

/// Error returned when parsing a `Classification` from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseClassificationError(String);

impl fmt::Display for ParseClassificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid classification: {}", self.0)
    }
}

impl std::error::Error for ParseClassificationError {}

impl std::str::FromStr for Classification {
    type Err = ParseClassificationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "income" => Ok(Self::Income),
            "expense" => Ok(Self::Expense),
            _ => Err(ParseClassificationError(s.to_string())),
        }
    }
}

impl TryFrom<&str> for Classification {
    type Error = ParseClassificationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for Classification {
    type Error = ParseClassificationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Basic category information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Category {
    /// Unique identifier
    pub id: CategoryId,
    /// Category name
    pub name: String,
    /// Color for UI display (hex code)
    pub color: String,
    /// Icon identifier
    pub icon: String,
}

/// Parent category reference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct CategoryParent {
    /// Parent category ID
    pub id: CategoryId,
    /// Parent category name
    pub name: String,
}

/// Detailed category information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct CategoryDetail {
    /// Unique identifier
    pub id: CategoryId,
    /// Category name
    pub name: String,
    /// Color for UI display (hex code)
    pub color: String,
    /// Icon identifier
    pub icon: String,
    /// Parent category (if this is a subcategory)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<CategoryParent>,
    /// Number of subcategories
    pub subcategories_count: u32,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// Legacy classification ("income" / "expense").
    ///
    /// Upstream Sure removed this attribute (the column was renamed to
    /// `classification_unused`); newer responses omit the key entirely. Older
    /// deployments still render it through their `_category.json.jbuilder`
    /// partial, so it is accepted here as an optional field for compatibility
    /// and is `None` when the server does not provide it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
}

/// Collection of categories with pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct CategoryCollection {
    /// List of categories
    pub categories: Vec<CategoryDetail>,
}

/// Request to create a new category
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct CreateCategoryRequest {
    /// Category data
    pub category: CreateCategoryData,
}

/// Data for creating a new category
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct CreateCategoryData {
    /// Category name
    pub name: String,
    /// Color for UI display (hex code)
    pub color: String,
    /// Lucide icon name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lucide_icon: Option<String>,
    /// Parent category ID for subcategories
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<CategoryId>,
}

/// Request to update an existing category
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct UpdateCategoryRequest {
    /// Category data
    pub category: UpdateCategoryData,
}

/// Data for updating a category
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct UpdateCategoryData {
    /// Category name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Color for UI display (hex code)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Lucide icon name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lucide_icon: Option<String>,
    /// Parent category ID for subcategories
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<CategoryId>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// New-shape Sure responses (post merged-upstream view changes) omit
    /// `classification`. The struct's `#[serde(default)]` should accept it as
    /// `None`.
    #[test]
    fn deserializes_new_shape_without_classification() {
        let json = r##"{
            "id": "65588a1e-8a33-4dfb-9ea4-1aa7c4a6d855",
            "name": "Groceries",
            "color": "#fd7f6f",
            "icon": "shapes",
            "parent": null,
            "subcategories_count": 0,
            "created_at": "2026-05-04T10:10:12Z",
            "updated_at": "2026-05-04T10:10:12Z"
        }"##;
        let detail: CategoryDetail = serde_json::from_str(json).expect("deserialise new shape");
        assert_eq!(detail.classification, None);
        assert_eq!(detail.name, "Groceries");
    }

    /// Older Sure deployments still emit `classification` on the category
    /// JSON. We must accept it without erroring under strict mode.
    #[test]
    fn deserializes_legacy_shape_with_classification() {
        let json = r##"{
            "id": "65588a1e-8a33-4dfb-9ea4-1aa7c4a6d855",
            "name": "Groceries",
            "classification": "expense",
            "color": "#fd7f6f",
            "icon": "shapes",
            "parent": null,
            "subcategories_count": 0,
            "created_at": "2026-05-04T10:10:12Z",
            "updated_at": "2026-05-04T10:10:12Z"
        }"##;
        let detail: CategoryDetail = serde_json::from_str(json).expect("deserialise legacy shape");
        assert_eq!(detail.classification.as_deref(), Some("expense"));
    }
}
