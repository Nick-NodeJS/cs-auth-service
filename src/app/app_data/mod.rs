use std::sync::{Mutex, Arc};

use super::services::{
  redis::service::RedisService,
  google::service::GoogleService,
};


#[derive(Clone)]
pub struct AppData {
  pub google_service: Arc<Mutex<GoogleService>>,
  pub redis_service: Arc<Mutex<RedisService>>,
}