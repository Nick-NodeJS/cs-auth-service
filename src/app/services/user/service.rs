use crate::app::{app_error::AppError, services::redis::service::RedisService};

use super::user::{UserProfile, GoogleProfile, User, UserActiveProfile};

pub struct UserService {
  redis_service: RedisService,
}

impl UserService {
  pub fn new(redis_service: RedisService) -> Result<Self, AppError> {
    Ok(UserService { redis_service })
  }
  pub async fn set_google_user(&self, user_data: GoogleProfile) -> Result<User, AppError> {
    let id = "fsdafsdf".to_string();
    Ok(User::new(id, UserActiveProfile::Google, UserProfile::Google(user_data))?)
  }
}