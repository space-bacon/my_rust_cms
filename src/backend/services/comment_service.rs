use crate::models::comment::Comment;
use crate::repositories::comment_repository::CommentRepository;

pub struct CommentService;

impl CommentService {
    pub fn create_comment(new_comment: Comment) -> Result<Comment, &'static str> {
        CommentRepository::create(new_comment)
    }

    pub fn get_comment(id: i32) -> Result<Comment, &'static str> {
        CommentRepository::find_by_id(id)
    }

    pub fn update_comment(id: i32, updated_comment: Comment) -> Result<Comment, &'static str> {
        CommentRepository::update(id, updated_comment)
    }

    pub fn delete_comment(id: i32) -> Result<(), &'static str> {
        CommentRepository::delete(id)
    }

    pub fn list_comments() -> Result<Vec<Comment>, &'static str> {
        CommentRepository::find_all()
    }
}
