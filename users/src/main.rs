mod dal;
mod dto_models;
mod routes;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dal::db_operations::PgUsers;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use env_logger::Env;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn do_migrations() {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::establish(&url).expect("Failed to establish connection");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Couldn't migrate tables");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    do_migrations();

    let repo: PgUsers = PgUsers::new(url.clone());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        #[derive(OpenApi)]
        #[openapi(
            paths(
                routes::create_user,
                routes::get_user_by_uuid,
                routes::delete_user_by_uuid
            ),
            components(schemas(dto_models::User, dto_models::NewUser))
        )]
        struct ApiDoc;
        let openapi = ApiDoc::openapi();

        App::new()
            .app_data(web::Data::new(repo.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(routes::create_user)
            .service(routes::get_user_by_uuid)
            .wrap(cors)
    })
    .bind("0.0.0.0:12000")?
    .run()
    .await
}
