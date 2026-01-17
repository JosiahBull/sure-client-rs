//! Categories CLI tool
//!
//! This tool provides commands for managing categories.
//!
//! Usage:
//!   cargo run --example categories -- --token YOUR_TOKEN list
//!   cargo run --example categories -- --token YOUR_TOKEN list --classification expense
//!   cargo run --example categories -- --token YOUR_TOKEN list --roots-only
//!   cargo run --example categories -- --token YOUR_TOKEN get --id CATEGORY_ID
//!   cargo run --example categories -- --token YOUR_TOKEN create --name "Groceries" --classification expense --color "#FF5733"
//!   cargo run --example categories -- --token YOUR_TOKEN update --id CATEGORY_ID --name "Updated Name"
//!   cargo run --example categories -- --token YOUR_TOKEN delete --id CATEGORY_ID

use clap::{Parser, Subcommand};
use sure_client_rs::models::category::Classification;
use sure_client_rs::{Auth, CategoryId, SureClient};
use url::Url;

#[derive(Parser)]
#[command(name = "categories")]
#[command(about = "Manage categories via the Sure API", long_about = None)]
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
    /// List all categories
    List {
        /// Page number (default: 1)
        #[arg(long)]
        page: Option<u32>,

        /// Items per page (default: 25, max: 100)
        #[arg(long, alias = "per-page")]
        per_page: Option<u32>,

        /// Filter by classification (income or expense)
        #[arg(long)]
        classification: Option<Classification>,

        /// Return only root categories (no parent)
        #[arg(long)]
        roots_only: bool,

        /// Filter by parent category ID (UUID)
        #[arg(long)]
        parent_id: Option<String>,
    },
    /// Get a specific category by ID
    Get {
        /// Category ID (UUID)
        #[arg(long)]
        id: String,
    },
    /// Create a new category
    Create {
        /// Category name
        #[arg(long)]
        name: String,

        /// Classification (income or expense)
        #[arg(long)]
        classification: Classification,

        /// Color in hex format (e.g., "#FF5733")
        #[arg(long)]
        color: String,

        /// Lucide icon name (optional)
        #[arg(long)]
        icon: Option<String>,

        /// Parent category ID for subcategories (UUID, optional)
        #[arg(long)]
        parent_id: Option<String>,
    },
    /// Update an existing category
    Update {
        /// Category ID (UUID)
        #[arg(long)]
        id: String,

        /// New category name (optional)
        #[arg(long)]
        name: Option<String>,

        /// New classification (income or expense, optional)
        #[arg(long)]
        classification: Option<Classification>,

        /// New color in hex format (optional)
        #[arg(long)]
        color: Option<String>,

        /// New Lucide icon name (optional)
        #[arg(long)]
        icon: Option<String>,

        /// New parent category ID (UUID, optional)
        #[arg(long)]
        parent_id: Option<String>,
    },
    /// Delete a category
    Delete {
        /// Category ID (UUID)
        #[arg(long)]
        id: String,
    },
}

fn validate_and_normalize_hex_code(code: &str) -> Option<String> {
    let code = code.trim();
    (code.len() == 7
        && code.starts_with('#')
        && code.chars().skip(1).all(|c| c.is_ascii_hexdigit()))
    .then(|| code.to_uppercase())
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
            classification,
            roots_only,
            parent_id,
        } => {
            let parent_id = if let Some(id_str) = &parent_id {
                Some(
                    CategoryId::parse(id_str)
                        .map_err(|e| anyhow::anyhow!("Invalid parent category ID: {}", e))?,
                )
            } else {
                None
            };

            let response = client
                .get_categories()
                .maybe_page(page)
                .maybe_per_page(per_page)
                .maybe_classification(classification)
                .roots_only(roots_only)
                .maybe_parent_id(parent_id.as_ref())
                .call()
                .await?;

            println!(
                "Categories (Page {} of {}):",
                response.pagination.page, response.pagination.total_pages
            );
            println!();

            for category in response.items.categories {
                println!("ID:             {}", category.id);
                println!("Name:           {}", category.name);
                println!("Classification: {}", category.classification);
                println!("Color:          {}", category.color);
                println!("Icon:           {}", category.icon);

                if let Some(parent) = category.parent {
                    println!("Parent:         {} ({})", parent.name, parent.id);
                }
                if category.subcategories_count > 0 {
                    println!("Subcategories:  {}", category.subcategories_count);
                }
                println!();
            }

            println!("Total: {} categories", response.pagination.total_count);
        }
        Commands::Get { id } => {
            let category_id = CategoryId::parse(&id)
                .map_err(|e| anyhow::anyhow!("Invalid category ID: {}", e))?;

            let category = client.get_category(&category_id).await?;

            println!("Category Details:");
            println!();
            println!("ID:             {}", category.id);
            println!("Name:           {}", category.name);
            println!("Classification: {}", category.classification);
            println!("Color:          {}", category.color);
            println!("Icon:           {}", category.icon);

            if let Some(parent) = category.parent {
                println!("Parent:         {} ({})", parent.name, parent.id);
            }

            if category.subcategories_count > 0 {
                println!("Subcategories:  {}", category.subcategories_count);
            }
        }
        Commands::Create {
            name,
            classification,
            color,
            icon,
            parent_id,
        } => {
            let parent_id = if let Some(id_str) = &parent_id {
                Some(
                    CategoryId::parse(id_str)
                        .map_err(|e| anyhow::anyhow!("Invalid parent category ID: {}", e))?,
                )
            } else {
                None
            };

            if validate_and_normalize_hex_code(&color).is_none() {
                return Err(anyhow::anyhow!(
                    "Invalid color code: {}. Must be in hex format like '#FF5733'.",
                    color
                ));
            }

            let category = client
                .create_category()
                .name(name)
                .classification(classification)
                .color(color)
                .maybe_lucide_icon(icon)
                .maybe_parent_id(parent_id)
                .call()
                .await?;

            println!("✓ Category created successfully!");
            println!();
            println!("ID:             {}", category.id);
            println!("Name:           {}", category.name);
            println!("Classification: {}", category.classification);
            println!("Color:          {}", category.color);
            println!("Icon:           {}", category.icon);

            if let Some(parent) = category.parent {
                println!("Parent:         {} ({})", parent.name, parent.id);
            }
        }
        Commands::Update {
            id,
            name,
            classification,
            color,
            icon,
            parent_id,
        } => {
            let category_id = CategoryId::parse(&id)
                .map_err(|e| anyhow::anyhow!("Invalid category ID: {}", e))?;

            let parent_id = if let Some(id_str) = &parent_id {
                Some(
                    CategoryId::parse(id_str)
                        .map_err(|e| anyhow::anyhow!("Invalid parent category ID: {}", e))?,
                )
            } else {
                None
            };

            let category = client
                .update_category()
                .id(&category_id)
                .maybe_name(name)
                .maybe_classification(classification)
                .maybe_color(color)
                .maybe_lucide_icon(icon)
                .maybe_parent_id(parent_id)
                .call()
                .await?;

            println!("✓ Category updated successfully!");
            println!();
            println!("ID:             {}", category.id);
            println!("Name:           {}", category.name);
            println!("Classification: {}", category.classification);
            println!("Color:          {}", category.color);
            println!("Icon:           {}", category.icon);

            if let Some(parent) = category.parent {
                println!("Parent:         {} ({})", parent.name, parent.id);
            }
        }
        Commands::Delete { id } => {
            let category_id = CategoryId::parse(&id)
                .map_err(|e| anyhow::anyhow!("Invalid category ID: {}", e))?;

            let response = client.delete_category(&category_id).await?;

            println!("✓ {}", response.message);
        }
    }

    Ok(())
}
