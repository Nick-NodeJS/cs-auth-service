use cs_shared_lib::validation::validate_integer_in_range;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct FacebookConfig {
    pub facebook_client_id: String,
    pub facebook_client_secret: String,
    pub facebook_oauth_url: String,
    pub facebook_token_url: String,
    pub facebook_revoke_url: String,
    pub facebook_redirect_url: String,
    pub facebook_userinfo_url: String,
    pub facebook_cache_state_ttl_sec: u64,
    pub facebook_debug_token_url: String,
}

impl FacebookConfig {
    pub fn new() -> Self {
        dotenv().ok();

        // Google envs
        let facebook_client_id = dotenv::var("FACEBOOK_CLIENT_ID")
            .expect("Missing the FACEBOOK_CLIENT_ID environment variable.");
        let facebook_client_secret = dotenv::var("FACEBOOK_CLIENT_SECRET")
            .expect("Missing the FACEBOOK_CLIENT_SECRET environment variable.");
        let facebook_oauth_url = dotenv::var("FACEBOOK_OAUTH_URL")
            .expect("Missing the FACEBOOK_AUTH_URL environment variable.");
        let facebook_token_url = dotenv::var("FACEBOOK_TOKEN_URL")
            .expect("Missing the FACEBOOK_TOKEN_URL environment variable.");
        let facebook_debug_token_url = dotenv::var("FACEBOOK_DEBUG_TOKEN_URL")
            .expect("Missing the FACEBOOK_DEBUG_TOKEN_URL environment variable.");
        let facebook_revoke_url = dotenv::var("FACEBOOK_REVOKE_URL")
            .expect("Missing the FACEBOOK_REVOKE_URL environment variable.");
        let facebook_redirect_url = dotenv::var("FACEBOOK_REDIRECT_URL")
            .expect("Missing the FACEBOOK_REDIRECT_URL environment variable.");
        let facebook_userinfo_url = dotenv::var("FACEBOOK_USERINFO_URL")
            .expect("Missing the FACEBOOK_USERINFO_URL environment variable.");
        let facebook_cache_state_ttl_sec: u64 = dotenv::var("FACEBOOK_STATE_CACHE_TTL_SEC")
            .expect("FACEBOOK_STATE_CACHE_TTL_SEC environment variable is not set")
            .parse()
            .expect("Invalid FACEBOOK_STATE_CACHE_TTL_SEC");

        // Validate TTL in milliseconds to keep Facebook OAuth2 state in Redis
        // make sense to keep it not more than 3 min
        if !validate_integer_in_range(facebook_cache_state_ttl_sec, 1, 3 * 60) {
            panic!("FACEBOOK_STATE_CACHE_TTL_SEC out of the range");
        }

        Self {
            facebook_client_id,
            facebook_client_secret,
            facebook_oauth_url,
            facebook_token_url,
            facebook_debug_token_url,
            facebook_revoke_url,
            facebook_redirect_url,
            facebook_userinfo_url,
            facebook_cache_state_ttl_sec,
        }
    }
}
