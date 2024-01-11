#[cfg(test)]
mod tests {
    use bson::oid::ObjectId;

    use crate::app::{
        models::{
            common::AuthProviders,
            session::{NewSessionData, Session},
            session_metadata::SessionMetadata,
            session_tokens::SessionTokens,
            token::Token,
        },
        repositories::session::repository::SessionRepository,
        services::cache::{common::CacheServiceType, service::RedisCacheService},
    };

    // TODO!: implement whole session repo test logic including cache cleaning

    #[actix_rt::test]
    async fn set_session() {
        let redis_cache = RedisCacheService::new(CacheServiceType::Session)
            .expect("Error to create Session Cache in Session repository test");
        let mut session_repository = SessionRepository::new(redis_cache);
        let test_session = Session::new(NewSessionData {
            anonimous: false,
            auth_provider: AuthProviders::Google,
            user_id: ObjectId::new(),
            tokens: SessionTokens {
                access_token: Some(Token {
                    token_string: String::from("test_id_token_token"),
                    expire: None,
                }),
                refresh_token: Some(Token {
                    token_string: String::from("test_refresh_token_token"),
                    expire: None,
                }),
                extra_token: Some(Token {
                    token_string: String::from("test_access_token_token"),
                    expire: None,
                }),
            },
            session_metadata: SessionMetadata::new(),
        });

        let session_key = Session::get_session_key(&test_session.id);
        let result = session_repository
            .set_session(&session_key, &test_session, 300)
            .await;
        assert_eq!(result.is_ok(), true)
    }
}
