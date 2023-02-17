use crate::errors::RecRedError;
use crate::AppState;
use actix_web::{get, web, Responder};
use domain::instrument::read_all;

#[get("/")]
async fn get(data: web::Data<AppState>) -> Result<impl Responder, RecRedError> {
    let conn = &data.conn;
    let obj = read_all(conn).await.expect("could not read instruments");
    Ok(web::Json(obj))
}
