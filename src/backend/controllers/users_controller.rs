use actix_web::{get, post, put, delete, HttpResponse, Responder, web};
use crate::services::user_service::UserService;
use crate::models::user::User;

#[post("/api/users")]
async fn create_user(user_data: web::Json<User>) -> impl Responder {
    match UserService::create_user(user_data.into_inner()) {
        Ok(user) => HttpResponse::Created().json(user),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[get("/api/users")]
async fn get_all_users() -> impl Responder {
    match UserService::list_users() {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

#[get("/api/users/{id}")]
async fn get_user(id: web::Path<i32>) -> impl Responder {
    match UserService::get_user(id.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::NotFound().json(err),
    }
}

#[put("/api/users/{id}")]
async fn update_user(id: web::Path<i32>, user_data: web::Json<User>) -> impl Responder {
    match UserService::update_user(id.into_inner(), user_data.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[delete("/api/users/{id}")]
async fn delete_user(id: web::Path<i32>) -> impl Responder {
    match UserService::delete_user(id.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("User deleted"),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub fn init_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(create_user);
    cfg.service(get_all_users);
    cfg.service(get_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}
