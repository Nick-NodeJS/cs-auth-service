mod handlers;

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

async fn hello() -> HttpResponse {
    println!("get one!");
    HttpResponse::Ok().body("Hello from cs-auth-service!\n")
}


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
            .app_data(web::Data::new(google_config.clone()))
            .route("/", web::get().to(hello))
            .route("/login/google", web::get().to(login_with_google))
            .route("/callback/google/auth", web::get().to(google_auth_callback))
    })
    .bind(server_address_with_port)?
    .run()
    .await
}