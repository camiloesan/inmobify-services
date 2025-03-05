use actix_web::App;
use actix_cors::Cors;
use actix_web::HttpServer;
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .service(fs::Files::new("/images", "/images"))
            .wrap(cors)
    })
    .bind("0.0.0.0:12000")?
    .run()
    .await
}
