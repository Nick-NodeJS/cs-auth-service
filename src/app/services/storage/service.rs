use bson::Document;
use mongodb::bson::{self, doc};
use mongodb::options::ClientOptions;
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

    pub async fn get_user(&self, query: Document) -> Result<Option<User>, StorageServiceError> {
        // TODO: implement caching on this level

        let mut raw_user = self.get_users_collection().find(query, None).await?;
        if raw_user.advance().await? {
            let user = bson::from_slice(raw_user.current().as_bytes())?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    pub async fn update_user(
        &self,
        filter: Document,
        data_to_update: Document,
    ) -> Result<(), StorageServiceError> {
        // TODO: implement caching on this level
        self.get_users_collection()
            .find_one_and_update(filter, doc! { "$set": data_to_update }, None)
            .await?;
        Ok(())
    }

    pub async fn insert_user(&self, user: User) -> Result<(), StorageServiceError> {
        // TODO: check if it returns _id which is the same as user._id
        self.get_users_collection().insert_one(user, None).await?;
        Ok(())
    }

    fn get_users_collection(&self) -> Collection<User> {
        self.database
            .collection::<User>(&self.config.user_collection)
    }
}
