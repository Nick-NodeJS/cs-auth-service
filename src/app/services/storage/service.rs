use mongodb::options::ClientOptions;
use mongodb::{Client, Collection, Database};

use crate::config::mongodb_config::MongoDBConfig;

use super::error::StorageServiceError;

pub trait CollectionType {}

#[derive(Debug)]
pub struct StorageService {
    pub config: MongoDBConfig,
    database: Database,
}

impl StorageService {
    pub async fn new() -> Result<Self, StorageServiceError> {
        let config = MongoDBConfig::new();
        log::info!(
            "\nMongoDB database {} connecting... \n connection URI: {}\n",
            &config.database,
            config.get_connection_uri()
        );
        let client_options = ClientOptions::parse(config.get_connection_uri()).await?;
        let client = Client::with_options(client_options)?;
        let database = client.database(&config.database);
        log::info!("\nMongoDB database {} is connected.\n", &config.database);
        Ok(StorageService { config, database })
    }

    pub fn get_collection<T: CollectionType>(&self, collection_name: &str) -> Collection<T> {
        self.database.collection::<T>(collection_name)
    }
}
