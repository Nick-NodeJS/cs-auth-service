use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageServiceError {
    #[error("MongoDB error")]
    MongoDBError,
}

impl From<mongodb::error::Error> for StorageServiceError {
    fn from(err: mongodb::error::Error) -> Self {
        log::debug!("mongodb::error::Error: {}", err);
        return StorageServiceError::MongoDBError;
    }
}
