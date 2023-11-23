use redis::RedisError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionRepositoryError {
    #[error("Redis error")]
    RedisError,
}

impl From<RedisError> for SessionRepositoryError {
    fn from(err: RedisError) -> Self {
        log::debug!("Redis error: {:?}", err);
        return SessionRepositoryError::RedisError;
    }
}
