pub use sea_orm_migration::prelude::*;

mod m20221123_192127_registration;
mod m20221123_201400_create_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221123_192127_registration::Migration),
            Box::new(m20221123_201400_create_user::Migration),
        ]
    }
}
