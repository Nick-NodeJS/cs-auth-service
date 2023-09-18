use actix_web::HttpResponse;

pub async fn status() -> HttpResponse {
  HttpResponse::Ok().body("Hello from cs-auth-service!\n")
}