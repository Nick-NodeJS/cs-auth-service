use bson::Document;
use mongodb::bson::{self, doc};
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection, Database};

use crate::app::models::user::User;
use crate::app::services::cache::service::CacheService;
use crate::app::services::storage::service::StorageService;
use crate::config::mongodb_config::MongoDBConfig;

use super::error::UserRepositoryError;

pub struct UserRepository {
    collection: String,
    cache: CacheService,
    storage: StorageService,
}

impl UserRepository {
    pub fn new(collection: String, cache: CacheService, storage: StorageService) -> Self {
        UserRepository {
            collection,
            cache,
            storage,
        }
    }

    pub async fn get_user(&self, query: Document) -> Result<Option<User>, UserRepositoryError> {
        // TODO: implement caching on this level

        let mut raw_user = self.get_collection().find(query, None).await?;
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
    ) -> Result<(), UserRepositoryError> {
        // TODO: implement caching on this level
        self.get_collection()
            .find_one_and_update(filter, doc! { "$set": data_to_update }, None)
            .await?;
        Ok(())
    }

    pub async fn insert_user(&self, user: User) -> Result<(), UserRepositoryError> {
        // TODO: check if it returns _id which is the same as user._id
        self.get_collection().insert_one(user, None).await?;
        Ok(())
    }

    fn get_collection(&self) -> Collection<User> {
        self.storage.get_collection::<User>(&self.collection)
    }
}
