use actix_web::{get, post, delete, HttpResponse, Responder, web};
use crate::services::media_service::MediaService;
use crate::models::media::Media;

#[post("/api/media")]
async fn upload_media(media_data: web::Json<Media>) -> impl Responder {
    match MediaService::upload_media(media_data.into_inner()) {
        Ok(media) => HttpResponse::Created().json(media),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[get("/api/media")]
async fn get_all_media() -> impl Responder {
    match MediaService::list_media() {
        Ok(media) => HttpResponse::Ok().json(media),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

#[get("/api/media/{id}")]
async fn get_media(id: web::Path<i32>) -> impl Responder {
    match MediaService::get_media(id.into_inner()) {
        Ok(media) => HttpResponse::Ok().json(media),
        Err(err) => HttpResponse::NotFound().json(err),
    }
}

#[delete("/api/media/{id}")]
async fn delete_media(id: web::Path<i32>) -> impl Responder {
    match MediaService::delete_media(id.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Media deleted"),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub fn init_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(upload_media);
    cfg.service(get_all_media);
    cfg.service(get_media);
    cfg.service(delete_media);
}
