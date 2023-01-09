use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rpassword::prompt_password;

/// Prompts the user for a password, hides the input, hashes the result, and returns the hash.
pub async fn get_password_from_user() -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    let hasher = Argon2::default();

    let password = prompt_password("Password:  ")?;

    let hash = hasher.hash_password(password.as_bytes(), &salt)?;

    // println!("Password is: {password}");
    // println!("Hash is: {hash}");

    Ok(hash.to_string())
}

/// Prompts the user for a password, then compares it to a hash to see if they are the same.
pub async fn verify_password(password_hash: &String) -> anyhow::Result<bool> {
    let attempt = prompt_password("Password:  ")?;

    let parsed_hash = PasswordHash::new(&password_hash)?;

    let passwords_match = Argon2::default()
        .verify_password(attempt.as_bytes(), &parsed_hash)
        .is_ok();

    // println!("Passwords match? {passwords_match}");

    Ok(passwords_match)
}
