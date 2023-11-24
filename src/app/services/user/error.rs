use thiserror::Error;

use crate::app::repositories::user::error::UserRepositoryError;

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("Wrong profile error")]
    WrongProfileError,

    #[error("StorageService error")]
    StorageServiceError,
}

impl From<UserRepositoryError> for UserServiceError {
    fn from(err: UserRepositoryError) -> Self {
        log::debug!("UserRepositoryError: {:?}", err);
        return UserServiceError::StorageServiceError;
    }
}
