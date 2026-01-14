use crate::models::account::{
    AccountCollection, AccountDetail, AccountKind, CreateAccountData, CreateAccountRequest,
    UpdateAccountData, UpdateAccountRequest,
};
use crate::models::{DeleteResponse, PaginatedResponse};
use crate::types::AccountId;
use crate::{ApiError, error::ApiResult};
use bon::bon;
use reqwest::Method;
use rust_decimal::Decimal;
use std::collections::HashMap;
use url::Url;

use super::SureClient;

const MAX_PER_PAGE: u32 = 100;

#[bon]
impl SureClient {
    /// List accounts
    ///
    /// Retrieves a paginated list of accounts.
    ///
    /// # Arguments
    /// * `page` - Page number (default: 1)
    /// * `per_page` - Items per page (default: 25, max: 100)
    ///
    /// # Returns
    /// A paginated response containing accounts and pagination metadata.
    ///
    /// # Errors
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// // Use defaults (page 1, per_page 25)
    /// let response = client.get_accounts().call().await?;
    ///
    /// for account in response.items.accounts {
    ///     println!("{}: {:?} {:?}", account.name, account.balance, account.currency);
    /// }
    ///
    /// // Or customize parameters using the builder
    /// let response = client.get_accounts().page(2).per_page(50).call().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[builder]
    pub async fn get_accounts(
        &self,
        #[builder(default = 1)] page: u32,
        #[builder(default = 25)] per_page: u32,
    ) -> ApiResult<PaginatedResponse<AccountCollection>> {
        if per_page > MAX_PER_PAGE {
            return Err(ApiError::InvalidParameter(format!(
                "per_page cannot exceed {MAX_PER_PAGE}",
            )));
        }

        let mut query_params = HashMap::new();

        query_params.insert("page", page.to_string());
        query_params.insert("per_page", per_page.to_string());

        self.execute_request(Method::GET, "/api/v1/accounts", Some(&query_params), None)
            .await
    }

    /// Get a specific account by ID
    ///
    /// Retrieves detailed information about a single account.
    ///
    /// # Arguments
    /// * `id` - The account ID to retrieve
    ///
    /// # Returns
    /// Detailed account information.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the account doesn't exist.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken, AccountId};
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let account_id = AccountId::new(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap());
    /// let account = client.get_account(&account_id).await?;
    ///
    /// println!("Account: {}", account.name);
    /// println!("Balance: {}", account.balance);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_account(&self, id: &AccountId) -> ApiResult<AccountDetail> {
        self.execute_request(Method::GET, &format!("/api/v1/accounts/{}", id), None, None)
            .await
    }
}

#[bon]
impl SureClient {
    /// Create a new account
    ///
    /// Creates a new account with the specified details.
    ///
    /// # Arguments
    /// * `request` - The account creation request containing all required fields
    ///
    /// # Returns
    /// The newly created account with full details.
    ///
    /// # Errors
    /// Returns `ApiError::ValidationError` if required fields are missing or invalid.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    /// use sure_client_rs::models::account::AccountKind;
    /// use rust_decimal::Decimal;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let account = client.create_account()
    ///     .name("Checking Account".to_string())
    ///     .kind(AccountKind::Depository)
    ///     .balance(Decimal::new(100000, 2)) // $1,000.00
    ///     .currency("USD".to_string())
    ///     .subtype("checking".to_string())
    ///     .institution_name("Bank of Example".to_string())
    ///     .call()
    ///     .await?;
    ///
    /// println!("Created account: {}", account.name);
    /// # Ok(())
    /// # }
    /// ```
    #[builder]
    pub async fn create_account(
        &self,
        name: String,
        kind: AccountKind,
        balance: Decimal,
        currency: Option<String>,
        subtype: Option<String>,
        institution_name: Option<String>,
        institution_domain: Option<Url>,
        notes: Option<String>,
        accountable_attributes: Option<serde_json::Value>,
    ) -> ApiResult<AccountDetail> {
        let request = CreateAccountRequest {
            account: CreateAccountData {
                name,
                kind,
                balance,
                currency,
                subtype,
                institution_name,
                institution_domain,
                notes,
                accountable_attributes,
            },
        };

        self.execute_request(
            Method::POST,
            "/api/v1/accounts",
            None,
            Some(serde_json::to_string(&request)?),
        )
        .await
    }

    /// Update an account
    ///
    /// Updates an existing account with new values. Only fields provided in the
    /// request will be updated.
    ///
    /// # Arguments
    /// * `id` - The account ID to update
    /// * `request` - The account update request containing fields to update
    ///
    /// # Returns
    /// The updated account.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the account doesn't exist.
    /// Returns `ApiError::ValidationError` if the provided values are invalid.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken, AccountId};
    /// use rust_decimal::Decimal;
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let account_id = AccountId::new(Uuid::new_v4());
    ///
    /// let account = client.update_account()
    ///     .id(&account_id)
    ///     .name("Updated Account Name".to_string())
    ///     .balance(Decimal::new(150000, 2)) // $1,500.00
    ///     .call()
    ///     .await?;
    ///
    /// println!("Updated account: {}", account.name);
    /// # Ok(())
    /// # }
    /// ```
    #[builder]
    pub async fn update_account(
        &self,
        id: &AccountId,
        name: Option<String>,
        balance: Option<Decimal>,
        subtype: Option<String>,
        institution_name: Option<String>,
        institution_domain: Option<Url>,
        notes: Option<String>,
        accountable_attributes: Option<serde_json::Value>,
    ) -> ApiResult<AccountDetail> {
        let request = UpdateAccountRequest {
            account: UpdateAccountData {
                name,
                balance,
                subtype,
                institution_name,
                institution_domain,
                notes,
                accountable_attributes,
            },
        };

        self.execute_request(
            Method::PATCH,
            &format!("/api/v1/accounts/{}", id),
            None,
            Some(serde_json::to_string(&request)?),
        )
        .await
    }

    /// Delete an account
    ///
    /// Permanently deletes an account.
    ///
    /// # Arguments
    /// * `id` - The account ID to delete
    ///
    /// # Returns
    /// A confirmation message.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the account doesn't exist.
    /// Returns `ApiError::ValidationError` if the account cannot be deleted (e.g., linked account).
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken, AccountId};
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let account_id = AccountId::new(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap());
    /// let response = client.delete_account(&account_id).await?;
    ///
    /// println!("Deleted: {}", response.message);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_account(&self, id: &AccountId) -> ApiResult<DeleteResponse> {
        self.execute_request(
            Method::DELETE,
            &format!("/api/v1/accounts/{}", id),
            None,
            None,
        )
        .await
    }
}
