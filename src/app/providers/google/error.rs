use thiserror::Error;

#[derive(Debug, Error)]
pub enum GoogleProviderError {
    #[error("OAuth2 certificates response has no expires header")]
    OAuth2CertificatesResponse,

    #[error("Revoke request error")]
    RevokeRequestError,
}
