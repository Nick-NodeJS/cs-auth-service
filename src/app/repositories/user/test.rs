#[cfg(test)]
mod tests {
    use crate::{
        app::{
            repositories::user::repository::UserRepository,
            services::{
                cache::{common::CacheServiceType, service::RedisCacheService},
                storage::service::StorageService,
            },
            // tests::test_data::TestData,
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
