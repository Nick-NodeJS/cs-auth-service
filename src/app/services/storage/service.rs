use dotenv::dotenv;
use serde::Deserialize;

use crate::app::services::storage::mongodb::MongoDBStorageService;
use crate::app::services::user::user::User;

use super::error::StorageServiceError;
use super::postgres::PostgresStorageService;

const STORAGE: &str = "STORAGE";

pub trait StorageService {
    fn set_user(&self, user: User) -> Result<(), StorageServiceError>;
}

pub fn get_storage_service() -> Box<dyn StorageService> {
    match dotenv::var(STORAGE) {
        Ok(storage) => match storage.to_lowercase().as_ref() {
            "mongodb" => Box::new(MongoDBStorageService::new()),
            "postgres" => Box::new(PostgresStorageService::new()),
            _ => {
                log::debug!("Incorrect STORAGE env, use default MongoDB storage");
                Box::new(MongoDBStorageService::new())
            }
        },
        Err(err) => {
            log::debug!("No STORAGE env, use default MongoDB storage");
            Box::new(MongoDBStorageService::new())
        }
    }
}
