use crate::{
    database,
    error::{match_sqlx_error_code, AppErrors},
    password,
};
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

/// Adds an account and password to the database
/// 
/// # Parameters
/// * `pool` - [sqlx::SqlitePool] of connections to the database
/// * `account` - name of the account to add to the local database
pub async fn add_account(pool: &SqlitePool, account: &String) -> anyhow::Result<()> {
    println!("Adding account {account}");
    let pw = password::get_password_from_user()?;

    let result = database::add_account(pool, account, &pw).await;

    // Code 1555 is the specific code for a Unique Constraint violation. If the user is attempting 
    // to add an account that already exists, throw a custom error message instead of the uglier 
    // default one.
    match_sqlx_error_code(
        result,
        &"1555",
        anyhow!(AppErrors::AccountAlreadyExists(account.to_string())),
    ).await?;

    // println!("Pw hash is: {pw}");

    Ok(())
}
