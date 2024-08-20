use actix_web::{App, HttpServer};
use crate::config::Config;
use crate::controllers::{auth_controller, posts_controller, users_controller, comments_controller, media_controller, settings_controller, builder_controller};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();

    HttpServer::new(move || {
        App::new()
            .configure(auth_controller::init_routes)
            .configure(posts_controller::init_routes)
            .configure(users_controller::init_routes)
            .configure(comments_controller::init_routes)
            .configure(media_controller::init_routes)
            .configure(settings_controller::init_routes)
            .configure(builder_controller::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
