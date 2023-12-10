use actix_web::cookie::Cookie;
use bson::{oid::ObjectId, serde_helpers::chrono_datetime_as_bson_datetime};
use chrono::{DateTime, Utc};
use redis::{ErrorKind, FromRedisValue, RedisError, RedisResult, Value as RedisValue};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use super::{
    common::AuthProviders, session_metadata::SessionMetadata, session_tokens::SessionTokens,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub auth_provider: AuthProviders,
    pub user_id: ObjectId,
    pub session_id: Uuid,
    pub tokens: SessionTokens,
    pub metadata: SessionMetadata,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
}

impl Session {
    pub fn new(auth_provider: AuthProviders, tokens: SessionTokens, user_id: ObjectId) -> Self {
        let now = Utc::now();
        Session {
            auth_provider,
            user_id,
            // TODO: update session_id generation according to actix-web example
            session_id: Uuid::new_v4(),
            tokens,
            metadata: SessionMetadata::new(),
            created_at: now,
            updated_at: now,
        }
    }
    pub fn get_session_key(session: &Session) -> String {
        format!("session::{}", session.session_id)
    }
    pub fn get_user_sessions_key(user_id: &str) -> String {
        format!("user::sessions::{}", user_id)
    }
    pub fn session_to_cache_string(session: &Session) -> Result<String, serde_json::Error> {
        serde_json::to_string::<Session>(session)
    }
    pub fn get_id_json(session: Session) -> Value {
        // it should use encrypted session_id(see actix-web example)
        json!({
            "session": session.session_id
        })
    }
    pub fn set_session_cookie(session: Session) -> Cookie<'static> {
        // it should gets session cookie with encrypted session_id
        todo!()
    }
}

impl FromRedisValue for Session {
    fn from_redis_value(value: &RedisValue) -> RedisResult<Session> {
        match *value {
            RedisValue::Data(ref data) => Ok(serde_json::from_slice::<Session>(data)?),
            _ => Err(RedisError::from((
                ErrorKind::TypeError,
                "Response was of incompatible type",
                format!("(response was {:?})", value),
            ))),
        }
    }
}
