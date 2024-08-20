use std::convert::Infallible;
use warp::Filter;
use log::info;

pub fn logging_middleware() -> impl Filter<Extract = (impl warp::Reply,), Error = Infallible> + Clone {
    warp::log::custom(|info| {
        let method = info.method().to_string();
        let path = info.path().to_string();
        info!("Request: {} {}", method, path);
    })
}
