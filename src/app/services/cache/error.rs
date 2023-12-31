use redis::RedisError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheServiceError {
    #[error("Redis error")]
    RedisError,
}

impl From<RedisError> for CacheServiceError {
    fn from(err: RedisError) -> Self {
        log::debug!("RedisError: {:?}", err);
        return CacheServiceError::RedisError;
    }
}
