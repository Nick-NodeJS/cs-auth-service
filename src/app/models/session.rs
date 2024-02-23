use actix_utils::future::{ready, Ready};
use actix_web::{dev::Payload, error::Error, FromRequest, HttpMessage, HttpRequest};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use redis::{
    ErrorKind, FromRedisValue, RedisError, RedisResult, RedisWrite, ToRedisArgs,
    Value as RedisValue,
};
use serde::{Deserialize, Serialize};

use std::convert::TryInto;

use rand::{distributions::Alphanumeric, rngs::OsRng, Rng as _};

use super::{
    common::AuthProviders, session_metadata::SessionMetadata, session_tokens::SessionTokens,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewSessionData {
    pub anonimous: bool,
    pub auth_provider: AuthProviders,
    pub user_id: ObjectId,
    pub tokens: SessionTokens,
    pub session_metadata: SessionMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub anonimous: bool,
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
            anonimous: session_data.anonimous,
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

    pub fn is_anonymous(&self) -> bool {
        self.anonimous
    }

    pub fn generate_session_id() -> String {
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

    pub fn get_anonymous_session(req: Option<&HttpRequest>) -> Session {
        let mut session_metadata = SessionMetadata::new();
        if let Some(request) = req {
            session_metadata.set_metadata_from_request(request);
        }
        Session::new(NewSessionData {
            anonimous: true,
            auth_provider: AuthProviders::CyberSherlock,
            user_id: ObjectId::new(),
            tokens: SessionTokens::empty_tokens(),
            session_metadata,
        })
    }

    pub fn get_session_from_http_request(req: &HttpRequest) -> Session {
        if let Some(session) = req.extensions_mut().get::<Session>() {
            session.to_owned()
        } else {
            Session::get_anonymous_session(None)
        }
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

impl FromRequest for Session {
    type Error = Error;
    type Future = Ready<Result<Session, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(Ok(Session::get_session_from_http_request(&req)))
    }
}

impl ToRedisArgs for Session {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        // Serialize the Session to JSON and write it to the output
        let serialized = serde_json::to_vec(self).expect("Failed to serialize Session to JSON");
        out.write_arg(&serialized)
    }
}

// impl CacheValue for Session {}
