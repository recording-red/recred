use crate::errors::RecRedError;
use crate::AppState;
use actix_web::{get, post, web, Responder};
use domain::channel::{create, read, read_all};
use entity::channel;

#[post("/")]
async fn post(
    data: web::Data<AppState>,
    json: web::Json<channel::Model>,
) -> Result<impl Responder, RecRedError> {
    let conn = &data.conn;
    let data = json.into_inner();
    let obj = create(conn, data).await.expect("could not create channel");
    Ok(web::Json(obj))
}

#[get("/")]
async fn list(data: web::Data<AppState>) -> Result<impl Responder, RecRedError> {
    let conn = &data.conn;
    let obj = read_all(conn).await.expect("could not read channels");
    Ok(web::Json(obj))
}

#[get("/{id}/")]
async fn get(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, RecRedError> {
    let id = path.into_inner();
    let conn = &data.conn;
    let obj = read(conn, id).await.expect("could not read channel");
    Ok(web::Json(obj))
}
