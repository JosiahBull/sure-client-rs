use crate::ApiError;
use crate::error::ApiResult;
use crate::models::transaction::{
    CreateTransactionData, CreateTransactionRequest, Transaction, TransactionCollection,
    TransactionNature, TransactionType, UpdateTransactionData, UpdateTransactionRequest,
};
use crate::models::{DeleteResponse, PaginatedResponse};
use crate::types::{AccountId, CategoryId, MerchantId, TagId, TransactionId};
use bon::bon;
use chrono::NaiveDate;
use reqwest::Method;
use rust_decimal::Decimal;
use std::collections::HashMap;

use super::SureClient;

const MAX_PER_PAGE: u32 = 100;

#[bon]
impl SureClient {
    /// List transactions with optional filters
    ///
    /// Retrieves a paginated list of transactions. Results can be filtered by various criteria
    /// including date range, amount, account, category, merchant, tags, and search text.
    ///
    /// # Arguments
    /// * `page` - Page number (default: 1)
    /// * `per_page` - Items per page (default: 25, max: 100)
    /// * `account_id` - Filter by single account ID
    /// * `account_ids` - Filter by multiple account IDs
    /// * `category_id` - Filter by single category ID
    /// * `category_ids` - Filter by multiple category IDs
    /// * `merchant_id` - Filter by single merchant ID
    /// * `merchant_ids` - Filter by multiple merchant IDs
    /// * `tag_ids` - Filter by tag IDs
    /// * `start_date` - Filter transactions from this date (inclusive)
    /// * `end_date` - Filter transactions until this date (inclusive)
    /// * `min_amount` - Filter by minimum amount
    /// * `max_amount` - Filter by maximum amount
    /// * `transaction_type` - Filter by transaction type (income or expense)
    /// * `search` - Search by name, notes, or merchant name
    ///
    /// # Returns
    /// A paginated response containing transactions and pagination metadata.
    ///
    /// # Errors
    /// Returns `ApiError::Unauthorized` if the bearer token is invalid or expired.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    /// use chrono::NaiveDate;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// // Use defaults (page 1, per_page 25, no filters)
    /// let response = client.get_transactions().call().await?;
    ///
    /// // Or customize with builder
    /// let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    /// let response = client.get_transactions()
    ///     .page(2)
    ///     .per_page(50)
    ///     .start_date(&start)
    ///     .end_date(&end)
    ///     .search("coffee")
    ///     .call()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[builder]
    pub async fn get_transactions(
        &self,
        #[builder(default = 1)] page: u32,
        #[builder(default = 25)] per_page: u32,
        account_id: Option<&AccountId>,
        account_ids: Option<&[AccountId]>,
        category_id: Option<&CategoryId>,
        category_ids: Option<&[CategoryId]>,
        merchant_id: Option<&MerchantId>,
        merchant_ids: Option<&[MerchantId]>,
        tag_ids: Option<&[TagId]>,
        start_date: Option<&NaiveDate>,
        end_date: Option<&NaiveDate>,
        min_amount: Option<Decimal>,
        max_amount: Option<Decimal>,
        transaction_type: Option<TransactionType>,
        search: Option<&str>,
    ) -> ApiResult<PaginatedResponse<TransactionCollection>> {
        if per_page > MAX_PER_PAGE {
            return Err(ApiError::InvalidParameter(format!(
                "per_page cannot exceed {MAX_PER_PAGE}",
            )));
        }

        let mut query_params = HashMap::new();

        query_params.insert("page", page.to_string());
        query_params.insert("per_page", per_page.to_string());

        if let Some(account_id) = account_id {
            query_params.insert("account_id", account_id.to_string());
        }

        if let Some(account_ids) = account_ids {
            for id in account_ids {
                query_params.insert("account_ids[]", id.to_string());
            }
        }

        if let Some(category_id) = category_id {
            query_params.insert("category_id", category_id.to_string());
        }

        if let Some(category_ids) = category_ids {
            for id in category_ids {
                query_params.insert("category_ids[]", id.to_string());
            }
        }

        if let Some(merchant_id) = merchant_id {
            query_params.insert("merchant_id", merchant_id.to_string());
        }

        if let Some(merchant_ids) = merchant_ids {
            for id in merchant_ids {
                query_params.insert("merchant_ids[]", id.to_string());
            }
        }

        if let Some(tag_ids) = tag_ids {
            for id in tag_ids {
                query_params.insert("tag_ids[]", id.to_string());
            }
        }

        if let Some(start_date) = start_date {
            query_params.insert("start_date", start_date.format("%Y-%m-%d").to_string());
        }

        if let Some(end_date) = end_date {
            query_params.insert("end_date", end_date.format("%Y-%m-%d").to_string());
        }

        if let Some(min_amount) = min_amount {
            query_params.insert("min_amount", min_amount.to_string());
        }

        if let Some(max_amount) = max_amount {
            query_params.insert("max_amount", max_amount.to_string());
        }

        if let Some(transaction_type) = transaction_type {
            query_params.insert("type", transaction_type.to_string());
        }

        if let Some(search) = search {
            query_params.insert("search", search.to_string());
        }

        self.execute_request(
            Method::GET,
            "/api/v1/transactions",
            Some(&query_params),
            None,
        )
        .await
    }

    /// Create a new transaction
    ///
    /// Creates a new transaction with the specified details.
    ///
    /// # Arguments
    /// * `account_id` - Account ID (required)
    /// * `date` - Transaction date (required)
    /// * `amount` - Transaction amount (required)
    /// * `name` - Transaction name/description (required)
    /// * `notes` - Additional notes
    /// * `currency` - Currency code (defaults to family currency)
    /// * `category_id` - Category ID
    /// * `merchant_id` - Merchant ID
    /// * `nature` - Transaction nature (determines sign)
    /// * `tag_ids` - Tag IDs
    ///
    /// # Returns
    /// The newly created transaction.
    ///
    /// # Errors
    /// Returns `ApiError::ValidationError` if required fields are missing or invalid.
    /// Returns `ApiError::Unauthorized` if the bearer token is invalid or expired.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken, AccountId};
    /// use chrono::NaiveDate;
    /// use rust_decimal::Decimal;
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let transaction = client.create_transaction()
    ///     .account_id(AccountId::new(Uuid::new_v4()))
    ///     .date(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
    ///     .amount(Decimal::new(4250, 2)) // $42.50
    ///     .name("Grocery Store".to_string())
    ///     .currency("USD".to_string())
    ///     .call()
    ///     .await?;
    ///
    /// println!("Created transaction: {}", transaction.id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[builder]
    pub async fn create_transaction(
        &self,
        account_id: AccountId,
        date: NaiveDate,
        amount: Decimal,
        name: String,
        notes: Option<String>,
        currency: Option<String>,
        category_id: Option<CategoryId>,
        merchant_id: Option<MerchantId>,
        nature: Option<TransactionNature>,
        tag_ids: Option<Vec<TagId>>,
    ) -> ApiResult<Transaction> {
        let request = CreateTransactionRequest {
            transaction: CreateTransactionData {
                account_id,
                date,
                amount,
                name,
                notes,
                currency,
                category_id,
                merchant_id,
                nature,
                tag_ids,
            },
        };

        self.execute_request(
            Method::POST,
            "/api/v1/transactions",
            None,
            Some(serde_json::to_string(&request)?),
        )
        .await
    }

    /// Get a specific transaction by ID
    ///
    /// Retrieves detailed information about a single transaction.
    ///
    /// # Arguments
    /// * `id` - The transaction ID to retrieve
    ///
    /// # Returns
    /// Detailed transaction information.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the transaction doesn't exist.
    /// Returns `ApiError::Unauthorized` if the bearer token is invalid or expired.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken, TransactionId};
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let transaction_id = TransactionId::new(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap());
    /// let transaction = client.get_transaction(&transaction_id).await?;
    ///
    /// println!("Transaction: {}", transaction.name);
    /// println!("Amount: {} {}", transaction.amount, transaction.currency);
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub async fn get_transaction(&self, id: &TransactionId) -> ApiResult<Transaction> {
        self.execute_request(
            Method::GET,
            &format!("/api/v1/transactions/{}", id),
            None,
            None,
        )
        .await
    }

    /// Update a transaction
    ///
    /// Updates an existing transaction with new values. Only fields provided will be updated.
    ///
    /// # Arguments
    /// * `id` - The transaction ID to update
    /// * `date` - Transaction date
    /// * `amount` - Transaction amount
    /// * `name` - Transaction name/description
    /// * `notes` - Additional notes
    /// * `currency` - Currency code
    /// * `category_id` - Category ID
    /// * `merchant_id` - Merchant ID
    /// * `nature` - Transaction nature
    /// * `tag_ids` - Tag IDs
    ///
    /// # Returns
    /// The updated transaction.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the transaction doesn't exist.
    /// Returns `ApiError::ValidationError` if the provided values are invalid.
    /// Returns `ApiError::Unauthorized` if the bearer token is invalid or expired.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken, TransactionId};
    /// use rust_decimal::Decimal;
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let transaction_id = TransactionId::new(Uuid::new_v4());
    ///
    /// let transaction = client.update_transaction()
    ///     .id(&transaction_id)
    ///     .amount(Decimal::new(5000, 2)) // Update to $50.00
    ///     .notes("Updated notes".to_string())
    ///     .call()
    ///     .await?;
    ///
    /// println!("Updated transaction: {}", transaction.id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[builder]
    pub async fn update_transaction(
        &self,
        id: &TransactionId,
        date: Option<NaiveDate>,
        amount: Option<Decimal>,
        name: Option<String>,
        notes: Option<String>,
        currency: Option<String>,
        category_id: Option<CategoryId>,
        merchant_id: Option<MerchantId>,
        nature: Option<TransactionNature>,
        tag_ids: Option<Vec<TagId>>,
    ) -> ApiResult<Transaction> {
        let request = UpdateTransactionRequest {
            transaction: UpdateTransactionData {
                date,
                amount,
                name,
                notes,
                currency,
                category_id,
                merchant_id,
                nature,
                tag_ids,
            },
        };

        self.execute_request(
            Method::PATCH,
            &format!("/api/v1/transactions/{}", id),
            None,
            Some(serde_json::to_string(&request)?),
        )
        .await
    }

    /// Delete a transaction
    ///
    /// Permanently deletes a transaction.
    ///
    /// # Arguments
    /// * `id` - The transaction ID to delete
    ///
    /// # Returns
    /// A confirmation message.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the transaction doesn't exist.
    /// Returns `ApiError::Unauthorized` if the bearer token is invalid or expired.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken, TransactionId};
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let transaction_id = TransactionId::new(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap());
    /// let response = client.delete_transaction(&transaction_id).await?;
    ///
    /// println!("Deleted: {}", response.message);
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub async fn delete_transaction(&self, id: &TransactionId) -> ApiResult<DeleteResponse> {
        self.execute_request(
            Method::DELETE,
            &format!("/api/v1/transactions/{}", id),
            None,
            None,
        )
        .await
    }
}
