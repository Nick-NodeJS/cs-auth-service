use mongodb::bson::ser::Error as MongoDBBsonError;
use redis::RedisError;
use thiserror::Error;

use crate::app::services::cache::error::CacheServiceError;

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("MongoDB error")]
    MongoDBError,

    #[error("MongoDBBsonError error")]
    MongoDBBsonError,

    #[error("MongoDB error")]
    MongoDBBsonDeError,

    #[error("Redis error")]
    RedisError,

    #[error("UpdateUser error")]
    UpdateUserError,

    #[error("CacheService error")]
    CacheServiceError,

    #[error("SerdeJson error")]
    SerdeJsonError,
}

impl From<RedisError> for UserRepositoryError {
    fn from(err: RedisError) -> Self {
        log::debug!("Redis error: {:?}", err);
        return UserRepositoryError::RedisError;
    }
}
impl From<MongoDBBsonError> for UserRepositoryError {
    fn from(err: MongoDBBsonError) -> Self {
        log::debug!("MongoDBBsonError: {}", err);
        return UserRepositoryError::MongoDBBsonError;
    }
}

impl From<mongodb::error::Error> for UserRepositoryError {
    fn from(err: mongodb::error::Error) -> Self {
        log::debug!("mongodb::error::Error: {}", err);
        return UserRepositoryError::MongoDBError;
    }
}

impl From<mongodb::bson::de::Error> for UserRepositoryError {
    fn from(err: mongodb::bson::de::Error) -> Self {
        log::debug!("mongodb::bson::de::Error: {}", err);
        return UserRepositoryError::MongoDBBsonDeError;
    }
}

impl From<CacheServiceError> for UserRepositoryError {
    fn from(err: CacheServiceError) -> Self {
        log::debug!("CacheServiceError: {}", err);
        return UserRepositoryError::CacheServiceError;
    }
}

impl From<serde_json::Error> for UserRepositoryError {
    fn from(err: serde_json::Error) -> Self {
        log::debug!("serde_json::Error: {}", err);
        return UserRepositoryError::SerdeJsonError;
    }
}
