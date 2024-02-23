use bson::doc;
use serde::{Deserialize, Serialize};

use super::{common::AuthProviders, user::UserId};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CyberSherlockProfile {
    pub user_id: UserId,
    pub name: String,
    pub email: Option<String>,
    pub email_verified: bool,
    pub phone: Option<String>,
    pub phone_verified: bool,
    pub picture: Option<String>,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleProfile {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FacebookProfile {
    pub user_id: String,
    pub name: String,
    pub email: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserProfile {
    CyberSherlock(CyberSherlockProfile),
    Google(GoogleProfile),
    Facebook(FacebookProfile),
}

impl UserProfile {
    pub fn get_provider(profile: &UserProfile) -> AuthProviders {
        match profile {
            UserProfile::Google(_) => AuthProviders::Google,
            UserProfile::Facebook(_) => AuthProviders::Facebook,
            _ => AuthProviders::CyberSherlock,
        }
    }
}
