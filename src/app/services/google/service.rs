use std::any::Any;
use std::collections::HashMap;
use std::sync::MutexGuard;

use actix_web::error::PayloadError;
use actix_web::web;
use awc::Client;
use awc::error::SendRequestError;
use cs_shared_lib::redis as Redis;
use jsonwebtoken as jwt;
use jwt::{decode, Validation, DecodingKey, Algorithm, TokenData};
use redis::Connection as RedisConnection;

use oauth2::url::Url;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl, AuthorizationCode, PkceCodeVerifier, TokenResponse,
};
use oauth2::reqwest::async_http_client;
use serde::{Serialize, Deserialize};
use serde_json::{Map, Value};

use crate::app::app_error::AppError;
use crate::app::services::user::user::GoogleProfile;
use crate::config::google_config::GoogleConfig;

pub struct GoogleService {
    oauth2_client: BasicClient,
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

      GoogleService {
        oauth2_client: client,
        config,
        google_oauth2_decoding_key,
      }
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
      let (authorize_url, csrf_state) = self.oauth2_client
        .authorize_url(CsrfToken::new_random)
        // This is requesting access to the user's profile.
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/plus.me".to_string(),
        ))
        .add_extra_param("access_type", "offline")
        .set_pkce_challenge(pkce_code_challenge)
        .url();
      return (
        authorize_url,
        csrf_state,
        pkce_code_verifier.secret().to_string(),
        self.config.google_redis_state_ttl_ms,
      );
    }

    /// It makes http request to GAPI and gets access token and refresh token.
    /// If user was registered during the test flow, you have to go to
    /// https://myaccount.google.com/connections?hl=en
    /// and delete all connection with this app to be able to receive refresh token,
    /// in another way it always returns access token only and you will have NoRefreshTokenResponseError (tested 03.11.2023)
    pub async fn get_tokens(&self, code: String, pkce_code_verifier: String) -> Result<(String, String), AppError> {
      // Exchange the code with a token.
      match self.oauth2_client
      .exchange_code(AuthorizationCode::new(code))
      .set_pkce_verifier(PkceCodeVerifier::new(pkce_code_verifier))
      .request_async(async_http_client).await {
        Ok(token_response) => {
          let access_token = token_response.access_token().secret();
          if let Some(refresh_token) = token_response.refresh_token() {
            return Ok((access_token.to_owned(), refresh_token.secret().to_owned()));
          } else {
            return Err(AppError::NoRefreshTokenResponseError);
          }
        },
        Err(err) => {
          log::debug!("OAuth2 RequestTokenError: {}", err.to_string());
          return Err(AppError::OAuth2RequestTokenError);
        },
      }
    }

    pub async fn get_user_profile(&self, tokens: (String, String)) -> Result<GoogleProfile, AppError> {
        // Validation configuration
        let validation = Validation::new(Algorithm::RS256);

        // Decode the Google access token
        let decoding_key = match &self.google_oauth2_decoding_key {
          Some(d_key) => d_key,
          None => return Err(AppError::NoDecodingKeyError),
        };

        // let token_data: TokenData<GoogleProfile> = decode<GoogleProfile>(
        //     &tokens.0.to_string(),
        //     decoding_key,
        //     &validation,
        //   )?;

        Ok(GoogleProfile {
          user_id: String::from(""),
          name: Some(String::from("")),
          email: Some(String::from("")),
        })
    }

    /// get code and state params from query string
    pub fn parse_query_string(&self, query_string: &str) -> Result<(String, String), AppError> {
      let try_params = web::Query::<HashMap<String, String>>::from_query(
        query_string,
      );
      match try_params {
        Ok(params) => {
          let code: String;
          if let Some(code_string) = params.get("code") {
            code = code_string.to_owned();
          } else {
            return Err(AppError::CodeParamError)
          };
          let state: String;
          if let Some(state_string) = params.get("state") {
            state = state_string.to_owned();
          } else {
            return Err(AppError::CodeParamError)
          };
          return Ok((code, state));
        },
        Err(err) => {
          log::error!("{}", err.to_string());
          return Err(AppError::QueryStringError)
        },
      }
    }

    /// #### Actions:
    /// 1 - decode user data by tokens (GAPI request)
    /// 
    /// 2 - create a new or update user in data storage
    /// 
    /// 3 - reflect user in cache
    pub async fn set_user_to_storage(
      &self,
      tokens: &(String, String),
      //  redis_connection: MutexGuard<'_, RedisConnection>,
    ) -> Result<(), AppError> {
      println!("tokens {:?}", tokens);
      let (access_token, refresh_token) = tokens;
      // let user_data = self.get_access_token_user_data(access_token)?;
      // println!("User data {:?}", user_data);
      /* TODO:
       - update google service to get OAuth2 cert on initial step(method new)
       - decode access_token -> token data(google service)
       - create database service
       - create new user or update existing user in database
       - set or update cache with token data
       */
      return Ok(())
    }

    /// return tokens as json object
    /// #### Arguments
    /// 
    /// * `tokens` - A Tuple of strings
    /// 
    /// ```
    /// (String, String)
    /// ```
    /// 
    /// where tokens\[0\] is access_token and tokens\[1\] is refresh_token
    /// 
    /// #### Response example:
    /// ```
    ///  {
    ///   "access_token": "$access_token",
    ///   "refresh_token": "$refresh_token"
    ///   }
    /// ```
    pub fn tokens_as_json(&self, tokens: (String, String)) -> Map<String, Value> {
      let (access_token, refresh_token) = tokens;
      let mut payload = Map::new();
      payload.insert("access_token".to_string(), Value::String(access_token));
      payload.insert("refresh_token".to_string(), Value::String(refresh_token));
      return payload;
    }

    pub fn get_state_from_cache(
      &self,
      code: String,
      redis_connection: &mut MutexGuard<'_, RedisConnection>,
    ) -> Option<String> {
        match Redis::get_value(redis_connection, &code) {
          Ok(state) => {
            return state;
          },
          Err(err) => {
            log::error!("REDIS SERVICE ERROR: {}", err);
            return None;
          }
      }
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