use clap::Args;
use sqlx::SqlitePool;

use crate::{database, error, password};

/// Check your knowledge of all accounts or a specific account's password
#[derive(Args)]
pub struct Check {
    pub account: Option<String>,
}

/// Iterate through all accounts in the database, check the password for each.
pub async fn check_all_accounts(pool: &SqlitePool) -> Result<(), error::AppErrors> {
    println!("Checking accounts...");

    let results = database::get_all_accounts(pool).await?;

    for result in &results {
        let correct = password::verify_password(result.password()).await?;
        println!("That is {correct}");
    }

    Ok(())
}

/// Check a specific account's password.
pub async fn check_account(pool: &SqlitePool, account: &String) -> Result<(), error::AppErrors> {
    println!("Checking account {account}");

    let result = database::get_account(pool, account).await?;

    let correct = password::verify_password(result.password()).await?;

    println!("That is {correct}");

    Ok(())
}
