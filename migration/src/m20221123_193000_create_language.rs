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
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await?;

        let insert_language = Query::insert()
            .into_table(Language::Table)
            .columns([Language::Id])
            .values_panic(["English".into()])
            .values_panic(["French".into()])
            .values_panic(["Japanese".into()])
            .values_panic(["German".into()])
            .values_panic(["Spanish".into()])
            .values_panic(["Italian".into()])
            .values_panic(["Arabic".into()])
            .values_panic(["Russian".into()])
            .values_panic(["Portuguese".into()])
            .to_owned();
        manager.exec_stmt(insert_language).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
}
