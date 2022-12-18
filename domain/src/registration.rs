use ::entity::regitration;
use db::registration::RegistrationQuery;
use sea_orm::error::DbErr;
use sea_orm::*;

pub async fn create(conn: &DbConn, data: registration::Model) -> Result<registration::ActiveModel, DbErr> {
    RegistrationQuery::create(conn, data).await
}

//async fn read() {}

//async fn read_all() {}

//async fn update()

//async fn delete() {}
