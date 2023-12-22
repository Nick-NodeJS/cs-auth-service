use crate::app::{models::session::Session, services::cache::service::CacheService};

use super::error::SessionRepositoryError;

#[derive(Debug)]
pub struct SessionRepository {
    storage: CacheService,
}

impl SessionRepository {
    pub fn new(storage: CacheService) -> Self {
        SessionRepository { storage }
    }

    pub async fn get_sessions(
        &mut self,
        user_sessions_key: &str,
    ) -> Result<Vec<Session>, SessionRepositoryError> {
        let session_keys = self.storage.hgetall(user_sessions_key)?;
        if session_keys.len() == 0 {
            return Ok(vec![]);
        }
        let keys = session_keys.keys().map(|key| key.to_owned()).collect();
        let sessions = self.storage.mget::<Session>(keys)?;
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
        session_ttl: i64,
    ) -> Result<(), SessionRepositoryError> {
        // TODO: implement user sessions in cache array updating in parallel(in one step) with session setting
        self.storage.set_value_with_ttl::<String>(
            session_key,
            CacheService::struct_to_cache_string(&session)?,
            session_ttl as usize,
        )?;
        self.storage.hset(
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
        dbg!(user_sessions_key, &session_keys);
        // TODO: implement user sessions in cache array updating in parallel(in one step) with session deleting
        self.storage.delete_values(session_keys.clone())?;
        self.storage
            .delete_hset_values(user_sessions_key, session_keys)?;
        Ok(())
    }
}
