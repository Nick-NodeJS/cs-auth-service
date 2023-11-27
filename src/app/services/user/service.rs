use bson::oid::ObjectId;

use crate::app::{
    models::{
        common::AuthProviders,
        session::Session,
        user::{User, UserProfile},
    },
    repositories::{session::repository::SessionRepository, user::repository::UserRepository},
    services::{
        cache::service::{CacheService, CacheServiceType},
        session::service::SessionService,
        storage::service::StorageService,
    },
};

use super::error::UserServiceError;

pub struct UserService {
    session_service: SessionService,
    user_repository: UserRepository,
}

impl UserService {
    pub async fn new() -> Result<Self, UserServiceError> {
        let storage_service = StorageService::new().await?;
        let user_cache_service = CacheService::new(CacheServiceType::User)?;
        let user_repository = UserRepository::new(
            storage_service.config.user_collection.clone(),
            user_cache_service,
            storage_service,
        );
        let session_cache_service = CacheService::new(CacheServiceType::Session)?;
        let session_service = SessionService::new(SessionRepository::new(session_cache_service));
        Ok(UserService {
            session_service,
            user_repository,
        })
    }

    // TODO:
    // - check it in sessions
    pub async fn check_if_user_logged_in_with_profile(
        &mut self,
        user_profile: UserProfile,
    ) -> Result<Option<Session>, UserServiceError> {
        if let Some(existen_user) = self.get_user_by_profile(user_profile.clone()).await? {
            if let Some(session) = self
                .session_service
                .get_session(existen_user.id, UserProfile::get_provider(user_profile))
                .await?
            {
                Ok(Some(session))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn create_user_with_profile(
        &self,
        user_profile: UserProfile,
    ) -> Result<User, UserServiceError> {
        let new_user = User::new(user_profile);
        self.user_repository.insert_user(new_user.clone()).await?;
        Ok(new_user)
    }

    pub async fn update_user_with_profile(
        &self,
        user_id: ObjectId,
        user_profile: UserProfile,
    ) -> Result<User, UserServiceError> {
        let query = UserRepository::get_update_user_profile_query(user_profile);
        let user = self
            .user_repository
            .update_user(UserRepository::get_find_user_by_id_query(user_id), query)
            .await?;
        Ok(user)
    }

    pub async fn get_user_by_profile(
        &self,
        user_profile: UserProfile,
    ) -> Result<Option<User>, UserServiceError> {
        let query = UserRepository::get_find_user_by_profile_query(user_profile);
        Ok(self.user_repository.get_user(query).await?)
    }

    pub async fn create_user_and_session(
        &mut self,
        user_profile: UserProfile,
        token: String,
    ) -> Result<(), UserServiceError> {
        let user = self.create_user_with_profile(user_profile.clone()).await?;
        self.session_service
            .set_session(UserProfile::get_provider(user_profile), user.id, token)
            .await?;
        Ok(())
    }

    pub async fn update_user_and_session(
        &mut self,
        user_profile: UserProfile,
        user_session: Session,
    ) -> Result<(), UserServiceError> {
        self.update_user_with_profile(user_session.user_id.clone(), user_profile.clone())
            .await?;
        self.session_service
            .update_session(user_session, None)
            .await?;
        Ok(())
    }
}
