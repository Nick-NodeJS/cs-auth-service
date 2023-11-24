use redis::RedisError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionServiceError {
    #[error("Redis error")]
    RedisError,
}

impl From<RedisError> for SessionServiceError {
    fn from(err: RedisError) -> Self {
        log::debug!("Redis error: {:?}", err);
        return SessionServiceError::RedisError;
    }
}
