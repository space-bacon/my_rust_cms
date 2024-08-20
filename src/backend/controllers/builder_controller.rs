use actix_web::{get, post, put, HttpResponse, Responder, web};
use crate::services::builder_service::{BuilderService, NewPageData};

#[post("/api/builder/pages")]
async fn create_page(new_page: web::Json<NewPageData>) -> impl Responder {
    match BuilderService::create_page(new_page.into_inner()) {
        Ok(page) => HttpResponse::Created().json(page),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[get("/api/builder/pages/{id}")]
async fn get_page(id: web::Path<i32>) -> impl Responder {
    match BuilderService::get_page(id.into_inner()) {
        Ok(page) => HttpResponse::Ok().json(page),
        Err(err) => HttpResponse::NotFound().json(err),
    }
}

#[put("/api/builder/pages/{id}")]
async fn update_page(id: web::Path<i32>, updated_page: web::Json<NewPageData>) -> impl Responder {
    match BuilderService::update_page(id.into_inner(), updated_page.into_inner()) {
        Ok(page) => HttpResponse::Ok().json(page),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

pub fn init_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(create_page);
    cfg.service(get_page);
    cfg.service(update_page);
}
