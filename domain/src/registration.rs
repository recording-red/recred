use ::entity::registration;
use db::registration::RegistrationQuery;
use sea_orm::error::DbErr;
use sea_orm::*;

pub async fn create(
    conn: &DbConn,
    data: registration::Model,
) -> Result<registration::Model, DbErr> {
    let obj = RegistrationQuery::save(conn, data).await?;
    obj.try_into_model()
}

//async fn read() {}

pub async fn read_all(db: &DbConn) -> Result<Vec<registration::Model>, DbErr> {
    RegistrationQuery::find(db).await
}

//async fn update()

//async fn delete() {}
