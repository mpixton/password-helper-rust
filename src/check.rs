use crate::{database, password};
use clap::Args;
use sqlx::SqlitePool;

/// Check your knowledge of all accounts or a specific account's password
///
/// [ACCOUNT] is optional. If provided, test only the <ACCOUNT> given. If omitted, loop through
/// all accounts and test each account once.
#[derive(Args)]
pub struct Check {
    /// If provided, name of the account to test password knowledge of.
    pub account: Option<String>,
}

/// Iterate through all accounts in the database, check the password for each.
pub async fn check_all_accounts(pool: &SqlitePool) -> anyhow::Result<()> {
    println!("Checking accounts...");

    let results = database::get_all_accounts(pool).await?;

    for result in &results {
        let correct = password::verify_password(result.password()).await?;
        println!("That is {correct}");
    }

    Ok(())
}

/// Check a specific account's password.
pub async fn check_account(pool: &SqlitePool, account: &String) -> anyhow::Result<()> {
    println!("Checking account {account}");

    let result = database::get_account(pool, account).await?;

    let correct = password::verify_password(result.password()).await?;

    println!("That is {correct}");

    Ok(())
}
