mod dal;
mod dto;
mod routes;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use diesel::{prelude::*, r2d2};
use dotenvy::dotenv;
use env_logger::Env;
use std::env;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

fn initialize_db_pool() -> DbPool {
    dotenv().ok();
    let conn_spec = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Database URL should be valid path to PostgreSQL server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::info!("Starting auth service");

    let pool = initialize_db_pool();
    log::info!("Database pool initialized");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .expose_headers(vec!["x-token"]);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .service(routes::login)
    })
    .bind("0.0.0.0:12002")?
    .run()
    .await
}
