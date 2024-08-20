use actix_web::{get, put, HttpResponse, Responder, web};
use crate::services::settings_service::SettingsService;
use crate::models::settings::Settings;

#[get("/api/settings")]
async fn get_settings() -> impl Responder {
    match SettingsService::get_settings() {
        Ok(settings) => HttpResponse::Ok().json(settings),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

#[put("/api/settings")]
async fn update_settings(settings_data: web::Json<Settings>) -> impl Responder {
    match SettingsService::update_settings(settings_data.into_inner()) {
        Ok(settings) => HttpResponse::Ok().json(settings),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

pub fn init_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_settings);
    cfg.service(update_settings);
}
