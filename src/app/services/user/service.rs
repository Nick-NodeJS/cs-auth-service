use crate::app::{
    models::user::{FacebookProfile, GoogleProfile, User, UserActiveProfile, UserProfile},
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
    pub async fn set_user(&self, user_profile: UserProfile) -> Result<(), UserServiceError> {
        // TODO: generate real id
        /*
            - set user
            - set user session
        */
        let user = match user_profile {
            UserProfile::Google(google_profile) => {
                let user = self.insert_or_update_google_user(google_profile).await?;
                Ok(())
            }
            UserProfile::Facebook(facebook_profile) => Ok(()),
            _ => Err(UserServiceError::WrongProfileError),
        };
        Ok(())
    }
    pub async fn check_if_google_user_logged_in(
        &self,
        google_user_id: String,
    ) -> Result<Option<String>, UserServiceError> {
        Ok(Some("fake_refresh_token".to_string()))
    }

    async fn insert_or_update_google_user(
        &self,
        google_profile: GoogleProfile,
    ) -> Result<User, UserServiceError> {
        if let Some(user) = self
            .storage_service
            .get_user_by_google_id(&google_profile.user_id)
            .await?
        {
            Ok(user)
        } else {
            let new_user = User::new(
                UserActiveProfile::Google,
                UserProfile::Google(google_profile),
            );
            self.storage_service.insert_user(new_user.clone()).await?;
            Ok(new_user)
        }
        /*
            - get user by google id
            - if exists, update user
            - else insert a new user
            - return fresh user
        */
    }

    async fn insert_or_update_facebook_user(
        &self,
        facebook_profile: FacebookProfile,
    ) -> Result<(), UserServiceError> {
        Ok(())
    }
}
