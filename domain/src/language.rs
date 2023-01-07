use ::entity::language;
use db::language::LanguageQuery;
use sea_orm::error::DbErr;
use sea_orm::*;

pub async fn create(conn: &DbConn, data: language::Model) -> Result<language::Model, DbErr> {
    let obj = LanguageQuery::save(conn, data).await?;
    obj.try_into_model()
}

//async fn read() {}

pub async fn read_all(db: &DbConn) -> Result<Vec<language::Model>, DbErr> {
    LanguageQuery::find(db).await
}

//async fn update()

//async fn delete() {}
