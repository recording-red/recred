use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn service() -> HttpResponse {
    HttpResponse::Ok().finish()
}
