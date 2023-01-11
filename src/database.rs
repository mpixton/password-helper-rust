//! Handles all calls made to the database.

use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};

/// Convenience struct providing read-only access to the account and password as stored in the database.
#[derive(FromRow)]
pub struct Account {
    account: String,
    password: String,
}

impl Account {
    /// Getter for `account`
    pub fn account(&self) -> &String {
        &self.account
    }

    /// Getter for `password`
    pub fn password(&self) -> &String {
        &self.password
    }
}

/// Set up the passwords table to store the account and passwords.
///
/// This code is called the first time that the program is called and it detects that there is no
/// database found at the expected location.
pub async fn setup_db() -> sqlx::Result<&'static str> {
    const DB_URL: &str = "sqlite://./passwords.db";

    if Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        // TODO
        // Allow for a verbose flag to be passed to the program that will control if this gets
        // logged or not. Linked to the larger todo of replacing println! with loq!.
        // println!("Database already exists");
    } else {
        println!("Creating local database...");
        match Sqlite::create_database(&DB_URL).await {
            Ok(_) => {
                println!("Create db success");
                println!("Setting up database...");

                // Disable WAL to prevent those pesky files from showing up?
                let pool = SqlitePool::connect(&DB_URL).await?;

                let mut conn = pool.acquire().await?;

                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS passwords (
                        account TEXT PRIMARY KEY,
                        password TEXT
                    );
                "#,
                )
                .execute(&mut conn)
                .await?;

                println!("Database set up");
            }
            Err(error) => panic!("error: {}", error),
        };
    };

    Ok(&DB_URL)
}

/// List all accounts stored in the database.
///
/// # Parameters
/// * `pool` - [sqlx::SqlitePool] of connections to the database
pub async fn get_all_accounts(pool: &SqlitePool) -> sqlx::Result<Vec<Account>> {
    let mut conn = pool.acquire().await?;

    let results: Vec<Account> = sqlx::query_as(
        r#"
        SELECT 
            account, 
            password
        FROM 
            passwords
        ;
    "#,
    )
    .fetch_all(&mut conn)
    .await?;

    Ok(results)
}

/// Find an account in the database.
///
/// # Parameters
/// * `pool` - [sqlx::SqlitePool] of connections to the database
/// * `account` - name of the account to find
pub async fn get_account(pool: &SqlitePool, account: &String) -> sqlx::Result<Account> {
    let mut conn = pool.acquire().await?;

    let result: Account = sqlx::query_as(
        r#"
        SELECT 
            account,
            password
        FROM
            passwords
        WHERE
            account = ?
        ;
    "#,
    )
    .bind(account)
    .fetch_one(&mut conn)
    .await?;

    Ok(result)
}

/// Edit an account's password.
///
/// # Parameters
/// * `pool` - [sqlx::SqlitePool] of connections to the database
/// * `account` - name of the account to edit
/// * `new_password_hash` - hash of the password to replace the current password hash
pub async fn update_account_password(
    pool: &SqlitePool,
    account: &String,
    new_password_hash: &String,
) -> sqlx::Result<()> {
    let mut conn = pool.acquire().await?;

    sqlx::query(
        r#"
        UPDATE 
            passwords
        SET 
            password = ?
        WHERE
            account = ? 
        ;
        "#,
    )
    .bind(new_password_hash)
    .bind(account)
    .execute(&mut conn)
    .await?;

    Ok(())
}

/// Add an account and hashed password to the database.
///
/// # Parameters
/// * `pool` - [sqlx::SqlitePool] of connections to the database
/// * `account` - name of the account to add
/// * `password` - hash of the password to add
pub async fn add_account(
    pool: &SqlitePool,
    account: &String,
    password: &String,
) -> sqlx::Result<()> {
    let mut conn = pool.acquire().await?;

    sqlx::query(
        r#"
        INSERT INTO 
            passwords (account, password)
        VALUES 
            (?, ?)
        ;
        "#,
    )
    .bind(account)
    .bind(password)
    .execute(&mut conn)
    .await?;

    Ok(())
}

/// Delete a specific account from the database.
///
/// /// # Parameters
/// * `pool` - [sqlx::SqlitePool] of connections to the database
/// * `account` - name of the account to delete
pub async fn delete_account(pool: &SqlitePool, account: &String) -> sqlx::Result<()> {
    let mut conn = pool.acquire().await?;

    sqlx::query(
        r#"
        DELETE FROM 
            passwords
        WHERE 
            account = ?
        ;
    "#,
    )
    .bind(account)
    .execute(&mut conn)
    .await?;

    Ok(())
}

/// Delete all accounts from the database.
///
/// # Parameters
/// * `pool` - [sqlx::SqlitePool] of connections to the database
pub async fn delete_all_accounts(pool: &SqlitePool) -> sqlx::Result<()> {
    let mut conn = pool.acquire().await?;

    sqlx::query(
        r#"
        DELETE FROM 
            passwords
        ;
    "#,
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(r#"VACUUM;"#).execute(&mut conn).await?;

    Ok(())
}
