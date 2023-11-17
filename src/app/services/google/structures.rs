use std::collections::HashMap;

use jsonwebtoken::DecodingKey;
use oauth2::basic::BasicClient;
use serde::{Deserialize, Serialize};

use crate::{app::services::cache::service::CacheService, config::google_config::GoogleConfig};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenHeaderObject {
    pub kid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleKeys {
    pub keys: Vec<GoogleCert>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleCert {
    pub kid: String,
    pub alg: String,
    pub n: String,
    pub e: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleTokens {
    pub access_token: String,
    pub id_token: String,
    pub refresh_token: Option<String>,
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
