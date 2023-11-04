use awc::error::{ SendRequestError, JsonPayloadError };
use jsonwebtoken::errors::Error as JWTError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GoogleServiceError {
  #[error("JWT decoding error")]
  JWTDecodingError,

  #[error("Send request error")]
  SendRequestError,

  #[error("Json payload error")]
  JsonPayloadError
}

impl From<JWTError> for GoogleServiceError {
  fn from(err: JWTError) -> Self {
    log::debug!("JWTDecodingError: {:?}", err);
    return GoogleServiceError::JWTDecodingError
  }
}

impl From<SendRequestError> for GoogleServiceError {
  fn from(err: SendRequestError) -> Self {
    log::debug!("SendRequestError: {:?}", err);
    return GoogleServiceError::SendRequestError
  }
}

impl From<JsonPayloadError> for GoogleServiceError {
  fn from(err: JsonPayloadError) -> Self {
    log::debug!("JsonPayloadError: {:?}", err);
    return GoogleServiceError::JsonPayloadError
  }
}

