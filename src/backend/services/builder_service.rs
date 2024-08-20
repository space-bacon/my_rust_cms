use crate::models::page::Page;
use crate::repositories::page_repository::PageRepository;
use serde::{Deserialize, Serialize};
use warp::reject::Reject;

#[derive(Serialize, Deserialize)]
pub struct NewPageData {
    pub title: String,
    pub content: String,  // JSON or serialized format for the content structure
}

pub struct BuilderService;

#[derive(Debug)]
struct BuilderError;
impl Reject for BuilderError {}

impl BuilderService {
    pub async fn create_page(data: NewPageData) -> Result<Page, warp::Rejection> {
        let new_page = Page {
            id: 0,  // Auto-incremented by the database
            title: data.title,
            content: data.content,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };
        PageRepository::create(new_page).map_err(|_| warp::reject::custom(BuilderError))
    }

    pub async fn get_page(id: i32) -> Result<Page, warp::Rejection> {
        PageRepository::find_by_id(id).map_err(|_| warp::reject::custom(BuilderError))
    }

    pub async fn update_page(id: i32, data: NewPageData) -> Result<Page, warp::Rejection> {
        let updated_page = Page {
            id,
            title: data.title,
            content: data.content,
            created_at: chrono::Utc::now().naive_utc(),  // Fetch original or use updated timestamp logic
            updated_at: chrono::Utc::now().naive_utc(),
        };
        PageRepository::update(id, updated_page).map_err(|_| warp::reject::custom(BuilderError))
    }
}
