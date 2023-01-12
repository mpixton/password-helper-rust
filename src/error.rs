use anyhow::anyhow;
use thiserror::Error;

/// Types of Errors that may occur in the program.
#[derive(Error, Debug)]
pub enum AppErrors {
    /// Error caused by reading input from the user
    #[error("Input is invalid")]
    Input(#[from] std::io::Error),
    /// Error caused by hashing input from the user
    #[error("Error hashing your input: {0}")]
    Hashing(#[from] argon2::password_hash::Error),
    /// Error occurred while communicating with the database
    #[error("Error occurred while communicating with the database")]
    Database(#[from] sqlx::Error),
    /// Specific error for a SQLX Database Error of unique constraint violation
    #[error("Account {0:?} already exists")]
    AccountAlreadyExists(String),
    /// Specific error for when record does not exists
    #[error("Account {0:?} does not exists")]
    AccountDoesNotExist(String),
}

/// Test for specific [sqlx::Error::Database] error code.'
/// 
/// If the `error_code` matches the [sqlx::Error::Database] code of the `result`, then throw 
/// `custom_error`. Else, the function will throw the error as is as an [anyhow::Error] with no
/// custom message.
/// 
/// # Parameters
/// * `result` - a [sqlx::Result] to have the Err value checked
/// * `error_code` - the [sqlx::Error::Database] `code` to check for
/// * `custom_error` - custom [anyhow::Error] to throw if the `result` Err code matches the `error_code`
pub async fn match_sqlx_error_code<T>(
    result: sqlx::Result<T>,
    error_code: &str,
    custom_error: anyhow::Error,
) -> anyhow::Result<()> {
    // This really ugly construct is the best way I found to get to the bottom of why the error was caused in
    // the database. 
    if let Err(outer_err) = result {
        match outer_err {
            sqlx::error::Error::Database(inner_err) => {
                if let Some(code) = inner_err.code() {
                    if *error_code == code {
                        Err(custom_error)
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

    Ok(())
}
