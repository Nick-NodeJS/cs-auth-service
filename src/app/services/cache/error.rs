use redis::RedisError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheServiceError {
    #[error("Redis error")]
    RedisError,

    #[error("SerdeJson error")]
    SerdeJsonError,
}

impl From<RedisError> for CacheServiceError {
    fn from(err: RedisError) -> Self {
        log::debug!("RedisError: {:?}", err);
        return CacheServiceError::RedisError;
    }
}

impl From<serde_json::Error> for CacheServiceError {
    fn from(err: serde_json::Error) -> Self {
        log::debug!("serde_json::Error: {:?}", err);
        return CacheServiceError::SerdeJsonError;
    }
}
