use crate::app::{models::session::Session, services::cache::service::RedisCacheService};

use super::error::SessionRepositoryError;

#[derive(Debug)]
pub struct SessionRepository {
    storage: RedisCacheService,
}

impl SessionRepository {
    pub fn new(storage: RedisCacheService) -> Self {
        SessionRepository { storage }
    }

    pub async fn get_sessions(
        &mut self,
        user_sessions_key: &str,
    ) -> Result<Vec<Session>, SessionRepositoryError> {
        let session_keys = self.storage.get_all_set_values(user_sessions_key)?;
        if session_keys.len() == 0 {
            return Ok(vec![]);
        }
        let keys = session_keys.keys().map(|key| key.to_owned()).collect();
        let sessions = self.storage.get_values::<Session>(keys)?;
        // in case the session key is still in user session set but session is not in cach
        // it returns Nil(None) and we filter the array
        let sessions_without_none = sessions.iter().filter_map(|s| s.to_owned()).collect();
        log::debug!("User sessions: {:?}", sessions_without_none);
        Ok(sessions_without_none)
    }

    pub async fn set_session(
        &mut self,
        session_key: &str,
        session: &Session,
        session_ttl: u64,
    ) -> Result<(), SessionRepositoryError> {
        // TODO: implement user sessions in cache array updating in parallel(in one step) with session setting
        self.storage
            .set_value_with_ttl::<Session>(session_key, session.clone(), session_ttl)?;
        self.storage.set(
            &Session::get_user_sessions_key(&session.user_id.to_string()),
            (
                Session::get_session_key(&session.id).as_ref(),
                session.auth_provider.to_string(),
            ),
        )?;
        Ok(())
    }

    pub async fn remove_sessions(
        &mut self,
        user_sessions_key: &str,
        session_keys: Vec<String>,
    ) -> Result<(), SessionRepositoryError> {
        // TODO: implement user sessions in cache array updating in parallel(in one step) with session deleting
        if session_keys.len() > 0 {
            self.storage.delete_values(session_keys.clone())?;
            self.storage
                .delete_set_values(user_sessions_key, session_keys)?;
        }
        Ok(())
    }

    pub async fn remove_sessions_by_keys(
        &mut self,
        session_keys: Vec<String>,
    ) -> Result<(), SessionRepositoryError> {
        // TODO: implement user sessions in cache array updating in parallel(in one step) with session deleting
        self.storage.delete_values(session_keys)?;
        Ok(())
    }
}
