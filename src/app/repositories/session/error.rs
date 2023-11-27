use redis::RedisError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionRepositoryError {
    #[error("Redis error")]
    RedisError,

    #[error("SerdeJson error")]
    SerdeJsonError,
}

impl From<RedisError> for SessionRepositoryError {
    fn from(err: RedisError) -> Self {
        log::debug!("Redis error: {:?}", err);
        return SessionRepositoryError::RedisError;
    }
}

impl From<serde_json::Error> for SessionRepositoryError {
    fn from(err: serde_json::Error) -> Self {
        log::debug!("serde_json::Error: {:?}", err);
        return SessionRepositoryError::SerdeJsonError;
    }
}
