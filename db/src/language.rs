use ::entity::{language, language::Entity as Language};
use sea_orm::entity::prelude::*;
use sea_orm::*;

pub struct LanguageQuery;

impl LanguageQuery {
    pub async fn find(db: &DbConn) -> Result<Vec<language::Model>, DbErr> {
        Language::find().all(db).await
    }

    pub async fn save(db: &DbConn, data: language::Model) -> Result<language::ActiveModel, DbErr> {
        language::ActiveModel {
            name: Set(data.name.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
}
