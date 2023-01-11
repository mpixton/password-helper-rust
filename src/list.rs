use crate::database;
use clap::Args;
use sqlx::SqlitePool;

/// List all accounts
#[derive(Args)]
pub struct Command {}

/// List all acounts in the database, showing a help message if they have no accounts.
pub async fn list_all_accounts(pool: &SqlitePool) -> anyhow::Result<()> {
    println!("Listing all accounts...");

    let results = database::get_all_accounts(pool).await?;

    if results.is_empty() {
        println!("You currently have no accounts saved.");
        println!("Use the command 'add <ACCOUNT>' to add an account.");
    }

    for result in &results {
        println!("{}", result.account());
    }

    Ok(())
}
