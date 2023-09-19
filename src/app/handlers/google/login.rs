use crate::app::app_data::AppData;

use actix_web::{web, HttpResponse};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl,
};

pub async fn login(app_data: web::Data<AppData>) -> HttpResponse {
    let config = &app_data.google_config;
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

    // Set pkce_code_verifier to Redis by key as csrf_state
    let redis_service = &app_data.redis_service.lock().unwrap();
    if let Err(e) = redis_service.set_value_with_ttl(
        csrf_state.secret().as_str(),
         pkce_code_verifier.secret().as_str(),
          config.google_redis_state_ttl_ms as usize,
        ).await {
            log::error!("REDIS SERVICE ERROR: {}", e);
            return HttpResponse::InternalServerError().body("Service unavailable")//Err(actix_web::error::ErrorInternalServerError(e));
    }

    // Redirect the user to the Google OAuth2 authorization page
    HttpResponse::Ok().body(authorize_url.to_string())
}