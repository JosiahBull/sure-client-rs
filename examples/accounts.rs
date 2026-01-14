//! Accounts CLI tool
//!
//! This tool provides commands for managing accounts.
//!
//! Usage:
//!   cargo run --example accounts -- --token YOUR_TOKEN list
//!   cargo run --example accounts -- --token YOUR_TOKEN list --page 2 --per-page 10
//!   cargo run --example accounts -- --token YOUR_TOKEN get --id ACCOUNT_ID
//!   cargo run --example accounts -- --token YOUR_TOKEN create --name "Checking" --type "Depository" --balance 1000.00
//!   cargo run --example accounts -- --token YOUR_TOKEN update --id ACCOUNT_ID --name "Updated Name"
//!   cargo run --example accounts -- --token YOUR_TOKEN delete --id ACCOUNT_ID

use clap::{Parser, Subcommand};
use rust_decimal::Decimal;
use sure_client_rs::models::account::AccountKind;
use sure_client_rs::{AccountId, Auth, SureClient};
use url::Url;

#[derive(Parser)]
#[command(name = "accounts")]
#[command(about = "Manage accounts via the Sure API", long_about = None)]
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
    /// List all accounts
    List {
        /// Page number (default: 1)
        #[arg(long)]
        page: Option<u32>,

        /// Items per page (default: 25, max: 100)
        #[arg(long, alias = "per-page")]
        per_page: Option<u32>,
    },
    /// Get a specific account by ID
    Get {
        /// Account ID (UUID)
        #[arg(long)]
        id: String,
    },
    /// Create a new account
    Create {
        /// Account name
        #[arg(long)]
        name: String,

        /// Account type (e.g., "Depository", "CreditCard", "Investment", "Property", "Loan", "OtherAsset", "OtherLiability")
        #[arg(long, alias = "type")]
        kind: AccountKind,

        /// Initial balance (optional)
        #[arg(long)]
        balance: Option<Decimal>,

        /// Currency code (optional, defaults to family currency)
        #[arg(long)]
        currency: Option<String>,

        /// Account subtype (e.g., "checking", "savings", optional)
        #[arg(long)]
        subtype: Option<String>,

        /// Financial institution name (optional)
        #[arg(long)]
        institution_name: Option<String>,

        /// Financial institution domain (optional)
        #[arg(long)]
        institution_domain: Option<Url>,

        /// Additional notes (optional)
        #[arg(long)]
        notes: Option<String>,
    },
    /// Update an existing account
    Update {
        /// Account ID (UUID)
        #[arg(long)]
        id: String,

        /// New account name (optional)
        #[arg(long)]
        name: Option<String>,

        /// New balance (optional)
        #[arg(long)]
        balance: Option<Decimal>,

        /// New account subtype (optional)
        #[arg(long)]
        subtype: Option<String>,

        /// New financial institution name (optional)
        #[arg(long)]
        institution_name: Option<String>,

        /// New financial institution domain (optional)
        #[arg(long)]
        institution_domain: Option<Url>,

        /// New notes (optional)
        #[arg(long)]
        notes: Option<String>,
    },
    /// Delete an account
    Delete {
        /// Account ID (UUID)
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
                .get_accounts()
                .maybe_page(page)
                .maybe_per_page(per_page)
                .call()
                .await?;

            println!(
                "Accounts (Page {} of {}):",
                response.pagination.page, response.pagination.total_pages
            );
            println!();

            for account in response.items.accounts {
                println!("ID:             {}", account.id);
                println!("Name:           {}", account.name);
                println!("Balance:        {}", account.balance);
                println!("Currency:       {}", account.currency);
                println!("Classification: {}", account.classification);
                println!("Type:           {}", account.kind);
                println!();
            }

            println!("Total: {} accounts", response.pagination.total_count);
        }
        Commands::Get { id } => {
            let account_id =
                AccountId::parse(&id).map_err(|e| anyhow::anyhow!("Invalid account ID: {}", e))?;

            let account = client.get_account(&account_id).await?;

            println!("Account Details:");
            println!();
            println!("ID:             {}", account.id);
            println!("Name:           {}", account.name);
            println!("Balance:        {}", account.balance);
            println!("Currency:       {}", account.currency);
            println!("Classification: {}", account.classification);
            println!("Type:           {}", account.kind);

            if let Some(subtype) = account.subtype {
                println!("Subtype:        {}", subtype);
            }
            if let Some(institution) = account.institution_name {
                println!("Institution:    {}", institution);
            }
            if let Some(domain) = account.institution_domain {
                println!("Domain:         {}", domain);
            }
            if let Some(notes) = account.notes {
                println!("Notes:          {}", notes);
            }

            println!("Active:         {}", account.is_active);
            println!("Created:        {}", account.created_at);
            println!("Updated:        {}", account.updated_at);
        }
        Commands::Create {
            name,
            kind,
            balance,
            currency,
            subtype,
            institution_name,
            institution_domain,
            notes,
        } => {
            let account = client
                .create_account()
                .name(name)
                .kind(kind)
                .maybe_balance(balance)
                .maybe_currency(currency)
                .maybe_subtype(subtype)
                .maybe_institution_name(institution_name)
                .maybe_institution_domain(institution_domain)
                .maybe_notes(notes)
                .call()
                .await?;

            println!("✓ Account created successfully!");
            println!();
            println!("ID:             {}", account.id);
            println!("Name:           {}", account.name);
            println!("Balance:        {}", account.balance);
            println!("Currency:       {}", account.currency);
            println!("Classification: {}", account.classification);
            println!("Type:           {}", account.kind);

            if let Some(subtype) = account.subtype {
                println!("Subtype:        {}", subtype);
            }
            if let Some(institution) = account.institution_name {
                println!("Institution:    {}", institution);
            }
        }
        Commands::Update {
            id,
            name,
            balance,
            subtype,
            institution_name,
            institution_domain,
            notes,
        } => {
            let account_id =
                AccountId::parse(&id).map_err(|e| anyhow::anyhow!("Invalid account ID: {}", e))?;

            let account = client
                .update_account()
                .id(&account_id)
                .maybe_name(name)
                .maybe_balance(balance)
                .maybe_subtype(subtype)
                .maybe_institution_name(institution_name)
                .maybe_institution_domain(institution_domain)
                .maybe_notes(notes)
                .call()
                .await?;

            println!("✓ Account updated successfully!");
            println!();
            println!("ID:             {}", account.id);
            println!("Name:           {}", account.name);
            println!("Balance:        {}", account.balance);
            println!("Currency:       {}", account.currency);
            println!("Classification: {}", account.classification);
            println!("Type:           {}", account.kind);
        }
        Commands::Delete { id } => {
            let account_id =
                AccountId::parse(&id).map_err(|e| anyhow::anyhow!("Invalid account ID: {}", e))?;

            let response = client.delete_account(&account_id).await?;

            println!("✓ {}", response.message);
        }
    }

    Ok(())
}
