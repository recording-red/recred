use crate::errors::RecRedError;
use crate::AppState;
use actix_web::{get, patch, post, web, Responder};
use domain::channel::{create, read, read_all, update_background};
use entity::channel;
use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
pub struct BackgroundForm {
    data: Vec<u8>,
}

#[patch("/background/{id}/")]
async fn patch_background(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Bytes,
) -> Result<impl Responder, RecRedError> {
    let conn = &data.conn;
    let id = path.into_inner();
    let data: Vec<u8> = body.to_vec();
    let data = {
        if data.len() == 0 {
            None
        } else {
            Some(data)
        }
    };
    let obj = update_background(conn, id, data)
        .await
        .expect("could not update background");
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
