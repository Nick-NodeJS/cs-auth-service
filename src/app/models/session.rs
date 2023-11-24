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
