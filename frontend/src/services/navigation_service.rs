use serde::{Deserialize, Serialize};
use crate::services::auth_service::get_auth_token;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NavigationItem {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub order: i32,
    pub is_active: bool,
    pub menu_area: String,
    pub parent_id: Option<i32>,
    pub icon: Option<String>,
    pub css_class: Option<String>,
    pub target: Option<String>,
    pub mobile_visible: bool,
    pub description: Option<String>,
    pub children: Option<Vec<NavigationItem>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MenuArea {
    pub id: i32,
    pub area_name: String,
    pub display_name: String,
    pub template_id: Option<i32>,
    pub settings: serde_json::Value,
    pub mobile_behavior: Option<String>,
    pub hamburger_icon: Option<String>,
    pub is_active: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MenuTemplate {
    pub id: i32,
    pub name: String,
    pub template_type: String,
    pub layout_style: String,
    pub settings: serde_json::Value,
    pub is_active: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentTemplate {
    pub id: i32,
    pub name: String,
    pub component_type: String,
    pub template_data: serde_json::Value,
    pub breakpoints: serde_json::Value,
    pub width_setting: Option<String>,
    pub max_width: Option<String>,
    pub is_default: bool,
    pub is_active: bool,
}

#[derive(Debug)]
pub enum NavigationServiceError {
    #[allow(dead_code)]
    NetworkError(String),
    #[allow(dead_code)]
    ParseError(String),
}

impl std::fmt::Display for NavigationServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NavigationServiceError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            NavigationServiceError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

pub async fn get_navigation_items() -> Result<Vec<NavigationItem>, NavigationServiceError> {
    match gloo_net::http::Request::get("http://localhost:8081/api/navigation")
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Vec<NavigationItem>>().await {
                    Ok(items) => Ok(items),
                    Err(e) => Err(NavigationServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

pub async fn create_navigation_item(item: &NavigationItem) -> Result<NavigationItem, NavigationServiceError> {
    let token = get_auth_token().map_err(|_| NavigationServiceError::NetworkError("Not authenticated".to_string()))?;
    
    match gloo_net::http::Request::post("http://localhost:8081/api/navigation")
        .header("Authorization", &format!("Bearer {}", token))
        .json(item)
        .map_err(|e| NavigationServiceError::ParseError(e.to_string()))?
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 201 {
                match response.json::<NavigationItem>().await {
                    Ok(created_item) => Ok(created_item),
                    Err(e) => Err(NavigationServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

pub async fn update_navigation_item(id: i32, item: &NavigationItem) -> Result<NavigationItem, NavigationServiceError> {
    let token = get_auth_token().map_err(|_| NavigationServiceError::NetworkError("Not authenticated".to_string()))?;
    
    match gloo_net::http::Request::put(&format!("http://localhost:8081/api/navigation/{}", id))
        .header("Authorization", &format!("Bearer {}", token))
        .json(item)
        .map_err(|e| NavigationServiceError::ParseError(e.to_string()))?
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<NavigationItem>().await {
                    Ok(updated_item) => Ok(updated_item),
                    Err(e) => Err(NavigationServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

pub async fn delete_navigation_item(id: i32) -> Result<(), NavigationServiceError> {
    let token = get_auth_token().map_err(|_| NavigationServiceError::NetworkError("Not authenticated".to_string()))?;
    
    match gloo_net::http::Request::delete(&format!("http://localhost:8081/api/navigation/{}", id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                Ok(())
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

// Enhanced navigation functions

pub async fn get_navigation_by_area(area: &str) -> Result<Vec<NavigationItem>, NavigationServiceError> {
    match gloo_net::http::Request::get(&format!("http://localhost:8081/api/navigation/area/{}", area))
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Vec<NavigationItem>>().await {
                    Ok(items) => Ok(items),
                    Err(e) => Err(NavigationServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

#[allow(dead_code)]
pub async fn get_menu_area(name: &str) -> Result<MenuArea, NavigationServiceError> {
    match gloo_net::http::Request::get(&format!("http://localhost:8081/api/menu-areas/{}", name))
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<MenuArea>().await {
                    Ok(area) => Ok(area),
                    Err(e) => Err(NavigationServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

pub async fn get_menu_areas() -> Result<Vec<MenuArea>, NavigationServiceError> {
    let token = get_auth_token().map_err(|_| NavigationServiceError::NetworkError("Not authenticated".to_string()))?;
    
    match gloo_net::http::Request::get("http://localhost:8081/api/menu-areas")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Vec<MenuArea>>().await {
                    Ok(areas) => Ok(areas),
                    Err(e) => Err(NavigationServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

pub async fn update_menu_area(name: &str, area: &MenuArea) -> Result<MenuArea, NavigationServiceError> {
    let token = get_auth_token().map_err(|_| NavigationServiceError::NetworkError("Not authenticated".to_string()))?;
    
    match gloo_net::http::Request::put(&format!("http://localhost:8081/api/menu-areas/{}", name))
        .header("Authorization", &format!("Bearer {}", token))
        .json(area)
        .map_err(|e| NavigationServiceError::ParseError(e.to_string()))?
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<MenuArea>().await {
                    Ok(updated_area) => Ok(updated_area),
                    Err(e) => Err(NavigationServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

#[allow(dead_code)]
pub async fn get_menu_templates() -> Result<Vec<MenuTemplate>, NavigationServiceError> {
    let token = get_auth_token().map_err(|_| NavigationServiceError::NetworkError("Not authenticated".to_string()))?;
    
    match gloo_net::http::Request::get("http://localhost:8081/api/menu-templates")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Vec<MenuTemplate>>().await {
                    Ok(templates) => Ok(templates),
                    Err(e) => Err(NavigationServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

pub async fn get_component_templates() -> Result<Vec<ComponentTemplate>, NavigationServiceError> {
    // Public endpoint - no authentication required
    match gloo_net::http::Request::get("http://localhost:8081/api/component-templates")
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Vec<ComponentTemplate>>().await {
                    Ok(templates) => Ok(templates),
                    Err(e) => Err(NavigationServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

#[allow(dead_code)]
pub async fn create_component_template(template: &ComponentTemplate) -> Result<ComponentTemplate, NavigationServiceError> {
    let token = get_auth_token().map_err(|_| NavigationServiceError::NetworkError("Not authenticated".to_string()))?;
    
    match gloo_net::http::Request::post("http://localhost:8081/api/component-templates")
        .header("Authorization", &format!("Bearer {}", token))
        .json(template)
        .map_err(|e| NavigationServiceError::ParseError(e.to_string()))?
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 201 {
                match response.json::<ComponentTemplate>().await {
                    Ok(created_template) => Ok(created_template),
                    Err(e) => Err(NavigationServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

pub async fn update_component_template(id: i32, template: &ComponentTemplate) -> Result<ComponentTemplate, NavigationServiceError> {
    let token = get_auth_token().map_err(|_| NavigationServiceError::NetworkError("Not authenticated".to_string()))?;
    
    match gloo_net::http::Request::put(&format!("http://localhost:8081/api/component-templates/{}", id))
        .header("Authorization", &format!("Bearer {}", token))
        .json(template)
        .map_err(|e| NavigationServiceError::ParseError(e.to_string()))?
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<ComponentTemplate>().await {
                    Ok(updated_template) => Ok(updated_template),
                    Err(e) => Err(NavigationServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

pub async fn get_all_component_templates_admin() -> Result<Vec<ComponentTemplate>, NavigationServiceError> {
    let token = get_auth_token().map_err(|_| NavigationServiceError::NetworkError("Not authenticated".to_string()))?;
    
    match gloo_net::http::Request::get("http://localhost:8081/api/component-templates/admin")
        .header("Authorization", &format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Vec<ComponentTemplate>>().await {
                    Ok(templates) => Ok(templates),
                    Err(e) => Err(NavigationServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

pub async fn toggle_component_template(id: i32) -> Result<ComponentTemplate, NavigationServiceError> {
    let token = get_auth_token().map_err(|_| NavigationServiceError::NetworkError("Not authenticated".to_string()))?;
    
    match gloo_net::http::Request::post(&format!("http://localhost:8081/api/component-templates/{}/toggle", id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<ComponentTemplate>().await {
                    Ok(toggled_template) => Ok(toggled_template),
                    Err(e) => Err(NavigationServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

pub async fn check_comments_enabled() -> bool {
    // Check if Comments component template is active
    match get_component_templates().await {
        Ok(templates) => {
            templates
                .iter()
                .find(|template| template.component_type == "Comments")
                .map(|template| template.is_active)
                .unwrap_or(true) // Default to enabled if not found
        }
        Err(_) => true, // Default to enabled on error
    }
} 