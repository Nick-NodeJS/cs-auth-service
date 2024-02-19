use std::sync::PoisonError;

use actix_web::{error, http::header::ContentType, HttpResponse};
// use actix_web_thiserror_derive::ResponseError;
use log::error;
use thiserror::Error;
use validator::ValidationError;

use super::{
    providers::error::ProviderError,
    services::{
        cache::error::CacheServiceError, common::error_as_json,
        storage::error::StorageServiceError, user::error::UserServiceError,
    },
};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Lock Mutex error")]
    LockError,

    #[error("{0}")]
    ProviderError(#[from] ProviderError),

    #[error("{0}")]
    UserServiceError(#[from] UserServiceError),

    #[error("{0}")]
    StorageServiceError(#[from] StorageServiceError),

    #[error("{0}")]
    CacheServiceError(#[from] CacheServiceError),

    #[error("{0}")]
    ValidationError(String),
}

impl From<ValidationError> for AppError {
    fn from(err: ValidationError) -> Self {
        log::debug!("Validation error: {}", &err);
        return AppError::ValidationError(err.to_string());
    }
}

impl<T> From<PoisonError<T>> for AppError {
    fn from(err: PoisonError<T>) -> Self {
        log::debug!("Lock Mutex error: {}", err);
        return AppError::LockError;
    }
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(error_as_json(self.to_string().as_ref()))
    }
}
