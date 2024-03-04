use chrono::{DateTime, Utc};
use std::str::FromStr;

use actix_web::http::Method;
use awc::error::HeaderValue;
use oauth2::http::HeaderMap;

use oauth2::basic::BasicClient;
use oauth2::url::Url;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, HttpRequest, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl,
};

use crate::app::models::session_tokens::SessionTokens;
use crate::app::models::token::Token;
use crate::app::models::user_profile::GoogleProfile;
use crate::app::providers::common::{get_login_cache_data_by_state, LoginCacheData};
use crate::app::providers::google::common::TokenClaims;
use crate::app::services::cache::service::RedisCacheService;
use crate::app::services::common::{async_http_request, get_x_www_form_headers, AsyncFn};
use crate::app::shared::jwt::decode_token;
use crate::config::google_config::GoogleConfig;

use super::super::error::ProviderError;
use super::common::{
    get_decoding_key_from_vec_cert, get_session_tokens, GoogleCert, GoogleKeys,
    GoogleTokenResponse, UserInfo,
};
use super::error::GoogleProviderError;

pub struct GoogleProvider {
    async_http_request: Box<dyn AsyncFn>,
    cache_service: RedisCacheService,
    config: GoogleConfig,
}

impl GoogleProvider {
    pub fn new(
        request: Box<dyn AsyncFn>,
        config: GoogleConfig,
        cache_service: RedisCacheService,
    ) -> Self {
        GoogleProvider {
            async_http_request: request,
            cache_service,
            config,
        }
    }

    pub async fn init(&mut self) -> Result<(), ProviderError> {
        // check Google certificates
        let _ = self.get_certificates().await?;
        Ok(())
    }

    pub async fn get_certificates(&mut self) -> Result<Vec<GoogleCert>, ProviderError> {
        // get Google certificates in case we do not have them in cache only
        if let Some(certificates_from_cache) = self.get_certificates_from_cache().await? {
            return Ok(certificates_from_cache);
        }
        let (google_certs, expires) = self.get_google_oauth2_certificates().await?;

        self.set_certificates_to_cache(google_certs.clone(), expires)
            .await?;

        Ok(google_certs)
    }

    pub fn get_authorization_url_data(&self) -> (Url, CsrfToken, String) {
        // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
        // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        let google_client_id = ClientId::new(self.config.google_client_id.to_string());
        let google_client_secret = ClientSecret::new(self.config.google_client_secret.to_string());
        let oauth_url = AuthUrl::new(self.config.google_oauth_url.to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new(self.config.google_token_url.to_string())
            .expect("Invalid token endpoint URL");

        // Set up the config for the Google OAuth2 process.
        let oauth2_client = BasicClient::new(
            google_client_id,
            Some(google_client_secret),
            oauth_url,
            Some(token_url),
        )
        .set_redirect_uri(
            RedirectUrl::new(self.config.google_redirect_url.to_string())
                .expect("Invalid redirect URL"),
        )
        // Google supports OAuth 2.0 Token Revocation (RFC-7009)
        .set_revocation_uri(
            RevocationUrl::new(self.config.google_revoke_url.to_string())
                .expect("Invalid revocation endpoint URL"),
        );

        let (authorize_url, csrf_state) = oauth2_client
            .authorize_url(CsrfToken::new_random)
            // This is requesting access to the user's profile.
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .add_extra_param("access_type", "offline")
            .set_pkce_challenge(pkce_code_challenge)
            .url();

        (
            authorize_url,
            csrf_state,
            pkce_code_verifier.secret().to_string(),
        )
    }

    pub fn set_auth_data_to_cache(
        &mut self,
        csrf_state: &str,
        login_cache_data: &LoginCacheData,
    ) -> Result<(), ProviderError> {
        self.cache_service.set_value_with_ttl::<String>(
            csrf_state,
            RedisCacheService::struct_to_cache_string(login_cache_data)?,
            self.config.google_cache_state_ttl_sec,
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

    /// It makes http request to GAPI and gets access token and refresh token.
    /// If user was registered during the test flow, you have to go to
    /// https://myaccount.google.com/connections?hl=en
    /// and delete all connection with this app to be able to receive refresh token,
    /// in another way it always returns access token only (tested 03.11.2023)
    pub async fn get_tokens(
        &mut self,
        code: String,
        pkce_code_verifier: String,
    ) -> Result<SessionTokens, ProviderError> {
        // Exchange the code with a token.

        // oauth2::BasicClient doesn't have in StandartTokenResponse "id_token"
        // that's why we use common async_http_client

        let url = Url::from_str(&self.config.google_token_url)?;
        let params: Vec<(&str, &str)> = vec![
            ("code", &code),
            ("client_id", &self.config.google_client_id),
            ("client_secret", &self.config.google_client_secret),
            ("redirect_uri", &self.config.google_redirect_url),
            ("grant_type", "authorization_code"),
            ("code_verifier", &pkce_code_verifier),
        ];
        let response = async_http_request(HttpRequest {
            method: Method::POST,
            url,
            headers: get_x_www_form_headers(),
            body: url::form_urlencoded::Serializer::new(String::new())
                .extend_pairs(params)
                .finish()
                .into_bytes(),
        })
        .await
        .map_err(|err| format!("Get token request error: {err}"))?;

        if !response.status_code.is_success() {
            return Err(ProviderError::OAuth2RequestTokenError);
        }
        let response = serde_json::from_slice::<GoogleTokenResponse>(&response.body)?;
        let tokens = get_session_tokens(response);
        Ok(tokens)
    }

    pub async fn get_user_profile(
        &mut self,
        token: Option<Token>,
    ) -> Result<GoogleProfile, ProviderError> {
        let access_token = match token {
            Some(token) => token.token_string,
            None => return Err(ProviderError::BadTokenStructure),
        };

        // Get DecodingKey
        let header = jsonwebtoken::decode_header(&access_token)?;
        let kid = match header.kid {
            Some(k) => k,
            None => {
                log::warn!("Bad token, no header found. Token: {}", &access_token);
                return Err(ProviderError::BadTokenStructure);
            }
        };
        let google_certs = self.get_certificates().await?;
        let decoding_key = get_decoding_key_from_vec_cert(google_certs, kid)?;

        // Get Google user profile
        let key = match decoding_key {
            Some(d_key) => d_key,
            None => {
                log::warn!(
                    "No decoding key for token: {}\n Trying to get user profile on GAPI...",
                    &access_token
                );
                return self.get_user_profile_on_gapi(&access_token).await;
            }
        };
        let token_data = decode_token::<TokenClaims>(&access_token, &key, true)?;
        log::debug!("\nToken data: {:?}\n", token_data);

        Ok(GoogleProfile {
            user_id: token_data.sub,
            name: token_data.name,
            email: token_data.email,
            email_verified: token_data.email_verified,
            picture: Some(token_data.picture),
        })
    }

    pub async fn get_user_profile_on_gapi(
        &self,
        token: &str,
    ) -> Result<GoogleProfile, ProviderError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(format!("Bearer {}", token).as_ref())?,
        );
        let user_info_response = match async_http_request(HttpRequest {
            method: Method::GET,
            url: Url::parse(&self.config.google_userinfo_url).expect("parse url error"),
            headers,
            body: vec![],
        })
        .await
        {
            Ok(response) => response,
            Err(_) => {
                return Err(ProviderError::TokenDataResponseError);
            }
        };
        let user_info = serde_json::from_slice::<UserInfo>(&user_info_response.body)?;
        return Ok(GoogleProfile {
            user_id: user_info.sub,
            name: user_info.name,
            email: user_info.email,
            email_verified: user_info.email_verified,
            picture: Some(user_info.picture),
        });
    }

    pub async fn logout(&self, tokens: SessionTokens) -> Result<(), ProviderError> {
        if let Some(token) = tokens.refresh_token {
            self.revoke_token(token.token_string.as_ref()).await
        } else {
            log::error!("Google logout error. No refresh token");
            Err(ProviderError::BadTokenStructure)
        }
    }

    pub async fn revoke_token(&self, token: &str) -> Result<(), ProviderError> {
        let mut url = Url::parse(&self.config.google_revoke_url).expect("parse url error");
        url.set_query(Some(format!("token={}", token).as_ref()));
        let revoke_response = match async_http_request(HttpRequest {
            method: Method::POST,
            url,
            headers: get_x_www_form_headers(),
            body: vec![],
        })
        .await
        {
            Ok(response) => response,
            Err(err) => {
                log::error!("Google revoke token request error: {}", err);
                return Err(ProviderError::GoogleProviderError(
                    GoogleProviderError::RevokeRequestError,
                ));
            }
        };
        if !revoke_response.status_code.is_success() {
            return Err(ProviderError::GoogleProviderError(
                GoogleProviderError::RevokeRequestError,
            ));
        }
        log::debug!("Revoked Google token {} successfuly", token);
        Ok(())
    }

    async fn set_certificates_to_cache(
        &mut self,
        certificates: Vec<GoogleCert>,
        expires: DateTime<Utc>,
    ) -> Result<(), ProviderError> {
        let now = Utc::now();
        let expired_duration = expires.signed_duration_since(now).num_seconds();
        self.cache_service.set_value_with_ttl(
            &self.config.google_cache_certs_key,
            serde_json::to_string(&certificates)?,
            expired_duration as u64,
        )?;
        Ok(())
    }

    async fn get_certificates_from_cache(
        &mut self,
    ) -> Result<Option<Vec<GoogleCert>>, ProviderError> {
        let try_certs = self
            .cache_service
            .get_value::<String>(&self.config.google_cache_certs_key)?;
        match try_certs {
            Some(certs_string) => {
                let certs = serde_json::from_str::<Vec<GoogleCert>>(&certs_string)?;
                Ok(Some(certs))
            }
            None => Ok(None),
        }
    }

    async fn get_google_oauth2_certificates(
        &mut self,
    ) -> Result<(Vec<GoogleCert>, DateTime<Utc>), ProviderError> {
        let jwks_response = match self
            .async_http_request
            .handle(HttpRequest {
                method: Method::GET,
                url: Url::parse(self.config.google_cert_url.clone().as_ref())
                    .expect("parse url error"),
                headers: HeaderMap::new(),
                body: vec![],
            })
            .await
        {
            Ok(response) => response,
            Err(_) => {
                return Err(ProviderError::GoogleProviderError(
                    GoogleProviderError::OAuth2CertificatesResponse,
                ));
            }
        };

        let cert_expires = match jwks_response.headers.get("expires") {
            Some(value) => value.to_str()?,
            None => {
                return Err(ProviderError::GoogleProviderError(
                    GoogleProviderError::OAuth2CertificatesResponse,
                ))
            }
        };
        let cert_expires_datetime: DateTime<Utc> =
            DateTime::parse_from_rfc2822(cert_expires)?.into();

        let jwt_keys = serde_json::from_slice::<GoogleKeys>(&jwks_response.body)?;

        Ok((jwt_keys.keys, cert_expires_datetime))
    }
}
