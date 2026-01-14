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

/// Basic category information
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
    /// Classification (income or expense)
    pub classification: Classification,
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
pub struct CreateCategoryRequest {
    /// Category data
    pub category: CreateCategoryData,
}

/// Data for creating a new category
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct CreateCategoryData {
    /// Category name
    pub name: String,
    /// Classification (income or expense)
    pub classification: Classification,
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
pub struct UpdateCategoryRequest {
    /// Category data
    pub category: UpdateCategoryData,
}

/// Data for updating a category
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct UpdateCategoryData {
    /// Category name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Classification (income or expense)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<Classification>,
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
