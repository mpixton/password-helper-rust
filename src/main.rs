// !!! Argon2 does not enable std feature flag by default.

//! Simple CLI for testing a user on their passwords.
//!
//! List of all commands:
//! - List - list all accounts
//! - Edit <account> - edit given account password
//! - Add <account> - add an account
//! - Check - test the user on all accounts in the database
//! - Check [account] - test the user on a specific account
//! - Remove - remove all accounts from the database
//! - Remove [account] - remove a specific account from the database

use clap::{Parser, Subcommand};
use sqlx::SqlitePool;

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
/// Program is a CLI for adding, editing, removing, listing, and most importantly, testing a
/// user's knowledge of their stored passwords. All passwords are hashed using Argon2 before
/// being stored in a local-only `SQLite` database. Even if your computer is compromised, all
/// your passwords are safe behind industry-grade hashing and the passwords do no leave your
/// device.
#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// All possible commands to interact with the program.
#[derive(Subcommand)]
enum Commands {
    Add(add::Command),
    Check(check::Command),
    Edit(edit::Command),
    List(list::Command),
    Remove(remove::Command),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the CLI struct
    let cli = Cli::parse();

    // Setup the db, if needed, and return the db connection options
    let connect_options = database::setup_db().await?;

    let pool = SqlitePool::connect_with(connect_options).await?;

    match &cli.command {
        Commands::List(_) => list::list_all_accounts(&pool).await,
        Commands::Add(data) => add::add_account(&pool, &data.account).await,
        Commands::Edit(data) => edit::edit_account(&pool, &data.account).await,
        Commands::Check(data) => match &data.account {
            Some(account) => check::check_account(&pool, account).await,
            None => check::check_all_accounts(&pool).await,
        },
        Commands::Remove(data) => match &data.account {
            Some(account) => remove::remove_account(&pool, account).await,
            None => remove::remove_all_accounts(&pool).await,
        },
    }?;

    Ok(())
}
