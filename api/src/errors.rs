extern crate derive_more;
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

use sea_orm::error::DbErr;
use derive_more::{Display, Error};


#[derive(Debug, Display, Error)]
pub enum RecRedError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,

    #[display(fmt = "not found")]
    NotFound,
}

impl error::ResponseError for RecRedError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            RecRedError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            RecRedError::BadClientData => StatusCode::BAD_REQUEST,
            RecRedError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            RecRedError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl From<DbErr> for RecRedError {
    fn from(_error: DbErr) -> Self {
        RecRedError::InternalError
        //match error {
        //    DbErr::RecordNotFound => RecRedError::NotFound,
        //    DbErr::Exec => RecRedError::BadClientData,
        //    _ => RecRedError::InternalError
        //}
    }
}

