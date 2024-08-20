use crate::models::post::Post;
use crate::repositories::post_repository::PostRepository;

pub struct PostService;

impl PostService {
    pub fn create_post(new_post: Post) -> Result<Post, &'static str> {
        PostRepository::create(new_post)
    }

    pub fn get_post(id: i32) -> Result<Post, &'static str> {
        PostRepository::find_by_id(id)
    }

    pub fn update_post(id: i32, updated_post: Post) -> Result<Post, &'static str> {
        PostRepository::update(id, updated_post)
    }

    pub fn delete_post(id: i32) -> Result<(), &'static str> {
        PostRepository::delete(id)
    }

    pub fn list_posts() -> Result<Vec<Post>, &'static str> {
        PostRepository::find_all()
    }
}
