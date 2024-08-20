use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;
use warp::reject::Reject;

pub struct UserService;

#[derive(Debug)]
struct UserError;
impl Reject for UserError {}

impl UserService {
    pub async fn create_user(new_user: User) -> Result<User, warp::Rejection> {
        UserRepository::create(new_user).map_err(|_| warp::reject::custom(UserError))
    }

    pub async fn get_user(id: i32) -> Result<User, warp::Rejection> {
        UserRepository::find_by_id(id).map_err(|_| warp::reject::custom(UserError))
    }

    pub async fn update_user(id: i32, updated_user: User) -> Result<User, warp::Rejection> {
        UserRepository::update(id, updated_user).map_err(|_| warp::reject::custom(UserError))
    }

    pub async fn delete_user(id: i32) -> Result<(), warp::Rejection> {
        UserRepository::delete(id).map_err(|_| warp::reject::custom(UserError))
    }

    pub async fn list_users() -> Result<Vec<User>, warp::Rejection> {
        UserRepository::find_all().map_err(|_| warp::reject::custom(UserError))
    }
}
