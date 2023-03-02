use crate::utils::generate_id;
use ::entity::{channel, channel::Entity as Channel};
use chrono::{FixedOffset, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::*;

pub struct ChannelQuery;

impl ChannelQuery {
    pub async fn find(db: &DbConn) -> Result<Vec<channel::Model>, DbErr> {
        Channel::find().all(db).await
    }

    pub async fn find_by_id(db: &DbConn, id: String) -> Result<Option<channel::Model>, DbErr> {
        Channel::find_by_id(id).one(db).await
    }

    pub async fn insert(db: &DbConn, data: channel::Model) -> Result<channel::Model, DbErr> {
        channel::ActiveModel {
            id: Set(generate_id().to_owned()),
            team_id: Set(data.team_id.to_owned()),
            name: Set(data.name.to_owned()),
            description: Set(data.description.to_owned()),
            banner: Set(data.banner.to_owned()),
            profile: Set(data.profile.to_owned()),
            language_id: Set(data.language_id.to_owned()),
            styles: Set(data.styles.to_owned()),
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

    pub async fn save(db: &DbConn, data: channel::Model) -> Result<channel::ActiveModel, DbErr> {
        channel::ActiveModel {
            id: Set(data.id.to_owned()),
            team_id: Set(data.team_id.to_owned()),
            name: Set(data.name.to_owned()),
            description: Set(data.description.to_owned()),
            banner: Set(data.banner.to_owned()),
            profile: Set(data.profile.to_owned()),
            language_id: Set(data.language_id.to_owned()),
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

    pub async fn save_banner(
        db: &DbConn,
        id: String,
        data: Option<Vec<u8>>,
    ) -> Result<channel::Model, DbErr> {
        let channel: Option<channel::Model> = Channel::find_by_id(id).one(db).await?;
        let mut channel: channel::ActiveModel = channel.unwrap().into();
        channel.banner = Set(data.to_owned());
        channel.updated_at = Set(Utc::now()
            .with_timezone(&FixedOffset::east_opt(0).unwrap())
            .to_owned());

        channel.update(db).await
    }

    pub async fn save_profile(
        db: &DbConn,
        id: String,
        data: Option<Vec<u8>>,
    ) -> Result<channel::Model, DbErr> {
        let channel: Option<channel::Model> = Channel::find_by_id(id).one(db).await?;
        let mut channel: channel::ActiveModel = channel.unwrap().into();
        channel.profile = Set(data.to_owned());
        channel.updated_at = Set(Utc::now()
            .with_timezone(&FixedOffset::east_opt(0).unwrap())
            .to_owned());

        channel.update(db).await
    }
}
