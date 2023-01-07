use crate::errors::RecRedError;
use crate::AppState;
use actix_web::{get, post, web, Responder};
use domain::user::{create, read_all};
use entity::user;

#[post("/")]
async fn post(
    data: web::Data<AppState>,
    user_json: web::Json<user::Model>,
) -> Result<impl Responder, RecRedError> {
    let conn = &data.conn;
    let user_data = user_json.into_inner();
    let obj = create(conn, user_data)
        .await
        .expect("could not create user");
    Ok(web::Json(obj))
}

#[get("/")]
async fn get(data: web::Data<AppState>) -> Result<impl Responder, RecRedError> {
    let conn = &data.conn;
    let obj = read_all(conn).await.expect("could not list users");
    Ok(web::Json(obj))
}
