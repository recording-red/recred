use ::entity::user;
use db::user::UserQuery;
use sea_orm::error::DbErr;
use sea_orm::*;

pub async fn create(db: &DbConn, data: user::Model) -> Result<user::ActiveModel, DbErr> {
    UserQuery::save(db, data).await
}

//async fn read_all() {}

pub async fn read_all(db: &DbConn) -> Result<Vec<user::Model>, DbErr> {
    UserQuery::find(db).await
}

//async fn update()

//async fn delete() {}
