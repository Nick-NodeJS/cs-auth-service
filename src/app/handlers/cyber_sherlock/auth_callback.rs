use actix_web::{
    web::{self},
    HttpRequest, HttpResponse,
};

use crate::app::{
    app_data::AppData,
    app_error::AppError,
    handlers::common::response::{SUCCESS, USER_SHOULD_RELOGIN},
    models::user_profile::UserProfile,
    providers::{
        common::parse_callback_query_string, cyber_sherlock::provider::CyberSherlockAuthProvider,
    },
    services::common::{error_as_json, result_as_json},
};

pub async fn auth_callback(
    req: HttpRequest,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, AppError> {
    let callback_query_data = parse_callback_query_string(req.query_string())?;

    let mut user_service = app_data.user_service.lock()?;
    let mut cyber_sherlock_auth_provider = app_data.cyber_sherlock_auth_provider.lock()?;
    log::debug!("{:?}", &callback_query_data.code);
    // Check if state the same it sent on login step
    let register_cache_data =
        cyber_sherlock_auth_provider.get_register_cache_data_by_code(&callback_query_data.code)?;

    CyberSherlockAuthProvider::validate_callback_state(
        &register_cache_data.pkce_code_verifier,
        &callback_query_data.state,
    )?;

    //TODO:
    // - implement uniq key on user storage or other logic to avoid email or phone duplicates

    let user_profile = cyber_sherlock_auth_provider.create_user_profile(&register_cache_data)?;

    let tokens = cyber_sherlock_auth_provider.get_tokens(&user_profile)?;

    if let Some(user_session) = user_service
        .get_user_session(
            tokens,
            UserProfile::CyberSherlock(user_profile.clone()),
            register_cache_data.session.metadata.clone(),
        )
        .await?
    {
        log::debug!(
            "User {} loged in with CyberSherlock successfuly",
            &user_session.user_id
        );
        let mut response = HttpResponse::Ok().json(result_as_json(SUCCESS));
        user_service.set_session_cookie(response.head_mut(), &user_session)?;
        user_service
            .remove_anonymous_session(register_cache_data.session)
            .await?;

        Ok(response)
    } else {
        //TODO: check if this logic is required
        log::warn!(
            "\nCyberSherlock user_id: {} has no data in system. Should relogin to CyberSherlock\n",
            user_profile.user_id
        );
        // TODO: investigate if it's better for UX to pass throw login to return auth_url on this step
        Ok(HttpResponse::Unauthorized().json(error_as_json(USER_SHOULD_RELOGIN)))
    }
}
