use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::Handle).string().not_null())
                    .col(ColumnDef::new(User::DisplayName).string().not_null())
                    .col(ColumnDef::new(User::FirstName).string())
                    .col(ColumnDef::new(User::LastName).string())
                    .col(ColumnDef::new(User::IsActive).boolean().default(false))
                    .col(ColumnDef::new(User::Password).string())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_index(
                Index::create()
                    .name("idx-unique-user-email")
                    .table(User::Table)
                    .col(User::Email)
                    .unique()
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_index(
                Index::create()
                    .name("idx-unique-user-handle")
                    .table(User::Table)
                    .col(User::Handle)
                    .unique()
                    .to_owned(),
            )
            .await

        // Channel
        //        manager
        //            .create_table(
        //                Table::create()
        //                    .table(Channel::Table)
        //                    .if_not_exists()
        //                    .col(
        //                        ColumnDef::new(Table::Id)
        //                            .integer()
        //                            .not_null()
        //                            .auto_increment()
        //                            .primary_key(),
        //                    )
        //                    .col(
        //                        ColumnDef::new(Channel::UserId)
        //                            .integer()
        //                            .not_null(),
        //                    ):
        //                    .foreign_key(
        //                        ForeignKey::create()
        //                            .name("fk-channel-user_id")
        //                            .from(LanguageLocal::Table, LanguageLocal::LocalId)
        //                            .to(Language::Table, Language::Id),
        //                    )
        //                    .to_owned(),
        //            )
        //        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    Id,
    Email,
    Handle,
    DisplayName,
    FirstName,
    LastName,
    IsActive,
    Password,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Channel {
    Table,
    Id,
    UserId,
    Miniature,
    Background,
    Description,
    Name,
    InstrumentIds,
    LanguageId,
    Styles,
}
