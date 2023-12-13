use std::fmt::{self};

use redis::{ErrorKind, FromRedisValue, RedisError, RedisResult, Value as RedisValue};
use serde::{Deserialize, Serialize};

use crate::app::models::session_metadata::SessionMetadata;

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenHeaderObject {
    pub kid: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GoogleKeys {
    pub keys: Vec<GoogleCert>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GoogleCert {
    pub kid: String,
    pub alg: String,
    pub n: String,
    pub e: String,
}

struct DisplayGoogleCerts<'a>(&'a Vec<GoogleCert>);

impl<'a> fmt::Display for DisplayGoogleCerts<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cert in self.0.iter() {
            writeln!(f, "\n{:?}", cert)?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub scope: String,
    pub expires_in: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenClaims {
    pub iss: String,
    pub azp: String,
    pub aud: String,
    pub sub: String,
    pub email: String,
    pub email_verified: bool,
    pub at_hash: String,
    pub name: String,
    pub picture: String,
    pub given_name: String,
    pub family_name: String,
    pub locale: String,
    pub iat: u32,
    pub exp: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub sub: String,
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub picture: String,
    pub given_name: String,
    pub family_name: String,
    pub locale: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginCacheData {
    pub pkce_code_verifier: String,
    pub session_metadata: SessionMetadata,
}

impl LoginCacheData {
    pub fn to_string(&self) -> &str {
        self.to_string()
    }
}
impl FromRedisValue for LoginCacheData {
    fn from_redis_value(value: &RedisValue) -> RedisResult<LoginCacheData> {
        match *value {
            RedisValue::Data(ref data) => Ok(serde_json::from_slice::<LoginCacheData>(data)?),
            _ => Err(RedisError::from((
                ErrorKind::TypeError,
                "Response was of incompatible type",
                format!("(response was {:?})", value),
            ))),
        }
    }
}
