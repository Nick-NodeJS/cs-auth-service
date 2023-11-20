use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageServiceError {
    #[error("MongoDB error")]
    MongoDBError,
}
