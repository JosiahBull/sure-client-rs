//! Transactions CLI tool
//!
//! This tool provides commands for managing transactions.
//!
//! Usage:
//!   cargo run --example transactions -- --token YOUR_TOKEN list
//!   cargo run --example transactions -- --token YOUR_TOKEN list --start-date 2024-01-01 --end-date 2024-12-31
//!   cargo run --example transactions -- --token YOUR_TOKEN get --id TRANSACTION_ID
//!   cargo run --example transactions -- --token YOUR_TOKEN create --account-id ACC_ID --date 2024-01-15 --amount 42.50 --name "Grocery Store"
//!   cargo run --example transactions -- --token YOUR_TOKEN update --id TRANSACTION_ID --notes "Updated notes"
//!   cargo run --example transactions -- --token YOUR_TOKEN delete --id TRANSACTION_ID

use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use rust_decimal::Decimal;
use sure_client_rs::models::transaction::{TransactionNature, TransactionType};
use sure_client_rs::{AccountId, Auth, CategoryId, MerchantId, SureClient, TagId, TransactionId};
use url::Url;

#[derive(Parser)]
#[command(name = "transactions")]
#[command(about = "Manage transactions via the Sure API", long_about = None)]
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
    /// List transactions with optional filters
    List {
        /// Page number (default: 1)
        #[arg(long)]
        page: Option<u32>,

        /// Items per page (default: 25, max: 100)
        #[arg(long, alias = "per-page")]
        per_page: Option<u32>,

        /// Filter by account ID (UUID)
        #[arg(long)]
        account_id: Option<String>,

        /// Filter by category ID (UUID)
        #[arg(long)]
        category_id: Option<String>,

        /// Filter by merchant ID (UUID)
        #[arg(long)]
        merchant_id: Option<String>,

        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,

        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,

        /// Minimum amount
        #[arg(long)]
        min_amount: Option<Decimal>,

        /// Maximum amount
        #[arg(long)]
        max_amount: Option<Decimal>,

        /// Transaction type (income or expense)
        #[arg(long)]
        transaction_type: Option<TransactionType>,

        /// Search by name, notes, or merchant name
        #[arg(long)]
        search: Option<String>,
    },
    /// Get a specific transaction by ID
    Get {
        /// Transaction ID (UUID)
        #[arg(long)]
        id: String,
    },
    /// Create a new transaction
    Create {
        /// Account ID (UUID)
        #[arg(long)]
        account_id: String,

        /// Transaction date (YYYY-MM-DD)
        #[arg(long)]
        date: String,

        /// Transaction amount (e.g., 42.50)
        #[arg(long)]
        amount: Decimal,

        /// Transaction name/description
        #[arg(long)]
        name: String,

        /// Additional notes (optional)
        #[arg(long)]
        notes: Option<String>,

        /// Currency code (optional, e.g., USD)
        #[arg(long)]
        currency: Option<String>,

        /// Category ID (UUID, optional)
        #[arg(long)]
        category_id: Option<String>,

        /// Merchant ID (UUID, optional)
        #[arg(long)]
        merchant_id: Option<String>,

        /// Transaction nature (optional: debit or credit)
        #[arg(long)]
        nature: Option<TransactionNature>,

        /// Tag IDs (UUID, comma-separated, optional)
        #[arg(long, value_delimiter = ',')]
        tag_ids: Option<Vec<String>>,
    },
    /// Update a transaction
    Update {
        /// Transaction ID (UUID)
        #[arg(long)]
        id: String,

        /// New transaction date (YYYY-MM-DD, optional)
        #[arg(long)]
        date: Option<String>,

        /// New transaction amount (optional)
        #[arg(long)]
        amount: Option<Decimal>,

        /// New transaction name (optional)
        #[arg(long)]
        name: Option<String>,

        /// New notes (optional)
        #[arg(long)]
        notes: Option<String>,

        /// New currency code (optional)
        #[arg(long)]
        currency: Option<String>,

        /// New category ID (UUID, optional)
        #[arg(long)]
        category_id: Option<String>,

        /// New merchant ID (UUID, optional)
        #[arg(long)]
        merchant_id: Option<String>,

        /// New transaction nature (optional: debit or credit)
        #[arg(long)]
        nature: Option<TransactionNature>,

        /// New tag IDs (UUID, comma-separated, optional)
        #[arg(long, value_delimiter = ',')]
        tag_ids: Option<Vec<String>>,
    },
    /// Delete a transaction
    Delete {
        /// Transaction ID (UUID)
        #[arg(long)]
        id: String,
    },
}

fn parse_date(s: &str) -> anyhow::Result<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| anyhow::anyhow!("Invalid date format '{}': {}. Use YYYY-MM-DD", s, e))
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
        Commands::List {
            page,
            per_page,
            account_id,
            category_id,
            merchant_id,
            start_date,
            end_date,
            min_amount,
            max_amount,
            transaction_type,
            search,
        } => {
            let account_id = if let Some(id_str) = &account_id {
                Some(
                    AccountId::parse(id_str)
                        .map_err(|e| anyhow::anyhow!("Invalid account ID: {}", e))?,
                )
            } else {
                None
            };

            let category_id = if let Some(id_str) = &category_id {
                Some(
                    CategoryId::parse(id_str)
                        .map_err(|e| anyhow::anyhow!("Invalid category ID: {}", e))?,
                )
            } else {
                None
            };

            let merchant_id = if let Some(id_str) = &merchant_id {
                Some(
                    MerchantId::parse(id_str)
                        .map_err(|e| anyhow::anyhow!("Invalid merchant ID: {}", e))?,
                )
            } else {
                None
            };

            let start_date = if let Some(date_str) = &start_date {
                Some(parse_date(date_str)?)
            } else {
                None
            };

            let end_date = if let Some(date_str) = &end_date {
                Some(parse_date(date_str)?)
            } else {
                None
            };

            let response = client
                .get_transactions()
                .maybe_page(page)
                .maybe_per_page(per_page)
                .maybe_account_id(account_id.as_ref())
                .maybe_category_id(category_id.as_ref())
                .maybe_merchant_id(merchant_id.as_ref())
                .maybe_start_date(start_date.as_ref())
                .maybe_end_date(end_date.as_ref())
                .maybe_min_amount(min_amount)
                .maybe_max_amount(max_amount)
                .maybe_transaction_type(transaction_type)
                .maybe_search(search.as_deref())
                .call()
                .await?;

            println!(
                "Transactions (Page {} of {}):",
                response.pagination.page, response.pagination.total_pages
            );
            println!();

            for transaction in response.items.transactions {
                println!("ID:          {}", transaction.id);
                println!("Date:        {}", transaction.date);
                println!("Name:        {}", transaction.name);
                println!(
                    "Amount:      {} {}",
                    transaction.amount, transaction.currency
                );
                println!("Account:     {}", transaction.account.name);

                if let Some(category) = &transaction.category {
                    println!("Category:    {}", category.name);
                }

                if let Some(merchant) = &transaction.merchant {
                    println!("Merchant:    {}", merchant.name);
                }

                if let Some(notes) = &transaction.notes {
                    println!("Notes:       {}", notes);
                }

                if !transaction.tags.is_empty() {
                    let tag_names: Vec<_> =
                        transaction.tags.iter().map(|t| t.name.as_str()).collect();
                    println!("Tags:        {}", tag_names.join(", "));
                }

                println!();
            }

            println!("Total: {} transactions", response.pagination.total_count);
        }
        Commands::Get { id } => {
            let transaction_id = TransactionId::parse(&id)
                .map_err(|e| anyhow::anyhow!("Invalid transaction ID: {}", e))?;

            let transaction = client.get_transaction(&transaction_id).await?;

            println!("Transaction Details:");
            println!();
            println!("ID:             {}", transaction.id);
            println!("Date:           {}", transaction.date);
            println!("Name:           {}", transaction.name);
            println!(
                "Amount:         {} {}",
                transaction.amount, transaction.currency
            );
            println!("Classification: {}", transaction.classification);
            println!(
                "Account:        {} ({})",
                transaction.account.name, transaction.account.id
            );

            if let Some(category) = &transaction.category {
                println!("Category:       {} ({})", category.name, category.id);
            }

            if let Some(merchant) = &transaction.merchant {
                println!("Merchant:       {} ({})", merchant.name, merchant.id);
            }

            if let Some(notes) = &transaction.notes {
                println!("Notes:          {}", notes);
            }

            if !transaction.tags.is_empty() {
                println!("Tags:");
                for tag in &transaction.tags {
                    println!("  - {} ({})", tag.name, tag.id);
                }
            }

            if let Some(transfer) = &transaction.transfer {
                println!();
                println!("Transfer:");
                println!("  Amount:    {} {}", transfer.amount, transfer.currency);
                if let Some(other_account) = &transfer.other_account {
                    println!("  To/From:   {} ({})", other_account.name, other_account.id);
                }
            }

            println!();
            println!("Created:        {}", transaction.created_at);
            println!("Updated:        {}", transaction.updated_at);
        }
        Commands::Create {
            account_id,
            date,
            amount,
            name,
            notes,
            currency,
            category_id,
            merchant_id,
            nature,
            tag_ids,
        } => {
            let account_id = AccountId::parse(&account_id)
                .map_err(|e| anyhow::anyhow!("Invalid account ID: {}", e))?;

            let date = parse_date(&date)?;

            let category_id = if let Some(id_str) = &category_id {
                Some(
                    CategoryId::parse(id_str)
                        .map_err(|e| anyhow::anyhow!("Invalid category ID: {}", e))?,
                )
            } else {
                None
            };

            let merchant_id = if let Some(id_str) = &merchant_id {
                Some(
                    MerchantId::parse(id_str)
                        .map_err(|e| anyhow::anyhow!("Invalid merchant ID: {}", e))?,
                )
            } else {
                None
            };

            let tag_ids = if let Some(tag_id_strs) = tag_ids {
                let parsed_ids: Result<Vec<TagId>, _> = tag_id_strs
                    .iter()
                    .map(|id_str| {
                        TagId::parse(id_str)
                            .map_err(|e| anyhow::anyhow!("Invalid tag ID '{}': {}", id_str, e))
                    })
                    .collect();
                Some(parsed_ids?)
            } else {
                None
            };

            let transaction = client
                .create_transaction()
                .account_id(account_id)
                .date(date)
                .amount(amount)
                .name(name)
                .maybe_notes(notes)
                .maybe_currency(currency)
                .maybe_category_id(category_id)
                .maybe_merchant_id(merchant_id)
                .maybe_nature(nature)
                .maybe_tag_ids(tag_ids)
                .call()
                .await?;

            println!("Transaction created successfully!");
            println!();
            println!("ID:      {}", transaction.id);
            println!("Date:    {}", transaction.date);
            println!("Name:    {}", transaction.name);
            println!("Amount:  {} {}", transaction.amount, transaction.currency);
            println!("Account: {}", transaction.account.name);
        }
        Commands::Update {
            id,
            date,
            amount,
            name,
            notes,
            currency,
            category_id,
            merchant_id,
            nature,
            tag_ids,
        } => {
            let transaction_id = TransactionId::parse(&id)
                .map_err(|e| anyhow::anyhow!("Invalid transaction ID: {}", e))?;

            let date = if let Some(date_str) = &date {
                Some(parse_date(date_str)?)
            } else {
                None
            };

            let category_id = if let Some(id_str) = &category_id {
                Some(
                    CategoryId::parse(id_str)
                        .map_err(|e| anyhow::anyhow!("Invalid category ID: {}", e))?,
                )
            } else {
                None
            };

            let merchant_id = if let Some(id_str) = &merchant_id {
                Some(
                    MerchantId::parse(id_str)
                        .map_err(|e| anyhow::anyhow!("Invalid merchant ID: {}", e))?,
                )
            } else {
                None
            };

            let tag_ids = if let Some(tag_id_strs) = tag_ids {
                let parsed_ids: Result<Vec<TagId>, _> = tag_id_strs
                    .iter()
                    .map(|id_str| {
                        TagId::parse(id_str)
                            .map_err(|e| anyhow::anyhow!("Invalid tag ID '{}': {}", id_str, e))
                    })
                    .collect();
                Some(parsed_ids?)
            } else {
                None
            };

            let transaction = client
                .update_transaction()
                .id(&transaction_id)
                .maybe_date(date)
                .maybe_amount(amount)
                .maybe_name(name)
                .maybe_notes(notes)
                .maybe_currency(currency)
                .maybe_category_id(category_id)
                .maybe_merchant_id(merchant_id)
                .maybe_nature(nature)
                .maybe_tag_ids(tag_ids)
                .call()
                .await?;

            println!("Transaction updated successfully!");
            println!();
            println!("ID:      {}", transaction.id);
            println!("Date:    {}", transaction.date);
            println!("Name:    {}", transaction.name);
            println!("Amount:  {} {}", transaction.amount, transaction.currency);
            println!("Updated: {}", transaction.updated_at);
        }
        Commands::Delete { id } => {
            let transaction_id = TransactionId::parse(&id)
                .map_err(|e| anyhow::anyhow!("Invalid transaction ID: {}", e))?;

            let response = client.delete_transaction(&transaction_id).await?;

            println!("Transaction deleted successfully!");
            println!("{}", response.message);
        }
    }

    Ok(())
}
