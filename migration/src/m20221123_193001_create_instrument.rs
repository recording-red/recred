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

        let insert_instrument = Query::insert()
            .into_table(Instrument::Table)
            .columns([Instrument::Name])
            .values_panic(["electric guitar".into()])
            .values_panic(["acoustic guitar".into()])
            .values_panic(["bass guitar".into()])
            .values_panic(["drum".into()])
            .values_panic(["singing".into()])
            .values_panic(["piano".into()])
            .values_panic(["violin".into()])
            .to_owned();
        manager.exec_stmt(insert_instrument).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
