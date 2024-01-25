use actix_web::{
    web::{self},
    HttpRequest, HttpResponse,
};

use crate::app::{
    app_data::AppData,
    app_error::AppError,
    models::user::UserProfile,
    services::common::{error_as_json, result_as_json},
};

pub async fn auth_callback(
    req: HttpRequest,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(result_as_json("success")))
}
