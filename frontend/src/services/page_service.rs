use serde::{Deserialize, Serialize};
use crate::services::auth_service::get_auth_token;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Page {
    pub id: Option<i32>,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug)]
pub enum PageServiceError {
    #[allow(dead_code)]
    NetworkError(String),
    #[allow(dead_code)]
    ParseError(String),
}

pub async fn get_pages() -> Result<Vec<Page>, PageServiceError> {
    match gloo_net::http::Request::get("http://localhost:8081/api/pages")
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Vec<Page>>().await {
                    Ok(pages) => Ok(pages),
                    Err(e) => Err(PageServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(PageServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(PageServiceError::NetworkError(e.to_string())),
    }
}

#[allow(dead_code)]
pub async fn create_page(page: &Page) -> Result<Page, PageServiceError> {
    let token = get_auth_token().map_err(|_| PageServiceError::NetworkError("Not authenticated".to_string()))?;
    
    let request = gloo_net::http::Request::post("http://localhost:8081/api/pages")
        .header("Authorization", &format!("Bearer {}", token))
        .json(page)
        .map_err(|e| PageServiceError::NetworkError(e.to_string()))?;
    
    match request.send().await {
        Ok(response) => {
            if response.status() == 201 {
                match response.json::<Page>().await {
                    Ok(page) => Ok(page),
                    Err(e) => Err(PageServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(PageServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(PageServiceError::NetworkError(e.to_string())),
    }
}

#[allow(dead_code)]
pub async fn update_page(id: i32, page: &Page) -> Result<Page, PageServiceError> {
    let token = get_auth_token().map_err(|_| PageServiceError::NetworkError("Not authenticated".to_string()))?;
    
    let request = gloo_net::http::Request::put(&format!("http://localhost:8081/api/pages/{}", id))
        .header("Authorization", &format!("Bearer {}", token))
        .json(page)
        .map_err(|e| PageServiceError::NetworkError(e.to_string()))?;
    
    match request.send().await {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Page>().await {
                    Ok(page) => Ok(page),
                    Err(e) => Err(PageServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(PageServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(PageServiceError::NetworkError(e.to_string())),
    }
}

#[allow(dead_code)]
pub async fn update_page_content(id: i32, content: &str) -> Result<Page, PageServiceError> {
    let token = get_auth_token().map_err(|_| PageServiceError::NetworkError("Not authenticated".to_string()))?;
    
    let request_body = serde_json::json!({
        "content": content
    });
    
    let request = gloo_net::http::Request::put(&format!("http://localhost:8081/api/pages/{}/content", id))
        .header("Authorization", &format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .map_err(|e| PageServiceError::NetworkError(e.to_string()))?;
    
    match request.send().await {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Page>().await {
                    Ok(page) => Ok(page),
                    Err(e) => Err(PageServiceError::ParseError(e.to_string())),
                }
            } else {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Err(PageServiceError::NetworkError(format!("HTTP {}: {}", response.status(), error_text)))
            }
        }
        Err(e) => Err(PageServiceError::NetworkError(e.to_string())),
    }
}

#[allow(dead_code)]
pub async fn delete_page(id: i32) -> Result<(), PageServiceError> {
    let token = get_auth_token().map_err(|_| PageServiceError::NetworkError("Not authenticated".to_string()))?;
    
    match gloo_net::http::Request::delete(&format!("http://localhost:8081/api/pages/{}", id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                Ok(())
            } else {
                Err(PageServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(PageServiceError::NetworkError(e.to_string())),
    }
}

pub async fn get_page_by_slug(slug: &str) -> Result<Page, PageServiceError> {
    match gloo_net::http::Request::get(&format!("http://localhost:8081/api/pages/slug/{}", slug))
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Page>().await {
                    Ok(page) => Ok(page),
                    Err(e) => Err(PageServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(PageServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(PageServiceError::NetworkError(e.to_string())),
    }
} 