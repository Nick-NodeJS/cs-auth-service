use std::any::Any;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::MutexGuard;

use actix_web::error::PayloadError;
use actix_web::http::{Method, StatusCode};
use actix_web::web::{self, Bytes, Form};
use awc::error::{HeaderValue, SendRequestError};
use awc::Client;
use base64::Engine;
use cs_shared_lib::redis as Redis;
use jsonwebtoken as jwt;
use jwt::{decode, Algorithm, DecodingKey, Header, TokenData, Validation};
use oauth2::http::status::InvalidStatusCode;
use oauth2::http::{request, HeaderMap};
use redis::Connection as RedisConnection;

use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::url::Url;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, HttpRequest, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, RevocationUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::app::services::cache::service::CacheService;
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

#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleTokens {
    pub access_token: String,
    pub id_token: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenClaims {
    iss: String,
    azp: String,
    pub aud: String,
    sub: String,
    email: String,
    email_verified: bool,
    at_hash: String,
    name: String,
    picture: String,
    given_name: String,
    family_name: String,
    locale: String,
    iat: u32,
    exp: u32,
}
pub struct GoogleService {
    oauth2_client: BasicClient,
    cache_service: CacheService,
    config: GoogleConfig,
    google_oauth2_decoding_key: DecodingKey,
}

impl GoogleService {
    pub async fn new(
        config: GoogleConfig,
        cache_service: CacheService,
    ) -> Result<Self, GoogleServiceError> {
        let google_client_id = ClientId::new(config.google_client_id.to_string());
        let google_client_secret = ClientSecret::new(config.google_client_secret.to_string());
        let oauth_url = AuthUrl::new(config.google_oauth_url.to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url =
            TokenUrl::new(config.google_token_url.to_string()).expect("Invalid token endpoint URL");

        // Set up the config for the Google OAuth2 process.
        let client = BasicClient::new(
            google_client_id,
            Some(google_client_secret),
            oauth_url,
            Some(token_url),
        )
        .set_redirect_uri(
            RedirectUrl::new(config.google_redirect_url.to_string()).expect("Invalid redirect URL"),
        )
        // Google supports OAuth 2.0 Token Revocation (RFC-7009)
        .set_revocation_uri(
            RevocationUrl::new(config.google_revoke_url.to_string())
                .expect("Invalid revocation endpoint URL"),
        );
        let google_oauth2_decoding_key: DecodingKey =
            get_google_oauth2_sert(&config.google_cert_url).await?;

        Ok(GoogleService {
            oauth2_client: client,
            cache_service,
            config,
            google_oauth2_decoding_key,
        })
    }

    pub fn get_authorization_url_data(&self) -> (Url, CsrfToken, String) {
        // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
        // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
        let (authorize_url, csrf_state) = self
            .oauth2_client
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
        );
    }

    pub fn set_auth_data_to_cache(
        &mut self,
        csrf_state: &str,
        pkce_code_verifier: &str,
    ) -> Result<(), GoogleServiceError> {
        // get redis service and set auth data in cache
        // let mut cache_service = app_data.cache_service.lock()?;
        self.cache_service.set_value_with_ttl(
            csrf_state,
            &pkce_code_verifier,
            self.config.google_redis_state_ttl_ms as usize,
        )?;
        Ok(())
    }

    pub fn get_pkce_code_verifier(&mut self, state: &str) -> Result<String, GoogleServiceError> {
        // process code and state
        let try_code: Option<String> = self.cache_service.get_value(state.clone().as_ref())?;
        if let Some(pkce_code_verifier_from_cache) = try_code {
            Ok(pkce_code_verifier_from_cache)
        } else {
            log::debug!("No callback request state {} in Redis", state);
            return Err(GoogleServiceError::CallbackStateCacheError);
        }
    }

    /// It makes http request to GAPI and gets access token and refresh token.
    /// If user was registered during the test flow, you have to go to
    /// https://myaccount.google.com/connections?hl=en
    /// and delete all connection with this app to be able to receive refresh token,
    /// in another way it always returns access token only (tested 03.11.2023)
    pub async fn get_tokens(
        &mut self,
        code: String,
        state: String,
    ) -> Result<GoogleTokens, GoogleServiceError> {
        // get pkce_code_verifier
        let pkce_code_verifier = self.get_pkce_code_verifier(&state)?;
        // Exchange the code with a token.

        // oauth2::BasicClient doesn't have in StandartTokenResponse "id_token"
        // that's why we use common async_http_client

        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        let url = Url::from_str(&self.config.google_token_url)?;
        log::debug!(
            "\ncode: {},\npkce_code_verifier: {}\n",
            code,
            pkce_code_verifier
        );
        let params: Vec<(&str, &str)> = vec![
            ("code", &code),
            ("client_id", &self.config.google_client_id),
            ("client_secret", &self.config.google_client_secret),
            ("redirect_uri", &self.config.google_redirect_url),
            ("grant_type", "authorization_code"),
            ("code_verifier", &pkce_code_verifier),
        ];
        let response = async_http_client(HttpRequest {
            method: Method::POST,
            url,
            headers,
            body: url::form_urlencoded::Serializer::new(String::new())
                .extend_pairs(params)
                .finish()
                .into_bytes(),
        })
        .await
        .map_err(|err| format!("Get token request error: {err}"))?;

        if !response.status_code.is_success() {
            log::error!(
                "Tokens request error, response body: {:?}",
                String::from_utf8_lossy(&response.body)
            );
            return Err(GoogleServiceError::OAuth2RequestTokenError);
        }
        let result = serde_json::from_slice::<GoogleTokens>(&response.body)?;
        log::debug!("\nGoogle token response: {:?}\n", result);
        // TODO: reimplement json body parsing more efficient to get strings without extra symbols(")
        Ok(result)
    }

    pub async fn get_user_profile(&self, token: &str) -> Result<GoogleProfile, GoogleServiceError> {
        let token_data = decode_token(token, &self.google_oauth2_decoding_key, true)?;
        println!("Token data: {:?}", token_data);

        Ok(GoogleProfile {
            user_id: token_data.sub,
            name: Some(token_data.name),
            email: token_data.email,
        })
    }

    /// get code and state params from query string
    pub fn parse_auth_query_string(
        &self,
        query_string: &str,
    ) -> Result<(String, String), GoogleServiceError> {
        let try_params = web::Query::<HashMap<String, String>>::from_query(query_string);
        match try_params {
            Ok(params) => {
                let code: String;
                if let Some(code_string) = params.get("code") {
                    code = code_string.to_owned();
                } else {
                    return Err(GoogleServiceError::CodeParamError);
                };
                let state: String;
                if let Some(state_string) = params.get("state") {
                    state = state_string.to_owned();
                } else {
                    return Err(GoogleServiceError::CodeParamError);
                };
                return Ok((code, state));
            }
            Err(err) => {
                log::error!("{}", err.to_string());
                return Err(GoogleServiceError::QueryStringError);
            }
        }
    }

    pub async fn revoke_token(&self, token: String) -> Result<(), GoogleServiceError> {
        // TODO: implement Google API token revokation
        Ok(())
    }
}

pub fn decode_token(
    token: &str,
    key: &DecodingKey,
    check_expiration: bool,
) -> Result<TokenClaims, GoogleServiceError> {
    // Validation configuration
    let mut validation = Validation::new(Algorithm::RS256);
    if !check_expiration {
        validation.validate_exp = false;
    }

    let token_data: TokenData<TokenClaims> = match decode(token, key, &validation) {
        Ok(data) => data,
        Err(err) => {
            log::error!("Decode Error: {}\n token: {}\n", err, token);
            return Err(GoogleServiceError::JWTDecodingError);
        }
    };

    Ok(token_data.claims)
}

async fn get_google_oauth2_sert(cert_url: &str) -> Result<DecodingKey, GoogleServiceError> {
    let jwks_response = async_http_client(HttpRequest {
        method: Method::GET,
        url: Url::parse(cert_url).expect("parse url error"),
        headers: HeaderMap::new(),
        body: vec![],
    })
    .await
    .expect("request Error");
    let jwks: Value =
        serde_json::from_str(&String::from_utf8(jwks_response.body).expect("error to string"))
            .expect("deserialize error");
    let key = jwks["keys"][0].clone();
    log::debug!("key {:?}, cert_url: {}", key, cert_url);
    // Create a decoding key from the selected key
    Ok(DecodingKey::from_rsa_components(
        &key["n"].as_str().unwrap(),
        &key["e"].as_str().unwrap(),
    )?)
}
