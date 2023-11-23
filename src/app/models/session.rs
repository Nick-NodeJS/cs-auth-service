use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::common::AuthProviders;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    auth_provider: AuthProviders,
    user_id: ObjectId,
    session_id: ObjectId,
    refresh_token: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Session {
    pub fn new(auth_provider: AuthProviders, user_id: ObjectId, refresh_token: String) -> Self {
        let now = Utc::now();
        Session {
            auth_provider,
            user_id,
            session_id: ObjectId::new(),
            refresh_token,
            created_at: now,
            updated_at: now,
        }
    }
    pub fn get_session_key(user_id: ObjectId, auth_provider: AuthProviders) -> String {
        format!("session::{}::{:?}", user_id, auth_provider)
    }
}
