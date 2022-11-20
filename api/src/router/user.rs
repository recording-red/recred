use crate::AppState;
use actix_web::{get, post, web, Error, HttpResponse};
use core::user::UserQuery;
use entity::user;

#[post("/")]
async fn create(
    data: web::Data<AppState>,
    user_json: web::Json<user::Model>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let user_data = user_json.into_inner();
    UserQuery::create(conn, user_data)
        .await
        .expect("could not create user");
    Ok(HttpResponse::Ok().finish())
}
