use crate::app::services::{cache::service::CacheService, storage::service::StorageService};

use super::{
    error::UserServiceError,
    user::{User, UserActiveProfile, UserProfile},
};

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
    // - add refresh token to session collection
    // - update cache with user session
    pub async fn set_user(&self, user_profile: UserProfile) -> Result<(), UserServiceError> {
        /// TODO: generate real id
        let id = "fsdafsdf".to_string();
        match user_profile {
            UserProfile::Google(google_profile) => {
                println!(
                    "New Google User: {:?}",
                    User::new(
                        id,
                        UserActiveProfile::Google,
                        UserProfile::Google(google_profile),
                    )?
                );
                Ok(())
            }
            UserProfile::Facebook(facebook_profile) => {
                println!(
                    "New Facebook User: {:?}",
                    User::new(
                        id,
                        UserActiveProfile::Facebook,
                        UserProfile::Facebook(facebook_profile),
                    )?
                );
                Ok(())
            }
            _ => Err(UserServiceError::WrongProfileError),
        }
    }
    pub async fn check_if_google_user_logged_in(
        &self,
        google_user_id: String,
    ) -> Result<Option<String>, UserServiceError> {
        Ok(Some("fake_refresh_token".to_string()))
    }
}
