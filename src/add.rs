use crate::{database, error::AppErrors, password};
use anyhow::anyhow;
use clap::Args;
use sqlx::SqlitePool;

/// Add an account
///
/// Asks a user for a password to store alongside the `account`. Only one `account` name may be
/// present in the database at a time.
#[derive(Args)]
pub struct Command {
    /// Name of the account to add
    pub account: String,
}

/// Ask a user for their account's password, and then store the account and password hash in the database.
pub async fn add_account(pool: &SqlitePool, account: &String) -> anyhow::Result<()> {
    println!("Adding account {account}");
    let pw = password::get_password_from_user()?;

    let result = database::add_account(pool, account, &pw).await;

    // This really ugly construct is the best way I found to get to the bottom of why the error was caused in
    // the database. Ultimately, errors caused by code 1555, which is the code for a Unique Constraint violation,
    // should be handled a little different. Instead of an uglier error message, we want a pretty message that
    // alerts the user that the account is already in the database. This error should be the most common one,
    // which is why we are not handling other errors.
    if let Err(outer_err) = result {
        match outer_err {
            sqlx::error::Error::Database(inner_err) => {
                if let Some(code) = inner_err.code() {
                    if *"1555" == code {
                        Err(anyhow!(AppErrors::AccountAlreadyExists(
                            account.to_string()
                        )))
                    } else {
                        Err(anyhow!(inner_err))
                    }
                } else {
                    Err(anyhow!(inner_err))
                }
            }
            _ => Err(anyhow!(AppErrors::Database(outer_err))),
        }?;
    };

    // println!("Pw hash is: {pw}");

    Ok(())
}
