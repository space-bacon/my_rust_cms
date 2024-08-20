use crate::models::comment::Comment;
use crate::repositories::comment_repository::CommentRepository;
use warp::reject::Reject;

pub struct CommentService;

#[derive(Debug)]
struct CommentError;
impl Reject for CommentError {}

impl CommentService {
    pub async fn create_comment(new_comment: Comment) -> Result<Comment, warp::Rejection> {
        CommentRepository::create(new_comment).map_err(|_| warp::reject::custom(CommentError))
    }

    pub async fn get_comment(id: i32) -> Result<Comment, warp::Rejection> {
        CommentRepository::find_by_id(id).map_err(|_| warp::reject::custom(CommentError))
    }

    pub async fn update_comment(id: i32, updated_comment: Comment) -> Result<Comment, warp::Rejection> {
        CommentRepository::update(id, updated_comment).map_err(|_| warp::reject::custom(CommentError))
    }

    pub async fn delete_comment(id: i32) -> Result<(), warp::Rejection> {
        CommentRepository::delete(id).map_err(|_| warp::reject::custom(CommentError))
    }

    pub async fn list_comments() -> Result<Vec<Comment>, warp::Rejection> {
        CommentRepository::find_all().map_err(|_| warp::reject::custom(CommentError))
    }
}
