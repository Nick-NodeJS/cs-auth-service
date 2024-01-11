use bson::oid::ObjectId;
use bson::Document;
use chrono::Utc;
use mongodb::bson::{self, doc};
use mongodb::Collection;

use crate::app::models::user::{User, UserProfile};
use crate::app::services::cache::service::RedisCacheService;
use crate::app::services::storage::service::StorageService;
use crate::config::user_config::UserConfig;

use super::error::UserRepositoryError;

#[derive(Debug)]
pub struct UserRepository {
    config: UserConfig,
    collection: String,
    cache: RedisCacheService,
    storage: StorageService,
}

//#[allow(unused)]
impl UserRepository {
    pub fn new(
        collection: String,
        cache: RedisCacheService,
        config: UserConfig,
        storage: StorageService,
    ) -> Self {
        UserRepository {
            config,
            collection,
            cache,
            storage,
        }
    }

    pub async fn find_user_by_id(
        &mut self,
        user_id: ObjectId,
    ) -> Result<Option<User>, UserRepositoryError> {
        if let Some(user) = self.get_user_from_cache(user_id)? {
            return Ok(Some(user));
        };
        let filter = UserRepository::get_find_user_by_id_query(user_id);
        let user = self.get_collection().find_one(filter, None).await?;
        Ok(user)
    }

    pub async fn find_user_by_profile(
        &mut self,
        profile: UserProfile,
    ) -> Result<Option<User>, UserRepositoryError> {
        let filter = UserRepository::get_find_user_by_profile_query(profile);
        let user = self.find_one(filter).await?;
        Ok(user)
    }

    pub async fn find_one(&self, filter: Document) -> Result<Option<User>, UserRepositoryError> {
        let user = self.get_collection().find_one(filter, None).await?;
        Ok(user)
    }

    pub async fn update_user(
        &mut self,
        user_id: ObjectId,
        data_to_update: Document,
    ) -> Result<User, UserRepositoryError> {
        let filter = UserRepository::get_find_user_by_id_query(user_id);
        if let Some(user) = self
            .get_collection()
            .find_one_and_update(
                filter.clone(),
                doc! { "$set": data_to_update.clone() },
                None,
            )
            .await?
        {
            self.set_user_in_cache(user.clone())?;
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

    pub async fn insert_user(&mut self, user: User) -> Result<(), UserRepositoryError> {
        self.get_collection().insert_one(user.clone(), None).await?;
        self.set_user_in_cache(user)?;
        Ok(())
    }

    fn get_collection(&self) -> Collection<User> {
        self.storage.get_collection::<User>(&self.collection)
    }

    fn get_user_from_cache(
        &mut self,
        user_id: ObjectId,
    ) -> Result<Option<User>, UserRepositoryError> {
        let user = match self
            .cache
            .get_value::<User>(&User::get_user_cache_key(user_id.to_string().as_ref()))?
        {
            Some(user_string) => user_string,
            None => return Ok(None),
        };

        Ok(Some(user))
    }

    fn set_user_in_cache(&mut self, user: User) -> Result<(), UserRepositoryError> {
        self.cache.set_value_with_ttl::<User>(
            &User::get_user_cache_key(user.id.to_string().as_ref()),
            user,
            self.config.user_cache_ttl_sec,
        )?;
        Ok(())
    }

    pub fn get_update_user_profile_query(user_profile: UserProfile) -> Document {
        let mut data_to_update = match user_profile {
            UserProfile::CyberSherlock(cyber_sherlock_profile) => {
                doc! {
                    "cybersherlock.name": cyber_sherlock_profile.name,
                    "cybersherlock.email": cyber_sherlock_profile.email,
                    "cybersherlock.email_verified": cyber_sherlock_profile.email_verified,
                    "cybersherlock.picture": cyber_sherlock_profile.picture,
                }
            }
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
            UserProfile::CyberSherlock(cyber_sherlock_profile) => {
                query.insert("cybersherlock.user_id", cyber_sherlock_profile.user_id);
            }
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
