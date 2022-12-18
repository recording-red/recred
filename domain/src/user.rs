use ::entity::user;
use db::user::UserQuery;
use sea_orm::error::DbErr;
use sea_orm::*;

pub async fn create(conn: &DbConn, data: user::Model) -> Result<user::ActiveModel, DbErr> {
    UserQuery::create(conn, data).await
}

//async fn read() {}
//
//async fn read_all() {}
//
//async fn update()
//
//async fn delete() {}
