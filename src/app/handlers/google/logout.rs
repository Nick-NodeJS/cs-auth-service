use actix_web::{web, HttpResponse};

use crate::app::{app_data::AppData, app_error::AppError};

pub async fn login(app_data: web::Data<AppData>) -> Result<HttpResponse, AppError> {
  Ok(HttpResponse::Ok().body("Google logout"))
}