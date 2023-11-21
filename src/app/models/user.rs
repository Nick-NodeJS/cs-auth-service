use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GoogleProfile {
    pub user_id: String,
    pub name: Option<String>,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FacebookProfile {
    pub user_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserProfile {
    Google(GoogleProfile),
    Facebook(FacebookProfile),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserActiveProfile {
    Google,
    Facebook,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    active_profile: UserActiveProfile,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    google: Option<GoogleProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    facebook: Option<FacebookProfile>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(active_profile: UserActiveProfile, profile: UserProfile) -> User {
        let now = Utc::now();
        let mut user = User {
            id: Some(ObjectId::new()),
            active_profile,
            google: None,
            facebook: None,
            created_at: now,
            updated_at: now,
        };
        match profile {
            UserProfile::Google(google_profile) => user.google = Some(google_profile),
            UserProfile::Facebook(facebook_profile) => user.facebook = Some(facebook_profile),
        }
        user
    }
}
