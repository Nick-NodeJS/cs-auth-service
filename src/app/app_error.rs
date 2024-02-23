use std::sync::PoisonError;

use actix_web::{error, http::header::ContentType, Error, HttpRequest, HttpResponse};
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
    ValidationError(#[from] ValidationError),
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

//TODO: implement default error handler

pub fn error_handler(err: actix_web_validator::Error, _: &HttpRequest) -> Error {
    let bs = format!("{}", &err);
    // Validation error response as json
    let response = HttpResponse::BadRequest().json(error_as_json(bs.as_ref()));
    error::InternalError::from_response(err, response).into()
}
