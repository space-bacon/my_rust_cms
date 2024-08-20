use warp::Filter;
use crate::services::comment_service::CommentService;
use crate::models::comment::Comment;
use warp::http::StatusCode;

async fn create_comment_handler(comment_data: Comment) -> Result<impl warp::Reply, warp::Rejection> {
    match CommentService::create_comment(comment_data) {
        Ok(comment) => Ok(warp::reply::with_status(warp::reply::json(&comment), StatusCode::CREATED)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::BAD_REQUEST)),
    }
}

async fn get_all_comments_handler() -> Result<impl warp::Reply, warp::Rejection> {
    match CommentService::list_comments() {
        Ok(comments) => Ok(warp::reply::json(&comments)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn get_comment_handler(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    match CommentService::get_comment(id) {
        Ok(comment) => Ok(warp::reply::json(&comment)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::NOT_FOUND)),
    }
}

async fn update_comment_handler(id: i32, comment_data: Comment) -> Result<impl warp::Reply, warp::Rejection> {
    match CommentService::update_comment(id, comment_data) {
        Ok(comment) => Ok(warp::reply::json(&comment)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::BAD_REQUEST)),
    }
}

async fn delete_comment_handler(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    match CommentService::delete_comment(id) {
        Ok(_) => Ok(warp::reply::with_status(warp::reply::json(&"Comment deleted"), StatusCode::OK)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

pub fn init_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let create_comment_route = warp::path!("api" / "comments")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create_comment_handler);

    let get_all_comments_route = warp::path!("api" / "comments")
        .and(warp::get())
        .and_then(get_all_comments_handler);

    let get_comment_route = warp::path!("api" / "comments" / i32)
        .and(warp::get())
        .and_then(get_comment_handler);

    let update_comment_route = warp::path!("api" / "comments" / i32)
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update_comment_handler);

    let delete_comment_route = warp::path!("api" / "comments" / i32)
        .and(warp::delete())
        .and_then(delete_comment_handler);

    warp::any().and(create_comment_route.or(get_all_comments_route).or(get_comment_route).or(update_comment_route).or(delete_comment_route))
}
