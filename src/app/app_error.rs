use std::sync::PoisonError;

use actix_web_thiserror_derive::ResponseError;
use oauth2::RequestTokenError;
use redis::RedisError;
use thiserror::Error;
use log::error;
use actix_web::http;

use super::services::{google::error::GoogleServiceError, user::error::UserServiceError};
// use jsonwebtoken::errors::Error as JWTError;

#[derive(Debug, Error, ResponseError)]
pub enum AppError {
  #[response(reason = "Internal service error")]
  #[error("Lock Mutex error")]
  LockError,

  #[response(reason = "Internal service error")]
  #[error("Redis error")]
  RedisError,

  // #[response(reason = "Internal service error")]
  // #[error("Pool error")]
  // PoolError,

  #[response(reason = "Bad Request")]
  #[error("Invalid query string")]
  QueryStringError,
  
  #[response(reason = "Bad Request")]
  #[error("Invalid code paramater")]
  CodeParamError,

  #[response(reason = "Bad Request")]
  #[error("No callback state in cache")]
  CallbackStateCacheError,

  #[response(reason = "Bad Request")]
  #[error("No refresh token in response")]
  NoRefreshTokenResponseError,

  #[response(reason = "Bad Request")]
  #[error("OAuth2 request token error")]
  OAuth2RequestTokenError,

  #[response(reason = "Internal service error")]
  #[error("No decoding key on Google Service")]
  NoDecodingKeyError,

  // #[response(reason = "Internal service error")]
  // #[error("JWT decoding error")]
  // JWTDecodingError,

  #[response(reason = "Internal service error")]
  #[error("GoogleService error")]
  GoogleServiceError,

  #[response(reason = "Internal service error")]
  #[error("UserService error")]
  UserServiceError,
}

impl<T> From<PoisonError<T>> for AppError {
  fn from(err: PoisonError<T>) -> Self {
    log::debug!("Lock Mutex error: {}", err);
    return AppError::LockError
  }
}

impl From<RedisError> for AppError {
  fn from(err: RedisError) -> Self {
    log::debug!("Redis error: {:?}", err);
    return AppError::RedisError
  }
}

impl From<UserServiceError> for AppError {
  fn from(err: UserServiceError) -> Self {
    log::debug!("UserServiceError: {:?}", err);
    return AppError::UserServiceError
  }
}

impl<T, P> From<RequestTokenError<T, P>> for AppError
where
  T: std::error::Error,
  P: oauth2::ErrorResponse,
{
  fn from(err: RequestTokenError<T, P>) -> Self {
    log::debug!("OAuth2 request token error: {:?}", err);
    return AppError::OAuth2RequestTokenError
  }
}

impl From<GoogleServiceError> for AppError {
  fn from(err: GoogleServiceError) -> Self {
    log::debug!("GoogleServiceError: {:?}", err);
    return AppError::GoogleServiceError
  }
}