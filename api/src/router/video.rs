use crate::AppState;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use bytes::Bytes;
use futures_util::StreamExt as _;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use uuid::Uuid;

#[post("/")]
async fn upload(mut body: web::Payload) -> actix_web::Result<String> {
    // for demonstration only; in a normal case use the `Bytes` extractor
    // collect payload stream into a bytes object
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    let id = Uuid::new_v4();
    let mut f = File::create(format!("/tmp/recred/{}", id))?;
    f.write_all(&Bytes::from(bytes))?;

    //Ok(format!("Request Body Bytes:\n{:?}", bytes))
    Ok(format!("Request Body Bytes:\n"))
}

//#[post("/")]
//async fn upload(
//    data: web::Data<AppState>,
//    //req: HttpRequest,
//    body: web::Bytes
//) -> impl Responder {
//    println!("youpi {:?}", body);
//    HttpResponse::Ok()
//        .finish()
//}
