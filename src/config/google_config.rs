use serde::Deserialize;
use dotenv::dotenv;

#[derive(Deserialize, Clone)]
pub struct GoogleConfig {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_oauth_url: String,
    pub google_token_url: String,
    pub google_revoke_url: String,
    pub google_redirect_url: String,
    pub google_plus_me_url: String,
    pub google_cert_url: String,
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

        Self {
            google_client_id,
            google_client_secret,
            google_oauth_url,
            google_token_url,
            google_revoke_url,
            google_redirect_url,
            google_plus_me_url,
            google_cert_url,
        }
    }
}
