use ::entity::{user, user::Entity as User};
use chrono::{FixedOffset, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::*;

pub struct UserQuery;

impl UserQuery {
    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }

    pub async fn create(db: &DbConn, data: user::Model) -> Result<user::ActiveModel, DbErr> {
        user::ActiveModel {
            email: Set(data.email.to_owned()),
            password: Set(data.password.to_owned()),
            created_at: Set(Utc::now().with_timezone(&FixedOffset::east(0)).to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
}
