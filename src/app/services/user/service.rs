use crate::app::{
    models::{
        common::AuthProviders,
        user::{User, UserProfile},
    },
    repositories::{session::repository::SessionRepository, user::repository::UserRepository},
    services::{
        cache::service::CacheService, session::service::SessionService,
        storage::service::StorageService,
    },
};

use super::error::UserServiceError;

pub struct UserService {
    session_service: SessionService,
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(cache_service: CacheService, storage_service: StorageService) -> Self {
        let user_repository = UserRepository::new(
            storage_service.config.user_collection.clone(),
            cache_service.clone(),
            storage_service,
        );
        let session_service = SessionService::new(SessionRepository::new(cache_service));
        UserService {
            session_service,
            user_repository,
        }
    }

    // TODO:
    // - check it in sessions
    pub async fn check_if_user_logged_in_with_profile(
        &self,
        user_profile: UserProfile,
    ) -> Result<Option<String>, UserServiceError> {
        Ok(Some("fake_refresh_token".to_string()))
    }

    pub async fn create_or_update_user_with_profile(
        &self,
        user_profile: UserProfile,
    ) -> Result<User, UserServiceError> {
        if let Some(existen_user) = self.get_user_by_profile(user_profile.clone()).await? {
            let query = UserRepository::get_update_user_profile_query(user_profile);
            self.user_repository
                .update_user(
                    UserRepository::get_find_user_by_id_query(existen_user.id),
                    query,
                )
                .await?;
            Ok(existen_user)
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
        let query = UserRepository::get_find_user_by_profile_query(user_profile);
        Ok(self.user_repository.get_user(query).await?)
    }

    pub async fn set_user_session(
        &self,
        user: User,
        auth_provider: AuthProviders,
        token: String,
    ) -> Result<(), UserServiceError> {
        Ok(())
    }

    pub async fn set_user_and_session(
        &self,
        user_profile: UserProfile,
        token: String,
    ) -> Result<(), UserServiceError> {
        let user = self
            .create_or_update_user_with_profile(user_profile.clone())
            .await?;
        self.set_user_session(user, UserProfile::get_provider(user_profile), token)
            .await?;
        Ok(())
    }
}
