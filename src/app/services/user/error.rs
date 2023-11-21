use thiserror::Error;

use crate::app::services::storage::error::StorageServiceError;

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("Wrong profile error")]
    WrongProfileError,

    #[error("StorageService error")]
    StorageServiceError,
}

impl From<StorageServiceError> for UserServiceError {
    fn from(err: StorageServiceError) -> Self {
        log::debug!("StorageServiceError: {:?}", err);
        return UserServiceError::StorageServiceError;
    }
}
