use bson::oid::ObjectId;

use crate::app::{
    models::{
        common::AuthProviders,
        session::Session,
        session_tokens::SessionTokens,
        user::{User, UserProfile},
    },
    repositories::{session::repository::SessionRepository, user::repository::UserRepository},
    services::{
        cache::service::{CacheService, CacheServiceType},
        session::service::SessionService,
        storage::service::StorageService,
    },
};

use super::error::UserServiceError;

pub struct UserService {
    session_service: SessionService,
    user_repository: UserRepository,
}

impl UserService {
    pub async fn new() -> Result<Self, UserServiceError> {
        let storage_service = StorageService::new().await?;
        let user_cache_service = CacheService::new(CacheServiceType::User)?;
        let user_repository = UserRepository::new(
            storage_service.config.user_collection.clone(),
            user_cache_service,
            storage_service,
        );
        let session_cache_service = CacheService::new(CacheServiceType::Session)?;
        let session_service = SessionService::new(SessionRepository::new(session_cache_service));
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
            Ok(self.update_user_with_profile(user.id, user_profile).await?)
        } else {
            let new_user = User::new(user_profile);
            self.user_repository.insert_user(new_user.clone()).await?;
            Ok(new_user)
        }
    }

    pub async fn update_user_with_profile(
        &mut self,
        user_id: ObjectId,
        user_profile: UserProfile,
    ) -> Result<User, UserServiceError> {
        let query = UserRepository::get_update_user_profile_query(user_profile);
        let user = self.user_repository.update_user(user_id, query).await?;
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

    pub async fn set_new_session(
        &mut self,
        auth_provider: AuthProviders,
        user: User,
        tokens: SessionTokens,
    ) -> Result<Session, UserServiceError> {
        let session = self
            .session_service
            .set_new_session(auth_provider, user, tokens)
            .await?;
        Ok(session)
    }

    // TODO: investigate if this method should split logic per each provider
    pub async fn get_user_session(
        &mut self,
        tokens: SessionTokens,
        user_profile: UserProfile,
    ) -> Result<Option<Session>, UserServiceError> {
        // if provider(in case Google API) returns no refresh token, it has to check if user was logged in before
        // if No(refresh token is not in system) - it returns None and user has to relogin on provider(Google)
        let provider = UserProfile::get_provider(&user_profile);
        if tokens.is_incompleted(&provider) {
            log::debug!("\nUser profile: {:?} has no refresh token\n", user_profile);
            // TODO: adjust session logic
            // in general:
            // it should return to client session token only wich is uuid_v4 now
            // on this step:
            // - in case it has no refresh token it should to find user by profile
            // - if user exists the user data(which it sets in storage and reflect in cache
            // on 1st successful login step with refresh token) has to have refresh token
            // the app creates a new session with the same user but user data stay the same
            // it just impact on updated_at and refresh profile data(just in case something was changed after 1st step)
            // so it means user uses one more client and every should have its own session
            // use resresh token expire datetime to manage session cache ttl in case it less than default session ttl
            // keep in cache google access_token and id_token to use in case it needs to touch some GAPI
            if let Some(mut user_session) = self
                .get_user_session_by_profile(user_profile.clone())
                .await?
            {
                let exiten_user = self
                    .update_user_with_profile(user_session.user_id.clone(), user_profile.clone())
                    .await?;
                user_session.tokens.update_tokens(tokens.clone());
                // TODO: clone user session with the same token
                let new_user_session = self
                    .set_new_session(AuthProviders::Google, exiten_user, user_session.tokens)
                    .await?;
                Ok(Some(new_user_session))
            } else {
                log::debug!("\nUser profile {:?} is not in system.\n", user_profile);
                Ok(None)
            }
        } else {
            let user = self.create_user_with_profile(user_profile).await?;
            let session = self.set_new_session(provider, user, tokens).await?;
            Ok(Some(session))
        }
    }
}
