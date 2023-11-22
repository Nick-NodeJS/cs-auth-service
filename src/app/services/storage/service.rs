use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{self, doc, to_bson, to_document, Bson, DateTime};
use mongodb::options::{ClientOptions, CreateCollectionOptions};
use mongodb::{Client, Collection, Database};

use crate::app::models::user::{GoogleProfile, User, UserActiveProfile};
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
        // TODO: implement caching on this level
        let query = doc! {
            "google.user_id": Bson::String(google_id.to_string()),
        };
        let mut raw_user = self.get_users_collection().find(query, None).await?;
        if raw_user.advance().await? {
            let user = bson::from_slice(raw_user.current().as_bytes())?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    pub async fn update_user_by_id_with_google_profile(
        &self,
        user_id: ObjectId,
        google_profile: GoogleProfile,
    ) -> Result<(), StorageServiceError> {
        // TODO: implement caching on this level
        let query = doc! {
            "_id": Bson::ObjectId(user_id),
        };
        let user_data = doc! {
            "google.name": google_profile.name,
            "google.email": google_profile.email,
            "google.email_verified": google_profile.email_verified,
            "google.picture": google_profile.picture,
            "updated_at": Utc::now(),
        };
        self.get_users_collection()
            .find_one_and_update(query, doc! { "$set": user_data }, None)
            .await?;
        Ok(())
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
