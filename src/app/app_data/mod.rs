use std::sync::{Mutex, Arc};
use super::services::{google::service::GoogleService, redis::service::RedisService};


#[derive(Clone)]
pub struct AppData {
  pub google_service: Arc<Mutex<GoogleService>>,
  pub redis_service: Arc<Mutex<RedisService>>,
  pub user_service: Arc<Mutex<UserService>>,
}