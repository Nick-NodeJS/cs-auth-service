use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct UserConfig {
    pub user_cache_ttl_sec: u32,
}

impl UserConfig {
    pub fn new() -> Self {
        dotenv().ok();

        let user_cache_ttl_sec = dotenv::var("USER_CACHE_TTL_SEC")
            .expect("USER_CACHE_TTL_SEC environment variable is not set")
            .parse()
            .expect("Invalid USER_CACHE_TTL_SEC");

        Self { user_cache_ttl_sec }
    }
}
