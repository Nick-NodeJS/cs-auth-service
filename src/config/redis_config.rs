use cs_shared_lib::validation::{is_valid_ipv4, validate_integer_in_range, validate_ip_port};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub facebook_database: i16,
    pub google_database: i16,
    pub session_database: i16,
    pub user_database: i16,
    pub cyber_sherlock_auth_database: i16,
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

        // Validate and parse the redis Facebook database
        let facebook_database = dotenv::var("REDIS_FACEBOOK_DATABASE")
            .expect("REDIS_FACEBOOK_DATABASE environment variable is not set")
            .parse()
            .expect("Invalid Redis Facebook database");

        if !validate_integer_in_range(facebook_database, 0, 15) {
            panic!("Redis Facebook database out of the range");
        }

        // Validate and parse the redis Google database
        let google_database = dotenv::var("REDIS_GOOGLE_DATABASE")
            .expect("REDIS_GOOGLE_DATABASE environment variable is not set")
            .parse()
            .expect("Invalid Redis Google database");

        if !validate_integer_in_range(google_database, 0, 15) {
            panic!("Redis Google database out of the range");
        }

        // Validate and parse the redis Session database
        let session_database = dotenv::var("REDIS_SESSION_DATABASE")
            .expect("REDIS_SESSION_DATABASE environment variable is not set")
            .parse()
            .expect("Invalid Redis Session database");

        if !validate_integer_in_range(session_database, 0, 15) {
            panic!("Redis Session database out of the range");
        }

        // Validate and parse the redis User database
        let user_database = dotenv::var("REDIS_USER_DATABASE")
            .expect("REDIS_USER_DATABASE environment variable is not set")
            .parse()
            .expect("Invalid Redis User database");

        if !validate_integer_in_range(user_database, 0, 15) {
            panic!("Redis User database out of the range");
        }

        // Validate and parse the redis cyber_sherlock_auth_database database
        let cyber_sherlock_auth_database = dotenv::var("REDIS_CYBER_SHERLOCK_AUTH_DATABASE")
            .expect("REDIS_CYBER_SHERLOCK_AUTH_DATABASE environment variable is not set")
            .parse()
            .expect("Invalid Redis CyberSherlockAuth database");

        if !validate_integer_in_range(cyber_sherlock_auth_database, 0, 15) {
            panic!("Redis CyberSherlockAuth database out of the range");
        }

        Self {
            host,
            port,
            facebook_database,
            google_database,
            session_database,
            user_database,
            cyber_sherlock_auth_database,
        }
    }

    // Combine server address and port to Redis connection URL
    pub fn get_redis_url(&self, database: i16) -> String {
        format!("redis://{}:{}/{}", self.host, self.port, database)
    }
}
