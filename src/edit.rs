use crate::{database, password};
use clap::Args;
use sqlx::SqlitePool;

/// Edit an account's password
///
/// Edit the <ACCOUNT>'s password. The command will silently fail if the <ACCOUNT> has not been previously
#[derive(Args)]
pub struct Command {
    /// Name of the account to edit
    pub account: String,
}

pub async fn edit_account(pool: &SqlitePool, account: &String) -> anyhow::Result<()> {
    println!("Editing account {account}");

    let new_password = password::get_password_from_user()?;

    database::update_account_password(pool, account, &new_password).await?;

    println!("{account} was updated");

    Ok(())
}
