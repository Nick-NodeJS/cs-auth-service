mod handlers;
mod redis;

use actix_web::{web, App, HttpResponse, HttpServer};

use log::info;
use env_logger::Env;
use crate::config::{
    app_config::AppConfig,
    google_config::GoogleConfig,
};

use crate::app::handlers::google::{
    auth_callback::auth_callback as google_auth_callback,
    login::login as login_with_google,
};

use crate::app::handlers::health_check::status;

use crate::app::redis::service::RedisService;

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

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope(format!("/api/{}", app_config.api_version).as_ref())
                .service(
                    web::scope("/auth")
                    .app_data(web::Data::new(google_config.clone()))
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