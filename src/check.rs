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
    let mut count_correct = 0;

    println!("Testing you on your accounts...");

    let results = database::get_all_accounts(pool).await?;

    for result in &results {
        let correct = _check_account_password(pool, &result.account()).await?;

        // TODO
        // Add a flag to allow users to go blind, not alerting them to the outcome of this account?
        println!("That is {correct}");

        count_correct = if correct {
            count_correct + 1
        } else {
            count_correct
        };
    }

    // TODO
    // Maybe provide for different message based on % correct?
    println!("Total Accounts: {}", results.len());
    println!("Total Correct: {count_correct}");

    Ok(())
}

/// Check a specific account's password.
///
/// # Parameters
/// * `pool` - [sqlx::SqlitePool] of connections to the database
/// * `account` - name of the account to test knowledge of
pub async fn check_account(pool: &SqlitePool, account: &String) -> anyhow::Result<()> {
    let correct = _check_account_password(pool, account).await?;

    println!("That is {correct}");

    Ok(())
}

/// Does the actual dirty work of checking a user's input against a known hash.
///
/// Function is separated out to allow for aggregation when called against a list of accounts and
/// to allow the [check_account] function to have a similar return type as the other commands
/// while minimizing code duplication.
async fn _check_account_password(pool: &SqlitePool, account: &String) -> anyhow::Result<bool> {
    println!("Testing your knowledge  of {account}'s password");

    // TODO: give the user more than one attempt?
    let result = database::get_account(pool, account).await?;

    password::verify_password(result.password())
}
