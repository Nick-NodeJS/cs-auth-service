mod handlers;
mod services;
mod app_data;
mod app_error;

use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer, middleware::Logger};

use cs_shared_lib::redis;
use log::info;
use env_logger::{Env, init_from_env, try_init_from_env};
use crate::app::app_data::AppData;
use crate::app::services::cache::service::CacheService;
use crate::app::services::user::service::UserService;
use crate::config::{
    app_config::AppConfig,
    google_config::GoogleConfig,
    redis_config::RedisConfig,
};

use crate::app::handlers::google::{
    auth_callback::auth_callback as google_auth_callback,
    login::login as login_with_google,
};

use crate::app::handlers::health_check::status;

use crate::app::services::google::service::GoogleService;


/**
 * TODO:
 * 1. finish redirect flow
 * 2. implement auth flow with multi providers
 * 3. tests
 * 4. docs
 */

pub async fn run() -> std::io::Result<()> {
    // Initialize the logger
    // env_logger::init_from_env(Env::default().filter_or("RUST_LOG", "info"));
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let app_config = AppConfig::new();
    // let redis_config = RedisConfig::new();

    info!("Service address {}", app_config.server_address_with_port());

    let server_address_with_port = app_config.server_address_with_port();

    let app_data = match AppData::new().await {
        Ok(data) => data,
        Err(err) => panic!("Error to create AppData: {:?}", err),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
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