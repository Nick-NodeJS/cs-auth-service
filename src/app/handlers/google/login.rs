use crate::config::google_config::GoogleConfig;

use actix_web::{web, HttpResponse};
use oauth2::basic::BasicClient;
use oauth2::{
  AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
  RevocationUrl, Scope, TokenUrl,
};

pub async fn login(config: web::Data<GoogleConfig>) -> HttpResponse {
  let google_client_id = ClientId::new(config.google_client_id.to_string());
  let google_client_secret = ClientSecret::new(config.google_client_secret.to_string());
  let oauth_url = AuthUrl::new(config.google_oauth_url.to_string())
      .expect("Invalid authorization endpoint URL");
  let token_url = TokenUrl::new(config.google_token_url.to_string())
      .expect("Invalid token endpoint URL");
  // Generate the authorization URL and CSRF state
  let authorize_url = "";

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

  // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
  // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
  let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

  // Generate the authorization URL to which we'll redirect the user.
  let (authorize_url, csrf_state) = client
      .authorize_url(CsrfToken::new_random)
      // This example is requesting access to the user's profile.
      .add_scope(Scope::new(
          "https://www.googleapis.com/auth/plus.me".to_string(),
      ))
      .set_pkce_challenge(pkce_code_challenge)
      .url();

  println!(
      "Open this URL in your browser:\n{}\n",
      authorize_url.to_string()
  );

  // Redirect the user to the Google OAuth2 authorization page
  HttpResponse::Ok().body(authorize_url.to_string())
}