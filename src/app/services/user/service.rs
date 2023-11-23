use crate::app::{
    models::user::{User, UserProfile},
    repositories::user::repository::UserRepository,
    services::{cache::service::CacheService, storage::service::StorageService},
};

use super::error::UserServiceError;

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(
        cache_service: CacheService,
        storage_service: StorageService,
    ) -> Result<Self, UserServiceError> {
        let user_repository = UserRepository::new(
            storage_service.config.user_collection.clone(),
            cache_service,
            storage_service,
        );
        Ok(UserService { user_repository })
    }

    // TODO:
    // - check it in sessions
    pub async fn check_if_user_logged_in(
        &self,
        user_profile: UserProfile,
    ) -> Result<Option<String>, UserServiceError> {
        Ok(Some("fake_refresh_token".to_string()))
    }

    pub async fn create_or_update_user_with_profile(
        &self,
        user_profile: UserProfile,
    ) -> Result<User, UserServiceError> {
        if let Some(user) = self.get_user_by_profile(user_profile.clone()).await? {
            let query = User::get_update_user_profile_query(user_profile);
            self.user_repository
                .update_user(User::get_find_user_by_id_query(user.id), query)
                .await?;
            Ok(user)
        } else {
            let new_user = User::new(user_profile);
            self.user_repository.insert_user(new_user.clone()).await?;
            Ok(new_user)
        }
    }

    pub async fn get_user_by_profile(
        &self,
        user_profile: UserProfile,
    ) -> Result<Option<User>, UserServiceError> {
        let query = User::get_find_user_by_profile_query(user_profile);
        Ok(self.user_repository.get_user(query).await?)
    }
}
