// https://docs.rs/clap/latest/clap/
// https://docs.rs/clap/latest/clap/_derive/index.html#overview
// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
// https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field
// https://docs.rs/rpassword/latest/rpassword/
// https://docs.rs/sqlx/latest/sqlx/
// https://docs.rs/argon2/latest/argon2/
// https://crates.io/crates/anyhow
// https://crates.io/crates/thiserror

// !!! Argon2 does not enable std feature flag by default.

//! List of all commands:
//! List - list all accounts
//! Edit <account> - edit given account password
//! Add <account> - add an account
//! Check - test the user on all accounts in the database
//! Check <account> - test the user on a specific account
//! Remove - remove all accounts from the database
//! Remove <account> - remove a specific account from the database

use clap::{Parser, Subcommand};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

mod add;
mod check;
mod database;
mod edit;
mod error;
mod list;
mod password;
mod remove;

/// Utility to help memorize passwords
///
/// This is a longer description.
#[derive(Parser)]
#[command(author, version, about, long_about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(add::Add),
    Check(check::Check),
    Edit(edit::Edit),
    List(list::List),
    Remove(remove::Remove),
    Testing,
}

#[tokio::main]
async fn main() -> Result<(), error::AppErrors> {
    let cli = Cli::parse();

    const DB_URL: &str = "sqlite://./db.db";

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating local database...");
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => {
                println!("Create db success");
                println!("Setting up database...");
                let pool = SqlitePool::connect(DB_URL).await?;
                database::setup_db(&pool).await?;
                println!("Database set up")
            }
            Err(error) => panic!("error: {}", error),
        };
    } else {
        println!("Database already exists");
    }

    let pool = SqlitePool::connect(DB_URL).await?;

    match &cli.command {
        Commands::List(_) => list::list_all_accounts(&pool).await,
        Commands::Add(data) => add::add_account(&pool, &data.account).await,
        Commands::Edit(data) => edit::edit_account(&pool, &data.account).await,
        Commands::Check(data) => match &data.account {
            Some(account) => check::check_account(&pool, &account).await,
            None => check::check_all_accounts(&pool).await,
        },
        Commands::Remove(data) => match &data.account {
            Some(account) => remove::remove_account(&pool, account).await,
            None => remove::remove_all_accounts(&pool).await,
        },
        Commands::Testing => {
            let hash = password::get_password_from_user().await?;
            // let test_hash = "argon2id$v=19$m=4096,t=3,p=1$kFqodVZyHfN9ZgRjRtdlhw$cKubT7PNsFGfX+BDd6RfyHKjtRwaDpmDLXbJS8ozlEE".to_string();
            let pw_ok = password::verify_password(&hash).await?;
            println!("Passwords ok: {pw_ok}");
            Ok(())
        }
    }?;

    Ok(())
}
