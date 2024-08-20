use warp::Filter;
use crate::services::auth_service::AuthService;

pub fn with_auth() -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header::optional("Authorization"))
        .and_then(authenticate)
}

async fn authenticate(auth_header: Option<String>) -> Result<(), warp::Rejection> {
    if let Some(token) = auth_header {
        if AuthService::validate_token(&token).is_ok() {
            return Ok(());
        }
    }
    Err(warp::reject::custom(AuthError))
}

#[derive(Debug)]
struct AuthError;
impl warp::reject::Reject for AuthError {}
