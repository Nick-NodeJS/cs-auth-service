use std::sync::PoisonError;

use actix_web_thiserror_derive::ResponseError;
use redis::RedisError;
use thiserror::Error;
use log::error;
use actix_web::http;

#[derive(Debug, Error, ResponseError)]
pub enum AppError {
  #[response(reason = "Internal service error")]
  #[error("Lock Mutex error")]
  LockError,

  #[response(reason = "Internal service error")]
  #[error("Redis error")]
  RedisError,
}

impl<T> From<PoisonError<T>> for AppError {
  fn from(err: PoisonError<T>) -> Self {
    log::debug!("Lock Mutex error: {}", err);
    return AppError::LockError
  }
}

impl From<RedisError> for AppError {
  fn from(err: RedisError) -> Self {
    log::debug!("Redis error: {}", err);
    return AppError::RedisError
  }
}