use std::collections::HashMap;

use serde_json::{Error as SerdeJsonError, Value};

use crate::app::{
    models::session::{session_as_key_value_vec, Session},
    services::cache::service::CacheService,
};

use super::error::SessionRepositoryError;

pub struct SessionRepository {
    storage: CacheService,
}

impl SessionRepository {
    pub fn new(storage: CacheService) -> Self {
        SessionRepository { storage }
    }

    pub async fn get_session(
        &mut self,
        session_key: &str,
    ) -> Result<Option<Session>, SessionRepositoryError> {
        let session_map = self.storage.hmget(session_key.clone())?;
        if session_map.len() == 0 {
            log::debug!("No session in cache, session_key: {}", session_key);
            return Ok(None);
        }
        let session = Session::from_hashmap(session_map);
        match session {
            Ok(session) => Ok(Some(session)),
            Err(err) => {
                log::error!(
                    "Error to deserialize Session: {}, session key:{}",
                    err,
                    session_key
                );
                Ok(None)
            }
        }
    }

    pub async fn set_session(
        &mut self,
        session_key: &str,
        session: Session,
    ) -> Result<(), SessionRepositoryError> {
        self.storage
            .hmset(session_key, &session_as_key_value_vec(session))?;
        Ok(())
    }
}
