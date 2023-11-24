use cs_shared_lib::validation::{is_valid_ipv4, validate_integer_in_range, validate_ip_port};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub database: u16,
}

impl RedisConfig {
    pub fn new() -> Self {
        dotenv().ok();

        let host = dotenv::var("REDIS_HOST").expect("REDIS_HOST environment variable is not set");

        // Validate and parse the redis port
        let port = dotenv::var("REDIS_PORT")
            .expect("REDIS_PORT environment variable is not set")
            .parse()
            .expect("Invalid redis port");

        if !validate_ip_port(port) {
            panic!("REDIS_PORT out of the range");
        }

        // Validate redis address using the ip-address crate
        if !is_valid_ipv4(&host) {
            panic!("Invalid redis address");
        }

        // Validate and parse the redis database
        let database = dotenv::var("REDIS_DATABASE")
            .expect("REDIS_DATABASE environment variable is not set")
            .parse()
            .expect("Invalid Redis database");

        if !validate_integer_in_range(database, 0, 15) {
            panic!("Redis database out of the range");
        }

        Self {
            host,
            port,
            database,
        }
    }

    // Combine server address and port to Redis connection URL
    pub fn get_redis_url(&self) -> String {
        format!("redis://{}:{}/{}", self.host, self.port, self.database,)
    }
}
