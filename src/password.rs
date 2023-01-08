use rpassword::prompt_password;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use crate::error::AppErrors;

/// Prompts the user for a password, hides the input, hashes the result, and returns the hash.
pub fn get_password_from_user() -> anyhow::Result<String, AppErrors> {
    let password = prompt_password("Password:  ")?;

    let salt = SaltString::generate(&mut OsRng);
    
    let hasher = Argon2::default();

    let hash = hasher.hash_password(password.as_bytes(), &salt)?;

    println!("Password is: {password}");
    println!("Hash is: {hash}");

    Ok(hash.to_string())
}

#[allow(dead_code)]
pub fn verify_password(password_hash: &String) -> anyhow::Result<bool, AppErrors> {
    let attempt = prompt_password("Password:  ")?;

    let parsed_hash = PasswordHash::new(&password_hash)?;

    let passwords_match = Argon2::default().verify_password(attempt.as_bytes(), &parsed_hash).is_ok();

    println!("Passwords match? {passwords_match}");

    Ok(passwords_match)
}
