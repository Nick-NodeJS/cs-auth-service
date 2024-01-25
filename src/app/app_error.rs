use std::sync::PoisonError;

use actix_web::http;
use actix_web_thiserror_derive::ResponseError;
use log::error;
use thiserror::Error;

use super::{
    providers::error::ProviderError,
    services::{
        cache::error::CacheServiceError, storage::error::StorageServiceError,
        user::error::UserServiceError,
    },
};

#[derive(Debug, Error, ResponseError)]
pub enum AppError {
    #[response(reason = "Internal service error")]
    #[error("Lock Mutex error")]
    LockError,

    #[response(reason = "Internal service error")]
    #[error("Provider Error: {0}")]
    ProviderError(#[from] ProviderError),

    #[response(reason = "Internal service error")]
    #[error("UserService error: {0}")]
    UserServiceError(#[from] UserServiceError),

    #[response(reason = "Internal service error")]
    #[error("StorageService error: {0}")]
    StorageServiceError(#[from] StorageServiceError),

    #[response(reason = "Internal service error")]
    #[error("CacheService error: {0}")]
    CacheServiceError(#[from] CacheServiceError),
}

impl<T> From<PoisonError<T>> for AppError {
    fn from(err: PoisonError<T>) -> Self {
        log::debug!("Lock Mutex error: {}", err);
        return AppError::LockError;
    }
}
