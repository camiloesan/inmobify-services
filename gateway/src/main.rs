<<<<<<< HEAD
use actix_cors::Cors;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, Responder, http::Method, web};
use reqwest::{Client, RequestBuilder};
use serde_json::Value;
=======
use actix_web::{
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder, http::Method as ActixMethod, web,
};
use reqwest::{
    Client, Method as ReqwestMethod, header::HeaderMap as ReqwestHeaderMap, header::HeaderName,
    header::HeaderValue,
};
>>>>>>> 877e630357016ba8099fe9589fbff8b3bc4aefae

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("API Gateway is running!")
}

async fn proxy_to_service(
    client: web::Data<Client>,
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Bytes,
    service_port: u16,
) -> Result<HttpResponse, Error> {
    let service_url = format!("http://localhost:{}/{}", service_port, path.into_inner());
    let method = req.method().clone();

    let reqwest_method = match method {
        ActixMethod::GET => ReqwestMethod::GET,
        ActixMethod::POST => ReqwestMethod::POST,
        ActixMethod::PUT => ReqwestMethod::PUT,
        ActixMethod::DELETE => ReqwestMethod::DELETE,
        ActixMethod::HEAD => ReqwestMethod::HEAD,
        ActixMethod::OPTIONS => ReqwestMethod::OPTIONS,
        ActixMethod::CONNECT => ReqwestMethod::CONNECT,
        ActixMethod::PATCH => ReqwestMethod::PATCH,
        ActixMethod::TRACE => ReqwestMethod::TRACE,
        _ => return Ok(HttpResponse::MethodNotAllowed().body("Method not supported")),
    };

    let mut reqwest_headers = ReqwestHeaderMap::new();
    for (key, value) in req.headers() {
        if let Ok(header_name) = HeaderName::from_bytes(key.as_ref()) {
            if let Ok(header_value) = HeaderValue::from_bytes(value.as_bytes()) {
                reqwest_headers.insert(header_name, header_value);
            }
        }
    }

    let request_builder = client
        .request(reqwest_method, service_url)
        .headers(reqwest_headers)
        .body(body);

    let response = request_builder
        .send()
        .await
        .expect("could not send request");

    let mut builder = HttpResponse::build(response.status());

    for (key, value) in response.headers() {
        if !key.as_str().starts_with("connection") {
            if let Ok(header_value) =
                actix_web::http::header::HeaderValue::from_bytes(value.as_bytes())
            {
                builder.insert_header((key.as_str(), header_value));
            }
        }
    }

    match response.bytes().await {
        Ok(bytes) => Ok(builder.body(bytes)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
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

async fn proxy_auth(
    client: web::Data<Client>,
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Bytes,
) -> Result<impl Responder, Error> {
    proxy_to_service(client, path, req, body, 12002).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::new();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .app_data(web::Data::new(client.clone()))
<<<<<<< HEAD
            .wrap(cors)// Share the HTTP client across requests
=======
>>>>>>> 877e630357016ba8099fe9589fbff8b3bc4aefae
            .route("/health", web::get().to(health_check))
            .route(
                "/imf-properties/{path:.*}",
                web::route().to(proxy_properties),
            )
            .route("/imf-users/{path:.*}", web::route().to(proxy_users))
            .route("/imf-payments/{path:.*}", web::route().to(proxy_payments))
            .route(
                "/imf-appointments/{path:.*}",
                web::route().to(proxy_appointments),
            )
            .route("/imf-auth/{path:.*}", web::route().to(proxy_auth))
    })
    .bind("0.0.0.0:12000")?
    .run()
    .await
}
