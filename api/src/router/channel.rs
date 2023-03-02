use crate::errors::RecRedError;
use crate::AppState;
use actix_web::{get, patch, post, web, Responder};
use domain::channel::{create, read, read_all, update_banner, update_profile};
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

#[patch("/banner/{id}/")]
async fn patch_banner(
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
    let obj = update_banner(conn, id, data)
        .await
        .expect("could not update banner");
    Ok(web::Json(obj))
}

#[patch("/profile/{id}/")]
async fn patch_profile(
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
    let obj = update_profile(conn, id, data)
        .await
        .expect("could not update profile");
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
