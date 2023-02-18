use ::entity::team;
use db::team::TeamQuery;
use sea_orm::*;

pub async fn create(db: &DbConn, data: team::Model) -> Result<team::Model, DbErr> {
    TeamQuery::insert(db, data).await
    // obj.try_into_model()
}

pub async fn read(db: &DbConn, id: String) -> Result<Option<team::Model>, DbErr> {
    TeamQuery::find_by_id(db, id).await
}

pub async fn read_all(db: &DbConn) -> Result<Vec<team::Model>, DbErr> {
    TeamQuery::find(db).await
}

//async fn update()

//async fn delete() {}
