use oauth2::url::Url;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl, AuthorizationCode, PkceCodeVerifier, TokenResponse,
};
use oauth2::reqwest::http_client;

use crate::config::google_config::GoogleConfig;

pub struct GoogleService {
    client: BasicClient,
    config: GoogleConfig,
}

impl GoogleService {
    pub fn new(config: GoogleConfig) -> Self {
      let google_client_id = ClientId::new(config.google_client_id.to_string());
      let google_client_secret = ClientSecret::new(config.google_client_secret.to_string());
      let oauth_url = AuthUrl::new(config.google_oauth_url.to_string())
          .expect("Invalid authorization endpoint URL");
      let token_url = TokenUrl::new(config.google_token_url.to_string())
          .expect("Invalid token endpoint URL");
  
      // Set up the config for the Google OAuth2 process.
      let client = BasicClient::new(
          google_client_id,
          Some(google_client_secret),
          oauth_url,
          Some(token_url),
      )
      .set_redirect_uri(
          RedirectUrl::new(config.google_redirect_url.to_string())
              .expect("Invalid redirect URL"),
      )
      // Google supports OAuth 2.0 Token Revocation (RFC-7009)
      .set_revocation_uri(
          RevocationUrl::new(config.google_revoke_url.to_string())
              .expect("Invalid revocation endpoint URL"),
      );
        GoogleService { client, config }
    }

    pub fn get_authorization_url_data(&self) -> (Url, CsrfToken, String, u32) {
      // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
      // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
      let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
      let (authorize_url, csrf_state) = self.client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the user's profile.
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/plus.me".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();
      return (
        authorize_url,
        csrf_state,
        pkce_code_verifier.secret().to_string(),
        self.config.google_redis_state_ttl_ms,
      );
    }

    pub fn get_tokens(&self, code: String, pkce_code_verifier: String) -> Result<(String, String), String> {
      // Exchange the code with a token.
      match self.client
      .exchange_code(AuthorizationCode::new(code))
      .set_pkce_verifier(PkceCodeVerifier::new(pkce_code_verifier))
      .request(http_client) {
        Ok(token_response) => {
          let access_token = token_response.access_token().secret();
          if let Some(refresh_token) = token_response.refresh_token() {
            return Ok((access_token.to_owned(), refresh_token.secret().to_owned()));
          } else {
            return Err("token response doesn't have refresh token".to_string());
          }
        },
        Err(err) => {
          return Err(err.to_string());
        },
      };
    }
}