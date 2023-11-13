use actix_web::{web, HttpRequest, HttpResponse};

use crate::app::{app_data::AppData, app_error::AppError};

pub async fn auth_callback(
    req: HttpRequest,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, AppError> {
    // let mut cache_service = app_data.cache_service.lock()?;
    let mut google_service = app_data.google_service.lock()?;
    let (code, state) = google_service.parse_auth_query_string(req.query_string())?;
    // process code and state to get tokens
    // TODO: update google_service.get_user_data() to get GoogleProfile and tokens,
    // user_service.set_google_user().await, including data storage and cache updating
    let tokens = google_service.get_tokens(code, state).await?;
    // let token_data = google_service.get_user_profile(tokens.clone()).await?;
    // println!("Google User: {:?}", token_data);
    let tokens_as_json = google_service.tokens_as_json(tokens);
    Ok(HttpResponse::Ok().json(tokens_as_json))
}
