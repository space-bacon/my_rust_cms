use crate::models::post::Post;
use crate::repositories::post_repository::PostRepository;
use warp::reject::Reject;

pub struct PostService;

#[derive(Debug)]
struct PostError;
impl Reject for PostError {}

impl PostService {
    pub async fn create_post(new_post: Post) -> Result<Post, warp::Rejection> {
        PostRepository::create(new_post).map_err(|_| warp::reject::custom(PostError))
    }

    pub async fn get_post(id: i32) -> Result<Post, warp::Rejection> {
        PostRepository::find_by_id(id).map_err(|_| warp::reject::custom(PostError))
    }

    pub async fn update_post(id: i32, updated_post: Post) -> Result<Post, warp::Rejection> {
        PostRepository::update(id, updated_post).map_err(|_| warp::reject::custom(PostError))
    }

    pub async fn delete_post(id: i32) -> Result<(), warp::Rejection> {
        PostRepository::delete(id).map_err(|_| warp::reject::custom(PostError))
    }

    pub async fn list_posts() -> Result<Vec<Post>, warp::Rejection> {
        PostRepository::find_all().map_err(|_| warp::reject::custom(PostError))
    }
}
