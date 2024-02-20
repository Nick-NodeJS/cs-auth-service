pub mod app_data;
pub mod app_error;
pub mod common;
pub mod handlers;
pub mod middlewares;
pub mod models;
pub mod providers;
pub mod repositories;
pub mod services;
pub mod shared;

use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_validator::JsonConfig;

use crate::app::app_data::AppData;
use crate::app::common::api_path::{
    API, AUTH, CALLBACK, CYBER_SHERLOCK, FACEBOOK, GOOGLE, LOGIN, LOGOUT, REGISTER, STATUS, V1,
};
use crate::app::common::error_handler;
use crate::app::handlers::logout::logout;
use crate::app::middlewares::session::SessionMiddleware;
use crate::app::services::cache::common::CacheServiceType;
use crate::app::services::cache::service::RedisCacheService;
use crate::config::app_config::AppConfig;
use crate::config::session_config::SessionConfig;
use env_logger::Env;
use log::info;

use crate::app::handlers::cyber_sherlock::{
    auth_callback::auth_callback as cyber_sherlock_auth_callback,
    login::login as login_with_cyber_sherlock, register::register as cyber_sherlock_register,
};
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
            .app_data(JsonConfig::default().error_handler(error_handler))
            .service(
                web::scope(API).service(
                    web::scope(V1)
                        .wrap(SessionMiddleware::new(
                            session_cache_service.clone(),
                            SessionConfig::new(),
                        ))
                        .service(
                            web::scope(AUTH)
                                .service(
                                    web::scope(CYBER_SHERLOCK)
                                        .route(LOGIN, web::post().to(login_with_cyber_sherlock))
                                        .route(
                                            CALLBACK,
                                            web::get().to(cyber_sherlock_auth_callback),
                                        )
                                        .route(REGISTER, web::post().to(cyber_sherlock_register)),
                                )
                                .service(
                                    web::scope(FACEBOOK)
                                        .route(LOGIN, web::get().to(login_with_facebook))
                                        .route(CALLBACK, web::get().to(facebook_auth_callback)),
                                )
                                .service(
                                    web::scope(GOOGLE)
                                        .route(LOGIN, web::get().to(login_with_google))
                                        .route(CALLBACK, web::get().to(google_auth_callback)),
                                )
                                .route(LOGOUT, web::get().to(logout)),
                        ),
                ),
            )
            .route(STATUS, web::get().to(status))
    })
    .bind(server_address_with_port)?
    .run()
    .await
}
