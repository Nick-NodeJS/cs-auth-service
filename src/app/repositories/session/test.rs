#[cfg(test)]
mod tests {

    use crate::{
        app::{
            models::session::Session,
            repositories::session::repository::SessionRepository,
            services::cache::{common::CacheServiceType, service::RedisCacheService},
        },
        tests::test_data::TestData,
    };

    // Set fake Google Session to Cache
    #[actix_rt::test]
    async fn set_session() {
        let (mut session_repository, test_data) = initialize();

        let session_key = Session::get_session_key(&test_data.google_session.id);
        let result = session_repository
            .set_session(&session_key, &test_data.google_session, 10)
            .await;
        assert_eq!(result.is_ok(), true);

        let user_sessions_key = Session::get_user_sessions_key(&test_data.user.id.to_string());

        let result = session_repository
            .remove_sessions(&user_sessions_key, vec![session_key.clone()])
            .await;
        assert_eq!(result.is_ok(), true);
    }

    // Get fake Google Session from Cache(we need before set fake session to Cache to be able to get it by session key)
    #[actix_rt::test]
    async fn get_sessions() {
        let (mut session_repository, test_data) = initialize();

        let session_key = Session::get_session_key(&test_data.google_session.id);
        let result = session_repository
            .set_session(&session_key, &test_data.google_session, 10)
            .await;
        assert_eq!(result.is_ok(), true);
        match session_repository.get_sessions(&session_key).await {
            Ok(sessions) => {
                let session = sessions.into_iter().next();
                assert_eq!(session.is_some(), true);
                if let Some(s) = session {
                    let s_key = Session::get_session_key(&s.id);
                    assert_eq!(s_key, session_key);
                }
            }
            Err(err) => {
                println!("Error to get fake user sessions: {}", err);
            }
        };

        let user_sessions_key = Session::get_user_sessions_key(&test_data.user.id.to_string());

        let result = session_repository
            .remove_sessions(&user_sessions_key, vec![session_key.clone()])
            .await;
        assert_eq!(result.is_ok(), true);
    }

    #[actix_rt::test]
    async fn remove_sessions() {
        let (mut session_repository, test_data) = initialize();

        let session_key = Session::get_session_key(&test_data.google_session.id);
        let result = session_repository
            .set_session(&session_key, &test_data.google_session, 10)
            .await;
        assert_eq!(result.is_ok(), true);

        let user_sessions_key = Session::get_user_sessions_key(&test_data.user.id.to_string());

        let result = session_repository
            .remove_sessions(&user_sessions_key, vec![session_key.clone()])
            .await;
        assert_eq!(result.is_ok(), true);

        let result = session_repository.get_sessions(&session_key).await;
        assert_eq!(result.is_ok(), true);
        if let Ok(sessions) = result {
            assert_eq!(sessions.len(), 0);
        }
    }

    fn initialize() -> (SessionRepository, TestData) {
        let redis_cache = RedisCacheService::new(CacheServiceType::Session)
            .expect("Error to create Session Cache");
        let session_repository = SessionRepository::new(redis_cache);

        let test_data = TestData::new();
        (session_repository, test_data)
    }
}
