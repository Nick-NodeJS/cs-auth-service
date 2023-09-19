// src/config.rs
use serde::Deserialize;
use dotenv::dotenv;
use cs_shared_lib::{
  is_valid_ipv4,
  validate_ip_port,
  validate_integer_in_range,
};

#[derive(Deserialize, Clone)]
pub struct RedisConfig {
  pub redis_address: String,
  pub redis_port: u16,
  pub redis_database: u16,
}

impl RedisConfig {
    pub fn new() -> Self {
      dotenv().ok();

      let redis_address = dotenv::var("REDIS_HOST")
          .expect("REDIS_HOST environment variable is not set");

      // Validate and parse the redis port
      let redis_port = dotenv::var("REDIS_PORT")
          .expect("REDIS_PORT environment variable is not set")
          .parse()
          .expect("Invalid redis port");

      if !validate_ip_port(redis_port) {
        panic!("Server port out of the range");
      }

      // Validate redis address using the ip-address crate
      if !is_valid_ipv4(&redis_address) {
          panic!("Invalid redis address");
      }

      // Validate and parse the redis database
      let redis_database = dotenv::var("REDIS_DATABASE")
          .unwrap_or_else(|_| "1".to_string())
          .parse()
          .map_err(|_| "Invalid Redis database")
          .expect("Invalid Redis database");

      if !validate_integer_in_range(redis_database, 0, 15) {
        panic!("Redis database out of the range");
          }

      Self {
          redis_address,
          redis_port,
          redis_database,
      }
    }

    // Combine server address and port as a single string
    pub fn get_redis_url(&self) -> String {
      format!(
        "redis://{}:{}/{}",
        self.redis_address,
        self.redis_port,
        self.redis_database,
      )
  }
}