use crate::errors::RecRedError;
use crate::AppState;
use actix_web::{get, post, web, Error, Responder};
use domain::registration::{create, read_all};
use entity::registration;

#[post("/")]
async fn post(
    data: web::Data<AppState>,
    json: web::Json<registration::Model>,
) -> Result<impl Responder, RecRedError> {
    let conn = &data.conn;
    let data = json.into_inner();
    let obj = create(conn, data)
        .await
        .expect("could not create user");
    Ok(web::Json(obj))
}

#[get("/")]
async fn get(data: web::Data<AppState>) -> Result<impl Responder, Error> {
    let conn = &data.conn;
    let obj = read_all(conn).await.expect("could not list users");
    Ok(web::Json(obj))
}
