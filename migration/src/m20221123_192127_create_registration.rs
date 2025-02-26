use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Registration::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Registration::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Registration::Email).string().not_null())
                    .col(
                        ColumnDef::new(Registration::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Registration::Ip).string())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_index(
                Index::create()
                    .name("idx-registration-unique-email")
                    .table(Registration::Table)
                    .col(Registration::Email)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Registration::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Registration {
    Table,
    Id,
    Email,
    CreatedAt,
    Ip,
}
