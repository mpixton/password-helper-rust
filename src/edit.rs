use crate::{database, error, password};
use clap::Args;
use sqlx::SqlitePool;

/// Edit an account's password
#[derive(Args)]
pub struct Edit {
    pub account: String,
}

pub async fn edit_account(pool: &SqlitePool, account: &String) -> Result<(), error::AppErrors> {
    println!("Editing account {account}");

    let new_password = password::get_password_from_user().await?;

    database::update_account_password(pool, account, &new_password).await?;

    println!("{account} was updated");

    Ok(())
}
