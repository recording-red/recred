use ::entity::channel;
use db::channel::ChannelQuery;
use sea_orm::*;

pub async fn create(db: &DbConn, data: channel::Model) -> Result<channel::Model, DbErr> {
    ChannelQuery::insert(db, data).await
    // obj.try_into_model()
}

pub async fn read(db: &DbConn, id: String) -> Result<Option<channel::Model>, DbErr> {
    ChannelQuery::find_by_id(db, id).await
}

pub async fn read_all(db: &DbConn) -> Result<Vec<channel::Model>, DbErr> {
    ChannelQuery::find(db).await
}

//async fn update()

//async fn delete() {}
