use actix_web::HttpResponse;

pub async fn status() -> HttpResponse {
  HttpResponse::Ok().body("Status cs-auth-service ok!\n")
}