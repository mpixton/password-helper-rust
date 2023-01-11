use crate::database;
use clap::Args;
use sqlx::SqlitePool;

/// Remove all accounts or specific account
///
/// If `account` is provided, remove only that account. If omitted, purge all accounts from the
/// database and list all the accounts removed. If `account` is provided and doesn't exist, nothing
/// will happen.
#[derive(Args)]
pub struct Command {
    /// Name of the specific account to remove
    pub account: Option<String>,
}

/// Remove all accounts from the database.
///
/// # Parameters
/// * `pool` - [sqlx::SqlitePool] of connections to the database
pub async fn remove_all_accounts(pool: &SqlitePool) -> anyhow::Result<()> {
    println!("Removing all accounts...");

    let results = database::get_all_accounts(pool).await?;

    for result in &results {
        println!("Removing account {}", result.account());
    }

    database::delete_all_accounts(pool).await?;

    println!("All accounts removed");
    Ok(())
}

/// Remove a specific account from the database.
///
/// # Parameters
/// * `pool` - [sqlx::SqlitePool] of connections to the database
/// * `account` - name of the account to delete
pub async fn remove_account(pool: &SqlitePool, account: &String) -> anyhow::Result<()> {
    println!("Removing account {account}");

    database::delete_account(pool, account).await?;

    println!("{account} removed");
    Ok(())
}
