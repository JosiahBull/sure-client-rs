use crate::error::ApiResult;
use crate::models::valuation::{
    CreateValuationData, CreateValuationRequest, UpdateValuationData, UpdateValuationRequest,
    Valuation,
};
use crate::types::{AccountId, ValuationId};
use bon::bon;
use chrono::NaiveDate;
use reqwest::Method;
use rust_decimal::Decimal;

use super::SureClient;

#[bon]
impl SureClient {
    /// Create a new valuation entry for an account.
    ///
    /// Valuations are most commonly used to record point-in-time balances for
    /// asset accounts whose value drifts over time, such as properties, vehicles,
    /// or investment portfolios that are not synced from a brokerage. Each
    /// valuation appears as a `reconciliation` entry on the account's timeline.
    ///
    /// # Arguments
    /// * `account_id` - The account the valuation applies to (required).
    /// * `amount` - The valuation amount (required).
    /// * `date` - The valuation date (required).
    /// * `notes` - Optional notes attached to the entry.
    ///
    /// # Returns
    /// The created valuation entry.
    ///
    /// # Errors
    /// Returns `ApiError::ValidationError` if required fields are missing or invalid.
    /// Returns `ApiError::Unauthorized` if the credentials are missing/invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, AccountId};
    /// use chrono::NaiveDate;
    /// use rust_decimal::Decimal;
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let account_id = AccountId::new(Uuid::new_v4());
    /// let valuation = client.create_valuation()
    ///     .account_id(account_id)
    ///     .amount(Decimal::new(77000000, 2)) // $770,000.00
    ///     .date(NaiveDate::from_ymd_opt(2025, 12, 12).unwrap())
    ///     .call()
    ///     .await?;
    /// println!("Created valuation: {} on {}", valuation.amount, valuation.date);
    /// # Ok(())
    /// # }
    /// ```
    #[builder]
    pub async fn create_valuation(
        &self,
        account_id: AccountId,
        amount: Decimal,
        date: NaiveDate,
        notes: Option<String>,
    ) -> ApiResult<Valuation> {
        let request = CreateValuationRequest {
            valuation: CreateValuationData {
                account_id,
                amount,
                date,
                notes,
            },
        };

        self.execute_request(
            Method::POST,
            "/api/v1/valuations",
            None,
            Some(serde_json::to_string(&request)?),
        )
        .await
    }

    /// Get a specific valuation by ID.
    ///
    /// # Arguments
    /// * `id` - The valuation ID to retrieve.
    pub async fn get_valuation(&self, id: &ValuationId) -> ApiResult<Valuation> {
        self.execute_request(
            Method::GET,
            &format!("/api/v1/valuations/{}", id),
            None,
            None,
        )
        .await
    }

    /// Update an existing valuation entry.
    ///
    /// The Sure API requires both `amount` and `date` to be supplied together
    /// when changing the underlying reconciliation; `notes` may be updated
    /// independently.
    ///
    /// # Arguments
    /// * `id` - The valuation ID to update.
    /// * `amount` - New amount (must accompany `date` if either is set).
    /// * `date` - New date (must accompany `amount` if either is set).
    /// * `notes` - New notes.
    #[builder]
    pub async fn update_valuation(
        &self,
        id: &ValuationId,
        amount: Option<Decimal>,
        date: Option<NaiveDate>,
        notes: Option<String>,
    ) -> ApiResult<Valuation> {
        let request = UpdateValuationRequest {
            valuation: UpdateValuationData {
                amount,
                date,
                notes,
            },
        };

        self.execute_request(
            Method::PATCH,
            &format!("/api/v1/valuations/{}", id),
            None,
            Some(serde_json::to_string(&request)?),
        )
        .await
    }
}
