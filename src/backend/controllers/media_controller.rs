use warp::Filter;
use crate::services::media_service::MediaService;
use crate::models::media::Media;
use warp::http::StatusCode;

async fn upload_media_handler(media_data: Media) -> Result<impl warp::Reply, warp::Rejection> {
    match MediaService::upload_media(media_data) {
        Ok(media) => Ok(warp::reply::with_status(warp::reply::json(&media), StatusCode::CREATED)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::BAD_REQUEST)),
    }
}

async fn get_all_media_handler() -> Result<impl warp::Reply, warp::Rejection> {
    match MediaService::list_media() {
        Ok(media) => Ok(warp::reply::json(&media)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn get_media_handler(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    match MediaService::get_media(id) {
        Ok(media) => Ok(warp::reply::json(&media)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::NOT_FOUND)),
    }
}

async fn delete_media_handler(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    match MediaService::delete_media(id) {
        Ok(_) => Ok(warp::reply::with_status(warp::reply::json(&"Media deleted"), StatusCode::OK)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

pub fn init_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let upload_media_route = warp::path!("api" / "media")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(upload_media_handler);

    let get_all_media_route = warp::path!("api" / "media")
        .and(warp::get())
        .and_then(get_all_media_handler);

    let get_media_route = warp::path!("api" / "media" / i32)
        .and(warp::get())
        .and_then(get_media_handler);

    let delete_media_route = warp::path!("api" / "media" / i32)
        .and(warp::delete())
        .and_then(delete_media_handler);

    warp::any().and(upload_media_route.or(get_all_media_route).or(get_media_route).or(delete_media_route))
}
