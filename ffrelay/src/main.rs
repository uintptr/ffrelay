use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use ffrelay::token::{find_token, save_token};
use ffrelay_api::{api::FFRelayApi, types::FirefoxEmailRelayRequest};
use log::{LevelFilter, error};
use rstaples::logging::StaplesLogger;
use tabled::{
    Table,
    settings::{Rotate, Style},
};

#[derive(Args)]
pub struct CreateArgs {
    /// Email Description Context
    #[arg(short, long)]
    pub description: String,

    /// Address to create a address@yourdomain.mozmail.com
    #[arg(short, long)]
    pub address: Option<String>,
}

#[derive(Args)]
pub struct EmailIdArgs {
    /// Email id
    pub email_ids: Vec<u64>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new relay email
    #[command(visible_alias = "new")]
    CreateEmail(CreateArgs),
    /// List relay emails
    #[command(visible_alias = "ls")]
    ListEmail,

    #[command(visible_alias = "rm")]
    /// Delete a relay email
    DeleteEmail(EmailIdArgs),

    /// Profiles
    Profiles,

    /// Enable
    Enable(EmailIdArgs),

    /// Enable
    Disable(EmailIdArgs),
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct UserArgs {
    /// verbose
    #[arg(short, long)]
    pub verbose: bool,

    /// token
    #[arg(short, long)]
    pub token: Option<String>,

    /// Command
    #[command(subcommand)]
    pub command: Commands,
}

async fn command_disable(api: FFRelayApi, email_ids: Vec<u64>) -> Result<()> {
    for id in email_ids {
        match api.disable(id).await {
            Ok(_) => {
                println!("Disabled {id}");
            }
            Err(e) => {
                println!("Unable to disable {id} => {e}");
            }
        }
    }

    Ok(())
}

async fn command_enable(api: FFRelayApi, email_ids: Vec<u64>) -> Result<()> {
    for id in email_ids {
        match api.enable(id).await {
            Ok(_) => {
                println!("Enabled {id}");
            }
            Err(e) => {
                println!("Unable to enable {id} => {e}");
            }
        }
    }

    Ok(())
}

async fn command_profiles(api: FFRelayApi) -> Result<()> {
    let profiles = api.profiles().await?;

    let mut table = Table::new(profiles);
    table.with(Style::modern()).with(Rotate::Left);

    println!("{table}");

    Ok(())
}

async fn command_list(api: FFRelayApi) -> Result<()> {
    let emails = api.list().await?;

    let mut table = Table::new(emails);
    table.with(Style::modern());

    println!("{table}");
    Ok(())
}

async fn command_delete(api: FFRelayApi, email_ids: Vec<u64>) -> Result<()> {
    for id in email_ids {
        match api.delete(id).await {
            Ok(_) => {
                println!("Deleted {id}");
            }
            Err(e) => {
                println!("Unable to delete {id} => {e}");
            }
        }
    }

    Ok(())
}

async fn command_create(api: FFRelayApi, args: CreateArgs) -> Result<()> {
    let req = FirefoxEmailRelayRequest::builder()
        .description(args.description)
        .maybe_address(args.address)
        .build();

    let email = api.create(req).await?;

    println!("{email}");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = UserArgs::parse();

    let log_level = if args.verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Error
    };

    StaplesLogger::new()
        .with_colors()
        .with_log_level(log_level)
        .start();

    let token = if let Some(token) = &args.token {
        if let Err(e) = save_token(token) {
            error!("unable to save token ({e})");
        }
        token.to_string()
    } else {
        find_token()?
    };

    let api = FFRelayApi::new(token);

    match args.command {
        Commands::ListEmail => command_list(api).await,
        Commands::DeleteEmail(a) => command_delete(api, a.email_ids).await,
        Commands::CreateEmail(a) => command_create(api, a).await,
        Commands::Profiles => command_profiles(api).await,
        Commands::Enable(a) => command_enable(api, a.email_ids).await,
        Commands::Disable(a) => command_disable(api, a.email_ids).await,
    }
}
