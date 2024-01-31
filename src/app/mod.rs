pub mod app_data;
pub mod app_error;
pub mod handlers;
pub mod middlewares;
pub mod models;
pub mod providers;
pub mod repositories;
pub mod services;

use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::app::app_data::AppData;
use crate::app::handlers::logout::logout;
use crate::app::middlewares::session::SessionMiddleware;
use crate::app::services::cache::common::CacheServiceType;
use crate::app::services::cache::service::RedisCacheService;
use crate::config::app_config::AppConfig;
use crate::config::session_config::SessionConfig;
use env_logger::Env;
use log::info;

use crate::app::handlers::facebook::{
    auth_callback::auth_callback as facebook_auth_callback, login::login as login_with_facebook,
};
use crate::app::handlers::google::{
    auth_callback::auth_callback as google_auth_callback, login::login as login_with_google,
};

use crate::app::handlers::health_check::status;

/**
 * TODO:
 * 1. add tests
 * 2. add docs
 */

pub async fn run() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let app_config = AppConfig::new();

    info!("Service address {}", app_config.server_address_with_port());

    let server_address_with_port = app_config.server_address_with_port();

    let app_data = match AppData::new().await {
        Ok(data) => data,
        Err(err) => panic!("Error to create AppData: {:?}", err),
    };

    let session_cache_service = RedisCacheService::new(CacheServiceType::Session)
        .expect("Unable to create CacheService for Session Middleware");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_data.clone()))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/v1").service(
                            web::scope("/auth")
                                .service(
                                    web::scope("/facebook")
                                        .route("/login", web::get().to(login_with_facebook))
                                        .route("/callback", web::get().to(facebook_auth_callback)),
                                )
                                .service(
                                    web::scope("/google")
                                        .route("/login", web::get().to(login_with_google))
                                        .route("/callback", web::get().to(google_auth_callback)),
                                )
                                .wrap(SessionMiddleware::new(
                                    session_cache_service.clone(),
                                    SessionConfig::new(),
                                ))
                                .route("/logout", web::get().to(logout)),
                        ),
                    )
                    .route("/status", web::get().to(status)),
            )
    })
    .bind(server_address_with_port)?
    .run()
    .await
}
