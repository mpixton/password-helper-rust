use crate::{database, error::AppErrors, password};
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

    log::info!("Testing your knowledge of your passwords");

    let accounts = database::get_all_accounts(pool).await?;

    for account in &accounts {
        let correct = _check_account_password(account).await?;

        // TODO
        // Add a flag to allow users to go blind, not alerting them to the outcome of this account?
        log::info!("That is {correct}");

        count_correct = if correct {
            count_correct + 1
        } else {
            count_correct
        };
    }

    if accounts.is_empty() {
        log::info!("You currently have no accounts saved.");
        log::info!("Use the command 'add <ACCOUNT>' to add an account.");
    } else {
        // TODO
        // Maybe provide for different message based on % correct?
        log::info!("Total Accounts: {}", accounts.len());
        log::info!("Total Correct: {count_correct}");
    }

    Ok(())
}

/// Check a specific account's password.
///
/// # Parameters
/// * `pool` - [sqlx::SqlitePool] of connections to the database
/// * `account` - name of the account to test knowledge of
pub async fn check_account(pool: &SqlitePool, account: &String) -> anyhow::Result<()> {
    // TODO: give the user more than one attempt?
    let result = database::get_account(pool, account).await?;

    if let Some(account) = result {
        let correct = _check_account_password(&account).await?;
        log::info!("That is {correct}");
        Ok(())
    } else {
        Err(anyhow::anyhow!(AppErrors::AccountDoesNotExist(
            account.to_string()
        )))
    }
}

/// Does the actual dirty work of checking a user's input against a known hash.
///
/// Function is separated out to allow for aggregation when called against a list of accounts and
/// to allow the [check_account] function to have a similar return type as the other commands
/// while minimizing code duplication.
async fn _check_account_password(account: &database::Account) -> anyhow::Result<bool> {
    log::info!(
        "Testing your knowledge  of {}'s password",
        account.account()
    );

    password::verify_password(account.password())
}
