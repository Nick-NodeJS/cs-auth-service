use cs_shared_lib::validation::validate_integer_in_range;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GoogleConfig {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_oauth_url: String,
    pub google_token_url: String,
    pub google_revoke_url: String,
    pub google_redirect_url: String,
    pub google_plus_me_url: String,
    pub google_cert_url: String,
    pub google_cache_state_ttl_sec: u32,
    pub google_test_token: String,
    pub google_cache_certs_key: String,
}

impl GoogleConfig {
    pub fn new() -> Self {
        dotenv().ok();

        // Google envs
        let google_client_id = dotenv::var("GOOGLE_CLIENT_ID")
            .expect("Missing the GOOGLE_CLIENT_ID environment variable.");
        let google_client_secret = dotenv::var("GOOGLE_CLIENT_SECRET")
            .expect("Missing the GOOGLE_CLIENT_SECRET environment variable.");
        let google_oauth_url = dotenv::var("GOOGLE_OAUTH_URL")
            .expect("Missing the GOOGLE_AUTH_URL environment variable.");
        let google_token_url = dotenv::var("GOOGLE_TOKEN_URL")
            .expect("Missing the GOOGLE_TOKEN_URL environment variable.");
        let google_revoke_url = dotenv::var("GOOGLE_REVOKE_URL")
            .expect("Missing the GOOGLE_REVOKE_URL environment variable.");
        let google_redirect_url = dotenv::var("GOOGLE_REDIRECT_URL")
            .expect("Missing the GOOGLE_REDIRECT_URL environment variable.");
        let google_plus_me_url = dotenv::var("GOOGLE_PLUS_ME_URL")
            .expect("Missing the GOOGLE_PLUS_ME_URL environment variable.");
        let google_cert_url = dotenv::var("GOOGLE_CERT_URL")
            .expect("Missing the GOOGLE_CERT_URL environment variable.");
        let google_cache_certs_key = dotenv::var("GOOGLE_CACHE_CERTS_KEY")
            .expect("Missing the GOOGLE_CACHE_CERTS_KEY environment variable.");
        let google_cache_state_ttl_sec: u32 = dotenv::var("GOOGLE_STATE_CACHE_TTL_SEC")
            .expect("GOOGLE_STATE_CACHE_TTL_SEC environment variable is not set")
            .parse()
            .expect("Invalid GOOGLE_STATE_CACHE_TTL_SEC");
        let google_test_token = dotenv::var("GOOGLE_TEST_TOKEN")
            .expect("Missing the GOOGLE_CERT_URL environment variable.");

        // Validate TTL in milliseconds to keep Google OAuth2 state in Redis
        // make sense to keep it not more than 3 min
        if !validate_integer_in_range(google_cache_state_ttl_sec, 1, 3 * 60) {
            panic!("GOOGLE_STATE_CACHE_TTL_SEC out of the range");
        }

        Self {
            google_client_id,
            google_client_secret,
            google_oauth_url,
            google_token_url,
            google_revoke_url,
            google_redirect_url,
            google_plus_me_url,
            google_cert_url,
            google_cache_state_ttl_sec,
            google_cache_certs_key,
            google_test_token,
        }
    }
}
