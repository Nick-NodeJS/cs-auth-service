use bson::oid::ObjectId;

use crate::app::{
    models::{common::AuthProviders, session::Session},
    services::cache::service::CacheService,
};

use super::error::SessionServiceError;

pub struct SessionService {
    cache_service: CacheService,
}

impl SessionService {
    pub async fn new(cache_service: CacheService) -> Result<Self, SessionServiceError> {
        Ok(SessionService { cache_service })
    }

    pub async fn get_session(&self, user_id: &str) -> Result<Option<Session>, SessionServiceError> {
        // TODO: implement caching on this level

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
