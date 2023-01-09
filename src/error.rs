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
}
