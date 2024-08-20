use warp::Filter;
use crate::services::settings_service::SettingsService;
use crate::models::settings::Settings;
use warp::http::StatusCode;

async fn get_settings_handler() -> Result<impl warp::Reply, warp::Rejection> {
    match SettingsService::get_settings() {
        Ok(settings) => Ok(warp::reply::json(&settings)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn update_settings_handler(settings_data: Settings) -> Result<impl warp::Reply, warp::Rejection> {
    match SettingsService::update_settings(settings_data) {
        Ok(settings) => Ok(warp::reply::json(&settings)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::BAD_REQUEST)),
    }
}

pub fn init_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get_settings_route = warp::path!("api" / "settings")
        .and(warp::get())
        .and_then(get_settings_handler);

    let update_settings_route = warp::path!("api" / "settings")
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update_settings_handler);

    warp::any().and(get_settings_route.or(update_settings_route))
}
