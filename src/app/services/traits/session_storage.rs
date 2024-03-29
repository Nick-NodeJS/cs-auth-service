use crate::app::{models::session::Session, services::cache::error::CacheServiceError};

pub trait SessionStorage {
    fn load(&self, key: &str) -> Result<Option<Session>, CacheServiceError>;
    fn set(&self, session: &Session, ttl: u64) -> Result<(), CacheServiceError>;
}
