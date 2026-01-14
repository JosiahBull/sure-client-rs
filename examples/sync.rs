//! Sync CLI tool
//!
//! This tool provides commands for triggering family data synchronization.
//!
//! Usage:
//!   cargo run --example sync -- --token YOUR_TOKEN trigger

use clap::{Parser, Subcommand};
use sure_client_rs::{Auth, SureClient};

#[derive(Parser)]
#[command(name = "sync")]
#[command(about = "Manage family data synchronization via the Sure API", long_about = None)]
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
    /// Trigger a family data sync
    Trigger,
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
        Commands::Trigger => {
            let response = client.trigger_sync().await?;

            println!("Sync triggered successfully!");
            println!();
            println!("Status:  {:?}", response.status);
            println!("Message: {}", response.message);

            if let (Some(start), Some(end)) = (response.window_start_date, response.window_end_date) {
                println!();
                println!("Sync Window:");
                println!("  From: {}", start);
                println!("  To:   {}", end);
            }
        }
    }

    Ok(())
}
