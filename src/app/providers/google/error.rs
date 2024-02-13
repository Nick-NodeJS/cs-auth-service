use thiserror::Error;

#[derive(Debug, Error)]
pub enum GoogleProviderError {
    #[error("Error: OAuth2 certificates response has no expires header")]
    OAuth2CertificatesResponse,

    #[error("Error: Revoke request error")]
    RevokeRequestError,
}
