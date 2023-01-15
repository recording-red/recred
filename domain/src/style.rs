use ::entity::style;
use db::style::StyleQuery;
use sea_orm::error::DbErr;
use sea_orm::*;

pub async fn create(conn: &DbConn, data: style::Model) -> Result<style::Model, DbErr> {
    let obj = StyleQuery::save(conn, data).await?;
    obj.try_into_model()
}

//async fn read() {}

pub async fn read_all(db: &DbConn) -> Result<Vec<style::Model>, DbErr> {
    StyleQuery::find(db).await
}

//async fn update()

//async fn delete() {}
