use crate::models::account::{
    AccountCollection, AccountDetail, AccountableAttributes, CreateAccountData,
    CreateAccountRequest, UpdateAccountData, UpdateAccountRequest,
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
    /// Creates a new account with type-specific attributes. The account kind is
    /// automatically derived from the attributes you provide.
    ///
    /// # Arguments
    /// * `name` - The account name
    /// * `balance` - The initial account balance
    /// * `attributes` - Type-specific attributes that determine the account kind
    /// * `currency` - Optional currency code (defaults to family currency if not provided)
    /// * `institution_name` - Optional name of the financial institution
    /// * `institution_domain` - Optional domain of the financial institution
    /// * `notes` - Optional additional notes
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
    /// use sure_client_rs::models::account::{
    ///     AccountableAttributes, DepositoryAttributes, DepositorySubtype
    /// };
    /// use rust_decimal::Decimal;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let account = client.create_account()
    ///     .name("Checking Account".to_string())
    ///     .balance(Decimal::new(100000, 2)) // $1,000.00
    ///     .attributes(AccountableAttributes::Depository(DepositoryAttributes {
    ///         subtype: Some(DepositorySubtype::Checking),
    ///         locked_attributes: None,
    ///     }))
    ///     .currency(iso_currency::Currency::USD)
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
        balance: Decimal,
        attributes: AccountableAttributes,
        currency: Option<iso_currency::Currency>,
        institution_name: Option<String>,
        institution_domain: Option<Url>,
        notes: Option<String>,
    ) -> ApiResult<AccountDetail> {
        // Derive the account kind from the attributes
        let kind = attributes.kind();

        let request = CreateAccountRequest {
            account: CreateAccountData {
                name,
                kind,
                balance,
                currency,
                institution_name,
                institution_domain,
                notes,
                accountable_attributes: attributes,
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
    /// request will be updated. If updating attributes, the entire attributes object
    /// must be provided as it replaces the existing attributes.
    ///
    /// # Arguments
    /// * `id` - The account ID to update
    /// * `name` - Optional new account name
    /// * `balance` - Optional new balance
    /// * `institution_name` - Optional new institution name
    /// * `institution_domain` - Optional new institution domain
    /// * `notes` - Optional new notes
    /// * `attributes` - Optional new account-specific attributes (replaces existing)
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
    /// use sure_client_rs::models::account::{
    ///     AccountableAttributes, DepositoryAttributes, DepositorySubtype
    /// };
    /// use rust_decimal::Decimal;
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let account_id = AccountId::new(Uuid::new_v4());
    ///
    /// // Update just the name and balance
    /// let account = client.update_account()
    ///     .id(&account_id)
    ///     .name("Updated Account Name".to_string())
    ///     .balance(Decimal::new(150000, 2)) // $1,500.00
    ///     .call()
    ///     .await?;
    ///
    /// // Update attributes
    /// let updated = client.update_account()
    ///     .id(&account_id)
    ///     .attributes(AccountableAttributes::Depository(DepositoryAttributes {
    ///         subtype: Some(DepositorySubtype::Savings),
    ///         locked_attributes: None,
    ///     }))
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
        institution_name: Option<String>,
        institution_domain: Option<Url>,
        notes: Option<String>,
        attributes: Option<AccountableAttributes>,
    ) -> ApiResult<AccountDetail> {
        let request = UpdateAccountRequest {
            account: UpdateAccountData {
                name,
                balance,
                institution_name,
                institution_domain,
                notes,
                accountable_attributes: attributes,
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
