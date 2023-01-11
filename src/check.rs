use crate::{database, password};
use clap::Args;
use sqlx::SqlitePool;

/// Check your knowledge of all accounts or a specific account's password
///
/// `account` is optional. If provided, test only the `account` given. If omitted, loop through
/// all accounts and test each account once.
#[derive(Args)]
pub struct Command {
    /// If provided, name of the account to test password knowledge of.
    pub account: Option<String>,
}

/// Iterate through all accounts in the database, check the password for each.
/// 
/// # Parameters
/// * `pool` - [sqlx::SqlitePool] of connections to the database
pub async fn check_all_accounts(pool: &SqlitePool) -> anyhow::Result<()> {
    println!("Testing you on your accounts...");

    let results = database::get_all_accounts(pool).await?;

    for result in &results {
        check_account(pool, &result.account()).await?;
    }

    Ok(())
}

/// Check a specific account's password.
/// 
/// # Parameters
/// * `pool` - [sqlx::SqlitePool] of connections to the database
/// * `account` - name of the account to test knowledge of
pub async fn check_account(pool: &SqlitePool, account: &String) -> anyhow::Result<()> {
    println!("Testing your knowledge  of {account}'s password");

    // TODO: give the user more than one attempt?
    let result = database::get_account(pool, account).await?;

    let correct = password::verify_password(result.password())?;

    println!("That is {correct}");

    Ok(())
}
