//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
 
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize)]
#[sea_orm(table_name = "language")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::channel::Entity")]
    Channel,
}

impl Related<super::channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Channel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
