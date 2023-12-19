use actix_web::cookie::Cookie;
use bson::{oid::ObjectId, serde_helpers::chrono_datetime_as_bson_datetime};
use chrono::{DateTime, Utc};
use redis::{ErrorKind, FromRedisValue, RedisError, RedisResult, Value as RedisValue};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use std::convert::TryInto;

use rand::{distributions::Alphanumeric, rngs::OsRng, Rng as _};

use super::{
    common::AuthProviders, session_metadata::SessionMetadata, session_tokens::SessionTokens,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewSessionData {
    pub auth_provider: AuthProviders,
    pub user_id: ObjectId,
    pub tokens: SessionTokens,
    pub session_metadata: SessionMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub auth_provider: AuthProviders,
    pub user_id: ObjectId,
    pub id: String,
    pub tokens: SessionTokens,
    pub metadata: SessionMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Session {
    pub fn new(session_data: NewSessionData) -> Self {
        let now = Utc::now();
        Session {
            auth_provider: session_data.auth_provider,
            user_id: session_data.user_id,
            // TODO: update session_id generation according to actix-web example
            id: Session::generate_session_id(),
            tokens: session_data.tokens,
            metadata: session_data.session_metadata,
            created_at: now,
            updated_at: now,
        }
    }
    pub fn generate_session_id() -> String {
        //format!("session::{}", session.session_id)

        // Session key generation routine that follows [OWASP recommendations].
        //
        // [OWASP recommendations]: https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html#session-id-entropy
        //
        let value = std::iter::repeat(())
            .map(|()| OsRng.sample(Alphanumeric))
            .take(64)
            .collect::<Vec<_>>();

        // These unwraps will never panic because pre-conditions are always verified
        // (i.e. length and character set)
        String::from_utf8(value).unwrap().try_into().unwrap()
    }
    pub fn get_session_key(session_id: &str) -> String {
        format!("session::{}", session_id)
    }

    pub fn get_user_sessions_key(user_id: &str) -> String {
        format!("user::sessions::{}", user_id)
    }
    pub fn get_id_json(session: Session) -> Value {
        // it should use encrypted session_id(see actix-web example)
        json!({
            "session": session.id
        })
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
