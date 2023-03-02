use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Role::Id).string().not_null().primary_key())
                    .to_owned(),
            )
            .await?;

        let insert = Query::insert()
            .into_table(Role::Table)
            .columns([Role::Id])
            .values_panic(["owner".into()])
            .values_panic(["admin".into()])
            .values_panic(["moderator".into()])
            .values_panic(["publisher".into()])
            .to_owned();

        manager.exec_stmt(insert).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden

#[derive(Iden)]
pub enum Role {
    Table,
    Id,
}
