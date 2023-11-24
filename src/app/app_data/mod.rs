use std::sync::{Arc, Mutex};

use super::{
    app_error::AppError,
    services::{
        cache::service::CacheService, google::service::GoogleService,
        storage::service::StorageService, user::service::UserService,
    },
};

#[derive(Clone)]
pub struct AppData {
    pub google_service: Arc<Mutex<GoogleService>>,
    pub user_service: Arc<Mutex<UserService>>,
}

impl AppData {
    pub async fn new() -> Result<AppData, AppError> {
        // Set AppData to share services, configs etc

        let cache_service = match CacheService::new() {
            Ok(service) => service,
            Err(err) => panic!("{:?}", err),
        };

        let storage_service = StorageService::new().await?;

        let google_service = GoogleService::new(cache_service.clone()).await?;

        let user_service = UserService::new(cache_service, storage_service);

        let app_data = AppData {
            google_service: Arc::new(Mutex::new(google_service)),
            user_service: Arc::new(Mutex::new(user_service)),
        };
        Ok(app_data)
    }
}
