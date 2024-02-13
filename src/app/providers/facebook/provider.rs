use std::str::FromStr;

use actix_web::http::Method;

use oauth2::url::Url;
use oauth2::{HttpRequest, PkceCodeChallenge};

use crate::app::models::session_metadata::SessionMetadata;
use crate::app::models::session_tokens::SessionTokens;
use crate::app::models::token::Token;
use crate::app::models::user::FacebookProfile;
use crate::app::providers::common::{get_login_cache_data_by_state, LoginCacheData};
use crate::app::services::cache::service::RedisCacheService;
use crate::app::services::common::{get_x_www_form_headers, AsyncFn};
use crate::config::facebook_config::FacebookConfig;

use super::super::error::ProviderError;
use super::common::{
    get_session_tokens, FacebookTokenResponse, FacebookUserDeleteResponse, FacebookUserInfo,
    TokenData as FacebookTokenData, TokenDebugData, USER_PROFILE_FIELDS,
};
use super::error::FacebookProviderError;

pub struct FacebookProvider {
    async_http_request: Box<dyn AsyncFn>,
    cache_service: RedisCacheService,
    config: FacebookConfig,
}

impl FacebookProvider {
    pub fn new(
        request: Box<dyn AsyncFn>,
        config: FacebookConfig,
        cache_service: RedisCacheService,
    ) -> Self {
        FacebookProvider {
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
        let state = &pkce_code_challenge.as_str().to_string();

        let authorize_url = Url::parse_with_params(
            self.config.facebook_oauth_url.as_ref(),
            &[
                ("client_id", client_id),
                ("redirect_uri", redirect_uri),
                ("state", state),
                ("display", &String::from("popup")),
                ("response_type", &String::from("code")),
            ],
        )?;

        // set auth data to cache
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

    pub fn get_login_cache_data_by_state(
        &mut self,
        state: &str,
    ) -> Result<LoginCacheData, ProviderError> {
        // Every provider has its own cache database
        get_login_cache_data_by_state(&self.cache_service, state)
    }

    pub async fn get_tokens(&mut self, code: &str) -> Result<SessionTokens, ProviderError> {
        // Exchange the code with a token.

        let mut url = Url::from_str(&self.config.facebook_token_url)?;
        let query_params: Vec<(&str, &str)> = vec![
            ("code", code),
            ("client_id", &self.config.facebook_client_id),
            ("client_secret", &self.config.facebook_client_secret),
            ("redirect_uri", &self.config.facebook_redirect_url),
        ];
        let qs: Vec<String> = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        let qss = qs.join("&");
        url.set_query(Some(&qss));
        let response = self
            .async_http_request
            .handle(HttpRequest {
                method: Method::GET,
                url,
                headers: get_x_www_form_headers(),
                body: vec![],
            })
            .await
            .map_err(|err| format!("Get token request error: {err}"))?;

        if !response.status_code.is_success() {
            return Err(ProviderError::OAuth2RequestTokenError);
        }
        let response = serde_json::from_slice::<FacebookTokenResponse>(&response.body)?;
        let tokens = get_session_tokens(response);
        Ok(tokens)
    }

    pub async fn get_user_profile(
        &mut self,
        tokens: &SessionTokens,
    ) -> Result<FacebookProfile, ProviderError> {
        let access_token = match tokens.access_token.clone() {
            Some(token) => token,
            None => {
                return Err(ProviderError::BadTokenStructure);
            }
        };

        let user_token_data = self.get_user_token_data(&access_token).await?;

        let user_profile_data = self.get_user_profile_data(&user_token_data.user_id).await?;

        let mut profile = FacebookProfile {
            user_id: user_profile_data.id,
            name: user_profile_data.name,
            email: user_profile_data.email,
            picture: None,
        };

        if let Some(profile_picture_data) = user_profile_data.picture {
            profile.picture = match profile_picture_data.data {
                Some(picture_data) => Some(picture_data.url),
                None => None,
            }
        }

        Ok(profile)
    }

    pub async fn get_user_token_data(
        &mut self,
        token: &Token,
    ) -> Result<FacebookTokenData, ProviderError> {
        let mut url = Url::from_str(&self.config.facebook_debug_token_url)?;
        let query_access_token = vec![
            self.config.facebook_client_id.clone(),
            self.config.facebook_client_secret.clone(),
        ]
        .join("|");

        let query_params: Vec<(&str, &str)> = vec![
            ("input_token", &token.token_string),
            ("access_token", &query_access_token),
        ];

        url.query_pairs_mut().clear().extend_pairs(query_params);

        let response = self
            .async_http_request
            .handle(HttpRequest {
                method: Method::GET,
                url,
                headers: get_x_www_form_headers(),
                body: vec![],
            })
            .await
            .map_err(|err| format!("Get token request error: {err}"))?;

        if !response.status_code.is_success() {
            return Err(ProviderError::OAuth2RequestTokenError);
        }

        let response = serde_json::from_slice::<TokenDebugData>(&response.body)?;

        match response.data {
            Some(data) => Ok(data),
            None => Err(ProviderError::TokenDataResponseError),
        }
    }

    pub async fn get_user_profile_data(
        &mut self,
        user_id: &str,
    ) -> Result<FacebookUserInfo, ProviderError> {
        let url_string = vec![&self.config.facebook_graph_url, "/", user_id].join("");
        let mut url = Url::from_str(&url_string)?;
        let query_access_token = format!(
            "{}|{}",
            &self.config.facebook_client_id, &self.config.facebook_client_secret
        );

        let query_params: Vec<(&str, &str)> = vec![
            ("fields", USER_PROFILE_FIELDS),
            ("access_token", &query_access_token),
        ];
        let qs: Vec<String> = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        let qss = qs.join("&");
        url.set_query(Some(&qss));

        let response = self
            .async_http_request
            .handle(HttpRequest {
                method: Method::GET,
                url,
                headers: get_x_www_form_headers(),
                body: vec![],
            })
            .await
            .map_err(|err| format!("Get token request error: {err}"))?;

        if !response.status_code.is_success() {
            return Err(ProviderError::OAuth2RequestTokenError);
        }
        let response = serde_json::from_slice::<FacebookUserInfo>(&response.body)?;
        Ok(response)
    }

    pub async fn logout(&mut self, facebook_user_id: &str) -> Result<(), ProviderError> {
        let url_string = vec![
            &self.config.facebook_graph_url,
            "/",
            facebook_user_id,
            "/permissions",
        ]
        .join("");
        let mut url = Url::from_str(&url_string)?;

        let query_access_token = vec![
            self.config.facebook_client_id.clone(),
            self.config.facebook_client_secret.clone(),
        ]
        .join("|");

        let query_params: Vec<(&str, &str)> = vec![("access_token", &query_access_token)];

        url.query_pairs_mut().clear().extend_pairs(query_params);

        let response = self
            .async_http_request
            .handle(HttpRequest {
                method: Method::DELETE,
                url,
                headers: get_x_www_form_headers(),
                body: vec![],
            })
            .await
            .map_err(|err| format!("Get token request error: {err}"))?;

        if !response.status_code.is_success() {
            return Err(ProviderError::FacebookProviderError(
                FacebookProviderError::DeletePermissionsRequestError,
            ));
        }

        let response = serde_json::from_slice::<FacebookUserDeleteResponse>(&response.body)?;

        match response.success.is_some() {
            true => Ok(()),
            _ => Err(ProviderError::FacebookProviderError(
                FacebookProviderError::DeletePermissionsRequestError,
            )),
        }
    }
}
