use crate::components::{MenuStyle, MenuProperties};
use crate::services::api_service::{SettingData, update_settings, get_settings, get_public_settings};

pub async fn save_menu_style(area: &str, style: &MenuStyle) -> Result<(), String> {
    web_sys::console::log_1(&format!("💾 SAVE_MENU_STYLE: Saving style for area '{}' to DATABASE", area).into());
    
    // Apply styles to DOM immediately for visual feedback
    apply_menu_style_to_dom(area, style);
    
    // Save to database using the settings API
    let style_json = serde_json::to_string(style)
        .map_err(|e| format!("Failed to serialize menu style: {}", e))?;
    
    let settings_data = vec![
        SettingData {
            key: format!("menu_style_{}", area),
            value: style_json,
            setting_type: "menu".to_string(),
            description: Some(format!("Menu style configuration for {} area", area)),
        }
    ];
    
    match update_settings(settings_data).await {
        Ok(_) => {
            web_sys::console::log_1(&format!("✅ Successfully saved menu style for area '{}' to database", area).into());
            
            // Also store locally as backup/cache
            store_menu_style_locally(area, style);
            
            // Force a small delay and reapply to ensure it takes effect
            let area_clone = area.to_string();
            let style_clone = style.clone();
            gloo_timers::callback::Timeout::new(50, move || {
                web_sys::console::log_1(&format!("🔄 REAPPLYING: Menu style for area '{}'", area_clone).into());
                apply_menu_style_to_dom(&area_clone, &style_clone);
            }).forget();
            
            Ok(())
        }
        Err(e) => {
            web_sys::console::log_1(&format!("❌ Failed to save menu style to database: {}", e).into());
            
            // Fallback to localStorage if database save fails
            web_sys::console::log_1(&"📦 Falling back to localStorage storage".into());
            store_menu_style_locally(area, style);
            
            Err(format!("Failed to save to database: {}", e))
        }
    }
}

pub async fn load_menu_style(area: &str) -> Result<Option<MenuStyle>, String> {
    web_sys::console::log_1(&format!("📂 LOAD_MENU_STYLE: Loading style for area '{}' from DATABASE", area).into());
    
    // Try to load from database first using public endpoint (no auth required)
    match get_public_settings(Some("menu")).await {
        Ok(settings) => {
            let key = format!("menu_style_{}", area);
            
            // Find the setting for this area
            if let Some(setting) = settings.iter().find(|s| s.setting_key == key) {
                if let Some(ref value) = setting.setting_value {
                    match serde_json::from_str::<MenuStyle>(value) {
                        Ok(style) => {
                            web_sys::console::log_1(&format!("✅ Successfully loaded menu style for area '{}' from database", area).into());
                            
                            // Also cache locally for faster access
                            store_menu_style_locally(area, &style);
                            
                            return Ok(Some(style));
                        }
                        Err(e) => {
                            web_sys::console::log_1(&format!("❌ Failed to parse menu style from database: {}", e).into());
                        }
                    }
                } else {
                    web_sys::console::log_1(&format!("⚠️ Setting found but value is null for area '{}'", area).into());
                }
            } else {
                web_sys::console::log_1(&format!("ℹ️ No menu style found in database for area '{}'", area).into());
            }
        }
        Err(e) => {
            web_sys::console::log_1(&format!("❌ Failed to load settings from database: {}", e).into());
        }
    }
    
    // Fallback to localStorage if database load fails
    web_sys::console::log_1(&format!("📦 Falling back to localStorage for area '{}'", area).into());
    Ok(load_menu_style_locally(area))
}

pub fn load_and_apply_saved_menu_styles() {
    web_sys::console::log_1(&"🔄 Loading and applying all saved menu styles from DATABASE...".into());
    
    // List of menu areas to check
    let areas = ["header", "footer", "sidebar"];
    
    // Load from database asynchronously
    wasm_bindgen_futures::spawn_local(async move {
        for area in areas.iter() {
            match load_menu_style(area).await {
                Ok(Some(style)) => {
                    web_sys::console::log_1(&format!("🎨 Applying saved style for area '{}'", area).into());
                    apply_menu_style_to_dom(area, &style);
                }
                Ok(None) => {
                    web_sys::console::log_1(&format!("ℹ️ No saved style found for area '{}'", area).into());
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("❌ Failed to load style for area '{}': {}", area, e).into());
                    
                    // Fallback to localStorage
                    if let Some(style) = load_menu_style_locally(area) {
                        web_sys::console::log_1(&format!("📦 Using localStorage fallback for area '{}'", area).into());
                        apply_menu_style_to_dom(area, &style);
                    }
                }
            }
        }
    });
}

pub fn force_refresh_menu_styles() {
    web_sys::console::log_1(&"🔄 FORCE REFRESH: Reapplying all menu styles from DATABASE...".into());
    
    // Remove all existing menu style elements first
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        let areas = ["header", "footer", "sidebar"];
        for area in areas.iter() {
            let style_id = format!("menu-style-{}", area);
            if let Some(existing) = document.get_element_by_id(&style_id) {
                web_sys::console::log_1(&format!("🗑️ FORCE REFRESH: Removing existing style: {}", style_id).into());
                existing.remove();
            }
        }
    }
    
    // Small delay then reapply all styles from database
    gloo_timers::callback::Timeout::new(10, move || {
        load_and_apply_saved_menu_styles();
    }).forget();
}

pub fn apply_menu_style_to_dom(area: &str, style: &MenuStyle) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    // Generate CSS for the menu style
    let css = generate_menu_css(area, &style.properties);
    
    // Log for debugging
    web_sys::console::log_1(&format!("🎨 Applying menu style for area '{}': {}", area, css).into());
    
    // Remove existing style element if it exists
    let style_id = format!("menu-style-{}", area);
    if let Some(existing) = document.get_element_by_id(&style_id) {
        web_sys::console::log_1(&format!("🗑️ Removing existing style element: {}", style_id).into());
        existing.remove();
    }
    
    // Create new style element
    let style_element = document.create_element("style").unwrap();
    style_element.set_id(&style_id);
    style_element.set_text_content(Some(&css));
    
    // Append to head
    if let Some(head) = document.head() {
        let _ = head.append_child(&style_element);
        web_sys::console::log_1(&format!("✅ Applied menu style for area '{}'", area).into());
    } else {
        web_sys::console::log_1(&"❌ Could not find document head to append style".into());
    }
}

fn generate_menu_css(area: &str, properties: &MenuProperties) -> String {
    // Use higher specificity selectors to ensure our styles override defaults
    let selector = match area {
        "header" => "body .site-header .site-nav",
        "footer" => "body .site-footer .site-nav", 
        "floating" => "body .floating-nav",
        _ => "body .menu-custom"
    };
    
    let container_selector = match area {
        "header" => "body .site-header",
        "footer" => "body .site-footer", 
        "floating" => "body .floating-nav",
        _ => "body .menu-custom"
    };

    let flex_direction = if properties.layout == "vertical" { "column" } else { "row" };
    let justify_content = match properties.alignment.as_str() {
        "left" => "flex-start",
        "center" => "center",
        "right" => "flex-end",
        _ => "flex-start"
    };

    // Generate animation-specific CSS
    let hover_transform = generate_hover_transform(properties);
    let hover_box_shadow = generate_hover_box_shadow(properties);
    let underline_css = if properties.enable_underline_animation {
        generate_underline_css(selector, properties)
    } else {
        String::new()
    };

    format!(r#"
/* Override hardcoded header background */
{} {{
    background: {} !important;
}}

/* Menu Container Styles */
{} {{
    display: flex !important;
    flex-direction: {} !important;
    justify-content: {} !important;
    align-items: center !important;
    gap: {} !important;
    padding: {} !important;
    background: {} !important;
    border-radius: {} !important;
    border: {} !important;
    box-shadow: {} !important;
    transition: gap 0.3s ease, padding 0.3s ease !important;
}}

/* Menu Item Base Styles */
{} a {{
    color: {} !important;
    text-decoration: none !important;
    font-size: {} !important;
    font-weight: {} !important;
    text-transform: {} !important;
    padding: {} !important;
    border-radius: {} !important;
    transition: all {} {} !important;
    cursor: pointer !important;
    will-change: transform, background-color, color, box-shadow, border-color !important;
    position: relative !important;
    display: inline-block !important;
    border: {} solid {} !important;
    overflow: hidden !important;
    {}
}}

/* Hover States with Advanced Animations */
{} a:hover {{
    color: {} !important;
    background: {} !important;
    transform: {} !important;
    box-shadow: {} !important;
    border-color: {} !important;
    border-width: {} !important;
    {}
}}

/* Active States */
{} a:active {{
    color: {} !important;
    background: {} !important;
    transform: scale(0.98) !important;
}}

/* Focus States for Accessibility */
{} a:focus {{
    outline: 2px solid {} !important;
    outline-offset: 2px !important;
}}

{}

/* Mobile Responsive */
@media (max-width: {}) {{
    {} {{
        flex-direction: {} !important;
        gap: 8px !important;
    }}
    
    {} a {{
        font-size: calc({} * 0.9) !important;
        padding: calc({} * 0.8) !important;
    }}
}}
"#,
        // Container override and styles
        container_selector, properties.background,
        selector, flex_direction, justify_content, properties.item_spacing, properties.padding,
        properties.background, properties.border_radius, properties.border, properties.shadow,
        
        // Menu item base styles
        selector, properties.text_color, properties.font_size, properties.font_weight,
        properties.text_transform, properties.padding, properties.border_radius,
        properties.hover_animation_duration, properties.hover_animation_easing,
        properties.hover_border_width, properties.hover_border_color,
        String::new(), // Removed gradient shift
        
        // Hover states with animations
        selector, properties.hover_color, properties.hover_background,
        hover_transform, hover_box_shadow, properties.hover_border_color, properties.hover_border_width,
        String::new(), // Removed gradient shift
        
        // Active states
        selector, properties.active_color, properties.active_background,
        
        // Focus states
        selector, properties.hover_glow_color,
        
        // Animation components
        underline_css,
        
        // Mobile responsive
        properties.mobile_breakpoint, selector,
        if properties.mobile_layout == "stack" { "column" } else { "row" },
        selector, properties.font_size, properties.padding
    )
}

fn store_menu_style_locally(area: &str, style: &MenuStyle) {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let key = format!("menu_style_{}", area);
        if let Ok(json) = serde_json::to_string(style) {
            web_sys::console::log_1(&format!("💾 Saved menu style for area '{}' to localStorage", area).into());
            let _ = storage.set_item(&key, &json);
        }
    }
}

fn load_menu_style_locally(area: &str) -> Option<MenuStyle> {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let key = format!("menu_style_{}", area);
        if let Ok(Some(json)) = storage.get_item(&key) {
            if let Ok(style) = serde_json::from_str::<MenuStyle>(&json) {
                web_sys::console::log_1(&format!("📂 Loaded menu style for area '{}' from localStorage", area).into());
                return Some(style);
            }
        }
    }
    None
}

fn generate_hover_transform(properties: &MenuProperties) -> String {
    let mut transforms = Vec::new();
    
    // Add lift effect
    if properties.hover_lift_distance != "0px" && !properties.hover_lift_distance.is_empty() {
        transforms.push(format!("translateY(-{})", properties.hover_lift_distance));
    }
    
    // Add scale effect
    if properties.hover_scale != "1.0" && properties.hover_scale != "1" {
        transforms.push(format!("scale({})", properties.hover_scale));
    }
    
    // Add animation type specific transforms
    match properties.hover_animation_type.as_str() {
        "lift" => {
            if !transforms.iter().any(|t| t.contains("translateY")) {
                transforms.push("translateY(-2px)".to_string());
            }
        },
        "scale" => {
            if !transforms.iter().any(|t| t.contains("scale")) {
                transforms.push("scale(1.05)".to_string());
            }
        },
        _ => {}
    }
    
    if transforms.is_empty() {
        properties.hover_transform.clone()
    } else {
        transforms.join(" ")
    }
}

fn generate_hover_box_shadow(properties: &MenuProperties) -> String {
    let mut shadows = Vec::new();
    
    // Add glow effect
    if properties.hover_animation_type == "glow" || properties.hover_glow_intensity != "0px" {
        let intensity = if properties.hover_glow_intensity.is_empty() { "8px" } else { &properties.hover_glow_intensity };
        shadows.push(format!("0 0 {} {}", intensity, properties.hover_glow_color));
    }
    
    // Add lift shadow - removed automatic shadow for lift animation
    // Users can add their own shadow via the glow effect if desired
    
    // Add scale shadow
    if properties.hover_animation_type == "scale" {
        shadows.push("0 8px 25px rgba(0, 0, 0, 0.2)".to_string());
    }
    
    if shadows.is_empty() {
        "none".to_string()
    } else {
        shadows.join(", ")
    }
}



fn generate_underline_css(selector: &str, properties: &MenuProperties) -> String {
    if !properties.enable_underline_animation {
        return String::new();
    }
    
    // Simple slide underline animation
    format!(r#"
/* Underline Animation */
{} a::after {{
    content: '' !important;
    position: absolute !important;
    bottom: 0 !important;
    left: 0 !important;
    width: 0 !important;
    height: {} !important;
    background: {} !important;
    transition: width {} {} !important;
}}

{} a:hover::after {{
    width: 100% !important;
}}
"#, selector, properties.underline_thickness, properties.underline_color, 
    properties.hover_animation_duration, properties.hover_animation_easing, selector)
}

