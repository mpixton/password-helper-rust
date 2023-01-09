use clap::Args;
use sqlx::SqlitePool;

use crate::database;
use crate::error;

/// List all accounts
#[derive(Args)]
pub struct List {}

/// List all acounts in the database, showing a help message if they have no accounts.
pub async fn list_all_accounts(pool: &SqlitePool) -> Result<(), error::AppErrors> {
    println!("Listing all accounts...");

    let results = database::get_all_accounts(pool).await?;

    if results.len() == 0 {
        println!("You currently have no accounts saved.");
        println!("Use the command 'add <ACCOUNT>' to add an account.")
    }

    for result in &results {
        println!("{}", result.account());
    }

    Ok(())
}
