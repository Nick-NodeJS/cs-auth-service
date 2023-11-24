use std::sync::PoisonError;

use actix_web::http;
use actix_web_thiserror_derive::ResponseError;
use log::error;
use thiserror::Error;

use super::services::{
    google::error::GoogleServiceError, storage::error::StorageServiceError,
    user::error::UserServiceError,
};

#[derive(Debug, Error, ResponseError)]
pub enum AppError {
    #[response(reason = "Internal service error")]
    #[error("Lock Mutex error")]
    LockError,

    #[response(reason = "Internal service error")]
    #[error("GoogleService error")]
    GoogleServiceError,

    #[response(reason = "Internal service error")]
    #[error("UserService error")]
    UserServiceError,

    #[response(reason = "Internal service error")]
    #[error("StorageService error")]
    StorageServiceError,
}

impl<T> From<PoisonError<T>> for AppError {
    fn from(err: PoisonError<T>) -> Self {
        log::debug!("Lock Mutex error: {}", err);
        return AppError::LockError;
    }
}

impl From<UserServiceError> for AppError {
    fn from(err: UserServiceError) -> Self {
        log::debug!("UserServiceError: {:?}", err);
        return AppError::UserServiceError;
    }
}

impl From<GoogleServiceError> for AppError {
    fn from(err: GoogleServiceError) -> Self {
        log::debug!("GoogleServiceError: {:?}", err);
        return AppError::GoogleServiceError;
    }
}

impl From<StorageServiceError> for AppError {
    fn from(err: StorageServiceError) -> Self {
        log::debug!("StorageServiceError: {:?}", err);
        return AppError::StorageServiceError;
    }
}
