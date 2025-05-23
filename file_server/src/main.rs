use std::env;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::App;
use actix_web::HttpServer;
use env_logger::Env;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let images_path = concat!(env!("CARGO_MANIFEST_DIR"), "/images");

    HttpServer::new(move || {
        let relative_path = Files::new("/images", images_path).show_files_listing();
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .service(routes::upload)
            .service(routes::delete_file)
            .service(routes::delete_directory)
            .service(relative_path)
            .wrap(cors)
    })
    .bind("0.0.0.0:12006")?
    .run()
    .await
}
