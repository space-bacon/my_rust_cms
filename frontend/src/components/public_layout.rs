use yew::prelude::*;
use crate::services::navigation_service::{get_navigation_by_area, get_component_templates, ComponentTemplate};
use crate::services::api_service::get_public_settings;
use std::collections::HashMap;
use crate::pages::public::PublicPage;
use crate::pages::admin::design_system::{PublicColorScheme, apply_public_css_variables};
use wasm_bindgen::JsCast;
use crate::services::auth_context::use_auth;
use crate::components::EnhancedLiveEditSystem;

#[derive(Properties, PartialEq)]
pub struct PublicLayoutProps {
    pub children: Children,
    pub on_admin_click: Callback<()>,

    pub on_navigate: Option<Callback<PublicPage>>,
    pub current_page: String,
}

// Helper function to render the site logo based on header template settings
fn render_site_logo(component_templates: &[ComponentTemplate], site_title: &str) -> Html {
    let header_template = component_templates.iter()
        .find(|t| t.component_type == "header" && t.is_active);
    
    match header_template {
        Some(template) => {
            let logo_type = template.template_data.get("logo_type")
                .and_then(|v| v.as_str())
                .unwrap_or("text");
            
            if logo_type == "image" {
                if let Some(logo_url) = template.template_data.get("logo_url")
                    .and_then(|v| v.as_str())
                    .filter(|url| !url.trim().is_empty()) {
                    
                    let logo_height = template.template_data.get("logo_height")
                        .and_then(|v| v.as_str())
                        .unwrap_or("40px");
                    
                    html! {
                        <div class="site-logo">
                            <img 
                                src={logo_url.to_string()} 
                                alt={site_title.to_string()} 
                                style={format!("height: {}; max-width: 200px; object-fit: contain;", logo_height)}
                            />
                        </div>
                    }
                } else {
                    // Fallback to text if image URL is empty
                    html! { <h1 class="site-title">{site_title}</h1> }
                }
            } else {
                // Text logo
                html! { <h1 class="site-title">{site_title}</h1> }
            }
        }
        None => {
            // No header template, use text
            html! { <h1 class="site-title">{site_title}</h1> }
        }
    }
}

#[function_component(PublicLayout)]
pub fn public_layout(props: &PublicLayoutProps) -> Html {
    let auth = use_auth();
    let header_navigation_items = use_state(Vec::new);
    let footer_navigation_items = use_state(Vec::new);
    let component_templates = use_state(Vec::<ComponentTemplate>::new);
    let loading = use_state(|| true);
    let admin_button_visible = use_state(|| true); // Default to true until loaded
    let site_title = use_state(|| "My Rust CMS".to_string());
    let acid_mode = use_state(|| false);
    let site_style = use_state(|| String::new());
    let inner_container_style = use_state(|| String::new());
    let live_edit_enabled = use_state(|| false);

    // Load navigation items, component templates, and admin button setting
    {
        let header_navigation_items = header_navigation_items.clone();
        let footer_navigation_items = footer_navigation_items.clone();
        let component_templates = component_templates.clone();
        let loading = loading.clone();
        let admin_button_visible = admin_button_visible.clone();
        let site_title = site_title.clone();
        let acid_mode = acid_mode.clone();
        let site_style = site_style.clone();
        let inner_container_style = inner_container_style.clone();

        use_effect_with_deps(move |_| {
            web_sys::console::log_1(&"PublicLayout: Starting to fetch navigation items, templates, and settings".into());
            wasm_bindgen_futures::spawn_local(async move {
                // Load header and footer navigation items
                let header_nav_result = get_navigation_by_area("header").await;
                let footer_nav_result = get_navigation_by_area("footer").await;
                
                // Load component templates
                web_sys::console::log_1(&"🔄 Loading component templates from public endpoint...".into());
                let templates_result = get_component_templates().await;
                
                // Load site and container settings
                let settings_result = get_public_settings(Some("site")).await;
                let container_settings_result = get_public_settings(Some("container")).await;
                
                match header_nav_result {
                    Ok(items) => {
                        web_sys::console::log_1(&format!("Header navigation items loaded: {:?}", items).into());
                        header_navigation_items.set(items);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Header navigation error: {:?}", e).into());
                    }
                }
                
                match footer_nav_result {
                    Ok(items) => {
                        web_sys::console::log_1(&format!("Footer navigation items loaded: {:?}", items).into());
                        footer_navigation_items.set(items);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Footer navigation error: {:?}", e).into());
                    }
                }
                
                match templates_result {
                    Ok(templates) => {
                        web_sys::console::log_1(&format!("🔄 Loaded {} component templates from public endpoint", templates.len()).into());
                        for template in &templates {
                            web_sys::console::log_1(&format!("🔄 Template: ID={}, name='{}', type='{}', active={}", 
                                template.id, template.name, template.component_type, template.is_active).into());
                        }
                        component_templates.set(templates);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("❌ Error loading component templates: {:?}", e).into());
                    }
                }
                
                match settings_result {
                    Ok(settings) => {
                        web_sys::console::log_1(&format!("Settings loaded: {:?}", settings).into());
                        // Find admin button setting
                        if let Some(setting) = settings.iter().find(|s| s.setting_key == "admin_button_visible") {
                            if let Some(value) = &setting.setting_value {
                                let visible = value.parse::<bool>().unwrap_or(true);
                                admin_button_visible.set(visible);
                                web_sys::console::log_1(&format!("Admin button visibility set to: {}", visible).into());
                            }
                        }
                        
                        // Find site title setting
                        if let Some(setting) = settings.iter().find(|s| s.setting_key == "site_title") {
                            if let Some(ref value) = setting.setting_value {
                                if !value.trim().is_empty() {
                                    site_title.set(value.trim().to_string());
                                    web_sys::console::log_1(&format!("Site title set to: {}", value.trim()).into());
                                }
                            }
                        }
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Settings error: {:?}", e).into());
                        // Keep default value of true if settings fail to load
                    }
                }

                // Parse container settings for acid mode
                match container_settings_result {
                    Ok(settings) => {
                        let mut map: HashMap<String, String> = HashMap::new();
                        for s in &settings {
                            if let Some(val) = s.setting_value.clone() {
                                map.insert(s.setting_key.clone(), val);
                            }
                        }

                        if let Some(setting) = settings.iter().find(|s| s.setting_key == "container_acid_mode") {
                            if let Some(ref value) = setting.setting_value {
                                let enabled = value.trim().eq_ignore_ascii_case("true");
                                acid_mode.set(enabled);
                                web_sys::console::log_1(&format!("Acid mode set to: {}", enabled).into());
                            }
                        }

                        // Apply background and layout from container settings
                        let width_type = map.get("container_width_type").cloned().unwrap_or_default();
                        let max_width = map.get("container_max_width").cloned().unwrap_or_default();
                        let horizontal_padding = map.get("container_horizontal_padding").cloned().unwrap_or_default();

                        let mut container_css: Vec<String> = Vec::new();
                        if width_type == "fixed" && !max_width.is_empty() { container_css.push(format!("max-width: {}", max_width)); }
                        if !horizontal_padding.is_empty() { container_css.push(format!("padding-left: {}; padding-right: {}", horizontal_padding, horizontal_padding)); }
                        inner_container_style.set(container_css.join("; "));

                        // Backgrounds
                        let background_type = map.get("container_background_type").map(|s| s.as_str()).unwrap_or("none");
                        let overlay_color_raw = map.get("container_overlay_color").cloned().unwrap_or_default();
                        let overlay_opacity_raw = map.get("container_overlay_opacity").cloned().unwrap_or_else(|| "0".to_string());

                        // Helper: parse opacity which may be a float (0..1) or percentage (e.g., "30%")
                        fn parse_opacity(value: &str) -> f32 {
                            let v = value.trim();
                            if let Some(stripped) = v.strip_suffix('%') {
                                if let Ok(p) = stripped.trim().parse::<f32>() { return (p / 100.0).clamp(0.0, 1.0); }
                                return 0.0;
                            }
                            v.parse::<f32>().ok().map(|f| f.clamp(0.0, 1.0)).unwrap_or(0.0)
                        }

                        // Helper: build overlay gradient layer from color and opacity
                        fn build_overlay_layer(color: &str, opacity: f32) -> Option<String> {
                            if opacity <= 0.0 { return None; }
                            let c = color.trim();
                            if c.is_empty() { return None; }
                            // Supported: #RRGGBB, rgb(r,g,b), rgba(r,g,b,a)
                            if c.starts_with('#') && c.len() == 7 {
                                let r = u8::from_str_radix(&c[1..3], 16).unwrap_or(0);
                                let g = u8::from_str_radix(&c[3..5], 16).unwrap_or(0);
                                let b = u8::from_str_radix(&c[5..7], 16).unwrap_or(0);
                                Some(format!(
                                    "linear-gradient(rgba({}, {}, {}, {}), rgba({}, {}, {}, {}))",
                                    r, g, b, opacity, r, g, b, opacity
                                ))
                            } else if c.starts_with("rgb(") || c.starts_with("rgba(") {
                                // Normalize into rgba with provided opacity by replacing any existing alpha
                                // Simple approach: extract numbers
                                let inside = c.trim_start_matches("rgba(").trim_start_matches("rgb(").trim_end_matches(")");
                                let parts: Vec<&str> = inside.split(',').map(|s| s.trim()).collect();
                                if parts.len() >= 3 {
                                    let r = parts.get(0).and_then(|s| s.parse::<u8>().ok()).unwrap_or(0);
                                    let g = parts.get(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(0);
                                    let b = parts.get(2).and_then(|s| s.parse::<u8>().ok()).unwrap_or(0);
                                    Some(format!(
                                        "linear-gradient(rgba({}, {}, {}, {}), rgba({}, {}, {}, {}))",
                                        r, g, b, opacity, r, g, b, opacity
                                    ))
                                } else { None }
                            } else {
                                None
                            }
                        }

                        let overlay_alpha = parse_opacity(&overlay_opacity_raw);
                        let overlay_layer: Option<String> = build_overlay_layer(&overlay_color_raw, overlay_alpha);

                        let mut bg_layers: Vec<String> = Vec::new();
                        if let Some(layer) = overlay_layer { bg_layers.push(layer); }

                        match background_type {
                            "color" => {
                                if let Some(color) = map.get("container_background_color") { bg_layers.push(color.clone()); }
                            }
                            "gradient" => {
                                let from = map.get("container_gradient_from").cloned().unwrap_or("#000000".to_string());
                                let to = map.get("container_gradient_to").cloned().unwrap_or("#222222".to_string());
                                let angle = map.get("container_gradient_angle").cloned().unwrap_or("180deg".to_string());
                                bg_layers.push(format!("linear-gradient({} , {} , {} )", angle, from, to));
                            }
                            "image" => {
                                let url = map.get("container_background_image_url").cloned().unwrap_or_default();
                                if !url.is_empty() { bg_layers.push(format!("url('{}')", url)); }
                            }
                            "video" => {
                                // video will be rendered as element below; optional poster image layer could be added later
                            }
                            _ => {}
                        }

                        let mut bg_css: Vec<String> = Vec::new();
                        if !bg_layers.is_empty() {
                            bg_css.push(format!("background: {}", bg_layers.join(", ")));
                        }
                        if let Some(size) = map.get("container_background_image_size") { if !size.is_empty() { bg_css.push(format!("background-size: {}", size)); } }
                        if let Some(pos) = map.get("container_background_image_position") { if !pos.is_empty() { bg_css.push(format!("background-position: {}", pos)); } }
                        if matches!(background_type, "image" | "video") { bg_css.push("background-repeat: no-repeat".to_string()); bg_css.push("background-attachment: scroll".to_string()); }

                        let css = bg_css.join("; ");
                        if !css.is_empty() { site_style.set(css); }

                        // Sync container settings to <body> data-* attributes for live editing/preview
                        if let Some(window) = web_sys::window() {
                            if let Some(document) = window.document() {
                                if let Some(body) = document.body() {
                                    let _ = body.set_attribute("data-bg-type", background_type);
                                    let _ = body.set_attribute("data-acid-enabled", if *acid_mode { "true" } else { "false" });
                                    if let Some(url) = map.get("container_background_video_url") {
                                        let _ = body.set_attribute("data-bg-video-url", url);
                                    }
                                    if let Some(looping) = map.get("container_background_video_loop") {
                                        let _ = body.set_attribute("data-bg-video-loop", looping);
                                    }
                                    if let Some(autoplay) = map.get("container_background_video_autoplay") {
                                        let _ = body.set_attribute("data-bg-video-autoplay", autoplay);
                                    }
                                    if let Some(muted) = map.get("container_background_video_muted") {
                                        let _ = body.set_attribute("data-bg-video-muted", muted);
                                    }
                                    let _ = body.set_attribute("data-overlay-color", &overlay_color_raw);
                                    let _ = body.set_attribute("data-overlay-opacity", &overlay_opacity_raw);
                                }
                            }
                        }

                        // Create/update a dedicated background video layer attached to <body>
                        if let Some(window) = web_sys::window() {
                            if let Some(document) = window.document() {
                                // Remove any existing layer by id
                                if let Some(existing) = document.get_element_by_id("bg-video-layer") {
                                    if background_type != "video" { let _ = existing.remove(); }
                                }
                                if background_type == "video" {
                                    if let Some(url) = map.get("container_background_video_url").cloned() {
                                        if !url.is_empty() {
                                            let looping = map.get("container_background_video_loop").cloned().unwrap_or_else(|| "true".to_string());
                                            let autoplay = map.get("container_background_video_autoplay").cloned().unwrap_or_else(|| "true".to_string());
                                            let muted = map.get("container_background_video_muted").cloned().unwrap_or_else(|| "true".to_string());

                                            let container = document.get_element_by_id("bg-video-layer").unwrap_or_else(|| {
                                                let div = document.create_element("div").unwrap();
                                                div.set_attribute("id", "bg-video-layer").ok();
                                                div.set_attribute("style", "position: fixed; inset: 0; width: 100%; height: 100%; z-index: -2; pointer-events: none; overflow: hidden;").ok();
                                                document.body().unwrap().append_child(&div).ok();
                                                div
                                            });

                                            // YouTube detection
                                            let is_youtube = url.contains("youtube.com") || url.contains("youtu.be");
                                            if is_youtube {
                                                // Extract id
                                                let id = (|| {
                                                    if let Some(idx) = url.find("youtu.be/") { return url[idx+9..].split(['?', '&', '#']).next().map(|s| s.to_string()); }
                                                    if let Some(idx) = url.find("watch?v=") { return url[idx+8..].split(['&', '#']).next().map(|s| s.to_string()); }
                                                    if let Some(idx) = url.find("/shorts/") { return url[idx+8..].split(['?', '&', '#']).next().map(|s| s.to_string()); }
                                                    if let Some(idx) = url.find("/embed/") { return url[idx+7..].split(['?', '&', '#']).next().map(|s| s.to_string()); }
                                                    None
                                                })();
                                                if let Some(id) = id {
                                                    let embed_src = format!(
                                                        "https://www.youtube.com/embed/{}?autoplay={}&mute={}&loop={}&playlist={}&controls=0&showinfo=0&modestbranding=1&iv_load_policy=3&rel=0&playsinline=1",
                                                        id,
                                                        if autoplay == "true" { 1 } else { 0 },
                                                        if muted == "true" { 1 } else { 0 },
                                                        if looping == "true" { 1 } else { 0 },
                                                        id
                                                    );
                                                    container.set_inner_html(&format!(
                                                        "<iframe src=\"{}\" style=\"position:absolute; inset:0; width:100%; height:100%; border:0; pointer-events:none;\" allow=\"autoplay; encrypted-media; picture-in-picture\"></iframe>",
                                                        embed_src
                                                    ));
                                                }
                                            } else {
                                                container.set_inner_html(&format!(
                                                    "<video src=\"{}\" {} {} {} playsinline style=\"position:absolute; inset:0; width:100%; height:100%; object-fit:cover;\"></video>",
                                                    url,
                                                    if autoplay == "true" { "autoplay" } else { "" },
                                                    if looping == "true" { "loop" } else { "" },
                                                    if muted == "true" { "muted" } else { "" },
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Container settings error: {:?}", e).into());
                    }
                }
                
                loading.set(false);
            });
            // Cleanup when PublicLayout unmounts: remove background media & data attrs
            || {
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(el) = document.get_element_by_id("bg-video-layer") {
                            // Attempt to pause any <video> before removal
                            if let Some(video_el) = el.query_selector("video").ok().flatten() {
                                let _ = js_sys::Reflect::get(&video_el, &wasm_bindgen::JsValue::from_str("pause"))
                                    .ok()
                                    .and_then(|f| f.dyn_into::<js_sys::Function>().ok())
                                    .and_then(|f| f.call0(&video_el).ok());
                            }
                            let _ = el.remove();
                        }
                        if let Some(body) = document.body() {
                            let _ = body.remove_attribute("data-bg-type");
                            let _ = body.remove_attribute("data-bg-video-url");
                            let _ = body.remove_attribute("data-bg-video-loop");
                            let _ = body.remove_attribute("data-bg-video-autoplay");
                            let _ = body.remove_attribute("data-bg-video-muted");
                            let _ = body.remove_attribute("data-overlay-color");
                            let _ = body.remove_attribute("data-overlay-opacity");
                        }
                    }
                }
            }
        }, ());
    }

    // Apply default public theme on component mount
    {
        use_effect_with_deps(move |_| {
            web_sys::console::log_1(&"PublicLayout: Applying default public theme".into());
            let default_scheme = PublicColorScheme::default();
            apply_public_css_variables(&default_scheme);
            || ()
        }, ());
    }

    // Mirror acid-mode class to <body> for maximum selector compatibility
    {
        let acid_mode = acid_mode.clone();
        use_effect_with_deps(move |enabled| {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        let existing = body.get_attribute("class").unwrap_or_default();
                        let has = existing.split_whitespace().any(|c| c == "acid-mode");
                        let new_class = if **enabled {
                            if has { existing } else { format!("{}{}acid-mode", existing, if existing.is_empty() { "" } else { " " }) }
                        } else {
                            existing
                                .split_whitespace()
                                .filter(|c| *c != "acid-mode")
                                .collect::<Vec<_>>()
                                .join(" ")
                        };
                        let _ = body.set_attribute("class", &new_class);
                    }
                }
            }
            || ()
        }, acid_mode);
    }

    let on_admin_click = {
        let callback = props.on_admin_click.clone();
        Callback::from(move |_| callback.emit(()))
    };

    // Helper function to check if component is active
    let is_component_active = {
        let component_templates = component_templates.clone();
        move |component_type: &str| -> bool {
            component_templates.iter()
                .any(|t| t.component_type == component_type && t.is_active)
        }
    };

    // Helper function to get template styles (safe subset for public UI)
    let get_component_style = {
        let component_templates = component_templates.clone();
        move |component_type: &str| -> String {
                // Debug: Show all templates of this type
    let matching_templates: Vec<_> = component_templates.iter()
        .filter(|t| t.component_type == component_type && t.is_active)
        .collect();
    
    web_sys::console::log_1(&format!("🔍 Template Selection Debug for '{}' type:", component_type).into());
    for template in &matching_templates {
        web_sys::console::log_1(&format!("  - ID={}, name='{}', is_default={}", 
            template.id, template.name, template.is_default).into());
    }
    
    // Try to find default template first
    let default_template = component_templates.iter()
        .find(|t| t.component_type == component_type && t.is_active && t.is_default);
    
    let fallback_template = component_templates.iter()
        .find(|t| t.component_type == component_type && t.is_active);
    
    let selected_template = default_template.or(fallback_template);
    
    if let Some(template) = selected_template {
        web_sys::console::log_1(&format!("🎯 Selected template: ID={}, name='{}', is_default={}", 
            template.id, template.name, template.is_default).into());
                let mut styles = Vec::new();
                
                if let Some(height) = template.template_data.get("height").and_then(|v| v.as_str()) {
                    let mut h = height.to_string();
                    
                    // Ensure height has px units
                    if !h.ends_with("px") {
                        h = format!("{}px", h);
                    }
                    
                    if component_type == "header" {
                        // For header, enforce minimum height of 110px
                        if let Some(stripped) = h.strip_suffix("px") {
                            if let Ok(px) = stripped.trim().parse::<i32>() {
                                if px < 110 { h = "110px".to_string(); }
                            }
                        }
                    }
                    
                    styles.push(format!("height: {}", h));
                } else if component_type == "header" {
                    styles.push("height: 110px".to_string());
                }
                
                // Only allow position overrides for non-header components to avoid layout breaks
                if component_type != "header" {
                    if let Some(position) = template.template_data.get("position").and_then(|v| v.as_str()) {
                        styles.push(format!("position: {}", position));
                    }
                }
                
                // Enhanced background handling with new properties
                let bg_type = template.template_data.get("bg_type").and_then(|v| v.as_str()).unwrap_or("color");
                
                match bg_type {
                    "color" => {
                        if let Some(mut bg_color) = template.template_data.get("bg_color").and_then(|v| v.as_str()) {
                            // For header, coerce white to black per default theme requirement
                            if component_type == "header" && bg_color.trim().eq_ignore_ascii_case("#ffffff") {
                                bg_color = "#000000";
                            }
                            styles.push(format!("background-color: {} !important", bg_color));
                        } else {
                            // Fallback to legacy background/background_color properties
                            if let Some(mut bg) = template.template_data.get("background").and_then(|v| v.as_str()) {
                                if component_type == "header" && bg.trim().eq_ignore_ascii_case("#ffffff") { bg = "#000000"; }
                                styles.push(format!("background: {} !important", bg));
                            } else if let Some(mut bg) = template.template_data.get("background_color").and_then(|v| v.as_str()) {
                                if component_type == "header" && bg.trim().eq_ignore_ascii_case("#ffffff") { bg = "#000000"; }
                                styles.push(format!("background-color: {} !important", bg));
                            } else if component_type == "header" {
                                styles.push("background-color: #000000 !important".to_string());
                            }
                        }
                    },
                    "image" => {
                        if let Some(bg_image) = template.template_data.get("bg_image").and_then(|v| v.as_str()) {
                            styles.push(format!("background-image: url({})", bg_image));
                            styles.push("background-size: cover".to_string());
                            styles.push("background-position: center".to_string());
                            styles.push("background-repeat: no-repeat".to_string());
                        }
                    },
                    "gradient" => {
                        // Check for custom gradient first
                        if let Some(custom_gradient) = template.template_data.get("bg_gradient_custom").and_then(|v| v.as_str()) {
                            if !custom_gradient.trim().is_empty() {
                                styles.push(format!("background: {} !important", custom_gradient));
                            } else {
                                // Fall back to individual color fields
                                let start_color = template.template_data.get("bg_gradient_start").and_then(|v| v.as_str()).unwrap_or("#667eea");
                                let end_color = template.template_data.get("bg_gradient_end").and_then(|v| v.as_str()).unwrap_or("#764ba2");
                                let direction = template.template_data.get("bg_gradient_direction").and_then(|v| v.as_str()).unwrap_or("135deg");
                                // Convert hyphenated directions to valid CSS syntax
                                let css_direction = match direction {
                                    "to-top" => "to top",
                                    "to-bottom" => "to bottom", 
                                    "to-left" => "to left",
                                    "to-right" => "to right",
                                    "to-top-left" => "to top left",
                                    "to-top-right" => "to top right",
                                    "to-bottom-left" => "to bottom left",
                                    "to-bottom-right" => "to bottom right",
                                    _ => direction // Keep degrees and other valid values as-is
                                };
                                let gradient = format!("linear-gradient({}, {}, {})", css_direction, start_color, end_color);
                                styles.push(format!("background: {} !important", gradient));
                            }
                        } else {
                            // Fall back to individual color fields
                            let start_color = template.template_data.get("bg_gradient_start").and_then(|v| v.as_str()).unwrap_or("#667eea");
                            let end_color = template.template_data.get("bg_gradient_end").and_then(|v| v.as_str()).unwrap_or("#764ba2");
                            let direction = template.template_data.get("bg_gradient_direction").and_then(|v| v.as_str()).unwrap_or("135deg");
                            // Convert hyphenated directions to valid CSS syntax
                            let css_direction = match direction {
                                "to-top" => "to top",
                                "to-bottom" => "to bottom", 
                                "to-left" => "to left",
                                "to-right" => "to right",
                                "to-top-left" => "to top left",
                                "to-top-right" => "to top right",
                                "to-bottom-left" => "to bottom left",
                                "to-bottom-right" => "to bottom right",
                                _ => direction // Keep degrees and other valid values as-is
                            };
                            let gradient = format!("linear-gradient({}, {}, {})", css_direction, start_color, end_color);
                            styles.push(format!("background: {} !important", gradient));
                        }
                    },
                    "video" => {
                        if let Some(bg_video) = template.template_data.get("bg_video").and_then(|v| v.as_str()) {
                            // For video backgrounds, we'll need to add the video element via JavaScript
                            // For now, add a dark background as fallback
                            styles.push("background-color: #000000".to_string());
                            styles.push("position: relative".to_string());
                            styles.push(format!("--bg-video-url: '{}'", bg_video));
                        }
                    },
                    _ => {
                        // Default color handling for backwards compatibility
                        if component_type == "header" {
                            styles.push("background-color: #000000".to_string());
                        }
                    }
                }
                
                // Shape mask handling - upper and lower
                let shape_mask_upper = template.template_data.get("shape_mask_upper").and_then(|v| v.as_str()).unwrap_or("none");
                let shape_mask_upper_scale = template.template_data.get("shape_mask_upper_scale").and_then(|v| v.as_str()).unwrap_or("100");
                let shape_mask_lower = template.template_data.get("shape_mask_lower").and_then(|v| v.as_str()).unwrap_or("none");
                let shape_mask_lower_scale = template.template_data.get("shape_mask_lower_scale").and_then(|v| v.as_str()).unwrap_or("100");
                
                // Handle shape masks - apply directly for initial load
                if shape_mask_upper != "none" || shape_mask_lower != "none" {
                    // Get shape-specific parameters based on shape type
                    let shape_mask_upper_frequency = if matches!(shape_mask_upper, "wave" | "triangle" | "zigzag") {
                        template.template_data.get("shape_mask_upper_frequency").and_then(|v| v.as_str()).unwrap_or("2")
                    } else { "2" };
                    let shape_mask_upper_direction = if matches!(shape_mask_upper, "curve" | "tilt") {
                        template.template_data.get("shape_mask_upper_direction").and_then(|v| v.as_str()).unwrap_or("positive")
                    } else { "positive" };
                    let shape_mask_upper_amplitude = if shape_mask_upper == "curve" {
                        template.template_data.get("shape_mask_upper_amplitude").and_then(|v| v.as_str()).unwrap_or("50")
                    } else { "50" };
                    let shape_mask_upper_degrees = if shape_mask_upper == "tilt" {
                        template.template_data.get("shape_mask_upper_degrees").and_then(|v| v.as_str()).unwrap_or("15")
                    } else { "15" };
                    
                    let shape_mask_lower_frequency = if matches!(shape_mask_lower, "wave" | "triangle" | "zigzag") {
                        template.template_data.get("shape_mask_lower_frequency").and_then(|v| v.as_str()).unwrap_or("2")
                    } else { "2" };
                    let shape_mask_lower_direction = if matches!(shape_mask_lower, "curve" | "tilt") {
                        template.template_data.get("shape_mask_lower_direction").and_then(|v| v.as_str()).unwrap_or("positive")
                    } else { "positive" };
                    let shape_mask_lower_amplitude = if shape_mask_lower == "curve" {
                        template.template_data.get("shape_mask_lower_amplitude").and_then(|v| v.as_str()).unwrap_or("50")
                    } else { "50" };
                    let shape_mask_lower_degrees = if shape_mask_lower == "tilt" {
                        let degrees = template.template_data.get("shape_mask_lower_degrees").and_then(|v| v.as_str()).unwrap_or("15");
                        web_sys::console::log_1(&format!("🔍 PublicLayout: Template ID {} shape_mask_lower_degrees = '{}'", template.id, degrees).into());
                        degrees
                    } else { "15" };
                    
                    // Generate clip-path for shape masks
                    let clip_path = generate_clip_path_for_template(
                        shape_mask_upper, shape_mask_upper_scale, shape_mask_upper_frequency, shape_mask_upper_direction, shape_mask_upper_amplitude, shape_mask_upper_degrees,
                        shape_mask_lower, shape_mask_lower_scale, shape_mask_lower_frequency, shape_mask_lower_direction, shape_mask_lower_amplitude, shape_mask_lower_degrees
                    );
                    
                    if !clip_path.is_empty() {
                        web_sys::console::log_1(&format!("🎭 Template Shape Mask - Component: {}", component_type).into());
                        web_sys::console::log_1(&format!("  Upper: {} (freq: {}, scale: {})", shape_mask_upper, shape_mask_upper_frequency, shape_mask_upper_scale).into());
                        web_sys::console::log_1(&format!("  Lower: {} (freq: {}, scale: {})", shape_mask_lower, shape_mask_lower_frequency, shape_mask_lower_scale).into());
                        web_sys::console::log_1(&format!("  Generated clip-path: {}", clip_path).into());
                        styles.push(format!("clip-path: {}", clip_path));
                    }
                }
                
                // Optional text color overrides via CSS variables for header/footer
                if component_type == "header" {
                    if let Some(text_color) = template.template_data.get("text_color").and_then(|v| v.as_str()) {
                        styles.push(format!("--header-text: {}", text_color));
                    } else {
                        // Default to white for readability on black header
                        styles.push("--header-text: #ffffff".to_string());
                    }
                    if let Some(text_hover) = template.template_data.get("text_hover_color").and_then(|v| v.as_str()) {
                        styles.push(format!("--header-text-hover: {}", text_hover));
                    } else {
                        styles.push("--header-text-hover: #f7fafc".to_string());
                    }
                    if let Some(nav_hover) = template.template_data.get("nav_hover_color").and_then(|v| v.as_str()) {
                        styles.push(format!("--nav-hover-color: {}", nav_hover));
                    }
                    if let Some(nav_underline) = template.template_data.get("nav_underline_color").and_then(|v| v.as_str()) {
                        styles.push(format!("--nav-underline-color: {}", nav_underline));
                    }
                    if let Some(thickness) = template.template_data.get("nav_underline_thickness").and_then(|v| v.as_str()) {
                        styles.push(format!("--nav-underline-thickness: {}", thickness));
                    }
                    if let Some(anim) = template.template_data.get("nav_underline_animation").and_then(|v| v.as_str()) {
                        styles.push(format!("--nav-underline-animation: {}", anim));
                    }
                }
                if component_type == "footer" {
                    if let Some(text_color) = template.template_data.get("text_color").and_then(|v| v.as_str()) {
                        styles.push(format!("--footer-text: {}", text_color));
                    }
                    if let Some(text_muted) = template.template_data.get("text_muted").and_then(|v| v.as_str()) {
                        styles.push(format!("--footer-text-muted: {}", text_muted));
                    }
                    if let Some(bg) = template.template_data.get("background").and_then(|v| v.as_str()) {
                        styles.push(format!("--footer-background: {}", bg));
                    }
                }

                if let Some(z_index_val) = template.template_data.get("z_index") {
                    if let Some(z) = z_index_val.as_i64() {
                        styles.push(format!("z-index: {}", z));
                    } else if let Some(z) = z_index_val.as_str() {
                        styles.push(format!("z-index: {}", z));
                    }
                }
                
                if let Some(padding) = template.template_data.get("padding").and_then(|v| v.as_str()) {
                    styles.push(format!("padding: {}", padding));
                }
                
                if let Some(margin) = template.template_data.get("margin").and_then(|v| v.as_str()) {
                    styles.push(format!("margin: {}", margin));
                }
                
                if let Some(border) = template.template_data.get("border").and_then(|v| v.as_str()) {
                    styles.push(format!("border: {}", border));
                }
                
                if let Some(box_shadow) = template.template_data.get("box_shadow").and_then(|v| v.as_str()) {
                    styles.push(format!("box-shadow: {}", box_shadow));
                }
                
                let style_string = styles.join("; ");
                if !style_string.is_empty() {
                    web_sys::console::log_1(&format!("Applying {} template styles: {}", component_type, style_string).into());
                }
                style_string
            } else {
                String::new()
            }
        }
    };

    // Global style variables derived from specific component templates (e.g., posts_list)
    let global_style_vars = {
        let component_templates = component_templates.clone();
        move || -> String {
            let mut vars: Vec<String> = Vec::new();
            if let Some(posts_tpl) = component_templates.iter().find(|t| t.component_type == "posts_list" && t.is_active) {
                if let Some(bg) = posts_tpl.template_data.get("card_background").and_then(|v| v.as_str()) {
                    vars.push(format!("--posts-card-bg: {}", bg));
                }
                if let Some(radius) = posts_tpl.template_data.get("card_radius").and_then(|v| v.as_str()) {
                    vars.push(format!("--posts-card-radius: {}", radius));
                }
                if let Some(shadow) = posts_tpl.template_data.get("card_shadow").and_then(|v| v.as_str()) {
                    vars.push(format!("--posts-card-shadow: {}", shadow));
                }
                if let Some(title_color) = posts_tpl.template_data.get("title_color").and_then(|v| v.as_str()) {
                    vars.push(format!("--posts-title-color: {}", title_color));
                }
                if let Some(meta_color) = posts_tpl.template_data.get("meta_color").and_then(|v| v.as_str()) {
                    vars.push(format!("--posts-meta-color: {}", meta_color));
                }
                if let Some(link_color) = posts_tpl.template_data.get("link_color").and_then(|v| v.as_str()) {
                    vars.push(format!("--posts-link-color: {}", link_color));
                }
                if let Some(grid_gap) = posts_tpl.template_data.get("grid_gap").and_then(|v| v.as_str()) {
                    vars.push(format!("--posts-grid-gap: {}", grid_gap));
                }
            }
            // Hero variables (background/text)
            if let Some(hero_tpl) = component_templates.iter().find(|t| t.component_type == "hero" && t.is_active) {
                if let Some(bg) = hero_tpl.template_data.get("background").and_then(|v| v.as_str()) {
                    vars.push(format!("--hero-bg: {}", bg));
                }
                if let Some(color) = hero_tpl.template_data.get("text_color").and_then(|v| v.as_str()) {
                    vars.push(format!("--hero-text: {}", color));
                }
            }
            // Buttons
            if let Some(btn_tpl) = component_templates.iter().find(|t| t.component_type == "header" && t.is_active) {
                if let Some(bg) = btn_tpl.template_data.get("button_primary_bg").and_then(|v| v.as_str()) {
                    vars.push(format!("--button-primary-bg: {}", bg));
                }
                if let Some(text) = btn_tpl.template_data.get("button_primary_text").and_then(|v| v.as_str()) {
                    vars.push(format!("--button-primary-text: {}", text));
                }
                if let Some(hover_bg) = btn_tpl.template_data.get("button_primary_hover_bg").and_then(|v| v.as_str()) {
                    vars.push(format!("--button-primary-hover-bg: {}", hover_bg));
                }
            }
            // Badges
            if let Some(badge_tpl) = component_templates.iter().find(|t| t.component_type == "header" && t.is_active) {
                if let Some(bg) = badge_tpl.template_data.get("badge_bg").and_then(|v| v.as_str()) {
                    vars.push(format!("--badge-bg: {}", bg));
                }
                if let Some(text) = badge_tpl.template_data.get("badge_text").and_then(|v| v.as_str()) {
                    vars.push(format!("--badge-text: {}", text));
                }
            }
            // Background animation
            if let Some(site_tpl) = component_templates.iter().find(|t| t.component_type == "main_container" && t.is_active) {
                if let Some(anim) = site_tpl.template_data.get("background_animation").and_then(|v| v.as_str()) {
                    vars.push(format!("--bg-animation: {}", anim));
                }
            }
            vars.join("; ")
        }
    };

    let on_nav_item_click = {
        let on_navigate = props.on_navigate.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(on_navigate) = &on_navigate {
                if let Some(target) = e.target_dyn_into::<web_sys::HtmlElement>() {
                    if let Some(url) = target.get_attribute("data-url") {
                        let page = match url.as_str() {
                            "/" => PublicPage::Home,
                            "/posts" => PublicPage::Posts,
                            url if url.starts_with("/post/") => {
                                if let Ok(id) = url.trim_start_matches("/post/").parse::<i32>() {
                                    PublicPage::Post(id)
                                } else {
                                    return;
                                }
                            }
                            url if url.starts_with("/page/") => {
                                let slug = url.trim_start_matches("/page/");
                                PublicPage::Page(slug.to_string())
                            }
                            _ => return,
                        };
                        on_navigate.emit(page);
                    }
                }
            }
        })
    };

    html! {
        <div class={if *acid_mode { "public-site acid-mode" } else { "public-site" }} style={format!("{}{}{}; position: relative; z-index: 1",
            global_style_vars(),
            if !(*site_style).is_empty() { "; " } else { "" },
            (*site_style).clone()
        )}>
            {if is_component_active("header") {
                html! {
                    <header id="site-header" class="site-header" style={get_component_style("header")}>
                        <div class="container">
                            {render_site_logo(&component_templates, &site_title)}
                            <nav class="site-nav">
                                if !*loading {
                                    {{
                                        let items: Vec<_> = header_navigation_items.iter().filter(|item| item.is_active).collect();
                                        web_sys::console::log_1(&format!("Filtered header navigation items: {:?}", items).into());
                                        web_sys::console::log_1(&format!("Current page: {}", props.current_page).into());
                                        items.into_iter().map(|item| {
                                            let is_active = props.current_page == item.url.trim_start_matches('/');
                                            html! {
                                                <a 
                                                    href="#" 
                                                    class={if is_active { "nav-link active" } else { "nav-link" }}
                                                    data-url={item.url.clone()}
                                                    onclick={on_nav_item_click.clone()}
                                                >
                                                    {&item.title}
                                                </a>
                                            }
                                        }).collect::<Html>()
                                    }}
                                }
                                
                                {if *admin_button_visible {
                                    html! {
                                        <button class="nav-button admin-button" onclick={on_admin_click}>
                                            {"Admin"}
                                        </button>
                                    }
                                } else {
                                    html! {}
                                }}
                            </nav>
                        </div>
                    </header>
                }
            } else {
                html! {}
            }}

            <main id="site-container" class="site-main">
                // Background video layer
                { {
                    let attrs = || {
                        if let Some(window) = web_sys::window() {
                            if let Some(document) = window.document() {
                                if let Some(body) = document.body() {
                                    let url = body.get_attribute("data-bg-video-url");
                                    if let Some(url) = url {
                                        let looping = body.get_attribute("data-bg-video-loop").unwrap_or_else(|| "true".to_string());
                                        let autoplay = body.get_attribute("data-bg-video-autoplay").unwrap_or_else(|| "true".to_string());
                                        let muted = body.get_attribute("data-bg-video-muted").unwrap_or_else(|| "true".to_string());
                                        return Some((url, looping, autoplay, muted));
                                    }
                                }
                            }
                        }
                        None
                    };
                    if let Some((url, looping, autoplay, muted)) = attrs() {
                        // Detect YouTube and render an iframe background if so; otherwise use <video>
                        let is_youtube = url.contains("youtube.com") || url.contains("youtu.be");
                        if is_youtube {
                            let video_id = (|| {
                                // youtu.be/<id>
                                if let Some(idx) = url.find("youtu.be/") { return url[idx+9..].split(['?', '&', '#']).next().map(|s| s.to_string()); }
                                // youtube.com/watch?v=<id>
                                if let Some(idx) = url.find("watch?v=") { return url[idx+8..].split(['&', '#']).next().map(|s| s.to_string()); }
                                // youtube.com/shorts/<id>
                                if let Some(idx) = url.find("/shorts/") { return url[idx+8..].split(['?', '&', '#']).next().map(|s| s.to_string()); }
                                // youtube.com/embed/<id>
                                if let Some(idx) = url.find("/embed/") { return url[idx+7..].split(['?', '&', '#']).next().map(|s| s.to_string()); }
                                None
                            })();
                            if let Some(id) = video_id {
                                let embed_src = format!(
                                    "https://www.youtube.com/embed/{}?autoplay={}&mute={}&loop={}&playlist={}&controls=0&showinfo=0&modestbranding=1&iv_load_policy=3&rel=0&playsinline=1",
                                    id,
                                    if autoplay == "true" { 1 } else { 0 },
                                    if muted == "true" { 1 } else { 0 },
                                    if looping == "true" { 1 } else { 0 },
                                    id
                                );
                                return html! {
                                    <iframe
                                        class="bg-video-layer"
                                        src={embed_src}
                                        style="position: fixed; inset: 0; width: 100%; height: 100%; object-fit: cover; z-index: -2; pointer-events: none; border: 0;"
                                        allow="autoplay; encrypted-media; picture-in-picture"
                                        loading="eager"
                                    />
                                };
                            }
                        }
                        html!{
                            <video
                                class="bg-video-layer"
                                src={url}
                                autoplay={autoplay == "true"}
                                loop={looping == "true"}
                                muted={muted == "true"}
                                playsinline=true
                                style="position: fixed; inset: 0; width: 100%; height: 100%; object-fit: cover; z-index: -2; pointer-events: none;"
                            />
                        }
                    } else { html!{} }
                } }
                <div class="site-content" style="position: relative; z-index: 1;">
                    <div class="container" style={(*inner_container_style).clone()}>
                        {props.children.clone()}
                    </div>
                </div>
            </main>

            {if is_component_active("footer") {
                html! {
                    <footer id="site-footer" class="site-footer" style={get_component_style("footer")}>
                        <div class="container">
                            {if !footer_navigation_items.is_empty() {
                                html! {
                                    <nav class="footer-nav">
                                        {footer_navigation_items.iter().filter(|item| item.is_active).map(|item| {
                                            html! {
                                                <a 
                                                    href="#" 
                                                    class="footer-nav-link"
                                                    data-url={item.url.clone()}
                                                    onclick={on_nav_item_click.clone()}
                                                >
                                                    {&item.title}
                                                </a>
                                            }
                                        }).collect::<Html>()}
                                    </nav>
                                }
                            } else {
                                html! {}
                            }}
                            <p class="footer-copyright">{"© 2024 My Rust CMS. Built with Rust and Yew."}</p>
                        </div>
                    </footer>
                }
            } else {
                html! {}
            }}
            { if auth.is_authenticated && auth.user.as_ref().map(|u| u.role.as_str() == "admin").unwrap_or(false) {
                let on_toggle = {
                    let live_edit_enabled = live_edit_enabled.clone();
                    Callback::from(move |_| live_edit_enabled.set(!*live_edit_enabled))
                };
                html!{
                    <>
                        <button onclick={on_toggle} style="position: fixed; bottom: 16px; right: 16px; z-index: 9999; padding: 10px 14px; border-radius: 8px; border: 1px solid rgba(0,0,0,0.1); background: #111; color: #fff; opacity: 0.9; pointer-events: auto;">{
                            if *live_edit_enabled { "Disable Live Edit" } else { "Enable Live Edit" }
                        }</button>
                        <EnhancedLiveEditSystem
                            enabled={*live_edit_enabled}
                            component_templates={(*component_templates).clone()}
                            on_templates_updated={
                                let component_templates = component_templates.clone();
                                Callback::from(move |updated: Vec<ComponentTemplate>| {
                                    if updated.is_empty() { return; }
                                    
                                    // Update local state first
                                    let mut map: std::collections::HashMap<i32, ComponentTemplate> = component_templates.iter().map(|t| (t.id, t.clone())).collect();
                                    for template in &updated { 
                                        map.insert(template.id, template.clone()); 
                                    }
                                    component_templates.set(map.into_values().collect());
                                    
                                    // Save each updated template to backend
                                    for template in updated {
                                        let template_clone = template.clone();
                                        wasm_bindgen_futures::spawn_local(async move {
                                            web_sys::console::log_1(&format!("Saving template {} to backend...", template_clone.id).into());
                                            match crate::services::navigation_service::update_component_template(template_clone.id, &template_clone).await {
                                                Ok(_) => {
                                                    web_sys::console::log_1(&format!("✅ Successfully saved template {} to backend", template_clone.id).into());
                                                }
                                                Err(e) => {
                                                    web_sys::console::log_1(&format!("❌ Failed to save template {} to backend: {:?}", template_clone.id, e).into());
                                                }
                                            }
                                        });
                                    }
                                })
                            }
                            page_components={vec![]} // TODO: Get actual page components
                            on_page_components_updated={Callback::from(|_| {})} // TODO: Handle page component updates
                            current_page={None} // TODO: Get current page
                        />
                    </>
                }
            } else { html!{} }}
        </div>
    }
}

// Helper function to generate SVG data URLs for shape masks
fn generate_shape_svg_data_url(shape_type: &str, scale: f32, is_top: bool) -> String {
    let width = 1440;
    let height = (40.0 * scale) as i32;
    
    let path = match shape_type {
        "wave" => {
            if is_top {
                format!("M0,{} Q360,{} 720,{} T1440,{} L1440,0 L0,0 Z", height, height - 10, height, height)
            } else {
                format!("M0,0 Q360,10 720,0 T1440,0 L1440,{} L0,{} Z", height, height)
            }
        },
        "curve" => {
            if is_top {
                format!("M0,{} Q720,{} 1440,{} L1440,0 L0,0 Z", height, height - 20, height)
            } else {
                format!("M0,0 Q720,20 1440,0 L1440,{} L0,{} Z", height, height)
            }
        },
        "triangle" => {
            if is_top {
                format!("M0,{} L720,{} L1440,{} L1440,0 L0,0 Z", height, height - 15, height)
            } else {
                format!("M0,0 L720,15 L1440,0 L1440,{} L0,{} Z", height, height)
            }
        },
        "zigzag" => {
            if is_top {
                format!("M0,{} L240,{} L480,{} L720,{} L960,{} L1200,{} L1440,{} L1440,0 L0,0 Z", 
                    height, height - 8, height, height - 8, height, height - 8, height)
            } else {
                format!("M0,0 L240,8 L480,0 L720,8 L960,0 L1200,8 L1440,0 L1440,{} L0,{} Z", height, height)
            }
        },
        "arrow" => {
            if is_top {
                format!("M0,{} L720,{} L1440,{} L1080,{} L720,{} L360,{} Z", 
                    height, height - 15, height, height - 5, height - 10, height - 5)
            } else {
                format!("M0,0 L360,5 L720,10 L1080,5 L1440,0 L720,15 Z")
            }
        },
        "tilt" => {
            if is_top {
                format!("M0,{} L1440,{} L1440,0 L0,0 Z", height, height - 10)
            } else {
                format!("M0,0 L1440,10 L1440,{} L0,{} Z", height, height)
            }
        },
        _ => {
            if is_top {
                format!("M0,{} L1440,{} L1440,0 L0,0 Z", height, height)
            } else {
                format!("M0,0 L1440,0 L1440,{} L0,{} Z", height, height)
            }
        }
    };
    
    format!(
        "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 {} {}' fill='white'%3E%3Cpath d='{}'/%3E%3C/svg%3E",
        width, height, path
    )
}

// Generate clip-path for template rendering (mirrors the logic from enhanced_live_edit_system.rs)
fn generate_clip_path_for_template(
    shape_upper: &str, scale_upper: &str, frequency_upper: &str, direction_upper: &str, amplitude_upper: &str, degrees_upper: &str,
    shape_lower: &str, scale_lower: &str, frequency_lower: &str, direction_lower: &str, amplitude_lower: &str, degrees_lower: &str
) -> String {
    // Use the SAME isolated tilt logic as the live edit system
    if shape_upper == "tilt" || shape_lower == "tilt" {
        return generate_tilt_only_clip_path_for_template(
            shape_upper, degrees_upper, direction_upper,
            shape_lower, degrees_lower, direction_lower
        );
    }
    
    // Create proper polygon without duplicate points for non-tilt shapes
    let mut polygon_points = vec!["0% 0%".to_string()]; // Start at top-left
    
    // Add upper shape points (left to right along top edge) - but skip tilt shapes
    if shape_upper != "none" && !shape_upper.is_empty() && shape_upper != "tilt" {
        let scale_factor = scale_upper.parse::<f32>().unwrap_or(100.0) / 100.0;
        let frequency = frequency_upper.parse::<f32>().unwrap_or(2.0);
        let amplitude = amplitude_upper.parse::<f32>().ok();
        let degrees = degrees_upper.parse::<f32>().unwrap_or(15.0);
        let upper_points = generate_shape_points_for_template_new(shape_upper, scale_factor, frequency, true, Some(direction_upper), amplitude, Some(degrees));
        polygon_points.extend(upper_points);
    }
    
    // Add top-right corner
    polygon_points.push("100% 0%".to_string());
    
    // Add right edge to bottom-right
    polygon_points.push("100% 100%".to_string());
    
    // Add lower shape points (right to left along bottom edge) - but skip tilt shapes
    if shape_lower != "none" && !shape_lower.is_empty() && shape_lower != "tilt" {
        let scale_factor = scale_lower.parse::<f32>().unwrap_or(100.0) / 100.0;
        let frequency = frequency_lower.parse::<f32>().unwrap_or(2.0);
        let amplitude = amplitude_lower.parse::<f32>().ok();
        let degrees = degrees_lower.parse::<f32>().unwrap_or(15.0);
        let mut lower_points = generate_shape_points_for_template_new(shape_lower, scale_factor, frequency, false, Some(direction_lower), amplitude, Some(degrees));
        lower_points.reverse(); // Reverse for right-to-left traversal
        polygon_points.extend(lower_points);
    }
    
    // Add bottom-left corner to close the polygon
    polygon_points.push("0% 100%".to_string());
    
    if shape_upper != "none" || shape_lower != "none" {
        format!("polygon({})", polygon_points.join(", "))
    } else {
        String::new()
    }
}

// Generate tilt-only clip-path for template rendering (mirrors the logic from enhanced_live_edit_system.rs)
fn generate_tilt_only_clip_path_for_template(
    shape_upper: &str, degrees_upper: &str, direction_upper: &str,
    shape_lower: &str, degrees_lower: &str, direction_lower: &str
) -> String {
    let mut polygon_points = Vec::new();
    
    // Handle upper edge
    if shape_upper == "tilt" {
        let degrees = degrees_upper.parse::<f32>().unwrap_or(15.0);
        let is_left = direction_upper == "left";
        
        let tilt_amount = if degrees <= 0.5 {
            0.0 // Flat line for very small angles
        } else {
            degrees.min(45.0) * 0.8
        };
        
        let (left_y, right_y) = if is_left {
            (tilt_amount, 0.0) // Left corner goes down, right stays at 0
        } else {
            (0.0, tilt_amount) // Left stays at 0, right corner goes down
        };
        
        polygon_points.push(format!("0% {}%", left_y)); // Left corner tilted
        polygon_points.push(format!("100% {}%", right_y)); // Right corner tilted
    } else {
        polygon_points.push("0% 0%".to_string()); // Default top-left
        polygon_points.push("100% 0%".to_string()); // Default top-right
    }
    
    // Handle lower edge
    if shape_lower == "tilt" {
        let degrees = degrees_lower.parse::<f32>().unwrap_or(15.0);
        let is_left = direction_lower == "left";
        
        let tilt_amount = if degrees <= 0.5 {
            0.0 // Flat line for very small angles
        } else {
            degrees.min(45.0) * 0.8
        };
        
        let (right_y, left_y) = if is_left {
            (100.0, 100.0 - tilt_amount) // Right stays at 100%, left goes up
        } else {
            (100.0 - tilt_amount, 100.0) // Right goes up, left stays at 100%
        };
        
        // Add right edge to bottom-right corner
        polygon_points.push(format!("100% {}%", right_y));
        
        // Add tilted bottom-left corner
        polygon_points.push(format!("0% {}%", left_y));
    } else {
        // Default bottom-right and bottom-left
        polygon_points.push("100% 100%".to_string());
        polygon_points.push("0% 100%".to_string());
    }
    
    format!("polygon({})", polygon_points.join(", "))
}

// Generate shape points for template rendering (mirrors the logic from enhanced_live_edit_system.rs)
fn generate_shape_points_for_template(shape_type: &str, scale: f32, frequency: f32, is_top: bool) -> Vec<String> {
    let depth = (scale * 20.0).min(50.0); // Use scale directly for depth (max 50%)
    
    match shape_type {
        "wave" => {
            let mut points = Vec::new();
            let num_points = (frequency * 15.0) as i32 + 30; // More points for smoother waves
            for i in 0..=num_points {
                let x = (i as f32 / num_points as f32) * 100.0;
                // Create proper sine wave with correct phase
                let wave_input = (x / 100.0) * frequency * 2.0 * std::f32::consts::PI;
                let wave_value = wave_input.sin();
                
                let y = if is_top {
                    // For top wave: use scale for wave depth
                    let wave_offset = wave_value * depth * 0.5; // Half depth for wave variation
                    (depth + wave_offset).max(0.0).min(50.0)
                } else {
                    // For bottom wave: use scale for wave depth
                    let wave_offset = wave_value * depth * 0.5; // Half depth for wave variation
                    (100.0 - depth - wave_offset).max(50.0).min(100.0)
                };
                
                points.push(format!("{}% {}%", x, y));
            }
            points
        },
        "curve" => {
            let mut points = Vec::new();
            let num_points = (frequency * 5.0) as i32 + 10; // More curves for higher frequency
            
            // Generate smooth bezier-like curve using multiple points
            for i in 0..=num_points {
                let t = i as f32 / num_points as f32;
                let x = t * 100.0;
                
                // Quadratic curve formula: (1-t)²*P0 + 2(1-t)t*P1 + t²*P2
                let p0 = if is_top { 0.0 } else { 100.0 };
                let p1 = if is_top { depth } else { 100.0 - depth };
                let p2 = if is_top { 0.0 } else { 100.0 };
                
                let y = (1.0 - t).powi(2) * p0 + 2.0 * (1.0 - t) * t * p1 + t.powi(2) * p2;
                points.push(format!("{}% {}%", x, y.max(0.0).min(100.0)));
            }
            points
        },
        "zigzag" => {
            let mut points = Vec::new();
            let segments = (frequency * 2.0) as i32 + 2; // More segments for higher frequency
            for i in 0..=segments {
                let x = (i as f32 / segments as f32) * 100.0;
                let y = if is_top {
                    if i % 2 == 0 { 0.0 } else { depth }
                } else {
                    if i % 2 == 0 { 100.0 } else { 100.0 - depth }
                };
                points.push(format!("{}% {}%", x, y.max(0.0).min(100.0)));
            }
            points
        },
        "arrow" => {
            let mut points = Vec::new();
            let segments = (frequency * 2.0) as i32 + 2; // Arrow segments based on frequency
            for i in 0..=segments {
                let x = (i as f32 / segments as f32) * 100.0;
                let y = if is_top {
                    if i % 2 == 0 { 0.0 } else { depth }
                } else {
                    if i % 2 == 0 { 100.0 } else { 100.0 - depth }
                };
                points.push(format!("{}% {}%", x, y.max(0.0).min(100.0)));
            }
            points
        },
        "tilt" => {
            vec![
                if is_top {
                    format!("100% {}%", depth)
                } else {
                    format!("100% {}%", 100.0 - depth)
                }
            ]
        },
        _ => Vec::new(),
    }
}

// New shape generation function with enhanced parameters for template rendering
fn generate_shape_points_for_template_new(
    shape_type: &str, 
    scale: f32, 
    frequency: f32, 
    is_top: bool, 
    direction: Option<&str>, 
    amplitude: Option<f32>, 
    degrees: Option<f32>
) -> Vec<String> {
    let depth = (scale * 20.0).min(50.0); // Use scale directly for depth (max 50%)
    
    match shape_type {
        "wave" => {
            let mut points = Vec::new();
            let num_points = (frequency * 15.0) as i32 + 30; // More points for smoother waves
            
            for i in 0..=num_points {
                let x = (i as f32 / num_points as f32) * 100.0;
                // Create proper sine wave with correct phase
                let wave_input = (x / 100.0) * frequency * 2.0 * std::f32::consts::PI;
                let wave_value = wave_input.sin();
                
                let y = if is_top {
                    // For top wave: use scale for wave depth
                    let wave_offset = wave_value * depth * 0.5; // Half depth for wave variation
                    (depth + wave_offset).max(0.0).min(50.0)
                } else {
                    // For bottom wave: use scale for wave depth
                    let wave_offset = wave_value * depth * 0.5; // Half depth for wave variation
                    (100.0 - depth - wave_offset).max(50.0).min(100.0)
                };
                
                points.push(format!("{}% {}%", x, y));
            }
            points
        },
        "curve" => {
            let mut points = Vec::new();
            let curve_amplitude = amplitude.unwrap_or(50.0);
            let is_positive = direction.unwrap_or("positive") == "positive";
            let num_points = 30; // Fixed number of points for smooth curve
            
            // Generate smooth bezier-like curve using multiple points
            for i in 0..=num_points {
                let t = i as f32 / num_points as f32;
                let x = t * 100.0;
                
                // Quadratic curve formula with amplitude control
                let base_y = if is_top { 0.0 } else { 100.0 };
                let curve_height = (4.0 * t * (1.0 - t)) * (depth * curve_amplitude / 100.0);
                let curve_offset = if is_positive { curve_height } else { -curve_height };
                
                let y = if is_top { base_y + curve_offset } else { base_y - curve_offset };
                points.push(format!("{}% {}%", x, y.max(0.0).min(100.0)));
            }
            points
        },
        "triangle" => {
            let mut points = Vec::new();
            let peak_count = frequency.max(1.0) as usize;
            
            for peak in 0..peak_count {
                let start_x = (peak as f32) / (peak_count as f32) * 100.0;
                let end_x = ((peak + 1) as f32) / (peak_count as f32) * 100.0;
                let mid_x = (start_x + end_x) / 2.0;
                
                let base_y = if is_top { 0.0 } else { 100.0 };
                let peak_y = if is_top { depth } else { 100.0 - depth };
                
                // Add points for this triangle
                if peak == 0 {
                    points.push(format!("{}% {}%", start_x, base_y));
                }
                points.push(format!("{}% {}%", mid_x, peak_y));
                if peak == peak_count - 1 {
                    points.push(format!("{}% {}%", end_x, base_y));
                }
            }
            points
        },
        "zigzag" => {
            let mut points = Vec::new();
            let segments = (frequency * 2.0) as i32 + 2; // More segments for higher frequency
            for i in 0..=segments {
                let x = (i as f32 / segments as f32) * 100.0;
                let y = if is_top {
                    if i % 2 == 0 { 0.0 } else { depth }
                } else {
                    if i % 2 == 0 { 100.0 } else { 100.0 - depth }
                };
                points.push(format!("{}% {}%", x, y.max(0.0).min(100.0)));
            }
            points
        },
        "tilt" => {
            // Use the SAME tilt logic as the live edit system for consistency
            let tilt_degrees = degrees.unwrap_or(15.0);
            let is_left = direction.unwrap_or("right") == "left";
            
            // Use simple percentage mapping (same as live edit system)
            let tilt_amount = if tilt_degrees <= 0.5 {
                0.0 // Flat line for very small angles
            } else {
                tilt_degrees.min(45.0) * 0.8 // Same formula as live edit
            };
            
            if is_top {
                // For upper tilt
                let (left_y, right_y) = if is_left {
                    (tilt_amount, 0.0) // Left corner goes down, right stays at 0
                } else {
                    (0.0, tilt_amount) // Left stays at 0, right corner goes down
                };
                vec![format!("100% {}%", right_y)] // Only return the right corner point
            } else {
                // For lower tilt
                let (right_y, left_y) = if is_left {
                    (100.0, 100.0 - tilt_amount) // Right stays at 100%, left goes up
                } else {
                    (100.0 - tilt_amount, 100.0) // Right goes up, left stays at 100%
                };
                vec![format!("0% {}%", left_y)] // Only return the left corner point
            }
        },
        _ => Vec::new(),
    }
} 