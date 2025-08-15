use axum::{
    extract::{State, Path, Json},
    response::Json as ResponseJson,
    http::StatusCode,
};
use crate::{
    AppServices,
    models::{Navigation, NewNavigation, UpdateNavigation, MenuArea, MenuTemplate, ComponentTemplate, NewMenuTemplate, NewComponentTemplate, UpdateMenuArea, UpdateComponentTemplate},
    middleware::{
        validation::validate_text_content,
        errors::AppError,
    },
};

// Frontend-compatible Navigation structure (enhanced)
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FrontendNavigationItem {
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
    pub children: Option<Vec<FrontendNavigationItem>>,
}

impl From<Navigation> for FrontendNavigationItem {
    fn from(nav: Navigation) -> Self {
        FrontendNavigationItem {
            id: nav.id,
            title: nav.title,
            url: nav.url,
            order: nav.order_position,
            is_active: nav.is_active,
            menu_area: nav.menu_area,
            parent_id: nav.parent_id,
            icon: nav.icon,
            css_class: nav.css_class,
            target: nav.target,
            mobile_visible: nav.mobile_visible,
            description: nav.description,
            children: None, // Will be populated separately for hierarchical structures
        }
    }
}

// Menu Area structure for frontend
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FrontendMenuArea {
    pub id: i32,
    pub area_name: String,
    pub display_name: String,
    pub template_id: Option<i32>,
    pub settings: serde_json::Value,
    pub mobile_behavior: Option<String>,
    pub hamburger_icon: Option<String>,
    pub is_active: bool,
}

impl From<MenuArea> for FrontendMenuArea {
    fn from(area: MenuArea) -> Self {
        FrontendMenuArea {
            id: area.id,
            area_name: area.area_name,
            display_name: area.display_name,
            template_id: area.template_id,
            settings: area.settings,
            mobile_behavior: area.mobile_behavior,
            hamburger_icon: area.hamburger_icon,
            is_active: area.is_active,
        }
    }
}

// Menu Template structure for frontend
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FrontendMenuTemplate {
    pub id: i32,
    pub name: String,
    pub template_type: String,
    pub layout_style: String,
    pub settings: serde_json::Value,
    pub is_active: bool,
}

impl From<MenuTemplate> for FrontendMenuTemplate {
    fn from(template: MenuTemplate) -> Self {
        FrontendMenuTemplate {
            id: template.id,
            name: template.name,
            template_type: template.template_type,
            layout_style: template.layout_style,
            settings: template.settings,
            is_active: template.is_active,
        }
    }
}

// Component Template structure for frontend
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FrontendComponentTemplate {
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

impl From<ComponentTemplate> for FrontendComponentTemplate {
    fn from(template: ComponentTemplate) -> Self {
        FrontendComponentTemplate {
            id: template.id,
            name: template.name,
            component_type: template.component_type,
            template_data: template.template_data,
            breakpoints: template.breakpoints,
            width_setting: template.width_setting,
            max_width: template.max_width,
            is_default: template.is_default,
            is_active: template.is_active,
        }
    }
}

/// Get all active navigation items (public endpoint)
/// 
/// Returns navigation items for public site display.
/// Only returns active items, ordered by position.
/// No authentication required for public access.
pub async fn get_navigation(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<FrontendNavigationItem>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let nav_items = Navigation::list_active(&mut conn)?;
    let frontend_nav_items: Vec<FrontendNavigationItem> = nav_items.into_iter()
        .map(FrontendNavigationItem::from)
        .collect();
    Ok(ResponseJson(frontend_nav_items))
}

/// Create a new navigation item (admin only)
/// 
/// Creates a new navigation menu item.
/// Validates title and URL format.
/// Requires admin authentication.
pub async fn create_navigation_item(
    State(services): State<AppServices>, 
    Json(nav_item): Json<FrontendNavigationItem>
) -> Result<(StatusCode, ResponseJson<FrontendNavigationItem>), AppError> {
    // Validate input
    if nav_item.title.trim().is_empty() {
        return Err(AppError::ValidationError("Title cannot be empty".to_string()));
    }
    
    if nav_item.url.trim().is_empty() {
        return Err(AppError::ValidationError("URL cannot be empty".to_string()));
    }
    
    validate_text_content(&nav_item.title, 100)?;
    validate_text_content(&nav_item.url, 200)?;
    
    // Basic URL validation
    if !nav_item.url.starts_with('/') && !nav_item.url.starts_with("http") {
        return Err(AppError::ValidationError("URL must start with '/' or 'http'".to_string()));
    }
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let new_nav = NewNavigation {
        title: nav_item.title.trim().to_string(),
        url: nav_item.url.trim().to_string(),
        order_position: nav_item.order,
        is_active: nav_item.is_active,
        menu_area: nav_item.menu_area,
        parent_id: nav_item.parent_id,
        icon: nav_item.icon,
        css_class: nav_item.css_class,
        target: nav_item.target.or(Some("_self".to_string())),
        mobile_visible: nav_item.mobile_visible,
        description: nav_item.description,
    };
    
    let created_nav = Navigation::create(&mut conn, new_nav)?;
    let response = FrontendNavigationItem::from(created_nav);
    
    Ok((StatusCode::CREATED, ResponseJson(response)))
}

/// Update an existing navigation item (admin only)
/// 
/// Updates navigation item properties.
/// Validates title and URL format.
/// Requires admin authentication.
pub async fn update_navigation_item(
    State(services): State<AppServices>, 
    Path(id): Path<i32>, 
    Json(nav_item): Json<FrontendNavigationItem>
) -> Result<ResponseJson<FrontendNavigationItem>, AppError> {
    // Validate input
    if nav_item.title.trim().is_empty() {
        return Err(AppError::ValidationError("Title cannot be empty".to_string()));
    }
    
    if nav_item.url.trim().is_empty() {
        return Err(AppError::ValidationError("URL cannot be empty".to_string()));
    }
    
    validate_text_content(&nav_item.title, 100)?;
    validate_text_content(&nav_item.url, 200)?;
    
    // Basic URL validation
    if !nav_item.url.starts_with('/') && !nav_item.url.starts_with("http") {
        return Err(AppError::ValidationError("URL must start with '/' or 'http'".to_string()));
    }
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Check if navigation item exists
    let _existing_nav = Navigation::find_by_id(&mut conn, id)?
        .ok_or_else(|| AppError::NotFound("Navigation item not found".to_string()))?;
    
    let update_nav = UpdateNavigation {
        title: Some(nav_item.title.trim().to_string()),
        url: Some(nav_item.url.trim().to_string()),
        order_position: Some(nav_item.order),
        is_active: Some(nav_item.is_active),
        updated_at: None, // Will be set in the model
        menu_area: Some(nav_item.menu_area),
        parent_id: Some(nav_item.parent_id),
        icon: Some(nav_item.icon),
        css_class: Some(nav_item.css_class),
        target: Some(nav_item.target),
        mobile_visible: Some(nav_item.mobile_visible),
        description: Some(nav_item.description),
    };
    
    let updated_nav = Navigation::update(&mut conn, id, update_nav)?;
    let response = FrontendNavigationItem::from(updated_nav);
    
    Ok(ResponseJson(response))
}

/// Delete a navigation item (admin only)
/// 
/// Permanently deletes a navigation menu item.
/// Requires admin authentication.
pub async fn delete_navigation_item(
    State(services): State<AppServices>, 
    Path(id): Path<i32>
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Check if navigation item exists
    let _existing_nav = Navigation::find_by_id(&mut conn, id)?
        .ok_or_else(|| AppError::NotFound("Navigation item not found".to_string()))?;
    
    Navigation::delete(&mut conn, id)?;
    
    Ok(ResponseJson(serde_json::json!({
        "success": true,
        "message": "Navigation item deleted successfully"
    })))
}

/// Get navigation items by menu area (public endpoint)
/// 
/// Returns navigation items for a specific menu area with hierarchical structure.
/// Only returns active items, ordered by position.
/// No authentication required for public access.
pub async fn get_navigation_by_area(
    State(services): State<AppServices>,
    Path(area): Path<String>
) -> Result<ResponseJson<Vec<FrontendNavigationItem>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let nav_items = Navigation::list_by_area_with_children(&mut conn, &area)?;
    
    // Build hierarchical structure
    let mut items_map: std::collections::HashMap<i32, FrontendNavigationItem> = std::collections::HashMap::new();
    let mut root_items: Vec<FrontendNavigationItem> = Vec::new();
    
    // First pass: convert all items and collect them
    for nav in nav_items {
        let item = FrontendNavigationItem::from(nav);
        items_map.insert(item.id, item);
    }
    
    // Second pass: build hierarchy
    for (_, item) in items_map.drain() {
        if let Some(parent_id) = item.parent_id {
            // This is a child item, find its parent
            if let Some(parent) = root_items.iter_mut().find(|i| i.id == parent_id) {
                if parent.children.is_none() {
                    parent.children = Some(Vec::new());
                }
                parent.children.as_mut().unwrap().push(item);
            }
        } else {
            // This is a root item
            root_items.push(item);
        }
    }
    
    Ok(ResponseJson(root_items))
}

/// Get all menu areas (admin endpoint)
pub async fn get_menu_areas(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<FrontendMenuArea>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let areas = MenuArea::list_active(&mut conn)?;
    let frontend_areas: Vec<FrontendMenuArea> = areas.into_iter()
        .map(FrontendMenuArea::from)
        .collect();
    
    Ok(ResponseJson(frontend_areas))
}

/// Get menu area by name (public endpoint)
pub async fn get_menu_area_by_name(
    State(services): State<AppServices>,
    Path(name): Path<String>
) -> Result<ResponseJson<FrontendMenuArea>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let area = MenuArea::find_by_name(&mut conn, &name)?
        .ok_or_else(|| AppError::NotFound("Menu area not found".to_string()))?;
    
    Ok(ResponseJson(FrontendMenuArea::from(area)))
}

/// Update menu area (admin endpoint)
pub async fn update_menu_area(
    State(services): State<AppServices>,
    Path(name): Path<String>,
    Json(area_data): Json<FrontendMenuArea>
) -> Result<ResponseJson<FrontendMenuArea>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Find existing area
    let existing_area = MenuArea::find_by_name(&mut conn, &name)?
        .ok_or_else(|| AppError::NotFound("Menu area not found".to_string()))?;
    
    let update_data = UpdateMenuArea {
        display_name: Some(area_data.display_name),
        template_id: Some(area_data.template_id),
        settings: Some(area_data.settings),
        mobile_behavior: Some(area_data.mobile_behavior),
        hamburger_icon: Some(area_data.hamburger_icon),
        is_active: Some(area_data.is_active),
        updated_at: Some(chrono::Utc::now().naive_utc()),
    };
    
    use diesel::prelude::*;
    use crate::schema::menu_areas;
    
    let updated_area = diesel::update(menu_areas::table.find(existing_area.id))
        .set(update_data)
        .get_result::<MenuArea>(&mut conn)?;
    
    Ok(ResponseJson(FrontendMenuArea::from(updated_area)))
}

/// Get menu templates by type (admin endpoint)
pub async fn get_menu_templates_by_type(
    State(services): State<AppServices>,
    Path(template_type): Path<String>
) -> Result<ResponseJson<Vec<FrontendMenuTemplate>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let templates = MenuTemplate::find_by_type(&mut conn, &template_type)?;
    let frontend_templates: Vec<FrontendMenuTemplate> = templates.into_iter()
        .map(FrontendMenuTemplate::from)
        .collect();
    
    Ok(ResponseJson(frontend_templates))
}

/// Get all menu templates (admin endpoint)
pub async fn get_menu_templates(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<FrontendMenuTemplate>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    use diesel::prelude::*;
    use crate::schema::menu_templates;
    
    let templates = menu_templates::table
        .filter(menu_templates::is_active.eq(true))
        .load::<MenuTemplate>(&mut conn)?;
    
    let frontend_templates: Vec<FrontendMenuTemplate> = templates.into_iter()
        .map(FrontendMenuTemplate::from)
        .collect();
    
    Ok(ResponseJson(frontend_templates))
}

/// Create menu template (admin endpoint)
pub async fn create_menu_template(
    State(services): State<AppServices>,
    Json(template_data): Json<FrontendMenuTemplate>
) -> Result<(StatusCode, ResponseJson<FrontendMenuTemplate>), AppError> {
    validate_text_content(&template_data.name, 100)?;
    validate_text_content(&template_data.template_type, 50)?;
    validate_text_content(&template_data.layout_style, 50)?;
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let new_template = NewMenuTemplate {
        name: template_data.name,
        template_type: template_data.template_type,
        layout_style: template_data.layout_style,
        settings: template_data.settings,
        is_active: template_data.is_active,
    };
    
    use diesel::prelude::*;
    use crate::schema::menu_templates;
    
    let created_template = diesel::insert_into(menu_templates::table)
        .values(&new_template)
        .get_result::<MenuTemplate>(&mut conn)?;
    
    Ok((StatusCode::CREATED, ResponseJson(FrontendMenuTemplate::from(created_template))))
}

/// Get component templates by type (admin endpoint)
pub async fn get_component_templates_by_type(
    State(services): State<AppServices>,
    Path(component_type): Path<String>
) -> Result<ResponseJson<Vec<FrontendComponentTemplate>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let templates = ComponentTemplate::find_by_type(&mut conn, &component_type)?;
    let frontend_templates: Vec<FrontendComponentTemplate> = templates.into_iter()
        .map(FrontendComponentTemplate::from)
        .collect();
    
    Ok(ResponseJson(frontend_templates))
}

/// Get all component templates (public site - active only)
pub async fn get_component_templates(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<FrontendComponentTemplate>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    use diesel::prelude::*;
    use crate::schema::component_templates;
    
    let templates = component_templates::table
        .filter(component_templates::is_active.eq(true))
        .load::<ComponentTemplate>(&mut conn)?;
    
    // Debug: Log what we're returning
    for template in &templates {
        if template.component_type == "header" {
            println!("🔍 Backend: Returning header template ID {} with data: {}", template.id, serde_json::to_string_pretty(&template.template_data).unwrap_or_default());
        }
    }
    
    let frontend_templates: Vec<FrontendComponentTemplate> = templates.into_iter()
        .map(FrontendComponentTemplate::from)
        .collect();
    
    Ok(ResponseJson(frontend_templates))
}

/// Get all component templates for admin (including inactive ones)
pub async fn get_all_component_templates_admin(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<FrontendComponentTemplate>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    use diesel::prelude::*;
    use crate::schema::component_templates;
    
    let templates = component_templates::table
        .load::<ComponentTemplate>(&mut conn)?;
    
    let frontend_templates: Vec<FrontendComponentTemplate> = templates.into_iter()
        .map(FrontendComponentTemplate::from)
        .collect();
    
    Ok(ResponseJson(frontend_templates))
}

/// Create component template (admin endpoint)
pub async fn create_component_template(
    State(services): State<AppServices>,
    Json(template_data): Json<FrontendComponentTemplate>
) -> Result<(StatusCode, ResponseJson<FrontendComponentTemplate>), AppError> {
    validate_text_content(&template_data.name, 100)?;
    validate_text_content(&template_data.component_type, 50)?;
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let new_template = NewComponentTemplate {
        name: template_data.name,
        component_type: template_data.component_type,
        template_data: template_data.template_data,
        breakpoints: template_data.breakpoints,
        width_setting: template_data.width_setting,
        max_width: template_data.max_width,
        is_default: template_data.is_default,
        is_active: template_data.is_active,
    };
    
    use diesel::prelude::*;
    use crate::schema::component_templates;
    
    let created_template = diesel::insert_into(component_templates::table)
        .values(&new_template)
        .get_result::<ComponentTemplate>(&mut conn)?;
    
    Ok((StatusCode::CREATED, ResponseJson(FrontendComponentTemplate::from(created_template))))
}

/// Update component template (admin endpoint)
pub async fn update_component_template(
    State(services): State<AppServices>,
    Path(id): Path<i32>,
    Json(template_data): Json<FrontendComponentTemplate>
) -> Result<ResponseJson<FrontendComponentTemplate>, AppError> {
    validate_text_content(&template_data.name, 100)?;
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    use diesel::prelude::*;
    use crate::schema::component_templates;
    
    // Check if template exists
    let _existing_template = component_templates::table
        .find(id)
        .first::<ComponentTemplate>(&mut conn)
        .optional()?
        .ok_or_else(|| AppError::NotFound("Component template not found".to_string()))?;
    
    // Debug: Log what we're about to save
    println!("🔧 Backend: Updating template ID {} with data: {}", id, serde_json::to_string_pretty(&template_data.template_data).unwrap_or_default());
    
    let update_data = UpdateComponentTemplate {
        name: Some(template_data.name),
        template_data: Some(template_data.template_data),
        breakpoints: Some(template_data.breakpoints),
        width_setting: Some(template_data.width_setting),
        max_width: Some(template_data.max_width),
        is_default: Some(template_data.is_default),
        is_active: Some(template_data.is_active),
        updated_at: Some(chrono::Utc::now().naive_utc()),
    };
    
    let updated_template = diesel::update(component_templates::table.find(id))
        .set(update_data)
        .get_result::<ComponentTemplate>(&mut conn)?;
    
    Ok(ResponseJson(FrontendComponentTemplate::from(updated_template)))
}

// Toggle component template active state
pub async fn toggle_component_template(
    State(services): State<AppServices>,
    Path(id): Path<i32>
) -> Result<ResponseJson<FrontendComponentTemplate>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseConnection(e.to_string()))?;

    use crate::schema::component_templates;
    use diesel::prelude::*;
    
    // Get current template to check current active state
    let current_template = component_templates::table
        .find(id)
        .first::<ComponentTemplate>(&mut conn)
        .map_err(|e| AppError::DatabaseQuery(e.to_string()))?;
    
    // Toggle the active state
    let new_active_state = !current_template.is_active;
    
    let update_data = UpdateComponentTemplate {
        name: None,
        template_data: None,
        breakpoints: None,
        width_setting: None,
        max_width: None,
        is_default: None,
        is_active: Some(new_active_state),
        updated_at: Some(chrono::Utc::now().naive_utc()),
    };
    
    let updated_template = diesel::update(component_templates::table.find(id))
        .set(update_data)
        .get_result::<ComponentTemplate>(&mut conn)?;
    
    Ok(ResponseJson(FrontendComponentTemplate::from(updated_template)))
}