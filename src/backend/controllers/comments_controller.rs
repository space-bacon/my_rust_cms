use actix_web::{get, post, put, delete, HttpResponse, Responder, web};
use crate::services::comment_service::CommentService;
use crate::models::comment::Comment;

#[post("/api/comments")]
async fn create_comment(comment_data: web::Json<Comment>) -> impl Responder {
    match CommentService::create_comment(comment_data.into_inner()) {
        Ok(comment) => HttpResponse::Created().json(comment),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[get("/api/comments")]
async fn get_all_comments() -> impl Responder {
    match CommentService::list_comments() {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

#[get("/api/comments/{id}")]
async fn get_comment(id: web::Path<i32>) -> impl Responder {
    match CommentService::get_comment(id.into_inner()) {
        Ok(comment) => HttpResponse::Ok().json(comment),
        Err(err) => HttpResponse::NotFound().json(err),
    }
}

#[put("/api/comments/{id}")]
async fn update_comment(id: web::Path<i32>, comment_data: web::Json<Comment>) -> impl Responder {
    match CommentService::update_comment(id.into_inner(), comment_data.into_inner()) {
        Ok(comment) => HttpResponse::Ok().json(comment),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[delete("/api/comments/{id}")]
async fn delete_comment(id: web::Path<i32>) -> impl Responder {
    match CommentService::delete_comment(id.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Comment deleted"),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub fn init_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(create_comment);
    cfg.service(get_all_comments);
    cfg.service(get_comment);
    cfg.service(update_comment);
    cfg.service(delete_comment);
}
