use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppErrors {
    #[error("Input is invalid")]
    InputError(#[from] std::io::Error),
    #[error("Error occurred while hashing your input")]
    HashingError(#[from] argon2::password_hash::Error),
    #[error("Error occurred while communicating with the database")]
    DatabaseError,
}