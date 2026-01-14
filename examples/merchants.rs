//! Merchants CLI tool
//!
//! This tool provides commands for managing merchants.
//!
//! Usage:
//!   cargo run --example merchants -- --token YOUR_TOKEN list
//!   cargo run --example merchants -- --token YOUR_TOKEN get --id MERCHANT_ID
//!   cargo run --example merchants -- --token YOUR_TOKEN create --name "Starbucks" --color "#00704A"
//!   cargo run --example merchants -- --token YOUR_TOKEN update --id MERCHANT_ID --name "Updated Name"
//!   cargo run --example merchants -- --token YOUR_TOKEN delete --id MERCHANT_ID

use clap::{Parser, Subcommand};
use sure_client_rs::{Auth, MerchantId, SureClient};
use url::Url;

#[derive(Parser)]
#[command(name = "merchants")]
#[command(about = "Manage merchants via the Sure API", long_about = None)]
struct Cli {
    /// API key or JWT bearer token for authentication
    #[arg(long, env = "SURE_TOKEN")]
    token: String,

    /// Base URL for the API (defaults to production)
    #[arg(long, env = "SURE_BASE_URL", default_value = "http://localhost:3000")]
    base_url: Url,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all merchants
    List {
        /// Page number (default: 1)
        #[arg(long)]
        page: Option<u32>,

        /// Items per page (default: 25, max: 100)
        #[arg(long, alias = "per-page")]
        per_page: Option<u32>,
    },
    /// Get a specific merchant by ID
    Get {
        /// Merchant ID (UUID)
        #[arg(long)]
        id: String,
    },
    /// Create a new merchant
    Create {
        /// Merchant name
        #[arg(long)]
        name: String,

        /// Color in hex format (e.g., "#FF5733", optional)
        #[arg(long)]
        color: Option<String>,
    },
    /// Update an existing merchant
    Update {
        /// Merchant ID (UUID)
        #[arg(long)]
        id: String,

        /// New merchant name (optional)
        #[arg(long)]
        name: Option<String>,

        /// New color in hex format (optional)
        #[arg(long)]
        color: Option<String>,
    },
    /// Delete a merchant
    Delete {
        /// Merchant ID (UUID)
        #[arg(long)]
        id: String,
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
        Commands::List { page, per_page } => {
            let response = client
                .get_merchants()
                .maybe_page(page)
                .maybe_per_page(per_page)
                .call()
                .await?;

            println!(
                "Merchants (Page {} of {}):",
                response.pagination.page, response.pagination.total_pages
            );
            println!();

            for merchant in response.items.merchants {
                println!("ID:    {}", merchant.id);
                println!("Name:  {}", merchant.name);
                if let Some(color) = merchant.color {
                    println!("Color: {}", color);
                }
                println!();
            }

            println!("Total: {} merchants", response.pagination.total_count);
        }
        Commands::Get { id } => {
            let merchant_id = MerchantId::parse(&id)
                .map_err(|e| anyhow::anyhow!("Invalid merchant ID: {}", e))?;

            let merchant = client.get_merchant(&merchant_id).await?;

            println!("Merchant Details:");
            println!();
            println!("ID:         {}", merchant.id);
            println!("Name:       {}", merchant.name);
            if let Some(color) = merchant.color {
                println!("Color:      {}", color);
            }
            println!("Created:    {}", merchant.created_at);
            println!("Updated:    {}", merchant.updated_at);
        }
        Commands::Create { name, color } => {
            let merchant = client
                .create_merchant()
                .name(name)
                .maybe_color(color)
                .call()
                .await?;

            println!("✓ Merchant created successfully!");
            println!();
            println!("ID:    {}", merchant.id);
            println!("Name:  {}", merchant.name);
            if let Some(color) = merchant.color {
                println!("Color: {}", color);
            }
        }
        Commands::Update { id, name, color } => {
            let merchant_id = MerchantId::parse(&id)
                .map_err(|e| anyhow::anyhow!("Invalid merchant ID: {}", e))?;

            let merchant = client
                .update_merchant()
                .id(&merchant_id)
                .maybe_name(name)
                .maybe_color(color)
                .call()
                .await?;

            println!("✓ Merchant updated successfully!");
            println!();
            println!("ID:    {}", merchant.id);
            println!("Name:  {}", merchant.name);
            if let Some(color) = merchant.color {
                println!("Color: {}", color);
            }
        }
        Commands::Delete { id } => {
            let merchant_id = MerchantId::parse(&id)
                .map_err(|e| anyhow::anyhow!("Invalid merchant ID: {}", e))?;

            let response = client.delete_merchant(&merchant_id).await?;

            println!("✓ {}", response.message);
        }
    }

    Ok(())
}
