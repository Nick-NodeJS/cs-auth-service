use thiserror::Error;

use crate::app::services::cache::error::CacheServiceError;

#[derive(Debug, Error)]
pub enum SessionRepositoryError {
    #[error("CacheService error")]
    CacheServiceError,

    #[error("SerdeJson error")]
    SerdeJsonError,
}

impl From<CacheServiceError> for SessionRepositoryError {
    fn from(err: CacheServiceError) -> Self {
        log::debug!("CacheService error: {:?}", err);
        return SessionRepositoryError::CacheServiceError;
    }
}

impl From<serde_json::Error> for SessionRepositoryError {
    fn from(err: serde_json::Error) -> Self {
        log::debug!("serde_json::Error: {:?}", err);
        return SessionRepositoryError::SerdeJsonError;
    }
}
