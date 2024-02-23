use std::string::FromUtf8Error;

use actix_web::http::header::InvalidHeaderValue;
use awc::error::{JsonPayloadError, SendRequestError};
use jsonwebtoken::errors::Error as JWTError;
use oauth2::RequestTokenError;
use thiserror::Error;

use crate::app::services::cache::error::CacheServiceError;

use super::{
    cyber_sherlock::error::CyberSherlockAuthProviderError, facebook::error::FacebookProviderError,
    google::error::GoogleProviderError,
};

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("{0}")]
    CyberSherlockAuthProviderError(#[from] CyberSherlockAuthProviderError),

    #[error("{0}")]
    FacebookProviderError(#[from] FacebookProviderError),

    #[error("{0}")]
    GoogleProviderError(#[from] GoogleProviderError),

    #[error("{0}")]
    CacheServiceError(#[from] CacheServiceError),

    #[error("JWT decoding error")]
    JWTDecodingError,

    #[error("Send request error")]
    SendRequestError,

    #[error("Json payload error")]
    JsonPayloadError,

    #[error("Error")]
    Error,

    #[error("No callback state in cache error")]
    CallbackStateCacheError,

    #[error("Token data response error")]
    TokenDataResponseError,

    #[error("OAuth2 request token error")]
    OAuth2RequestTokenError,

    #[error("Invalid query string error")]
    QueryStringError,

    #[error("Invalid code paramater error")]
    CodeParamError,

    #[error("Invalid state paramater error")]
    StateParamError,

    #[error("Error to parse Url string")]
    UrlParseError,

    #[error("Serde json error")]
    SerdeJsonError,

    #[error("FromUtf8Error")]
    FromUtf8Error,

    #[error("Wrong token, unable to get header error")]
    BadTokenStructure,

    #[error("Base64 decode error")]
    Base64DecodeError,

    #[error("HeaderToStr error")]
    HeaderToStrError,

    #[error("chrono::Parse error")]
    ChronoParseError,

    #[error("InvalidHeaderValue error")]
    InvalidHeaderValue,
}

impl From<JWTError> for ProviderError {
    fn from(err: JWTError) -> Self {
        log::debug!("JWTDecodingError: {:?}", err);
        return ProviderError::JWTDecodingError;
    }
}

impl From<SendRequestError> for ProviderError {
    fn from(err: SendRequestError) -> Self {
        log::debug!("SendRequestError: {:?}", err);
        return ProviderError::SendRequestError;
    }
}

impl From<JsonPayloadError> for ProviderError {
    fn from(err: JsonPayloadError) -> Self {
        log::debug!("JsonPayloadError: {:?}", err);
        return ProviderError::JsonPayloadError;
    }
}

impl From<String> for ProviderError {
    fn from(err: String) -> Self {
        log::debug!("Error: {}", err);
        return ProviderError::Error;
    }
}

impl<T, P> From<RequestTokenError<T, P>> for ProviderError
where
    T: std::error::Error,
    P: oauth2::ErrorResponse,
{
    fn from(err: RequestTokenError<T, P>) -> Self {
        log::debug!("OAuth2 request token error: {:?}", err);
        return ProviderError::OAuth2RequestTokenError;
    }
}

impl From<oauth2::url::ParseError> for ProviderError {
    fn from(err: oauth2::url::ParseError) -> Self {
        log::debug!("oauth2::url::ParseError: {}", err);
        return ProviderError::UrlParseError;
    }
}

impl From<serde_json::Error> for ProviderError {
    fn from(err: serde_json::Error) -> Self {
        log::debug!("serde_json::Error: {}", err);
        return ProviderError::SerdeJsonError;
    }
}

impl From<FromUtf8Error> for ProviderError {
    fn from(err: FromUtf8Error) -> Self {
        log::debug!("FromUtf8Error: {}", err);
        return ProviderError::FromUtf8Error;
    }
}

impl From<base64::DecodeError> for ProviderError {
    fn from(err: base64::DecodeError) -> Self {
        log::debug!("base64::DecodeError: {}", err);
        return ProviderError::Base64DecodeError;
    }
}

impl From<actix_web::http::header::ToStrError> for ProviderError {
    fn from(err: actix_web::http::header::ToStrError) -> Self {
        log::debug!("actix_web::http::header::ToStrError: {}", err);
        return ProviderError::HeaderToStrError;
    }
}

impl From<chrono::ParseError> for ProviderError {
    fn from(err: chrono::ParseError) -> Self {
        log::debug!("chrono::ParseError: {}", err);
        return ProviderError::ChronoParseError;
    }
}

impl From<InvalidHeaderValue> for ProviderError {
    fn from(err: InvalidHeaderValue) -> Self {
        log::debug!("InvalidHeaderValue: {}", err);
        return ProviderError::InvalidHeaderValue;
    }
}
