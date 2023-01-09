use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppErrors {
    #[error("Input is invalid")]
    InputError(#[from] std::io::Error),
    #[error("Error hashing your input: {0}")]
    HashingError(#[from] argon2::password_hash::Error),
    #[error("Error occurred while communicating with the database")]
    DatabaseError(#[from] sqlx::Error),
}
