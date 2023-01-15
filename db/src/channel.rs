use ::entity::{channel, channel::Entity as Channel, team};
use chrono::{FixedOffset, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::*;

pub struct ChannelQuery;

impl ChannelQuery {
    pub async fn find(db: &DbConn) -> Result<Vec<channel::Model>, DbErr> {
        Channel::find().all(db).await
    }

    pub async fn find_by_id(db: &DbConn, id: i64) -> Result<Option<channel::Model>, DbErr> {
        Channel::find_by_id(id).one(db).await
    }

    pub async fn save(db: &DbConn, data: channel::Model) -> Result<channel::ActiveModel, DbErr> {
        let team_data = team::ActiveModel {
            user_id: Set(1.to_owned()),
            role_id: Set(1.to_owned()),
            created_at: Set(Utc::now()
                .with_timezone(&FixedOffset::east_opt(0).unwrap())
                .to_owned()),
            updated_at: Set(Utc::now()
                .with_timezone(&FixedOffset::east_opt(0).unwrap())
                .to_owned()),
            ..Default::default()
        }
        .save(db)
        .await?
        .try_into_model()?;

        channel::ActiveModel {
            team_id: Set(team_data.id.to_owned()),
            name: Set(data.name.to_owned()),
            description: Set(data.description.to_owned()),
            miniature: Set(data.miniature.to_owned()),
            background: Set(data.background.to_owned()),
            language_id: Set(data.language_id.to_owned()),
            instruments: Set(data.instruments.to_owned()),
            styles: Set(data.styles.to_owned()),
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
