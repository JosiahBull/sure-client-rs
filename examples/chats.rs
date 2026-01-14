//! Chats CLI tool
//!
//! This tool provides commands for managing chats and messages.
//!
//! Usage:
//!   cargo run --example chats -- --token YOUR_TOKEN list
//!   cargo run --example chats -- --token YOUR_TOKEN create --title "My Chat"
//!   cargo run --example chats -- --token YOUR_TOKEN get --id CHAT_ID
//!   cargo run --example chats -- --token YOUR_TOKEN update --id CHAT_ID --title "Updated Title"
//!   cargo run --example chats -- --token YOUR_TOKEN delete --id CHAT_ID
//!   cargo run --example chats -- --token YOUR_TOKEN create-message --chat-id CHAT_ID --content "Hello"
//!   cargo run --example chats -- --token YOUR_TOKEN retry-message --chat-id CHAT_ID

use clap::{Parser, Subcommand};
use sure_client_rs::models::chat::{
    CreateChatRequest, CreateMessageRequest, UpdateChatRequest,
};
use sure_client_rs::{Auth, SureClient};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "chats")]
#[command(about = "Manage chats via the Sure API", long_about = None)]
struct Cli {
    /// API key or JWT bearer token for authentication
    #[arg(long, env = "SURE_TOKEN")]
    token: String,

    /// Base URL for the API (defaults to production)
    #[arg(long, env = "SURE_BASE_URL", default_value = "https://api.sure.app")]
    base_url: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all chats
    List,
    /// Create a new chat
    Create {
        /// Chat title (optional)
        #[arg(long)]
        title: Option<String>,
    },
    /// Get a specific chat by ID
    Get {
        /// Chat ID (UUID)
        #[arg(long)]
        id: String,
    },
    /// Update a chat
    Update {
        /// Chat ID (UUID)
        #[arg(long)]
        id: String,

        /// New chat title
        #[arg(long)]
        title: String,
    },
    /// Delete a chat
    Delete {
        /// Chat ID (UUID)
        #[arg(long)]
        id: String,
    },
    /// Create a message in a chat
    CreateMessage {
        /// Chat ID (UUID)
        #[arg(long)]
        chat_id: String,

        /// Message content
        #[arg(long)]
        content: String,
    },
    /// Retry generating an AI response for the last message in a chat
    RetryMessage {
        /// Chat ID (UUID)
        #[arg(long)]
        chat_id: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let client = SureClient::new(
        reqwest::Client::new(),
        Auth::api_key(cli.token),
        cli.base_url,
    );

    match cli.command {
        Commands::List => {
            let response = client.get_chats().call().await?;

            println!(
                "Chats (Page {} of {}):",
                response.pagination.page, response.pagination.total_pages
            );
            println!();

            for chat in response.items.chats {
                println!("ID:           {}", chat.id);
                println!("Title:        {}", chat.title);
                println!("Created:      {}", chat.created_at);
                println!("Updated:      {}", chat.updated_at);
                println!();
            }

            println!("Total: {} chats", response.pagination.total_count);
        }
        Commands::Create { title } => {
            let request = CreateChatRequest {
                title: title.unwrap_or_else(|| "New Chat".to_string()),
                message: None,
                model: None,
            };

            let chat = client.create_chat(&request).await?;

            println!("Chat created successfully!");
            println!();
            println!("ID:           {}", chat.id);
            println!("Title:        {}", chat.title);
            println!("Created:      {}", chat.created_at);
            println!("Updated:      {}", chat.updated_at);
        }
        Commands::Get { id } => {
            let chat_id =
                Uuid::parse_str(&id).map_err(|e| anyhow::anyhow!("Invalid chat ID: {}", e))?;

            let chat = client.get_chat(&chat_id).await?;

            println!("Chat Details:");
            println!();
            println!("ID:           {}", chat.id);
            println!("Title:        {}", chat.title);
            println!("Created:      {}", chat.created_at);
            println!("Updated:      {}", chat.updated_at);

            if !chat.messages.is_empty() {
                println!();
                println!("Messages ({}):", chat.messages.len());
                println!();

                for message in chat.messages {
                    println!("  Message ID:   {}", message.id);
                    println!("  Type:         {:?}", message.message_type);
                    println!("  Role:         {:?}", message.role);
                    println!("  Content:      {}", message.content);
                    println!("  Created:      {}", message.created_at);

                    if let Some(model) = &message.model {
                        println!("  Model:        {}", model);
                    }

                    if let Some(tool_calls) = &message.tool_calls {
                        if !tool_calls.is_empty() {
                            println!("  Tool Calls:   {}", tool_calls.len());
                        }
                    }

                    println!();
                }
            }
        }
        Commands::Update { id, title } => {
            let chat_id =
                Uuid::parse_str(&id).map_err(|e| anyhow::anyhow!("Invalid chat ID: {}", e))?;

            let request = UpdateChatRequest { title };

            let chat = client.update_chat(&chat_id, &request).await?;

            println!("Chat updated successfully!");
            println!();
            println!("ID:           {}", chat.id);
            println!("Title:        {}", chat.title);
            println!("Updated:      {}", chat.updated_at);
        }
        Commands::Delete { id } => {
            let chat_id =
                Uuid::parse_str(&id).map_err(|e| anyhow::anyhow!("Invalid chat ID: {}", e))?;

            client.delete_chat(&chat_id).await?;

            println!("Chat deleted successfully!");
        }
        Commands::CreateMessage { chat_id, content } => {
            let chat_id = Uuid::parse_str(&chat_id)
                .map_err(|e| anyhow::anyhow!("Invalid chat ID: {}", e))?;

            let request = CreateMessageRequest {
                content,
                model: None,
            };

            let message = client.create_message(&chat_id, &request).await?;

            println!("Message created successfully!");
            println!();
            println!("Message ID:   {}", message.id);
            println!("Type:         {:?}", message.message_type);
            println!("Role:         {:?}", message.role);
            println!("Content:      {}", message.content);
            println!("Created:      {}", message.created_at);

            if let Some(model) = &message.model {
                println!("Model:        {}", model);
            }
        }
        Commands::RetryMessage { chat_id } => {
            let chat_id = Uuid::parse_str(&chat_id)
                .map_err(|e| anyhow::anyhow!("Invalid chat ID: {}", e))?;

            let response = client.retry_message(&chat_id).await?;

            println!("Message retry initiated!");
            println!("{}", response.message);
        }
    }

    Ok(())
}
