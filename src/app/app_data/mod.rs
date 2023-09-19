use std::sync::{Mutex, Arc};

use crate::config::google_config::GoogleConfig;

use super::redis::service::RedisService;


#[derive(Clone)]
pub struct AppData {
  pub google_config: GoogleConfig,
  pub redis_service: Arc<Mutex<RedisService>>,
}