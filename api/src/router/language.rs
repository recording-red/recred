use crate::errors::RecRedError;
use crate::AppState;
use actix_web::{get, post, web, Responder};
use domain::language::{create, read_all};
use entity::language;

#[post("/")]
async fn post(
    data: web::Data<AppState>,
    json: web::Json<language::Model>,
) -> Result<impl Responder, RecRedError> {
    let conn = &data.conn;
    let data = json.into_inner();
    let obj = create(conn, data).await.expect("could not create language");
    Ok(web::Json(obj))
}

#[get("/")]
async fn get(data: web::Data<AppState>) -> Result<impl Responder, RecRedError> {
    let conn = &data.conn;
    let obj = read_all(conn).await.expect("could not read languages");
    Ok(web::Json(obj))
}
