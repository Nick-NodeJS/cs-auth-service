use cs_shared_lib::validation::{is_valid_ipv4, validate_ip_port};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub app_id: String,
    pub auth_callback_url: String,
    pub server_address: String,
    pub server_port: u16,
    pub jwt_access_token_ttl_sec: i64,
    pub jwt_refresh_token_ttl_sec: i64,
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv().ok();

        let app_id = dotenv::var("APP_ID").expect("APP_ID environment variable is not set");

        let server_address =
            dotenv::var("SERVER_ADDRESS").expect("SERVER_ADDRESS environment variable is not set");

        // Validate and parse the server port
        let server_port = dotenv::var("SERVER_PORT")
            .expect("SERVER_PORT environment variable is not set")
            .parse()
            .expect("Invalid server port");

        if !validate_ip_port(server_port) {
            panic!("Server port out of the range");
        }

        // Validate server address using the ip-address crate
        if !is_valid_ipv4(&server_address) {
            panic!("Invalid server address");
        }

        let auth_callback_url = dotenv::var("AUTH_CALLBACK_URL")
            .expect("Missing the AUTH_CALLBACK_URL environment variable.");

        let jwt_access_token_ttl_sec = dotenv::var("ACCESS_TOKEN_TTL_SEC")
            .expect("ACCESS_TOKEN_TTL_SEC environment variable is not set")
            .parse()
            .expect("Invalid ACCESS_TOKEN_TTL_SEC");

        let jwt_refresh_token_ttl_sec = dotenv::var("REFRESH_TOKEN_TTL_SEC")
            .expect("REFRESH_TOKEN_TTL_SEC environment variable is not set")
            .parse()
            .expect("Invalid REFRESH_TOKEN_TTL_SEC");

        let jwt_secret =
            dotenv::var("JWT_SECRET").expect("JWT_SECRET environment variable is not set");

        Self {
            app_id,
            auth_callback_url,
            jwt_access_token_ttl_sec,
            jwt_refresh_token_ttl_sec,
            jwt_secret,
            server_address,
            server_port,
            // Add other configuration settings here
        }
    }

    // Combine server address and port as a single string
    pub fn server_address_with_port(&self) -> String {
        format!("{}:{}", self.server_address, self.server_port)
    }
}
