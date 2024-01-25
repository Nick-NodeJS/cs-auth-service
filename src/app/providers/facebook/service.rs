use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::str::FromStr;

use actix_web::http::Method;
use actix_web::web;
use awc::error::HeaderValue;
use base64::Engine;
use jsonwebtoken as jwt;
use jwt::{decode, Algorithm, DecodingKey, TokenData, Validation};
use oauth2::http::HeaderMap;

use oauth2::basic::BasicClient;
use oauth2::url::Url;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, HttpRequest, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl,
};

use crate::app::models::session_metadata::SessionMetadata;
use crate::app::models::session_tokens::SessionTokens;
use crate::app::models::token::Token;
use crate::app::models::user::GoogleProfile;
use crate::app::services::cache::service::RedisCacheService;
use crate::app::services::common::{
    async_http_request, get_x_www_form_headers, AsyncFn, LoginCacheData,
};
use crate::config::facebook_config::FacebookConfig;
use crate::config::google_config::GoogleConfig;

use super::super::error::ProviderError;
// use super::common::{
//     GoogleCert, GoogleKeys, GoogleTokenResponse, TokenClaims, TokenHeaderObject, UserInfo,
// };
use super::error::FacebookServiceError;

pub struct FacebookService {
    async_http_request: Box<dyn AsyncFn>,
    cache_service: RedisCacheService,
    config: FacebookConfig,
}

impl FacebookService {
    pub fn new(
        request: Box<dyn AsyncFn>,
        config: FacebookConfig,
        cache_service: RedisCacheService,
    ) -> Self {
        FacebookService {
            async_http_request: request,
            cache_service,
            config,
        }
    }

    pub fn get_authorization_url_data(
        &mut self,
        session_metadata: SessionMetadata,
    ) -> Result<String, ProviderError> {
        // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        let client_id = &self.config.facebook_client_id;
        //let facebook_client_secret = self.config.facebook_client_secret;
        let redirect_uri = &self.config.facebook_redirect_url;
        let state = &pkce_code_verifier.secret().to_string();

        let authorize_url = Url::parse_with_params(
            self.config.facebook_oauth_url.as_ref(),
            &[
                ("client_id", client_id),
                ("redirect_uri", redirect_uri),
                ("state", state),
            ],
        )?;

        // set auth data in cache
        let login_cache_data = LoginCacheData {
            pkce_code_verifier: pkce_code_verifier.secret().to_string(),
            session_metadata,
        };
        self.set_auth_data_to_cache(pkce_code_challenge.as_str(), &login_cache_data)?;

        Ok(authorize_url.to_string())
    }

    pub fn set_auth_data_to_cache(
        &mut self,
        csrf_state: &str,
        login_cache_data: &LoginCacheData,
    ) -> Result<(), ProviderError> {
        self.cache_service.set_value_with_ttl::<String>(
            csrf_state,
            RedisCacheService::struct_to_cache_string(login_cache_data)?,
            self.config.facebook_cache_state_ttl_sec,
        )?;
        Ok(())
    }
}
