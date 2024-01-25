use thiserror::Error;

#[derive(Debug, Error)]
pub enum GoogleServiceError {
    #[error("OAuth2 certificates response has no expires header")]
    OAuth2CertificatesResponse,
}
