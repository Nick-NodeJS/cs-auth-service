use std::sync::{Mutex, Arc};
use crate::config::google_config::GoogleConfig;

use super::{services::{
  google::service::GoogleService,
  cache::service::CacheService,
  user::service::UserService
}, app_error::AppError};


#[derive(Clone)]
pub struct AppData {
  pub google_service: Arc<Mutex<GoogleService>>,
  pub cache_service: Arc<Mutex<CacheService>>,
  pub user_service: Arc<Mutex<UserService>>,
}

impl AppData {
  pub async fn new() -> Result<AppData, AppError> {
    // Set AppData to share services, configs etc
    let google_config = GoogleConfig::new();
    
    let google_service = GoogleService::new(google_config).await?;
    let cache_service = match CacheService::new() {
        Ok(service) => service,
        Err(err) => panic!("{:?}", err),
    };
    let user_service = UserService::new(cache_service.clone())?;
    let app_data = AppData {
        google_service: Arc::new(Mutex::new(google_service)),
        cache_service: Arc::new(Mutex::new(cache_service)),
        user_service: Arc::new(Mutex::new(user_service)),
    };
    Ok(app_data)
  }
}