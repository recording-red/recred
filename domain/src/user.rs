use ::entity::user;
use db::user::UserQuery;
use sea_orm::*;

pub async fn create(db: &DbConn, data: user::Model) -> Result<user::Model, DbErr> {
    UserQuery::insert(db, data).await
    //obj.try_into_model()
}

pub async fn read(db: &DbConn, id: String) -> Result<Option<user::Model>, DbErr> {
    UserQuery::find_by_id(db, id).await
}

pub async fn read_all(db: &DbConn) -> Result<Vec<user::Model>, DbErr> {
    UserQuery::find(db).await
}

//async fn update()

//async fn delete() {}
