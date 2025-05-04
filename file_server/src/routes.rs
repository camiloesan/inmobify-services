use actix_multipart::Multipart;
use actix_web::web;
use actix_web::HttpResponse;
use futures::StreamExt;
use serde::Deserialize;
use std::fs;
use std::io::Write;

#[actix_web::post("/upload/{property_uuid}")]
async fn upload(
    path: web::Path<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, actix_web::Error> {
    log::info!("request to upload file received");

    let property_uuid = path.into_inner();
    let upload_dir = format!("images/{}", sanitize_filename(&property_uuid));
    fs::create_dir_all(&upload_dir)?;

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition().clone();
        let filename = content_disposition
            .get_filename()
            .map_or("unnamed".to_string(), |f| f.to_string());

        let sanitized_filename = format!("{}/{}", upload_dir, sanitize_filename(&filename));

        let mut f = fs::File::create(&sanitized_filename)?;
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f.write_all(&data)?;
        }
    }

    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
struct DeleteFilePayload {
    filename: String,
}

#[actix_web::delete("/file/{id}")]
async fn delete_file(
    id: web::Path<String>,
    data: web::Json<DeleteFilePayload>,
) -> Result<HttpResponse, actix_web::Error> {
    log::info!("request to delete file received");

    let basedir = id.into_inner();

    let file_path = format!("images/{}/{}", basedir, data.filename);
    fs::remove_file(file_path)?;

    // if directory is empty, delete it
    let dir_path = format!("images/{}", basedir);
    if fs::read_dir(&dir_path)?.next().is_none() {
        fs::remove_dir(dir_path)?;
    }

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::delete("/directory/{id}")]
async fn delete_directory(id: web::Path<String>) -> Result<HttpResponse, actix_web::Error> {
    log::info!("request to delete directory received");

    let basedir = id.into_inner();

    let dir_path = format!("images/{}", basedir);
    fs::remove_dir_all(dir_path)?;

    Ok(HttpResponse::Ok().finish())
}

fn sanitize_filename(filename: &str) -> String {
    filename
        .replace("..", "")
        .replace("/", "")
        .replace("\\", "")
}
