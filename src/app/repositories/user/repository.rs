use bson::oid::ObjectId;
use bson::Document;
use chrono::Utc;
use mongodb::bson::{self, doc};
use mongodb::Collection;

use crate::app::models::user::{User, UserProfile};
use crate::app::services::cache::service::CacheService;
use crate::app::services::storage::service::StorageService;

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
    ) -> Result<User, UserRepositoryError> {
        // TODO: implement caching on this level
        if let Some(user) = self
            .get_collection()
            .find_one_and_update(
                filter.clone(),
                doc! { "$set": data_to_update.clone() },
                None,
            )
            .await?
        {
            Ok(user)
        } else {
            log::error!(
                "Error to update user. filter: {}, data: {}",
                filter,
                data_to_update
            );
            Err(UserRepositoryError::UpdateUserError)
        }
    }

    pub async fn insert_user(&self, user: User) -> Result<(), UserRepositoryError> {
        // TODO: check if it returns _id which is the same as user._id
        self.get_collection().insert_one(user, None).await?;
        Ok(())
    }

    fn get_collection(&self) -> Collection<User> {
        self.storage.get_collection::<User>(&self.collection)
    }

    pub fn get_update_user_profile_query(user_profile: UserProfile) -> Document {
        let mut data_to_update = match user_profile {
            UserProfile::Google(google_profile) => {
                doc! {
                    "google.name": google_profile.name,
                    "google.email": google_profile.email,
                    "google.email_verified": google_profile.email_verified,
                    "google.picture": google_profile.picture,
                }
            }
            UserProfile::Facebook(facebook_profile) => {
                doc! {
                    "facebook.name": facebook_profile.name,
                    "facebook.email": facebook_profile.email,
                }
            }
        };
        data_to_update.insert("updated_at", Utc::now());

        data_to_update
    }
    pub fn get_find_user_by_profile_query(user_profile: UserProfile) -> Document {
        let mut query = doc! {};
        match user_profile {
            UserProfile::Google(google_profile) => {
                query.insert("google.user_id", google_profile.user_id);
            }
            UserProfile::Facebook(facebook_profile) => {
                query.insert("facebook.user_id", facebook_profile.user_id);
            }
        }

        query
    }
    pub fn get_find_user_by_id_query(user_id: ObjectId) -> Document {
        doc! {
            "_id": user_id
        }
    }
}
