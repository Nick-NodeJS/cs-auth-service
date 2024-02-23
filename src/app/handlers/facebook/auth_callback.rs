use actix_web::{
    web::{self},
    HttpRequest, HttpResponse,
};

use crate::app::{
    app_data::AppData,
    app_error::AppError,
    handlers::common::response::{SUCCESS, USER_SHOULD_RELOGIN},
    models::{session::Session, user_profile::UserProfile},
    providers::common::parse_callback_query_string,
    services::common::{error_as_json, result_as_json},
};

pub async fn auth_callback(
    req: HttpRequest,
    app_data: web::Data<AppData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    let callback_query_data = parse_callback_query_string(req.query_string())?;

    let mut user_service = app_data.user_service.lock()?;
    let mut facebook_provider = app_data.facebook_provider.lock()?;

    // It's critical to get LoginCacheData before tokens getting by code!!!
    // this way it checks if state is valid
    let login_cache_data =
        facebook_provider.get_login_cache_data_by_state(&callback_query_data.state)?;

    let tokens = facebook_provider
        .get_tokens(&callback_query_data.code)
        .await?;

    let user_profile = facebook_provider.get_user_profile(&tokens).await?;

    if let Some(user_session) = user_service
        .get_user_session(
            tokens,
            UserProfile::Facebook(user_profile.clone()),
            login_cache_data.session.metadata.clone(),
        )
        .await?
    {
        log::debug!(
            "User {} loged in with Facebook successfuly",
            &user_session.user_id
        );
        let mut response = HttpResponse::Ok().json(result_as_json(SUCCESS));
        user_service.set_session_cookie(response.head_mut(), &user_session)?;
        user_service
            .remove_anonymous_sessions(vec![login_cache_data.session, session])
            .await?;

        Ok(response)
    } else {
        log::warn!(
            "\nFacebook user_id: {} has no data in system. Should relogin to Facebook\n",
            user_profile.user_id
        );
        // TODO: investigate if it's better for UX to pass throw login to return auth_url on this step
        Ok(HttpResponse::Unauthorized().json(error_as_json(USER_SHOULD_RELOGIN)))
    }
}
