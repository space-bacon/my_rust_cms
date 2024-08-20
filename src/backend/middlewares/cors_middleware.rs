use warp::filters::cors::Cors;

pub fn cors_middleware() -> Cors {
    warp::cors()
        .allow_origin("http://localhost:8080")
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allow_headers(vec!["Authorization", "Accept", "Content-Type"])
        .max_age(3600)
}
