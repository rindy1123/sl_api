use std::env;

use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

mod api;

#[derive(Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_test_writer().init();
    let db_url = env::var("DATABASE_URL").unwrap();
    let host = env::var("API_HOST").unwrap();
    let port = env::var("API_PORT").unwrap();
    let address = format!("{host}:{port}");

    let conn = Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    let state = AppState { conn };

    info!("Starting server at: {}", address);
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(api::personal_info::list_personal_info)
            .service(api::personal_info::create_personal_info)
            .app_data(web::Data::new(state.clone()))
    })
    .bind(address)?
    .run()
    .await
}
