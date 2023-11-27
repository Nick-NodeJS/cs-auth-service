use bson::oid::ObjectId;
use chrono::Utc;

use crate::{
    app::{
        models::{common::AuthProviders, session::Session},
        repositories::session::repository::SessionRepository,
    },
    config::session_config::SessionConfig,
};

use super::error::SessionServiceError;

pub struct SessionService {
    config: SessionConfig,
    repository: SessionRepository,
}

impl SessionService {
    pub fn new(repository: SessionRepository) -> Self {
        let config = SessionConfig::new();
        SessionService { config, repository }
    }

    pub async fn get_session(
        &mut self,
        user_id: ObjectId,
        auth_provider: AuthProviders,
    ) -> Result<Option<Session>, SessionServiceError> {
        let session_key = Session::get_session_key(user_id, auth_provider);
        if let Some(session) = self.repository.get_session(&session_key).await? {
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    pub async fn update_session(
        &mut self,
        mut session: Session,
        token: Option<String>,
    ) -> Result<(), SessionServiceError> {
        let session_key = Session::get_session_key(session.user_id, session.auth_provider.clone());
        session.updated_at = Utc::now();
        if let Some(session_token) = token {
            session.token = session_token;
        }
        self.repository.set_session(&session_key, session).await?;
        Ok(())
    }

    // TODO: token in most cases has expiration time
    // - set session ttl as token expiration
    pub async fn set_session(
        &mut self,
        auth_provider: AuthProviders,
        user_id: ObjectId,
        token: String,
    ) -> Result<(), SessionServiceError> {
        let session = Session::new(auth_provider.clone(), user_id, token);
        let session_key = Session::get_session_key(user_id, auth_provider);
        self.repository.set_session(&session_key, session).await?;
        Ok(())
    }
}
