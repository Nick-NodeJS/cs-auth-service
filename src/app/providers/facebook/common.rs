use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::app::models::{session_tokens::SessionTokens, token::Token};

#[derive(Debug, Deserialize, Serialize)]
pub struct FacebookTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

pub fn get_session_tokens(tokens: FacebookTokenResponse) -> SessionTokens {
    let expire = Some(Utc::now() + Duration::seconds(tokens.expires_in));
    SessionTokens {
        access_token: Some(Token {
            token_string: tokens.access_token,
            expire,
        }),
        refresh_token: None,
        extra_token: None,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub app_id: String,
    pub r#type: String,
    pub application: String,
    pub data_access_expires_at: u32,
    pub expires_at: u32,
    pub is_valid: bool,
    pub issued_at: u32,
    pub scopes: Vec<String>,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDebugData {
    #[serde(default)]
    pub data: Option<TokenData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PictureData {
    height: u32,
    is_silhouette: bool,
    pub url: String,
    width: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Picture {
    #[serde(default)]
    pub data: Option<PictureData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FacebookUserInfo {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub picture: Option<Picture>,
}

pub const USER_PROFILE_FIELDS: &str = "id,name,email,picture";

#[derive(Debug, Deserialize, Serialize)]
pub struct FacebookUserDeleteResponse {
    #[serde(default)]
    pub success: Option<bool>,
    // TODO: check the error response
    #[serde(default)]
    pub error: Option<String>,
}
