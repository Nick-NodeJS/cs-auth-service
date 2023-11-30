use cs_shared_lib::validation::validate_integer_in_range;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct SessionConfig {
    pub session_ttl_sec: u32,
}

impl SessionConfig {
    pub fn new() -> Self {
        dotenv().ok();

        let session_ttl_sec = dotenv::var("SESSION_TTL_SEC")
            .expect("SESSION_TTL_SEC environment variable is not set")
            .parse()
            .expect("Invalid SESSION_TTL_SEC");

        // Session ttl in range (1 min - 30 days)
        if !validate_integer_in_range(session_ttl_sec, 60, 30 * 24 * 60 * 60) {
            panic!("Session ttl out of the range(1 min - 30 days)");
        }

        Self { session_ttl_sec }
    }
}
