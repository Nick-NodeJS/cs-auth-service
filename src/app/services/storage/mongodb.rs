use crate::app::services::user::user::User;

use super::{error::StorageServiceError, service::StorageService};

pub struct MongoDBStorageService;

impl MongoDBStorageService {
    pub fn new() -> Self {
        MongoDBStorageService {}
    }
}

impl StorageService for MongoDBStorageService {
    fn set_user(&self, user: User) -> Result<(), StorageServiceError> {
        Ok(())
    }
}
