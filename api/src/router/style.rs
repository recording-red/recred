use crate::errors::RecRedError;
use crate::AppState;
use actix_web::{get, post, web, Responder};
use domain::style::{create, read_all};
use entity::style;

#[post("/")]
async fn post(
    data: web::Data<AppState>,
    json: web::Json<style::Model>,
) -> Result<impl Responder, RecRedError> {
    let conn = &data.conn;
    let data = json.into_inner();
    let obj = create(conn, data).await.expect("could not create style");
    Ok(web::Json(obj))
}

#[get("/")]
async fn get(data: web::Data<AppState>) -> Result<impl Responder, RecRedError> {
    let conn = &data.conn;
    let obj = read_all(conn).await.expect("could not read styles");
    Ok(web::Json(obj))
}
