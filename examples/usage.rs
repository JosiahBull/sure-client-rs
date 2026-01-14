//! Usage CLI tool
//!
//! This tool provides commands for checking API usage statistics.
//!
//! Usage:
//!   cargo run --example usage -- --token YOUR_TOKEN get

use clap::{Parser, Subcommand};
use sure_client_rs::models::usage::UsageResponse;
use sure_client_rs::{Auth, SureClient};

#[derive(Parser)]
#[command(name = "usage")]
#[command(about = "Check API usage via the Sure API", long_about = None)]
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
    /// Get current API usage statistics
    Get,
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
        Commands::Get => {
            let response = client.get_usage().await?;

            println!("API Usage Statistics:");
            println!();

            match response {
                UsageResponse::ApiKey(usage) => {
                    println!("Authentication: API Key");
                    println!();
                    println!("API Key:");
                    println!("  Name:       {}", usage.api_key.name);
                    println!("  Scopes:     {}", usage.api_key.scopes.join(", "));
                    println!("  Created:    {}", usage.api_key.created_at);
                    if let Some(last_used) = usage.api_key.last_used_at {
                        println!("  Last Used:  {}", last_used);
                    }

                    println!();
                    println!("Rate Limiting:");
                    println!("  Tier:           {:?}", usage.rate_limit.tier);
                    println!(
                        "  Limit:          {}",
                        usage.rate_limit.limit.unwrap_or_default()
                    );
                    println!("  Current Count:  {}", usage.rate_limit.current_count);
                    println!(
                        "  Remaining:      {}",
                        usage.rate_limit.remaining.unwrap_or_default()
                    );
                    println!(
                        "  Reset In:       {} seconds",
                        usage.rate_limit.reset_in_seconds
                    );
                    println!("  Reset At:       {}", usage.rate_limit.reset_at);
                }
                UsageResponse::OAuth(usage) => {
                    println!("Authentication: {:?}", usage.authentication_method);
                    println!();
                    println!("Message: {}", usage.message);
                }
            }
        }
    }

    Ok(())
}
