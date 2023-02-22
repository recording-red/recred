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

pub async fn update_background(
    db: &DbConn,
    id: String,
    data: Option<Vec<u8>>,
) -> Result<channel::Model, DbErr> {
    ChannelQuery::save_background(db, id, data).await
}

//async fn update()

//async fn delete() {}
