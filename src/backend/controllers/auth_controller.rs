use actix_web::{post, HttpResponse, Responder, web};
use crate::services::auth_service::AuthService;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

#[post("/api/auth/login")]
async fn login(auth_data: web::Json<AuthData>) -> impl Responder {
    match AuthService::login(&auth_data.username, &auth_data.password) {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(err) => HttpResponse::Unauthorized().json(err),
    }
}

#[post("/api/auth/register")]
async fn register(auth_data: web::Json<AuthData>) -> impl Responder {
    match AuthService::register(&auth_data.username, &auth_data.username, &auth_data.password) {
        Ok(_) => HttpResponse::Created().json("User registered"),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

pub fn init_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
}
