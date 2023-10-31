use std::sync::{Mutex, Arc};

use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;

use super::services::{google::service::GoogleService, redis::service::RedisService};


#[derive(Clone)]
pub struct AppData {
  pub google_service: Arc<Mutex<GoogleService>>,
  pub redis_service: Arc<Mutex<RedisService>>,
}