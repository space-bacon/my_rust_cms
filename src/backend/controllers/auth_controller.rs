use warp::Filter;
use crate::services::auth_service::AuthService;
use serde::Deserialize;
use warp::http::StatusCode;

#[derive(Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

async fn login_handler(auth_data: AuthData) -> Result<impl warp::Reply, warp::Rejection> {
    match AuthService::login(&auth_data.username, &auth_data.password) {
        Ok(token) => Ok(warp::reply::json(&token)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::UNAUTHORIZED)),
    }
}

async fn register_handler(auth_data: AuthData) -> Result<impl warp::Reply, warp::Rejection> {
    match AuthService::register(&auth_data.username, &auth_data.username, &auth_data.password) {
        Ok(_) => Ok(warp::reply::with_status(warp::reply::json(&"User registered"), StatusCode::CREATED)),
        Err(err) => Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::BAD_REQUEST)),
    }
}

pub fn init_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let login_route = warp::path!("api" / "auth" / "login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login_handler);

    let register_route = warp::path!("api" / "auth" / "register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(register_handler);

    warp::any().and(login_route.or(register_route))
}
