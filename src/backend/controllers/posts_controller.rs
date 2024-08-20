use actix_web::{get, post, put, delete, HttpResponse, Responder, web};
use crate::services::post_service::PostService;
use crate::models::post::Post;

#[post("/api/posts")]
async fn create_post(post_data: web::Json<Post>) -> impl Responder {
    match PostService::create_post(post_data.into_inner()) {
        Ok(post) => HttpResponse::Created().json(post),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[get("/api/posts")]
async fn get_all_posts() -> impl Responder {
    match PostService::list_posts() {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

#[get("/api/posts/{id}")]
async fn get_post(id: web::Path<i32>) -> impl Responder {
    match PostService::get_post(id.into_inner()) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::NotFound().json(err),
    }
}

#[put("/api/posts/{id}")]
async fn update_post(id: web::Path<i32>, post_data: web::Json<Post>) -> impl Responder {
    match PostService::update_post(id.into_inner(), post_data.into_inner()) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[delete("/api/posts/{id}")]
async fn delete_post(id: web::Path<i32>) -> impl Responder {
    match PostService::delete_post(id.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Post deleted"),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub fn init_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(create_post);
    cfg.service(get_all_posts);
    cfg.service(get_post);
    cfg.service(update_post);
    cfg.service(delete_post);
}
