mod dal;
mod dto;
mod routes;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use diesel::{prelude::*, r2d2};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use env_logger::Env;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

/// Run diesel migrations at compile-time
fn run_migrations() {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::establish(&url).expect("Failed to establish connection");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Couldn't migrate tables");
}

/// Initialize database connection pool based on `DATABASE_URL` environment variable.
fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to postgresql server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    run_migrations();

    let pool = initialize_db_pool();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        #[derive(OpenApi)]
        #[openapi(
            paths(routes::fetch_boosted_properties),
            components(schemas(dto::property_summary::PropertySummary))
        )]
        struct ApiDoc;
        let openapi = ApiDoc::openapi();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(routes::fetch_boosted_properties)
            .service(routes::fetch_property_details)
            .service(routes::create_property)
            .service(routes::update_img_path)
            .service(routes::get_states)
            .service(routes::get_user_properties)
            .service(routes::delete_property)
            .wrap(cors)
    })
    .bind("0.0.0.0:12004")?
    .run()
    .await
}
