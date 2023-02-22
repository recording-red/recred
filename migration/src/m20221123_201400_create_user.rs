use crate::m20221123_193000_create_language::Language;
use crate::m20221123_193002_create_role::Role;
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
                            .char_len(11)
                            .not_null()
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
            .unwrap();

        // Team
        manager
            .create_table(
                Table::create()
                    .table(Team::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Team::Id)
                            .char_len(11)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Team::UserId).char_len(11).not_null())
                    .col(ColumnDef::new(Team::RoleId).integer().not_null())
                    .col(
                        ColumnDef::new(Team::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Team::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-team-user_id")
                            .from(Team::Table, Team::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-team-role_id")
                            .from(Team::Table, Team::RoleId)
                            .to(Role::Table, Role::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Channel
        manager
            .create_table(
                Table::create()
                    .table(Channel::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Channel::Id)
                            .char_len(11)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Channel::TeamId).char_len(11).not_null())
                    .col(ColumnDef::new(Channel::Name).string().null())
                    .col(ColumnDef::new(Channel::Description).string().null())
                    .col(
                        ColumnDef::new(Channel::Miniature)
                            .blob(BlobSize::Blob(None))
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Channel::Background)
                            .blob(BlobSize::Blob(None))
                            .null(),
                    )
                    .col(ColumnDef::new(Channel::LanguageId).integer().default(2))
                    .col(
                        ColumnDef::new(Channel::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Channel::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-channel-team_id")
                            .from(Channel::Table, Channel::TeamId)
                            .to(Team::Table, Team::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-channel-language_id")
                            .from(Channel::Table, Channel::LanguageId)
                            .to(Language::Table, Language::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Channel::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Team::Table).to_owned())
            .await?;
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
enum Team {
    Table,
    Id,
    UserId,
    RoleId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Channel {
    Table,
    Id,
    TeamId,
    Name,
    Description,
    Miniature,
    Background,
    LanguageId,
    CreatedAt,
    UpdatedAt,
}
