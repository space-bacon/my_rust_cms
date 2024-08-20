use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;

pub struct UserService;

impl UserService {
    pub fn create_user(new_user: User) -> Result<User, &'static str> {
        UserRepository::create(new_user)
    }

    pub fn get_user(id: i32) -> Result<User, &'static str> {
        UserRepository::find_by_id(id)
    }

    pub fn update_user(id: i32, updated_user: User) -> Result<User, &'static str> {
        UserRepository::update(id, updated_user)
    }

    pub fn delete_user(id: i32) -> Result<(), &'static str> {
        UserRepository::delete(id)
    }

    pub fn list_users() -> Result<Vec<User>, &'static str> {
        UserRepository::find_all()
    }
}
