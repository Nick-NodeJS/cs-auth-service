use mongodb::bson::ser::Error as MongoDBBsonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageServiceError {
    #[error("MongoDB error")]
    MongoDBError,

    #[error("MongoDB error")]
    MongoDBBsonError,
}

impl From<MongoDBBsonError> for StorageServiceError {
    fn from(err: MongoDBBsonError) -> Self {
        log::debug!("MongoDBBsonError: {}", err);
        return StorageServiceError::MongoDBBsonError;
    }
}

impl From<mongodb::error::Error> for StorageServiceError {
    fn from(err: mongodb::error::Error) -> Self {
        log::debug!("mongodb::error::Error: {}", err);
        return StorageServiceError::MongoDBError;
    }
}
