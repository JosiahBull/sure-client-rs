use crate::ApiError;
use crate::error::ApiResult;
use crate::models::PaginatedResponse;
use crate::models::chat::{
    ChatCollection, ChatDetail, CreateChatRequest, CreateMessageRequest, MessageResponse,
    RetryResponse, UpdateChatRequest,
};
use bon::bon;
use reqwest::Method;
use std::collections::HashMap;
use uuid::Uuid;

use super::SureClient;

const MAX_PER_PAGE: u32 = 100;

#[bon]
impl SureClient {
    /// List chats
    ///
    /// Retrieves a paginated list of chats.
    ///
    /// # Arguments
    /// * `page` - Page number (default: 1)
    /// * `per_page` - Items per page (default: 25, max: 100)
    ///
    /// # Returns
    /// A paginated response containing chats and pagination metadata.
    ///
    /// # Errors
    /// Returns `ApiError::Forbidden` if AI features are disabled.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// // Use defaults (page 1, per_page 25)
    /// let response = client.get_chats().call().await?;
    ///
    /// for chat in response.items.chats {
    ///     println!("Chat: {} ({} messages)", chat.title, chat.message_count);
    /// }
    ///
    /// // Or customize parameters using the builder
    /// let response = client.get_chats().page(2).per_page(50).call().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[builder]
    pub async fn get_chats(
        &self,
        #[builder(default = 1)] page: u32,
        #[builder(default = 25)] per_page: u32,
    ) -> ApiResult<PaginatedResponse<ChatCollection>> {
        if per_page > MAX_PER_PAGE {
            return Err(ApiError::InvalidParameter(format!(
                "per_page cannot exceed {MAX_PER_PAGE}",
            )));
        }

        let mut query_params = HashMap::new();

        query_params.insert("page", page.to_string());
        query_params.insert("per_page", per_page.to_string());

        self.execute_request(Method::GET, "/api/v1/chats", Some(&query_params), None)
            .await
    }

    /// Create a new chat
    ///
    /// Creates a new chat with an optional initial message.
    ///
    /// # Arguments
    /// * `title` - Chat title (required)
    /// * `message` - Optional initial message
    /// * `model` - Optional OpenAI model identifier
    ///
    /// # Returns
    /// Detailed information about the created chat.
    ///
    /// # Errors
    /// Returns `ApiError::ValidationError` if validation fails.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let chat = client.create_chat()
    ///     .title("Monthly budget review".to_string())
    ///     .message("Help me analyze my spending".to_string())
    ///     .call()
    ///     .await?;
    ///
    /// println!("Created chat: {}", chat.title);
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[builder]
    pub async fn create_chat(
        &self,
        title: String,
        message: Option<String>,
        model: Option<String>,
    ) -> ApiResult<ChatDetail> {
        let request = CreateChatRequest {
            title,
            message,
            model,
        };

        self.execute_request(
            Method::POST,
            "/api/v1/chats",
            None,
            Some(serde_json::to_string(&request)?),
        )
        .await
    }

    /// Get a specific chat
    ///
    /// Retrieves detailed information about a chat including its messages.
    ///
    /// # Arguments
    /// * `id` - The chat ID to retrieve
    ///
    /// # Returns
    /// Detailed chat information including messages.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the chat doesn't exist.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let chat_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    /// let chat = client.get_chat(&chat_id).await?;
    ///
    /// println!("Chat: {} with {} messages", chat.title, chat.messages.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub async fn get_chat(&self, id: &Uuid) -> ApiResult<ChatDetail> {
        self.execute_request(Method::GET, &format!("/api/v1/chats/{}", id), None, None)
            .await
    }

    /// Update a chat
    ///
    /// Updates the title of an existing chat.
    ///
    /// # Arguments
    /// * `id` - The chat ID to update
    /// * `title` - Updated chat title
    ///
    /// # Returns
    /// Updated chat information.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the chat doesn't exist.
    /// Returns `ApiError::ValidationError` if validation fails.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let chat_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    ///
    /// let chat = client.update_chat()
    ///     .id(&chat_id)
    ///     .title("Updated chat title".to_string())
    ///     .call()
    ///     .await?;
    ///
    /// println!("Updated chat: {}", chat.title);
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[builder]
    pub async fn update_chat(&self, id: &Uuid, title: String) -> ApiResult<ChatDetail> {
        let request = UpdateChatRequest { title };

        self.execute_request(
            Method::PATCH,
            &format!("/api/v1/chats/{}", id),
            None,
            Some(serde_json::to_string(&request)?),
        )
        .await
    }

    /// Delete a chat
    ///
    /// Permanently deletes a chat and all its messages.
    ///
    /// # Arguments
    /// * `id` - The chat ID to delete
    ///
    /// # Returns
    /// Unit type on successful deletion.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the chat doesn't exist.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let chat_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    /// client.delete_chat(&chat_id).await?;
    /// println!("Chat deleted");
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub async fn delete_chat(&self, id: &Uuid) -> ApiResult<()> {
        self.execute_request::<serde_json::Value>(
            Method::DELETE,
            &format!("/api/v1/chats/{}", id),
            None,
            None,
        )
        .await?;
        Ok(())
    }

    /// Create a message in a chat
    ///
    /// Sends a new message to a chat and triggers an AI response.
    ///
    /// # Arguments
    /// * `chat_id` - The chat ID to send the message to
    /// * `content` - Message content
    /// * `model` - Optional model identifier
    ///
    /// # Returns
    /// The created message with response status.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the chat doesn't exist.
    /// Returns `ApiError::ValidationError` if validation fails.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let chat_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    ///
    /// let message = client.create_message()
    ///     .chat_id(&chat_id)
    ///     .content("What were my expenses last month?".to_string())
    ///     .call()
    ///     .await?;
    ///
    /// println!("Message sent: {}", message.content);
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[builder]
    pub async fn create_message(
        &self,
        chat_id: &Uuid,
        content: String,
        model: Option<String>,
    ) -> ApiResult<MessageResponse> {
        let request = CreateMessageRequest { content, model };

        self.execute_request(
            Method::POST,
            &format!("/api/v1/chats/{}/messages", chat_id),
            None,
            Some(serde_json::to_string(&request)?),
        )
        .await
    }

    /// Retry the last assistant response
    ///
    /// Retries generating the last assistant message in a chat.
    ///
    /// # Arguments
    /// * `chat_id` - The chat ID to retry the response for
    ///
    /// # Returns
    /// Retry response with the new message ID.
    ///
    /// # Errors
    /// Returns `ApiError::NotFound` if the chat doesn't exist.
    /// Returns `ApiError::ValidationError` if no assistant message is available to retry.
    /// Returns `ApiError::Unauthorized` if the API key is invalid.
    /// Returns `ApiError::Network` if the request fails due to network issues.
    ///
    /// # Example
    /// ```no_run
    /// use sure_client_rs::{SureClient, BearerToken};
    /// use uuid::Uuid;
    ///
    /// # async fn example(client: SureClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let chat_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    /// let response = client.retry_message(&chat_id).await?;
    /// println!("Retry started: {}", response.message);
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub async fn retry_message(&self, chat_id: &Uuid) -> ApiResult<RetryResponse> {
        self.execute_request(
            Method::POST,
            &format!("/api/v1/chats/{}/messages/retry", chat_id),
            None,
            None,
        )
        .await
    }
}
