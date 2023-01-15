use ::entity::{style, style::Entity as Style};
use sea_orm::entity::prelude::*;
use sea_orm::*;

pub struct StyleQuery;

impl StyleQuery {
    pub async fn find(db: &DbConn) -> Result<Vec<style::Model>, DbErr> {
        Style::find().all(db).await
    }

    pub async fn save(db: &DbConn, data: style::Model) -> Result<style::ActiveModel, DbErr> {
        style::ActiveModel {
            name: Set(data.name.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
}
