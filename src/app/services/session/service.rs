use actix_web::{
    cookie::{Cookie, CookieJar},
    dev::{ResponseHead, ServiceRequest},
    http::header::{HeaderValue, SET_COOKIE},
};
use bson::oid::ObjectId;

use crate::{
    app::{
        models::{
            common::AuthProviders,
            session::{NewSessionData, Session},
        },
        repositories::session::repository::SessionRepository,
        services::cache::service::RedisCacheService,
    },
    config::{
        cookie_config::{CookieConfiguration, CookieContentSecurity},
        session_config::SessionConfig,
    },
};

use super::error::SessionServiceError;

#[derive(Debug)]
pub struct SessionService {
    pub config: SessionConfig,
    repository: SessionRepository,
}

impl SessionService {
    pub fn new(config: SessionConfig, session_cache_service: RedisCacheService) -> Self {
        let repository = SessionRepository::new(session_cache_service);
        SessionService { config, repository }
    }

    pub async fn get_sessions(
        &mut self,
        user_id: ObjectId,
        auth_provider: AuthProviders,
    ) -> Result<Vec<Session>, SessionServiceError> {
        let sessions_key = Session::get_user_sessions_key(&user_id.to_string());
        let sessions = self.repository.get_sessions(sessions_key.as_ref()).await?;
        let user_sessions = sessions
            .into_iter()
            .filter(|s| s.auth_provider.is_equal(&auth_provider))
            .collect();
        Ok(user_sessions)
    }

    // TODO: token in most cases has expiration time
    // - set session ttl as token expiration
    pub async fn set_new_session(
        &mut self,
        new_session_data: NewSessionData,
    ) -> Result<Session, SessionServiceError> {
        let session = Session::new(new_session_data);
        let session_key = Session::get_session_key(&session.id);
        self.repository
            .set_session(&session_key, &session, self.config.session_ttl_sec)
            .await?;
        Ok(session)
    }

    pub async fn remove_sessions_by_session(
        &mut self,
        session: Session,
    ) -> Result<(), SessionServiceError> {
        let user_sessions_key = Session::get_user_sessions_key(&session.user_id.to_string());
        let user_sessions = self.repository.get_sessions(&user_sessions_key).await?;
        let session_keys_to_remove: Vec<String> = user_sessions
            .into_iter()
            .filter(|s| s.auth_provider.is_equal(&session.auth_provider))
            .map(|s| Session::get_session_key(&s.id))
            .collect();
        self.repository
            .remove_sessions(&user_sessions_key, session_keys_to_remove)
            .await?;
        Ok(())
    }

    pub async fn remove_session_by_id(
        &mut self,
        session_id: &str,
    ) -> Result<(), SessionServiceError> {
        self.repository
            .remove_session_by_key(Session::get_session_key(&session_id))
            .await?;
        Ok(())
    }

    pub fn set_cookie_session_id(
        config: &CookieConfiguration,
        response_head: &mut ResponseHead,
        session_id: String,
    ) -> Result<(), SessionServiceError> {
        // it should gets session cookie with encrypted session id
        // let config = &self.config.cookie_config;
        let mut cookie = Cookie::new(config.name.clone(), session_id);

        cookie.set_secure(config.secure);
        cookie.set_http_only(config.http_only);
        cookie.set_same_site(config.same_site);
        cookie.set_path(config.path.clone());

        if let Some(max_age) = config.max_age {
            cookie.set_max_age(max_age);
        }

        if let Some(ref domain) = config.domain {
            cookie.set_domain(domain.clone());
        }

        let mut jar = CookieJar::new();
        jar.private_mut(&config.key).add(cookie);

        // set cookie
        let cookie = jar.delta().next().unwrap();
        let val = HeaderValue::from_str(&cookie.encoded().to_string())
            .map_err(|_| SessionServiceError::SetCookieToResponseError)?;

        response_head.headers_mut().append(SET_COOKIE, val);

        Ok(())
    }

    pub fn get_cookie_session_id(
        config: &CookieConfiguration,
        request: &ServiceRequest,
    ) -> Option<String> {
        // it should gets session id from cookie
        let cookies = match request.cookies().ok() {
            Some(c) => c,
            None => {
                log::debug!("No cookies on request, ignoring...");

                return None;
            }
        };
        let session_cookie = match cookies.iter().find(|&cookie| cookie.name() == config.name) {
            Some(c) => c,
            None => {
                log::debug!("No session cookie on request, ignoring...");

                return None;
            }
        };

        let mut jar = CookieJar::new();
        jar.add_original(session_cookie.clone());

        let verification_result = match config.content_security {
            CookieContentSecurity::Signed => jar.signed(&config.key).get(&config.name),
            CookieContentSecurity::Private => jar.private(&config.key).get(&config.name),
        };

        if verification_result.is_none() {
            log::warn!(
                "The session cookie attached to the incoming request failed to pass cryptographic \
                checks (signature verification/decryption)."
            );
        }

        match verification_result?.value().to_owned().try_into() {
            Ok(session_key) => Some(session_key),
            Err(_) => {
                log::warn!("Invalid session key, ignoring...");

                None
            }
        }
    }
}
