use bson::{doc, Document};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::common::AuthProviders;

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
    Google(GoogleProfile),
    Facebook(FacebookProfile),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    active_profile: AuthProviders,
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
            active_profile: AuthProviders::Google,
            google: None,
            facebook: None,
            created_at: now,
            updated_at: now,
        };
        match profile {
            UserProfile::Google(google_profile) => user.google = Some(google_profile),
            UserProfile::Facebook(facebook_profile) => {
                user.facebook = Some(facebook_profile);
                user.active_profile = AuthProviders::Facebook;
            }
        }
        user
    }

    pub fn get_update_user_profile_query(user_profile: UserProfile) -> Document {
        let mut data_to_update = match user_profile {
            UserProfile::Google(google_profile) => {
                doc! {
                    "google.name": google_profile.name,
                    "google.email": google_profile.email,
                    "google.email_verified": google_profile.email_verified,
                    "google.picture": google_profile.picture,
                }
            }
            UserProfile::Facebook(facebook_profile) => {
                doc! {
                    "facebook.name": facebook_profile.name,
                    "facebook.email": facebook_profile.email,
                }
            }
        };
        data_to_update.insert("updated_at", Utc::now());

        data_to_update
    }
    pub fn get_find_user_by_profile_query(user_profile: UserProfile) -> Document {
        let mut query = doc! {};
        match user_profile {
            UserProfile::Google(google_profile) => {
                query.insert("google.user_id", google_profile.user_id);
            }
            UserProfile::Facebook(facebook_profile) => {
                query.insert("facebook.user_id", facebook_profile.user_id);
            }
        }

        query
    }
    pub fn get_find_user_by_id_query(user_id: ObjectId) -> Document {
        doc! {
            "_id": user_id
        }
    }
}
