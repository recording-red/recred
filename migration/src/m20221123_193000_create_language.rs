use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // Language
        manager
            .create_table(
                Table::create()
                    .table(Language::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Language::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Language::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(LanguageLocal::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(LanguageLocal::Id).integer().not_null())
                    .col(ColumnDef::new(LanguageLocal::LanguageId).integer().not_null())
                    .col(ColumnDef::new(LanguageLocal::Name).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-language_local-id")
                            .from(LanguageLocal::Table, LanguageLocal::Id)
                            .to(Language::Table, Language::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-language_local-local_id")
                            .from(LanguageLocal::Table, LanguageLocal::LanguageId)
                            .to(Language::Table, Language::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LanguageLocal::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Language::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden

#[derive(Iden)]
pub enum Language {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum LanguageLocal {
    Table,
    Id,
    LanguageId,
    Name,
}
