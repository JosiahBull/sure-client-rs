//! # Sure API Client
//!
//! A type-safe Rust client for the Sure API, providing comprehensive access to
//! financial data including transactions, categories, accounts, chat functionality, and authentication.
//!
//! ## Features
//!
//! - **Type-safe API**: Compile-time guarantees prevent common errors
//! - **Comprehensive error handling**: Detailed, actionable error types
//! - **Full async/await support**: Built on tokio and reqwest
//! - **Complete API coverage**: Accounts, transactions, categories, chats, authentication, sync, and usage
//! - **UUID-based identifiers**: Type-safe wrappers for all IDs
//! - **Pagination support**: Built-in pagination handling for list endpoints
//!
//! ## Quick Start
//!
//! ### Using API Key Authentication
//! ```no_run
//! use sure_client_rs::{SureClient, Auth};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client with your API key
//!     let client = SureClient::new(
//!         reqwest::Client::new(),
//!         Auth::api_key("your_api_key"),
//!         "http://localhost:3000".to_string().parse().unwrap(),
//!     );
//!
//!     // List all categories
//!     let categories = client.get_categories().call().await?;
//!     for category in categories.items.categories {
//!         println!("{}: {}", category.name, category.classification);
//!     }
//!
//!     // List recent transactions
//!     let transactions = client.get_transactions()
//!         .page(1)
//!         .per_page(25)
//!         .call()
//!         .await?;
//!
//!     for transaction in transactions.items.transactions {
//!         println!("{}: {} {}",
//!             transaction.name,
//!             transaction.amount,
//!             transaction.currency
//!         );
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Using Bearer Token Authentication
//! ```no_run
//! use sure_client_rs::{SureClient, Auth};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client with a JWT bearer token
//!     let client = SureClient::new(
//!         reqwest::Client::new(),
//!         Auth::bearer("your_jwt_token"),
//!         "http://localhost:3000".to_string().parse().unwrap(),
//!     );
//!
//!     let categories = client.get_categories().call().await?;
//!     for category in categories.items.categories {
//!         println!("{}: {}", category.name, category.classification);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Authentication
//!
//! The Sure API supports two authentication methods:
//!
//! ### API Key Authentication (X-Api-Key header)
//! ```no_run
//! use sure_client_rs::{SureClient, Auth};
//!
//! let client = SureClient::new(
//!     reqwest::Client::new(),
//!     Auth::api_key("your_api_key"),
//!     "http://localhost:3000".to_string().parse().unwrap(),
//! );
//! ```
//!
//! ### Bearer Token Authentication (Authorization header)
//! ```no_run
//! use sure_client_rs::{SureClient, Auth};
//!
//! let client = SureClient::new(
//!     reqwest::Client::new(),
//!     Auth::bearer("your_jwt_token"),
//!     "http://localhost:3000".to_string().parse().unwrap(),
//! );
//! ```
//!
//! ## Working with Categories
//!
//! ```no_run
//! use sure_client_rs::{SureClient, BearerToken, CategoryId};
//! use sure_client_rs::models::category::Classification;
//! use uuid::Uuid;
//!
//! # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
//! // Get expense categories
//! let categories = client.get_categories()
//!     .page(1)
//!     .per_page(25)
//!     .classification(Classification::Expense)
//!     .call()
//!     .await?;
//!
//! // Get a specific category
//! let category_id = CategoryId::new(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap());
//! let category = client.get_category(&category_id).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Working with Transactions
//!
//! ```no_run
//! use sure_client_rs::{SureClient, BearerToken, AccountId};
//! use chrono::{DateTime, TimeZone, Utc};
//! use rust_decimal::Decimal;
//! use uuid::Uuid;
//!
//! # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
//! // Create a transaction using the builder pattern
//! let transaction = client.create_transaction()
//!     .account_id(AccountId::new(Uuid::new_v4()))
//!     .date(Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap())
//!     .amount(Decimal::new(4250, 2)) // $42.50
//!     .name("Grocery Store".to_string())
//!     .currency(iso_currency::Currency::USD)
//!     .call()
//!     .await?;
//!
//! // Update a transaction
//! let updated = client.update_transaction()
//!     .id(&transaction.id)
//!     .notes("Updated notes".to_string())
//!     .call()
//!     .await?;
//!
//! // Delete a transaction
//! let response = client.delete_transaction(&transaction.id).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling
//!
//! The client uses a comprehensive error type that covers both API-level and
//! client-level errors:
//!
//! ```no_run
//! use sure_client_rs::{SureClient, ApiError};
//!
//! # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
//! match client.get_categories().call().await {
//!     Ok(categories) => {
//!         // Handle success
//!     }
//!     Err(ApiError::Unauthorized { message }) => {
//!         // Handle authentication error
//!     }
//!     Err(ApiError::NotFound { message }) => {
//!         // Handle not found error
//!     }
//!     Err(ApiError::RateLimited { message }) => {
//!         // Handle rate limiting
//!     }
//!     Err(e) => {
//!         // Handle other errors
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Development and Testing
//!
//! For local development, you can configure the client to use a different base URL:
//!
//! ```no_run
//! use sure_client_rs::{SureClient, BearerToken};
//!
//! let client = SureClient::new(
//!     reqwest::Client::new(),
//!     BearerToken::new("your_api_key"),
//!     "http://localhost:3000".to_string().parse().unwrap(),
//! );
//! ```

// Module declarations
mod client;
mod error;
pub mod models;
pub(crate) mod serde;
mod types;

// Public re-exports
pub use client::SureClient;
pub use error::{ApiError, ApiResult};
pub use types::{
    AccountId, ApiKey, Auth, BearerToken, CategoryId, MerchantId, TagId, TransactionId,
};
