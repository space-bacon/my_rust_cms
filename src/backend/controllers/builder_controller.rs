use warp::Filter;
use crate::services::builder_service::{BuilderService, NewPageData};
use warp::http::StatusCode;

async fn create_page_handler(new_page: NewPageData) -> Result<impl warp::Reply, warp::Rejection> {
    match BuilderService::create_page(new_page) {
        Ok(page) => Ok(warp::reply::with_status(warp::reply::json(&page), StatusCode::CREATED)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::BAD_REQUEST)),
    }
}

async fn get_page_handler(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    match BuilderService::get_page(id) {
        Ok(page) => Ok(warp::reply::json(&page)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::NOT_FOUND)),
    }
}

async fn update_page_handler(id: i32, updated_page: NewPageData) -> Result<impl warp::Reply, warp::Rejection> {
    match BuilderService::update_page(id, updated_page) {
        Ok(page) => Ok(warp::reply::json(&page)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::BAD_REQUEST)),
    }
}

pub fn init_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let create_page_route = warp::path!("api" / "builder" / "pages")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create_page_handler);

    let get_page_route = warp::path!("api" / "builder" / "pages" / i32)
        .and(warp::get())
        .and_then(get_page_handler);

    let update_page_route = warp::path!("api" / "builder" / "pages" / i32)
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update_page_handler);

    warp::any().and(create_page_route.or(get_page_route).or(update_page_route))
}
