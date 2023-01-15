use ::entity::channel;
use db::channel::ChannelQuery;
use sea_orm::*;

pub async fn create(db: &DbConn, data: channel::Model) -> Result<channel::Model, DbErr> {
    let obj = ChannelQuery::save(db, data).await?;
    obj.try_into_model()
}

//async fn read() {}

pub async fn read_all(db: &DbConn) -> Result<Vec<channel::Model>, DbErr> {
    ChannelQuery::find(db).await
}

//async fn update()

//async fn delete() {}
