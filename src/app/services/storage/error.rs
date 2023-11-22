use mongodb::bson::ser::Error as MongoDBBsonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageServiceError {
    #[error("MongoDB error")]
    MongoDBError,

    #[error("MongoDBBsonError error")]
    MongoDBBsonError,

    #[error("MongoDB error")]
    MongoDBBsonDeError,
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

impl From<mongodb::bson::de::Error> for StorageServiceError {
    fn from(err: mongodb::bson::de::Error) -> Self {
        log::debug!("mongodb::bson::de::Error: {}", err);
        return StorageServiceError::MongoDBBsonDeError;
    }
}
