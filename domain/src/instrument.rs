use ::entity::instrument;
use db::instrument::InstrumentQuery;
use sea_orm::error::DbErr;
use sea_orm::*;

//async fn read() {}

pub async fn read_all(db: &DbConn) -> Result<Vec<instrument::Model>, DbErr> {
    InstrumentQuery::find(db).await
}

//async fn update()

//async fn delete() {}
