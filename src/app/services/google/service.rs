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

use crate::app::models::session_tokens::SessionTokens;
use crate::app::models::token::Token;
use crate::app::models::user::GoogleProfile;
use crate::app::services::cache::service::RedisCacheService;
use crate::app::services::common::{async_http_request, get_x_www_form_headers, AsyncFn};
use crate::app::services::google::common::GoogleTokenResponse;
use crate::config::google_config::GoogleConfig;

use super::common::{
    GoogleCert, GoogleKeys, LoginCacheData, TokenClaims, TokenHeaderObject, UserInfo,
};
use super::error::GoogleServiceError;

pub struct GoogleService {
    async_http_request: Box<dyn AsyncFn>,
    cache_service: RedisCacheService,
    config: GoogleConfig,
}

impl GoogleService {
    pub fn new(
        request: Box<dyn AsyncFn>,
        config: GoogleConfig,
        cache_service: RedisCacheService,
    ) -> Self {
        GoogleService {
            async_http_request: request,
            cache_service,
            config,
        }
    }

    pub async fn init(&mut self) -> Result<(), GoogleServiceError> {
        // check Google certificates
        let _ = self.get_certificates().await?;
        Ok(())
    }

    pub async fn get_certificates(&mut self) -> Result<Vec<GoogleCert>, GoogleServiceError> {
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
    ) -> Result<(), GoogleServiceError> {
        self.cache_service.set_value_with_ttl::<String>(
            csrf_state,
            RedisCacheService::struct_to_cache_string(login_cache_data)?,
            self.config.google_cache_state_ttl_sec,
        )?;
        Ok(())
    }

    pub fn get_pkce_code_verifier(
        &mut self,
        state: &str,
    ) -> Result<LoginCacheData, GoogleServiceError> {
        // process code and state
        let login_cache_data_value = self
            .cache_service
            .get_value::<LoginCacheData>(state.clone().as_ref())?;
        if let Some(login_cache_data) = login_cache_data_value {
            Ok(login_cache_data)
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
        pkce_code_verifier: String,
    ) -> Result<SessionTokens, GoogleServiceError> {
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
            return Err(GoogleServiceError::OAuth2RequestTokenError);
        }
        let response = serde_json::from_slice::<GoogleTokenResponse>(&response.body)?;
        let tokens = get_session_tokens(response);
        Ok(tokens)
    }

    pub async fn get_token_key(
        &mut self,
        token: &str,
    ) -> Result<Option<DecodingKey>, GoogleServiceError> {
        let token_string = token.to_string();
        let token_parts: Vec<&str> = token_string.split('.').collect();
        let header = match token_parts.into_iter().next() {
            Some(header) => header,
            None => {
                log::warn!("Bad token, no header found. Token: {}", token_string);
                return Err(GoogleServiceError::BadTokenStructure);
            }
        };
        let decoded_slice = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(header)?;
        let header_object = serde_json::from_slice::<TokenHeaderObject>(&decoded_slice)?;
        let google_certs = self.get_certificates().await?;
        let decoding_key = get_decoding_key_from_vec_cert(google_certs, header_object.kid)?;
        Ok(decoding_key)
    }

    pub async fn get_user_profile(
        &mut self,
        token: Option<Token>,
    ) -> Result<GoogleProfile, GoogleServiceError> {
        let access_token = match token {
            Some(token) => token.token_string,
            None => return Err(GoogleServiceError::BadTokenStructure),
        };
        let key = match self.get_token_key(access_token.clone().as_ref()).await? {
            Some(decoding_key) => decoding_key,
            None => {
                log::warn!(
                    "No decoding key for token: {}\n Trying to get user profile on GAPI...",
                    access_token
                );
                return self.get_user_profile_on_gapi(&access_token).await;
            }
        };
        let token_data = decode_token(&access_token, &key, true)?;
        log::debug!("\nToken data: {:?}\n", token_data);

        Ok(GoogleProfile {
            user_id: token_data.sub,
            name: token_data.name,
            email: token_data.email,
            email_verified: token_data.email_verified,
            picture: token_data.picture,
        })
    }

    pub async fn get_user_profile_on_gapi(
        &self,
        token: &str,
    ) -> Result<GoogleProfile, GoogleServiceError> {
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
                return Err(GoogleServiceError::TokenDataResponseError);
            }
        };
        let user_info = serde_json::from_slice::<UserInfo>(&user_info_response.body)?;
        return Ok(GoogleProfile {
            user_id: user_info.sub,
            name: user_info.name,
            email: user_info.email,
            email_verified: user_info.email_verified,
            picture: user_info.picture,
        });
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

    pub async fn logout(&self, tokens: SessionTokens) -> Result<(), GoogleServiceError> {
        if let Some(token) = tokens.refresh_token {
            self.revoke_token(token.token_string.as_ref()).await
        } else {
            log::error!("Google logout error. No refresh token");
            Err(GoogleServiceError::BadTokenStructure)
        }
    }

    pub async fn revoke_token(&self, token: &str) -> Result<(), GoogleServiceError> {
        let mut url = Url::parse(&self.config.google_revoke_url).expect("parse url error");
        url.set_query(Some(format!("token={}", token.clone()).as_ref()));
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
                return Err(GoogleServiceError::RevokeRequestError);
            }
        };
        if !revoke_response.status_code.is_success() {
            return Err(GoogleServiceError::RevokeRequestError);
        }
        log::debug!("Revoked Google token {} successfuly", token);
        Ok(())
    }

    async fn set_certificates_to_cache(
        &mut self,
        certificates: Vec<GoogleCert>,
        expires: DateTime<Utc>,
    ) -> Result<(), GoogleServiceError> {
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
    ) -> Result<Option<Vec<GoogleCert>>, GoogleServiceError> {
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
    ) -> Result<(Vec<GoogleCert>, DateTime<Utc>), GoogleServiceError> {
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
                return Err(GoogleServiceError::OAuth2CertificatesResponse);
            }
        };

        let cert_expires = match jwks_response.headers.get("expires") {
            Some(value) => value.to_str()?,
            None => return Err(GoogleServiceError::OAuth2CertificatesResponse),
        };
        let cert_expires_datetime: DateTime<Utc> =
            DateTime::parse_from_rfc2822(cert_expires)?.into();

        let jwt_keys = serde_json::from_slice::<GoogleKeys>(&jwks_response.body)?;

        Ok((jwt_keys.keys, cert_expires_datetime))
    }
}

pub fn get_session_tokens(tokens: GoogleTokenResponse) -> SessionTokens {
    let expire = Some(Utc::now() + Duration::seconds(tokens.expires_in));
    let refresh_token = match tokens.refresh_token {
        Some(token) => Some(Token {
            token_string: token,
            expire: None,
        }),
        None => None,
    };
    SessionTokens {
        access_token: Some(Token {
            token_string: tokens.id_token,
            expire,
        }),
        refresh_token,
        extra_token: Some(Token {
            token_string: tokens.access_token,
            expire,
        }),
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
            log::warn!("Decode Error: {}\n token: {}\n", err, token);
            return Err(GoogleServiceError::JWTDecodingError);
        }
    };

    Ok(token_data.claims)
}

pub fn get_decoding_key_from_vec_cert(
    certs: Vec<GoogleCert>,
    kid: String,
) -> Result<Option<DecodingKey>, GoogleServiceError> {
    let cert = certs.clone().into_iter().find(|c| c.kid == kid);
    if let Some(certificate) = cert {
        let key = DecodingKey::from_rsa_components(&certificate.n, &certificate.e)?;
        Ok(Some(key.clone()))
    } else {
        log::error!(
            "No certificate found in cache for kid: {}. Certificates: {:?}",
            kid,
            &certs
        );
        Ok(None)
    }
}
