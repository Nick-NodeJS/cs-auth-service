use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use derivative::Derivative;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::str::FromStr;

use actix_web::http::Method;
use actix_web::web;
use awc::error::HeaderValue;
use base64::Engine;
use jsonwebtoken as jwt;
use jwt::{decode, Algorithm, DecodingKey, TokenData, Validation};
use oauth2::http::HeaderMap;

use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::url::Url;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, HttpRequest, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl,
};

use crate::app::models::user::GoogleProfile;
use crate::app::services::cache::service::{CacheService, CacheServiceType};
use crate::config::google_config::GoogleConfig;

use super::error::GoogleServiceError;
use super::structures::{GoogleCert, GoogleKeys, GoogleTokens, TokenClaims, TokenHeaderObject};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct GoogleService {
    oauth2_client: BasicClient,
    cache_service: CacheService,
    config: GoogleConfig,
    // #[derivative(Debug = "ignore")]
    // google_oauth2_decoding_keys: HashMap<String, (GoogleCert, DecodingKey)>,
}

impl GoogleService {
    pub async fn new() -> Result<Self, GoogleServiceError> {
        let cache_service = CacheService::new(CacheServiceType::Google)?;
        let config = GoogleConfig::new();
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

        Ok(GoogleService {
            oauth2_client: client,
            cache_service,
            config,
        })
    }

    pub async fn init(&mut self) -> Result<(), GoogleServiceError> {
        let (google_certs, expires) =
            get_google_oauth2_certificates(&self.config.google_cert_url).await?;

        self.set_certificates_to_cache(google_certs, expires)
            .await?;
        Ok(())
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
        self.cache_service.set_value_with_ttl(
            csrf_state,
            &pkce_code_verifier,
            self.config.google_cache_state_ttl_sec as usize,
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

    /*
    TODO: reimplement certificates logic.
    - they should be received in initial step and stored in cache with expiration which is in header response
    - in case no certificates in cache update them by getting again request to GAPI
    - in case fail on this step or in case the token is wrong, try to use GAPI /v3/token endpoint to get user profile
     */
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
        let decoding_key = match self.get_certificates_from_cache().await? {
            Some(certs) => get_decoding_key_from_vec_cert(certs, header_object.kid.clone())?,
            None => {
                log::debug!("No certificates found in cache. Trying to refresh them on GAPI...");
                let (google_certs, expires) =
                    get_google_oauth2_certificates(&self.config.google_cert_url).await?;

                self.set_certificates_to_cache(google_certs.clone(), expires)
                    .await?;
                get_decoding_key_from_vec_cert(google_certs, header_object.kid)?
            }
        };
        Ok(decoding_key)
    }

    pub async fn get_user_profile(
        &mut self,
        token: &str,
    ) -> Result<GoogleProfile, GoogleServiceError> {
        let key = match self.get_token_key(token.clone()).await? {
            Some(decoding_key) => decoding_key,
            None => {
                log::warn!(
                    "No decoding key for token: {}\n Trying to get user profile on GAPI...",
                    token
                );
                // TODO: implement http request to https://oauth2.googleapis.com/token to get user profile
                return Ok(GoogleProfile {
                    user_id: "fake_google_id".to_string(),
                    name: "fake_name".to_string(),
                    email: "fake_email".to_string(),
                    email_verified: false,
                    picture: "fake_picture".to_string(),
                });
            }
        };
        let token_data = decode_token(token, &key, true)?;
        log::debug!("\nToken data: {:?}\n", token_data);

        Ok(GoogleProfile {
            user_id: token_data.sub,
            name: token_data.name,
            email: token_data.email,
            email_verified: token_data.email_verified,
            picture: token_data.picture,
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
        // TODO: implement Google API token revoketion
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
            &serde_json::to_string(&certificates)?,
            expired_duration as usize,
        )?;
        Ok(())
    }

    async fn get_certificates_from_cache(
        &mut self,
    ) -> Result<Option<Vec<GoogleCert>>, GoogleServiceError> {
        let try_certs = self
            .cache_service
            .get_value(&self.config.google_cache_certs_key)?;
        match try_certs {
            Some(certs_string) => {
                let certs = serde_json::from_str::<Vec<GoogleCert>>(&certs_string)?;
                Ok(Some(certs))
            }
            None => Ok(None),
        }
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

async fn get_google_oauth2_certificates(
    cert_url: &str,
) -> Result<(Vec<GoogleCert>, DateTime<Utc>), GoogleServiceError> {
    let jwks_response = async_http_client(HttpRequest {
        method: Method::GET,
        url: Url::parse(cert_url).expect("parse url error"),
        headers: HeaderMap::new(),
        body: vec![],
    })
    .await
    .expect("request Error");

    let cert_expires = match jwks_response.headers.get("expires") {
        Some(value) => value.to_str()?,
        None => return Err(GoogleServiceError::OAuth2CertificatesResponse),
    };
    let cert_expires_datetime: DateTime<Utc> = DateTime::parse_from_rfc2822(cert_expires)?.into();

    let jwt_keys = serde_json::from_slice::<GoogleKeys>(&jwks_response.body)?;
    // let mut keys: HashMap<String, GoogleCert> = HashMap::new();
    // jwt_keys.keys.into_iter().for_each(|cert| {
    //     let key = DecodingKey::from_rsa_components(&cert.n, &cert.e);
    //     match key {
    //         Ok(k) => {
    //             keys.insert(cert.kid.clone(), (cert, k));
    //             return ();
    //         }
    //         Err(err) => log::error!(
    //             "Error to create Decoding key for cert: {:?}, error: {}",
    //             cert,
    //             err
    //         ),
    //     };
    // });
    Ok((jwt_keys.keys, cert_expires_datetime))
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
