use std::string::FromUtf8Error;

use actix_web::http::header::InvalidHeaderValue;
use awc::error::{JsonPayloadError, SendRequestError};
use jsonwebtoken::errors::Error as JWTError;
use oauth2::RequestTokenError;
use thiserror::Error;

use crate::app::services::cache::error::CacheServiceError;

#[derive(Debug, Error)]
pub enum GoogleServiceError {
    #[error("CacheService error")]
    CacheServiceError,

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

    #[error("Token data response error")]
    TokenDataResponseError,

    #[error("OAuth2 request token error")]
    OAuth2RequestTokenError,

    #[error("Invalid query string")]
    QueryStringError,

    #[error("Invalid code paramater")]
    CodeParamError,

    #[error("Error to parse Url string")]
    UrlParseError,

    #[error("Serde json error")]
    SerdeJsonError,

    #[error("FromUtf8Error")]
    FromUtf8Error,

    #[error("Wrong token, unable to get header")]
    BadTokenStructure,

    #[error("Base64 decode error")]
    Base64DecodeError,

    #[error("OAuth2 certificates response has no expires header")]
    OAuth2CertificatesResponse,

    #[error("Revoke request error")]
    RevokeRequestError,

    #[error("HeaderToStr error")]
    HeaderToStrError,

    #[error("chrono::Parse error")]
    ChronoParseError,

    #[error("InvalidHeaderValue error")]
    InvalidHeaderValue,
}

impl From<CacheServiceError> for GoogleServiceError {
    fn from(err: CacheServiceError) -> Self {
        log::debug!("CacheService error: {:?}", err);
        return GoogleServiceError::CacheServiceError;
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

impl From<oauth2::url::ParseError> for GoogleServiceError {
    fn from(err: oauth2::url::ParseError) -> Self {
        log::debug!("oauth2::url::ParseError: {}", err);
        return GoogleServiceError::UrlParseError;
    }
}

impl From<serde_json::Error> for GoogleServiceError {
    fn from(err: serde_json::Error) -> Self {
        log::debug!("serde_json::Error: {}", err);
        return GoogleServiceError::SerdeJsonError;
    }
}

impl From<FromUtf8Error> for GoogleServiceError {
    fn from(err: FromUtf8Error) -> Self {
        log::debug!("FromUtf8Error: {}", err);
        return GoogleServiceError::FromUtf8Error;
    }
}

impl From<base64::DecodeError> for GoogleServiceError {
    fn from(err: base64::DecodeError) -> Self {
        log::debug!("base64::DecodeError: {}", err);
        return GoogleServiceError::Base64DecodeError;
    }
}

impl From<actix_web::http::header::ToStrError> for GoogleServiceError {
    fn from(err: actix_web::http::header::ToStrError) -> Self {
        log::debug!("actix_web::http::header::ToStrError: {}", err);
        return GoogleServiceError::HeaderToStrError;
    }
}

impl From<chrono::ParseError> for GoogleServiceError {
    fn from(err: chrono::ParseError) -> Self {
        log::debug!("chrono::ParseError: {}", err);
        return GoogleServiceError::ChronoParseError;
    }
}

impl From<InvalidHeaderValue> for GoogleServiceError {
    fn from(err: InvalidHeaderValue) -> Self {
        log::debug!("InvalidHeaderValue: {}", err);
        return GoogleServiceError::InvalidHeaderValue;
    }
}
