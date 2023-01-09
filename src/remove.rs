use clap::Args;
use sqlx::SqlitePool;

use crate::{database, error};

/// Remove all accounts or specific account
#[derive(Args)]
pub struct Remove {
    pub account: Option<String>,
}

/// Remove all accounts from the database.
pub async fn remove_all_accounts(pool: &SqlitePool) -> Result<(), error::AppErrors> {
    // let accounts = ["Bw", "Linux-Work"];
    // println!("Removing all accounts...");
    // for account in accounts {
    //     println!("Removing {account}");
    // }

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
pub async fn remove_account(pool: &SqlitePool, account: &String) -> Result<(), error::AppErrors> {
    println!("Removing account {account}");

    database::delete_account(pool, account).await?;

    println!("{account} removed");
    Ok(())
}
