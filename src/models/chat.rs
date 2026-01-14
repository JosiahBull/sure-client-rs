use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Pagination;

/// Tool call information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct ToolCall {
    /// Tool call ID
    pub id: Uuid,
    /// Function name
    pub function_name: String,
    /// Function arguments
    pub function_arguments: serde_json::Value,
    /// Function result
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function_result: Option<serde_json::Value>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Message type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    /// User message
    UserMessage,
    /// Assistant message
    AssistantMessage,
}

/// Message role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    /// User role
    User,
    /// Assistant role
    Assistant,
}

/// Message in a chat
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Message {
    /// Message ID
    pub id: Uuid,
    /// Message type
    #[serde(rename = "type")]
    pub message_type: MessageType,
    /// Message role
    pub role: MessageRole,
    /// Message content
    pub content: String,
    /// Model identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// Tool calls associated with this message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

/// AI response status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AiResponseStatus {
    /// Pending
    Pending,
    /// Complete
    Complete,
    /// Failed
    Failed,
}

/// Message response with additional fields
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct MessageResponse {
    /// Message ID
    pub id: Uuid,
    /// Message type
    #[serde(rename = "type")]
    pub message_type: MessageType,
    /// Message role
    pub role: MessageRole,
    /// Message content
    pub content: String,
    /// Model identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// Tool calls associated with this message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// Chat ID
    pub chat_id: Uuid,
    /// AI response status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ai_response_status: Option<AiResponseStatus>,
    /// AI response message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ai_response_message: Option<String>,
}

/// Chat resource base information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct ChatResource {
    /// Chat ID
    pub id: Uuid,
    /// Chat title
    pub title: String,
    /// Error message (if any)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Chat summary with message count
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct ChatSummary {
    /// Chat ID
    pub id: Uuid,
    /// Chat title
    pub title: String,
    /// Error message (if any)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// Number of messages in the chat
    pub message_count: u32,
    /// Timestamp of last message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_message_at: Option<DateTime<Utc>>,
}

/// Detailed chat information with messages
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct ChatDetail {
    /// Chat ID
    pub id: Uuid,
    /// Chat title
    pub title: String,
    /// Error message (if any)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// Messages in the chat
    pub messages: Vec<Message>,
    /// Pagination information (if paginated)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

/// Collection of chat summaries
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct ChatCollection {
    /// List of chats
    pub chats: Vec<ChatSummary>,
}

/// Retry response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct RetryResponse {
    /// Response message
    pub message: String,
    /// Message ID
    pub message_id: Uuid,
}

/// Create chat request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct CreateChatRequest {
    /// Chat title
    pub title: String,
    /// Optional initial message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Optional OpenAI model identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Update chat request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct UpdateChatRequest {
    /// Updated chat title
    pub title: String,
}

/// Create message request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct CreateMessageRequest {
    /// Message content
    pub content: String,
    /// Optional model identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}
