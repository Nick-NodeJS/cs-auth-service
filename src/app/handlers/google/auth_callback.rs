use actix_web::HttpResponse;

pub async fn auth_callback() -> HttpResponse {
  println!("google_callback!");
  HttpResponse::Ok().body("Hello from cs-auth-service!\n")
}