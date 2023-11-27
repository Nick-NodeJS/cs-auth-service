use thiserror::Error;

use crate::app::{
    repositories::user::error::UserRepositoryError,
    services::{
        cache::error::CacheServiceError, session::error::SessionServiceError,
        storage::error::StorageServiceError,
    },
};

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("UserRepository error")]
    UserRepositoryError,

    #[error("Wrong profile error")]
    WrongProfileError,

    #[error("SessionService error")]
    SessionServiceError,

    #[error("CacheService error")]
    CacheServiceError,

    #[error("StorageService error")]
    StorageServiceError,
}

impl From<UserRepositoryError> for UserServiceError {
    fn from(err: UserRepositoryError) -> Self {
        log::debug!("UserRepositoryError: {:?}", err);
        return UserServiceError::UserRepositoryError;
    }
}

impl From<SessionServiceError> for UserServiceError {
    fn from(err: SessionServiceError) -> Self {
        log::debug!("SessionServiceError: {:?}", err);
        return UserServiceError::SessionServiceError;
    }
}

impl From<CacheServiceError> for UserServiceError {
    fn from(err: CacheServiceError) -> Self {
        log::debug!("CacheServiceError: {:?}", err);
        return UserServiceError::CacheServiceError;
    }
}

impl From<StorageServiceError> for UserServiceError {
    fn from(err: StorageServiceError) -> Self {
        log::debug!("StorageServiceError: {:?}", err);
        return UserServiceError::StorageServiceError;
    }
}
