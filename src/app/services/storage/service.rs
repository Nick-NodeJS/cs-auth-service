use mongodb::bson::{to_bson, to_document};
use mongodb::options::{ClientOptions, CreateCollectionOptions};
use mongodb::{Client, Collection, Database};

use crate::app::models::user::User;
use crate::config::mongodb_config::MongoDBConfig;

use super::error::StorageServiceError;

pub struct StorageService {
    config: MongoDBConfig,
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
    pub async fn get_user_by_google_id(
        &self,
        google_id: &str,
    ) -> Result<Option<User>, StorageServiceError> {
        Ok(None)
    }

    pub async fn insert_user(&self, user: User) -> Result<(), StorageServiceError> {
        self.get_users_collection().insert_one(user, None).await?;
        Ok(())
    }

    fn get_users_collection(&self) -> Collection<User> {
        self.database
            .collection::<User>(&self.config.user_collection)
    }
}
