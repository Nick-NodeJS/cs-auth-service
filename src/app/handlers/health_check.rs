use actix_web::HttpResponse;

use super::common::response::SUCCESS;

pub async fn status() -> HttpResponse {
    HttpResponse::Ok().body(SUCCESS)
}
