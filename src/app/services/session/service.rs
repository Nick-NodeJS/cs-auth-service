use bson::oid::ObjectId;

use crate::{
    app::{
        models::{
            common::AuthProviders, session::Session, session_tokens::SessionTokens, user::User,
        },
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

    pub async fn get_sessions(
        &mut self,
        user_id: ObjectId,
        auth_provider: AuthProviders,
    ) -> Result<Vec<Session>, SessionServiceError> {
        let sessions_key = Session::get_user_sessions_key(&user_id.to_string());
        let sessions = self.repository.get_sessions(sessions_key.as_ref()).await?;
        // TODO: check how to compare auth_provider
        let user_sessions = sessions.into_iter().filter(|s| true).collect();
        Ok(user_sessions)
    }

    // TODO: token in most cases has expiration time
    // - set session ttl as token expiration
    pub async fn set_new_session(
        &mut self,
        auth_provider: AuthProviders,
        user: User,
        tokens: SessionTokens,
    ) -> Result<Session, SessionServiceError> {
        let session = Session::new(auth_provider.clone(), tokens, user.id);
        let session_key = Session::get_session_key(&session);
        self.repository
            .set_session(&session_key, &session, self.config.session_ttl_sec)
            .await?;
        Ok(session)
    }
}
