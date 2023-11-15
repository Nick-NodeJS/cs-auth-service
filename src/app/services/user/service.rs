use crate::app::services::cache::service::CacheService;

use super::{
    error::UserServiceError,
    user::{User, UserActiveProfile, UserProfile},
};

pub struct UserService {
    cache_service: CacheService,
}

impl UserService {
    pub fn new(cache_service: CacheService) -> Result<Self, UserServiceError> {
        Ok(UserService { cache_service })
    }
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
        Ok(Some("".to_string()))
    }
}