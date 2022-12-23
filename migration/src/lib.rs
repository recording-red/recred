pub use sea_orm_migration::prelude::*;

mod m20221123_192127_create_registration;
mod m20221123_193000_create_language;
mod m20221123_193001_create_instrument;
mod m20221123_201400_create_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221123_192127_create_registration::Migration),
            Box::new(m20221123_193000_create_language::Migration),
            Box::new(m20221123_193001_create_instrument::Migration),
            Box::new(m20221123_201400_create_user::Migration),
        ]
    }
}
