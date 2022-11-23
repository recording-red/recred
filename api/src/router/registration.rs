use crate::AppState;
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use core::registration::RegistrationQuery;
use entity::registration;

#[post("/")]
async fn create(
    data: web::Data<AppState>,
    registration_json: web::Json<registration::Model>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let registration_data = registration_json.into_inner();
    RegistrationQuery::create(conn, registration_data)
        .await
        .expect("could not create registration");
    Ok(HttpResponse::Ok().finish())
}

#[get("/")]
async fn find(data: web::Data<AppState>) -> Result<impl Responder, Error> {
    let conn = &data.conn;
    let objs = RegistrationQuery::find(conn)
        .await
        .expect("could not list registrations");
    Ok(web::Json(objs))
}
