use crate::app::services::user::user::User;

use super::{error::StorageServiceError, service::StorageService};

pub struct PostgresStorageService {}

impl PostgresStorageService {
    pub fn new() -> Self {
        PostgresStorageService {}
    }
}

impl StorageService for PostgresStorageService {
    fn set_user(&self, user: User) -> Result<(), StorageServiceError> {
        Ok(())
    }
}
