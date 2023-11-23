use bson::oid::ObjectId;

use crate::app::{
    models::{common::AuthProviders, session::Session},
    repositories::session::repository::SessionRepository,
    services::cache::service::CacheService,
};

use super::error::SessionServiceError;

pub struct SessionService {
    repository: SessionRepository,
}

impl SessionService {
    pub async fn new(repository: SessionRepository) -> Result<Self, SessionServiceError> {
        Ok(SessionService { repository })
    }

    pub async fn get_session(&self, user_id: &str) -> Result<Option<Session>, SessionServiceError> {
        Ok(None)
    }

    pub async fn update_session(
        &self,
        auth_provider: AuthProviders,
        user_id: ObjectId,
        refresh_token: String,
    ) -> Result<(), SessionServiceError> {
        Ok(())
    }

    pub async fn insert_session(
        &self,
        auth_provider: AuthProviders,
        user_id: ObjectId,
        refresh_token: String,
    ) -> Result<(), SessionServiceError> {
        Ok(())
    }
}
