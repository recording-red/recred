use ::entity::{instrument, instrument::Entity as Instrument};
use sea_orm::entity::prelude::*;

pub struct InstrumentQuery;

impl InstrumentQuery {
    pub async fn find(db: &DbConn) -> Result<Vec<instrument::Model>, DbErr> {
        Instrument::find().all(db).await
    }
}
