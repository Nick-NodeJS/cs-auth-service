use thiserror::Error;

#[derive(Debug, Error)]
pub enum CyberSherlockAuthProviderError {
    #[error("Argon2 Password Hash error")]
    Argon2PassHashError,

    #[error("Error: Bad Login Query Data, must have `email` or `mobile`")]
    BadLoginQueryData,
}

impl From<argon2::password_hash::Error> for CyberSherlockAuthProviderError {
    fn from(err: argon2::password_hash::Error) -> Self {
        log::debug!("SendRequestError: {:?}", err);
        return CyberSherlockAuthProviderError::Argon2PassHashError;
    }
}
