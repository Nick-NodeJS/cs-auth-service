#[cfg(test)]
mod tests {
    use crate::{
        app::{
            models::user::UserProfile,
            repositories::user::repository::UserRepository,
            services::{
                cache::{common::CacheServiceType, service::RedisCacheService},
                storage::service::StorageService,
            },
        },
        config::user_config::UserConfig,
        tests::test_data::TestData,
    };

    #[actix_rt::test]
    async fn insert_user() {
        // user repository and test data
        let (mut user_repository, test_data) = intialize().await;

        let result = user_repository.insert_user(test_data.user.clone()).await;
        assert_eq!(result.is_ok(), true);

        let inserted_result = match user_repository
            .find_user_by_id(test_data.user.id.clone())
            .await
        {
            Ok(user) => user,
            Err(err) => {
                assert!(false, "{}", err.to_string());
                None
            }
        };
        assert_eq!(inserted_result.is_some(), true);

        if let Some(user) = inserted_result {
            let result = user_repository.delete_by_id(user.id).await;
            assert_eq!(result.is_ok(), true)
        }
    }

    #[actix_rt::test]
    async fn find_user_by_profile() {
        // user repository and test data
        let (mut user_repository, test_data) = intialize().await;

        // Seed user data
        let result = user_repository.insert_user(test_data.user.clone()).await;
        assert_eq!(result.is_ok(), true);

        let user_profile = match test_data.user.google {
            Some(google_profile) => google_profile,
            None => return assert!(false, "Bad test user google profile!"),
        };

        match user_repository
            .find_user_by_profile(UserProfile::Google(user_profile))
            .await
        {
            Ok(try_user) => {
                if let Some(user) = try_user {
                    assert_eq!(user.id, test_data.user.id);
                } else {
                    return assert!(false, "User not found by find_user_by_profile!?");
                }
            }
            Err(err) => assert!(false, "Error find_user_by_profile {}", err),
        };

        let result = user_repository.delete_by_id(test_data.user.id).await;
        assert_eq!(result.is_ok(), true)
    }

    async fn intialize() -> (UserRepository, TestData) {
        let user_config = UserConfig::new();
        // Storage service
        let storage_service = StorageService::new()
            .await
            .expect("Error to create Storage");

        // User Cache service
        let user_cache_service = RedisCacheService::new(CacheServiceType::User)
            .expect("Error to create User Cache Service");

        (
            UserRepository::new(user_cache_service, user_config, storage_service),
            TestData::new(),
        )
    }
}
