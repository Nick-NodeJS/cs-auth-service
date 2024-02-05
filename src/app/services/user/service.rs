use actix_web::dev::ResponseHead;
use bson::oid::ObjectId;

use crate::{
    app::{
        models::{
            session::{NewSessionData, Session},
            session_metadata::SessionMetadata,
            session_tokens::SessionTokens,
            user::{User, UserProfile},
        },
        repositories::user::repository::UserRepository,
        services::{
            cache::service::RedisCacheService, session::service::SessionService,
            storage::service::StorageService,
        },
    },
    config::{session_config::SessionConfig, user_config::UserConfig},
};

use super::error::UserServiceError;

#[derive(Debug)]
pub struct UserService {
    session_service: SessionService,
    user_repository: UserRepository,
}

impl UserService {
    pub async fn new(
        config: UserConfig,
        storage_service: StorageService,
        user_cache_service: RedisCacheService,
        session_cache_service: RedisCacheService,
    ) -> Result<Self, UserServiceError> {
        let user_repository = UserRepository::new(user_cache_service, config, storage_service);
        let session_config = SessionConfig::new();
        let session_service = SessionService::new(session_config, session_cache_service);
        Ok(UserService {
            session_service,
            user_repository,
        })
    }

    pub async fn get_user_session_by_profile(
        &mut self,
        user_profile: UserProfile,
    ) -> Result<Option<Session>, UserServiceError> {
        if let Some(existen_user) = self.get_user_by_profile(user_profile.clone()).await? {
            let user_sessions = self
                .session_service
                .get_sessions(existen_user.id, UserProfile::get_provider(&user_profile))
                .await?;
            // TODO: filter or find user session by profile.get_auth_provider
            Ok(user_sessions.into_iter().next())
        } else {
            Ok(None)
        }
    }

    pub async fn create_user_with_profile(
        &mut self,
        user_profile: UserProfile,
    ) -> Result<User, UserServiceError> {
        if let Some(user) = self.get_user_by_profile(user_profile.clone()).await? {
            Ok(self
                .update_user_with_profile(&user.id, user_profile)
                .await?)
        } else {
            let new_user = User::new(user_profile);
            self.user_repository.insert_user(&new_user).await?;
            Ok(new_user)
        }
    }

    pub async fn update_user_with_profile(
        &mut self,
        user_id: &ObjectId,
        user_profile: UserProfile,
    ) -> Result<User, UserServiceError> {
        let user = self
            .user_repository
            .update_user_with_profile(user_id, user_profile)
            .await?;
        Ok(user)
    }

    pub async fn get_user_by_profile(
        &mut self,
        user_profile: UserProfile,
    ) -> Result<Option<User>, UserServiceError> {
        let user = self
            .user_repository
            .find_user_by_profile(user_profile)
            .await?;
        Ok(user)
    }

    pub async fn get_user_by_id(
        &mut self,
        user_id: &ObjectId,
    ) -> Result<Option<User>, UserServiceError> {
        let user = self.user_repository.find_user_by_id(user_id).await?;
        Ok(user)
    }

    pub async fn set_new_session(
        &mut self,
        new_session_data: NewSessionData,
    ) -> Result<Session, UserServiceError> {
        let session = self
            .session_service
            .set_new_session(new_session_data)
            .await?;
        Ok(session)
    }

    // TODO: investigate if this method should split logic per each provider
    pub async fn get_user_session(
        &mut self,
        tokens: SessionTokens,
        user_profile: UserProfile,
        session_metadata: SessionMetadata,
    ) -> Result<Option<Session>, UserServiceError> {
        // if provider(in case Google API) returns no refresh token, it has to check if user was logged in before
        // if No(refresh token is not in system) - it returns None and user has to relogin on provider(Google)
        let provider = UserProfile::get_provider(&user_profile);
        if !tokens.is_completed(&provider) {
            log::debug!(
                "\nUser profile: {:?} has incompleted tokens\n",
                user_profile
            );
            // TODO: adjust session logic
            // in general:
            // it should return to client session token only which is uuid_v4 now
            // on this step:
            // - in case it has incompleted token it should to find user by profile
            // - if user exists the user data(which it sets in storage and reflect in cache
            // on 1st successful login step with completed token) has to have all tokens
            // the app insert a new user client in session with the same user but user data stay the same
            // it just impact on updated_at and refresh profile data(just in case something was changed after 1st step)
            // so it means user uses one more client and every should have its own UserClient in user session
            // use resresh token expire datetime to manage session cache ttl in case it less than default session ttl
            // keep in cache google access_token and id_token to use in case it needs to touch some GAPI
            if let Some(mut user_session) = self
                .get_user_session_by_profile(user_profile.clone())
                .await?
            {
                let exiten_user = self
                    .update_user_with_profile(&user_session.user_id, user_profile.clone())
                    .await?;
                user_session.tokens.update_tokens(tokens.clone());
                // TODO: clone user session with the same token
                let new_user_session = self
                    .set_new_session(NewSessionData {
                        anonimous: false,
                        auth_provider: provider,
                        user_id: exiten_user.id,
                        tokens: user_session.tokens,
                        session_metadata,
                    })
                    .await?;
                Ok(Some(new_user_session))
            } else {
                log::debug!("\nUser profile {:?} is not in system.\n", user_profile);
                Ok(None)
            }
        } else {
            // TODO:
            // - set in session user client
            let user = self.create_user_with_profile(user_profile).await?;
            let session = self
                .set_new_session(NewSessionData {
                    anonimous: false,
                    auth_provider: provider,
                    user_id: user.id,
                    tokens: tokens,
                    session_metadata,
                })
                .await?;
            Ok(Some(session))
        }
    }

    pub fn set_session_cookie(
        &self,
        response_head: &mut ResponseHead,
        session: &Session,
    ) -> Result<(), UserServiceError> {
        Ok(SessionService::set_cookie_session_id(
            &self.session_service.config.cookie_config,
            response_head,
            session.id.clone(),
        )?)
    }

    pub async fn logout_by_session(&mut self, session: Session) -> Result<(), UserServiceError> {
        self.session_service
            .remove_sessions_by_session(session)
            .await?;
        Ok(())
    }
}
