use awc::error::{JsonPayloadError, SendRequestError};
use jsonwebtoken::errors::Error as JWTError;
use oauth2::RequestTokenError;
use redis::RedisError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GoogleServiceError {
    #[error("Redis error")]
    RedisError,

    #[error("JWT decoding error")]
    JWTDecodingError,

    #[error("Send request error")]
    SendRequestError,

    #[error("Json payload error")]
    JsonPayloadError,

    #[error("Error")]
    Error,

    #[error("No callback state in cache")]
    CallbackStateCacheError,

    #[error("No refresh token in response")]
    NoRefreshTokenResponseError,

    #[error("OAuth2 request token error")]
    OAuth2RequestTokenError,

    #[error("Invalid query string")]
    QueryStringError,

    #[error("Invalid code paramater")]
    CodeParamError,
}

impl From<RedisError> for GoogleServiceError {
    fn from(err: RedisError) -> Self {
        log::debug!("Redis error: {:?}", err);
        return GoogleServiceError::RedisError;
    }
}

impl From<JWTError> for GoogleServiceError {
    fn from(err: JWTError) -> Self {
        log::debug!("JWTDecodingError: {:?}", err);
        return GoogleServiceError::JWTDecodingError;
    }
}

impl From<SendRequestError> for GoogleServiceError {
    fn from(err: SendRequestError) -> Self {
        log::debug!("SendRequestError: {:?}", err);
        return GoogleServiceError::SendRequestError;
    }
}

impl From<JsonPayloadError> for GoogleServiceError {
    fn from(err: JsonPayloadError) -> Self {
        log::debug!("JsonPayloadError: {:?}", err);
        return GoogleServiceError::JsonPayloadError;
    }
}

impl From<String> for GoogleServiceError {
    fn from(err: String) -> Self {
        log::debug!("Error: {}", err);
        return GoogleServiceError::Error;
    }
}

impl<T, P> From<RequestTokenError<T, P>> for GoogleServiceError
where
    T: std::error::Error,
    P: oauth2::ErrorResponse,
{
    fn from(err: RequestTokenError<T, P>) -> Self {
        log::debug!("OAuth2 request token error: {:?}", err);
        return GoogleServiceError::OAuth2RequestTokenError;
    }
}
