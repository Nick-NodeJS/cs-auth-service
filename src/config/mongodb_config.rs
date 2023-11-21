use cs_shared_lib::validation::{is_valid_ipv4, validate_ip_port};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MongoDBConfig {
    connection_uri_prefix: String,
    password: String,
    connection_options: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user_name: String,
    pub user_collection: String,
}

impl MongoDBConfig {
    pub fn new() -> Self {
        dotenv().ok();

        let host =
            dotenv::var("MONGODB_HOST").expect("MONGODB_HOST environment variable is not set");

        // Validate and parse the mongodb port
        let port = dotenv::var("MONGODB_PORT")
            .expect("MONGODB_PORT environment variable is not set")
            .parse()
            .expect("Invalid MONGODB port");

        if !validate_ip_port(port) {
            panic!("MONGODB port out of the range");
        }

        // Validate mongodb address using the ip-address crate
        if !is_valid_ipv4(&host) {
            panic!("Invalid MONGODB_HOST");
        }

        let connection_uri_prefix = dotenv::var("MONGODB_CONNECTION_URI_PREFIX")
            .expect("MONGODB_CONNECTION_URI_PREFIX environment variable is not set");

        let password = dotenv::var("MONGODB_PASSWORD")
            .expect("MONGODB_PASSWORD environment variable is not set");

        let database = dotenv::var("MONGODB_DATABASE")
            .expect("MONGODB_DATABASE environment variable is not set");

        let user_name = dotenv::var("MONGODB_USERNAME")
            .expect("MONGODB_USERNAME environment variable is not set");

        let connection_options = dotenv::var("MONGODB_CONNECTION_OPTIONS")
            .expect("MONGODB_CONNECTION_OPTIONS environment variable is not set");

        let user_collection = dotenv::var("MONGODB_USER_COLLECTION")
            .expect("MONGODB_USER_COLLECTION environment variable is not set");

        Self {
            connection_uri_prefix,
            connection_options,
            password,
            host,
            port,
            database,
            user_name,
            user_collection,
        }
    }

    // Combine config params to get connection uri
    pub fn get_connection_uri(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}?{}",
            self.connection_uri_prefix,
            self.user_name,
            self.password,
            self.host,
            self.port,
            self.database,
            self.connection_options,
        )
    }
}
