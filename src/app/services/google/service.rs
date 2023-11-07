use std::any::Any;
use std::collections::HashMap;
use std::sync::MutexGuard;

use actix_web::error::PayloadError;
use actix_web::http::Method;
use actix_web::web;
use awc::Client;
use awc::error::SendRequestError;
use cs_shared_lib::redis as Redis;
use jsonwebtoken as jwt;
use jwt::{decode, Validation, DecodingKey, Algorithm, TokenData};
use oauth2::http::{HeaderMap, request};
use redis::Connection as RedisConnection;

use oauth2::url::Url;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl, AuthorizationCode, PkceCodeVerifier, TokenResponse, HttpRequest,
};
use oauth2::reqwest::{async_http_client, http_client};
use serde::{Serialize, Deserialize};
use serde_json::{Map, Value};

use crate::app::app_error::AppError;
use crate::app::services::user::user::GoogleProfile;
use crate::config::google_config::GoogleConfig;

use super::error::GoogleServiceError;


/**
 * "azp": "208797228814-b81ki7a2fnjepfaph64isme6i26oomgv.apps.googleusercontent.com",
    "aud": "208797228814-b81ki7a2fnjepfaph64isme6i26oomgv.apps.googleusercontent.com",
    "sub": "109265904531099102897",
    "scope": "openid",
    "exp": "1699298566",
    "expires_in": "3555",
    "access_type": "offline"
 */
#[derive(Debug, serde::Deserialize, Serialize)]
struct TokenClaims {
  azp: String,
  aud: String,
  sub: String,
  scope: String,
  exp: String,
  expires_in: String,
  access_type: String,
}
pub struct GoogleService {
    oauth2_client: BasicClient,
    config: GoogleConfig,
    google_oauth2_decoding_key: DecodingKey,
}

impl GoogleService {
    pub async fn new(config: GoogleConfig) -> Result<Self, GoogleServiceError> {
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
      let google_oauth2_decoding_key: DecodingKey = get_google_oauth2_sert(&config.google_cert_url).await?;

      Ok(GoogleService {
        oauth2_client: client,
        config,
        google_oauth2_decoding_key,
      })
    }

    pub fn get_authorization_url_data(&self) -> (Url, CsrfToken, String, u32) {
      // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
      // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
      let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
      let (authorize_url, csrf_state) = self.oauth2_client
        .authorize_url(CsrfToken::new_random)
        // This is requesting access to the user's profile.
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
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
      // match self.oauth2_client
      // .exchange_code(AuthorizationCode::new(code))
      // .set_pkce_verifier(PkceCodeVerifier::new(pkce_code_verifier))
      // .request_async(async_http_client).await {
      //   Ok(token_response) => {
      //     let access_token = token_response.access_token().secret();
      //     if let Some(refresh_token) = token_response.refresh_token() {
      //       return Ok((access_token.to_owned(), refresh_token.secret().to_owned()));
      //     } else {
      //       return Err(AppError::NoRefreshTokenResponseError);
      //     }
      //   },
      //   Err(err) => {
      //     log::debug!("OAuth2 RequestTokenError: {}", err.to_string());
      //     return Err(AppError::OAuth2RequestTokenError);
      //   },
      // }

      println!("\ncode: {},\npkce_code_verifier: {}\n", code, pkce_code_verifier);
      Ok(("fake_access_token".to_string(), "fake_refresh_token".to_string()))
    }

    /**
     * tokens:
     * "ya29.a0AfB_byByPqYWjgxasfvxtuPFKvnuu6uS9Y7Tw5UIWC6kCMpt25BPTmYeqcQxI9IK_iMaOV-TfebaEfWpAQL1AL83P-SWcjYp-dMr082FwNYD4wuYv8jLVH77OKFJ-5VL2h-iK978TMjY56qDZYUVuZCivLVFg_jYuPS5aCgYKATkSARESFQGOcNnCXRzrsSi4TibZ0lyGTuxHyg0171", "1//09Ob98-h15LlVCgYIARAAGAkSNwF-L9IrKv3v6yBTLeDxasPdfzBJX308BIX8KRXjwldKFl6ABQBMHKuj6MjXP9NnH3Qrq79JfSs"
     * 
     * key:
     * Object {
     * "alg": String("RS256"),
     * "e": String("AQAB"),
     * "kid": String("f5f4bf46e52b31d9b6249f7309ad0338400680cd"),
     * "kty": String("RSA"),
     * "n": String("q5hcowR4IuPiSvHbwj9Rv9j2XRnrgbAAFYBqoLBwUV5GVIiNPKnQBYa8ZEIK2naj9gqpo3DU9lx7d7RzeVlzCS5eUA2LV94--KbT0YgIJnApj5-hyDIaevI1Sf2YQr_cntgVLvxqfW1n9ZvbQSitz5Tgh0cplZvuiWMFPu4_mh6B3ShEKIl-qi-h0cZJlRcIf0ZwkfcDOTE8bqEzWUvlCpCH9FK6Mo9YLjw5LroBcHdUbOg3Keu0uW5SCEi-2XBQgCF6xF3kliciwwnv2HhCPyTiX0paM_sT2uKspYock-IQglQ2TExoJqbYZe6CInSHiAA68fkSkJQDnuRZE7XTJQ"),
     * "use": String("sig")
     * }
     */
    
    pub async fn get_user_profile(&self, tokens: (String, String)) -> Result<GoogleProfile, GoogleServiceError> {
        // Validation configuration
        // let validation = Validation::new(Algorithm::RS256);

//         // Decode the Google access token
//         let decoding_key = get_google_oauth2_sert("https://www.googleapis.com/oauth2/v3/certs").await?;//&self.google_oauth2_decoding_key;
println!("Tokens: {:?}\n", tokens);
//         let token_data: TokenData<TokenClaims> = match decode(
//             &tokens.0.to_string(),
//             &decoding_key,
//             &validation,
//           ) {
//             Ok(data) => data,
//             Err(err) => {
//               log::error!("Decode Error: {}\n tokens: {:?}\n", err, tokens);
//               return Err(GoogleServiceError::JWTDecodingError)
//             }
//           };
//           println!("Token data: {:?}", token_data);

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}",tokens.0).parse().expect(""));
        let response = async_http_client(
          HttpRequest {
            method: Method::GET,
            url: Url::parse("https://www.googleapis.com/oauth2/v3/userinfo").expect("parse url error"),
            headers,
            body: vec![],
          }
        ).await.map_err(|err|format!("Userinfo request error: {err}"))?;
        let user_info: Value;
        if response.status_code.is_success() {
          user_info = serde_json::from_slice(&response.body)
            .expect("userinfo response body deserialize error");
        } else {
          log::error!("Error to get userinfo: {:?}", response);
          return Err(GoogleServiceError::SendRequestError)
        }

        Ok(GoogleProfile {
          user_id: user_info["sub"].as_str().unwrap().to_string(),
          name: Some(user_info["name"].as_str().unwrap().to_string()),
          email: user_info["email"].as_str().unwrap().to_string(),
        })
    }

    /// get code and state params from query string
    pub fn parse_auth_query_string(&self, query_string: &str) -> Result<(String, String), AppError> {
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

}

async fn get_google_oauth2_sert(cert_url: &str) -> Result<DecodingKey, GoogleServiceError> {
  let jwks_response = async_http_client(HttpRequest { 
    method: Method::GET,
    url: Url::parse(cert_url).expect("parse url error"),
    headers: HeaderMap::new(),
    body: vec![]
  }).await
    .expect("request Error");
  let jwks: Value = serde_json::from_str(
    &String::from_utf8(jwks_response.body)
      .expect("error to string")
    )
    .expect("deserialize error");
  let key = jwks["keys"][0].clone();
  log::debug!("key {:?}, cert_url: {}", key, cert_url);
  // Create a decoding key from the selected key
  Ok(DecodingKey::from_rsa_components(
    &key["n"].as_str().unwrap(),
    &key["e"].as_str().unwrap(),
  )?)
}