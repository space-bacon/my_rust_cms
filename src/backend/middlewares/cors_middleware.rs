use actix_cors::Cors;
use actix_web::http;

pub fn cors_middleware() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:8080")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}
