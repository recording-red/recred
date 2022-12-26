use crate::errors::RecRedError;
use crate::AppState;
use actix_web::{post, web, HttpResponse, Responder};
use domain::user::create as dcreate;
use entity::user;

#[post("/")]
async fn create(
    data: web::Data<AppState>,
    user_json: web::Json<user::Model>,
) -> Result<impl Responder, RecRedError> {
    let conn = &data.conn;
    let user_data = user_json.into_inner();
    let obj = dcreate(conn, user_data).await?;
    Ok(HttpResponse::Ok().finish())
}
