use cs_shared_lib::validation::{is_valid_ipv4, validate_ip_port};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub server_address: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv().ok();

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
