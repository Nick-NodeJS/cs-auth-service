use bson::doc;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::app::services::storage::service::CollectionType;

use super::common::AuthProviders;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CyberSherlockProfile {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub picture: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleProfile {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub picture: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FacebookProfile {
    pub user_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    active_profile: AuthProviders,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cybersherlock: Option<CyberSherlockProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    google: Option<GoogleProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    facebook: Option<FacebookProfile>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(profile: UserProfile) -> User {
        let now = Utc::now();
        let mut user = User {
            id: ObjectId::new(),
            active_profile: AuthProviders::CyberSherlock,
            cybersherlock: None,
            google: None,
            facebook: None,
            created_at: now,
            updated_at: now,
        };
        match profile {
            UserProfile::CyberSherlock(cyber_sherlock_profile) => {
                user.cybersherlock = Some(cyber_sherlock_profile);
                user.active_profile = AuthProviders::CyberSherlock;
            }
            UserProfile::Google(google_profile) => {
                user.google = Some(google_profile);
                user.active_profile = AuthProviders::Google;
            }
            UserProfile::Facebook(facebook_profile) => {
                user.facebook = Some(facebook_profile);
                user.active_profile = AuthProviders::Facebook;
            }
        }
        user
    }
    pub fn get_user_cache_key(user_id: &str) -> String {
        format!("user::{}", user_id)
    }
    pub fn user_to_cache_string(user: User) -> Result<String, serde_json::Error> {
        serde_json::to_string::<User>(&user)
    }
}

impl CollectionType for User {}
