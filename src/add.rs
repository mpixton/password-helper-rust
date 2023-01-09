use crate::{database, error::AppErrors, password};
use clap::Args;
use sqlx::SqlitePool;

/// Add an account
#[derive(Args)]
pub struct Add {
    pub account: String,
}

/// Ask a user for their account's password, and then store the account and password hash in the database.
pub async fn add_account(pool: &SqlitePool, account: &String) -> Result<(), AppErrors> {
    println!("Adding account {account}");
    let pw = password::get_password_from_user().await?;

    database::add_account(&pool, account, &pw).await?;

    println!("Pw hash is: {pw}");
    Ok(())
}
