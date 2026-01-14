//! Authentication CLI tool
//!
//! This tool provides commands for authentication operations.
//!
//! Usage:
//!   cargo run --example auth -- signup --email user@example.com --password "MyPass123!" --first-name John --last-name Doe
//!   cargo run --example auth -- login --email user@example.com --password "MyPass123!"
//!   cargo run --example auth -- refresh --refresh-token YOUR_REFRESH_TOKEN

use clap::{Parser, Subcommand};
use sure_client_rs::SureClient;
use sure_client_rs::models::auth::{DeviceInfo, RefreshDeviceInfo, SignupUserData};
use url::Url;
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "auth")]
#[command(about = "Authentication operations via the Sure API", long_about = None)]
struct Cli {
    /// Base URL for the API (defaults to production)
    #[arg(long, env = "SURE_BASE_URL", default_value = "http://localhost:3000")]
    base_url: Url,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sign up a new user
    Signup {
        /// User email address
        #[arg(long)]
        email: String,

        /// User password
        #[arg(long)]
        password: String,

        /// User's first name
        #[arg(long)]
        first_name: String,

        /// User's last name
        #[arg(long)]
        last_name: String,
    },
    /// Log in an existing user
    Login {
        /// User email address
        #[arg(long)]
        email: String,

        /// User password
        #[arg(long)]
        password: String,
    },
    /// Refresh an access token
    Refresh {
        /// Refresh token
        #[arg(long)]
        refresh_token: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Auth endpoints don't require authentication, so we create a client without a token
    // We'll use a dummy auth that won't be used
    let client = SureClient::new(
        reqwest::Client::new(),
        sure_client_rs::Auth::api_key("unused"),
        cli.base_url,
    );

    match cli.command {
        Commands::Signup {
            email,
            password,
            first_name,
            last_name,
        } => {
            let device = DeviceInfo {
                device_id: format!("cli-{}", uuid::Uuid::new_v4()),
                device_name: "Claude CLI".to_string(),
                device_type: "cli".to_string(),
                os_version: std::env::consts::OS.to_string(),
                app_version: env!("CARGO_PKG_VERSION").to_string(),
            };

            let response = client
                .signup()
                .user(SignupUserData {
                    email,
                    password,
                    first_name,
                    last_name,
                })
                .device(device)
                .call()
                .await?;

            println!("Signup successful!");
            println!();
            println!("Access Token:  {}", response.access_token);
            println!("Refresh Token: {}", response.refresh_token);
            println!("Token Type:    {:?}", response.token_type);
            println!("Expires In:    {} seconds", response.expires_in.as_secs());
            println!();
            println!("User Info:");
            println!("  ID:         {}", response.user.id);
            println!("  Email:      {}", response.user.email);
            println!("  First Name: {}", response.user.first_name);
            println!("  Last Name:  {}", response.user.last_name);
        }
        Commands::Login { email, password } => {
            let device = DeviceInfo {
                device_id: format!("cli-{}", Uuid::new_v4()),
                device_name: "Claude CLI".to_string(),
                device_type: "cli".to_string(),
                os_version: std::env::consts::OS.to_string(),
                app_version: env!("CARGO_PKG_VERSION").to_string(),
            };

            let response = client
                .login()
                .email(email)
                .password(password)
                .device(device)
                .call()
                .await?;

            println!("Login successful!");
            println!();
            println!("Access Token:  {}", response.access_token);
            println!("Refresh Token: {}", response.refresh_token);
            println!("Token Type:    {:?}", response.token_type);
            println!("Expires In:    {} seconds", response.expires_in.as_secs());
            println!();
            println!("User Info:");
            println!("  ID:         {}", response.user.id);
            println!("  Email:      {}", response.user.email);
            println!("  First Name: {}", response.user.first_name);
            println!("  Last Name:  {}", response.user.last_name);
        }
        Commands::Refresh { refresh_token } => {
            let device = RefreshDeviceInfo {
                device_id: format!("cli-{}", Uuid::new_v4()),
            };

            let response = client
                .refresh_token()
                .refresh_token(refresh_token)
                .device(device)
                .call()
                .await?;

            println!("Token refresh successful!");
            println!();
            println!("Access Token:  {}", response.access_token);
            println!("Refresh Token: {}", response.refresh_token);
            println!("Token Type:    {:?}", response.token_type);
            println!("Expires In:    {} seconds", response.expires_in.as_secs());
        }
    }

    Ok(())
}
