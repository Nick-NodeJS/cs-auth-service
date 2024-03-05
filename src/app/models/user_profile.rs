use bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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
impl CyberSherlockProfile {
    pub fn to_json(&self) -> Value {
        json!({
            "user_id": self.user_id,
            "name": self.name,
            "email": self.email,
            "email_verified": self.email_verified,
            "phone": self.phone,
            "phone_verified": self.phone_verified,
            "picture": self.picture,
        })
    }
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
