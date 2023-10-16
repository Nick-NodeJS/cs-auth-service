mod handlers;
mod services;
mod app_data;

use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};

use log::info;
use env_logger::Env;
use crate::app::app_data::AppData;
use crate::config::{
    app_config::AppConfig,
    google_config::GoogleConfig,
};

use crate::app::handlers::google::{
    auth_callback::auth_callback as google_auth_callback,
    login::login as login_with_google,
};

use crate::app::handlers::health_check::status;

use crate::app::services::{
    redis::service::RedisService,
    google::service::GoogleService,
};


/**
 * TODO:
 * 1. finish redirect flow
 * 2. implement auth flow with multi providers
 * 3. tests
 * 4. docs
 */

pub async fn run() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::Builder::from_env(Env::default().filter_or("RUST_LOG", "info")).init();

    let app_config = AppConfig::new();
    let google_config = GoogleConfig::new();

    info!("Service address {}", app_config.server_address_with_port());

    let server_address_with_port = app_config.server_address_with_port();

    // Set AppData to share services, configs etc
    let mut google_service = GoogleService::new(google_config);
    if let Err(err) = google_service.init().await {
        panic!("Error to init Google Service: {}", err.to_string());
    }
    let redis_service = match RedisService::new() {
        Ok(service) => service,
        Err(err) => panic!("{:?}", err),
    };
    let app_data = AppData {
        google_service: Arc::new(Mutex::new(google_service)),
        redis_service: Arc::new(Mutex::new(redis_service)),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .service(
                web::scope(format!("/api/{}", app_config.api_version).as_ref())
                .service(
                    web::scope("/auth")
                    .route("/google", web::get().to(login_with_google))
                    .route("/google/callback", web::get().to(google_auth_callback))
                )
                .route("/status", web::get().to(status))
                // .service(
                //     web::scope("/users")
                //     .wrap(authentication_middleware)
                //     .route("/me", web::get().to(user_profile))
                // )
            )
    })
    .bind(server_address_with_port)?
    .run()
    .await
}