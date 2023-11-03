use crate::app::{app_error::AppError, services::redis::service::RedisService};

use super::user::{UserProfile, GoogleProfile, User, UserActiveProfile};

pub struct UserService {}

impl UserService {
  pub fn new() -> Result<Self, AppError> {
    Ok(UserService {})
  }
  pub async fn set_google_user(&self, user_data: GoogleProfile, redis_service: RedisService) -> Result<User, AppError> {
    let id = "fsdafsdf".to_string();
    Ok(User::new(id, UserActiveProfile::Google, user_data)?)
  }
}