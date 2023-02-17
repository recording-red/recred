use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Style::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Style::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Style::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        let insert = Query::insert()
            .into_table(Style::Table)
            .columns([Style::Name])
            .values_panic(["jazz".into()])
            .values_panic(["blues".into()])
            .values_panic(["rock".into()])
            .values_panic(["hard rock".into()])
            .values_panic(["metal".into()])
            .values_panic(["funk".into()])
            .values_panic(["reggae".into()])
            .values_panic(["country".into()])
            .values_panic(["pop".into()])
            .values_panic(["hip hop".into()])
            .values_panic(["neo soul".into()])
            .values_panic(["ska".into()])
            .to_owned();

        manager.exec_stmt(insert).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Style::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden

#[derive(Iden)]
pub enum Style {
    Table,
    Id,
    Name,
}
