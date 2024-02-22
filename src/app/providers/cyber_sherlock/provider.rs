use chrono::{Duration, Utc};
use oauth2::PkceCodeChallenge;
use url::Url;

use crate::{
    app::{
        models::{
            session::Session, session_tokens::SessionTokens, token::Token, user::User,
            user_profile::CyberSherlockProfile,
        },
        providers::{error::ProviderError, notification::provider::NotificationProvider},
        services::cache::service::RedisCacheService,
        shared::jwt::encode_claims,
    },
    config::app_config::AppConfig,
};

use super::{
    common::{
        hash_password, AccessTokenClaims, RefreshTokenClaims, RegisterCacheData, RegisterQueryData,
    },
    error::CyberSherlockAuthProviderError,
};

pub struct CyberSherlockAuthProvider {
    cache_service: RedisCacheService,
    config: AppConfig,
    notification_provider: NotificationProvider,
}

impl CyberSherlockAuthProvider {
    pub fn new(
        config: AppConfig,
        cache_service: RedisCacheService,
        notification_provider: NotificationProvider,
    ) -> Self {
        CyberSherlockAuthProvider {
            cache_service,
            config,
            notification_provider,
        }
    }

    pub fn get_authorization_url(
        &mut self,
        register_query_data: &RegisterQueryData,
        session: Session,
    ) -> Result<String, ProviderError> {
        // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
        let code = &pkce_code_challenge.as_str().to_string();
        let auth_url = Url::parse_with_params(
            self.config.auth_callback_url.as_ref(),
            &[("state", pkce_code_verifier.secret().to_string())],
        )?;
        // set auth data to cache
        let register_cache_data = RegisterCacheData {
            pkce_code_verifier: pkce_code_verifier.secret().to_string(),
            session,
            hash: hash_password(register_query_data.password.as_str()).map_err(|_| {
                ProviderError::CyberSherlockAuthProviderError(
                    CyberSherlockAuthProviderError::Argon2PassHashError,
                )
            })?,
            email: register_query_data.email.clone(),
            phone: register_query_data.phone.clone(),
        };
        self.set_auth_data_to_cache(code, &register_cache_data)?;

        self.send_auth_code(code, register_query_data)?;
        Ok(auth_url.to_string())
    }

    pub fn set_auth_data_to_cache(
        &mut self,
        key: &str,
        data: &RegisterCacheData,
    ) -> Result<(), ProviderError> {
        // TODO: add to app config auth code cache ttl
        let cache_ttl = 300 as u64;
        self.cache_service.set_value_with_ttl(
            key,
            RedisCacheService::struct_to_cache_string(data)?,
            cache_ttl,
        )?;
        Ok(())
    }

    pub fn send_auth_code(
        &mut self,
        code: &str,
        register_query_data: &RegisterQueryData,
    ) -> Result<(), ProviderError> {
        if let Some(email) = register_query_data.email.to_owned() {
            //TODO: implement auth email to send not only code
            self.notification_provider.send_email(code, email)?;
        } else {
            let mobile = register_query_data.phone.to_owned().ok_or(
                ProviderError::CyberSherlockAuthProviderError(
                    CyberSherlockAuthProviderError::BadLoginQueryData,
                ),
            )?;
            //TODO: implement auth message to send not only code
            self.notification_provider.send_mobile(code, mobile)?;
        }
        Ok(())
    }

    pub fn get_register_cache_data_by_code(
        &mut self,
        code: &str,
    ) -> Result<RegisterCacheData, ProviderError> {
        let data = self.cache_service.get_value::<RegisterCacheData>(code)?;
        log::debug!("{:?}", &data);
        if let Some(register_data) = data {
            return Ok(register_data);
        }
        Err(ProviderError::CallbackStateCacheError)
    }

    pub fn get_tokens(
        &self,
        user_profile: &CyberSherlockProfile,
    ) -> Result<SessionTokens, ProviderError> {
        let config = self.config.clone();
        let profile = user_profile.clone();
        let access_token_expiration =
            Utc::now() + Duration::seconds(self.config.jwt_access_token_ttl_sec);
        let access_token_claims = AccessTokenClaims {
            aud: config.app_id.clone(),
            sub: profile.user_id.to_string(),
            email: profile.email,
            email_verified: user_profile.email_verified,
            phone: profile.phone,
            phone_verified: user_profile.phone_verified,
            name: profile.name,
            exp: access_token_expiration.timestamp(),
        };
        let access_token_string = encode_claims::<AccessTokenClaims>(
            &access_token_claims,
            &self.config.jwt_secret,
            None,
        )?;

        let refresh_token_expiration =
            Utc::now() + Duration::seconds(self.config.jwt_refresh_token_ttl_sec);
        let refresh_token_claims = RefreshTokenClaims {
            aud: config.app_id,
            sub: user_profile.user_id.to_string(),
            exp: refresh_token_expiration.timestamp(),
        };
        let refresh_token_string = encode_claims::<RefreshTokenClaims>(
            &refresh_token_claims,
            &self.config.jwt_secret,
            None,
        )?;
        let tokens = SessionTokens {
            access_token: Some(Token {
                token_string: access_token_string,
                expire: Some(access_token_expiration),
            }),
            refresh_token: Some(Token {
                token_string: refresh_token_string,
                expire: Some(refresh_token_expiration),
            }),
            extra_token: None,
        };
        Ok(tokens)
    }

    pub fn create_user_profile(
        &self,
        user_data: &RegisterCacheData,
    ) -> Result<CyberSherlockProfile, ProviderError> {
        let user_profile = CyberSherlockProfile {
            user_id: User::generate_user_id(),
            name: String::from(""),
            email: user_data.email.clone(),
            email_verified: false,
            phone: user_data.phone.clone(),
            phone_verified: false,
            picture: None,
            hash: user_data.hash.clone(),
        };
        Ok(user_profile)
    }

    pub fn validate_callback_state(
        cache_state: &str,
        callback_state: &str,
    ) -> Result<(), ProviderError> {
        if cache_state == callback_state {
            return Ok(());
        }
        log::debug!(
            "Callback state {} doesnt equal to cache state {}",
            callback_state,
            cache_state
        );
        Err(ProviderError::CallbackStateCacheError)
    }
}
