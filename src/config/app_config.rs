// src/config.rs
use serde::Deserialize;
use dotenv::dotenv;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub server_address: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv().ok();

        let server_address = dotenv::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());

        // Validate and parse the server port
        let server_port = dotenv::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|_| "Invalid server port")
            .expect("Invalid server port");

        // Validate server address using the ip-address crate
        if !is_valid_ipv4(&server_address) {
            panic!("Invalid server address");
        }

        Self {
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

fn is_valid_ipv4(ip: &str) -> bool {
    // TODO upgrade this Regex validation to use some lib

    let ip_pattern = regex::Regex::new(r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$").unwrap();
    ip_pattern.is_match(ip)
}
