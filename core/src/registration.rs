use ::entity::{registration, registration::Entity as Registration};
use chrono::{FixedOffset, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::*;

pub struct RegistrationQuery;

impl RegistrationQuery {
    pub async fn find(db: &DbConn) -> Result<Vec<registration::Model>, DbErr> {
        Registration::find().all(db).await
    }

    pub async fn create(
        db: &DbConn,
        data: registration::Model,
    ) -> Result<registration::ActiveModel, DbErr> {
        registration::ActiveModel {
            email: Set(data.email.to_owned()),
            created_at: Set(Utc::now().with_timezone(&FixedOffset::east(0)).to_owned()),
            ip: Set(data.ip.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
}
