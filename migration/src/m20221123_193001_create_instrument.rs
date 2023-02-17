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
                    .col(
                        ColumnDef::new(InstrumentLocal::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(InstrumentLocal::InstrumentId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InstrumentLocal::LanguageId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(InstrumentLocal::Name).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-instrument-instrument_id")
                            .from(InstrumentLocal::Table, InstrumentLocal::InstrumentId)
                            .to(Instrument::Table, Instrument::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-instrument-language_id")
                            .from(InstrumentLocal::Table, InstrumentLocal::LanguageId)
                            .to(Language::Table, Language::Id),
                    )
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
        manager.exec_stmt(insert_instrument).await?;

        let insert_instrument_local = Query::insert()
            .into_table(InstrumentLocal::Table)
            .columns([
                InstrumentLocal::InstrumentId,
                InstrumentLocal::LanguageId,
                InstrumentLocal::Name,
            ])
            //english
            .values_panic([1.into(), 1.into(), "electric guitar".into()])
            .values_panic([2.into(), 1.into(), "acoustic guitar".into()])
            .values_panic([3.into(), 1.into(), "bass guitar".into()])
            .values_panic([4.into(), 1.into(), "drums".into()])
            .values_panic([5.into(), 1.into(), "voice".into()])
            .values_panic([6.into(), 1.into(), "piano".into()])
            .values_panic([7.into(), 1.into(), "violin".into()])
            //french
            .values_panic([1.into(), 2.into(), "guitare électrique".into()])
            .values_panic([2.into(), 2.into(), "guitare accoustique".into()])
            .values_panic([3.into(), 2.into(), "guitare basse".into()])
            .values_panic([3.into(), 2.into(), "batterie".into()])
            .values_panic([5.into(), 2.into(), "voix".into()])
            .values_panic([6.into(), 2.into(), "piano".into()])
            .values_panic([7.into(), 2.into(), "violon".into()])
            //japanese
            .values_panic([1.into(), 3.into(), "エレクトリックギター".into()])
            .values_panic([2.into(), 3.into(), "アコースティックギター".into()])
            .values_panic([3.into(), 3.into(), "バス".into()])
            .values_panic([4.into(), 3.into(), "ドラム".into()])
            .values_panic([5.into(), 3.into(), "声".into()])
            .values_panic([6.into(), 3.into(), "ピアノ".into()])
            .values_panic([7.into(), 3.into(), "バイオリン".into()])
            //german
            .values_panic([1.into(), 4.into(), "E-Gitarre".into()])
            .values_panic([2.into(), 4.into(), "Gitarre".into()])
            .values_panic([3.into(), 4.into(), "Bassgitarre".into()])
            .values_panic([4.into(), 4.into(), "Schlagzeug".into()])
            .values_panic([5.into(), 4.into(), "Stimme".into()])
            .values_panic([6.into(), 4.into(), "Klavier".into()])
            .values_panic([7.into(), 4.into(), "Violine".into()])
            //spanish
            .values_panic([1.into(), 5.into(), "guitarra electrica".into()])
            .values_panic([2.into(), 5.into(), "guitarra acustica".into()])
            .values_panic([3.into(), 5.into(), "bajo".into()])
            .values_panic([4.into(), 5.into(), "batería".into()])
            .values_panic([5.into(), 5.into(), "voz".into()])
            .values_panic([6.into(), 5.into(), "piano".into()])
            .values_panic([7.into(), 5.into(), "violín".into()])
            //italian
            .values_panic([1.into(), 6.into(), "chitarra elettrica".into()])
            .values_panic([2.into(), 6.into(), "chitarra acustica".into()])
            .values_panic([3.into(), 6.into(), "basso".into()])
            .values_panic([4.into(), 6.into(), "batteria".into()])
            .values_panic([5.into(), 6.into(), "voce".into()])
            .values_panic([6.into(), 6.into(), "pianoforte".into()])
            .values_panic([7.into(), 6.into(), "violino".into()])
            //arabic
            .values_panic([1.into(), 7.into(), "الجيتار الكهربائي".into()])
            .values_panic([2.into(), 7.into(), "الغيتار الصوتي".into()])
            .values_panic([3.into(), 7.into(), "قيثارة ذات صوت جهير".into()])
            .values_panic([4.into(), 7.into(), "طبول".into()])
            .values_panic([5.into(), 7.into(), "صوت".into()])
            .values_panic([6.into(), 7.into(), "بيانو".into()])
            .values_panic([7.into(), 7.into(), "كمان".into()])
            //russian
            .values_panic([1.into(), 8.into(), "Электрогитара".into()])
            .values_panic([2.into(), 8.into(), "акустическая гитара".into()])
            .values_panic([3.into(), 8.into(), "бас-гитара".into()])
            .values_panic([4.into(), 8.into(), "барабаны".into()])
            .values_panic([5.into(), 8.into(), "голос".into()])
            .values_panic([6.into(), 8.into(), "пианино".into()])
            .values_panic([7.into(), 8.into(), "скрипка".into()])
            //portuguese
            .values_panic([1.into(), 9.into(), "guitarra elétrica".into()])
            .values_panic([2.into(), 9.into(), "violão".into()])
            .values_panic([3.into(), 9.into(), "baixo".into()])
            .values_panic([4.into(), 9.into(), "bateria".into()])
            .values_panic([5.into(), 9.into(), "voz".into()])
            .values_panic([6.into(), 9.into(), "piano".into()])
            .values_panic([7.into(), 9.into(), "violino".into()])
            .to_owned();
        manager.exec_stmt(insert_instrument_local).await
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
    InstrumentId,
    LanguageId,
    Name,
}
