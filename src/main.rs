use std::env;

use actix_cors::Cors;
use actix_web::web::ServiceConfig;
use actix_web::{middleware::Logger, web};
use migration::{Migrator, MigratorTrait};
use sea_orm::sqlx::PgPool;
use sea_orm::{DatabaseConnection, SqlxPostgresConnector};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;

mod api;

#[derive(Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    let frontend_origin = secrets
        .get("FRONTEND_ORIGIN")
        .expect("FRONTEND_ORIGIN was not found");

    let conn = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
    Migrator::up(&conn, None).await.unwrap();
    let state = AppState { conn };

    let config = move |cfg: &mut ServiceConfig| {
        let logger = Logger::default();
        let cors = Cors::default()
            .allowed_origin(&frontend_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec!["Content-Type"])
            .max_age(3600);
        cfg.service(
            web::scope("/api")
                .wrap(logger)
                .wrap(cors)
                .service(api::days::list_days)
                .service(api::days::create_day)
                .service(api::days::get_day)
                .service(api::ping::ping),
        )
        .app_data(web::Data::new(state.clone()));
    };
    Ok(config.into())
}
