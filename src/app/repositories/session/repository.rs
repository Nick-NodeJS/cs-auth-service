use std::collections::HashMap;

use bson::oid::ObjectId;

use crate::app::{models::session::Session, services::cache::service::CacheService};

use super::error::SessionRepositoryError;

pub struct SessionRepository {
    storage: CacheService,
}

impl SessionRepository {
    pub async fn new(storage: CacheService) -> Result<Self, SessionRepositoryError> {
        Ok(SessionRepository { storage })
    }

    pub async fn get_session(
        &self,
        session_key: &str,
    ) -> Result<Option<Session>, SessionRepositoryError> {
        Ok(None)
    }

    pub async fn get_sessions(
        &self,
        session_keys: Vec<&str>,
    ) -> Result<Vec<Session>, SessionRepositoryError> {
        Ok(vec![])
    }

    pub async fn set_session(
        &self,
        session_key: &str,
        session: Session,
    ) -> Result<(), SessionRepositoryError> {
        Ok(())
    }

    pub async fn set_sessions(
        &self,
        sessions: HashMap<&str, Session>,
    ) -> Result<(), SessionRepositoryError> {
        Ok(())
    }
}
