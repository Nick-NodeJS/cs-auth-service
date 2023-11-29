use std::collections::HashMap;

use bson::{oid::ObjectId, serde_helpers::chrono_datetime_as_bson_datetime};
use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};

use super::common::AuthProviders;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub auth_provider: AuthProviders,
    pub user_id: ObjectId,
    session_id: ObjectId,
    pub token: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
}

impl Session {
    pub fn new(auth_provider: AuthProviders, user_id: ObjectId, token: String) -> Self {
        let now = Utc::now();
        Session {
            auth_provider,
            user_id,
            session_id: ObjectId::new(),
            token,
            created_at: now,
            updated_at: now,
        }
    }
    pub fn get_session_key(user_id: ObjectId, auth_provider: AuthProviders) -> String {
        format!("session::{}::{:?}", user_id, auth_provider)
    }
    pub fn from_hashmap(map: HashMap<String, String>) -> Result<Self, &'static str> {
        // Extract values from the HashMap
        let auth_provider = map.get("auth_provider").ok_or("Missing auth_provider")?;
        let user_id = map.get("user_id").ok_or("Missing user_id")?;
        let session_id = map.get("session_id").ok_or("Missing session_id")?;
        let token = map.get("token").ok_or("Missing token")?;
        let created_at = map.get("created_at").ok_or("Missing created_at")?;
        let updated_at = map.get("updated_at").ok_or("Missing updated_at")?;

        // Parse or convert values to the desired types
        let auth_provider: AuthProviders =
            auth_provider.parse().map_err(|_| "Invalid auth_provider")?;
        let user_id: ObjectId = user_id.parse().map_err(|_| "Invalid user_id")?;
        let session_id: ObjectId = session_id.parse().map_err(|_| "Invalid session_id")?;
        let created_at: DateTime<Utc> = DateTime::parse_from_rfc3339(created_at)
            .map_err(|_| "Invalid created_at format")?
            .with_timezone(&Utc);
        let updated_at: DateTime<Utc> = DateTime::parse_from_rfc3339(updated_at)
            .map_err(|_| "Invalid updated_at format")?
            .with_timezone(&Utc);

        // Create a Session instance
        Ok(Session {
            auth_provider,
            user_id,
            session_id,
            token: token.to_string(),
            created_at,
            updated_at,
        })
    }
}

pub fn session_as_key_value_vec(session: Session) -> Vec<(&'static str, String)> {
    vec![
        ("auth_provider", format!("{:?}", session.auth_provider)),
        ("user_id", session.user_id.to_string()),
        ("session_id", session.session_id.to_string()),
        ("token", format!("{}", session.token)),
        (
            "created_at",
            format!(
                "{}",
                session
                    .created_at
                    .to_rfc3339_opts(SecondsFormat::Millis, true)
            ),
        ),
        (
            "updated_at",
            format!(
                "{}",
                session
                    .updated_at
                    .to_rfc3339_opts(SecondsFormat::Millis, true)
            ),
        ),
    ]
}
