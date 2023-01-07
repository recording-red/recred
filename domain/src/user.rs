use ::entity::user;
use db::user::UserQuery;
use sea_orm::error::DbErr;
use sea_orm::*;

pub async fn create(db: &DbConn, data: user::Model) -> Result<user::Model, DbErr> {
    let obj = UserQuery::save(db, data).await?;
    obj.try_into_model()
}

//async fn read() {}

pub async fn read_all(db: &DbConn) -> Result<Vec<user::Model>, DbErr> {
    UserQuery::find(db).await
}

//async fn update()

//async fn delete() {}
