use crate::models::page::Page;
use crate::repositories::page_repository::PageRepository;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewPageData {
    pub title: String,
    pub content: String,  // JSON or serialized format for the content structure
}

pub struct BuilderService;

impl BuilderService {
    pub fn create_page(data: NewPageData) -> Result<Page, &'static str> {
        let new_page = Page {
            id: 0,  // Auto-incremented by the database
            title: data.title,
            content: data.content,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };
        PageRepository::create(new_page)
    }

    pub fn get_page(id: i32) -> Result<Page, &'static str> {
        PageRepository::find_by_id(id)
    }

    pub fn update_page(id: i32, data: NewPageData) -> Result<Page, &'static str> {
        let updated_page = Page {
            id,
            title: data.title,
            content: data.content,
            created_at: chrono::Utc::now().naive_utc(),  // Fetch original or use updated timestamp logic
            updated_at: chrono::Utc::now().naive_utc(),
        };
        PageRepository::update(id, updated_page)
    }
}
