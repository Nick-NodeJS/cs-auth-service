use actix_web::error::PayloadError;
use awc::Client;
use awc::error::SendRequestError;
use jsonwebtoken as jwt;
use jwt::{decode, Validation, DecodingKey, Algorithm, TokenData};

use oauth2::url::Url;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl, AuthorizationCode, PkceCodeVerifier, TokenResponse,
};
use oauth2::reqwest::http_client;
use serde::{Serialize, Deserialize};

use crate::config::google_config::GoogleConfig;

pub struct GoogleService {
    client: BasicClient,
    config: GoogleConfig,
    google_oauth2_decoding_key: Option<DecodingKey>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // Add the fields you need from the token here
    iss: String,
    sub: String,
    aud: String,
    exp: usize,
    iat: usize,
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
      let google_oauth2_decoding_key: Option<DecodingKey> = None;

      GoogleService { client, config, google_oauth2_decoding_key }
    }

    pub async fn init(&mut self) -> Result<(), String> {
      return Ok(());
      /*match get_google_oauth2_sert(&self.config.google_cert_url).await {
        Ok(google_oauth2_decoding_key) => {
          self.google_oauth2_decoding_key = Some(google_oauth2_decoding_key);
          return Ok(())
        },
        Err(err) => {
          return Err(format!("Error to get Google OAuth2 sertificate: {}", err));
        }
      }*/
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

    pub fn get_access_token_user_data(&self, access_token: &str) -> Result<TokenData<Claims>, String> {
        // Validation configuration
        let validation = Validation::new(Algorithm::RS256);

        // Decode the Google access token
        let decoding_key = match &self.google_oauth2_decoding_key {
          Some(d_key) => d_key,
          None => return Err("No decoding key on Google Service!".to_string()),
        };

        let token_data = match decode(
            access_token,
            decoding_key,
            &validation,
        ) {
          Ok(t_data) => t_data,
          Err(err) => return Err(err.to_string()),
        };

        Ok(token_data)
    }
}

async fn get_google_oauth2_sert(url: &str) -> Result<DecodingKey, String> {
  // let headers = HeaderMap::new();
  // let response = request::get(url, headers).await?.json::<serde_json::Value>()?;
  let client = Client::new();
  let mut jwks_response = match client.get(url).send().await {
    Ok(jwks_response_try) => jwks_response_try,
    Err(err) => return Err(err.to_string()),
  };
  let jwks = match jwks_response.json::<serde_json::Value>().await {
    Ok(jwk_try) => jwk_try,
    Err(err) => return Err(err.to_string()),
  };
  let key = jwks["keys"][0].clone();
  println!("key {:?}, url: {}", key, url);
  // Create a decoding key from the selected key
  return match DecodingKey::from_rsa_components(
    &key["n"].as_str().unwrap(),
    &key["e"].as_str().unwrap(),
  ) {
    Ok(decoding_key) => Ok(decoding_key),
    Err(err) => Err(err.to_string()),
  };
}