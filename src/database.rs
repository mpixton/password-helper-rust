use sqlx::{FromRow, SqlitePool};

/// Rust representation of the table in the database.
#[derive(FromRow)]
pub struct Account {
    account: String,
    password: String,
}

impl Account {
    /// Getter for the account prop.
    pub fn account(&self) -> &String {
        &self.account
    }

    /// Getter for the password prop.
    pub fn password(&self) -> &String {
        &self.password
    }
}

/// Set up the passwords table to store the account and passwords.
pub async fn setup_db(pool: &SqlitePool) -> sqlx::Result<()> {
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

    Ok(())
}

/// List all accounts stored in the database.
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
