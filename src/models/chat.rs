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

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::UserMessage => write!(f, "user_message"),
            MessageType::AssistantMessage => write!(f, "assistant_message"),
        }
    }
}

/// Error returned when parsing a `MessageType` from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseMessageTypeError(String);

impl std::fmt::Display for ParseMessageTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid message type: {}", self.0)
    }
}

impl std::error::Error for ParseMessageTypeError {}

impl std::str::FromStr for MessageType {
    type Err = ParseMessageTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user_message" => Ok(MessageType::UserMessage),
            "assistant_message" => Ok(MessageType::AssistantMessage),
            _ => Err(ParseMessageTypeError(s.to_string())),
        }
    }
}

impl TryFrom<&str> for MessageType {
    type Error = ParseMessageTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for MessageType {
    type Error = ParseMessageTypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
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

impl std::fmt::Display for MessageRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageRole::User => write!(f, "user"),
            MessageRole::Assistant => write!(f, "assistant"),
        }
    }
}

/// Error returned when parsing a `MessageRole` from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseMessageRoleError(String);

impl std::fmt::Display for ParseMessageRoleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid message role: {}", self.0)
    }
}

impl std::error::Error for ParseMessageRoleError {}

impl std::str::FromStr for MessageRole {
    type Err = ParseMessageRoleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(MessageRole::User),
            "assistant" => Ok(MessageRole::Assistant),
            _ => Err(ParseMessageRoleError(s.to_string())),
        }
    }
}

impl TryFrom<&str> for MessageRole {
    type Error = ParseMessageRoleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for MessageRole {
    type Error = ParseMessageRoleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
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

impl std::fmt::Display for AiResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiResponseStatus::Pending => write!(f, "pending"),
            AiResponseStatus::Complete => write!(f, "complete"),
            AiResponseStatus::Failed => write!(f, "failed"),
        }
    }
}

/// Error returned when parsing an `AiResponseStatus` from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseAiResponseStatusError(String);

impl std::fmt::Display for ParseAiResponseStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid AI response status: {}", self.0)
    }
}

impl std::error::Error for ParseAiResponseStatusError {}

impl std::str::FromStr for AiResponseStatus {
    type Err = ParseAiResponseStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(AiResponseStatus::Pending),
            "complete" => Ok(AiResponseStatus::Complete),
            "failed" => Ok(AiResponseStatus::Failed),
            _ => Err(ParseAiResponseStatusError(s.to_string())),
        }
    }
}

impl TryFrom<&str> for AiResponseStatus {
    type Error = ParseAiResponseStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for AiResponseStatus {
    type Error = ParseAiResponseStatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
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
pub(crate) struct CreateChatRequest {
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
pub(crate) struct UpdateChatRequest {
    /// Updated chat title
    pub title: String,
}

/// Create message request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub(crate) struct CreateMessageRequest {
    /// Message content
    pub content: String,
    /// Optional model identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}
