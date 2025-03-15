use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, Responder, http::Method, web};
use reqwest::{Client, RequestBuilder};
use serde_json::Value;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("API Gateway is running!")
}

/// Forwards requests to services on localhost
/// On POST and PUT converts the received bytes to JSON
async fn proxy_to_service(
    client: web::Data<Client>,
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Bytes,
    service_port: u16,
) -> Result<impl Responder, Error> {
    let service_url = format!("http://localhost:{}/{}", service_port, path.into_inner());

    // Build the request based on the HTTP method
    let method = req.method();
    let request_builder: RequestBuilder = match *method {
        Method::GET => client.get(&service_url),
        Method::POST => {
            let json_body: Value = serde_json::from_slice(&body)
                .map_err(actix_web::error::ErrorBadRequest)?;
            client.post(&service_url).json(&json_body)
        },
        Method::PUT => {
            let json_body: Value = serde_json::from_slice(&body)
                .map_err(actix_web::error::ErrorBadRequest)?;
            client
                .put(&service_url)
                .json(&json_body)
        },
        Method::DELETE => client.delete(&service_url),
        _ => return Ok(HttpResponse::MethodNotAllowed().body("Method not supported")),
    };

    let response = request_builder
        .send()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let status = response.status();
    let body: Value = response
        .json()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::build(status).json(body))
}

// Handlers for each service
async fn proxy_properties(
    client: web::Data<Client>,
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Bytes,
) -> Result<impl Responder, Error> {
    proxy_to_service(client, path, req, body, 12004).await
}

async fn proxy_users(
    client: web::Data<Client>,
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Bytes,
) -> Result<impl Responder, Error> {
    proxy_to_service(client, path, req, body, 12005).await
}

async fn proxy_payments(
    client: web::Data<Client>,
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Bytes,
) -> Result<impl Responder, Error> {
    proxy_to_service(client, path, req, body, 12003).await
}

async fn proxy_appointments(
    client: web::Data<Client>,
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Bytes,
) -> Result<impl Responder, Error> {
    proxy_to_service(client, path, req, body, 12001).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::new(); // HTTP client for proxying requests

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone())) // Share the HTTP client across requests
            .route("/health", web::get().to(health_check))
            .route("/imf-properties/{path:.*}", web::route().to(proxy_properties))
            .route("/imf-users/{path:.*}", web::route().to(proxy_users))
            .route("/imf-payments/{path:.*}", web::route().to(proxy_payments))
            .route(
                "/imf-appointments/{path:.*}",
                web::route().to(proxy_appointments),
            )
    })
    .bind("0.0.0.0:12000")?
    .run()
    .await
}
