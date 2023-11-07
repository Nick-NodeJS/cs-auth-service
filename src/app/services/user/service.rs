use crate::app::services::cache::service::CacheService;

use super::{user::{UserProfile, GoogleProfile, User, UserActiveProfile}, error::UserServiceError};

pub struct UserService {
  cache_service: CacheService,
}

impl UserService {
  pub fn new(cache_service: CacheService) -> Result<Self, UserServiceError> {
    Ok(UserService { cache_service })
  }
  pub async fn set_user(&self, user_profile: UserProfile) -> Result<User, UserServiceError> {
    /// TODO: generate real id
    let id = "fsdafsdf".to_string();
    match user_profile {
      UserProfile::Google(google_profile) => Ok(
        User::new(id, UserActiveProfile::Google, UserProfile::Google(google_profile)
      )?),
      UserProfile::Facebook(facebook_profile) => Ok(
        User::new(id, UserActiveProfile::Facebook, UserProfile::Facebook(facebook_profile)
      )?),
      _ => Err(UserServiceError::WrongProfileError)
    }
    
  }
}