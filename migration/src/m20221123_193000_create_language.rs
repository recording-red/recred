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
            .values_panic(["Spanish".into()])
            .values_panic(["Italian".into()])
            .values_panic(["Arabic".into()])
            .values_panic(["Russian".into()])
            .values_panic(["Portuguese".into()])
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
            .values_panic([5.into(), 1.into(), "Spanish".into()])
            .values_panic([6.into(), 1.into(), "Italian".into()])
            .values_panic([7.into(), 1.into(), "Arabic".into()])
            .values_panic([8.into(), 1.into(), "Russian".into()])
            .values_panic([9.into(), 1.into(), "Portuguese".into()])
            //french
            .values_panic([1.into(), 2.into(), "Anglais".into()])
            .values_panic([2.into(), 2.into(), "Français".into()])
            .values_panic([3.into(), 2.into(), "Japonais".into()])
            .values_panic([4.into(), 2.into(), "Allemand".into()])
            .values_panic([5.into(), 2.into(), "Espagnol".into()])
            .values_panic([6.into(), 2.into(), "Italien".into()])
            .values_panic([7.into(), 2.into(), "Arabe".into()])
            .values_panic([8.into(), 2.into(), "Russe".into()])
            .values_panic([9.into(), 2.into(), "Portuguais".into()])
            //japanese
            .values_panic([1.into(), 3.into(), "英語".into()])
            .values_panic([2.into(), 3.into(), "フランス語".into()])
            .values_panic([3.into(), 3.into(), "日本語".into()])
            .values_panic([4.into(), 3.into(), "ドイツ語".into()])
            .values_panic([5.into(), 3.into(), "スペイン語".into()])
            .values_panic([6.into(), 3.into(), "イタリア語".into()])
            .values_panic([7.into(), 3.into(), "アラビア語".into()])
            .values_panic([8.into(), 3.into(), "ロシア語".into()])
            .values_panic([9.into(), 3.into(), "ポルトガル語".into()])
            //german
            .values_panic([1.into(), 4.into(), "Englisch".into()])
            .values_panic([2.into(), 4.into(), "Französisch".into()])
            .values_panic([3.into(), 4.into(), "Japanisch".into()])
            .values_panic([4.into(), 4.into(), "Deutsch".into()])
            .values_panic([5.into(), 4.into(), "Spanisch".into()])
            .values_panic([6.into(), 4.into(), "Italienisch".into()])
            .values_panic([7.into(), 4.into(), "Arabisch".into()])
            .values_panic([8.into(), 4.into(), "Russisch".into()])
            .values_panic([9.into(), 4.into(), "Portugiesisch".into()])
            //spanish
            .values_panic([1.into(), 5.into(), "inglés".into()])
            .values_panic([2.into(), 5.into(), "francés".into()])
            .values_panic([3.into(), 5.into(), "japonés".into()])
            .values_panic([4.into(), 5.into(), "alemán".into()])
            .values_panic([5.into(), 5.into(), "español".into()])
            .values_panic([6.into(), 5.into(), "italiano".into()])
            .values_panic([7.into(), 5.into(), "arábica".into()])
            .values_panic([8.into(), 5.into(), "ruso".into()])
            .values_panic([9.into(), 5.into(), "portugués".into()])
            //italian
            .values_panic([1.into(), 6.into(), "inglese".into()])
            .values_panic([2.into(), 6.into(), "francese".into()])
            .values_panic([3.into(), 6.into(), "giapponese".into()])
            .values_panic([4.into(), 6.into(), "tedesco".into()])
            .values_panic([5.into(), 6.into(), "spagnolo".into()])
            .values_panic([6.into(), 6.into(), "italiano".into()])
            .values_panic([7.into(), 6.into(), "arabo".into()])
            .values_panic([8.into(), 6.into(), "russo".into()])
            .values_panic([9.into(), 6.into(), "portoghese".into()])
            //arabic
            .values_panic([1.into(), 7.into(), "إنجليزي".into()])
            .values_panic([2.into(), 7.into(), "فرنسي".into()])
            .values_panic([3.into(), 7.into(), "اليابانية".into()])
            .values_panic([4.into(), 7.into(), "ألمانية".into()])
            .values_panic([5.into(), 7.into(), "الأسبانية".into()])
            .values_panic([6.into(), 7.into(), "الايطالية".into()])
            .values_panic([7.into(), 7.into(), "عربي".into()])
            .values_panic([8.into(), 7.into(), "الروسية".into()])
            .values_panic([9.into(), 7.into(), "البرتغالية".into()])
            //russian
            .values_panic([1.into(), 8.into(), "английский".into()])
            .values_panic([2.into(), 8.into(), "французский".into()])
            .values_panic([3.into(), 8.into(), "Японский".into()])
            .values_panic([4.into(), 8.into(), "Немецкий".into()])
            .values_panic([5.into(), 8.into(), "испанский".into()])
            .values_panic([6.into(), 8.into(), "итальянский".into()])
            .values_panic([7.into(), 8.into(), "арабский".into()])
            .values_panic([8.into(), 8.into(), "русский".into()])
            .values_panic([9.into(), 8.into(), "португальский".into()])
            //portuguese
            .values_panic([1.into(), 9.into(), "inglês".into()])
            .values_panic([2.into(), 9.into(), "francesa".into()])
            .values_panic([3.into(), 9.into(), "japonês".into()])
            .values_panic([4.into(), 9.into(), "alemão".into()])
            .values_panic([5.into(), 9.into(), "espanhol".into()])
            .values_panic([6.into(), 9.into(), "italiano".into()])
            .values_panic([7.into(), 9.into(), "árabe".into()])
            .values_panic([8.into(), 9.into(), "russo".into()])
            .values_panic([9.into(), 9.into(), "português".into()])
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
