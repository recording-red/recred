use crate::m20221123_193000_create_language::Language;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Instrument
        manager
            .create_table(
                Table::create()
                    .table(Instrument::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Instrument::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Instrument::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(InstrumentLocal::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(InstrumentLocal::Id).integer().not_null())
                    .col(
                        ColumnDef::new(InstrumentLocal::LanguageId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(InstrumentLocal::Name).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-instrument_local-id")
                            .from(InstrumentLocal::Table, InstrumentLocal::Id)
                            .to(Instrument::Table, Instrument::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-instrument_local-language_id")
                            .from(InstrumentLocal::Table, InstrumentLocal::LanguageId)
                            .to(Language::Table, Language::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(InstrumentLocal::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Instrument::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden

#[derive(Iden)]
enum Instrument {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum InstrumentLocal {
    Table,
    Id,
    LanguageId,
    Name,
}
