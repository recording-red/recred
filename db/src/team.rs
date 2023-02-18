use crate::utils::generate_id;
use ::entity::{team, team::Entity as Team};
use chrono::{FixedOffset, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::*;

pub struct TeamQuery;

impl TeamQuery {
    pub async fn find(db: &DbConn) -> Result<Vec<team::Model>, DbErr> {
        Team::find().all(db).await
    }

    pub async fn find_by_id(db: &DbConn, id: String) -> Result<Option<team::Model>, DbErr> {
        Team::find_by_id(id).one(db).await
    }

    pub async fn insert(db: &DbConn, data: team::Model) -> Result<team::Model, DbErr> {
        team::ActiveModel {
            id: Set(generate_id().to_owned()),
            user_id: Set(data.user_id.to_owned()),
            role_id: Set(data.role_id.to_owned()),
            created_at: Set(Utc::now()
                .with_timezone(&FixedOffset::east_opt(0).unwrap())
                .to_owned()),
            updated_at: Set(Utc::now()
                .with_timezone(&FixedOffset::east_opt(0).unwrap())
                .to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn save(db: &DbConn, data: team::Model) -> Result<team::ActiveModel, DbErr> {
        team::ActiveModel {
            id: Set(data.id.to_owned()),
            user_id: Set(data.user_id.to_owned()),
            role_id: Set(data.role_id.to_owned()),
            created_at: Set(Utc::now()
                .with_timezone(&FixedOffset::east_opt(0).unwrap())
                .to_owned()),
            updated_at: Set(Utc::now()
                .with_timezone(&FixedOffset::east_opt(0).unwrap())
                .to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
}
