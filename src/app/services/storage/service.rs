use mongodb::options::ClientOptions;
use mongodb::{Client, Database};

use crate::app::services::user::user::User;
use crate::config::mongodb_config::MongoDBConfig;

use super::error::StorageServiceError;

pub struct StorageService {
    config: MongoDBConfig,
    database: Database,
}

impl StorageService {
    pub async fn new() -> Result<Self, StorageServiceError> {
        let config = MongoDBConfig::new();
        println!(
            "MongoDB database {} connecting...{}",
            &config.database,
            config.get_connection_uri()
        );
        let client_options = ClientOptions::parse(config.get_connection_uri()).await?;
        let client = Client::with_options(client_options)?;
        let database = client.database(&config.database);
        println!("MongoDB database {} is connected", &config.database);
        Ok(StorageService { config, database })
    }
    fn set_user(&self, user: User) -> Result<(), StorageServiceError> {
        Ok(())
    }
}
