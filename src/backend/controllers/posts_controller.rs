use warp::Filter;
use crate::services::post_service::PostService;
use crate::models::post::Post;
use warp::http::StatusCode;

async fn create_post_handler(post_data: Post) -> Result<impl warp::Reply, warp::Rejection> {
    match PostService::create_post(post_data) {
        Ok(post) => Ok(warp::reply::with_status(warp::reply::json(&post), StatusCode::CREATED)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::BAD_REQUEST)),
    }
}

async fn get_all_posts_handler() -> Result<impl warp::Reply, warp::Rejection> {
    match PostService::list_posts() {
        Ok(posts) => Ok(warp::reply::json(&posts)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn get_post_handler(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    match PostService::get_post(id) {
        Ok(post) => Ok(warp::reply::json(&post)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::NOT_FOUND)),
    }
}

async fn update_post_handler(id: i32, post_data: Post) -> Result<impl warp::Reply, warp::Rejection> {
    match PostService::update_post(id, post_data) {
        Ok(post) => Ok(warp::reply::json(&post)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::BAD_REQUEST)),
    }
}

async fn delete_post_handler(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    match PostService::delete_post(id) {
        Ok(_) => Ok(warp::reply::with_status(warp::reply::json(&"Post deleted"), StatusCode::OK)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

pub fn init_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let create_post_route = warp::path!("api" / "posts")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create_post_handler);

    let get_all_posts_route = warp::path!("api" / "posts")
        .and(warp::get())
        .and_then(get_all_posts_handler);

    let get_post_route = warp::path!("api" / "posts" / i32)
        .and(warp::get())
        .and_then(get_post_handler);

    let update_post_route = warp::path!("api" / "posts" / i32)
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update_post_handler);

    let delete_post_route = warp::path!("api" / "posts" / i32)
        .and(warp::delete())
        .and_then
