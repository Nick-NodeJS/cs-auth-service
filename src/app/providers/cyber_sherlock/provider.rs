use oauth2::PkceCodeChallenge;
use url::Url;

use crate::{
    app::{
        models::session_metadata::SessionMetadata,
        providers::{error::ProviderError, notification::provider::NotificationProvider},
        services::cache::service::RedisCacheService,
    },
    config::app_config::AppConfig,
};

use super::{
    common::{hash_password, LoginCacheData, LoginQueryData},
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
        login_query_data: &LoginQueryData,
        session_metadata: SessionMetadata,
    ) -> Result<String, ProviderError> {
        // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
        let code = &pkce_code_challenge.as_str().to_string();
        let auth_url = Url::parse_with_params(
            self.config.auth_callback_url.as_ref(),
            &[("state", pkce_code_verifier.secret().to_string())],
        )?;
        // set auth data to cache
        let login_cache_data = LoginCacheData {
            pkce_code_verifier: pkce_code_verifier.secret().to_string(),
            session_metadata,
            hash: hash_password(login_query_data.password.as_str()).map_err(|_| {
                ProviderError::CyberSherlockAuthProviderError(
                    CyberSherlockAuthProviderError::Argon2PassHashError,
                )
            })?,
        };
        self.set_auth_data_to_cache(code, &login_cache_data)?;

        self.send_auth_code(code, login_query_data)?;
        Ok(auth_url.to_string())
    }

    pub fn set_auth_data_to_cache(
        &mut self,
        key: &str,
        data: &LoginCacheData,
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
        login_query_data: &LoginQueryData,
    ) -> Result<(), ProviderError> {
        if let Some(email) = login_query_data.email.to_owned() {
            self.notification_provider.send_email(code, email)?;
        } else {
            let mobile = login_query_data.mobile.to_owned().ok_or(
                ProviderError::CyberSherlockAuthProviderError(
                    CyberSherlockAuthProviderError::BadLoginQueryData,
                ),
            )?;
            self.notification_provider.send_mobile(code, mobile)?;
        }
        Ok(())
    }
}
