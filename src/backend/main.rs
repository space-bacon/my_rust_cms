use warp::Filter;
use crate::config::Config;
use crate::controllers::{
    auth_controller, posts_controller, users_controller, comments_controller,
    media_controller, settings_controller, builder_controller,
};

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    // Combine all route configurations into one.
    let routes = auth_controller::init_routes()
        .or(posts_controller::init_routes())
        .or(users_controller::init_routes())
        .or(comments_controller::init_routes())
        .or(media_controller::init_routes())
        .or(settings_controller::init_routes())
        .or(builder_controller::init_routes());

    // Serve the combined routes.
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
