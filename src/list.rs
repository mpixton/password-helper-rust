use crate::database;
use clap::Args;
use sqlx::SqlitePool;

/// List all accounts
#[derive(Args)]
pub struct Command {}

/// List all acounts in the database.
///
/// Lists all accounts stored, showing a help message if they have no accounts.
///
/// # Parameters
/// * `pool` - [sqlx::SqlitePool] of connections to the database
pub async fn list_all_accounts(pool: &SqlitePool) -> anyhow::Result<()> {
    log::info!("Listing all accounts...");

    let results = database::get_all_accounts(pool).await?;

    if results.is_empty() {
        log::info!("You currently have no accounts saved.");
        log::info!("Use the command 'add <ACCOUNT>' to add an account.");
    }

    for result in &results {
        log::info!("{}", result.account());
    }

    Ok(())
}
