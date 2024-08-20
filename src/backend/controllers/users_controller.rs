use warp::Filter;
use crate::services::user_service::UserService;
use crate::models::user::User;
use warp::http::StatusCode;

async fn create_user_handler(user_data: User) -> Result<impl warp::Reply, warp::Rejection> {
    match UserService::create_user(user_data) {
        Ok(user) => Ok(warp::reply::with_status(warp::reply::json(&user), StatusCode::CREATED)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::BAD_REQUEST)),
    }
}

async fn get_all_users_handler() -> Result<impl warp::Reply, warp::Rejection> {
    match UserService::list_users() {
        Ok(users) => Ok(warp::reply::json(&users)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

async fn get_user_handler(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    match UserService::get_user(id) {
        Ok(user) => Ok(warp::reply::json(&user)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::NOT_FOUND)),
    }
}

async fn update_user_handler(id: i32, user_data: User) -> Result<impl warp::Reply, warp::Rejection> {
    match UserService::update_user(id, user_data) {
        Ok(user) => Ok(warp::reply::json(&user)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::BAD_REQUEST)),
    }
}

async fn delete_user_handler(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    match UserService::delete_user(id) {
        Ok(_) => Ok(warp::reply::with_status(warp::reply::json("User deleted"), StatusCode::OK)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

pub fn init_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let create_user_route = warp::path!("api" / "users")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create_user_handler);

    let get_all_users_route = warp::path!("api" / "users")
        .and(warp::get())
        .and_then(get_all_users_handler);

    let get_user_route = warp::path!("api" / "users" / i32)
        .and(warp::get())
        .and_then(get_user_handler);

    let update_user_route = warp::path!("api" / "users" / i32)
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update_user_handler);

    let delete_user_route = warp::path!("api" / "users" / i32)
        .and(warp::delete())
        .and_then(delete_user_handler);

    warp::any().and(
        create_user_route
            .or(get_all_users_route)
            .or(get_user_route)
            .or(update_user_route)
            .or(delete_user_route),
    )
}
