use crate::app::{
    models::user::{User, UserProfile},
    services::{cache::service::CacheService, storage::service::StorageService},
};

use super::error::UserServiceError;

pub struct UserService {
    cache_service: CacheService,
    storage_service: StorageService,
}

impl UserService {
    pub fn new(
        cache_service: CacheService,
        storage_service: StorageService,
    ) -> Result<Self, UserServiceError> {
        Ok(UserService {
            cache_service,
            storage_service,
        })
    }

    // TODO:
    // - create a new or update existing user
    // - insert/update user refresh token in session collection
    // - update cache with user session
    pub async fn check_if_user_logged_in(
        &self,
        user_id: String,
    ) -> Result<Option<String>, UserServiceError> {
        Ok(Some("fake_refresh_token".to_string()))
    }

    pub async fn create_or_update_user_with_profile(
        &self,
        user_profile: UserProfile,
    ) -> Result<User, UserServiceError> {
        if let Some(user) = self.get_user_by_profile(user_profile.clone()).await? {
            let query = User::get_update_user_profile_query(user_profile);
            self.storage_service
                .update_user(User::get_find_user_by_id_query(user.id), query)
                .await?;
            Ok(user)
        } else {
            let new_user = User::new(user_profile);
            self.storage_service.insert_user(new_user.clone()).await?;
            Ok(new_user)
        }
    }

    pub async fn get_user_by_profile(
        &self,
        user_profile: UserProfile,
    ) -> Result<Option<User>, UserServiceError> {
        let query = User::get_find_user_by_profile_query(user_profile);
        Ok(self.storage_service.get_user(query).await?)
    }
}
