use crate::{database, password};
use clap::Args;
use sqlx::SqlitePool;

/// Edit an account's password
///
/// Edit the `accounts`'s password. The command will silently fail if the `account` has not been
/// previously added.
#[derive(Args)]
pub struct Command {
    /// Name of the account to edit
    pub account: String,
}

/// Edit an account's password.
///
/// # Parameters
/// * `pool` - [sqlx::SqlitePool] of connections to the database
/// * `account` - name of the account to edit the password for
pub async fn edit_account(pool: &SqlitePool, account: &String) -> anyhow::Result<()> {
    // TODO
    // Should this create a new account if the `account` provided doesn't exist?
    println!("Editing account {account}");

    let new_password = password::get_password_from_user()?;

    database::update_account_password(pool, account, &new_password).await?;

    println!("{account} was updated");

    Ok(())
}
