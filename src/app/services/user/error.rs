use thiserror::Error;

use crate::app::{
    repositories::user::error::UserRepositoryError, services::session::error::SessionServiceError,
};

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("UserRepository error")]
    UserRepositoryError,

    #[error("Wrong profile error")]
    WrongProfileError,

    #[error("StorageService error")]
    StorageServiceError,

    #[error("SessionService error")]
    SessionServiceError,
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
