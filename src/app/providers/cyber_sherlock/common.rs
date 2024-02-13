use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::app::models::session_metadata::SessionMetadata;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct LoginQueryData {
    #[validate(email)]
    pub email: Option<String>,
    #[validate(phone)]
    pub mobile: Option<String>,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub verify_password: String,
}
impl LoginQueryData {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.email.is_none() && self.mobile.is_none() {
            let error = "Must have `email` or `mobile` field";
            log::warn!("LoginQueryData validation error: {}", error);
            return Err(ValidationError::new(error));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginCacheData {
    pub pkce_code_verifier: String,
    pub session_metadata: SessionMetadata,
    pub hash: String,
}

// Function to hash a password
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    // Generate a salt
    let salt = SaltString::generate(&mut OsRng);

    // Configure the argon2 hash parameters
    let argon2 = Argon2::default();

    // Hash the password with the salt and config
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(hash)
}

// Function to verify a password against its hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(&hash)?;
    // Verify the password against the hash
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
