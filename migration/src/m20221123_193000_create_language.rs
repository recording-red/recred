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

        // LanguageLocal
        manager
            .create_table(
                Table::create()
                    .table(LanguageLocal::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LanguageLocal::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LanguageLocal::LanguageId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(LanguageLocal::LocalId).integer().not_null())
                    .col(ColumnDef::new(LanguageLocal::Name).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-language-language_id")
                            .from(LanguageLocal::Table, LanguageLocal::LanguageId)
                            .to(Language::Table, Language::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-language-local_id")
                            .from(LanguageLocal::Table, LanguageLocal::LocalId)
                            .to(Language::Table, Language::Id),
                    )
                    .to_owned(),
            )
            .await?;

        let insert_language = Query::insert()
            .into_table(Language::Table)
            .columns([Language::Name])
            .values_panic(["English".into()])
            .values_panic(["French".into()])
            .values_panic(["Japanese".into()])
            .values_panic(["German".into()])
            .to_owned();
        manager.exec_stmt(insert_language).await?;

        let insert_language_local = Query::insert()
            .into_table(LanguageLocal::Table)
            .columns([
                LanguageLocal::LanguageId,
                LanguageLocal::LocalId,
                LanguageLocal::Name,
            ])
            //english
            .values_panic([1.into(), 1.into(), "English".into()])
            .values_panic([2.into(), 1.into(), "French".into()])
            .values_panic([3.into(), 1.into(), "Japanese".into()])
            .values_panic([4.into(), 1.into(), "German".into()])
            //french
            .values_panic([1.into(), 2.into(), "Anglais".into()])
            .values_panic([2.into(), 2.into(), "Français".into()])
            .values_panic([3.into(), 2.into(), "Japonais".into()])
            .values_panic([4.into(), 2.into(), "Allemand".into()])
            //japanese
            .values_panic([1.into(), 3.into(), "英語".into()])
            .values_panic([2.into(), 3.into(), "フランス語".into()])
            .values_panic([3.into(), 3.into(), "日本語".into()])
            .values_panic([4.into(), 3.into(), "ドイツ語".into()])
            //german
            .values_panic([1.into(), 4.into(), "Englisch".into()])
            .values_panic([2.into(), 4.into(), "Französisch".into()])
            .values_panic([3.into(), 4.into(), "Japanisch".into()])
            .values_panic([4.into(), 4.into(), "Deutsch".into()])
            .to_owned();
        manager.exec_stmt(insert_language_local).await
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
    LocalId,
    Name,
}
