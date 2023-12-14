use thiserror::Error;

use crate::app::repositories::session::error::SessionRepositoryError;

#[derive(Debug, Error)]
pub enum SessionServiceError {
    #[error("SessionRepository error")]
    SessionRepositoryError,

    #[error("SetCookieToResponse error")]
    SetCookieToResponseError,
}

impl From<SessionRepositoryError> for SessionServiceError {
    fn from(err: SessionRepositoryError) -> Self {
        log::debug!("SessionRepositoryError: {:?}", err);
        return SessionServiceError::SessionRepositoryError;
    }
}
