use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserServiceError {
  #[error("Wrong profile error")]
  WrongProfileError,
}