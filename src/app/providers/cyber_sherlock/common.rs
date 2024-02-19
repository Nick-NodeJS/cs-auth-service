use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use redis::{ErrorKind, FromRedisValue, RedisError, RedisResult, Value as RedisValue};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::app::models::session_metadata::SessionMetadata;

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenClaims {
    pub aud: String,
    pub sub: String,
    pub email: Option<String>,
    pub email_verified: bool,
    pub phone: Option<String>,
    pub phone_verified: bool,
    pub name: String,
    pub exp: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshTokenClaims {
    pub aud: String,
    pub sub: String,
    pub exp: i64,
}

pub fn check_email_and_phone(
    email: &Option<String>,
    phone: &Option<String>,
) -> Result<(), ValidationError> {
    if email.is_none() && phone.is_none() {
        let error = "Must have `email` or `phone` field";
        log::warn!("RegisterQueryData validation error: {}", error);
        return Err(ValidationError::new(error));
    }
    Ok(())
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct RegisterQueryData {
    #[validate(email)]
    pub email: Option<String>,
    #[validate(phone)]
    pub phone: Option<String>,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub verify_password: String,
}
impl RegisterQueryData {
    pub fn validate(&self) -> Result<(), ValidationError> {
        check_email_and_phone(&self.email, &self.phone)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct LoginQueryData {
    #[validate(email)]
    pub email: Option<String>,
    #[validate(phone)]
    pub phone: Option<String>,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
}
impl LoginQueryData {
    pub fn validate(&self) -> Result<(), ValidationError> {
        check_email_and_phone(&self.email, &self.phone)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterCacheData {
    pub pkce_code_verifier: String,
    pub session_metadata: SessionMetadata,
    pub hash: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl FromRedisValue for RegisterCacheData {
    fn from_redis_value(value: &RedisValue) -> RedisResult<RegisterCacheData> {
        match *value {
            RedisValue::Data(ref data) => Ok(serde_json::from_slice::<RegisterCacheData>(data)?),
            _ => Err(RedisError::from((
                ErrorKind::TypeError,
                "Response was of incompatible type",
                format!("(response was {:?})", value),
            ))),
        }
    }
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
