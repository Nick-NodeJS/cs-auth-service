use std::sync::{Arc, Mutex};

use crate::config::{
    app_config::AppConfig, facebook_config::FacebookConfig, google_config::GoogleConfig,
    user_config::UserConfig,
};

use super::{
    app_error::AppError,
    providers::{
        cyber_sherlock::provider::CyberSherlockAuthProvider, facebook::provider::FacebookProvider,
        google::provider::GoogleProvider, notification::provider::NotificationProvider,
    },
    services::{
        cache::{common::CacheServiceType, service::RedisCacheService},
        common::async_http_request,
        storage::service::StorageService,
        user::service::UserService,
    },
};

#[derive(Clone)]
pub struct AppData {
    pub facebook_provider: Arc<Mutex<FacebookProvider>>,
    pub google_provider: Arc<Mutex<GoogleProvider>>,
    pub cyber_sherlock_auth_provider: Arc<Mutex<CyberSherlockAuthProvider>>,
    pub user_service: Arc<Mutex<UserService>>,
}

impl AppData {
    pub async fn new() -> Result<AppData, AppError> {
        // Set AppData to share services, configs etc

        // Storage service
        let storage_service = StorageService::new().await?;

        // User Cache service
        let user_cache_service = RedisCacheService::new(CacheServiceType::User)?;

        // Session Cache service
        let session_cache_service = RedisCacheService::new(CacheServiceType::Session)?;

        // Facebook Cache service
        let facebook_cache_service = RedisCacheService::new(CacheServiceType::Facebook)?;

        let facebook_config = FacebookConfig::new();

        // let request = async_http_request;
        let facebook_provider = FacebookProvider::new(
            Box::new(async_http_request),
            facebook_config,
            facebook_cache_service,
        );

        // Google Cache service
        let google_cache_service = RedisCacheService::new(CacheServiceType::Google)?;

        let google_config = GoogleConfig::new();

        // let request = async_http_request;
        let mut google_provider = GoogleProvider::new(
            Box::new(async_http_request),
            google_config,
            google_cache_service,
        );

        google_provider.init().await?;

        let app_config = AppConfig::new();
        // CyberSherlockAuth Cache service
        let cyber_sherlock_auth_cache_service =
            RedisCacheService::new(CacheServiceType::CyberSherlock)?;

        let notification_provider = NotificationProvider::new(Box::new(async_http_request));

        let cyber_sherlock_auth_provider = CyberSherlockAuthProvider::new(
            app_config,
            cyber_sherlock_auth_cache_service,
            notification_provider,
        );

        let user_config = UserConfig::new();

        let user_service = UserService::new(
            user_config,
            storage_service,
            user_cache_service,
            session_cache_service,
        )
        .await?;

        let app_data = AppData {
            cyber_sherlock_auth_provider: Arc::new(Mutex::new(cyber_sherlock_auth_provider)),
            facebook_provider: Arc::new(Mutex::new(facebook_provider)),
            google_provider: Arc::new(Mutex::new(google_provider)),
            user_service: Arc::new(Mutex::new(user_service)),
        };
        Ok(app_data)
    }
}
