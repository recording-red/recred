use ::entity::{language, language::Entity as Language};
use chrono::{FixedOffset, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::*;

pub struct LanguageQuery;

impl LanguageQuery {
    pub async fn find(db: &DbConn) -> Result<Vec<instrument::Model>, DbErr> {
        Language::find().all(db).await
    }
}
