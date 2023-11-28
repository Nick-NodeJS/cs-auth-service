use std::sync::{Arc, Mutex};

use super::{
    app_error::AppError,
    services::{google::service::GoogleService, user::service::UserService},
};

#[derive(Clone)]
pub struct AppData {
    pub google_service: Arc<Mutex<GoogleService>>,
    pub user_service: Arc<Mutex<UserService>>,
}

impl AppData {
    pub async fn new() -> Result<AppData, AppError> {
        // Set AppData to share services, configs etc
        let mut google_service = GoogleService::new().await?;
        google_service.init().await?;

        let user_service = UserService::new().await?;

        let app_data = AppData {
            google_service: Arc::new(Mutex::new(google_service)),
            user_service: Arc::new(Mutex::new(user_service)),
        };
        Ok(app_data)
    }
}
