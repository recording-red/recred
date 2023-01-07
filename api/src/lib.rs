pub mod errors;
pub mod router;

use crate::router::{health, registration, user};
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use db::sea_orm::{Database, DatabaseConnection};
use std::env;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().finish()
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    // get env vars
    // should change that to use env var and not .env, but good enough for poc
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    // establish connection to database and apply migrations
    // should defo be in a separate bin, but good enough for the poc
    let conn = Database::connect(&db_url).await.unwrap();
    // Migrator::up(&conn, None).await.unwrap();

    // Starting the server
    let state = AppState { conn };
    let _server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(state.clone()))
            .default_service(web::route().to(not_found))
            .service(web::scope("/health").service(health::service))
            .service(
                web::scope("/registration")
                    .service(user::post)
                    .service(user::get),
            )
            .service(
                web::scope("/user")
                    .service(registration::post)
                    .service(registration::get),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}
