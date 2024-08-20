use crate::models::comment::Comment;
use diesel::prelude::*;

pub struct CommentRepository;

impl CommentRepository {
    pub fn create(new_comment: Comment) -> Result<Comment, &'static str> {
        // Implement logic to insert the comment into the database
        Ok(new_comment)
    }

    pub fn find_by_id(id: i32) -> Result<Comment, &'static str> {
        // Implement logic to find a comment by id
        Err("Comment not found")
    }

    pub fn update(id: i32, updated_comment: Comment) -> Result<Comment, &'static str> {
        // Implement logic to update a comment by id
        Ok(updated_comment)
    }

    pub fn delete(id: i32) -> Result<(), &'static str> {
        // Implement logic to delete a comment by id
        Ok(())
    }

    pub fn find_all() -> Result<Vec<Comment>, &'static str> {
        // Implement logic to list all comments
        Ok(vec![])
    }
}
