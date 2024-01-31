use thiserror::Error;

#[derive(Debug, Error)]
pub enum FacebookProviderError {
    #[error("Delete permissions request error")]
    DeletePermissionsRequestError,
}
