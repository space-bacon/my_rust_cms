use yew::prelude::*;
use wasm_bindgen::JsCast;
use crate::services::navigation_service::{MenuArea, ComponentTemplate, NavigationItem, get_menu_areas, get_component_templates, get_all_component_templates_admin, update_menu_area, update_component_template, get_navigation_by_area, toggle_component_template};
use crate::services::api_service::{SettingData, get_settings, update_settings, get_templates, Template, MediaItem};
use crate::components::{ModernMenuDesigner, MenuStyle};
use crate::services::modern_menu_service::{save_menu_style, load_menu_style, force_refresh_menu_styles};
use serde_json::Value as JsonValue;
use serde_json::json;
use wasm_bindgen::JsValue;
use crate::components::simple_notification::SimpleNotification;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub enum TemplateView {
    MenuAreas,
    ComponentTemplates,
    ContainerSettings,
}



#[derive(Clone, PartialEq)]
pub struct ContainerSettings {
    // Breakpoints
    pub mobile_breakpoint: String,
    pub tablet_breakpoint: String,
    pub desktop_breakpoint: String,
    pub wide_breakpoint: String,
    
    // Typography
    pub base_font_size: String,
    pub scale_ratio: String,
    pub line_height: String,
    
    // Container
    pub width_type: String,
    pub max_width: String,
    pub horizontal_padding: String,

    // Background & Media
    pub background_type: String,        // none | color | gradient | image | video
    pub background_color: String,
    pub gradient_from: String,
    pub gradient_to: String,
    pub gradient_angle: String,
    pub background_image_url: String,
    pub background_image_size: String,
    pub background_image_position: String,
    pub background_video_url: String,
    pub background_video_loop: bool,
    pub background_video_autoplay: bool,
    pub background_video_muted: bool,
    pub overlay_color: String,
    pub overlay_opacity: String,

    // Borders & Effects
    pub border_radius: String,
    pub border_width: String,
    pub border_color: String,
    pub box_shadow: String,

    // Animation
    pub animation: String,

    // Acid Mode - animated gradient borders for all components
    pub acid_mode: bool,
}

impl Default for ContainerSettings {
    fn default() -> Self {
        Self {
            mobile_breakpoint: "768px".to_string(),
            tablet_breakpoint: "1024px".to_string(),
            desktop_breakpoint: "1200px".to_string(),
            wide_breakpoint: "1440px".to_string(),
            base_font_size: "16px".to_string(),
            scale_ratio: "1.25".to_string(),
            line_height: "1.5".to_string(),
            width_type: "fixed".to_string(),
            max_width: "1200px".to_string(),
            horizontal_padding: "1rem".to_string(),

            background_type: "none".to_string(),
            background_color: "#ffffff".to_string(),
            gradient_from: "#ffffff".to_string(),
            gradient_to: "#ffffff".to_string(),
            gradient_angle: "180deg".to_string(),
            background_image_url: "".to_string(),
            background_image_size: "cover".to_string(),
            background_image_position: "center".to_string(),
            background_video_url: "".to_string(),
            background_video_loop: true,
            background_video_autoplay: true,
            background_video_muted: true,
            overlay_color: "#000000".to_string(),
            overlay_opacity: "0.3".to_string(),

            border_radius: "0px".to_string(),
            border_width: "0px".to_string(),
            border_color: "#000000".to_string(),
            box_shadow: "none".to_string(),

            animation: "none".to_string(),

            acid_mode: false,
        }
    }
}



#[function_component(TemplateManager)]
pub fn template_manager() -> Html {
    let current_view = use_state(|| TemplateView::MenuAreas);
    let menu_areas = use_state(Vec::<MenuArea>::new);

    let component_templates = use_state(Vec::<ComponentTemplate>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let templates = use_state(Vec::<Template>::new);
    let selected_template_id = use_state(|| None::<i32>);
    let show_import_modal = use_state(|| false);
    let import_json_text = use_state(String::new);
    let show_export_modal = use_state(|| false);
    let export_json_text = use_state(String::new);
    let applied_default_once = use_state(|| false);
    let notify_message = use_state(|| None::<(String, String)>);
    
    // Modern Menu Designer State
    let show_modern_designer = use_state(|| false);
    let current_menu_area = use_state(|| "header".to_string());
    let current_menu_style = use_state(|| None::<MenuStyle>);

    // Load initial data
    {
        let menu_areas = menu_areas.clone();
        let component_templates = component_templates.clone();
        let loading = loading.clone();
        let error = error.clone();
        let templates_for_effect = templates.clone();

        let selected_for_effect = selected_template_id.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                
                // Check authentication first
                if let Err(_) = crate::services::auth_service::get_auth_token() {
                    error.set(Some("Please log in to access the admin panel".to_string()));
                    loading.set(false);
                    return;
                }
                
                // Load template data in parallel
                log::info!("🔐 Authentication check passed, loading template data...");
                let areas_result = get_menu_areas().await;
                let components_result = get_all_component_templates_admin().await;
                let templates_result = get_templates().await;
                // Also fetch existing container settings to decide whether to auto-apply defaults
                let existing_container_settings = get_settings(Some("container")).await;
                
                match (&areas_result, &components_result, &templates_result) {
                    (Ok(areas), Ok(components), Ok(tpls)) => {
                        log::info!("✅ Successfully loaded {} menu areas and {} component templates", areas.len(), components.len());
                        menu_areas.set(areas.clone());
                        component_templates.set(components.clone());
                        templates_for_effect.set(tpls.clone());
                        // Auto-select and apply default template once, but only when there are no existing container settings
                        if !*applied_default_once {
                            let should_apply_defaults = match &existing_container_settings {
                                Ok(settings_vec) => {
                                    // If there are zero container settings, we consider this a first-time init
                                    settings_vec.is_empty()
                                }
                                Err(_) => {
                                    // If we failed to fetch, be conservative and do NOT override user settings
                                    false
                                }
                            };

                            if should_apply_defaults {
                                if let Some(default_tpl) = tpls.iter().find(|t| t.name.to_lowercase() == "default")
                                    .or_else(|| tpls.first()) {
                                    selected_for_effect.set(Some(default_tpl.id));
                                    // Apply the template layout
                                    if let Ok(layout_json) = serde_json::from_str::<serde_json::Value>(&default_tpl.layout) {
                                        // Apply menu areas
                                        if let Some(areas_arr) = layout_json.get("menu_areas").and_then(|v| v.as_array()) {
                                            for area in areas_arr {
                                                if let (Some(area_name), Some(is_active)) = (area.get("area_name").and_then(|v| v.as_str()), area.get("is_active").and_then(|v| v.as_bool())) {
                                                    if let Some(existing) = areas.iter().find(|a| a.area_name == area_name) {
                                                        let mut updated = existing.clone();
                                                        updated.is_active = is_active;
                                                        if let Some(settings) = area.get("settings").cloned() { updated.settings = settings; }
                                                        if let Some(mobile) = area.get("mobile_behavior").and_then(|v| v.as_str()) { updated.mobile_behavior = Some(mobile.to_string()); }
                                                        if let Some(icon) = area.get("hamburger_icon").and_then(|v| v.as_str()) { updated.hamburger_icon = Some(icon.to_string()); }
                                                        let name = area_name.to_string();
                                                        wasm_bindgen_futures::spawn_local(async move {
                                                            let _ = update_menu_area(&name, &updated).await;
                                                        });
                                                    }
                                                }
                                            }
                                        }
                                        // Apply component templates
                                        if let Some(components_arr) = layout_json.get("component_templates").and_then(|v| v.as_array()) {
                                            for comp in components_arr {
                                                if let Some(component_type) = comp.get("component_type").and_then(|v| v.as_str()) {
                                                    if let Some(existing) = components.iter().find(|t| t.component_type == component_type) {
                                                        let mut updated = existing.clone();
                                                        if let Some(data) = comp.get("template_data").cloned() { updated.template_data = data; }
                                                        if let Some(bp) = comp.get("breakpoints").cloned() { updated.breakpoints = bp; }
                                                        if let Some(width) = comp.get("width_setting").and_then(|v| v.as_str()) { updated.width_setting = Some(width.to_string()); }
                                                        if let Some(max_w) = comp.get("max_width").and_then(|v| v.as_str()) { updated.max_width = Some(max_w.to_string()); }
                                                        if let Some(is_active) = comp.get("is_active").and_then(|v| v.as_bool()) { updated.is_active = is_active; }
                                                        let id = updated.id;
                                                        wasm_bindgen_futures::spawn_local(async move {
                                                            let _ = update_component_template(id, &updated).await;
                                                        });
                                                    }
                                                }
                                            }
                                        }
                                        // Apply container settings
                                        if let Some(container) = layout_json.get("container_settings").and_then(|v| v.as_object()) {
                                            let mut settings_data: Vec<SettingData> = Vec::new();
                                            for (key, value) in container.iter() {
                                                let setting_key = format!("container_{}", key);
                                                let setting_value = if value.is_string() { value.as_str().unwrap_or("").to_string() } else { value.to_string() };
                                                settings_data.push(SettingData { key: setting_key, value: setting_value, setting_type: "container".to_string(), description: None });
                                            }
                                            wasm_bindgen_futures::spawn_local(async move {
                                                let _ = update_settings(settings_data).await;
                                            });
                                        }
                                    }
                                }
                            }
                            applied_default_once.set(true);
                        }
                        loading.set(false);
                    }
                    (Err(areas_err), Ok(_), Ok(tpls)) => {
                        log::error!("❌ Failed to load menu areas: {:?}", areas_err);
                        error.set(Some("Failed to load menu areas".to_string()));
                        templates_for_effect.set(tpls.clone());
                        loading.set(false);
                    }
                    (Ok(_), Err(components_err), Ok(tpls)) => {
                        log::error!("❌ Failed to load component templates: {:?}", components_err);
                        
                        // Try fallback to public endpoint if auth failed
                        if components_err.to_string().contains("Not authenticated") {
                            log::info!("🔄 Trying fallback to public component templates...");
                            match get_component_templates().await {
                                Ok(fallback_components) => {
                                    log::info!("✅ Loaded {} public component templates", fallback_components.len());
                                    component_templates.set(fallback_components);
                                    error.set(Some("Loaded in read-only mode. Please log in for full access.".to_string()));
                                    templates_for_effect.set(tpls.clone());
                                    loading.set(false);
                                    return;
                                }
                                Err(fallback_err) => {
                                    log::error!("❌ Fallback also failed: {:?}", fallback_err);
                                }
                            }
                        }
                        
                        error.set(Some("Failed to load component templates".to_string()));
                        loading.set(false);
                    }
                    (Err(areas_err), Err(components_err), Ok(tpls)) => {
                        log::error!("❌ Failed to load both menu areas and component templates. Areas: {:?}, Components: {:?}", areas_err, components_err);
                        
                        // Check if it's an auth issue and try fallback for components
                        if components_err.to_string().contains("Not authenticated") {
                            log::info!("🔄 Trying fallback to public component templates...");
                            match get_component_templates().await {
                                Ok(fallback_components) => {
                                    log::info!("✅ Loaded {} public component templates", fallback_components.len());
                                    component_templates.set(fallback_components);
                                    menu_areas.set(Vec::new()); // Set empty menu areas
                                    error.set(Some("Authentication required. Limited functionality available.".to_string()));
                                    templates_for_effect.set(tpls.clone());
                                    loading.set(false);
                                    return;
                                }
                                Err(fallback_err) => {
                                    log::error!("❌ Fallback also failed: {:?}", fallback_err);
                                }
                            }
                        }
                        
                        error.set(Some("Failed to load template data".to_string()));
                        loading.set(false);
                    }
                    (_, _, Err(t_err)) => {
                        log::error!("❌ Failed to load master templates: {:?}", t_err);
                        // proceed without templates dropdown
                        if let (Ok(areas), Ok(components)) = (&areas_result, &components_result) {
                            menu_areas.set(areas.clone());
                            component_templates.set(components.clone());
                        }
                        loading.set(false);
                    }
                }
            });
            || ()
        }, ());
    }

    // Apply selected master template
    let apply_selected_template = {
        let selected_template_id_state = selected_template_id.clone();
        let templates_state = templates.clone();
        let selected_template_id_setter = selected_template_id.clone();
        let menu_areas = menu_areas.clone();
        let component_templates = component_templates.clone();
        let error = error.clone();
        let notify_apply = notify_message.clone();
        Callback::from(move |_| {
            if let Some(template_id) = *selected_template_id_state {
                if let Some(selected) = (*templates_state).iter().find(|t| t.id == template_id) {
                    // Parse layout JSON: expected keys: menu_areas: [], component_templates: [], container_settings: {}
                    if let Ok(layout_json) = serde_json::from_str::<JsonValue>(&selected.layout) {
                        // Apply menu areas
                        if let Some(areas) = layout_json.get("menu_areas").and_then(|v| v.as_array()) {
                            for area in areas {
                                if let (Some(area_name), Some(is_active)) = (area.get("area_name").and_then(|v| v.as_str()), area.get("is_active").and_then(|v| v.as_bool())) {
                                    if let Some(existing) = (*menu_areas).iter().find(|a| a.area_name == area_name) {
                                        let mut updated = existing.clone();
                                        updated.is_active = is_active;
                                        if let Some(settings) = area.get("settings").cloned() { updated.settings = settings; }
                                        if let Some(mobile) = area.get("mobile_behavior").and_then(|v| v.as_str()) { updated.mobile_behavior = Some(mobile.to_string()); }
                                        if let Some(icon) = area.get("hamburger_icon").and_then(|v| v.as_str()) { updated.hamburger_icon = Some(icon.to_string()); }
                                        let name = area_name.to_string();
                                        wasm_bindgen_futures::spawn_local(async move {
                                            let _ = update_menu_area(&name, &updated).await;
                                        });
                                    }
                                }
                            }
                        }

                        // Apply component templates by component_type
                        if let Some(components) = layout_json.get("component_templates").and_then(|v| v.as_array()) {
                            for comp in components {
                                if let Some(component_type) = comp.get("component_type").and_then(|v| v.as_str()) {
                                    if let Some(existing) = (*component_templates).iter().find(|t| t.component_type == component_type) {
                                        let mut updated = existing.clone();
                                        if let Some(data) = comp.get("template_data").cloned() { updated.template_data = data; }
                                        if let Some(bp) = comp.get("breakpoints").cloned() { updated.breakpoints = bp; }
                                        if let Some(width) = comp.get("width_setting").and_then(|v| v.as_str()) { updated.width_setting = Some(width.to_string()); }
                                        if let Some(max_w) = comp.get("max_width").and_then(|v| v.as_str()) { updated.max_width = Some(max_w.to_string()); }
                                        if let Some(is_active) = comp.get("is_active").and_then(|v| v.as_bool()) { updated.is_active = is_active; }
                                        let id = updated.id;
                                        wasm_bindgen_futures::spawn_local(async move {
                                            let _ = update_component_template(id, &updated).await;
                                        });
                                    }
                                }
                            }
                        }

                        // Apply container settings via system settings API
                        if let Some(container) = layout_json.get("container_settings").and_then(|v| v.as_object()) {
                            let mut settings_data: Vec<SettingData> = Vec::new();
                            for (key, value) in container.iter() {
                                // Store all container settings under container_ prefix
                                let setting_key = format!("container_{}", key);
                                let setting_value = if value.is_string() { value.as_str().unwrap_or("").to_string() } else { value.to_string() };
                                settings_data.push(SettingData { key: setting_key, value: setting_value, setting_type: "container".to_string(), description: None });
                            }
                            wasm_bindgen_futures::spawn_local(async move {
                                let _ = update_settings(settings_data).await;
                            });
                        }
                        // Update selected dropdown to reflect applied template
                        selected_template_id_setter.set(Some(selected.id));
                        // Show success notification
                        notify_apply.set(Some((format!("Applied template: {}", selected.name), "success".to_string())));
                    } else {
                        error.set(Some("Invalid template layout JSON".to_string()))
                    }
                }
            }
        })
    };

    // Import template modal handlers
    let open_import_modal = {
        let show_import_modal = show_import_modal.clone();
        Callback::from(move |_| show_import_modal.set(true))
    };

    // Export current configuration
    let export_current_configuration = {
        let menu_areas = menu_areas.clone();
        let component_templates = component_templates.clone();
        let show_export_modal = show_export_modal.clone();
        let export_json_text = export_json_text.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let menu_areas_data = (*menu_areas).clone();
            let component_templates_data = (*component_templates).clone();
            let show_export_modal = show_export_modal.clone();
            let export_json_text = export_json_text.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // pull container settings from backend to include latest values
                match get_settings(Some("container")).await {
                    Ok(settings_vec) => {
                        let mut container_map = serde_json::Map::new();
                        for s in settings_vec {
                            if let Some(val) = s.setting_value {
                                if let Some(stripped) = s.setting_key.strip_prefix("container_") {
                                    container_map.insert(stripped.to_string(), JsonValue::String(val));
                                }
                            }
                        }

                        let export_obj = json!({
                            "menu_areas": menu_areas_data.iter().map(|a| json!({
                                "area_name": a.area_name,
                                "display_name": a.display_name,
                                "is_active": a.is_active,
                                "settings": a.settings,
                                "mobile_behavior": a.mobile_behavior,
                                "hamburger_icon": a.hamburger_icon,
                            })).collect::<Vec<_>>(),
                            "component_templates": component_templates_data.iter().map(|t| json!({
                                "name": t.name,
                                "component_type": t.component_type,
                                "template_data": t.template_data,
                                "breakpoints": t.breakpoints,
                                "width_setting": t.width_setting,
                                "max_width": t.max_width,
                                "is_default": t.is_default,
                                "is_active": t.is_active,
                            })).collect::<Vec<_>>(),
                            "container_settings": JsonValue::Object(container_map),
                        });

                        let pretty = serde_json::to_string_pretty(&export_obj).unwrap_or_else(|_| "{}".to_string());
                        export_json_text.set(pretty);
                        show_export_modal.set(true);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load container settings for export: {}", e)));
                    }
                }
            });
        })
    };
    let close_import_modal = {
        let show_import_modal = show_import_modal.clone();
        let import_json_text = import_json_text.clone();
        Callback::from(move |_| { show_import_modal.set(false); import_json_text.set(String::new()); })
    };
    let on_import_text_change = {
        let import_json_text = import_json_text.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            import_json_text.set(input.value());
        })
    };
    let apply_imported_template = {
        let import_json_text = import_json_text.clone();
        let show_import_modal = show_import_modal.clone();
        let menu_areas = menu_areas.clone();
        let component_templates = component_templates.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let text = (*import_json_text).clone();
            if text.trim().is_empty() { return; }
            match serde_json::from_str::<JsonValue>(&text) {
                Ok(layout_json) => {
                    // Reuse same application logic as above
                    if let Some(areas) = layout_json.get("menu_areas").and_then(|v| v.as_array()) {
                        for area in areas {
                            if let (Some(area_name), Some(is_active)) = (area.get("area_name").and_then(|v| v.as_str()), area.get("is_active").and_then(|v| v.as_bool())) {
                                if let Some(existing) = (*menu_areas).iter().find(|a| a.area_name == area_name) {
                                    let mut updated = existing.clone();
                                    updated.is_active = is_active;
                                    if let Some(settings) = area.get("settings").cloned() { updated.settings = settings; }
                                    if let Some(mobile) = area.get("mobile_behavior").and_then(|v| v.as_str()) { updated.mobile_behavior = Some(mobile.to_string()); }
                                    if let Some(icon) = area.get("hamburger_icon").and_then(|v| v.as_str()) { updated.hamburger_icon = Some(icon.to_string()); }
                                    let name = area_name.to_string();
                                    wasm_bindgen_futures::spawn_local(async move {
                                        let _ = update_menu_area(&name, &updated).await;
                                    });
                                }
                            }
                        }
                    }
                    if let Some(components) = layout_json.get("component_templates").and_then(|v| v.as_array()) {
                        for comp in components {
                            if let Some(component_type) = comp.get("component_type").and_then(|v| v.as_str()) {
                                if let Some(existing) = (*component_templates).iter().find(|t| t.component_type == component_type) {
                                    let mut updated = existing.clone();
                                    if let Some(data) = comp.get("template_data").cloned() { updated.template_data = data; }
                                    if let Some(bp) = comp.get("breakpoints").cloned() { updated.breakpoints = bp; }
                                    if let Some(width) = comp.get("width_setting").and_then(|v| v.as_str()) { updated.width_setting = Some(width.to_string()); }
                                    if let Some(max_w) = comp.get("max_width").and_then(|v| v.as_str()) { updated.max_width = Some(max_w.to_string()); }
                                    if let Some(is_active) = comp.get("is_active").and_then(|v| v.as_bool()) { updated.is_active = is_active; }
                                    let id = updated.id;
                                    wasm_bindgen_futures::spawn_local(async move {
                                        let _ = update_component_template(id, &updated).await;
                                    });
                                }
                            }
                        }
                    }
                    if let Some(container) = layout_json.get("container_settings").and_then(|v| v.as_object()) {
                        let mut settings_data: Vec<SettingData> = Vec::new();
                        for (key, value) in container.iter() {
                            let setting_key = format!("container_{}", key);
                            let setting_value = if value.is_string() { value.as_str().unwrap_or("").to_string() } else { value.to_string() };
                            settings_data.push(SettingData { key: setting_key, value: setting_value, setting_type: "container".to_string(), description: None });
                        }
                        wasm_bindgen_futures::spawn_local(async move {
                            let _ = update_settings(settings_data).await;
                        });
                    }
                    show_import_modal.set(false);
                }
                Err(e) => {
                    error.set(Some(format!("Failed to parse template JSON: {}", e)));
                }
            }
        })
    };

    let switch_to_areas = {
        let current_view = current_view.clone();
        Callback::from(move |_| current_view.set(TemplateView::MenuAreas))
    };

    let switch_to_components = {
        let current_view = current_view.clone();
        Callback::from(move |_| current_view.set(TemplateView::ComponentTemplates))
    };

    let switch_to_container = {
        let current_view = current_view.clone();
        Callback::from(move |_| current_view.set(TemplateView::ContainerSettings))
    };

    // Modern Menu Designer Callbacks
    let open_modern_designer = {
        let show_modern_designer = show_modern_designer.clone();
        let current_menu_area = current_menu_area.clone();
        let current_menu_style = current_menu_style.clone();
        
        Callback::from(move |area: String| {
            current_menu_area.set(area.clone());
            
            // Force refresh all menu styles first
            force_refresh_menu_styles();
            
            // Load existing style for this area
            let current_menu_style = current_menu_style.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(Some(style)) = load_menu_style(&area).await {
                    current_menu_style.set(Some(style));
                } else {
                    current_menu_style.set(None);
                }
            });
            
            show_modern_designer.set(true);
        })
    };

    let close_modern_designer = {
        let show_modern_designer = show_modern_designer.clone();
        Callback::from(move |_| {
            show_modern_designer.set(false);
            // Force refresh menu styles when closing to ensure they're applied
            force_refresh_menu_styles();
        })
    };

    let save_menu_style_callback = {
        let notify_message = notify_message.clone();
        let current_menu_area = current_menu_area.clone();
        let current_menu_style = current_menu_style.clone();
        let show_modern_designer = show_modern_designer.clone();
        
        Callback::from(move |style: MenuStyle| {
            let area = (*current_menu_area).clone();
            let notify_message = notify_message.clone();
            let current_menu_style = current_menu_style.clone();
            let show_modern_designer = show_modern_designer.clone();
            
            // Update the local state with the new style
            current_menu_style.set(Some(style.clone()));
            
            // Debug logging
            web_sys::console::log_1(&format!("🔧 TEMPLATE_MANAGER: Saving style for area '{}', style: {:?}", area, style).into());
            
            // Check if this is a final save (when modal is about to close) or live update
            let is_final_save = !*show_modern_designer;
            
            // Save the style to database
            wasm_bindgen_futures::spawn_local(async move {
                match save_menu_style(&area, &style).await {
                    Ok(_) => {
                        if is_final_save {
                            // Show success notification for final save
                            notify_message.set(Some(("success".to_string(), "Menu style saved successfully!".to_string())));
                        } else {
                            // Just log success for live updates
                            web_sys::console::log_1(&"🎨 Menu style auto-saved successfully".into());
                        }
                    }
                    Err(e) => {
                        // Show error notifications for both cases
                        notify_message.set(Some(("error".to_string(), format!("Failed to save menu style: {}", e))));
                    }
                }
            });
        })
    };



    html! {
        <div class="template-manager">
            <div class="page-header">
                <div>
                    <h1>{"Template Manager"}</h1>
                    <p>{"Configure menu areas, component templates, and global container settings"}</p>
                </div>
            </div>

            <div class="template-controls" style="margin: 1rem 0; padding: 0.75rem; background: var(--admin-surface, #fafbfc); border: 1px solid var(--admin-border-light, #e5e7eb); border-radius: 8px; display: flex; gap: 0.75rem; align-items: center; flex-wrap: wrap;">
                <label for="master-template" style="font-weight: 600;">{"Templates:"}</label>
                <select id="master-template" class="property-select" style="max-width: 260px;"
                    onchange={{
                        let selected_template_id = selected_template_id.clone();
                        Callback::from(move |e: Event| {
                            if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                let value = select.value();
                                if let Ok(id) = value.parse::<i32>() { selected_template_id.set(Some(id)); }
                            }
                        })
                    }}
                >
                    {for (*templates).iter().map(|t| {
                        let is_selected = selected_template_id.map(|id| id == t.id).unwrap_or(false);
                        html! { <option value={t.id.to_string()} selected={is_selected}>{t.name.clone()}</option> }
                    })}
                </select>
                <button class="btn-primary" onclick={{ &apply_selected_template }} disabled={selected_template_id.is_none()}>
                    {"Apply Template"}
                </button>
                <button class="btn-secondary" onclick={{ &open_import_modal }}>{"Import Template"}</button>
                <button class="btn-secondary" onclick={{ &export_current_configuration }}>{"Export Template"}</button>
            </div>

            {if *show_import_modal {
                html! {
                    <div class="editor-modal">
                        <div class="editor-overlay" onclick={close_import_modal.clone()}></div>
                        <div class="editor-panel" style="max-width: 720px;">
                            <div class="editor-header">
                                <h3>{"Import Template JSON"}</h3>
                                <button class="close-btn" onclick={close_import_modal.clone()}>{"×"}</button>
                            </div>
                            <div class="editor-content">
                                <p>{"Paste a template JSON with keys: menu_areas, component_templates, container_settings"}</p>
                                <textarea class="code-input" style="width: 100%; height: 300px;" onchange={on_import_text_change.clone()}>{ (*import_json_text).clone() }</textarea>
                            </div>
                            <div class="editor-actions">
                                <button class="btn-primary" onclick={apply_imported_template.clone()} disabled={(*import_json_text).trim().is_empty()}>{"Apply Imported"}</button>
                                <button class="btn-secondary" onclick={close_import_modal.clone()}>{"Cancel"}</button>
                            </div>
                        </div>
                    </div>
                }
            } else { html!{} }}

            {if *show_export_modal {
                html! {
                    <div class="editor-modal">
                        <div class="editor-overlay" onclick={{
                            let show_export_modal = show_export_modal.clone();
                            Callback::from(move |_| show_export_modal.set(false))
                        }}></div>
                        <div class="editor-panel" style="max-width: 820px;">
                            <div class="editor-header">
                                <h3>{"Exported Template JSON"}</h3>
                                <button class="close-btn" onclick={{
                                    let show_export_modal = show_export_modal.clone();
                                    Callback::from(move |_| show_export_modal.set(false))
                                }}>{"×"}</button>
                            </div>
                            <div class="editor-content">
                                <pre class="code-block" style="width: 100%; max-height: 360px; overflow: auto; margin: 0; padding: 12px; background: #0b1220; color: #e5e7eb; border: 1px solid #1f2937; border-radius: 8px;">
<code style="font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace; font-size: 12px; line-height: 1.5; white-space: pre;">{ (*export_json_text).clone() }</code>
                                </pre>
                            </div>
                            <div class="editor-actions">
                                <button class="btn-secondary" onclick={{
                                    let text = (*export_json_text).clone();
                                    Callback::from(move |_| {
                                        // Create a Blob and trigger download via temporary anchor
                                        let window = web_sys::window().unwrap();
                                        let document = window.document().unwrap();
                                        let a = document.create_element("a").unwrap();
                                        let blob = web_sys::Blob::new_with_str_sequence(&js_sys::Array::of1(&JsValue::from_str(&text))).unwrap();
                                        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                                        a.set_attribute("href", &url).ok();
                                        a.set_attribute("download", "template_export.json").ok();
                                        let a_html: web_sys::HtmlElement = a.dyn_into().unwrap();
                                        a_html.click();
                                        web_sys::Url::revoke_object_url(&url).ok();
                                    })
                                }}>{"Download JSON"}</button>
                                <button class="btn-secondary" onclick={{
                                    let show_export_modal = show_export_modal.clone();
                                    Callback::from(move |_| show_export_modal.set(false))
                                }}>{"Close"}</button>
                            </div>
                        </div>
                    </div>
                }
            } else { html!{} }}

            {if let Some(ref error_msg) = *error {
                html! {
                    <div class="error-message">
                        <strong>{"Error: "}</strong>{error_msg}
                    </div>
                }
            } else {
                html! {}
            }}

            <div class="template-tabs">
                <button 
                    class={if *current_view == TemplateView::MenuAreas { "tab-button active" } else { "tab-button" }}
                    onclick={switch_to_areas}
                >
                    {"📍 Menu Areas"}
                </button>
                <button 
                    class={if *current_view == TemplateView::ComponentTemplates { "tab-button active" } else { "tab-button" }}
                    onclick={switch_to_components}
                >
                    {"🧩 Component Templates"}
                </button>
                <button 
                    class={if *current_view == TemplateView::ContainerSettings { "tab-button active" } else { "tab-button" }}
                    onclick={switch_to_container}
                >
                    {"📦 Container Settings"}
                </button>
            </div>


            <div class="template-content">
                {if let Some((msg, kind)) = (*notify_message).clone() {
                    html! { <SimpleNotification message={msg} notification_type={kind} on_close={{
                        let notify_message = notify_message.clone();
                        Callback::from(move |_| notify_message.set(None))
                    }} /> }
                } else { html!{} }}
                {if *loading {
                    html! {
                        <div class="loading">
                            <div class="loading-spinner"></div>
                            <p>{"Loading template configuration..."}</p>
                        </div>
                    }
                } else {
                    match (*current_view).clone() {
                        TemplateView::MenuAreas => html! { 
                            <MenuAreasView 
                                menu_areas={(*menu_areas).clone()}
                                on_customize={open_modern_designer.clone()}
                                on_toggle={{
                                    let menu_areas = menu_areas.clone();
                                    let error = error.clone();
                                    Callback::from(move |(area_name, is_active): (String, bool)| {
                                        web_sys::console::log_1(&format!("Toggle {} to {}", area_name, is_active).into());
                                        
                                        // Update local state immediately for responsive UI
                                        let mut areas = (*menu_areas).clone();
                                        if let Some(area) = areas.iter_mut().find(|a| a.area_name == area_name) {
                                            area.is_active = is_active;
                                            let updated_area = area.clone(); // Clone before setting state
                                            menu_areas.set(areas.clone());
                                            
                                            // Persist to backend
                                            let menu_areas_clone = menu_areas.clone();
                                            let error_clone = error.clone();
                                            let area_name_clone = area_name.clone();
                                            wasm_bindgen_futures::spawn_local(async move {
                                                match update_menu_area(&area_name_clone, &updated_area).await {
                                                    Ok(_) => {
                                                        web_sys::console::log_1(&format!("Successfully updated {} area", area_name_clone).into());
                                                    }
                                                    Err(e) => {
                                                        error_clone.set(Some(format!("Failed to update {}: {:?}", area_name_clone, e)));
                                                        // Revert local state on error
                                                        let mut reverted_areas = areas;
                                                        if let Some(revert_area) = reverted_areas.iter_mut().find(|a| a.area_name == area_name_clone) {
                                                            revert_area.is_active = !is_active;
                                                            menu_areas_clone.set(reverted_areas);
                                                        }
                                                    }
                                                }
                                            });
                                        } else {
                                            // Handle standard areas that might not exist in backend yet
                                            error.set(Some(format!("Area '{}' not found in backend", area_name)));
                                        }
                                    })
                                }}
                            /> 
                        },
                        TemplateView::ComponentTemplates => html! { 
                            <ComponentTemplatesView 
                                component_templates={(*component_templates).clone()}
                                on_modify={Callback::noop()}
                                on_template_toggled={{
                                    let component_templates = component_templates.clone();
                                    Callback::from(move |updated_template: ComponentTemplate| {
                                        // Update the component template in the local state
                                        let mut templates = (*component_templates).clone();
                                        if let Some(index) = templates.iter().position(|t| t.id == updated_template.id) {
                                            templates[index] = updated_template;
                                            component_templates.set(templates);
                                        }
                                    })
                                }}
                            /> 
                        },
                        TemplateView::ContainerSettings => html! { <ContainerSettingsView /> },
                    }
                }}
            </div>
            
            // Modern Menu Designer Modal
            {if *show_modern_designer {
                html! {
                    <ModernMenuDesigner
                        menu_area={(*current_menu_area).clone()}
                        current_style={(*current_menu_style).clone()}
                        on_style_change={save_menu_style_callback.clone()}
                        on_save_and_close={Some(Callback::noop())}
                        on_close={close_modern_designer.clone()}
                    />
                }
            } else {
                html! {}
            }}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MenuAreasViewProps {
    pub menu_areas: Vec<MenuArea>,
    pub on_toggle: Callback<(String, bool)>,
    pub on_customize: Callback<String>,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MenuAreaCustomization {
    pub area_name: String,
    
    // Background
    pub background_type: String, // "solid", "linear-gradient", "radial-gradient", "conic-gradient", "image"
    pub background_color: String,
    pub gradient_start: String,
    pub gradient_end: String,
    pub gradient_middle: String, // For 3-color gradients
    pub gradient_direction: String,
    pub gradient_shape: String, // For radial: "circle", "ellipse"
    pub gradient_position: String, // "center", "top", "bottom", etc.
    pub background_image: String,
    pub background_opacity: String, // "0" to "100"
    
    // Text Colors
    pub text_color: String,
    
    // Text Decoration & Underlines
    pub underline_enabled: bool,
    pub underline_type: String, // "solid", "dotted", "dashed", "double", "wavy"
    pub underline_thickness: String, // "1px", "2px", "3px", etc.
    pub underline_color: String,
    pub underline_animation: String, // "none", "slide-in", "fade-in", "grow", "pulse"
    pub underline_animation_duration: String,
    pub underline_position: String, // "bottom", "top", "through"
    
    // Hover Effects
    pub hover_type: String, // "color", "gradient", "shape", "button"
    pub hover_color: String,
    pub hover_gradient_start: String,
    pub hover_gradient_end: String,
    pub hover_gradient_direction: String,
    pub hover_shape: String, // "rectangle", "rounded", "circle", "custom"
    pub hover_shape_size: String,
    pub hover_animation: String,
    
    // Active Effects  
    pub active_type: String, // "color", "gradient", "shape", "button"
    pub active_color: String,
    pub active_gradient_start: String,
    pub active_gradient_end: String,
    pub active_gradient_direction: String,
    
    // Intro Animation
    pub intro_animation_type: String,
    pub intro_animation_duration: String,
    pub intro_animation_delay: String,
    pub intro_animation_easing: String,
    pub intro_animation_trigger: String,
    pub intro_animation_offset: String,
    pub intro_animation_repeat: bool,
    pub active_shape: String,
    pub active_shape_size: String,
    pub active_animation: String,
    
    // Animation Targeting
    pub animation_type: String, // "none", "slide", "fade", "bounce", "rotate", "scale"
    pub animation_duration: String,
    pub animation_target: String, // "buttons", "text", "hover_shapes", "active_shapes", "all"
    
    // Layout
    pub border_radius: String,
    pub padding: String,
    pub margin: String,
    pub box_shadow: String,
    
    // Effects
    pub effects: String, // "none", "glassmorphism", "neumorphism", "glow", "shadow"
    pub effects_intensity: String,
    pub effects_target: String, // "buttons", "text", "hover_shapes", "active_shapes", "all"
    
    // Text Effects
    pub text_shadow_enabled: bool,
    pub text_shadow_type: String, // "none", "glow", "drop-shadow", "outline", "neon"
    pub text_shadow_color: String,
    pub text_shadow_intensity: String,
    pub text_shadow_blur: String,
    pub text_shadow_offset_x: String,
    pub text_shadow_offset_y: String,
    
    // Shape Masks
    pub shape_mask_upper: String,
    pub shape_mask_lower: String,
    pub shape_mask_scale: String,
    
    // Mobile Menu
    pub mobile_hamburger_style: String, // "lines", "dots", "arrow", "custom"
    pub mobile_hamburger_color: String,
    pub mobile_hamburger_bg: String,
    pub mobile_hamburger_size: String,
    pub mobile_hamburger_padding: String,
    pub mobile_dropdown_animation: String, // "slide", "fade", "scale", "flip"
    pub mobile_dropdown_direction: String, // "down", "up", "left", "right"
    pub mobile_item_hover_type: String,
    pub mobile_item_hover_shape: String,
    pub mobile_background_type: String,
    pub mobile_background_color: String,
}

impl Default for MenuAreaCustomization {
    fn default() -> Self {
        Self {
            area_name: String::new(),
            
            // Background
            background_type: "solid".to_string(),
            background_color: "#ffffff".to_string(),
            gradient_start: "#667eea".to_string(),
            gradient_end: "#764ba2".to_string(),
            gradient_middle: "#7c3aed".to_string(),
            gradient_direction: "to-right".to_string(),
            gradient_shape: "circle".to_string(),
            gradient_position: "center".to_string(),
            background_image: String::new(),
            background_opacity: "100".to_string(),
            
            // Text Colors
            text_color: "#333333".to_string(),
            
            // Text Decoration & Underlines
            underline_enabled: false,
            underline_type: "solid".to_string(),
            underline_thickness: "2px".to_string(),
            underline_color: "#007bff".to_string(),
            underline_animation: "none".to_string(),
            underline_animation_duration: "0.3s".to_string(),
            underline_position: "bottom".to_string(),
            
            // Hover Effects
            hover_type: "color".to_string(),
            hover_color: "#007bff".to_string(),
            hover_gradient_start: "#3b82f6".to_string(),
            hover_gradient_end: "#1d4ed8".to_string(),
            hover_gradient_direction: "to-right".to_string(),
            hover_shape: "rectangle".to_string(),
            hover_shape_size: "100%".to_string(),
            hover_animation: "fade".to_string(),
            
            // Active Effects
            active_type: "color".to_string(),
            active_color: "#0056b3".to_string(),
            active_gradient_start: "#1d4ed8".to_string(),
            active_gradient_end: "#1e40af".to_string(),
            active_gradient_direction: "to-right".to_string(),
            active_shape: "rectangle".to_string(),
            active_shape_size: "100%".to_string(),
            active_animation: "scale".to_string(),
            
            // Intro Animation
            intro_animation_type: "none".to_string(),
            intro_animation_duration: "0.6s".to_string(),
            intro_animation_delay: "0s".to_string(),
            intro_animation_easing: "ease-out".to_string(),
            intro_animation_trigger: "scroll".to_string(),
            intro_animation_offset: "100px".to_string(),
            intro_animation_repeat: false,
            
            // Animation Targeting
            animation_type: "slide".to_string(),
            animation_duration: "0.3s".to_string(),
            animation_target: "all".to_string(),
            
            // Layout
            border_radius: "8px".to_string(),
            padding: "16px".to_string(),
            margin: "0px".to_string(),
            box_shadow: "0 2px 8px rgba(0,0,0,0.1)".to_string(),
            
            // Effects
            effects: "none".to_string(),
            effects_intensity: "50".to_string(),
            effects_target: "all".to_string(),
            
            // Text Effects
            text_shadow_enabled: false,
            text_shadow_type: "none".to_string(),
            text_shadow_color: "#000000".to_string(),
            text_shadow_intensity: "50".to_string(),
            text_shadow_blur: "4".to_string(),
            text_shadow_offset_x: "0".to_string(),
            text_shadow_offset_y: "2".to_string(),
            
            // Shape Masks
            shape_mask_upper: "none".to_string(),
            shape_mask_lower: "none".to_string(),
            shape_mask_scale: "100".to_string(),
            
            // Mobile Menu
            mobile_hamburger_style: "lines".to_string(),
            mobile_hamburger_color: "#ffffff".to_string(),
            mobile_hamburger_bg: "rgba(255, 255, 255, 0.1)".to_string(),
            mobile_hamburger_size: "28px".to_string(),
            mobile_hamburger_padding: "14px".to_string(),
            mobile_dropdown_animation: "slide".to_string(),
            mobile_dropdown_direction: "down".to_string(),
            mobile_item_hover_type: "color".to_string(),
            mobile_item_hover_shape: "rectangle".to_string(),
            mobile_background_type: "solid".to_string(),
            mobile_background_color: "#ffffff".to_string(),
        }
    }
}

#[function_component(MenuAreasView)]
pub fn menu_areas_view(props: &MenuAreasViewProps) -> Html {
    let customizing_area = use_state(|| None::<String>);
    let area_customizations = use_state(|| load_menu_customizations());
    
    // Apply existing customizations on component load
    {
        let area_customizations = area_customizations.clone();
        use_effect_with_deps(move |_| {
            let customizations = (*area_customizations).clone();
            for (area_name, customization) in customizations.iter() {
                let css = generate_menu_css(customization, area_name);
                inject_menu_css(area_name, &css);
            }
            || ()
        }, ());
    }
    
    let get_area_info = |area_name: &str| -> (String, String, String, bool) {
        if let Some(area) = props.menu_areas.iter().find(|a| a.area_name == area_name) {
            (
                area.display_name.clone(),
                format!("Status: {}", if area.is_active { "Active" } else { "Inactive" }),
                if area.is_active { "area-status active" } else { "area-status inactive" }.to_string(),
                area.is_active
            )
        } else {
            // Default data for standard areas
            match area_name {
                "header" => ("Header Menu".to_string(), "Status: Active".to_string(), "area-status active".to_string(), true),
                "footer" => ("Footer Menu".to_string(), "Status: Active".to_string(), "area-status active".to_string(), true),
                "floating" => ("Floating Menu".to_string(), "Status: Disabled".to_string(), "area-status inactive".to_string(), false),
                _ => ("Unknown".to_string(), "Status: Unknown".to_string(), "area-status".to_string(), false)
            }
        }
    };

    let handle_toggle = {
        let on_toggle = props.on_toggle.clone();
        Callback::from(move |(area_name, is_active): (String, bool)| {
            // For now, just call the callback - the parent will handle the actual API call
            on_toggle.emit((area_name, is_active));
        })
    };

    html! {
        <div class="menu-areas-section">
            <h2>{"Menu Areas Configuration"}</h2>
            <p>{"Enable or disable different menu areas for your site"}</p>
            
            <div class="area-cards">
                <div class="area-card">
                    <div class="area-header">
                        <h3>{"📱 Header Menu"}</h3>
                        <div class="area-controls">
                            <button 
                                class="customize-btn"
                                onclick={{
                                    let on_customize = props.on_customize.clone();
                                    Callback::from(move |_| {
                                        on_customize.emit("header".to_string());
                                    })
                                }}
                                style="
                                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                                    color: white;
                                    border: none;
                                    padding: 8px 16px;
                                    border-radius: 6px;
                                    cursor: pointer;
                                    font-size: 12px;
                                    font-weight: 600;
                                    transition: all 0.2s ease;
                                "
                            >
                                {"🎨 Customize"}
                            </button>
                        </div>
                    </div>
                    <p>{"Main navigation with mobile hamburger support"}</p>
                    <div class="area-toggle" style="margin-bottom: 8px;">
                        <label class="toggle-switch">
                            <input 
                                type="checkbox"
                                checked={get_area_info("header").3}
                                onchange={
                                    let handle_toggle = handle_toggle.clone();
                                    Callback::from(move |e: Event| {
                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                        handle_toggle.emit(("header".to_string(), target.checked()));
                                    })
                                }
                            />
                            <span class="slider"></span>
                        </label>
                    </div>
                    <div class={get_area_info("header").2}>{get_area_info("header").1}</div>
                </div>
                
                <div class="area-card">
                    <div class="area-header">
                        <h3>{"🦶 Footer Menu"}</h3>
                        <div class="area-controls">
                            <button 
                                class="customize-btn"
                                onclick={{
                                    let on_customize = props.on_customize.clone();
                                    Callback::from(move |_| {
                                        on_customize.emit("footer".to_string());
                                    })
                                }}
                                style="
                                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                                    color: white;
                                    border: none;
                                    padding: 8px 16px;
                                    border-radius: 6px;
                                    cursor: pointer;
                                    font-size: 12px;
                                    font-weight: 600;
                                    transition: all 0.2s ease;
                                "
                            >
                                {"🎨 Customize"}
                            </button>
                        </div>
                    </div>
                    <p>{"Footer navigation with layout options"}</p>
                    <div class="area-toggle" style="margin-bottom: 8px;">
                        <label class="toggle-switch">
                            <input 
                                type="checkbox"
                                checked={get_area_info("footer").3}
                                onchange={
                                    let handle_toggle = handle_toggle.clone();
                                    Callback::from(move |e: Event| {
                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                        handle_toggle.emit(("footer".to_string(), target.checked()));
                                    })
                                }
                            />
                            <span class="slider"></span>
                        </label>
                    </div>
                    <div class={get_area_info("footer").2}>{get_area_info("footer").1}</div>
                </div>
                
                <div class="area-card">
                    <div class="area-header">
                        <h3>{"🎈 Floating Menu"}</h3>
                        <div class="area-controls">
                            <button 
                                class="customize-btn"
                                onclick={{
                                    let on_customize = props.on_customize.clone();
                                    Callback::from(move |_| {
                                        on_customize.emit("floating".to_string());
                                    })
                                }}
                                style="
                                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                                    color: white;
                                    border: none;
                                    padding: 8px 16px;
                                    border-radius: 6px;
                                    cursor: pointer;
                                    font-size: 12px;
                                    font-weight: 600;
                                    transition: all 0.2s ease;
                                "
                            >
                                {"🎨 Customize"}
                            </button>
                        </div>
                    </div>
                    <p>{"Floating navigation elements"}</p>
                    <div class="area-toggle" style="margin-bottom: 8px;">
                        <label class="toggle-switch">
                            <input 
                                type="checkbox"
                                checked={get_area_info("floating").3}
                                onchange={
                                    let handle_toggle = handle_toggle.clone();
                                    Callback::from(move |e: Event| {
                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                        handle_toggle.emit(("floating".to_string(), target.checked()));
                                    })
                                }
                            />
                            <span class="slider"></span>
                        </label>
                    </div>
                    <div class={get_area_info("floating").2}>{get_area_info("floating").1}</div>
                </div>

                // Show custom menu areas
                {for props.menu_areas.iter().filter(|area| area.area_name.starts_with("custom_")).map(|area| {
                    html! {
                        <div class="area-card">
                            <div class="area-header">
                                <h3>{format!("🧩 {}", area.display_name)}</h3>
                                <div class="area-toggle">
                                    <label class="toggle-switch">
                                        <input 
                                            type="checkbox"
                                            checked={area.is_active}
                                            onchange={
                                                let handle_toggle = handle_toggle.clone();
                                                let area_name = area.area_name.clone();
                                                Callback::from(move |e: Event| {
                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                    handle_toggle.emit((area_name.clone(), target.checked()));
                                                })
                                            }
                                        />
                                        <span class="slider"></span>
                                    </label>
                                </div>
                            </div>
                            <p>{"Custom menu for page builder integration"}</p>
                            <div class={if area.is_active { "area-status active" } else { "area-status inactive" }}>
                                {format!("Status: {}", if area.is_active { "Active" } else { "Inactive" })}
                            </div>
                        </div>
                    }
                })}
            </div>

            // Customization Modal
            {if let Some(ref area_name) = *customizing_area {
                let current_customization = area_customizations.get(area_name).cloned().unwrap_or_default();
                html! {
                    <div class="customization-modal-overlay" style="
                        position: fixed;
                        top: 0;
                        left: 0;
                        right: 0;
                        bottom: 0;
                        background: rgba(0, 0, 0, 0.8);
                        backdrop-filter: blur(8px);
                        z-index: 250000;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        padding: 2.5vh 2.5vw;
                        overflow-y: auto;
                    ">
                        <div class="customization-modal" style="
                            background: transparent;
                            border-radius: 0;
                            box-shadow: none;
                            max-width: 1600px;
                            width: 98vw;
                            max-height: 95vh;
                            overflow: visible;
                            position: relative;
                            z-index: 250001;
                        ">
                            <div class="modal-header" style="
                                padding: 24px;
                                border-bottom: 1px solid #e1e5e9;
                                display: flex;
                                justify-content: space-between;
                                align-items: center;
                                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                                color: white;
                                border-radius: 16px 16px 0 0;
                            ">
                                <h2 style="margin: 0; font-size: 24px; font-weight: 700;">
                                    {format!("🎨 Customize {} Menu", 
                                        match area_name.as_str() {
                                            "header" => "Header",
                                            "footer" => "Footer", 
                                            "floating" => "Floating",
                                            _ => "Menu"
                                        }
                                    )}
                                </h2>
                                <button 
                                    onclick={{
                                        let customizing_area = customizing_area.clone();
                                        Callback::from(move |_| {
                                            customizing_area.set(None);
                                        })
                                    }}
                                    style="
                                        background: rgba(255, 255, 255, 0.2);
                                        border: none;
                                        color: white;
                                        width: 32px;
                                        height: 32px;
                                        border-radius: 50%;
                                        cursor: pointer;
                                        display: flex;
                                        align-items: center;
                                        justify-content: center;
                                        font-size: 18px;
                                        transition: all 0.2s ease;
                                    "
                                >
                                    {"×"}
                                </button>
                            </div>
                            
                            <div class="modal-content" style="padding: 24px;">
                                {render_menu_customization_panel(&current_customization, area_customizations.clone(), area_name.clone(), customizing_area.clone())}
                            </div>
                        </div>
                    </div>
                }
            } else { html! {} }}
        </div>
    }
}

// Function to render the comprehensive menu customization panel
fn render_menu_customization_panel(
    customization: &MenuAreaCustomization,
    area_customizations: UseStateHandle<HashMap<String, MenuAreaCustomization>>,
    area_name: String,
    customizing_area: UseStateHandle<Option<String>>
) -> Html {
                    let update_customization = {
        let area_customizations = area_customizations.clone();
        let area_name = area_name.clone();
        Callback::from(move |updates: Vec<(String, String)>| {
            let mut customizations = (*area_customizations).clone();
            if let Some(mut current) = customizations.get(&area_name).cloned() {
                for (key, value) in updates {
                    match key.as_str() {
                        // Background
                        "background_type" => current.background_type = value,
                        "background_color" => current.background_color = value,
                        "gradient_start" => current.gradient_start = value,
                        "gradient_end" => current.gradient_end = value,
                        "gradient_direction" => current.gradient_direction = value,
                        "background_image" => current.background_image = value,
                        
                        // Text
                        "text_color" => current.text_color = value,
                        
                        // Text Decoration & Underlines
                        "underline_enabled" => current.underline_enabled = value == "true",
                        "underline_type" => current.underline_type = value,
                        "underline_thickness" => current.underline_thickness = value,
                        "underline_color" => current.underline_color = value,
                        "underline_animation" => current.underline_animation = value,
                        "underline_animation_duration" => current.underline_animation_duration = value,
                        "underline_position" => current.underline_position = value,
                        
                        // Background (new fields)
                        "gradient_middle" => current.gradient_middle = value,
                        "gradient_shape" => current.gradient_shape = value,
                        "gradient_position" => current.gradient_position = value,
                        "background_opacity" => current.background_opacity = value,
                        
                        // Hover Effects
                        "hover_type" => current.hover_type = value,
                        "hover_color" => current.hover_color = value,
                        "hover_gradient_start" => current.hover_gradient_start = value,
                        "hover_gradient_end" => current.hover_gradient_end = value,
                        "hover_gradient_direction" => current.hover_gradient_direction = value,
                        "hover_shape" => current.hover_shape = value,
                        "hover_shape_size" => current.hover_shape_size = value,
                        "hover_animation" => current.hover_animation = value,
                        
                        // Active Effects
                        "active_type" => current.active_type = value,
                        "active_color" => current.active_color = value,
                        "active_gradient_start" => current.active_gradient_start = value,
                        "active_gradient_end" => current.active_gradient_end = value,
                        "active_gradient_direction" => current.active_gradient_direction = value,
                        "active_shape" => current.active_shape = value,
                        "active_shape_size" => current.active_shape_size = value,
                        "active_animation" => current.active_animation = value,
                        
                        // Animation
                        "animation_type" => current.animation_type = value,
                        "animation_duration" => current.animation_duration = value,
                        "animation_target" => current.animation_target = value,
                        
                        // Layout
                        "border_radius" => current.border_radius = value,
                        "padding" => current.padding = value,
                        "margin" => current.margin = value,
                        "box_shadow" => current.box_shadow = value,
                        
                        // Effects
                        "effects" => current.effects = value,
                        "effects_intensity" => current.effects_intensity = value,
                        "effects_target" => current.effects_target = value,
                        
                        // Text Effects
                        "text_shadow_enabled" => current.text_shadow_enabled = value == "true",
                        "text_shadow_type" => current.text_shadow_type = value,
                        "text_shadow_color" => current.text_shadow_color = value,
                        "text_shadow_intensity" => current.text_shadow_intensity = value,
                        "text_shadow_blur" => current.text_shadow_blur = value,
                        "text_shadow_offset_x" => current.text_shadow_offset_x = value,
                        "text_shadow_offset_y" => current.text_shadow_offset_y = value,
                        
                        // Shape Masks
                        "shape_mask_upper" => current.shape_mask_upper = value,
                        "shape_mask_lower" => current.shape_mask_lower = value,
                        "shape_mask_scale" => current.shape_mask_scale = value,
                        
                        // Mobile Menu
                        "mobile_hamburger_style" => current.mobile_hamburger_style = value,
                        "mobile_dropdown_animation" => current.mobile_dropdown_animation = value,
                        "mobile_dropdown_direction" => current.mobile_dropdown_direction = value,
                        "mobile_item_hover_type" => current.mobile_item_hover_type = value,
                        "mobile_item_hover_shape" => current.mobile_item_hover_shape = value,
                        "mobile_background_type" => current.mobile_background_type = value,
                        "mobile_background_color" => current.mobile_background_color = value,
                        
                        _ => {}
                    }
                }
                customizations.insert(area_name.clone(), current);
                area_customizations.set(customizations);
            }
        })
    };

    html! {
        <div class="modern-menu-customization-panel" style="
            background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%);
            border-radius: 20px;
            overflow: hidden;
            box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
            width: 100%;
            min-width: 900px;
            margin: 0 auto;
        ">
            // Clean Header
            <div class="panel-header" style="
                background: linear-gradient(135deg, #1e293b 0%, #334155 100%);
                color: white;
                padding: 24px 32px;
                border-bottom: 1px solid rgba(255, 255, 255, 0.1);
            ">
                <h2 style="margin: 0; font-size: 24px; font-weight: 700; display: flex; align-items: center; gap: 12px;">
                    <span style="font-size: 28px;">{"🎨"}</span>
                    {format!("Menu Designer - {}", match area_name.as_str() {
                        "header" => "Header",
                        "footer" => "Footer", 
                        "floating" => "Floating",
                        _ => "Menu"
                    })}
                </h2>
            </div>

            // Main Content with Enhanced Sections
            <div class="panel-content" style="
                padding: 32px 40px;
                background: white;
                max-height: 65vh;
                overflow-y: auto;
                min-height: 400px;
            ">
                <div class="customization-grid" style="display: grid; gap: 32px;">
                    
                    // Background & Colors Section
                    {render_background_section(customization, update_customization.clone())}

                    // Text Decoration & Underlines Section
                    {render_underline_section(customization, update_customization.clone())}

                    // Hover & Active Effects Section
                    {render_hover_active_section(customization, update_customization.clone())}

                    // Animation & Effects Section
                    {render_animation_effects_section(customization, update_customization.clone())}

                    // Mobile Menu Section
                    {render_mobile_menu_section(customization, update_customization.clone())}

                    // Layout & Shape Masks Section
                    {render_layout_shapes_section(customization, update_customization.clone())}

                    // Creative Menu Construction Section
                    {render_creative_construction_section(customization, update_customization.clone())}

                    // Live Preview Section
                    {render_live_preview_section(customization, &area_name)}
                </div>
            </div>

            // Clean Action Bar
            <div class="action-bar" style="
                background: linear-gradient(135deg, #f1f5f9 0%, #e2e8f0 100%);
                padding: 20px 40px;
                border-top: 1px solid #e2e8f0;
                display: flex;
                justify-content: flex-end;
                align-items: center;
            ">
                <div class="action-buttons" style="display: flex; gap: 12px;">
                    <button 
                        class="reset-btn"
                        style="
                            background: white;
                            color: #64748b;
                            border: 2px solid #e2e8f0;
                            padding: 14px 28px;
                            border-radius: 12px;
                            cursor: pointer;
                            font-weight: 600;
                            font-size: 14px;
                            transition: all 0.3s ease;
                            display: flex;
                            align-items: center;
                            gap: 8px;
                            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
                        "
                        onclick={{
                            let area_customizations = area_customizations.clone();
                            let area_name = area_name.clone();
                            Callback::from(move |_| {
                                reset_to_defaults(area_customizations.clone(), area_name.clone());
                            })
                        }}
                    >
                        <span style="font-size: 16px;">{"🔄"}</span>
                        {"Reset"}
                    </button>
                    <button 
                        class="apply-btn"
                        style="
                            background: var(--admin-primary-gradient);
                            color: white;
                            border: none;
                            padding: 14px 32px;
                            border-radius: 12px;
                            cursor: pointer;
                            font-weight: 700;
                            font-size: 14px;
                            transition: all 0.3s ease;
                            display: flex;
                            align-items: center;
                            gap: 8px;
                            box-shadow: 0 10px 25px rgba(59, 130, 246, 0.4);
                            position: relative;
                            overflow: hidden;
                        "
                        onclick={{
                            let area_customizations = area_customizations.clone();
                            let customizing_area = customizing_area.clone();
                            Callback::from(move |_| {
                                apply_menu_customizations(area_customizations.clone());
                                customizing_area.set(None);
                            })
                        }}
                    >
                        <span style="font-size: 16px;">{"✨"}</span>
                        {"Apply Changes"}
                        // Shimmer effect
                        <div style="
                            position: absolute;
                            top: 0;
                            left: -100%;
                            width: 100%;
                            height: 100%;
                            background: linear-gradient(90deg, transparent, rgba(255,255,255,0.2), transparent);
                            animation: shimmer 2s infinite;
                        "></div>
                    </button>
                </div>
            </div>
        </div>
    }
}

// New Enhanced Section Functions

fn render_background_section(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="enhanced-section" style="
            background: linear-gradient(135deg, #f8fafc 0%, #ffffff 100%);
            border-radius: 16px;
            padding: 28px;
            border: 1px solid #e2e8f0;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
        ">
            <div class="section-header" style="
                display: flex;
                align-items: center;
                gap: 12px;
                margin-bottom: 24px;
                padding-bottom: 16px;
                border-bottom: 2px solid #e2e8f0;
            ">
                <div style="
                    background: var(--admin-primary-gradient);
                    color: white;
                    width: 40px;
                    height: 40px;
                    border-radius: 12px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-size: 18px;
                ">
                    {"🎨"}
                </div>
                <div>
                    <h3 style="margin: 0; font-size: 20px; font-weight: 700; color: #1f2937;">
                        {"Background & Colors"}
                    </h3>
                    <p style="margin: 0; color: #6b7280; font-size: 14px;">
                        {"Set the visual foundation of your menu"}
                    </p>
                </div>
            </div>

            <div class="controls-grid" style="display: grid; gap: 24px;">
                // Background Type Selector with Visual Cards
                <div class="control-group">
                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 12px; font-size: 14px;">
                        {"Background Type"}
                    </label>
                    <div class="bg-type-cards" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(80px, 1fr)); gap: 12px; max-width: 500px;">
                        {render_background_type_cards(customization, update_customization.clone())}
                    </div>
                </div>

                // Dynamic Background Controls
                <div class="control-group">
                    {render_background_controls(customization, update_customization.clone())}
                </div>

                // Color Palette
                <div class="control-group">
                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 12px; font-size: 14px;">
                        {"Color Palette"}
                    </label>
                    <div class="color-grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; max-width: 800px;">
                        {render_enhanced_color_controls(customization, update_customization.clone())}
                    </div>
                </div>
            </div>
        </div>
    }
}

fn render_underline_section(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="enhanced-section" style="
            background: linear-gradient(135deg, #fef3c7 0%, #ffffff 100%);
            border-radius: 16px;
            padding: 28px;
            border: 1px solid #f59e0b;
            box-shadow: 0 4px 6px -1px rgba(245, 158, 11, 0.1);
        ">
            <div class="section-header" style="
                display: flex;
                align-items: center;
                gap: 12px;
                margin-bottom: 24px;
                padding-bottom: 16px;
                border-bottom: 2px solid #f59e0b;
            ">
                <div style="
                    background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
                    color: white;
                    width: 40px;
                    height: 40px;
                    border-radius: 12px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-size: 18px;
                ">
                    {"✏️"}
                </div>
                <div>
                    <h3 style="margin: 0; font-size: 20px; font-weight: 700; color: #1f2937;">
                        {"Text Decoration & Underlines"}
                    </h3>
                    <p style="margin: 0; color: #6b7280; font-size: 14px;">
                        {"Add stylish underlines and text decorations to menu items"}
                    </p>
                </div>
            </div>

            <div class="controls-grid" style="display: grid; gap: 24px;">
                // Enable/Disable Toggle
                <div class="control-group">
                    <label style="display: flex; align-items: center; gap: 12px; font-weight: 600; color: #374151; cursor: pointer;">
                        <input 
                            type="checkbox"
                            checked={customization.underline_enabled}
                            onchange={{
                                let update_customization = update_customization.clone();
                                Callback::from(move |e: Event| {
                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                    update_customization.emit(vec![("underline_enabled".to_string(), target.checked().to_string())]);
                                })
                            }}
                            style="
                                width: 20px;
                                height: 20px;
                                accent-color: #f59e0b;
                            "
                        />
                        <span>{"Enable Underline Effects"}</span>
                    </label>
                </div>

                if customization.underline_enabled {
                    // Underline Style Controls
                    <div class="control-group">
                        <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 12px; font-size: 14px;">
                            {"Underline Style"}
                        </label>
                        <div class="underline-type-cards" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(100px, 1fr)); gap: 12px; max-width: 600px;">
                            {render_underline_type_cards(customization, update_customization.clone())}
                        </div>
                    </div>

                    // Underline Properties
                    <div class="control-group">
                        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px;">
                            // Thickness
                            <div class="form-group">
                                <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                                    {"Thickness"}
                                </label>
                                <select 
                                    value={customization.underline_thickness.clone()}
                                    onchange={{
                                        let update_customization = update_customization.clone();
                                        Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                            update_customization.emit(vec![("underline_thickness".to_string(), target.value())]);
                                        })
                                    }}
                                    style="
                                        width: 100%;
                                        padding: 10px;
                                        border: 2px solid #e9ecef;
                                        border-radius: 8px;
                                        font-size: 14px;
                                        background: white;
                                    "
                                >
                                    <option value="1px">{"Thin (1px)"}</option>
                                    <option value="2px">{"Normal (2px)"}</option>
                                    <option value="3px">{"Thick (3px)"}</option>
                                    <option value="4px">{"Bold (4px)"}</option>
                                    <option value="5px">{"Extra Bold (5px)"}</option>
                                </select>
                            </div>

                            // Color
                            <div class="form-group">
                                <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                                    {"Underline Color"}
                                </label>
                                <input 
                                    type="color"
                                    value={customization.underline_color.clone()}
                                    onchange={{
                                        let update_customization = update_customization.clone();
                                        Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                            update_customization.emit(vec![("underline_color".to_string(), target.value())]);
                                        })
                                    }}
                                    style="
                                        width: 100%;
                                        height: 44px;
                                        border: 2px solid #e9ecef;
                                        border-radius: 8px;
                                        cursor: pointer;
                                    "
                                />
                            </div>

                            // Position
                            <div class="form-group">
                                <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                                    {"Position"}
                                </label>
                                <select 
                                    value={customization.underline_position.clone()}
                                    onchange={{
                                        let update_customization = update_customization.clone();
                                        Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                            update_customization.emit(vec![("underline_position".to_string(), target.value())]);
                                        })
                                    }}
                                    style="
                                        width: 100%;
                                        padding: 10px;
                                        border: 2px solid #e9ecef;
                                        border-radius: 8px;
                                        font-size: 14px;
                                        background: white;
                                    "
                                >
                                    <option value="bottom">{"Bottom"}</option>
                                    <option value="top">{"Top"}</option>
                                    <option value="through">{"Strike Through"}</option>
                                </select>
                            </div>
                        </div>
                    </div>

                    // Animation Controls
                    <div class="control-group">
                        <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 12px; font-size: 14px;">
                            {"Animation Effects"}
                        </label>
                        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px;">
                            // Animation Type
                            <div class="form-group">
                                <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                                    {"Animation Type"}
                                </label>
                                <select 
                                    value={customization.underline_animation.clone()}
                                    onchange={{
                                        let update_customization = update_customization.clone();
                                        Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                            update_customization.emit(vec![("underline_animation".to_string(), target.value())]);
                                        })
                                    }}
                                    style="
                                        width: 100%;
                                        padding: 10px;
                                        border: 2px solid #e9ecef;
                                        border-radius: 8px;
                                        font-size: 14px;
                                        background: white;
                                    "
                                >
                                    <option value="none">{"No Animation"}</option>
                                    <option value="slide-in">{"Slide In ➡️"}</option>
                                    <option value="fade-in">{"Fade In ✨"}</option>
                                    <option value="grow">{"Grow 📈"}</option>
                                    <option value="pulse">{"Pulse 💓"}</option>
                                </select>
                            </div>

                            // Animation Duration
                            <div class="form-group">
                                <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                                    {"Animation Duration"}
                                </label>
                                <select 
                                    value={customization.underline_animation_duration.clone()}
                                    onchange={{
                                        let update_customization = update_customization.clone();
                                        Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                            update_customization.emit(vec![("underline_animation_duration".to_string(), target.value())]);
                                        })
                                    }}
                                    style="
                                        width: 100%;
                                        padding: 10px;
                                        border: 2px solid #e9ecef;
                                        border-radius: 8px;
                                        font-size: 14px;
                                        background: white;
                                    "
                                >
                                    <option value="0.1s">{"Fast (0.1s)"}</option>
                                    <option value="0.2s">{"Quick (0.2s)"}</option>
                                    <option value="0.3s">{"Normal (0.3s)"}</option>
                                    <option value="0.5s">{"Smooth (0.5s)"}</option>
                                    <option value="0.8s">{"Slow (0.8s)"}</option>
                                </select>
                            </div>
                        </div>
                    </div>
                }
            </div>
        </div>
    }
}

fn render_underline_type_cards(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    let types = vec![
        ("solid", "━", "Solid"),
        ("dotted", "┅", "Dotted"),
        ("dashed", "╌", "Dashed"),
        ("double", "═", "Double"),
        ("wavy", "〰", "Wavy"),
    ];

    html! {
        <>
            {types.into_iter().map(|(value, icon, label)| {
                let is_active = customization.underline_type == value;
                let update_customization = update_customization.clone();
                html! {
                    <button
                        style={format!("
                            background: {};
                            border: 2px solid {};
                            color: {};
                            padding: 12px 8px;
                            border-radius: 12px;
                            cursor: pointer;
                            font-size: 12px;
                            font-weight: 600;
                            transition: all 0.2s ease;
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                            gap: 4px;
                        ",
                            if is_active { "linear-gradient(135deg, #f59e0b 0%, #d97706 100%)" } else { "white" },
                            if is_active { "#f59e0b" } else { "#e5e7eb" },
                            if is_active { "white" } else { "#6b7280" }
                        )}
                        onclick={Callback::from(move |_| {
                            update_customization.emit(vec![("underline_type".to_string(), value.to_string())]);
                        })}
                    >
                        <span style="font-size: 16px;">{icon}</span>
                        <span>{label}</span>
                    </button>
                }
            }).collect::<Html>()}
        </>
    }
}

fn render_background_type_cards(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    let types = vec![
        ("solid", "🎯", "Solid"),
        ("linear-gradient", "📐", "Linear"), 
        ("radial-gradient", "🔵", "Radial"),
        ("conic-gradient", "🌀", "Conic"),
        ("image", "🖼️", "Image"),
    ];

    html! {
        <>
            {types.into_iter().map(|(value, icon, label)| {
                let is_active = customization.background_type == value;
                let update_customization = update_customization.clone();
                html! {
                    <button
                        style={format!("
                            background: {};
                            border: 2px solid {};
                            color: {};
                            padding: 12px 8px;
                            border-radius: 12px;
                            cursor: pointer;
                            font-size: 12px;
                            font-weight: 600;
                            transition: all 0.2s ease;
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                            gap: 4px;
                        ",
                            if is_active { "var(--admin-primary-gradient)" } else { "white" },
                            if is_active { "#3b82f6" } else { "#e5e7eb" },
                            if is_active { "white" } else { "#6b7280" }
                        )}
                        onclick={Callback::from(move |_| {
                            update_customization.emit(vec![("background_type".to_string(), value.to_string())]);
                        })}
                    >
                        <span style="font-size: 16px;">{icon}</span>
                        <span>{label}</span>
                    </button>
                }
            }).collect::<Html>()}
        </>
    }
}

fn render_enhanced_color_controls(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    let colors = vec![
        ("text_color", "Text Color", "🔤", &customization.text_color),
        ("hover_color", "Hover Color", "👆", &customization.hover_color),
        ("active_color", "Active Color", "✨", &customization.active_color),
    ];

    html! {
        <>
            {colors.into_iter().map(|(key, label, icon, value)| {
                let update_customization = update_customization.clone();
                html! {
                    <div class="color-control" style="
                        background: white;
                        border: 1px solid #e5e7eb;
                        border-radius: 12px;
                        padding: 16px;
                        transition: all 0.2s ease;
                    ">
                        <label style="
                            display: flex;
                            align-items: center;
                            gap: 8px;
                            font-weight: 600;
                            color: #374151;
                            margin-bottom: 8px;
                            font-size: 14px;
                        ">
                            <span style="font-size: 16px;">{icon}</span>
                            {label}
                        </label>
                        <div style="display: flex; gap: 8px; align-items: center;">
                            <input
                                type="color"
                                value={value.clone()}
                                onchange={{
                                    let key = key.to_string();
                                    let update_customization = update_customization.clone();
                                    Callback::from(move |e: Event| {
                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                        update_customization.emit(vec![(key.clone(), target.value())]);
                                    })
                                }}
                                style="
                                    width: 50px;
                                    height: 50px;
                                    border: none;
                                    border-radius: 8px;
                                    cursor: pointer;
                                "
                            />
                            <input
                                type="text"
                                value={value.clone()}
                                onchange={{
                                    let key = key.to_string();
                                    Callback::from(move |e: Event| {
                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                        update_customization.emit(vec![(key.clone(), target.value())]);
                                    })
                                }}
                                style="
                                    flex: 1;
                                    padding: 8px 12px;
                                    border: 1px solid #d1d5db;
                                    border-radius: 6px;
                                    font-family: monospace;
                                    font-size: 12px;
                                "
                            />
                        </div>
                        <div style={format!("
                            margin-top: 8px;
                            height: 20px;
                            border-radius: 6px;
                            background: {};
                            border: 1px solid #e5e7eb;
                        ", value)} data-color-preview={value.clone()}></div>
                    </div>
                }
            }).collect::<Html>()}
        </>
    }
}

fn render_hover_active_section(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="section-card" style="
            background: white;
            border-radius: 16px;
            padding: 24px;
            border: 1px solid #e2e8f0;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
        ">
            <h3 style="margin: 0 0 20px 0; font-size: 18px; font-weight: 700; color: #1f2937; display: flex; align-items: center; gap: 8px;">
                <span style="font-size: 20px;">{"🎯"}</span>
                {"Hover & Active Effects"}
            </h3>
            
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 24px;">
                // Hover Effects
                <div>
                    <h4 style="margin: 0 0 16px 0; font-size: 16px; font-weight: 600; color: #374151;">{"Hover Effects"}</h4>
                    {render_effect_controls("hover", &customization.hover_type, &customization.hover_color, &customization.hover_gradient_start, &customization.hover_gradient_end, &customization.hover_gradient_direction, &customization.hover_shape, &customization.hover_shape_size, &customization.hover_animation, update_customization.clone())}
                </div>
                
                // Active Effects
                <div>
                    <h4 style="margin: 0 0 16px 0; font-size: 16px; font-weight: 600; color: #374151;">{"Active Effects"}</h4>
                    {render_effect_controls("active", &customization.active_type, &customization.active_color, &customization.active_gradient_start, &customization.active_gradient_end, &customization.active_gradient_direction, &customization.active_shape, &customization.active_shape_size, &customization.active_animation, update_customization.clone())}
                </div>
            </div>
        </div>
    }
}

fn render_animation_effects_section(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="section-card" style="
            background: white;
            border-radius: 16px;
            padding: 24px;
            border: 1px solid #e2e8f0;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
        ">
            <h3 style="margin: 0 0 20px 0; font-size: 18px; font-weight: 700; color: #1f2937; display: flex; align-items: center; gap: 8px;">
                <span style="font-size: 20px;">{"✨"}</span>
                {"Animation & Effects"}
            </h3>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px;">
                {render_animation_controls(customization, update_customization.clone())}
                {render_effects_controls(customization, update_customization.clone())}
            </div>
        </div>
    }
}

fn render_mobile_menu_section(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="section-card enhanced-mobile-designer" style="
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            border-radius: 20px;
            padding: 0;
            border: none;
            box-shadow: 0 10px 30px rgba(102, 126, 234, 0.3);
            overflow: hidden;
            position: relative;
        ">
            // Header with gradient overlay
            <div style="
                background: rgba(255, 255, 255, 0.95);
                backdrop-filter: blur(10px);
                padding: 24px;
                border-bottom: 1px solid rgba(255, 255, 255, 0.2);
            ">
                <h3 style="
                    margin: 0 0 8px 0; 
                    font-size: 24px; 
                    font-weight: 800; 
                    background: linear-gradient(135deg, #667eea, #764ba2);
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                    display: flex; 
                    align-items: center; 
                    gap: 12px;
                ">
                    <span style="font-size: 28px; filter: drop-shadow(0 2px 4px rgba(0,0,0,0.1));">{"📱"}</span>
                    {"Enhanced Mobile Menu Designer"}
                </h3>
                <p style="
                    margin: 0;
                    color: #64748b;
                    font-size: 14px;
                    font-weight: 500;
                ">
                    {"Design responsive mobile navigation with advanced controls and live preview"}
                </p>
            </div>
            
            // Content area with improved layout
            <div style="
                background: white;
                padding: 32px;
                display: grid; 
                gap: 24px;
            ">
                // Top row - Hamburger and Layout controls
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 24px;">
                    {render_enhanced_hamburger_controls(customization, update_customization.clone())}
                    {render_mobile_layout_controls(customization, update_customization.clone())}
                </div>
                
                // Middle row - Animation and behavior
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 24px;">
                    {render_enhanced_animation_controls(customization, update_customization.clone())}
                    {render_mobile_behavior_controls(customization, update_customization.clone())}
                </div>
                
                // Bottom row - Styling and preview
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 24px;">
                    {render_enhanced_styling_controls(customization, update_customization.clone())}
                    {render_mobile_preview_panel(customization)}
                </div>
            </div>
        </div>
    }
}

fn render_layout_shapes_section(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="section-card" style="
            background: white;
            border-radius: 16px;
            padding: 24px;
            border: 1px solid #e2e8f0;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
        ">
            <h3 style="margin: 0 0 20px 0; font-size: 18px; font-weight: 700; color: #1f2937; display: flex; align-items: center; gap: 8px;">
                <span style="font-size: 20px;">{"🎨"}</span>
                {"Layout & Shape Masks"}
            </h3>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px;">
                {render_layout_section(customization, update_customization.clone())}
                {render_shape_masks_section(customization, update_customization.clone())}
            </div>
        </div>
    }
}

// Advanced Control Helper Functions

fn render_effect_controls(
    prefix: &str, 
    effect_type: &str, 
    color: &str, 
    gradient_start: &str, 
    gradient_end: &str, 
    gradient_direction: &str, 
    shape: &str, 
    shape_size: &str, 
    animation: &str,
    update_customization: Callback<Vec<(String, String)>>
) -> Html {
    html! {
        <div class="effect-controls" style="
            background: #f8fafc;
            border-radius: 12px;
            padding: 16px;
            border: 1px solid #e2e8f0;
        ">
            // Effect Type Selector
            <div class="control-group" style="margin-bottom: 16px;">
                <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                    {"Effect Type"}
                </label>
                <div class="effect-type-buttons" style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 4px;">
                    {["color", "gradient", "shape", "button"].iter().map(|&type_val| {
                        let is_active = effect_type == type_val;
                        let update_customization = update_customization.clone();
                        let prefix = prefix.to_string();
                        html! {
                            <button
                                style={format!("
                                    background: {};
                                    border: 1px solid {};
                                    color: {};
                                    padding: 6px 8px;
                                    border-radius: 6px;
                                    cursor: pointer;
                                    font-size: 11px;
                                    font-weight: 600;
                                    transition: all 0.2s ease;
                                ",
                                    if is_active { "#3b82f6" } else { "white" },
                                    if is_active { "#3b82f6" } else { "#d1d5db" },
                                    if is_active { "white" } else { "#6b7280" }
                                )}
                                onclick={Callback::from(move |_| {
                                    update_customization.emit(vec![(format!("{}_type", prefix), type_val.to_string())]);
                                })}
                            >
                                {match type_val {
                                    "color" => "Color",
                                    "gradient" => "Gradient", 
                                    "shape" => "Shape",
                                    "button" => "Button",
                                    _ => type_val
                                }}
                            </button>
                        }
                    }).collect::<Html>()}
                </div>
            </div>

            // Dynamic Controls Based on Type
            {match effect_type {
                "color" => render_color_effect_controls(prefix, color, update_customization.clone()),
                "gradient" => render_gradient_effect_controls(prefix, gradient_start, gradient_end, gradient_direction, update_customization.clone()),
                "shape" => render_shape_effect_controls(prefix, shape, shape_size, animation, update_customization.clone()),
                "button" => render_button_effect_controls(prefix, color, shape, animation, update_customization.clone()),
                _ => html! {}
            }}
        </div>
    }
}

fn render_color_effect_controls(prefix: &str, color: &str, update_customization: Callback<Vec<(String, String)>>) -> Html {
    let color = color.to_string();
    let prefix = prefix.to_string();
    
    html! {
        <div class="color-effect-controls">
            <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                {"Color"}
            </label>
            <div style="display: flex; gap: 8px; align-items: center;">
                <input
                    type="color"
                    value={color.clone()}
                    onchange={{
                        let prefix = prefix.clone();
                        let update_customization = update_customization.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                            update_customization.emit(vec![(format!("{}_color", prefix), target.value())]);
                        })
                    }}
                    style="
                        width: 40px;
                        height: 40px;
                        border: none;
                        border-radius: 8px;
                        cursor: pointer;
                    "
                />
                <input
                    type="text"
                    value={color}
                    onchange={{
                        let prefix = prefix.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                            update_customization.emit(vec![(format!("{}_color", prefix), target.value())]);
                        })
                    }}
                    style="
                        flex: 1;
                        padding: 8px 12px;
                        border: 1px solid #d1d5db;
                        border-radius: 6px;
                        font-family: monospace;
                        font-size: 12px;
                    "
                />
            </div>
        </div>
    }
}

fn render_gradient_effect_controls(prefix: &str, gradient_start: &str, gradient_end: &str, gradient_direction: &str, update_customization: Callback<Vec<(String, String)>>) -> Html {
    let prefix = prefix.to_string();
    let gradient_start = gradient_start.to_string();
    let gradient_end = gradient_end.to_string();
    let gradient_direction = gradient_direction.to_string();
    
    html! {
        <div class="gradient-effect-controls" style="display: grid; gap: 12px;">
            // Start Color
            <div>
                <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 6px; font-size: 12px;">
                    {"Start Color"}
                </label>
                <div style="display: flex; gap: 6px; align-items: center;">
                    <input
                        type="color"
                        value={gradient_start.clone()}
                        onchange={{
                            let prefix = prefix.clone();
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![(format!("{}_gradient_start", prefix), target.value())]);
                            })
                        }}
                        style="width: 30px; height: 30px; border: none; border-radius: 6px; cursor: pointer;"
                    />
                    <input
                        type="text"
                        value={gradient_start}
                        onchange={{
                            let prefix = prefix.clone();
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![(format!("{}_gradient_start", prefix), target.value())]);
                            })
                        }}
                        style="flex: 1; padding: 6px 8px; border: 1px solid #d1d5db; border-radius: 4px; font-family: monospace; font-size: 11px;"
                    />
                </div>
            </div>

            // End Color
            <div>
                <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 6px; font-size: 12px;">
                    {"End Color"}
                </label>
                <div style="display: flex; gap: 6px; align-items: center;">
                    <input
                        type="color"
                        value={gradient_end.clone()}
                        onchange={{
                            let prefix = prefix.clone();
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![(format!("{}_gradient_end", prefix), target.value())]);
                            })
                        }}
                        style="width: 30px; height: 30px; border: none; border-radius: 6px; cursor: pointer;"
                    />
                    <input
                        type="text"
                        value={gradient_end}
                        onchange={{
                            let prefix = prefix.clone();
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![(format!("{}_gradient_end", prefix), target.value())]);
                            })
                        }}
                        style="flex: 1; padding: 6px 8px; border: 1px solid #d1d5db; border-radius: 4px; font-family: monospace; font-size: 11px;"
                    />
                </div>
            </div>

            // Direction
            <div>
                <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 6px; font-size: 12px;">
                    {"Direction"}
                </label>
                <select
                    value={gradient_direction}
                    onchange={{
                        let prefix = prefix.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                            update_customization.emit(vec![(format!("{}_gradient_direction", prefix), target.value())]);
                        })
                    }}
                    style="width: 100%; padding: 6px 8px; border: 1px solid #d1d5db; border-radius: 4px; font-size: 12px;"
                >
                    <option value="to-right">{"→ Right"}</option>
                    <option value="to-left">{"← Left"}</option>
                    <option value="to-bottom">{"↓ Down"}</option>
                    <option value="to-top">{"↑ Up"}</option>
                    <option value="to-bottom-right">{"↘ Bottom Right"}</option>
                    <option value="to-bottom-left">{"↙ Bottom Left"}</option>
                    <option value="to-top-right">{"↗ Top Right"}</option>
                    <option value="to-top-left">{"↖ Top Left"}</option>
                </select>
            </div>
        </div>
    }
}

fn render_shape_effect_controls(prefix: &str, shape: &str, shape_size: &str, animation: &str, update_customization: Callback<Vec<(String, String)>>) -> Html {
    let prefix = prefix.to_string();
    let shape = shape.to_string();
    let shape_size = shape_size.to_string();
    let animation = animation.to_string();
    
    html! {
        <div class="shape-effect-controls" style="display: grid; gap: 12px;">
            // Shape Type
            <div>
                <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 6px; font-size: 12px;">
                    {"Shape"}
                </label>
                <div class="shape-buttons" style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 4px;">
                    {["rectangle", "rounded", "circle", "custom"].iter().map(|&shape_val| {
                        let is_active = shape == shape_val;
                        let update_customization = update_customization.clone();
                        let prefix = prefix.clone();
                        html! {
                            <button
                                style={format!("
                                    background: {};
                                    border: 1px solid {};
                                    color: {};
                                    padding: 4px 6px;
                                    border-radius: 4px;
                                    cursor: pointer;
                                    font-size: 10px;
                                    font-weight: 600;
                                    transition: all 0.2s ease;
                                ",
                                    if is_active { "#3b82f6" } else { "white" },
                                    if is_active { "#3b82f6" } else { "#d1d5db" },
                                    if is_active { "white" } else { "#6b7280" }
                                )}
                                onclick={Callback::from(move |_| {
                                    update_customization.emit(vec![(format!("{}_shape", prefix), shape_val.to_string())]);
                                })}
                            >
                                {match shape_val {
                                    "rectangle" => "▭",
                                    "rounded" => "▢",
                                    "circle" => "●",
                                    "custom" => "✦",
                                    _ => shape_val
                                }}
                            </button>
                        }
                    }).collect::<Html>()}
                </div>
            </div>

            // Size
            <div>
                <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 6px; font-size: 12px;">
                    {"Size"}
                </label>
                <input
                    type="text"
                    value={shape_size}
                    onchange={{
                        let prefix = prefix.clone();
                        let update_customization = update_customization.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                            update_customization.emit(vec![(format!("{}_shape_size", prefix), target.value())]);
                        })
                    }}
                    placeholder="100%, 50px, etc."
                    style="width: 100%; padding: 6px 8px; border: 1px solid #d1d5db; border-radius: 4px; font-size: 12px;"
                />
            </div>

            // Animation
            <div>
                <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 6px; font-size: 12px;">
                    {"Animation"}
                </label>
                <select
                    value={animation}
                    onchange={{
                        let prefix = prefix.clone();
                        let update_customization = update_customization.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                            update_customization.emit(vec![(format!("{}_animation", prefix), target.value())]);
                        })
                    }}
                    style="width: 100%; padding: 6px 8px; border: 1px solid #d1d5db; border-radius: 4px; font-size: 12px;"
                >
                    <option value="none">{"None"}</option>
                    <option value="fade">{"Fade"}</option>
                    <option value="scale">{"Scale"}</option>
                    <option value="slide">{"Slide"}</option>
                    <option value="rotate">{"Rotate"}</option>
                    <option value="bounce">{"Bounce"}</option>
                </select>
            </div>
        </div>
    }
}

fn render_button_effect_controls(prefix: &str, color: &str, shape: &str, _animation: &str, update_customization: Callback<Vec<(String, String)>>) -> Html {
    let prefix = prefix.to_string();
    let shape = shape.to_string();
    
    html! {
        <div class="button-effect-controls" style="display: grid; gap: 12px;">
            <p style="margin: 0; font-size: 12px; color: #6b7280; font-style: italic;">
                {"Button mode creates full button-style hover effects"}
            </p>
            
            {render_color_effect_controls(&prefix, color, update_customization.clone())}
            
            <div>
                <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 6px; font-size: 12px;">
                    {"Button Style"}
                </label>
                <select
                    value={shape}
                    onchange={{
                        let prefix = prefix.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                            update_customization.emit(vec![(format!("{}_shape", prefix), target.value())]);
                        })
                    }}
                    style="width: 100%; padding: 6px 8px; border: 1px solid #d1d5db; border-radius: 4px; font-size: 12px;"
                >
                    <option value="rectangle">{"Rectangle"}</option>
                    <option value="rounded">{"Rounded"}</option>
                    <option value="pill">{"Pill"}</option>
                    <option value="outline">{"Outline"}</option>
                </select>
            </div>
        </div>
    }
}

// Advanced Animation & Effects Controls
fn render_animation_controls(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="animation-controls" style="
            background: #f8fafc;
            border-radius: 12px;
            padding: 20px;
            border: 1px solid #e2e8f0;
        ">
            <h4 style="margin: 0 0 16px 0; font-size: 16px; font-weight: 600; color: #374151; display: flex; align-items: center; gap: 8px;">
                <span style="font-size: 18px;">{"🎬"}</span>
                {"Animations"}
            </h4>
            
            <div style="display: grid; gap: 16px;">
                // Animation Type
                <div>
                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                        {"Animation Type"}
                    </label>
                    <div class="animation-type-grid" style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 6px;">
                        {["none", "slide", "fade", "bounce", "rotate", "scale"].iter().map(|&anim_type| {
                            let is_active = customization.animation_type == anim_type;
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 1px solid {};
                                        color: {};
                                        padding: 8px 12px;
                                        border-radius: 6px;
                                        cursor: pointer;
                                        font-size: 12px;
                                        font-weight: 600;
                                        transition: all 0.2s ease;
                                        display: flex;
                                        align-items: center;
                                        justify-content: center;
                                        gap: 4px;
                                    ",
                                        if is_active { "#3b82f6" } else { "white" },
                                        if is_active { "#3b82f6" } else { "#d1d5db" },
                                        if is_active { "white" } else { "#6b7280" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("animation_type".to_string(), anim_type.to_string())]);
                                    })}
                                >
                                    <span>{match anim_type {
                                        "none" => "🚫",
                                        "slide" => "➡️",
                                        "fade" => "👻",
                                        "bounce" => "🏀",
                                        "rotate" => "🔄",
                                        "scale" => "🔍",
                                        _ => "✨"
                                    }}</span>
                                    {anim_type.to_uppercase()}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Animation Target
                <div>
                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                        {"Animation Target"}
                    </label>
                    <div class="target-grid" style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 6px;">
                        {["all", "buttons", "text", "hover_shapes", "active_shapes"].iter().map(|&target| {
                            let is_active = customization.animation_target == target;
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 1px solid {};
                                        color: {};
                                        padding: 6px 10px;
                                        border-radius: 6px;
                                        cursor: pointer;
                                        font-size: 11px;
                                        font-weight: 600;
                                        transition: all 0.2s ease;
                                    ",
                                        if is_active { "#10b981" } else { "white" },
                                        if is_active { "#10b981" } else { "#d1d5db" },
                                        if is_active { "white" } else { "#6b7280" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("animation_target".to_string(), target.to_string())]);
                                    })}
                                >
                                    {match target {
                                        "all" => "🎯 All",
                                        "buttons" => "🔘 Buttons",
                                        "text" => "📝 Text",
                                        "hover_shapes" => "🎨 Hover",
                                        "active_shapes" => "⚡ Active",
                                        _ => target
                                    }}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Animation Duration
                <div>
                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                        {"Duration: "}{&customization.animation_duration}
                    </label>
                    <input
                        type="range"
                        min="0.1"
                        max="2.0"
                        step="0.1"
                        value={customization.animation_duration.trim_end_matches('s').to_string()}
                        oninput={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                let value = format!("{}s", target.value());
                                update_customization.emit(vec![("animation_duration".to_string(), value)]);
                            })
                        }}
                        style="
                            width: 100%;
                            height: 6px;
                            border-radius: 3px;
                            background: #e2e8f0;
                            outline: none;
                            cursor: pointer;
                        "
                    />
                    <div style="display: flex; justify-content: space-between; font-size: 11px; color: #6b7280; margin-top: 4px;">
                        <span>{"0.1s"}</span>
                        <span>{"Fast"}</span>
                        <span>{"Slow"}</span>
                        <span>{"2.0s"}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn render_effects_controls(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <>
        <div class="effects-controls" style="
            background: #f8fafc;
            border-radius: 12px;
            padding: 20px;
            border: 1px solid #e2e8f0;
        ">
            <h4 style="margin: 0 0 16px 0; font-size: 16px; font-weight: 600; color: #374151; display: flex; align-items: center; gap: 8px;">
                <span style="font-size: 18px;">{"✨"}</span>
                {"Visual Effects"}
            </h4>
            
            <div style="display: grid; gap: 16px;">
                // Effect Type
                <div>
                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                        {"Effect Type"}
                    </label>
                    <div class="effects-type-grid" style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 6px;">
                        {["none", "glassmorphism", "neumorphism", "glow", "shadow", "text-glow", "text-outline", "text-neon"].iter().map(|&effect_type| {
                            let is_active = customization.effects == effect_type;
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 1px solid {};
                                        color: {};
                                        padding: 10px 12px;
                                        border-radius: 8px;
                                        cursor: pointer;
                                        font-size: 12px;
                                        font-weight: 600;
                                        transition: all 0.2s ease;
                                        display: flex;
                                        align-items: center;
                                        justify-content: center;
                                        gap: 6px;
                                    ",
                                        if is_active { "#8b5cf6" } else { "white" },
                                        if is_active { "#8b5cf6" } else { "#d1d5db" },
                                        if is_active { "white" } else { "#6b7280" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("effects".to_string(), effect_type.to_string())]);
                                    })}
                                >
                                    <span>{match effect_type {
                                        "none" => "🚫",
                                        "glassmorphism" => "🔮",
                                        "neumorphism" => "🎭",
                                        "glow" => "💫",
                                        "shadow" => "🌑",
                                        "text-glow" => "✨",
                                        "text-outline" => "📝",
                                        "text-neon" => "🌈",
                                        _ => "✨"
                                    }}</span>
                                    {match effect_type {
                                        "glassmorphism" => "Glass",
                                        "neumorphism" => "Neuro",
                                        "text-glow" => "Text Glow",
                                        "text-outline" => "Outline",
                                        "text-neon" => "Neon",
                                        _ => effect_type
                                    }.to_uppercase()}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Effects Target
                <div>
                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                        {"Effects Target"}
                    </label>
                    <div class="target-grid" style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 6px;">
                        {["all", "buttons", "text", "hover_shapes", "active_shapes"].iter().map(|&target| {
                            let is_active = customization.effects_target == target;
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 1px solid {};
                                        color: {};
                                        padding: 6px 10px;
                                        border-radius: 6px;
                                        cursor: pointer;
                                        font-size: 11px;
                                        font-weight: 600;
                                        transition: all 0.2s ease;
                                    ",
                                        if is_active { "#f59e0b" } else { "white" },
                                        if is_active { "#f59e0b" } else { "#d1d5db" },
                                        if is_active { "white" } else { "#6b7280" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("effects_target".to_string(), target.to_string())]);
                                    })}
                                >
                                    {match target {
                                        "all" => "🎯 All",
                                        "buttons" => "🔘 Buttons",
                                        "text" => "📝 Text",
                                        "hover_shapes" => "🎨 Hover",
                                        "active_shapes" => "⚡ Active",
                                        _ => target
                                    }}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Effects Intensity
                <div>
                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                        {"Intensity: "}{&customization.effects_intensity}{"%"}
                    </label>
                    <input
                        type="range"
                        min="0"
                        max="100"
                        step="5"
                        value={customization.effects_intensity.clone()}
                        oninput={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("effects_intensity".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            height: 6px;
                            border-radius: 3px;
                            background: linear-gradient(to right, #e2e8f0, #8b5cf6);
                            outline: none;
                            cursor: pointer;
                        "
                    />
                    <div style="display: flex; justify-content: space-between; font-size: 11px; color: #6b7280; margin-top: 4px;">
                        <span>{"0%"}</span>
                        <span>{"Subtle"}</span>
                        <span>{"Strong"}</span>
                        <span>{"100%"}</span>
                    </div>
                </div>
            </div>
        </div>
        
        {render_text_shadow_section(customization, update_customization.clone())}
        </>
    }
}

#[allow(dead_code)]
fn render_mobile_hamburger_controls(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="mobile-hamburger-controls" style="
            background: #f0f9ff;
            border-radius: 12px;
            padding: 20px;
            border: 1px solid #bae6fd;
        ">
            <h4 style="margin: 0 0 16px 0; font-size: 16px; font-weight: 600; color: #0c4a6e; display: flex; align-items: center; gap: 8px;">
                <span style="font-size: 18px;">{"🍔"}</span>
                {"Hamburger Style"}
            </h4>
            
            <div style="display: grid; gap: 16px;">
                // Hamburger Icon Style
                <div>
                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                        {"Icon Style"}
                    </label>
                    <div class="hamburger-style-grid" style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 8px;">
                        {["lines", "dots", "arrow", "custom"].iter().map(|&style| {
                            let is_active = customization.mobile_hamburger_style == style;
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 1px solid {};
                                        color: {};
                                        padding: 12px 16px;
                                        border-radius: 8px;
                                        cursor: pointer;
                                        font-size: 12px;
                                        font-weight: 600;
                                        transition: all 0.2s ease;
                                        display: flex;
                                        flex-direction: column;
                                        align-items: center;
                                        gap: 6px;
                                    ",
                                        if is_active { "#0ea5e9" } else { "white" },
                                        if is_active { "#0ea5e9" } else { "#bae6fd" },
                                        if is_active { "white" } else { "#0c4a6e" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("mobile_hamburger_style".to_string(), style.to_string())]);
                                    })}
                                >
                                    <div style="font-size: 16px;">
                                        {match style {
                                            "lines" => "☰",
                                            "dots" => "⋮",
                                            "arrow" => "▶",
                                            "custom" => "✦",
                                            _ => "☰"
                                        }}
                                    </div>
                                    <span>{style.to_uppercase()}</span>
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Visual Preview
                <div style="
                    background: white;
                    border: 2px dashed #bae6fd;
                    border-radius: 8px;
                    padding: 16px;
                    text-align: center;
                ">
                    <div style="font-size: 11px; color: #64748b; margin-bottom: 8px;">{"Preview"}</div>
                    <div style="
                        display: inline-block;
                        padding: 8px 12px;
                        background: #f1f5f9;
                        border-radius: 6px;
                        font-size: 18px;
                        color: #0c4a6e;
                    ">
                        {match customization.mobile_hamburger_style.as_str() {
                            "lines" => "☰",
                            "dots" => "⋮",
                            "arrow" => "▶",
                            "custom" => "✦",
                            _ => "☰"
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[allow(dead_code)]
fn render_mobile_dropdown_controls(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="mobile-dropdown-controls" style="
            background: #f0fdf4;
            border-radius: 12px;
            padding: 20px;
            border: 1px solid #bbf7d0;
        ">
            <h4 style="margin: 0 0 16px 0; font-size: 16px; font-weight: 600; color: #14532d; display: flex; align-items: center; gap: 8px;">
                <span style="font-size: 18px;">{"📱"}</span>
                {"Dropdown Animation"}
            </h4>
            
            <div style="display: grid; gap: 16px;">
                // Animation Type
                <div>
                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                        {"Animation"}
                    </label>
                    <div class="dropdown-animation-grid" style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 8px;">
                        {["slide", "fade", "scale", "flip"].iter().map(|&animation| {
                            let is_active = customization.mobile_dropdown_animation == animation;
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 1px solid {};
                                        color: {};
                                        padding: 10px 12px;
                                        border-radius: 8px;
                                        cursor: pointer;
                                        font-size: 12px;
                                        font-weight: 600;
                                        transition: all 0.2s ease;
                                        display: flex;
                                        align-items: center;
                                        justify-content: center;
                                        gap: 6px;
                                    ",
                                        if is_active { "#22c55e" } else { "white" },
                                        if is_active { "#22c55e" } else { "#bbf7d0" },
                                        if is_active { "white" } else { "#14532d" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("mobile_dropdown_animation".to_string(), animation.to_string())]);
                                    })}
                                >
                                    <span>{match animation {
                                        "slide" => "📐",
                                        "fade" => "👻",
                                        "scale" => "🔍",
                                        "flip" => "🔄",
                                        _ => "✨"
                                    }}</span>
                                    {animation.to_uppercase()}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Direction
                <div>
                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                        {"Direction"}
                    </label>
                    <div class="dropdown-direction-grid" style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 8px;">
                        {["down", "up", "left", "right"].iter().map(|&direction| {
                            let is_active = customization.mobile_dropdown_direction == direction;
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 1px solid {};
                                        color: {};
                                        padding: 8px 12px;
                                        border-radius: 6px;
                                        cursor: pointer;
                                        font-size: 11px;
                                        font-weight: 600;
                                        transition: all 0.2s ease;
                                        display: flex;
                                        align-items: center;
                                        justify-content: center;
                                        gap: 4px;
                                    ",
                                        if is_active { "#16a34a" } else { "white" },
                                        if is_active { "#16a34a" } else { "#bbf7d0" },
                                        if is_active { "white" } else { "#14532d" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("mobile_dropdown_direction".to_string(), direction.to_string())]);
                                    })}
                                >
                                    <span>{match direction {
                                        "down" => "⬇️",
                                        "up" => "⬆️",
                                        "left" => "⬅️",
                                        "right" => "➡️",
                                        _ => "⬇️"
                                    }}</span>
                                    {direction.to_uppercase()}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Item Hover Type
                <div>
                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                        {"Item Hover"}
                    </label>
                    <div class="item-hover-grid" style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 8px;">
                        {["color", "gradient", "shape", "button"].iter().map(|&hover_type| {
                            let is_active = customization.mobile_item_hover_type == hover_type;
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 1px solid {};
                                        color: {};
                                        padding: 6px 10px;
                                        border-radius: 6px;
                                        cursor: pointer;
                                        font-size: 10px;
                                        font-weight: 600;
                                        transition: all 0.2s ease;
                                    ",
                                        if is_active { "#15803d" } else { "white" },
                                        if is_active { "#15803d" } else { "#bbf7d0" },
                                        if is_active { "white" } else { "#14532d" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("mobile_item_hover_type".to_string(), hover_type.to_string())]);
                                    })}
                                >
                                    {hover_type.to_uppercase()}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            </div>
        </div>
    }
}

// Enhanced Mobile Menu Control Functions
fn render_enhanced_hamburger_controls(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="enhanced-control-panel" style="
            background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
            border-radius: 16px;
            padding: 24px;
            border: 1px solid #0ea5e9;
            position: relative;
            overflow: hidden;
        ">
            // Decorative background element
            <div style="
                position: absolute;
                top: -20px;
                right: -20px;
                width: 80px;
                height: 80px;
                background: linear-gradient(45deg, #0ea5e9, #06b6d4);
                border-radius: 50%;
                opacity: 0.1;
            "></div>
            
            <h4 style="
                margin: 0 0 20px 0; 
                font-size: 18px; 
                font-weight: 700; 
                color: #0c4a6e; 
                display: flex; 
                align-items: center; 
                gap: 10px;
                position: relative;
                z-index: 1;
            ">
                <span style="
                    font-size: 22px; 
                    background: linear-gradient(45deg, #0ea5e9, #06b6d4);
                    border-radius: 8px;
                    padding: 6px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                ">{"🍔"}</span>
                {"Hamburger Icon"}
            </h4>
            
            <div style="display: grid; gap: 20px; position: relative; z-index: 1;">
                // Icon Style with enhanced visuals
                <div>
                    <label style="
                        display: block; 
                        font-weight: 600; 
                        color: #1e293b; 
                        margin-bottom: 12px; 
                        font-size: 14px;
                        text-transform: uppercase;
                        letter-spacing: 0.5px;
                    ">
                        {"Icon Style"}
                    </label>
                    <div class="enhanced-button-grid" style="
                        display: grid; 
                        grid-template-columns: repeat(2, 1fr); 
                        gap: 12px;
                    ">
                        {["lines", "dots", "arrow", "custom"].iter().map(|&style| {
                            let is_active = customization.mobile_hamburger_style == style;
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    class="enhanced-style-button"
                                    style={format!("
                                        background: {};
                                        border: 2px solid {};
                                        color: {};
                                        padding: 16px 20px;
                                        border-radius: 12px;
                                        cursor: pointer;
                                        font-size: 13px;
                                        font-weight: 700;
                                        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
                                        display: flex;
                                        flex-direction: column;
                                        align-items: center;
                                        gap: 8px;
                                        transform: {};
                                        box-shadow: {};
                                        position: relative;
                                        overflow: hidden;
                                    ",
                                        if is_active { 
                                            "linear-gradient(135deg, #0ea5e9, #06b6d4)" 
                                        } else { 
                                            "white" 
                                        },
                                        if is_active { "#0ea5e9" } else { "#e2e8f0" },
                                        if is_active { "white" } else { "#0c4a6e" },
                                        if is_active { "scale(1.05)" } else { "scale(1)" },
                                        if is_active { 
                                            "0 8px 25px rgba(14, 165, 233, 0.3)" 
                                        } else { 
                                            "0 2px 8px rgba(0, 0, 0, 0.1)" 
                                        }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("mobile_hamburger_style".to_string(), style.to_string())]);
                                    })}

                                >
                                    // Ripple effect background
                                    {if is_active {
                                        html! {
                                            <div style="
                                                position: absolute;
                                                top: 0;
                                                left: 0;
                                                right: 0;
                                                bottom: 0;
                                                background: radial-gradient(circle, rgba(255,255,255,0.2) 0%, transparent 70%);
                                                animation: pulse 2s infinite;
                                            "></div>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                    
                                    <div style="
                                        font-size: 24px; 
                                        position: relative; 
                                        z-index: 1;
                                        filter: drop-shadow(0 2px 4px rgba(0,0,0,0.1));
                                    ">
                                        {match style {
                                            "lines" => "☰",
                                            "dots" => "⋮",
                                            "arrow" => "▶",
                                            "custom" => "✦",
                                            _ => "☰"
                                        }}
                                    </div>
                                    <span style="
                                        position: relative; 
                                        z-index: 1;
                                        text-transform: uppercase;
                                        letter-spacing: 0.5px;
                                    ">
                                        {style}
                                    </span>
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Hamburger Colors
                <div>
                    <label style="
                        display: block; 
                        font-weight: 600; 
                        color: #1e293b; 
                        margin-bottom: 12px; 
                        font-size: 14px;
                        text-transform: uppercase;
                        letter-spacing: 0.5px;
                    ">
                        {"Colors"}
                    </label>
                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px;">
                        <div>
                            <label style="display: block; font-size: 12px; color: #64748b; margin-bottom: 4px;">{"Icon Color"}</label>
                            <input 
                                type="color" 
                                value={customization.mobile_hamburger_color.clone()}
                                style="width: 100%; height: 40px; border: none; border-radius: 8px; cursor: pointer;"
                                onchange={{
                                    let update_customization = update_customization.clone();
                                    Callback::from(move |e: Event| {
                                        if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                            update_customization.emit(vec![("mobile_hamburger_color".to_string(), input.value())]);
                                        }
                                    })
                                }}
                            />
                        </div>
                        <div>
                            <label style="display: block; font-size: 12px; color: #64748b; margin-bottom: 4px;">{"Background"}</label>
                            <input 
                                type="color" 
                                value={customization.mobile_hamburger_bg.clone()}
                                style="width: 100%; height: 40px; border: none; border-radius: 8px; cursor: pointer;"
                                onchange={{
                                    let update_customization = update_customization.clone();
                                    Callback::from(move |e: Event| {
                                        if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                            update_customization.emit(vec![("mobile_hamburger_bg".to_string(), input.value())]);
                                        }
                                    })
                                }}
                            />
                        </div>
                    </div>
                </div>

                // Hamburger Size & Spacing
                <div>
                    <label style="
                        display: block; 
                        font-weight: 600; 
                        color: #1e293b; 
                        margin-bottom: 12px; 
                        font-size: 14px;
                        text-transform: uppercase;
                        letter-spacing: 0.5px;
                    ">
                        {"Size & Spacing"}
                    </label>
                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px;">
                        <div>
                            <label style="display: block; font-size: 12px; color: #64748b; margin-bottom: 4px;">{"Icon Size (px)"}</label>
                            <input 
                                type="range" 
                                min="16" 
                                max="40" 
                                value={customization.mobile_hamburger_size.parse::<i32>().unwrap_or(26).to_string()}
                                style="width: 100%;"
                                onchange={{
                                    let update_customization = update_customization.clone();
                                    Callback::from(move |e: Event| {
                                        if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                            update_customization.emit(vec![("mobile_hamburger_size".to_string(), format!("{}px", input.value()))]);
                                        }
                                    })
                                }}
                            />
                            <span style="font-size: 11px; color: #94a3b8;">{format!("{}px", customization.mobile_hamburger_size.parse::<i32>().unwrap_or(26))}</span>
                        </div>
                        <div>
                            <label style="display: block; font-size: 12px; color: #64748b; margin-bottom: 4px;">{"Padding (px)"}</label>
                            <input 
                                type="range" 
                                min="4" 
                                max="20" 
                                value={customization.mobile_hamburger_padding.parse::<i32>().unwrap_or(12).to_string()}
                                style="width: 100%;"
                                onchange={{
                                    let update_customization = update_customization.clone();
                                    Callback::from(move |e: Event| {
                                        if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                            update_customization.emit(vec![("mobile_hamburger_padding".to_string(), format!("{}px", input.value()))]);
                                        }
                                    })
                                }}
                            />
                            <span style="font-size: 11px; color: #94a3b8;">{format!("{}px", customization.mobile_hamburger_padding.parse::<i32>().unwrap_or(12))}</span>
                        </div>
                    </div>
                </div>

                // Interactive Preview with animation
                <div style="
                    background: linear-gradient(135deg, white 0%, #f8fafc 100%);
                    border: 2px solid #e2e8f0;
                    border-radius: 12px;
                    padding: 20px;
                    text-align: center;
                    position: relative;
                    overflow: hidden;
                ">
                    <div style="
                        font-size: 12px; 
                        color: #64748b; 
                        margin-bottom: 12px;
                        font-weight: 600;
                        text-transform: uppercase;
                        letter-spacing: 1px;
                    ">
                        {"Live Preview"}
                    </div>
                    <div class="hamburger-preview" style="
                        display: inline-block;
                        padding: 12px 16px;
                        background: linear-gradient(135deg, #1e293b 0%, #334155 100%);
                        border-radius: 8px;
                        font-size: 24px;
                        color: white;
                        cursor: pointer;
                        transition: all 0.3s ease;
                        box-shadow: 0 4px 12px rgba(30, 41, 59, 0.3);
                        position: relative;
                        overflow: hidden;
                    "
                    onclick={Callback::from(|_e: MouseEvent| {
                        // Click animation handled by CSS
                    })}
                    >
                        // Shine effect
                        <div style="
                            position: absolute;
                            top: 0;
                            left: -100%;
                            width: 100%;
                            height: 100%;
                            background: linear-gradient(90deg, transparent, rgba(255,255,255,0.2), transparent);
                            animation: shine 3s infinite;
                        "></div>
                        
                        <span style="position: relative; z-index: 1;">
                            {match customization.mobile_hamburger_style.as_str() {
                                "lines" => "☰",
                                "dots" => "⋮",
                                "arrow" => "▶",
                                "custom" => "✦",
                                _ => "☰"
                            }}
                        </span>
                    </div>
                    <div style="
                        font-size: 11px; 
                        color: #94a3b8; 
                        margin-top: 8px;
                        font-style: italic;
                    ">
                        {"Click to test interaction"}
                    </div>
                </div>
            </div>
        </div>
    }
}

fn render_mobile_layout_controls(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="enhanced-control-panel" style="
            background: linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%);
            border-radius: 16px;
            padding: 24px;
            border: 1px solid #22c55e;
            position: relative;
            overflow: hidden;
        ">
            <div style="
                position: absolute;
                top: -20px;
                right: -20px;
                width: 80px;
                height: 80px;
                background: linear-gradient(45deg, #22c55e, #16a34a);
                border-radius: 50%;
                opacity: 0.1;
            "></div>
            
            <h4 style="
                margin: 0 0 20px 0; 
                font-size: 18px; 
                font-weight: 700; 
                color: #14532d; 
                display: flex; 
                align-items: center; 
                gap: 10px;
                position: relative;
                z-index: 1;
            ">
                <span style="
                    font-size: 22px; 
                    background: linear-gradient(45deg, #22c55e, #16a34a);
                    border-radius: 8px;
                    padding: 6px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                ">{"📐"}</span>
                {"Layout & Position"}
            </h4>
            
            <div style="display: grid; gap: 20px; position: relative; z-index: 1;">
                // Menu Position
                <div>
                    <label style="
                        display: block; 
                        font-weight: 600; 
                        color: #1e293b; 
                        margin-bottom: 12px; 
                        font-size: 14px;
                        text-transform: uppercase;
                        letter-spacing: 0.5px;
                    ">
                        {"Menu Position"}
                    </label>
                    <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 12px;">
                        {["left", "right", "center", "full"].iter().map(|&position| {
                            let is_active = customization.mobile_dropdown_direction == position;
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 2px solid {};
                                        color: {};
                                        padding: 12px 16px;
                                        border-radius: 10px;
                                        cursor: pointer;
                                        font-size: 12px;
                                        font-weight: 600;
                                        transition: all 0.3s ease;
                                        display: flex;
                                        align-items: center;
                                        justify-content: center;
                                        gap: 6px;
                                        transform: {};
                                    ",
                                        if is_active { "#22c55e" } else { "white" },
                                        if is_active { "#22c55e" } else { "#d1d5db" },
                                        if is_active { "white" } else { "#14532d" },
                                        if is_active { "scale(1.05)" } else { "scale(1)" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("mobile_dropdown_direction".to_string(), position.to_string())]);
                                    })}
                                >
                                    <span>{match position {
                                        "left" => "⬅️",
                                        "right" => "➡️",
                                        "center" => "🎯",
                                        "full" => "📱",
                                        _ => "📍"
                                    }}</span>
                                    {position.to_uppercase()}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Width Control
                <div>
                    <label style="
                        display: block; 
                        font-weight: 600; 
                        color: #1e293b; 
                        margin-bottom: 12px; 
                        font-size: 14px;
                        text-transform: uppercase;
                        letter-spacing: 0.5px;
                    ">
                        {"Menu Width"}
                    </label>
                    <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 8px;">
                        {["narrow", "medium", "wide"].iter().map(|&width| {
                            let is_active = customization.mobile_background_type == width; // Reusing field for demo
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 1px solid {};
                                        color: {};
                                        padding: 8px 12px;
                                        border-radius: 8px;
                                        cursor: pointer;
                                        font-size: 11px;
                                        font-weight: 600;
                                        transition: all 0.2s ease;
                                    ",
                                        if is_active { "#16a34a" } else { "white" },
                                        if is_active { "#16a34a" } else { "#d1d5db" },
                                        if is_active { "white" } else { "#14532d" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("mobile_background_type".to_string(), width.to_string())]);
                                    })}
                                >
                                    {width.to_uppercase()}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            </div>
        </div>
    }
}

fn render_enhanced_animation_controls(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="enhanced-control-panel" style="
            background: linear-gradient(135deg, #fefce8 0%, #fef3c7 100%);
            border-radius: 16px;
            padding: 24px;
            border: 1px solid #f59e0b;
            position: relative;
            overflow: hidden;
        ">
            <div style="
                position: absolute;
                top: -20px;
                right: -20px;
                width: 80px;
                height: 80px;
                background: linear-gradient(45deg, #f59e0b, #d97706);
                border-radius: 50%;
                opacity: 0.1;
            "></div>
            
            <h4 style="
                margin: 0 0 20px 0; 
                font-size: 18px; 
                font-weight: 700; 
                color: #92400e; 
                display: flex; 
                align-items: center; 
                gap: 10px;
                position: relative;
                z-index: 1;
            ">
                <span style="
                    font-size: 22px; 
                    background: linear-gradient(45deg, #f59e0b, #d97706);
                    border-radius: 8px;
                    padding: 6px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                ">{"✨"}</span>
                {"Animations & Effects"}
            </h4>
            
            <div style="display: grid; gap: 20px; position: relative; z-index: 1;">
                // Animation Type
                <div>
                    <label style="
                        display: block; 
                        font-weight: 600; 
                        color: #1e293b; 
                        margin-bottom: 12px; 
                        font-size: 14px;
                        text-transform: uppercase;
                        letter-spacing: 0.5px;
                    ">
                        {"Animation Style"}
                    </label>
                    <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 12px;">
                        {["slide", "fade", "scale", "bounce"].iter().map(|&animation| {
                            let is_active = customization.mobile_dropdown_animation == animation;
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 2px solid {};
                                        color: {};
                                        padding: 14px 18px;
                                        border-radius: 10px;
                                        cursor: pointer;
                                        font-size: 12px;
                                        font-weight: 600;
                                        transition: all 0.3s ease;
                                        display: flex;
                                        align-items: center;
                                        justify-content: center;
                                        gap: 8px;
                                        transform: {};
                                    ",
                                        if is_active { "#f59e0b" } else { "white" },
                                        if is_active { "#f59e0b" } else { "#d1d5db" },
                                        if is_active { "white" } else { "#92400e" },
                                        if is_active { "scale(1.05)" } else { "scale(1)" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("mobile_dropdown_animation".to_string(), animation.to_string())]);
                                    })}
                                >
                                    <span style="font-size: 16px;">{match animation {
                                        "slide" => "📐",
                                        "fade" => "👻",
                                        "scale" => "🔍",
                                        "bounce" => "🏀",
                                        _ => "✨"
                                    }}</span>
                                    {animation.to_uppercase()}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Speed Control
                <div>
                    <label style="
                        display: block; 
                        font-weight: 600; 
                        color: #1e293b; 
                        margin-bottom: 12px; 
                        font-size: 14px;
                        text-transform: uppercase;
                        letter-spacing: 0.5px;
                    ">
                        {"Animation Speed"}
                    </label>
                    <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 8px;">
                        {["slow", "normal", "fast"].iter().map(|&speed| {
                            let is_active = speed == "normal"; // Default selection for demo
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 1px solid {};
                                        color: {};
                                        padding: 8px 12px;
                                        border-radius: 8px;
                                        cursor: pointer;
                                        font-size: 11px;
                                        font-weight: 600;
                                        transition: all 0.2s ease;
                                    ",
                                        if is_active { "#d97706" } else { "white" },
                                        if is_active { "#d97706" } else { "#d1d5db" },
                                        if is_active { "white" } else { "#92400e" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        // Add speed control logic here
                                        web_sys::console::log_1(&format!("Animation speed: {}", speed).into());
                                    })}
                                >
                                    {speed.to_uppercase()}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[allow(dead_code)]
fn render_mobile_behavior_controls(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="enhanced-control-panel" style="
            background: linear-gradient(135deg, #fdf2f8 0%, #fce7f3 100%);
            border-radius: 16px;
            padding: 24px;
            border: 1px solid #ec4899;
            position: relative;
            overflow: hidden;
        ">
            <div style="
                position: absolute;
                top: -20px;
                right: -20px;
                width: 80px;
                height: 80px;
                background: linear-gradient(45deg, #ec4899, #db2777);
                border-radius: 50%;
                opacity: 0.1;
            "></div>
            
            <h4 style="
                margin: 0 0 20px 0; 
                font-size: 18px; 
                font-weight: 700; 
                color: #831843; 
                display: flex; 
                align-items: center; 
                gap: 10px;
                position: relative;
                z-index: 1;
            ">
                <span style="
                    font-size: 22px; 
                    background: linear-gradient(45deg, #ec4899, #db2777);
                    border-radius: 8px;
                    padding: 6px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                ">{"⚙️"}</span>
                {"Behavior & Interaction"}
            </h4>
            
            <div style="display: grid; gap: 20px; position: relative; z-index: 1;">
                // Close Behavior
                <div>
                    <label style="
                        display: block; 
                        font-weight: 600; 
                        color: #1e293b; 
                        margin-bottom: 12px; 
                        font-size: 14px;
                        text-transform: uppercase;
                        letter-spacing: 0.5px;
                    ">
                        {"Close Behavior"}
                    </label>
                    <div style="display: grid; gap: 8px;">
                        {[("auto", "Auto-close on navigate"), ("manual", "Manual close only"), ("outside", "Close on outside click")].iter().map(|(value, label)| {
                            let is_active = *value == "outside"; // Default for demo
                            let update_customization = update_customization.clone();
                            html! {
                                <label style="
                                    display: flex;
                                    align-items: center;
                                    gap: 12px;
                                    padding: 12px;
                                    background: white;
                                    border: 2px solid #f3f4f6;
                                    border-radius: 8px;
                                    cursor: pointer;
                                    transition: all 0.2s ease;
                                    font-size: 13px;
                                    font-weight: 500;
                                ">
                                    <input 
                                        type="radio" 
                                        name="close-behavior"
                                        checked={is_active}
                                        style="
                                            width: 16px;
                                            height: 16px;
                                            accent-color: #ec4899;
                                        "
                                        onchange={Callback::from(move |_| {
                                            web_sys::console::log_1(&format!("Close behavior: {}", value).into());
                                        })}
                                    />
                                    <span style="color: #374151;">{*label}</span>
                                </label>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Overlay Settings
                <div>
                    <label style="
                        display: block; 
                        font-weight: 600; 
                        color: #1e293b; 
                        margin-bottom: 12px; 
                        font-size: 14px;
                        text-transform: uppercase;
                        letter-spacing: 0.5px;
                    ">
                        {"Overlay Effect"}
                    </label>
                    <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 8px;">
                        {["blur", "darken", "none", "custom"].iter().map(|&overlay| {
                            let is_active = overlay == "blur"; // Default for demo
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 1px solid {};
                                        color: {};
                                        padding: 10px 14px;
                                        border-radius: 8px;
                                        cursor: pointer;
                                        font-size: 11px;
                                        font-weight: 600;
                                        transition: all 0.2s ease;
                                        text-transform: uppercase;
                                    ",
                                        if is_active { "#ec4899" } else { "white" },
                                        if is_active { "#ec4899" } else { "#d1d5db" },
                                        if is_active { "white" } else { "#831843" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        web_sys::console::log_1(&format!("Overlay: {}", overlay).into());
                                    })}
                                >
                                    {overlay}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            </div>
        </div>
    }
}

fn render_enhanced_styling_controls(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="enhanced-control-panel" style="
            background: linear-gradient(135deg, #f3e8ff 0%, #e9d5ff 100%);
            border-radius: 16px;
            padding: 24px;
            border: 1px solid #a855f7;
            position: relative;
            overflow: hidden;
        ">
            <div style="
                position: absolute;
                top: -20px;
                right: -20px;
                width: 80px;
                height: 80px;
                background: linear-gradient(45deg, #a855f7, #9333ea);
                border-radius: 50%;
                opacity: 0.1;
            "></div>
            
            <h4 style="
                margin: 0 0 20px 0; 
                font-size: 18px; 
                font-weight: 700; 
                color: #581c87; 
                display: flex; 
                align-items: center; 
                gap: 10px;
                position: relative;
                z-index: 1;
            ">
                <span style="
                    font-size: 22px; 
                    background: linear-gradient(45deg, #a855f7, #9333ea);
                    border-radius: 8px;
                    padding: 6px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                ">{"🎨"}</span>
                {"Visual Styling"}
            </h4>
            
            <div style="display: grid; gap: 20px; position: relative; z-index: 1;">
                // Theme Presets
                <div>
                    <label style="
                        display: block; 
                        font-weight: 600; 
                        color: #1e293b; 
                        margin-bottom: 12px; 
                        font-size: 14px;
                        text-transform: uppercase;
                        letter-spacing: 0.5px;
                    ">
                        {"Theme Presets"}
                    </label>
                    <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 12px;">
                        {[("modern", "#3b82f6"), ("elegant", "#1f2937"), ("vibrant", "#f59e0b"), ("minimal", "#6b7280")].iter().map(|(theme, color)| {
                            let is_active = *theme == "modern"; // Default for demo
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 2px solid {};
                                        color: white;
                                        padding: 12px 16px;
                                        border-radius: 10px;
                                        cursor: pointer;
                                        font-size: 12px;
                                        font-weight: 600;
                                        transition: all 0.3s ease;
                                        display: flex;
                                        align-items: center;
                                        justify-content: center;
                                        gap: 6px;
                                        transform: {};
                                        box-shadow: {};
                                    ",
                                        color,
                                        if is_active { "#a855f7" } else { color },
                                        if is_active { "scale(1.05)" } else { "scale(1)" },
                                        if is_active { "0 4px 12px rgba(168, 85, 247, 0.3)" } else { "0 2px 4px rgba(0,0,0,0.1)" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        web_sys::console::log_1(&format!("Theme: {}", theme).into());
                                    })}
                                >
                                    <div style="
                                        width: 12px;
                                        height: 12px;
                                        border-radius: 50%;
                                        background: rgba(255,255,255,0.3);
                                    "></div>
                                    {theme.to_uppercase()}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Background Style
                <div>
                    <label style="
                        display: block; 
                        font-weight: 600; 
                        color: #1e293b; 
                        margin-bottom: 12px; 
                        font-size: 14px;
                        text-transform: uppercase;
                        letter-spacing: 0.5px;
                    ">
                        {"Background Style"}
                    </label>
                    <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 8px;">
                        {["solid", "gradient", "glass"].iter().map(|&bg_style| {
                            let is_active = customization.mobile_background_type == bg_style;
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 1px solid {};
                                        color: {};
                                        padding: 8px 12px;
                                        border-radius: 8px;
                                        cursor: pointer;
                                        font-size: 11px;
                                        font-weight: 600;
                                        transition: all 0.2s ease;
                                        text-transform: uppercase;
                                    ",
                                        if is_active { "#a855f7" } else { "white" },
                                        if is_active { "#a855f7" } else { "#d1d5db" },
                                        if is_active { "white" } else { "#581c87" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("mobile_background_type".to_string(), bg_style.to_string())]);
                                    })}
                                >
                                    {bg_style}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            </div>
        </div>
    }
}

fn render_mobile_preview_panel(customization: &MenuAreaCustomization) -> Html {
    html! {
        <div class="mobile-preview-panel" style="
            background: linear-gradient(135deg, #1e293b 0%, #334155 100%);
            border-radius: 16px;
            padding: 24px;
            border: 1px solid #475569;
            position: relative;
            overflow: hidden;
            min-height: 400px;
        ">
            // Decorative elements
            <div style="
                position: absolute;
                top: -50px;
                right: -50px;
                width: 120px;
                height: 120px;
                background: radial-gradient(circle, rgba(59, 130, 246, 0.3) 0%, transparent 70%);
                border-radius: 50%;
            "></div>
            <div style="
                position: absolute;
                bottom: -30px;
                left: -30px;
                width: 80px;
                height: 80px;
                background: radial-gradient(circle, rgba(34, 197, 94, 0.3) 0%, transparent 70%);
                border-radius: 50%;
            "></div>
            
            <h4 style="
                margin: 0 0 20px 0; 
                font-size: 18px; 
                font-weight: 700; 
                color: white; 
                display: flex; 
                align-items: center; 
                gap: 10px;
                position: relative;
                z-index: 1;
            ">
                <span style="
                    font-size: 22px; 
                    background: var(--admin-primary-gradient);
                    border-radius: 8px;
                    padding: 6px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                ">{"📱"}</span>
                {"Live Preview"}
            </h4>
            
            // Mobile mockup
            <div style="
                position: relative;
                z-index: 1;
                display: flex;
                justify-content: center;
                align-items: center;
                height: 300px;
            ">
                <div class="mobile-mockup" style="
                    width: 200px;
                    height: 350px;
                    background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%);
                    border-radius: 24px;
                    padding: 20px 16px;
                    border: 3px solid #64748b;
                    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
                    position: relative;
                    overflow: hidden;
                ">
                    // Screen content
                    <div style="
                        width: 100%;
                        height: 100%;
                        background: white;
                        border-radius: 16px;
                        position: relative;
                        overflow: hidden;
                        box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.1);
                    ">
                        // Header bar
                        <div style="
                            height: 40px;
                            background: linear-gradient(135deg, #1e293b, #334155);
                            display: flex;
                            align-items: center;
                            justify-content: space-between;
                            padding: 0 12px;
                            color: white;
                        ">
                            <div style="font-size: 12px; font-weight: 600;">{"My Site"}</div>
                            <div style="
                                font-size: 16px;
                                cursor: pointer;
                                padding: 4px;
                                border-radius: 4px;
                                transition: all 0.2s ease;
                            "
                            onclick={Callback::from(|_e: MouseEvent| {
                                // Click animation handled by CSS
                            })}
                            >
                                {match customization.mobile_hamburger_style.as_str() {
                                    "lines" => "☰",
                                    "dots" => "⋮",
                                    "arrow" => "▶",
                                    "custom" => "✦",
                                    _ => "☰"
                                }}
                            </div>
                        </div>
                        
                        // Content area
                        <div style="
                            padding: 16px 12px;
                            height: calc(100% - 40px);
                            display: flex;
                            flex-direction: column;
                            gap: 8px;
                        ">
                            <div style="
                                height: 20px;
                                background: linear-gradient(90deg, #e2e8f0, #f1f5f9);
                                border-radius: 4px;
                                animation: shimmer 2s infinite;
                            "></div>
                            <div style="
                                height: 16px;
                                background: linear-gradient(90deg, #f1f5f9, #e2e8f0);
                                border-radius: 4px;
                                width: 80%;
                                animation: shimmer 2s infinite 0.5s;
                            "></div>
                            <div style="
                                height: 16px;
                                background: linear-gradient(90deg, #e2e8f0, #f1f5f9);
                                border-radius: 4px;
                                width: 60%;
                                animation: shimmer 2s infinite 1s;
                            "></div>
                        </div>
                    </div>
                    
                    // Home indicator (for modern phones)
                    <div style="
                        position: absolute;
                        bottom: 8px;
                        left: 50%;
                        transform: translateX(-50%);
                        width: 40px;
                        height: 4px;
                        background: #64748b;
                        border-radius: 2px;
                    "></div>
                </div>
            </div>
            
            // Settings summary
            <div style="
                position: relative;
                z-index: 1;
                margin-top: 20px;
                padding: 16px;
                background: rgba(255, 255, 255, 0.1);
                border-radius: 12px;
                backdrop-filter: blur(10px);
            ">
                <div style="
                    font-size: 12px;
                    color: #cbd5e1;
                    margin-bottom: 8px;
                    font-weight: 600;
                    text-transform: uppercase;
                    letter-spacing: 0.5px;
                ">
                    {"Current Settings"}
                </div>
                <div style="
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    gap: 8px;
                    font-size: 11px;
                    color: #e2e8f0;
                ">
                    <div>{"Icon: "}<span style="color: #60a5fa;">{&customization.mobile_hamburger_style}</span></div>
                    <div>{"Animation: "}<span style="color: #34d399;">{&customization.mobile_dropdown_animation}</span></div>
                    <div>{"Position: "}<span style="color: #fbbf24;">{&customization.mobile_dropdown_direction}</span></div>
                    <div>{"Background: "}<span style="color: #f472b6;">{&customization.mobile_background_type}</span></div>
                </div>
            </div>
        </div>
    }
}

#[allow(dead_code)]
fn render_mobile_styling_controls(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="mobile-styling-controls" style="
            background: #fefce8;
            border-radius: 12px;
            padding: 20px;
            border: 1px solid #fde047;
        ">
            <h4 style="margin: 0 0 16px 0; font-size: 16px; font-weight: 600; color: #713f12; display: flex; align-items: center; gap: 8px;">
                <span style="font-size: 18px;">{"🎨"}</span>
                {"Mobile Styling"}
            </h4>
            
            <div style="display: grid; gap: 16px;">
                // Background Type
                <div>
                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                        {"Background"}
                    </label>
                    <div class="mobile-bg-grid" style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 6px;">
                        {["solid", "gradient", "blur"].iter().map(|&bg_type| {
                            let is_active = customization.mobile_background_type == bg_type;
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 1px solid {};
                                        color: {};
                                        padding: 8px 10px;
                                        border-radius: 6px;
                                        cursor: pointer;
                                        font-size: 11px;
                                        font-weight: 600;
                                        transition: all 0.2s ease;
                                        display: flex;
                                        align-items: center;
                                        justify-content: center;
                                        gap: 4px;
                                    ",
                                        if is_active { "#eab308" } else { "white" },
                                        if is_active { "#eab308" } else { "#fde047" },
                                        if is_active { "white" } else { "#713f12" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("mobile_background_type".to_string(), bg_type.to_string())]);
                                    })}
                                >
                                    <span>{match bg_type {
                                        "solid" => "⬜",
                                        "gradient" => "🌈",
                                        "blur" => "🌫️",
                                        _ => "⬜"
                                    }}</span>
                                    {bg_type.to_uppercase()}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Background Color
                <div>
                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                        {"Background Color"}
                    </label>
                    <div style="display: flex; gap: 8px; align-items: center;">
                        <input
                            type="color"
                            value={customization.mobile_background_color.clone()}
                            onchange={{
                                let update_customization = update_customization.clone();
                                Callback::from(move |e: Event| {
                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                    update_customization.emit(vec![("mobile_background_color".to_string(), target.value())]);
                                })
                            }}
                            style="
                                width: 50px;
                                height: 40px;
                                border: none;
                                border-radius: 8px;
                                cursor: pointer;
                            "
                        />
                        <input
                            type="text"
                            value={customization.mobile_background_color.clone()}
                            onchange={{
                                let update_customization = update_customization.clone();
                                Callback::from(move |e: Event| {
                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                    update_customization.emit(vec![("mobile_background_color".to_string(), target.value())]);
                                })
                            }}
                            style="
                                flex: 1;
                                padding: 8px 12px;
                                border: 1px solid #fde047;
                                border-radius: 6px;
                                font-family: monospace;
                                font-size: 12px;
                                background: white;
                            "
                        />
                    </div>
                </div>

                // Item Hover Shape
                <div>
                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                        {"Item Hover Shape"}
                    </label>
                    <div class="hover-shape-grid" style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 6px;">
                        {["rectangle", "rounded", "circle", "custom"].iter().map(|&shape| {
                            let is_active = customization.mobile_item_hover_shape == shape;
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 1px solid {};
                                        color: {};
                                        padding: 8px 6px;
                                        border-radius: 6px;
                                        cursor: pointer;
                                        font-size: 10px;
                                        font-weight: 600;
                                        transition: all 0.2s ease;
                                        display: flex;
                                        flex-direction: column;
                                        align-items: center;
                                        gap: 2px;
                                    ",
                                        if is_active { "#ca8a04" } else { "white" },
                                        if is_active { "#ca8a04" } else { "#fde047" },
                                        if is_active { "white" } else { "#713f12" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("mobile_item_hover_shape".to_string(), shape.to_string())]);
                                    })}
                                >
                                    <span style="font-size: 12px;">{match shape {
                                        "rectangle" => "▭",
                                        "rounded" => "▢",
                                        "circle" => "●",
                                        "custom" => "✦",
                                        _ => "▭"
                                    }}</span>
                                    <span style="font-size: 8px;">{shape.chars().take(4).collect::<String>().to_uppercase()}</span>
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            </div>
        </div>
    }
}

fn render_creative_construction_section(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="section-card" style="
            background: white;
            border-radius: 16px;
            padding: 24px;
            border: 1px solid #e2e8f0;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
        ">
            <h3 style="margin: 0 0 20px 0; font-size: 18px; font-weight: 700; color: #1f2937; display: flex; align-items: center; gap: 8px;">
                <span style="font-size: 20px;">{"🚀"}</span>
                {"Creative Menu Construction"}
            </h3>
            
            <div style="display: grid; gap: 24px;">
                // Menu Layout Modes
                <div class="construction-group">
                    <h4 style="margin: 0 0 12px 0; font-size: 16px; font-weight: 600; color: #374151;">{"Layout Modes"}</h4>
                    <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(120px, 1fr)); gap: 8px;">
                        {["horizontal", "vertical", "circular", "diagonal", "floating", "sidebar"].iter().map(|&layout| {
                            let is_active = customization.padding.contains(layout); // Using padding as temp storage
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 1px solid {};
                                        color: {};
                                        padding: 12px 8px;
                                        border-radius: 8px;
                                        cursor: pointer;
                                        font-size: 11px;
                                        font-weight: 600;
                                        transition: all 0.2s ease;
                                        display: flex;
                                        flex-direction: column;
                                        align-items: center;
                                        gap: 4px;
                                    ",
                                        if is_active { "#6366f1" } else { "white" },
                                        if is_active { "#6366f1" } else { "#d1d5db" },
                                        if is_active { "white" } else { "#6b7280" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("padding".to_string(), format!("layout-{}", layout))]);
                                    })}
                                >
                                    <span style="font-size: 14px;">{match layout {
                                        "horizontal" => "↔️",
                                        "vertical" => "↕️",
                                        "circular" => "🔄",
                                        "diagonal" => "↗️",
                                        "floating" => "🎈",
                                        "sidebar" => "📋",
                                        _ => "📐"
                                    }}</span>
                                    <span>{layout.to_uppercase()}</span>
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Interactive Elements
                <div class="construction-group">
                    <h4 style="margin: 0 0 12px 0; font-size: 16px; font-weight: 600; color: #374151;">{"Interactive Elements"}</h4>
                    <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(100px, 1fr)); gap: 8px;">
                        {["magnetic", "parallax", "morphing", "particles", "waves", "3d-tilt"].iter().map(|&effect| {
                            let is_active = customization.margin.contains(effect); // Using margin as temp storage
                            let update_customization = update_customization.clone();
                            html! {
                                <button
                                    style={format!("
                                        background: {};
                                        border: 1px solid {};
                                        color: {};
                                        padding: 10px 6px;
                                        border-radius: 6px;
                                        cursor: pointer;
                                        font-size: 10px;
                                        font-weight: 600;
                                        transition: all 0.2s ease;
                                        display: flex;
                                        flex-direction: column;
                                        align-items: center;
                                        gap: 3px;
                                    ",
                                        if is_active { "#ec4899" } else { "white" },
                                        if is_active { "#ec4899" } else { "#d1d5db" },
                                        if is_active { "white" } else { "#6b7280" }
                                    )}
                                    onclick={Callback::from(move |_| {
                                        update_customization.emit(vec![("margin".to_string(), format!("effect-{}", effect))]);
                                    })}
                                >
                                    <span style="font-size: 12px;">{match effect {
                                        "magnetic" => "🧲",
                                        "parallax" => "🌌",
                                        "morphing" => "🦋",
                                        "particles" => "✨",
                                        "waves" => "🌊",
                                        "3d-tilt" => "📐",
                                        _ => "⚡"
                                    }}</span>
                                    <span>{effect.chars().take(6).collect::<String>().to_uppercase()}</span>
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Advanced Behaviors
                <div class="construction-group">
                    <h4 style="margin: 0 0 12px 0; font-size: 16px; font-weight: 600; color: #374151;">{"Advanced Behaviors"}</h4>
                    <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 12px;">
                        // Smart Positioning
                        <div style="
                            background: #f8fafc;
                            border-radius: 8px;
                            padding: 16px;
                            border: 1px solid #e2e8f0;
                        ">
                            <h5 style="margin: 0 0 8px 0; font-size: 14px; font-weight: 600; color: #374151;">{"🎯 Smart Positioning"}</h5>
                            <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 4px;">
                                {["auto-center", "follow-cursor", "edge-snap", "viewport-aware"].iter().map(|&behavior| {
                                    let update_customization = update_customization.clone();
                                    html! {
                                        <button
                                            style="
                                                background: white;
                                                border: 1px solid #d1d5db;
                                                color: #6b7280;
                                                padding: 6px 8px;
                                                border-radius: 4px;
                                                cursor: pointer;
                                                font-size: 9px;
                                                font-weight: 600;
                                                transition: all 0.2s ease;
                                            "
                                            onclick={Callback::from(move |_| {
                                                update_customization.emit(vec![("box_shadow".to_string(), format!("behavior-{}", behavior))]);
                                            })}
                                        >
                                            {behavior.replace("-", " ").to_uppercase()}
                                        </button>
                                    }
                                }).collect::<Html>()}
                            </div>
                        </div>

                        // Responsive Adaptation
                        <div style="
                            background: #f0fdf4;
                            border-radius: 8px;
                            padding: 16px;
                            border: 1px solid #bbf7d0;
                        ">
                            <h5 style="margin: 0 0 8px 0; font-size: 14px; font-weight: 600; color: #14532d;">{"📱 Responsive Magic"}</h5>
                            <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 4px;">
                                {["breakpoint-morph", "device-adapt", "orientation-shift", "size-scale"].iter().map(|&behavior| {
                                    let update_customization = update_customization.clone();
                                    html! {
                                        <button
                                            style="
                                                background: white;
                                                border: 1px solid #bbf7d0;
                                                color: #14532d;
                                                padding: 6px 8px;
                                                border-radius: 4px;
                                                cursor: pointer;
                                                font-size: 9px;
                                                font-weight: 600;
                                                transition: all 0.2s ease;
                                            "
                                            onclick={Callback::from(move |_| {
                                                update_customization.emit(vec![("border_radius".to_string(), format!("responsive-{}", behavior))]);
                                            })}
                                        >
                                            {behavior.replace("-", " ").to_uppercase()}
                                        </button>
                                    }
                                }).collect::<Html>()}
                            </div>
                        </div>
                    </div>
                </div>

                // Construction Preview
                <div style="
                    background: linear-gradient(135deg, #f1f5f9 0%, #e2e8f0 100%);
                    border-radius: 12px;
                    padding: 20px;
                    border: 2px dashed #cbd5e1;
                    text-align: center;
                ">
                    <div style="font-size: 14px; font-weight: 600; color: #475569; margin-bottom: 8px;">
                        {"🎨 Creative Construction Preview"}
                    </div>
                    <div style="font-size: 12px; color: #64748b; margin-bottom: 12px;">
                        {"Your menu will adapt and transform based on selected behaviors"}
                    </div>
                    <div style="
                        display: inline-flex;
                        align-items: center;
                        gap: 8px;
                        background: white;
                        padding: 8px 16px;
                        border-radius: 8px;
                        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
                    ">
                        <span style="font-size: 16px;">{"🚀"}</span>
                        <span style="font-size: 12px; font-weight: 600; color: #374151;">{"Dynamic Menu System"}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

// Helper functions for rendering customization sections
fn render_background_controls(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    match customization.background_type.as_str() {
        "solid" => html! {
            <>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Background Color"}
                    </label>
                    <input 
                        type="color"
                        value={customization.background_color.clone()}
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("background_color".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            height: 44px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            cursor: pointer;
                        "
                    />
                </div>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Background Opacity (%)"}
                    </label>
                    <input 
                        type="range"
                        min="0"
                        max="100"
                        value={customization.background_opacity.clone()}
                        oninput={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("background_opacity".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            height: 44px;
                        "
                    />
                    <div style="text-align: center; margin-top: 4px; font-size: 12px; color: #6c757d;">
                        {format!("{}%", customization.background_opacity)}
                    </div>
                </div>
            </>
        },
        "linear-gradient" => html! {
            <>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Gradient Start"}
                    </label>
                    <input 
                        type="color"
                        value={customization.gradient_start.clone()}
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("gradient_start".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            height: 44px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            cursor: pointer;
                        "
                    />
                </div>
                <div class="form-group" style="grid-column: 1 / -1;">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Gradient End"}
                    </label>
                    <input 
                        type="color"
                        value={customization.gradient_end.clone()}
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("gradient_end".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            height: 44px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            cursor: pointer;
                        "
                    />
                </div>
                <div class="form-group" style="grid-column: 1 / -1; margin-top: 16px;">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Gradient Direction"}
                    </label>
                    <select 
                        value={customization.gradient_direction.clone()}
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                update_customization.emit(vec![("gradient_direction".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            padding: 10px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            font-size: 14px;
                            background: white;
                        "
                    >
                        <option value="to-right">{"Left to Right →"}</option>
                        <option value="to-left">{"Right to Left ←"}</option>
                        <option value="to-bottom">{"Top to Bottom ↓"}</option>
                        <option value="to-top">{"Bottom to Top ↑"}</option>
                        <option value="to-bottom-right">{"Diagonal ↘"}</option>
                        <option value="to-bottom-left">{"Diagonal ↙"}</option>
                        <option value="to-top-right">{"Diagonal ↗"}</option>
                        <option value="to-top-left">{"Diagonal ↖"}</option>
                    </select>
                </div>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Middle Color (Optional)"}
                    </label>
                    <input 
                        type="color"
                        value={customization.gradient_middle.clone()}
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("gradient_middle".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            height: 44px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            cursor: pointer;
                        "
                    />
                </div>
            </>
        },
        "radial-gradient" => html! {
            <>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Gradient Start"}
                    </label>
                    <input 
                        type="color"
                        value={customization.gradient_start.clone()}
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("gradient_start".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            height: 44px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            cursor: pointer;
                        "
                    />
                </div>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Gradient End"}
                    </label>
                    <input 
                        type="color"
                        value={customization.gradient_end.clone()}
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("gradient_end".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            height: 44px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            cursor: pointer;
                        "
                    />
                </div>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Gradient Shape"}
                    </label>
                    <select 
                        value={customization.gradient_shape.clone()}
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                update_customization.emit(vec![("gradient_shape".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            padding: 10px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            font-size: 14px;
                            background: white;
                        "
                    >
                        <option value="circle">{"Circle ⚪"}</option>
                        <option value="ellipse">{"Ellipse ⭕"}</option>
                    </select>
                </div>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Gradient Position"}
                    </label>
                    <select 
                        value={customization.gradient_position.clone()}
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                update_customization.emit(vec![("gradient_position".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            padding: 10px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            font-size: 14px;
                            background: white;
                        "
                    >
                        <option value="center">{"Center"}</option>
                        <option value="top">{"Top"}</option>
                        <option value="bottom">{"Bottom"}</option>
                        <option value="left">{"Left"}</option>
                        <option value="right">{"Right"}</option>
                        <option value="top left">{"Top Left"}</option>
                        <option value="top right">{"Top Right"}</option>
                        <option value="bottom left">{"Bottom Left"}</option>
                        <option value="bottom right">{"Bottom Right"}</option>
                    </select>
                </div>
            </>
        },
        "conic-gradient" => html! {
            <>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Gradient Start"}
                    </label>
                    <input 
                        type="color"
                        value={customization.gradient_start.clone()}
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("gradient_start".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            height: 44px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            cursor: pointer;
                        "
                    />
                </div>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Gradient End"}
                    </label>
                    <input 
                        type="color"
                        value={customization.gradient_end.clone()}
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("gradient_end".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            height: 44px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            cursor: pointer;
                        "
                    />
                </div>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Gradient Position"}
                    </label>
                    <select 
                        value={customization.gradient_position.clone()}
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                update_customization.emit(vec![("gradient_position".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            padding: 10px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            font-size: 14px;
                            background: white;
                        "
                    >
                        <option value="center">{"Center"}</option>
                        <option value="top">{"Top"}</option>
                        <option value="bottom">{"Bottom"}</option>
                        <option value="left">{"Left"}</option>
                        <option value="right">{"Right"}</option>
                    </select>
                </div>
            </>
        },
        "image" => html! {
            <div class="form-group">
                <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                    {"Background Image URL"}
                </label>
                <input 
                    type="text"
                    value={customization.background_image.clone()}
                    placeholder="https://example.com/image.jpg"
                    onchange={{
                        let update_customization = update_customization.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                            update_customization.emit(vec![("background_image".to_string(), target.value())]);
                        })
                    }}
                    style="
                        width: 100%;
                        padding: 10px;
                        border: 2px solid #e9ecef;
                        border-radius: 8px;
                        font-size: 14px;
                    "
                />
            </div>
        },
        _ => html! {}
    }
}

#[allow(dead_code)]
fn render_color_controls(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <>
            <div class="form-group">
                <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                    {"Text Color"}
                </label>
                <input 
                    type="color"
                    value={customization.text_color.clone()}
                    onchange={{
                        let update_customization = update_customization.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                            update_customization.emit(vec![("text_color".to_string(), target.value())]);
                        })
                    }}
                    style="
                        width: 100%;
                        height: 44px;
                        border: 2px solid #e9ecef;
                        border-radius: 8px;
                        cursor: pointer;
                    "
                />
            </div>
            <div class="form-group">
                <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                    {"Hover Color"}
                </label>
                <input 
                    type="color"
                    value={customization.hover_color.clone()}
                    onchange={{
                        let update_customization = update_customization.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                            update_customization.emit(vec![("hover_color".to_string(), target.value())]);
                        })
                    }}
                    style="
                        width: 100%;
                        height: 44px;
                        border: 2px solid #e9ecef;
                        border-radius: 8px;
                        cursor: pointer;
                    "
                />
            </div>
            <div class="form-group">
                <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                    {"Active Color"}
                </label>
                <input 
                    type="color"
                    value={customization.active_color.clone()}
                    onchange={{
                        let update_customization = update_customization.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                            update_customization.emit(vec![("active_color".to_string(), target.value())]);
                        })
                    }}
                    style="
                        width: 100%;
                        height: 44px;
                        border: 2px solid #e9ecef;
                        border-radius: 8px;
                        cursor: pointer;
                    "
                />
            </div>
        </>
    }
}

#[allow(dead_code)]
fn render_animation_section(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="customization-section" style="
            background: #f8f9fa;
            padding: 20px;
            border-radius: 12px;
            border: 1px solid #e9ecef;
        ">
            <h3 style="margin: 0 0 16px 0; color: #495057; font-size: 18px; font-weight: 600;">
                {"✨ Animation & Effects"}
            </h3>
            
            <div class="form-grid" style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px;">
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Animation Type"}
                    </label>
                    <select 
                        value={customization.animation_type.clone()}
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                update_customization.emit(vec![("animation_type".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            padding: 10px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            font-size: 14px;
                            background: white;
                        "
                    >
                        <option value="none">{"No Animation"}</option>
                        <option value="slide">{"Slide"}</option>
                        <option value="fade">{"Fade"}</option>
                        <option value="bounce">{"Bounce"}</option>
                        <option value="scale">{"Scale"}</option>
                        <option value="rotate">{"Rotate"}</option>
                    </select>
                </div>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Animation Duration"}
                    </label>
                    <input 
                        type="text"
                        value={customization.animation_duration.clone()}
                        placeholder="0.3s"
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("animation_duration".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            padding: 10px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            font-size: 14px;
                        "
                    />
                </div>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Effects"}
                    </label>
                    <select 
                        value={customization.effects.clone()}
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                update_customization.emit(vec![("effects".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            padding: 10px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            font-size: 14px;
                            background: white;
                        "
                    >
                        <option value="none">{"No Effects"}</option>
                        <option value="glassmorphism">{"Glassmorphism"}</option>
                        <option value="neumorphism">{"Neumorphism"}</option>
                        <option value="shadow">{"Drop Shadow"}</option>
                        <option value="glow">{"Glow Effect"}</option>
                    </select>
                </div>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Effects Intensity (%)"}
                    </label>
                    <input 
                        type="range"
                        min="0"
                        max="100"
                        value={customization.effects_intensity.clone()}
                        oninput={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("effects_intensity".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            height: 44px;
                        "
                    />
                    <div style="text-align: center; margin-top: 4px; font-size: 12px; color: #6c757d;">
                        {format!("{}%", customization.effects_intensity)}
                    </div>
                </div>
            </div>
        </div>
    }
}

fn render_layout_section(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="customization-section" style="
            background: #f8f9fa;
            padding: 20px;
            border-radius: 12px;
            border: 1px solid #e9ecef;
        ">
            <h3 style="margin: 0 0 16px 0; color: #495057; font-size: 18px; font-weight: 600;">
                {"📐 Layout & Spacing"}
            </h3>
            
            <div class="form-grid" style="display: grid; grid-template-columns: 1fr 1fr 1fr 1fr; gap: 16px;">
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Border Radius"}
                    </label>
                    <input 
                        type="text"
                        value={customization.border_radius.clone()}
                        placeholder="8px"
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("border_radius".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            padding: 10px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            font-size: 14px;
                        "
                    />
                </div>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Padding"}
                    </label>
                    <input 
                        type="text"
                        value={customization.padding.clone()}
                        placeholder="16px"
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("padding".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            padding: 10px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            font-size: 14px;
                        "
                    />
                </div>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Margin"}
                    </label>
                    <input 
                        type="text"
                        value={customization.margin.clone()}
                        placeholder="0px"
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("margin".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            padding: 10px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            font-size: 14px;
                        "
                    />
                </div>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Box Shadow"}
                    </label>
                    <input 
                        type="text"
                        value={customization.box_shadow.clone()}
                        placeholder="0 2px 8px rgba(0,0,0,0.1)"
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("box_shadow".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            padding: 10px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            font-size: 14px;
                        "
                    />
                </div>
            </div>
        </div>
    }
}

fn render_shape_masks_section(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="customization-section" style="
            background: #f8f9fa;
            padding: 20px;
            border-radius: 12px;
            border: 1px solid #e9ecef;
        ">
            <h3 style="margin: 0 0 16px 0; color: #495057; font-size: 18px; font-weight: 600;">
                {"🎭 Shape Masks"}
            </h3>
            
            <div class="form-grid" style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 16px;">
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Upper Shape"}
                    </label>
                    <select 
                        value={customization.shape_mask_upper.clone()}
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                update_customization.emit(vec![("shape_mask_upper".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            padding: 10px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            font-size: 14px;
                            background: white;
                        "
                    >
                        <option value="none">{"None"}</option>
                        <option value="wave">{"Wave"}</option>
                        <option value="tilt">{"Tilt"}</option>
                        <option value="curve">{"Curve"}</option>
                        <option value="zigzag">{"Zigzag"}</option>
                    </select>
                </div>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Lower Shape"}
                    </label>
                    <select 
                        value={customization.shape_mask_lower.clone()}
                        onchange={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                update_customization.emit(vec![("shape_mask_lower".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            padding: 10px;
                            border: 2px solid #e9ecef;
                            border-radius: 8px;
                            font-size: 14px;
                            background: white;
                        "
                    >
                        <option value="none">{"None"}</option>
                        <option value="wave">{"Wave"}</option>
                        <option value="tilt">{"Tilt"}</option>
                        <option value="curve">{"Curve"}</option>
                        <option value="zigzag">{"Zigzag"}</option>
                    </select>
                </div>
                <div class="form-group">
                    <label style="display: block; margin-bottom: 8px; font-weight: 500; color: #495057;">
                        {"Shape Scale (%)"}
                    </label>
                    <input 
                        type="range"
                        min="50"
                        max="200"
                        value={customization.shape_mask_scale.clone()}
                        oninput={{
                            let update_customization = update_customization.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                update_customization.emit(vec![("shape_mask_scale".to_string(), target.value())]);
                            })
                        }}
                        style="
                            width: 100%;
                            height: 44px;
                        "
                    />
                    <div style="text-align: center; margin-top: 4px; font-size: 12px; color: #6c757d;">
                        {format!("{}%", customization.shape_mask_scale)}
                    </div>
                </div>
            </div>
        </div>
    }
}

fn render_live_preview_section(customization: &MenuAreaCustomization, area_name: &str) -> Html {
    let generated_css = generate_menu_css(customization, area_name);
    
    html! {
        <div class="customization-section" style="
            background: #f8f9fa;
            padding: 20px;
            border-radius: 12px;
            border: 1px solid #e9ecef;
        ">
            <h3 style="margin: 0 0 16px 0; color: #495057; font-size: 18px; font-weight: 600;">
                {"👁️ Live Preview"}
            </h3>
            
            <div class="preview-container" style="
                background: #ffffff;
                border: 2px solid #e9ecef;
                border-radius: 8px;
                padding: 20px;
                margin-bottom: 16px;
            ">
                <div class="menu-preview" style={format!("
                    {}
                    display: flex;
                    gap: 20px;
                    align-items: center;
                    padding: 12px 20px;
                    border-radius: 8px;
                    min-height: 60px;
                ", generated_css)}>
                    <div class="logo-preview" style="
                        font-weight: bold;
                        font-size: 18px;
                    ">
                        {"🌟 Logo"}
                    </div>
                    <nav class="nav-preview" style="display: flex; gap: 16px;">
                        <a href="#" style={format!("
                            {};
                            color: {};
                        ", 
                            generate_nav_link_styles_admin(customization),
                            customization.text_color
                        )}>{"Home"}</a>
                        <a href="#" style={format!("
                            {};
                            color: {};
                        ", 
                            generate_nav_link_styles_admin(customization),
                            customization.text_color
                        )}>{"About"}</a>
                        <a href="#" style={format!("
                            {};
                            color: {};
                            opacity: 0.8;
                        ", 
                            generate_nav_link_styles_admin(customization),
                            customization.text_color
                        )}>{"Contact"}</a>
                    </nav>
                </div>
            </div>
            
            <details style="margin-top: 16px;">
                <summary style="
                    cursor: pointer;
                    font-weight: 600;
                    color: #495057;
                    padding: 8px 0;
                ">
                    {"🔧 Generated CSS"}
                </summary>
                <pre style="
                    background: #f8f9fa;
                    border: 1px solid #e9ecef;
                    border-radius: 4px;
                    padding: 12px;
                    font-size: 12px;
                    overflow-x: auto;
                    margin-top: 8px;
                    white-space: pre-wrap;
                ">{generated_css}</pre>
            </details>
        </div>
    }
}

fn generate_text_shadow_css(customization: &MenuAreaCustomization) -> String {
    let intensity = customization.text_shadow_intensity.parse::<f32>().unwrap_or(50.0) / 100.0;
    let blur = customization.text_shadow_blur.parse::<f32>().unwrap_or(4.0);
    let offset_x = customization.text_shadow_offset_x.parse::<f32>().unwrap_or(0.0);
    let offset_y = customization.text_shadow_offset_y.parse::<f32>().unwrap_or(2.0);
    let color = &customization.text_shadow_color;
    
    match customization.text_shadow_type.as_str() {
        "glow" => {
            // Multiple shadows for glow effect
            let glow_blur = blur * intensity;
            format!(
                "text-shadow: 0 0 {}px {}, 0 0 {}px {}, {}px {}px {}px rgba(0,0,0,0.3)",
                glow_blur,
                color,
                glow_blur * 2.0,
                color,
                offset_x,
                offset_y,
                blur * 0.5
            )
        },
        "drop-shadow" => {
            // Standard drop shadow
            format!(
                "text-shadow: {}px {}px {}px {}",
                offset_x,
                offset_y,
                blur * intensity,
                color
            )
        },
        "outline" => {
            // Text outline using multiple shadows
            let outline_size = (blur * intensity).max(1.0);
            format!(
                "text-shadow: -{}px -{}px 0 {}, {}px -{}px 0 {}, -{}px {}px 0 {}, {}px {}px 0 {}",
                outline_size, outline_size, color,
                outline_size, outline_size, color,
                outline_size, outline_size, color,
                outline_size, outline_size, color
            )
        },
        "neon" => {
            // Neon glow effect with multiple colored shadows
            let neon_blur = blur * intensity;
            format!(
                "text-shadow: 0 0 {}px {}, 0 0 {}px {}, 0 0 {}px {}, {}px {}px {}px rgba(0,0,0,0.8)",
                neon_blur * 0.5,
                color,
                neon_blur,
                color,
                neon_blur * 2.0,
                color,
                offset_x,
                offset_y,
                blur * 0.3
            )
        },
        _ => String::new()
    }
}

fn generate_menu_css(customization: &MenuAreaCustomization, _area_name: &str) -> String {
    let _css_rules: Vec<String> = Vec::new();
    
    // Generate shape mask clip-path if needed
    let clip_path = generate_shape_mask_clip_path_admin(customization);
    
    // Main menu container styles
    let mut container_styles = Vec::new();
    
    // Background
    let opacity_factor = customization.background_opacity.parse::<f32>().unwrap_or(100.0) / 100.0;
    match customization.background_type.as_str() {
        "solid" => {
            if opacity_factor < 1.0 {
                // Convert hex to rgba for opacity support
                let hex_color = customization.background_color.trim_start_matches('#');
                if hex_color.len() == 6 {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        u8::from_str_radix(&hex_color[0..2], 16),
                        u8::from_str_radix(&hex_color[2..4], 16),
                        u8::from_str_radix(&hex_color[4..6], 16),
                    ) {
                        container_styles.push(format!("background: rgba({}, {}, {}, {})", r, g, b, opacity_factor));
                    } else {
                        container_styles.push(format!("background: {}", customization.background_color));
                    }
                } else {
                    container_styles.push(format!("background: {}", customization.background_color));
                }
            } else {
                container_styles.push(format!("background: {}", customization.background_color));
            }
        },
        "linear-gradient" => {
            let direction = match customization.gradient_direction.as_str() {
                "to-right" => "to right",
                "to-left" => "to left", 
                "to-bottom" => "to bottom",
                "to-top" => "to top",
                "to-bottom-right" => "to bottom right",
                "to-bottom-left" => "to bottom left",
                "to-top-right" => "to top right",
                "to-top-left" => "to top left",
                _ => "to right"
            };
            if !customization.gradient_middle.is_empty() && customization.gradient_middle != "#7c3aed" {
                container_styles.push(format!(
                    "background: linear-gradient({}, {}, {}, {})",
                    direction,
                    customization.gradient_start,
                    customization.gradient_middle,
                    customization.gradient_end
                ));
            } else {
                container_styles.push(format!(
                    "background: linear-gradient({}, {}, {})",
                    direction,
                    customization.gradient_start,
                    customization.gradient_end
                ));
            }
        },
        "radial-gradient" => {
            let shape = &customization.gradient_shape;
            let position = &customization.gradient_position;
            container_styles.push(format!(
                "background: radial-gradient({} {} at {}, {}, {})",
                shape,
                "closest-side",
                position,
                customization.gradient_start,
                customization.gradient_end
            ));
        },
        "conic-gradient" => {
            let position = &customization.gradient_position;
            container_styles.push(format!(
                "background: conic-gradient(from 0deg at {}, {}, {})",
                position,
                customization.gradient_start,
                customization.gradient_end
            ));
        },
        "image" => {
            if !customization.background_image.is_empty() {
                container_styles.push(format!(
                    "background: url('{}') center/cover",
                    customization.background_image
                ));
            }
        },
        _ => {}
    }
    
    // Colors
    container_styles.push(format!("color: {}", customization.text_color));
    
    // Text Shadow Effects
    if customization.text_shadow_enabled {
        let shadow_css = generate_text_shadow_css(customization);
        if !shadow_css.is_empty() {
            container_styles.push(shadow_css);
        }
    }
    
    // Layout
    container_styles.push(format!("border-radius: {}", customization.border_radius));
    container_styles.push(format!("padding: {}", customization.padding));
    container_styles.push(format!("margin: {}", customization.margin));
    
    // Shape mask
    if !clip_path.is_empty() {
        container_styles.push(format!("clip-path: {}", clip_path));
    }
    
    // Effects
    match customization.effects.as_str() {
        "glassmorphism" => {
            let intensity = customization.effects_intensity.parse::<f32>().unwrap_or(50.0) / 100.0;
            container_styles.push(format!("backdrop-filter: blur({}px)", 10.0 * intensity));
            container_styles.push(format!("background: rgba(255, 255, 255, {})", 0.1 * intensity));
            container_styles.push(format!("border: 1px solid rgba(255, 255, 255, {})", 0.2 * intensity));
        },
        "neumorphism" => {
            let intensity = customization.effects_intensity.parse::<f32>().unwrap_or(50.0) / 100.0;
            container_styles.push(format!(
                "box-shadow: {}px {}px {}px rgba(0, 0, 0, {}), -{}px -{}px {}px rgba(255, 255, 255, {})",
                (8.0 * intensity) as i32,
                (8.0 * intensity) as i32,
                (16.0 * intensity) as i32,
                0.1 * intensity,
                (8.0 * intensity) as i32,
                (8.0 * intensity) as i32,
                (16.0 * intensity) as i32,
                0.5 * intensity
            ));
        },
        "shadow" => {
            let intensity = customization.effects_intensity.parse::<f32>().unwrap_or(50.0) / 100.0;
            container_styles.push(format!(
                "box-shadow: 0 {}px {}px rgba(0, 0, 0, {})",
                (4.0 * intensity) as i32,
                (8.0 * intensity) as i32,
                0.15 * intensity
            ));
        },
        "glow" => {
            let intensity = customization.effects_intensity.parse::<f32>().unwrap_or(50.0) / 100.0;
            container_styles.push(format!(
                "box-shadow: 0 0 {}px {}",
                (20.0 * intensity) as i32,
                customization.text_color
            ));
        },
        _ => {}
    }
    
    // Animation
    if customization.animation_type != "none" {
        container_styles.push(format!("transition: all {}", customization.animation_duration));
    }
    
    // Hamburger menu CSS variables
    let hamburger_vars = vec![
        format!("--hamburger-color: {}", customization.mobile_hamburger_color),
        format!("--hamburger-bg: {}", customization.mobile_hamburger_bg),
        format!("--hamburger-icon-size: {}", customization.mobile_hamburger_size),
        format!("--hamburger-padding: {}", customization.mobile_hamburger_padding),
        format!("--hamburger-hover-bg: rgba(255, 255, 255, 0.15)"),
        format!("--hamburger-hover-color: {}", customization.text_color),
    ];
    
    // Navigation link styles with hover gradients
    let nav_styles = generate_nav_link_styles_admin(customization);
    let hover_styles = generate_hover_gradient_styles_admin(customization);
    
    format!("{}; {}; nav a {{ {} }} nav a:hover {{ {} }}", 
        container_styles.join("; "),
        hamburger_vars.join("; "),
        nav_styles,
        hover_styles
    )
}



fn generate_shape_mask_clip_path_admin(customization: &MenuAreaCustomization) -> String {
    let mut points = Vec::new();
    
    // Start with basic rectangle points
    let mut top_points = vec!["0% 0%".to_string(), "100% 0%".to_string()];
    let mut bottom_points = vec!["100% 100%".to_string(), "0% 100%".to_string()];
    
    // Apply upper shape mask
    if customization.shape_mask_upper != "none" {
        top_points = generate_shape_points_admin(&customization.shape_mask_upper, true, &customization.shape_mask_scale);
    }
    
    // Apply lower shape mask
    if customization.shape_mask_lower != "none" {
        bottom_points = generate_shape_points_admin(&customization.shape_mask_lower, false, &customization.shape_mask_scale);
    }
    
    // Combine points for polygon
    points.extend(top_points);
    points.extend(bottom_points);
    
    if points.len() > 4 {
        format!("polygon({})", points.join(", "))
    } else {
        String::new()
    }
}

fn generate_shape_points_admin(shape_type: &str, is_upper: bool, scale: &str) -> Vec<String> {
    let scale_factor = scale.parse::<f32>().unwrap_or(100.0) / 100.0;
    let amplitude = 10.0 * scale_factor;
    
    match shape_type {
        "wave" => {
            if is_upper {
                vec![
                    "0% 0%".to_string(),
                    format!("25% {}%", amplitude),
                    format!("50% 0%"),
                    format!("75% {}%", amplitude),
                    "100% 0%".to_string(),
                ]
            } else {
                vec![
                    format!("100% {}%", 100.0 - amplitude),
                    format!("75% 100%"),
                    format!("50% {}%", 100.0 - amplitude),
                    format!("25% 100%"),
                    format!("0% {}%", 100.0 - amplitude),
                ]
            }
        },
        "tilt" => {
            if is_upper {
                vec![
                    "0% 0%".to_string(),
                    format!("100% {}%", amplitude),
                ]
            } else {
                vec![
                    format!("100% {}%", 100.0 - amplitude),
                    "0% 100%".to_string(),
                ]
            }
        },
        "curve" => {
            if is_upper {
                vec![
                    "0% 0%".to_string(),
                    format!("50% {}%", amplitude),
                    "100% 0%".to_string(),
                ]
            } else {
                vec![
                    format!("100% {}%", 100.0 - amplitude),
                    format!("50% 100%"),
                    format!("0% {}%", 100.0 - amplitude),
                ]
            }
        },
        "zigzag" => {
            if is_upper {
                vec![
                    "0% 0%".to_string(),
                    format!("20% {}%", amplitude),
                    format!("40% 0%"),
                    format!("60% {}%", amplitude),
                    format!("80% 0%"),
                    "100% 0%".to_string(),
                ]
            } else {
                vec![
                    format!("100% {}%", 100.0 - amplitude),
                    format!("80% 100%"),
                    format!("60% {}%", 100.0 - amplitude),
                    format!("40% 100%"),
                    format!("20% {}%", 100.0 - amplitude),
                    "0% 100%".to_string(),
                ]
            }
        },
        _ => vec!["0% 0%".to_string(), "100% 0%".to_string()]
    }
}

fn generate_nav_link_styles_admin(customization: &MenuAreaCustomization) -> String {
    let mut styles = vec![
        format!("color: {}", customization.text_color),
        "padding: 8px 16px".to_string(),
        "border-radius: 6px".to_string(),
        "display: inline-block".to_string(),
        "position: relative".to_string(),
    ];
    
    // Handle underline styles
    if customization.underline_enabled {
        let text_decoration = match customization.underline_position.as_str() {
            "through" => format!("line-through {} {} {}", 
                customization.underline_type, 
                customization.underline_thickness, 
                customization.underline_color),
            "top" => {
                // For top underlines, we'll use a pseudo-element approach
                styles.push("text-decoration: none".to_string());
                "none".to_string()
            },
            _ => {
                // Default bottom underline
                format!("underline {} {} {}", 
                    customization.underline_type, 
                    customization.underline_thickness, 
                    customization.underline_color)
            }
        };
        
        if text_decoration != "none" {
            styles.push(format!("text-decoration: {}", text_decoration));
        } else {
            styles.push("text-decoration: none".to_string());
        }
        
        // Add animation styles
        if customization.underline_animation != "none" {
            let animation_css = match customization.underline_animation.as_str() {
                "slide-in" => format!("transition: text-decoration-color {} ease", customization.underline_animation_duration),
                "fade-in" => format!("transition: opacity {} ease", customization.underline_animation_duration),
                "grow" => format!("transition: text-decoration-thickness {} ease", customization.underline_animation_duration),
                "pulse" => format!("animation: underline-pulse {} infinite", customization.underline_animation_duration),
                _ => format!("transition: all {} ease", customization.underline_animation_duration)
            };
            styles.push(animation_css);
        }
    } else {
        styles.push("text-decoration: none".to_string());
    }
    
    if customization.animation_type != "none" {
        styles.push(format!("transition: all {} ease", customization.animation_duration));
    }
    
    styles.join("; ")
}

fn generate_hover_gradient_styles_admin(customization: &MenuAreaCustomization) -> String {
    let hover_bg = match customization.background_type.as_str() {
        "linear-gradient" => {
            let direction = customization.gradient_direction.replace("-", " ");
            format!(
                "linear-gradient({}, {}, {})",
                direction,
                customization.hover_color,
                customization.active_color
            )
        },
        "radial-gradient" => {
            format!(
                "radial-gradient({} closest-side at {}, {}, {})",
                customization.gradient_shape,
                customization.gradient_position,
                customization.hover_color,
                customization.active_color
            )
        },
        "conic-gradient" => {
            format!(
                "conic-gradient(from 0deg at {}, {}, {})",
                customization.gradient_position,
                customization.hover_color,
                customization.active_color
            )
        },
        _ => customization.hover_color.clone()
    };
    
    let styles = vec![
        format!("background: {}", hover_bg),
        "color: white".to_string(),
        "transform: translateY(-2px)".to_string(),
        "box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15)".to_string(),
    ];
    
    styles.join("; ")
}

fn reset_to_defaults(area_customizations: UseStateHandle<HashMap<String, MenuAreaCustomization>>, area_name: String) {
    let mut customizations = (*area_customizations).clone();
    customizations.insert(area_name, MenuAreaCustomization::default());
    area_customizations.set(customizations);
    web_sys::console::log_1(&"🔄 Reset menu customization to defaults".into());
}

fn apply_menu_customizations(area_customizations: UseStateHandle<HashMap<String, MenuAreaCustomization>>) {
    let customizations = (*area_customizations).clone();
    
    // Generate and inject CSS for each menu area
    for (area_name, customization) in customizations.iter() {
        let css = generate_menu_css(customization, area_name);
        inject_menu_css(area_name, &css);
        
        // Store in localStorage for persistence
        store_menu_customization(area_name, customization);
    }
    
    web_sys::console::log_1(&"✅ Applied menu customizations to public pages".into());
}

fn inject_menu_css(area_name: &str, css: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            // Remove existing style element for this area
            let style_id = format!("menu-customization-{}", area_name);
            if let Some(existing_style) = document.get_element_by_id(&style_id) {
                existing_style.remove();
            }
            
            // Create new style element
            if let Ok(style_element) = document.create_element("style") {
                style_element.set_id(&style_id);
                
                let css_selector = match area_name {
                    "header" => ".site-header nav, .site-header .header-nav, .site-header .hamburger-menu-container, .site-header",
                    "footer" => ".site-footer nav, .site-footer .footer-nav", 
                    "floating" => ".floating-menu, .floating-nav",
                    _ => &format!(".{}-menu", area_name)
                };
                
                let full_css = format!("{} {{ {} }}", css_selector, css);
                style_element.set_text_content(Some(&full_css));
                
                if let Some(head) = document.head() {
                    let _ = head.append_child(&style_element);
                }
            }
        }
    }
}

fn store_menu_customization(area_name: &str, customization: &MenuAreaCustomization) {
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            let key = format!("menu_customization_{}", area_name);
            if let Ok(json) = serde_json::to_string(customization) {
                let _ = storage.set_item(&key, &json);
            }
        }
    }
}

fn load_menu_customizations() -> HashMap<String, MenuAreaCustomization> {
    let mut customizations = HashMap::new();
    
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            for area_name in &["header", "footer", "floating"] {
                let key = format!("menu_customization_{}", area_name);
                if let Ok(Some(json)) = storage.get_item(&key) {
                    if let Ok(customization) = serde_json::from_str::<MenuAreaCustomization>(&json) {
                        customizations.insert(area_name.to_string(), customization);
                    }
                }
            }
        }
    }
    
    // Fill in defaults for any missing areas
    for area_name in &["header", "footer", "floating"] {
        if !customizations.contains_key(*area_name) {
            customizations.insert(area_name.to_string(), MenuAreaCustomization {
                area_name: area_name.to_string(),
                ..MenuAreaCustomization::default()
            });
        }
    }
    
    customizations
}

// Helper functions for rendering component templates
fn render_component_preview(component_type: &str, header_navigation: &UseStateHandle<Vec<NavigationItem>>, footer_navigation: &UseStateHandle<Vec<NavigationItem>>) -> Html {
    match component_type {
        "Comments" => html! {
            <div class="live-comments-preview">
                <div class="comment-bubble">
                    <div class="comment-avatar">
                        <img src="https://www.gravatar.com/avatar/c5d59b67e7a7eb5b7e5e1e5e2e5e3e5e?s=32&d=identicon&r=pg" 
                             alt="Avatar" style="width: 32px; height: 32px; border-radius: 50%;" />
                    </div>
                    <div class="comment-content">
                        <div class="comment-header">
                            <span class="comment-author">{"John Doe"}</span>
                            <span class="comment-time">{"2 hours ago"}</span>
                        </div>
                        <div class="comment-text">
                            <p>{"Great post! Really helpful content."}</p>
                        </div>
                    </div>
                </div>
                <div class="comment-form-preview">
                    <div style="padding: 8px; background: #f5f5f5; border-radius: 8px; font-size: 12px; text-align: center; color: #666;">
                        {"💬 Comment Form (Login Required)"}
                    </div>
                </div>
            </div>
        },
        "header" => html! {
            <div class="live-header-preview">
                <div class="live-nav-bar">
                    <span class="live-logo">{"🏠 My Site"}</span>
                    <div class="live-nav-items">
                        {
                            if (*header_navigation).is_empty() {
                                html! {
                                    <>
                                        <span>{"Home"}</span>
                                        <span>{"Posts"}</span>
                                        <span>{"About"}</span>
                                    </>
                                }
                            } else {
                                html! {
                                    <>
                                        {for (*header_navigation).iter().take(4).map(|item| {
                                            html! { <span>{&item.title}</span> }
                                        })}
                                    </>
                                }
                            }
                        }
                    </div>
                </div>
            </div>
        },
        "footer" => html! {
            <div class="live-footer-preview">
                <div class="live-footer-content">
                    <div class="live-footer-nav">
                        {
                            if (*footer_navigation).is_empty() {
                                html! {
                                    <>
                                        <span>{"Privacy"}</span>
                                        <span>{"Terms"}</span>
                                        <span>{"Contact"}</span>
                                    </>
                                }
                            } else {
                                html! {
                                    <>
                                        {for (*footer_navigation).iter().take(4).map(|item| {
                                            html! { <span>{&item.title}</span> }
                                        })}
                                    </>
                                }
                            }
                        }
                    </div>
                    <p>{"© 2024 My Rust CMS - Built with Rust & Yew"}</p>
                </div>
            </div>
        },
        "sidebar" => html! {
            <div class="live-sidebar-preview">
                <div class="sidebar-header">{"📋 Sidebar"}</div>
                <div class="sidebar-items">
                    <div class="sidebar-item">{"Navigation"}</div>
                    <div class="sidebar-item">{"Recent Posts"}</div>
                    <div class="sidebar-item">{"Categories"}</div>
                    <div class="sidebar-item">{"Archives"}</div>
                </div>
            </div>
        },
        "modal" => html! {
            <div class="live-modal-preview">
                <div class="modal-backdrop">
                    <div class="modal-content">
                        <div class="modal-header">
                            <span>{"Modal Title"}</span>
                            <span class="modal-close">{"×"}</span>
                        </div>
                        <div class="modal-body">{"Content area"}</div>
                        <div class="modal-footer">
                            <button class="modal-btn">{"Cancel"}</button>
                            <button class="modal-btn primary">{"Confirm"}</button>
                        </div>
                    </div>
                </div>
            </div>
        },
        "main_container" => html! {
            <div class="live-container-preview">
                <div class="container-outline">
                    <div class="container-header">{"📦 Main Container"}</div>
                    <div class="container-content">
                        <div class="content-block">{"Header"}</div>
                        <div class="content-block main">{"Main Content Area"}</div>
                        <div class="content-block">{"Footer"}</div>
                    </div>
                    <div class="container-info">{"Max-width: 1200px"}</div>
                </div>
            </div>
        },
        _ => html! {
            <div class="generic-component-preview">
                <div class="preview-placeholder">
                    <span>{"Component Preview"}</span>
                </div>
            </div>
        }
    }
}

fn get_component_title(component_type: &str) -> &'static str {
    match component_type {
        "header" => "🎯 Header Component",
        "footer" => "📍 Footer Component", 
        "sidebar" => "📋 Sidebar Component",
        "modal" => "🪟 Modal Component",
        "main_container" => "📦 Container Component",
        "Comments" => "💬 Comments Component",
        _ => "🧩 Component Template"
    }
}

fn get_component_description(component_type: &str) -> &'static str {
    match component_type {
        "header" => "Main site header with navigation, logo, and mobile responsive design",
        "footer" => "Site footer with navigation links, copyright, and customizable layout options",
        "sidebar" => "Configurable sidebar for additional navigation, widgets, and content areas",
        "modal" => "Overlay modals for forms, dialogs, and interactive content with backdrop styling",
        "main_container" => "Main content container with width, padding, and responsive layout settings",
        "Comments" => "Interactive comment system with Gravatar avatars, text message styling, and authentication",
        _ => "Customizable component template with configurable properties"
    }
}

fn get_component_name(component_type: &str) -> &'static str {
    match component_type {
        "header" => "Header",
        "footer" => "Footer",
        "sidebar" => "Sidebar",
        "modal" => "Modal",
        "main_container" => "Container",
        "Comments" => "Comments",
        _ => "Component"
    }
}

fn render_component_tags(component_type: &str) -> Html {
    match component_type {
        "header" => html! {
            <>
                <span class="property-tag">{"Sticky Position"}</span>
                <span class="property-tag">{"1200px Max"}</span>
                <span class="property-tag">{"Mobile Responsive"}</span>
            </>
        },
        "footer" => html! {
            <>
                <span class="property-tag">{"Full Width"}</span>
                <span class="property-tag">{"Horizontal Layout"}</span>
                <span class="property-tag">{"Responsive"}</span>
            </>
        },
        "Comments" => html! {
            <>
                <span class="property-tag">{"Gravatar Avatars"}</span>
                <span class="property-tag">{"Text Message Style"}</span>
                <span class="property-tag">{"Authentication"}</span>
            </>
        },
        "sidebar" => html! {
            <>
                <span class="property-tag">{"250px Width"}</span>
                <span class="property-tag">{"Left/Right Position"}</span>
                <span class="property-tag">{"Collapsible"}</span>
            </>
        },
        "modal" => html! {
            <>
                <span class="property-tag">{"Center Positioned"}</span>
                <span class="property-tag">{"Backdrop Blur"}</span>
                <span class="property-tag">{"Animation"}</span>
            </>
        },
        "main_container" => html! {
            <>
                <span class="property-tag">{"Max Width"}</span>
                <span class="property-tag">{"Auto Margins"}</span>
                <span class="property-tag">{"Responsive"}</span>
            </>
        },
        _ => html! {
            <>
                <span class="property-tag">{"Configurable"}</span>
                <span class="property-tag">{"Custom Layout"}</span>
            </>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ComponentTemplatesViewProps {
    pub component_templates: Vec<ComponentTemplate>,
    pub on_modify: Callback<(String, String)>, // (component_id, property)
    pub on_template_toggled: Callback<ComponentTemplate>, // Called when template is toggled
}

#[function_component(ComponentTemplatesView)]
pub fn component_templates_view(props: &ComponentTemplatesViewProps) -> Html {
    let editing_component = use_state(|| None::<String>);
    let editing_template = use_state(|| None::<ComponentTemplate>);
    let saving = use_state(|| false);
    let save_error = use_state(|| None::<String>);

    
    // Navigation items for live previews
    let header_navigation = use_state(Vec::<NavigationItem>::new);
    let footer_navigation = use_state(Vec::<NavigationItem>::new);

    // Load navigation items for previews
    {
        let header_navigation = header_navigation.clone();
        let footer_navigation = footer_navigation.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // Load header navigation
                if let Ok(header_items) = get_navigation_by_area("header").await {
                    header_navigation.set(header_items);
                }
                
                // Load footer navigation  
                if let Ok(footer_items) = get_navigation_by_area("footer").await {
                    footer_navigation.set(footer_items);
                }
            });
            || ()
        }, ());
    }

    let close_editor = {
        let editing_component = editing_component.clone();
        let editing_template = editing_template.clone();
        let save_error = save_error.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            editing_component.set(None);
            editing_template.set(None);
            save_error.set(None);
        })
    };

    let save_template = {
        let editing_template = editing_template.clone();
        let saving = saving.clone();
        let save_error = save_error.clone();
        let close_editor = close_editor.clone();
        Callback::from(move |_| {
            if let Some(template) = (*editing_template).clone() {
                saving.set(true);
                save_error.set(None);
                let saving_clone = saving.clone();
                let save_error_clone = save_error.clone();
                let close_editor_clone = close_editor.clone();
                
                wasm_bindgen_futures::spawn_local(async move {
                    match update_component_template(template.id, &template).await {
                        Ok(_) => {
                            web_sys::console::log_1(&"Component template saved successfully".into());
                            saving_clone.set(false);
                            // Close editor programmatically - create a dummy mouse event
                            if let Some(_window) = web_sys::window() {
                                if let Ok(event) = web_sys::MouseEvent::new("click") {
                                    close_editor_clone.emit(event);
                                }
                            }
                        }
                        Err(e) => {
                            save_error_clone.set(Some(format!("Failed to save template: {:?}", e)));
                            saving_clone.set(false);
                        }
                    }
                });
            }
        })
    };

    // Helper function to update template data
    let update_template_data = {
        let editing_template = editing_template.clone();
        Callback::from(move |(key, value): (String, serde_json::Value)| {
            if let Some(mut template) = (*editing_template).clone() {
                if let Some(data) = template.template_data.as_object_mut() {
                    data.insert(key, value);
                    editing_template.set(Some(template));
                }
            }
        })
    };



    html! {
        <div class="component-templates-section">
            <h2>{"Component Templates"}</h2>
            <p>{"Manage templates for major layout components with live styling"}</p>
            
            {if let Some(ref component_id) = *editing_component {
                html! {
                    <div class="editor-modal">
                        <div class="editor-overlay" onclick={close_editor.clone()}></div>
                        <div class="editor-panel">
                            <div class="editor-header">
                                <h3>{format!("Edit {} Component", component_id.replace("_", " "))}</h3>
                                <button class="close-btn" onclick={close_editor.clone()}>{"×"}</button>
                            </div>
                            <div class="editor-content">
                                <div class="editor-sidebar">
                                    {match component_id.as_str() {
                                        "header" => html! {
                                            <>
                                                <div class="property-group">
                                                    <h4>{"Header Layout"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Position"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("position".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <option value="static">{"Static"}</option>
                                                            <option value="sticky" selected=true>{"Sticky"}</option>
                                                            <option value="fixed">{"Fixed"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Container Width"}</label>
                                                        <select class="property-select">
                                                            <option value="full">{"Full Width"}</option>
                                                            <option value="contained" selected=true>{"Container (1200px)"}</option>
                                                            <option value="fluid">{"Fluid"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Height"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("height")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("80px")
                                                                        .to_string()
                                                                } else {
                                                                    "80px".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("height".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                            onblur={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: FocusEvent| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("height".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Z-Index"}</label>
                                                        <input type="number" value="1000" class="property-input" />
                                                    </div>
                                                </div>
                                                
                                                <div class="property-group">
                                                    <h4>{"Navigation Style"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Navigation Layout"}</label>
                                                        <select class="property-select">
                                                            <option value="horizontal" selected=true>{"Horizontal"}</option>
                                                            <option value="centered">{"Centered"}</option>
                                                            <option value="split">{"Logo Left / Menu Right"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Item Spacing"}</label>
                                                        <input type="text" value="2rem" class="property-input" />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Hover Effect"}</label>
                                                        <select class="property-select">
                                                            <option value="underline" selected=true>{"Underline"}</option>
                                                            <option value="background">{"Background Color"}</option>
                                                            <option value="scale">{"Scale"}</option>
                                                            <option value="none">{"None"}</option>
                                                        </select>
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Colors & Styling"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Background Type"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("bg_type".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <option value="color" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("bg_type")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("color") == "color"
                                                                } else {
                                                                    true
                                                                }
                                                            }}>{"Color"}</option>
                                                            <option value="gradient" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("bg_type")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("color") == "gradient"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Gradient"}</option>
                                                            <option value="image" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("bg_type")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("color") == "image"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Image"}</option>
                                                            <option value="video" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("bg_type")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("color") == "video"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Video"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Background Color"}</label>
                                                        <input 
                                                            type="color" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("bg_color")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("#000000")
                                                                        .to_string()
                                                                } else {
                                                                    "#000000".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("bg_color".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    
                                                    {if let Some(template) = (*editing_template).as_ref() {
                                                        let bg_type = template.template_data.get("bg_type")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("color");
                                                        
                                                        if bg_type == "gradient" {
                                                            html! {
                                                                <>
                                                                    <div class="property-item">
                                                                        <label>{"Gradient Start Color"}</label>
                                                                        <input 
                                                                            type="color" 
                                                                            value={{
                                                                                template.template_data.get("bg_gradient_start")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("#667eea")
                                                                                    .to_string()
                                                                            }}
                                                                            class="property-input"
                                                                            onchange={{
                                                                                let update_template_data = update_template_data.clone();
                                                                                Callback::from(move |e: Event| {
                                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                        let value = target.value();
                                                                                        update_template_data.emit(("bg_gradient_start".to_string(), serde_json::Value::String(value)));
                                                                                    }
                                                                                })
                                                                            }}
                                                                        />
                                                                    </div>
                                                                    <div class="property-item">
                                                                        <label>{"Gradient End Color"}</label>
                                                                        <input 
                                                                            type="color" 
                                                                            value={{
                                                                                template.template_data.get("bg_gradient_end")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("#764ba2")
                                                                                    .to_string()
                                                                            }}
                                                                            class="property-input"
                                                                            onchange={{
                                                                                let update_template_data = update_template_data.clone();
                                                                                Callback::from(move |e: Event| {
                                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                        let value = target.value();
                                                                                        update_template_data.emit(("bg_gradient_end".to_string(), serde_json::Value::String(value)));
                                                                                    }
                                                                                })
                                                                            }}
                                                                        />
                                                                    </div>
                                                                    <div class="property-item">
                                                                        <label>{"Gradient Direction"}</label>
                                                                        <select 
                                                                            class="property-select"
                                                                            onchange={{
                                                                                let update_template_data = update_template_data.clone();
                                                                                Callback::from(move |e: Event| {
                                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                                        let value = target.value();
                                                                                        update_template_data.emit(("bg_gradient_direction".to_string(), serde_json::Value::String(value)));
                                                                                    }
                                                                                })
                                                                            }}
                                                                        >
                                                                            <option value="to-right" selected={{
                                                                                template.template_data.get("bg_gradient_direction")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("to-right") == "to-right"
                                                                            }}>{"Left to Right"}</option>
                                                                            <option value="to-left" selected={{
                                                                                template.template_data.get("bg_gradient_direction")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("to-right") == "to-left"
                                                                            }}>{"Right to Left"}</option>
                                                                            <option value="to-bottom" selected={{
                                                                                template.template_data.get("bg_gradient_direction")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("to-right") == "to-bottom"
                                                                            }}>{"Top to Bottom"}</option>
                                                                            <option value="to-top" selected={{
                                                                                template.template_data.get("bg_gradient_direction")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("to-right") == "to-top"
                                                                            }}>{"Bottom to Top"}</option>
                                                                            <option value="135deg" selected={{
                                                                                template.template_data.get("bg_gradient_direction")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("to-right") == "135deg"
                                                                            }}>{"Diagonal (135°)"}</option>
                                                                        </select>
                                                                    </div>
                                                                    <div class="property-item">
                                                                        <label>{"Custom Gradient (CSS)"}</label>
                                                                        <input 
                                                                            type="text" 
                                                                            placeholder="e.g., linear-gradient(135deg, #667eea 0%, #764ba2 100%)"
                                                                            value={{
                                                                                template.template_data.get("bg_gradient_custom")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("")
                                                                                    .to_string()
                                                                            }}
                                                                            class="property-input"
                                                                            onchange={{
                                                                                let update_template_data = update_template_data.clone();
                                                                                Callback::from(move |e: Event| {
                                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                        let value = target.value();
                                                                                        update_template_data.emit(("bg_gradient_custom".to_string(), serde_json::Value::String(value)));
                                                                                    }
                                                                                })
                                                                            }}
                                                                        />
                                                                        <small style="color: #666; font-size: 12px;">{"Leave empty to use color pickers above"}</small>
                                                                    </div>
                                                                </>
                                                            }
                                                        } else if bg_type == "image" {
                                                            html! {
                                                                <div class="property-item">
                                                                    <label>{"Background Image URL"}</label>
                                                                    <input 
                                                                        type="text" 
                                                                        value={{
                                                                            template.template_data.get("bg_image")
                                                                                .and_then(|v| v.as_str())
                                                                                .unwrap_or("")
                                                                                .to_string()
                                                                        }}
                                                                        class="property-input"
                                                                        onchange={{
                                                                            let update_template_data = update_template_data.clone();
                                                                            Callback::from(move |e: Event| {
                                                                                if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                    let value = target.value();
                                                                                    update_template_data.emit(("bg_image".to_string(), serde_json::Value::String(value)));
                                                                                }
                                                                            })
                                                                        }}
                                                                    />
                                                                </div>
                                                            }
                                                        } else if bg_type == "video" {
                                                            html! {
                                                                <div class="property-item">
                                                                    <label>{"Background Video URL"}</label>
                                                                    <input 
                                                                        type="text" 
                                                                        value={{
                                                                            template.template_data.get("bg_video")
                                                                                .and_then(|v| v.as_str())
                                                                                .unwrap_or("")
                                                                                .to_string()
                                                                        }}
                                                                        class="property-input"
                                                                        onchange={{
                                                                            let update_template_data = update_template_data.clone();
                                                                            Callback::from(move |e: Event| {
                                                                                if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                    let value = target.value();
                                                                                    update_template_data.emit(("bg_video".to_string(), serde_json::Value::String(value)));
                                                                                }
                                                                            })
                                                                        }}
                                                                    />
                                                                </div>
                                                            }
                                                        } else {
                                                            html! {}
                                                        }
                                                    } else {
                                                        html! {}
                                                    }}
                                                    
                                                    <div class="property-item">
                                                        <label>{"Text Color"}</label>
                                                        <input 
                                                            type="color" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("text_color")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("#ffffff")
                                                                        .to_string()
                                                                } else {
                                                                    "#ffffff".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("text_color".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Text Hover Color"}</label>
                                                        <input 
                                                            type="color" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("text_hover_color")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("#f7fafc")
                                                                        .to_string()
                                                                } else {
                                                                    "#f7fafc".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("text_hover_color".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Logo Settings"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Logo Type"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("logo_type".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <option value="text" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("logo_type")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("text") == "text"
                                                                } else { true }
                                                            }}>{"Text Logo"}</option>
                                                            <option value="image" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("logo_type")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("text") == "image"
                                                                } else { false }
                                                            }}>{"Image Logo"}</option>
                                                            <option value="icon" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("logo_type")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("text") == "icon"
                                                                } else { false }
                                                            }}>{"Icon + Text"}</option>
                                                        </select>
                                                    </div>
                                                    
                                                    {if let Some(template) = (*editing_template).as_ref() {
                                                        let logo_type = template.template_data.get("logo_type")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("text");
                                                        
                                                        if logo_type == "image" {
                                                            html! {
                                                                <>
                                                                    <div class="property-item">
                                                                        <label>{"Logo Image URL"}</label>
                                                                        <div style="display: flex; gap: 8px;">
                                                                            <input 
                                                                                type="text" 
                                                                                value={{
                                                                                    template.template_data.get("logo_url")
                                                                                        .and_then(|v| v.as_str())
                                                                                        .unwrap_or("")
                                                                                        .to_string()
                                                                                }}
                                                                                class="property-input"
                                                                                style="flex: 1;"
                                                                                onchange={{
                                                                                    let update_template_data = update_template_data.clone();
                                                                                    Callback::from(move |e: Event| {
                                                                                        if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                            let value = target.value();
                                                                                            update_template_data.emit(("logo_url".to_string(), serde_json::Value::String(value)));
                                                                                        }
                                                                                    })
                                                                                }}
                                                                            />
                                                                            <button 
                                                                                class="btn-secondary"
                                                                                style="padding: 4px 8px; font-size: 12px;"
                                                                                onclick={{
                                                                                    Callback::from(move |_| {
                                                                                        // TODO: Open media library picker
                                                                                        web_sys::console::log_1(&"Open media library picker".into());
                                                                                    })
                                                                                }}
                                                                            >
                                                                                {"Browse"}
                                                                            </button>
                                                                        </div>
                                                                    </div>
                                                                    <div class="property-item">
                                                                        <label>{"Logo Height"}</label>
                                                                        <input 
                                                                            type="text" 
                                                                            value={{
                                                                                template.template_data.get("logo_height")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("40px")
                                                                                    .to_string()
                                                                            }}
                                                                            class="property-input"
                                                                            onchange={{
                                                                                let update_template_data = update_template_data.clone();
                                                                                Callback::from(move |e: Event| {
                                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                        let value = target.value();
                                                                                        update_template_data.emit(("logo_height".to_string(), serde_json::Value::String(value)));
                                                                                    }
                                                                                })
                                                                            }}
                                                                        />
                                                                    </div>
                                                                </>
                                                            }
                                                        } else {
                                                            html! {
                                                                <>
                                                                    <div class="property-item">
                                                                        <label>{"Logo Text"}</label>
                                                                        <input 
                                                                            type="text" 
                                                                            value={{
                                                                                template.template_data.get("logo_text")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("My Site")
                                                                                    .to_string()
                                                                            }}
                                                                            class="property-input"
                                                                            onchange={{
                                                                                let update_template_data = update_template_data.clone();
                                                                                Callback::from(move |e: Event| {
                                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                        let value = target.value();
                                                                                        update_template_data.emit(("logo_text".to_string(), serde_json::Value::String(value)));
                                                                                    }
                                                                                })
                                                                            }}
                                                                        />
                                                                    </div>
                                                                    <div class="property-item">
                                                                        <label>{"Logo Font Size"}</label>
                                                                        <input 
                                                                            type="text" 
                                                                            value={{
                                                                                template.template_data.get("logo_size")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("1.5rem")
                                                                                    .to_string()
                                                                            }}
                                                                            class="property-input"
                                                                            onchange={{
                                                                                let update_template_data = update_template_data.clone();
                                                                                Callback::from(move |e: Event| {
                                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                        let value = target.value();
                                                                                        update_template_data.emit(("logo_size".to_string(), serde_json::Value::String(value)));
                                                                                    }
                                                                                })
                                                                            }}
                                                                        />
                                                                    </div>
                                                                    <div class="property-item">
                                                                        <label>{"Logo Font Weight"}</label>
                                                                        <select 
                                                                            class="property-select"
                                                                            onchange={{
                                                                                let update_template_data = update_template_data.clone();
                                                                                Callback::from(move |e: Event| {
                                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                                        let value = target.value();
                                                                                        update_template_data.emit(("logo_font_weight".to_string(), serde_json::Value::String(value)));
                                                                                    }
                                                                                })
                                                                            }}
                                                                        >
                                                                            <option value="400" selected={{
                                                                                template.template_data.get("logo_font_weight")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("600") == "400"
                                                                            }}>{"Normal"}</option>
                                                                            <option value="600" selected={{
                                                                                template.template_data.get("logo_font_weight")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("600") == "600"
                                                                            }}>{"Semi-Bold"}</option>
                                                                            <option value="700" selected={{
                                                                                template.template_data.get("logo_font_weight")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("600") == "700"
                                                                            }}>{"Bold"}</option>
                                                                        </select>
                                                                    </div>
                                                                </>
                                                            }
                                                        }
                                                    } else {
                                                        html! {}
                                                    }}
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Navigation Properties"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Navigation Hover Color"}</label>
                                                        <input 
                                                            type="color" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("nav_hover_color")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("#f7fafc")
                                                                        .to_string()
                                                                } else {
                                                                    "#f7fafc".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("nav_hover_color".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Underline Color"}</label>
                                                        <input 
                                                            type="color" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("nav_underline_color")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("#ffffff")
                                                                        .to_string()
                                                                } else {
                                                                    "#ffffff".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("nav_underline_color".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Underline Thickness"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("nav_underline_thickness")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("2px")
                                                                        .to_string()
                                                                } else {
                                                                    "2px".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("nav_underline_thickness".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Underline Animation"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("nav_underline_animation".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <option value="none" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("nav_underline_animation")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "none"
                                                                } else {
                                                                    true
                                                                }
                                                            }}>{"None"}</option>
                                                            <option value="slide" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("nav_underline_animation")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "slide"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Slide"}</option>
                                                            <option value="fade" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("nav_underline_animation")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "fade"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Fade"}</option>
                                                        </select>
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Button Properties"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Primary Button Background"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("button_primary_bg")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("linear-gradient(135deg, rgba(255,255,255,0.2), rgba(255,255,255,0.1))")
                                                                        .to_string()
                                                                } else {
                                                                    "linear-gradient(135deg, rgba(255,255,255,0.2), rgba(255,255,255,0.1))".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("button_primary_bg".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Primary Button Hover Background"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("button_primary_hover_bg")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("linear-gradient(135deg, rgba(255,255,255,0.3), rgba(255,255,255,0.2))")
                                                                        .to_string()
                                                                } else {
                                                                    "linear-gradient(135deg, rgba(255,255,255,0.3), rgba(255,255,255,0.2))".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("button_primary_hover_bg".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Primary Button Text Color"}</label>
                                                        <input 
                                                            type="color" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("button_primary_text")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("#ffffff")
                                                                        .to_string()
                                                                } else {
                                                                    "#ffffff".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("button_primary_text".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Scroll Effects"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Scroll Effect"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("scroll_effect".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <option value="none" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("scroll_effect")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "none"
                                                                } else {
                                                                    true
                                                                }
                                                            }}>{"None"}</option>
                                                            <option value="shrink" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("scroll_effect")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "shrink"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Shrink"}</option>
                                                            <option value="fade" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("scroll_effect")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "fade"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Fade"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Animation Duration (ms)"}</label>
                                                        <input 
                                                            type="range" 
                                                            min="100" 
                                                            max="2500" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("scroll_duration")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("300")
                                                                        .to_string()
                                                                } else {
                                                                    "300".to_string()
                                                                }
                                                            }}
                                                            class="property-slider"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("scroll_duration".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                        <span class="slider-value">{{
                                                            if let Some(template) = (*editing_template).as_ref() {
                                                                format!("{}ms", template.template_data.get("scroll_duration")
                                                                    .and_then(|v| v.as_str())
                                                                    .unwrap_or("300"))
                                                            } else {
                                                                "300ms".to_string()
                                                            }
                                                        }}</span>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Logo Scale (%)"}</label>
                                                        <input 
                                                            type="range" 
                                                            min="10" 
                                                            max="100" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shrink_logo_scale")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("80")
                                                                        .to_string()
                                                                } else {
                                                                    "80".to_string()
                                                                }
                                                            }}
                                                            class="property-slider"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("shrink_logo_scale".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                        <span class="slider-value">{{
                                                            if let Some(template) = (*editing_template).as_ref() {
                                                                format!("{}%", template.template_data.get("shrink_logo_scale")
                                                                    .and_then(|v| v.as_str())
                                                                    .unwrap_or("80"))
                                                            } else {
                                                                "80%".to_string()
                                                            }
                                                        }}</span>
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Mobile Behavior"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Mobile Menu Style"}</label>
                                                        <select class="property-select">
                                                            <option value="hamburger" selected=true>{"Hamburger Menu"}</option>
                                                            <option value="dots">{"Three Dots"}</option>
                                                            <option value="hidden">{"Hide Menu"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Mobile Breakpoint"}</label>
                                                        <input type="text" value="768px" class="property-input" />
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Effects"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Visual Effects"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("effects".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <option value="none" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("effects")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "none"
                                                                } else { true }
                                                            }}>{"None"}</option>
                                                            <option value="glassmorphism" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("effects")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "glassmorphism"
                                                                } else { false }
                                                            }}>{"Glassmorphism"}</option>
                                                            <option value="neumorphism" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("effects")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "neumorphism"
                                                                } else { false }
                                                            }}>{"Neumorphism"}</option>
                                                            <option value="claymorphism" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("effects")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "claymorphism"
                                                                } else { false }
                                                            }}>{"Claymorphism"}</option>
                                                            <option value="cybermorphism" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("effects")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "cybermorphism"
                                                                } else { false }
                                                            }}>{"Cybermorphism"}</option>
                                                        </select>
                                                    </div>
                                                    
                                                    {if let Some(template) = (*editing_template).as_ref() {
                                                        let effects = template.template_data.get("effects").and_then(|v| v.as_str()).unwrap_or("none");
                                                        let effects_intensity = template.template_data.get("effects_intensity").and_then(|v| v.as_str()).unwrap_or("50");
                                                        if effects != "none" {
                                                            html! {
                                                                <div class="property-item">
                                                                    <label>{"Multiply Intensity (%)"}</label>
                                                                    <input 
                                                                        type="range" 
                                                                        min="0" 
                                                                        max="100" 
                                                                        value={effects_intensity.to_string()}
                                                                        class="property-range"
                                                                        onchange={{
                                                                            let update_template_data = update_template_data.clone();
                                                                            Callback::from(move |e: Event| {
                                                                                if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                    let value = target.value();
                                                                                    update_template_data.emit(("effects_intensity".to_string(), serde_json::Value::String(value)));
                                                                                }
                                                                            })
                                                                        }}
                                                                    />
                                                                    <span class="range-value">{format!("{}%", effects_intensity)}</span>
                                                                </div>
                                                            }
                                                        } else { html! {} }
                                                    } else { html! {} }}
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Shape Masks"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Upper Shape"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("shape_mask_upper".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <option value="none" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_upper")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "none"
                                                                } else { true }
                                                            }}>{"None"}</option>
                                                            <option value="wave" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_upper")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "wave"
                                                                } else { false }
                                                            }}>{"Wave"}</option>
                                                            <option value="curve" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_upper")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "curve"
                                                                } else { false }
                                                            }}>{"Curve"}</option>
                                                            <option value="triangle" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_upper")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "triangle"
                                                                } else { false }
                                                            }}>{"Triangle"}</option>
                                                            <option value="tilt" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_upper")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "tilt"
                                                                } else { false }
                                                            }}>{"Tilt"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Lower Shape"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("shape_mask_lower".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <option value="none" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_lower")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "none"
                                                                } else { true }
                                                            }}>{"None"}</option>
                                                            <option value="wave" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_lower")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "wave"
                                                                } else { false }
                                                            }}>{"Wave"}</option>
                                                            <option value="curve" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_lower")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "curve"
                                                                } else { false }
                                                            }}>{"Curve"}</option>
                                                            <option value="triangle" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_lower")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "triangle"
                                                                } else { false }
                                                            }}>{"Triangle"}</option>
                                                            <option value="tilt" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_lower")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "tilt"
                                                                } else { false }
                                                            }}>{"Tilt"}</option>
                                                        </select>
                                                    </div>
                                                </div>
                                            </>
                                        },
                                        "footer" => html! {
                                            <>
                                                <div class="property-group">
                                                    <h4>{"Basic Properties"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Background Color"}</label>
                                                        <input 
                                                            type="color" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("bg_color")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("#000000")
                                                                        .to_string()
                                                                } else {
                                                                    "#000000".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("bg_color".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Text Color"}</label>
                                                        <input 
                                                            type="color" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("text_color")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("#ffffff")
                                                                        .to_string()
                                                                } else {
                                                                    "#ffffff".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("text_color".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Muted Text Color"}</label>
                                                        <input 
                                                            type="color" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("text_muted")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("#e2e8f0")
                                                                        .to_string()
                                                                } else {
                                                                    "#e2e8f0".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("text_muted".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Height"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("height")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("auto")
                                                                        .to_string()
                                                                } else {
                                                                    "auto".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("height".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Effects"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Visual Effects"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("effects".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <option value="none" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("effects")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "none"
                                                                } else {
                                                                    true
                                                                }
                                                            }}>{"None"}</option>
                                                            <option value="glassmorphism" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("effects")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "glassmorphism"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Glassmorphism"}</option>
                                                            <option value="neumorphism" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("effects")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "neumorphism"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Neumorphism"}</option>
                                                            <option value="claymorphism" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("effects")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "claymorphism"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Claymorphism"}</option>
                                                            <option value="cybermorphism" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("effects")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "cybermorphism"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Cybermorphism"}</option>
                                                        </select>
                                                    </div>
                                                    
                                                    {if let Some(template) = (*editing_template).as_ref() {
                                                        let effects = template.template_data.get("effects")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("none");
                                                        
                                                        if effects != "none" {
                                                            html! {
                                                                <div class="property-item">
                                                                    <label>{"Effects Intensity (%)"}</label>
                                                                    <input 
                                                                        type="range" 
                                                                        min="0" 
                                                                        max="100" 
                                                                        value={{
                                                                            template.template_data.get("effects_intensity")
                                                                                .and_then(|v| v.as_str())
                                                                                .unwrap_or("50")
                                                                                .to_string()
                                                                        }}
                                                                        class="property-range"
                                                                        onchange={{
                                                                            let update_template_data = update_template_data.clone();
                                                                            Callback::from(move |e: Event| {
                                                                                if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                    let value = target.value();
                                                                                    update_template_data.emit(("effects_intensity".to_string(), serde_json::Value::String(value)));
                                                                                }
                                                                            })
                                                                        }}
                                                                    />
                                                                </div>
                                                            }
                                                        } else {
                                                            html! {}
                                                        }
                                                    } else {
                                                        html! {}
                                                    }}
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Shape Masks"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Upper Shape"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("shape_mask_upper".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <option value="none" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_upper")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "none"
                                                                } else {
                                                                    true
                                                                }
                                                            }}>{"None"}</option>
                                                            <option value="wave" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_upper")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "wave"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Wave"}</option>
                                                            <option value="curve" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_upper")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "curve"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Curve"}</option>
                                                            <option value="triangle" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_upper")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "triangle"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Triangle"}</option>
                                                            <option value="tilt" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_upper")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "tilt"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Tilt"}</option>
                                                            <option value="zigzag" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_upper")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "zigzag"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Zigzag"}</option>
                                                        </select>
                                                    </div>
                                                    
                                                    {if let Some(template) = (*editing_template).as_ref() {
                                                        let shape_mask_upper = template.template_data.get("shape_mask_upper")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("none");
                                                        
                                                        if shape_mask_upper != "none" {
                                                            html! {
                                                                <div class="property-item">
                                                                    <label>{"Upper Scale (%)"}</label>
                                                                    <input 
                                                                        type="range" 
                                                                        min="10" 
                                                                        max="200" 
                                                                        value={{
                                                                            template.template_data.get("shape_mask_upper_scale")
                                                                                .and_then(|v| v.as_str())
                                                                                .unwrap_or("100")
                                                                                .to_string()
                                                                        }}
                                                                        class="property-range"
                                                                        onchange={{
                                                                            let update_template_data = update_template_data.clone();
                                                                            Callback::from(move |e: Event| {
                                                                                if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                    let value = target.value();
                                                                                    update_template_data.emit(("shape_mask_upper_scale".to_string(), serde_json::Value::String(value)));
                                                                                }
                                                                            })
                                                                        }}
                                                                    />
                                                                </div>
                                                            }
                                                        } else {
                                                            html! {}
                                                        }
                                                    } else {
                                                        html! {}
                                                    }}

                                                    <div class="property-item">
                                                        <label>{"Lower Shape"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("shape_mask_lower".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <option value="none" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_lower")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "none"
                                                                } else {
                                                                    true
                                                                }
                                                            }}>{"None"}</option>
                                                            <option value="wave" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_lower")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "wave"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Wave"}</option>
                                                            <option value="curve" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_lower")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "curve"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Curve"}</option>
                                                            <option value="triangle" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_lower")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "triangle"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Triangle"}</option>
                                                            <option value="tilt" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_lower")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "tilt"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Tilt"}</option>
                                                            <option value="zigzag" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("shape_mask_lower")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "zigzag"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Zigzag"}</option>
                                                        </select>
                                                    </div>
                                                    
                                                    {if let Some(template) = (*editing_template).as_ref() {
                                                        let shape_mask_lower = template.template_data.get("shape_mask_lower")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("none");
                                                        
                                                        if shape_mask_lower != "none" {
                                                            html! {
                                                                <div class="property-item">
                                                                    <label>{"Lower Scale (%)"}</label>
                                                                    <input 
                                                                        type="range" 
                                                                        min="10" 
                                                                        max="200" 
                                                                        value={{
                                                                            template.template_data.get("shape_mask_lower_scale")
                                                                                .and_then(|v| v.as_str())
                                                                                .unwrap_or("100")
                                                                                .to_string()
                                                                        }}
                                                                        class="property-range"
                                                                        onchange={{
                                                                            let update_template_data = update_template_data.clone();
                                                                            Callback::from(move |e: Event| {
                                                                                if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                    let value = target.value();
                                                                                    update_template_data.emit(("shape_mask_lower_scale".to_string(), serde_json::Value::String(value)));
                                                                                }
                                                                            })
                                                                        }}
                                                                    />
                                                                </div>
                                                            }
                                                        } else {
                                                            html! {}
                                                        }
                                                    } else {
                                                        html! {}
                                                    }}
                                                </div>
                                            </>
                                        },
                                        "sidebar" => html! {
                                            <>
                                                <div class="property-group">
                                                    <h4>{"Sidebar Layout"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Position"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("position".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <>
                                                                    <option value="left" selected={
                                                                        (*editing_template)
                                                                            .as_ref()
                                                                            .and_then(|t| t.template_data.get("position").and_then(|v| v.as_str()))
                                                                            .unwrap_or("right") == "left"
                                                                    }>{"Left Side"}</option>
                                                                    <option value="right" selected={
                                                                        (*editing_template)
                                                                            .as_ref()
                                                                            .and_then(|t| t.template_data.get("position").and_then(|v| v.as_str()))
                                                                            .unwrap_or("right") == "right"
                                                                    }>{"Right Side"}</option>
                                                                    <option value="both" selected={
                                                                        (*editing_template)
                                                                            .as_ref()
                                                                            .and_then(|t| t.template_data.get("position").and_then(|v| v.as_str()))
                                                                            .unwrap_or("right") == "both"
                                                                    }>{"Both Sides"}</option>
                                                            </>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Width"}</label>
                                                        <input 
                                                            type="text" 
                                                            class="property-input"
                                                            value={{
                                                                if let Some(t) = (*editing_template).as_ref() {
                                                                    t.template_data.get("width").and_then(|v| v.as_str()).unwrap_or("300px").to_string()
                                                                } else { "300px".to_string() }
                                                            }}
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("width".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                            onblur={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: FocusEvent| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("width".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Sticky"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        let is_sticky = value == "true";
                                                                        update_template_data.emit(("sticky".to_string(), serde_json::Value::Bool(is_sticky)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <>
                                                                    <option value="true" selected={
                                                                        (*editing_template)
                                                                            .as_ref()
                                                                            .and_then(|t| t.template_data.get("sticky").and_then(|v| v.as_bool()))
                                                                            .unwrap_or(true)
                                                                    }>{"Sticky"}</option>
                                                                    <option value="false" selected={
                                                                        !((*editing_template)
                                                                            .as_ref()
                                                                            .and_then(|t| t.template_data.get("sticky").and_then(|v| v.as_bool()))
                                                                            .unwrap_or(true))
                                                                    }>{"Not Sticky"}</option>
                                                            </>
                                                        </select>
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Mobile Behavior"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Mobile Display"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("mobile_display".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <>
                                                                    <option value="hidden" selected={
                                                                        (*editing_template)
                                                                            .as_ref()
                                                                            .and_then(|t| t.template_data.get("mobile_display").and_then(|v| v.as_str()))
                                                                            .unwrap_or("hidden") == "hidden"
                                                                    }>{"Hidden"}</option>
                                                                    <option value="bottom" selected={
                                                                        (*editing_template)
                                                                            .as_ref()
                                                                            .and_then(|t| t.template_data.get("mobile_display").and_then(|v| v.as_str()))
                                                                            .unwrap_or("hidden") == "bottom"
                                                                    }>{"Move to Bottom"}</option>
                                                                    <option value="drawer" selected={
                                                                        (*editing_template)
                                                                            .as_ref()
                                                                            .and_then(|t| t.template_data.get("mobile_display").and_then(|v| v.as_str()))
                                                                            .unwrap_or("hidden") == "drawer"
                                                                    }>{"Slide-out Drawer"}</option>
                                                                    <option value="accordion" selected={
                                                                        (*editing_template)
                                                                            .as_ref()
                                                                            .and_then(|t| t.template_data.get("mobile_display").and_then(|v| v.as_str()))
                                                                            .unwrap_or("hidden") == "accordion"
                                                                    }>{"Collapsible"}</option>
                                                            </>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Mobile Breakpoint"}</label>
                                                        <input 
                                                            type="text" 
                                                            class="property-input"
                                                            value={{
                                                                if let Some(t) = (*editing_template).as_ref() {
                                                                    t.template_data.get("mobile_breakpoint").and_then(|v| v.as_str()).unwrap_or("768px").to_string()
                                                                } else { "768px".to_string() }
                                                            }}
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("mobile_breakpoint".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Content Sections"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Default Sections"}</label>
                                                        <div class="checkbox-list">
                                                            <label class="checkbox-item">
                                                                <input type="checkbox" disabled=true checked=true />
                                                                <span>{"Navigation Links"}</span>
                                                            </label>
                                                            <label class="checkbox-item">
                                                                <input type="checkbox" disabled=true checked=true />
                                                                <span>{"Recent Posts"}</span>
                                                            </label>
                                                            <label class="checkbox-item">
                                                                <input type="checkbox" disabled=true />
                                                                <span>{"Categories"}</span>
                                                            </label>
                                                            <label class="checkbox-item">
                                                                <input type="checkbox" disabled=true />
                                                                <span>{"Archives"}</span>
                                                            </label>
                                                        </div>
                                                    </div>
                                                </div>
                                            </>
                                        },
                                        "modal" => html! {
                                            <>
                                                <div class="property-group">
                                                    <h4>{"Modal Behavior"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Animation Style"}</label>
                                                        <select class="property-select">
                                                            <option value="fade" selected=true>{"Fade In/Out"}</option>
                                                            <option value="scale">{"Scale In/Out"}</option>
                                                            <option value="slide-down">{"Slide Down"}</option>
                                                            <option value="slide-up">{"Slide Up"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Animation Duration"}</label>
                                                        <input type="text" value="300ms" class="property-input" />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Close on Backdrop Click"}</label>
                                                        <select class="property-select">
                                                            <option value="true" selected=true>{"Yes"}</option>
                                                            <option value="false">{"No"}</option>
                                                        </select>
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Modal Sizing"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Default Size"}</label>
                                                        <select class="property-select">
                                                            <option value="small">{"Small (400px)"}</option>
                                                            <option value="medium" selected=true>{"Medium (600px)"}</option>
                                                            <option value="large">{"Large (800px)"}</option>
                                                            <option value="xl">{"Extra Large (1000px)"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Max Height"}</label>
                                                        <input type="text" value="90vh" class="property-input" />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Mobile Behavior"}</label>
                                                        <select class="property-select">
                                                            <option value="responsive" selected=true>{"Responsive"}</option>
                                                            <option value="fullscreen">{"Full Screen"}</option>
                                                            <option value="bottom-sheet">{"Bottom Sheet"}</option>
                                                        </select>
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Backdrop & Overlay"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Backdrop Blur"}</label>
                                                        <input type="range" min="0" max="10" value="4" class="property-slider" />
                                                        <span class="slider-value">{"4px"}</span>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Backdrop Opacity"}</label>
                                                        <input type="range" min="0" max="100" value="50" class="property-slider" />
                                                        <span class="slider-value">{"50%"}</span>
                                                    </div>
                                                </div>
                                            </>
                                        },
                                        "main_container" => html! {
                                            <>
                                                <div class="property-group">
                                                    <h4>{"Container Properties"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Width Type"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("width_type".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <option value="fixed" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("width_type")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("fixed") == "fixed"
                                                                } else {
                                                                    true
                                                                }
                                                            }}>{"Fixed Width"}</option>
                                                            <option value="fluid" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("width_type")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("fixed") == "fluid"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Fluid Width"}</option>
                                                            <option value="full" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("width_type")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("fixed") == "full"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Full Width"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Max Width"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("max_width")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("1200px")
                                                                        .to_string()
                                                                } else {
                                                                    "1200px".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("max_width".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Padding"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("padding")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("1rem")
                                                                        .to_string()
                                                                } else {
                                                                    "1rem".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("padding".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Margin"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("margin")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("0 auto")
                                                                        .to_string()
                                                                } else {
                                                                    "0 auto".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("margin".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Background & Media Overlay"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Background Type"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("bg_type".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <option value="color" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("bg_type")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("color") == "color"
                                                                } else {
                                                                    true
                                                                }
                                                            }}>{"Color"}</option>
                                                            <option value="image" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("bg_type")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("color") == "image"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Image"}</option>
                                                            <option value="gradient" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("bg_type")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("color") == "gradient"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Gradient"}</option>
                                                            <option value="video" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("bg_type")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("color") == "video"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Video"}</option>
                                                        </select>
                                                    </div>
                                                    
                                                    {if let Some(template) = (*editing_template).as_ref() {
                                                        let bg_type = template.template_data.get("bg_type")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("color");
                                                        
                                                        match bg_type {
                                                            "color" => html! {
                                                                <div class="property-item">
                                                                    <label>{"Background Color"}</label>
                                                                    <input 
                                                                        type="color" 
                                                                        value={{
                                                                            template.template_data.get("bg_color")
                                                                                .and_then(|v| v.as_str())
                                                                                .unwrap_or("#ffffff")
                                                                                .to_string()
                                                                        }}
                                                                        class="property-input"
                                                                        onchange={{
                                                                            let update_template_data = update_template_data.clone();
                                                                            Callback::from(move |e: Event| {
                                                                                if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                    let value = target.value();
                                                                                    update_template_data.emit(("bg_color".to_string(), serde_json::Value::String(value)));
                                                                                }
                                                                            })
                                                                        }}
                                                                    />
                                                                </div>
                                                            },
                                                            "image" => html! {
                                                                <div class="property-item">
                                                                    <label>{"Background Image URL"}</label>
                                                                    <input 
                                                                        type="text" 
                                                                        value={{
                                                                            template.template_data.get("bg_image")
                                                                                .and_then(|v| v.as_str())
                                                                                .unwrap_or("")
                                                                                .to_string()
                                                                        }}
                                                                        class="property-input"
                                                                        onchange={{
                                                                            let update_template_data = update_template_data.clone();
                                                                            Callback::from(move |e: Event| {
                                                                                if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                    let value = target.value();
                                                                                    update_template_data.emit(("bg_image".to_string(), serde_json::Value::String(value)));
                                                                                }
                                                                            })
                                                                        }}
                                                                    />
                                                                </div>
                                                            },
                                                            "gradient" => html! {
                                                                <>
                                                                    <div class="property-item">
                                                                        <label>{"Gradient Start Color"}</label>
                                                                        <input 
                                                                            type="color" 
                                                                            value={{
                                                                                template.template_data.get("bg_gradient_start")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("#667eea")
                                                                                    .to_string()
                                                                            }}
                                                                            class="property-input"
                                                                            onchange={{
                                                                                let update_template_data = update_template_data.clone();
                                                                                Callback::from(move |e: Event| {
                                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                        let value = target.value();
                                                                                        update_template_data.emit(("bg_gradient_start".to_string(), serde_json::Value::String(value)));
                                                                                    }
                                                                                })
                                                                            }}
                                                                        />
                                                                    </div>
                                                                    <div class="property-item">
                                                                        <label>{"Gradient End Color"}</label>
                                                                        <input 
                                                                            type="color" 
                                                                            value={{
                                                                                template.template_data.get("bg_gradient_end")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("#764ba2")
                                                                                    .to_string()
                                                                            }}
                                                                            class="property-input"
                                                                            onchange={{
                                                                                let update_template_data = update_template_data.clone();
                                                                                Callback::from(move |e: Event| {
                                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                        let value = target.value();
                                                                                        update_template_data.emit(("bg_gradient_end".to_string(), serde_json::Value::String(value)));
                                                                                    }
                                                                                })
                                                                            }}
                                                                        />
                                                                    </div>
                                                                    <div class="property-item">
                                                                        <label>{"Gradient Direction"}</label>
                                                                        <select 
                                                                            class="property-select"
                                                                            onchange={{
                                                                                let update_template_data = update_template_data.clone();
                                                                                Callback::from(move |e: Event| {
                                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                                        let value = target.value();
                                                                                        update_template_data.emit(("bg_gradient_direction".to_string(), serde_json::Value::String(value)));
                                                                                    }
                                                                                })
                                                                            }}
                                                                        >
                                                                            <option value="to-right" selected={{
                                                                                template.template_data.get("bg_gradient_direction")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("to-right") == "to-right"
                                                                            }}>{"Left to Right"}</option>
                                                                            <option value="to-left" selected={{
                                                                                template.template_data.get("bg_gradient_direction")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("to-right") == "to-left"
                                                                            }}>{"Right to Left"}</option>
                                                                            <option value="to-bottom" selected={{
                                                                                template.template_data.get("bg_gradient_direction")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("to-right") == "to-bottom"
                                                                            }}>{"Top to Bottom"}</option>
                                                                            <option value="to-top" selected={{
                                                                                template.template_data.get("bg_gradient_direction")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("to-right") == "to-top"
                                                                            }}>{"Bottom to Top"}</option>
                                                                            <option value="135deg" selected={{
                                                                                template.template_data.get("bg_gradient_direction")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("to-right") == "135deg"
                                                                            }}>{"Diagonal (135°)"}</option>
                                                                            <option value="45deg" selected={{
                                                                                template.template_data.get("bg_gradient_direction")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("to-right") == "45deg"
                                                                            }}>{"Diagonal (45°)"}</option>
                                                                        </select>
                                                                    </div>
                                                                    <div class="property-item">
                                                                        <label>{"Custom Gradient (CSS)"}</label>
                                                                        <input 
                                                                            type="text" 
                                                                            value={{
                                                                                template.template_data.get("bg_gradient_custom")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("")
                                                                                    .to_string()
                                                                            }}
                                                                            class="property-input"
                                                                            onchange={{
                                                                                let update_template_data = update_template_data.clone();
                                                                                Callback::from(move |e: Event| {
                                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                        let value = target.value();
                                                                                        update_template_data.emit(("bg_gradient_custom".to_string(), serde_json::Value::String(value)));
                                                                                    }
                                                                                })
                                                                            }}
                                                                        />
                                                                    </div>
                                                                </>
                                                            },
                                                            "video" => html! {
                                                                <div class="property-item">
                                                                    <label>{"Background Video URL"}</label>
                                                                    <input 
                                                                        type="text" 
                                                                        value={{
                                                                            template.template_data.get("bg_video")
                                                                                .and_then(|v| v.as_str())
                                                                                .unwrap_or("")
                                                                                .to_string()
                                                                        }}
                                                                        class="property-input"
                                                                        onchange={{
                                                                            let update_template_data = update_template_data.clone();
                                                                            Callback::from(move |e: Event| {
                                                                                if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                    let value = target.value();
                                                                                    update_template_data.emit(("bg_video".to_string(), serde_json::Value::String(value)));
                                                                                }
                                                                            })
                                                                        }}
                                                                    />
                                                                </div>
                                                            },
                                                            _ => html! {}
                                                        }
                                                    } else {
                                                        html! {}
                                                    }}
                                                    
                                                    <div class="property-item" style="margin-top: 12px; padding-top: 12px; border-top: 1px solid #eee;">
                                                        <h5 style="margin: 0 0 8px 0; font-size: 12px; color: #666; font-weight: 600;">{"Media Overlay"}</h5>
                                                        <label>{"Overlay Color"}</label>
                                                        <input 
                                                            type="color" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("overlay_color")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("#000000")
                                                                        .to_string()
                                                                } else {
                                                                    "#000000".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("overlay_color".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Overlay Opacity"}</label>
                                                        <input 
                                                            type="range" 
                                                            min="0" 
                                                            max="1" 
                                                            step="0.1"
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("overlay_opacity")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("0.3")
                                                                        .to_string()
                                                                } else {
                                                                    "0.3".to_string()
                                                                }
                                                            }}
                                                            class="property-range"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("overlay_opacity".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Animation"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Entrance Animation"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("animation".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <option value="none" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("animation")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "none"
                                                                } else {
                                                                    true
                                                                }
                                                            }}>{"None"}</option>
                                                            <option value="fade-in" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("animation")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "fade-in"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Fade In"}</option>
                                                            <option value="slide-up" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("animation")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "slide-up"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Slide Up"}</option>
                                                            <option value="slide-down" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("animation")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "slide-down"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Slide Down"}</option>
                                                            <option value="zoom-in" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("animation")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "zoom-in"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Zoom In"}</option>
                                                        </select>
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Effects"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Visual Effects"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("effects".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <option value="none" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("effects")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "none"
                                                                } else {
                                                                    true
                                                                }
                                                            }}>{"None"}</option>
                                                            <option value="glassmorphism" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("effects")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "glassmorphism"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Glassmorphism"}</option>
                                                            <option value="neumorphism" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("effects")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "neumorphism"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Neumorphism"}</option>
                                                            <option value="claymorphism" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("effects")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "claymorphism"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Claymorphism"}</option>
                                                            <option value="cybermorphism" selected={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("effects")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("none") == "cybermorphism"
                                                                } else {
                                                                    false
                                                                }
                                                            }}>{"Cybermorphism"}</option>
                                                        </select>
                                                    </div>
                                                    
                                                    {if let Some(template) = (*editing_template).as_ref() {
                                                        let effects = template.template_data.get("effects")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("none");
                                                        
                                                        if effects != "none" {
                                                            html! {
                                                                <div class="property-item">
                                                                    <label>{"Effects Intensity (%)"}</label>
                                                                    <input 
                                                                        type="range" 
                                                                        min="0" 
                                                                        max="100" 
                                                                        value={{
                                                                            template.template_data.get("effects_intensity")
                                                                                .and_then(|v| v.as_str())
                                                                                .unwrap_or("50")
                                                                                .to_string()
                                                                        }}
                                                                        class="property-range"
                                                                        onchange={{
                                                                            let update_template_data = update_template_data.clone();
                                                                            Callback::from(move |e: Event| {
                                                                                if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                                    let value = target.value();
                                                                                    update_template_data.emit(("effects_intensity".to_string(), serde_json::Value::String(value)));
                                                                                }
                                                                            })
                                                                        }}
                                                                    />
                                                                </div>
                                                            }
                                                        } else {
                                                            html! {}
                                                        }
                                                    } else {
                                                        html! {}
                                                    }}
                                                </div>
                                            </>
                                        },

                                        _ => html! {
                                            <div class="property-group">
                                                <h4>{"Component Properties"}</h4>
                                                <p>{"Select a component to customize its properties."}</p>
                                            </div>
                                        }
                                    }}
                                </div>
                                
                                <div class="editor-preview">
                                    <h4>{"Live Preview"}</h4>
                                    <div class="preview-container">
                                        {match component_id.as_str() {
                                            "header" => html! {
                                                <div class="preview-header">
                                                    <div class="preview-nav">
                                                        <span class="preview-logo">{"🏠 My Site"}</span>
                                                        <div class="preview-menu">
                                                            {
                                                                if (*header_navigation).is_empty() {
                                                                    html! {
                                                                        <>
                                                                            <span class="preview-item">{"Home"}</span>
                                                                            <span class="preview-item">{"Posts"}</span>
                                                                            <span class="preview-item">{"About"}</span>
                                                                        </>
                                                                    }
                                                                } else {
                                                                    html! {
                                                                        <>
                                                                            {for (*header_navigation).iter().take(5).map(|item| {
                                                                                html! { <span class="preview-item">{&item.title}</span> }
                                                                            })}
                                                                        </>
                                                                    }
                                                                }
                                                            }
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            "footer" => html! {
                                                <div class="preview-footer">
                                                    <div class="preview-footer-content">
                                                        <div class="preview-footer-nav">
                                                            {
                                                                if (*footer_navigation).is_empty() {
                                                                    html! {
                                                                        <>
                                                                            <span class="preview-footer-item">{"Privacy"}</span>
                                                                            <span class="preview-footer-item">{"Terms"}</span>
                                                                            <span class="preview-footer-item">{"Contact"}</span>
                                                                        </>
                                                                    }
                                                                } else {
                                                                    html! {
                                                                        <>
                                                                            {for (*footer_navigation).iter().take(4).map(|item| {
                                                                                html! { <span class="preview-footer-item">{&item.title}</span> }
                                                                            })}
                                                                        </>
                                                                    }
                                                                }
                                                            }
                                                        </div>
                                                        <p class="preview-footer-text">{"© 2024 My Rust CMS - Built with Rust & Yew"}</p>
                                                    </div>
                                                </div>
                                            },
                                            "main_container" => html! {
                                                <div class="preview-container-component">
                                                    <div class="container-preview-outline" style="border: 2px dashed #ccc; padding: 20px; text-align: center;">
                                                        <h4 style="margin: 0 0 10px 0; color: #666;">{"📦 Main Container"}</h4>
                                                        <div style="background: #f5f5f5; padding: 15px; border-radius: 4px; margin: 10px 0;">
                                                            <div style="background: #e0e0e0; padding: 8px; margin: 5px 0; border-radius: 2px;">{"Header Area"}</div>
                                                            <div style="background: #fff; padding: 20px; margin: 10px 0; border-radius: 2px; min-height: 60px;">{"Main Content Area"}</div>
                                                            <div style="background: #e0e0e0; padding: 8px; margin: 5px 0; border-radius: 2px;">{"Footer Area"}</div>
                                                        </div>
                                                        <small style="color: #999;">
                                                            {format!("Max Width: {}", 
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("max_width")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("1200px")
                                                                } else {
                                                                    "1200px"
                                                                }
                                                            )}
                                                        </small>
                                                    </div>
                                                </div>
                                            },
                                            _ => html! {
                                                <div class="preview-placeholder">
                                                    <p>{format!("{} Component Preview", component_id)}</p>
                                                </div>
                                            }
                                        }}
                                    </div>
                                </div>
                            </div>
                            
                            {if let Some(ref error_msg) = *save_error {
                                html! {
                                    <div class="error-message" style="margin: 1rem;">
                                        <strong>{"Error: "}</strong>{error_msg}
                                    </div>
                                }
                            } else {
                                html! {}
                            }}

                            <div class="editor-actions">
                                <button 
                                    class="btn-primary" 
                                    onclick={save_template.clone()}
                                    disabled={*saving}
                                >
                                    {if *saving { "Saving..." } else { "Save Changes" }}
                                </button>
                                <button class="btn-secondary">{"Reset to Default"}</button>
                                <button class="btn-secondary" onclick={close_editor.clone()}>{"Cancel"}</button>
                            </div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
            
            <div class="template-component-grid">
                {for props.component_templates.iter()
                    .filter(|template| template.component_type != "main_container")
                    .map(|template| {
                    let template_clone = template.clone();
                    let editing_component = editing_component.clone();
                    let editing_template = editing_template.clone();
                    let on_template_toggled = props.on_template_toggled.clone();
                    
                    html! {
                        <div class={format!("component-card {}", if template.is_default { "primary" } else { "secondary" })}>
                            <div class="component-preview">
                                {render_component_preview(&template.component_type, &header_navigation, &footer_navigation)}
                            </div>
                            <div class="component-info">
                                <h4>{get_component_title(&template.component_type)}</h4>
                                <p>{get_component_description(&template.component_type)}</p>
                                <div class="component-properties">
                                    <span class={format!("property-tag {}", if template.is_active { "active" } else { "inactive" })}>
                                        {if template.is_active { "Active" } else { "Inactive" }}
                                    </span>
                                    {render_component_tags(&template.component_type)}
                                </div>
                                <div class="component-actions">
                                    <button 
                                        class="btn-primary"
                                        onclick={{
                                            let template_clone = template_clone.clone();
                                            let component_type = template.component_type.clone();
                                            Callback::from(move |_| {
                                                editing_component.set(Some(component_type.clone()));
                                                editing_template.set(Some(template_clone.clone()));
                                            })
                                        }}
                                    >
                                        {format!("Customize {}", get_component_name(&template.component_type))}
                                    </button>
                                    {
                                        if template.component_type != "sidebar" {
                                            html! {
                                                <div class="toggle-switch">
                                                    <input 
                                                        type="checkbox"
                                                        id={format!("toggle-{}", template.id)}
                                                        checked={template.is_active}
                                                        onchange={{
                                                            let template_id = template.id;
                                                            let on_template_toggled = on_template_toggled.clone();
                                                            Callback::from(move |_| {
                                                                let on_template_toggled = on_template_toggled.clone();
                                                                wasm_bindgen_futures::spawn_local(async move {
                                                                    match toggle_component_template(template_id).await {
                                                                        Ok(updated_template) => {
                                                                            on_template_toggled.emit(updated_template);
                                                                            log::info!("✅ Toggled component template {}", template_id);
                                                                        }
                                                                        Err(e) => {
                                                                            log::error!("❌ Failed to toggle component template: {:?}", e);
                                                                        }
                                                                    }
                                                                });
                                                            })
                                                        }}
                                                    />
                                                    <label for={format!("toggle-{}", template.id)} class="slider"></label>
                                                </div>
                                            }
                                        } else { html!{} }
                                    }
                                </div>
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

#[function_component(ContainerSettingsView)]
pub fn container_settings_view() -> Html {
    let settings = use_state(ContainerSettings::default);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    let success_message = use_state(|| None::<String>);
    let show_image_picker = use_state(|| false);
    let show_video_picker = use_state(|| false);

    // Load container settings from backend
    {
        let settings = settings.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                match get_settings(Some("container")).await {
                    Ok(backend_settings) => {
                        let mut container_settings = ContainerSettings::default();
                        
                        // Map backend settings to container settings
                        for setting in backend_settings {
                            match setting.setting_key.as_str() {
                                "container_mobile_breakpoint" => container_settings.mobile_breakpoint = setting.setting_value.unwrap_or_default(),
                                "container_tablet_breakpoint" => container_settings.tablet_breakpoint = setting.setting_value.unwrap_or_default(),
                                "container_desktop_breakpoint" => container_settings.desktop_breakpoint = setting.setting_value.unwrap_or_default(),
                                "container_wide_breakpoint" => container_settings.wide_breakpoint = setting.setting_value.unwrap_or_default(),
                                "container_base_font_size" => container_settings.base_font_size = setting.setting_value.unwrap_or_default(),
                                "container_scale_ratio" => container_settings.scale_ratio = setting.setting_value.unwrap_or_default(),
                                "container_line_height" => container_settings.line_height = setting.setting_value.unwrap_or_default(),
                                "container_width_type" => container_settings.width_type = setting.setting_value.unwrap_or_default(),
                                "container_max_width" => container_settings.max_width = setting.setting_value.unwrap_or_default(),
                                "container_horizontal_padding" => container_settings.horizontal_padding = setting.setting_value.unwrap_or_default(),
                                // New extended settings
                                "container_background_type" => container_settings.background_type = setting.setting_value.unwrap_or_default(),
                                "container_background_color" => container_settings.background_color = setting.setting_value.unwrap_or_default(),
                                "container_gradient_from" => container_settings.gradient_from = setting.setting_value.unwrap_or_default(),
                                "container_gradient_to" => container_settings.gradient_to = setting.setting_value.unwrap_or_default(),
                                "container_gradient_angle" => container_settings.gradient_angle = setting.setting_value.unwrap_or_default(),
                                "container_background_image_url" => container_settings.background_image_url = setting.setting_value.unwrap_or_default(),
                                "container_background_image_size" => container_settings.background_image_size = setting.setting_value.unwrap_or_default(),
                                "container_background_image_position" => container_settings.background_image_position = setting.setting_value.unwrap_or_default(),
                                "container_background_video_url" => container_settings.background_video_url = setting.setting_value.unwrap_or_default(),
                                "container_background_video_loop" => container_settings.background_video_loop = setting.setting_value.as_deref() == Some("true"),
                                "container_background_video_autoplay" => container_settings.background_video_autoplay = setting.setting_value.as_deref() == Some("true"),
                                "container_background_video_muted" => container_settings.background_video_muted = setting.setting_value.as_deref() == Some("true"),
                                "container_overlay_color" => container_settings.overlay_color = setting.setting_value.unwrap_or_default(),
                                "container_overlay_opacity" => container_settings.overlay_opacity = setting.setting_value.unwrap_or_default(),
                                "container_border_radius" => container_settings.border_radius = setting.setting_value.unwrap_or_default(),
                                "container_border_width" => container_settings.border_width = setting.setting_value.unwrap_or_default(),
                                "container_border_color" => container_settings.border_color = setting.setting_value.unwrap_or_default(),
                                "container_box_shadow" => container_settings.box_shadow = setting.setting_value.unwrap_or_default(),
                                "container_animation" => container_settings.animation = setting.setting_value.unwrap_or_default(),
                                "container_acid_mode" => container_settings.acid_mode = setting.setting_value.as_deref() == Some("true"),
                                _ => {}
                            }
                        }
                        
                        settings.set(container_settings);
                    },
                    Err(e) => {
                        error.set(Some(format!("Failed to load settings: {}", e)));
                    }
                }
                loading.set(false);
            });
            || ()
        }, ());
    }

    // Save settings callback
    let save_settings = {
        let settings = settings.clone();
        let loading = loading.clone();
        let error = error.clone();
        let success_message = success_message.clone();

        Callback::from(move |_| {
            let settings = settings.clone();
            let loading = loading.clone();
            let error = error.clone();
            let success_message = success_message.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);
                success_message.set(None);

                let mut settings_data = vec![
                    SettingData {
                        key: "container_mobile_breakpoint".to_string(),
                        value: settings.mobile_breakpoint.clone(),
                        setting_type: "container".to_string(),
                        description: Some("Mobile breakpoint for responsive design".to_string()),
                    },
                    SettingData {
                        key: "container_tablet_breakpoint".to_string(),
                        value: settings.tablet_breakpoint.clone(),
                        setting_type: "container".to_string(),
                        description: Some("Tablet breakpoint for responsive design".to_string()),
                    },
                    SettingData {
                        key: "container_desktop_breakpoint".to_string(),
                        value: settings.desktop_breakpoint.clone(),
                        setting_type: "container".to_string(),
                        description: Some("Desktop breakpoint for responsive design".to_string()),
                    },
                    SettingData {
                        key: "container_wide_breakpoint".to_string(),
                        value: settings.wide_breakpoint.clone(),
                        setting_type: "container".to_string(),
                        description: Some("Wide screen breakpoint for responsive design".to_string()),
                    },
                    SettingData {
                        key: "container_base_font_size".to_string(),
                        value: settings.base_font_size.clone(),
                        setting_type: "container".to_string(),
                        description: Some("Base font size for typography system".to_string()),
                    },
                    SettingData {
                        key: "container_scale_ratio".to_string(),
                        value: settings.scale_ratio.clone(),
                        setting_type: "container".to_string(),
                        description: Some("Scale ratio for typography system".to_string()),
                    },
                    SettingData {
                        key: "container_line_height".to_string(),
                        value: settings.line_height.clone(),
                        setting_type: "container".to_string(),
                        description: Some("Line height for typography system".to_string()),
                    },
                    SettingData {
                        key: "container_width_type".to_string(),
                        value: settings.width_type.clone(),
                        setting_type: "container".to_string(),
                        description: Some("Container width type (fixed/fluid/hybrid)".to_string()),
                    },
                    SettingData {
                        key: "container_max_width".to_string(),
                        value: settings.max_width.clone(),
                        setting_type: "container".to_string(),
                        description: Some("Maximum container width".to_string()),
                    },
                    SettingData {
                        key: "container_horizontal_padding".to_string(),
                        value: settings.horizontal_padding.clone(),
                        setting_type: "container".to_string(),
                        description: Some("Container horizontal padding".to_string()),
                    },
                ];

                // Push extended settings
                settings_data.extend([
                    SettingData { key: "container_background_type".to_string(), value: settings.background_type.clone(), setting_type: "container".to_string(), description: Some("Background mode".to_string()) },
                    SettingData { key: "container_background_color".to_string(), value: settings.background_color.clone(), setting_type: "container".to_string(), description: Some("Background color".to_string()) },
                    SettingData { key: "container_gradient_from".to_string(), value: settings.gradient_from.clone(), setting_type: "container".to_string(), description: Some("Gradient start color".to_string()) },
                    SettingData { key: "container_gradient_to".to_string(), value: settings.gradient_to.clone(), setting_type: "container".to_string(), description: Some("Gradient end color".to_string()) },
                    SettingData { key: "container_gradient_angle".to_string(), value: settings.gradient_angle.clone(), setting_type: "container".to_string(), description: Some("Gradient angle".to_string()) },
                    SettingData { key: "container_background_image_url".to_string(), value: settings.background_image_url.clone(), setting_type: "container".to_string(), description: Some("Background image URL".to_string()) },
                    SettingData { key: "container_background_image_size".to_string(), value: settings.background_image_size.clone(), setting_type: "container".to_string(), description: Some("Background image size".to_string()) },
                    SettingData { key: "container_background_image_position".to_string(), value: settings.background_image_position.clone(), setting_type: "container".to_string(), description: Some("Background image position".to_string()) },
                    SettingData { key: "container_background_video_url".to_string(), value: settings.background_video_url.clone(), setting_type: "container".to_string(), description: Some("Background video URL".to_string()) },
                    SettingData { key: "container_background_video_loop".to_string(), value: settings.background_video_loop.to_string(), setting_type: "container".to_string(), description: Some("Background video loop".to_string()) },
                    SettingData { key: "container_background_video_autoplay".to_string(), value: settings.background_video_autoplay.to_string(), setting_type: "container".to_string(), description: Some("Background video autoplay".to_string()) },
                    SettingData { key: "container_background_video_muted".to_string(), value: settings.background_video_muted.to_string(), setting_type: "container".to_string(), description: Some("Background video muted".to_string()) },
                    SettingData { key: "container_overlay_color".to_string(), value: settings.overlay_color.clone(), setting_type: "container".to_string(), description: Some("Overlay color".to_string()) },
                    SettingData { key: "container_overlay_opacity".to_string(), value: settings.overlay_opacity.clone(), setting_type: "container".to_string(), description: Some("Overlay opacity".to_string()) },
                    SettingData { key: "container_border_radius".to_string(), value: settings.border_radius.clone(), setting_type: "container".to_string(), description: Some("Border radius".to_string()) },
                    SettingData { key: "container_border_width".to_string(), value: settings.border_width.clone(), setting_type: "container".to_string(), description: Some("Border width".to_string()) },
                    SettingData { key: "container_border_color".to_string(), value: settings.border_color.clone(), setting_type: "container".to_string(), description: Some("Border color".to_string()) },
                    SettingData { key: "container_box_shadow".to_string(), value: settings.box_shadow.clone(), setting_type: "container".to_string(), description: Some("Box shadow".to_string()) },
                    SettingData { key: "container_animation".to_string(), value: settings.animation.clone(), setting_type: "container".to_string(), description: Some("Container animation".to_string()) },
                    SettingData { key: "container_acid_mode".to_string(), value: settings.acid_mode.to_string(), setting_type: "container".to_string(), description: Some("Enable animated gradient borders across components".to_string()) },
                ]);

                match update_settings(settings_data).await {
                    Ok(_) => {
                        success_message.set(Some("Container settings saved successfully!".to_string()));
                    },
                    Err(e) => {
                        error.set(Some(format!("Failed to save settings: {}", e)));
                    }
                }
                loading.set(false);
            });
        })
    };

    // Reset to defaults callback
    let reset_settings = {
        let settings = settings.clone();
        Callback::from(move |_| {
            settings.set(ContainerSettings::default());
        })
    };

    // Input change handlers
    let on_mobile_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut current_settings = (*settings).clone();
            current_settings.mobile_breakpoint = input.value();
            settings.set(current_settings);
        })
    };

    let on_tablet_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut current_settings = (*settings).clone();
            current_settings.tablet_breakpoint = input.value();
            settings.set(current_settings);
        })
    };

    let on_desktop_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut current_settings = (*settings).clone();
            current_settings.desktop_breakpoint = input.value();
            settings.set(current_settings);
        })
    };

    let on_wide_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut current_settings = (*settings).clone();
            current_settings.wide_breakpoint = input.value();
            settings.set(current_settings);
        })
    };

    let on_font_size_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let mut current_settings = (*settings).clone();
            current_settings.base_font_size = select.value();
            settings.set(current_settings);
        })
    };

    let on_scale_ratio_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let mut current_settings = (*settings).clone();
            current_settings.scale_ratio = select.value();
            settings.set(current_settings);
        })
    };

    let on_line_height_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut current_settings = (*settings).clone();
            current_settings.line_height = input.value();
            settings.set(current_settings);
        })
    };

    let on_width_type_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let mut current_settings = (*settings).clone();
            current_settings.width_type = select.value();
            settings.set(current_settings);
        })
    };

    let on_max_width_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut current_settings = (*settings).clone();
            current_settings.max_width = input.value();
            settings.set(current_settings);
        })
    };

    let on_padding_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut current_settings = (*settings).clone();
            current_settings.horizontal_padding = input.value();
            settings.set(current_settings);
        })
    };

    // Extended: background/media handlers
    let on_background_type_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.background_type = select.value();
            settings.set(s);
        })
    };
    let on_background_color_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.background_color = input.value();
            settings.set(s);
        })
    };
    let on_gradient_from_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.gradient_from = input.value();
            settings.set(s);
        })
    };
    let on_gradient_to_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.gradient_to = input.value();
            settings.set(s);
        })
    };
    let on_gradient_angle_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.gradient_angle = input.value();
            settings.set(s);
        })
    };
    let on_background_image_url_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.background_image_url = input.value();
            settings.set(s);
        })
    };
    let open_image_picker = {
        let show_image_picker = show_image_picker.clone();
        Callback::from(move |_| show_image_picker.set(true))
    };
    let close_image_picker = {
        let show_image_picker = show_image_picker.clone();
        Callback::from(move |_| show_image_picker.set(false))
    };
    let on_image_selected = {
        let settings = settings.clone();
        let show_image_picker = show_image_picker.clone();
        Callback::from(move |media: crate::services::api_service::MediaItem| {
            let mut s = (*settings).clone();
            s.background_image_url = format!("http://localhost:8081{}", media.url);
            settings.set(s);
            show_image_picker.set(false);
        })
    };
    let on_background_image_size_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.background_image_size = select.value();
            settings.set(s);
        })
    };
    let on_background_image_position_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.background_image_position = select.value();
            settings.set(s);
        })
    };
    let on_background_video_url_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.background_video_url = input.value();
            settings.set(s);
        })
    };
    let open_video_picker = {
        let show_video_picker = show_video_picker.clone();
        Callback::from(move |_| show_video_picker.set(true))
    };
    let close_video_picker = {
        let show_video_picker = show_video_picker.clone();
        Callback::from(move |_| show_video_picker.set(false))
    };
    let on_video_selected = {
        let settings = settings.clone();
        let show_video_picker = show_video_picker.clone();
        Callback::from(move |media: crate::services::api_service::MediaItem| {
            let mut s = (*settings).clone();
            s.background_video_url = format!("http://localhost:8081{}", media.url);
            settings.set(s);
            show_video_picker.set(false);
        })
    };
    let on_background_video_loop_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.background_video_loop = input.checked();
            settings.set(s);
        })
    };
    let on_background_video_autoplay_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.background_video_autoplay = input.checked();
            settings.set(s);
        })
    };
    let on_overlay_color_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.overlay_color = input.value();
            settings.set(s);
        })
    };
    let on_overlay_opacity_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.overlay_opacity = input.value();
            settings.set(s);
        })
    };
    let on_border_radius_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.border_radius = input.value();
            settings.set(s);
        })
    };
    let on_border_width_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.border_width = input.value();
            settings.set(s);
        })
    };

    // Acid Mode toggle handler
    let on_acid_mode_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.acid_mode = input.checked();
            settings.set(s);
        })
    };
    let on_border_color_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.border_color = input.value();
            settings.set(s);
        })
    };
    let on_box_shadow_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.box_shadow = input.value();
            settings.set(s);
        })
    };
    let on_animation_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let mut s = (*settings).clone();
            s.animation = select.value();
            settings.set(s);
        })
    };

    html! {
        <div class="container-settings-section">
            <h2>{"📦 Container Settings"}</h2>
            <p>{"Manage global breakpoints, typography, and container settings that tie into the design system"}</p>
            
            if let Some(error_msg) = (*error).as_ref() {
                <div class="error-message">
                    <span class="error-icon">{"⚠️"}</span>
                    <span>{error_msg}</span>
                </div>
            }
            
            if let Some(success_msg) = (*success_message).as_ref() {
                <div class="success-message">
                    <span class="success-icon">{"✅"}</span>
                    <span>{success_msg}</span>
                </div>
            }
            
            if *loading {
                <div class="loading-overlay">{"Loading..."}</div>
            }
            
            <div class="settings-grid">
                <div class="settings-card">
                    <h3>{"📐 Global Breakpoints"}</h3>
                    <p>{"Define responsive breakpoints for the entire site"}</p>
                    
                    <div class="breakpoint-list">
                        <div class="breakpoint-item">
                            <label>{"Mobile (max-width)"}</label>
                            <div class="breakpoint-input-group">
                                <input 
                                    type="text" 
                                    value={settings.mobile_breakpoint.clone()} 
                                    class="breakpoint-input"
                                    onchange={on_mobile_change}
                                />
                                <span class="breakpoint-preview">{format!("< {}", settings.mobile_breakpoint)}</span>
                            </div>
                        </div>
                        <div class="breakpoint-item">
                            <label>{"Tablet (max-width)"}</label>
                            <div class="breakpoint-input-group">
                                <input 
                                    type="text" 
                                    value={settings.tablet_breakpoint.clone()} 
                                    class="breakpoint-input"
                                    onchange={on_tablet_change}
                                />
                                <span class="breakpoint-preview">{format!("{} - {}", settings.mobile_breakpoint, settings.tablet_breakpoint)}</span>
                            </div>
                        </div>
                        <div class="breakpoint-item">
                            <label>{"Desktop (max-width)"}</label>
                            <div class="breakpoint-input-group">
                                <input 
                                    type="text" 
                                    value={settings.desktop_breakpoint.clone()} 
                                    class="breakpoint-input"
                                    onchange={on_desktop_change}
                                />
                                <span class="breakpoint-preview">{format!("{} - {}", settings.tablet_breakpoint, settings.desktop_breakpoint)}</span>
                            </div>
                        </div>
                        <div class="breakpoint-item">
                            <label>{"Wide Screen (max-width)"}</label>
                            <div class="breakpoint-input-group">
                                <input 
                                    type="text" 
                                    value={settings.wide_breakpoint.clone()} 
                                    class="breakpoint-input"
                                    onchange={on_wide_change}
                                />
                                <span class="breakpoint-preview">{format!("> {}", settings.desktop_breakpoint)}</span>
                            </div>
                        </div>
                    </div>
                </div>
                
                <div class="settings-card">
                    <h3>{"🎯 Typography System"}</h3>
                    <p>{"Configure global typography that integrates with the design system"}</p>
                    
                    <div class="typography-settings">
                        <div class="typography-group">
                            <h4>{"Base Settings"}</h4>
                            <div class="typography-item">
                                <label>{"Base Font Size"}</label>
                                <select class="typography-select" onchange={on_font_size_change}>
                                    <option value="14px" selected={settings.base_font_size == "14px"}>{"14px (Small)"}</option>
                                    <option value="16px" selected={settings.base_font_size == "16px"}>{"16px (Standard)"}</option>
                                    <option value="18px" selected={settings.base_font_size == "18px"}>{"18px (Large)"}</option>
                                </select>
                            </div>
                            <div class="typography-item">
                                <label>{"Scale Ratio"}</label>
                                <select class="typography-select" onchange={on_scale_ratio_change}>
                                    <option value="1.125" selected={settings.scale_ratio == "1.125"}>{"1.125 (Minor Second)"}</option>
                                    <option value="1.25" selected={settings.scale_ratio == "1.25"}>{"1.25 (Major Third)"}</option>
                                    <option value="1.5" selected={settings.scale_ratio == "1.5"}>{"1.5 (Perfect Fifth)"}</option>
                                    <option value="1.618" selected={settings.scale_ratio == "1.618"}>{"1.618 (Golden Ratio)"}</option>
                                </select>
                            </div>
                            <div class="typography-item">
                                <label>{"Line Height"}</label>
                                <input 
                                    type="number" 
                                    value={settings.line_height.clone()} 
                                    step="0.1" 
                                    min="1" 
                                    max="2" 
                                    class="typography-input"
                                    onchange={on_line_height_change}
                                />
                            </div>
                        </div>
                        
                        <div class="typography-preview">
                            <h4>{"Typography Preview"}</h4>
                            <div class="preview-text" style={format!("font-size: {}; line-height: {}", settings.base_font_size, settings.line_height)}>
                                <h1 class="preview-h1">{"Heading 1 - Main Title"}</h1>
                                <h2 class="preview-h2">{"Heading 2 - Section Title"}</h2>
                                <p class="preview-paragraph">
                                    {"This is a paragraph demonstrating the body text with current typography settings."}
                                </p>
                                <code class="preview-code">{"console.log('Code example');"}</code>
                            </div>
                        </div>
                    </div>
                </div>
                
                <div class="settings-card">
                    <h3>{"📦 Container Configuration"}</h3>
                    <p>{"Configure the main container that wraps site content"}</p>
                    
                    <div class="container-settings">
                        <div class="container-group">
                            <h4>{"Container Width"}</h4>
                            <div class="container-item">
                                <label>{"Width Type"}</label>
                                <select class="container-select" onchange={on_width_type_change}>
                                    <option value="fixed" selected={settings.width_type == "fixed"}>{"Fixed Width"}</option>
                                    <option value="fluid" selected={settings.width_type == "fluid"}>{"Fluid Width"}</option>
                                    <option value="hybrid" selected={settings.width_type == "hybrid"}>{"Hybrid"}</option>
                                </select>
                            </div>
                            <div class="container-item">
                                <label>{"Max Width"}</label>
                                <input 
                                    type="text" 
                                    value={settings.max_width.clone()} 
                                    class="container-input"
                                    onchange={on_max_width_change}
                                />
                            </div>
                            <div class="container-item">
                                <label>{"Horizontal Padding"}</label>
                                <input 
                                    type="text" 
                                    value={settings.horizontal_padding.clone()} 
                                    class="container-input"
                                    onchange={on_padding_change}
                                />
                            </div>
                        </div>
                    </div>
                </div>

                <div class="settings-card">
                    <h3>{"🌈 Background & Media"}</h3>
                    <p>{"Background color, gradient, image or video with overlay"}</p>
                    <div class="container-settings">
                        <div class="container-group">
                            <h4>{"Background"}</h4>
                            <div class="container-item">
                                <label>{"Type"}</label>
                                <select class="container-select" onchange={on_background_type_change}>
                                    <option value="none" selected={settings.background_type == "none"}>{"None"}</option>
                                    <option value="color" selected={settings.background_type == "color"}>{"Solid Color"}</option>
                                    <option value="gradient" selected={settings.background_type == "gradient"}>{"Gradient"}</option>
                                    <option value="image" selected={settings.background_type == "image"}>{"Image"}</option>
                                    <option value="video" selected={settings.background_type == "video"}>{"Video"}</option>
                                </select>
                            </div>
                            {
                                match settings.background_type.as_str() {
                                    "color" => html! {
                                        <div class="container-item">
                                            <label>{"Background Color"}</label>
                                            <input type="text" value={settings.background_color.clone()} class="container-input" onchange={on_background_color_change} />
                                        </div>
                                    },
                                    "gradient" => html! {
                                        <>
                                            <div class="container-item">
                                                <label>{"From"}</label>
                                                <input type="text" value={settings.gradient_from.clone()} class="container-input" onchange={on_gradient_from_change} />
                                            </div>
                                            <div class="container-item">
                                                <label>{"To"}</label>
                                                <input type="text" value={settings.gradient_to.clone()} class="container-input" onchange={on_gradient_to_change} />
                                            </div>
                                            <div class="container-item">
                                                <label>{"Angle"}</label>
                                                <input type="text" value={settings.gradient_angle.clone()} class="container-input" onchange={on_gradient_angle_change} />
                                            </div>
                                        </>
                                    },
                                    "image" => html! {
                                        <>
                                            <div class="container-item">
                                                <label>{"Image URL"}</label>
                                                <input type="text" value={settings.background_image_url.clone()} class="container-input" onchange={on_background_image_url_change} />
                                                <button class="btn-secondary" onclick={open_image_picker.clone()} style="margin-left: 8px;">{"Choose Image"}</button>
                                            </div>
                                            <div class="container-item">
                                                <label>{"Size"}</label>
                                                <select class="container-select" onchange={on_background_image_size_change}>
                                                    <option value="cover" selected={settings.background_image_size == "cover"}>{"Cover"}</option>
                                                    <option value="contain" selected={settings.background_image_size == "contain"}>{"Contain"}</option>
                                                    <option value="auto" selected={settings.background_image_size == "auto"}>{"Auto"}</option>
                                                </select>
                                            </div>
                                            <div class="container-item">
                                                <label>{"Position"}</label>
                                                <select class="container-select" onchange={on_background_image_position_change}>
                                                    <option value="center" selected={settings.background_image_position == "center"}>{"Center"}</option>
                                                    <option value="top" selected={settings.background_image_position == "top"}>{"Top"}</option>
                                                    <option value="bottom" selected={settings.background_image_position == "bottom"}>{"Bottom"}</option>
                                                    <option value="left" selected={settings.background_image_position == "left"}>{"Left"}</option>
                                                    <option value="right" selected={settings.background_image_position == "right"}>{"Right"}</option>
                                                </select>
                                            </div>
                                        </>
                                    },
                                    "video" => html! {
                                        <>
                                            <div class="container-item">
                                                <label>{"Video URL"}</label>
                                                <input type="text" value={settings.background_video_url.clone()} class="container-input" onchange={on_background_video_url_change} />
                                                <button class="btn-secondary" onclick={open_video_picker.clone()} style="margin-left: 8px;">{"Choose Video"}</button>
                                            </div>
                                            <div class="container-item">
                                                <label>{"Loop"}</label>
                                                <input type="checkbox" checked={settings.background_video_loop} onchange={on_background_video_loop_change} />
                                            </div>
                                            <div class="container-item">
                                                <label>{"Autoplay"}</label>
                                                <input type="checkbox" checked={settings.background_video_autoplay} onchange={on_background_video_autoplay_change} />
                                            </div>
                                            <div class="container-item">
                                                <label>{"Muted"}</label>
                                                <input type="checkbox" checked={settings.background_video_muted} onchange={
                                                    let settings = settings.clone();
                                                    Callback::from(move |e: Event| {
                                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                        let mut s = (*settings).clone();
                                                        s.background_video_muted = input.checked();
                                                        settings.set(s);
                                                    })
                                                } />
                                            </div>
                                        </>
                                    },
                                    _ => html! {}
                                }
                            }
                        </div>

                        <div class="container-group">
                            <h4>{"Overlay"}</h4>
                            <div class="container-item">
                                <label>{"Overlay Color"}</label>
                                <input type="text" value={settings.overlay_color.clone()} class="container-input" onchange={on_overlay_color_change} />
                            </div>
                            <div class="container-item">
                                <label>{"Overlay Opacity"}</label>
                                <input type="text" value={settings.overlay_opacity.clone()} class="container-input" onchange={on_overlay_opacity_change} />
                            </div>
                        </div>
                    </div>
                </div>

                <div class="settings-card">
                    <h3>{"🧱 Borders & Effects"}</h3>
                    <p>{"Borders and shadow for the main container"}</p>
                    <div class="container-settings">
                        <div class="container-group">
                            <div class="container-item">
                                <label>{"Border Radius"}</label>
                                <input type="text" value={settings.border_radius.clone()} class="container-input" onchange={on_border_radius_change} />
                            </div>
                            <div class="container-item">
                                <label>{"Border Width"}</label>
                                <input type="text" value={settings.border_width.clone()} class="container-input" onchange={on_border_width_change} />
                            </div>
                            <div class="container-item">
                                <label>{"Border Color"}</label>
                                <input type="text" value={settings.border_color.clone()} class="container-input" onchange={on_border_color_change} />
                            </div>
                            <div class="container-item">
                                <label>{"Box Shadow"}</label>
                                <input type="text" value={settings.box_shadow.clone()} class="container-input" onchange={on_box_shadow_change} />
                            </div>
                                <div class="container-item">
                                    <label>{"Acid Mode (animated gradient borders)"}</label>
                                    <input type="checkbox" checked={settings.acid_mode} onchange={on_acid_mode_change} />
                                </div>
                        </div>
                    </div>
                </div>

                <div class="settings-card">
                    <h3>{"🎞️ Animation"}</h3>
                    <p>{"Entrance animation for the main container"}</p>
                    <div class="container-settings">
                        <div class="container-group">
                            <div class="container-item">
                                <label>{"Animation"}</label>
                                <select class="container-select" onchange={on_animation_change}>
                                    <option value="none" selected={settings.animation == "none"}>{"None"}</option>
                                    <option value="fade-in" selected={settings.animation == "fade-in"}>{"Fade In"}</option>
                                    <option value="slide-up" selected={settings.animation == "slide-up"}>{"Slide Up"}</option>
                                    <option value="slide-down" selected={settings.animation == "slide-down"}>{"Slide Down"}</option>
                                    <option value="zoom-in" selected={settings.animation == "zoom-in"}>{"Zoom In"}</option>
                                </select>
                            </div>
                        </div>
                    </div>
                </div>
                // Media pickers
                <crate::components::MediaPicker 
                    show={*show_image_picker}
                    filter_images_only={true}
                    on_close={close_image_picker}
                    on_select={on_image_selected}
                    on_multi_select={None::<Callback<Vec<MediaItem>>>}
                    allow_multi_select={false}
                />
                <crate::components::MediaPicker 
                    show={*show_video_picker}
                    filter_images_only={false}
                    on_close={close_video_picker}
                    on_select={on_video_selected}
                    on_multi_select={None::<Callback<Vec<MediaItem>>>}
                    allow_multi_select={false}
                />
            </div>
            
            <div class="settings-actions">
                <button 
                    class="btn-primary large" 
                    onclick={save_settings}
                    disabled={*loading}
                >
                    {if *loading { "Saving..." } else { "Save Global Settings" }}
                </button>
                <button 
                    class="btn-secondary" 
                    onclick={reset_settings}
                    disabled={*loading}
                >
                    {"Reset to Defaults"}
                </button>
                <button class="btn-secondary" disabled={*loading}>{"Export Settings"}</button>
            </div>
        </div>
    }
}

fn render_text_shadow_section(customization: &MenuAreaCustomization, update_customization: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="customization-section" style="
            background: #f8f9fa;
            padding: 20px;
            border-radius: 12px;
            border: 1px solid #e9ecef;
            margin-top: 16px;
        ">
            <h4 style="margin: 0 0 16px 0; color: #495057; font-size: 18px; font-weight: 600; display: flex; align-items: center; gap: 8px;">
                <span>{"✨"}</span>
                {"Text Shadow Effects"}
            </h4>
            
            <div style="display: grid; gap: 16px;">
                // Text Shadow Toggle
                <div>
                    <label style="display: flex; align-items: center; gap: 8px; font-weight: 600; color: #374151; cursor: pointer;">
                        <input 
                            type="checkbox"
                            checked={customization.text_shadow_enabled}
                            onchange={{
                                let update_customization = update_customization.clone();
                                Callback::from(move |e: Event| {
                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                    update_customization.emit(vec![("text_shadow_enabled".to_string(), target.checked().to_string())]);
                                })
                            }}
                            style="
                                width: 18px;
                                height: 18px;
                                accent-color: #8b5cf6;
                            "
                        />
                        {"Enable Text Shadow Effects"}
                    </label>
                </div>

                {if customization.text_shadow_enabled {
                    html! {
                        <>
                            // Shadow Type Selection
                            <div>
                                <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                                    {"Shadow Type"}
                                </label>
                                <div class="shadow-type-grid" style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 6px;">
                                    {["glow", "drop-shadow", "outline", "neon"].iter().map(|&shadow_type| {
                                        let is_active = customization.text_shadow_type == shadow_type;
                                        let update_customization = update_customization.clone();
                                        html! {
                                            <button
                                                style={format!("
                                                    background: {};
                                                    border: 1px solid {};
                                                    color: {};
                                                    padding: 8px 10px;
                                                    border-radius: 6px;
                                                    cursor: pointer;
                                                    font-size: 11px;
                                                    font-weight: 600;
                                                    transition: all 0.2s ease;
                                                    display: flex;
                                                    align-items: center;
                                                    justify-content: center;
                                                    gap: 4px;
                                                ",
                                                    if is_active { "#8b5cf6" } else { "white" },
                                                    if is_active { "#8b5cf6" } else { "#d1d5db" },
                                                    if is_active { "white" } else { "#6b7280" }
                                                )}
                                                onclick={Callback::from(move |_| {
                                                    update_customization.emit(vec![("text_shadow_type".to_string(), shadow_type.to_string())]);
                                                })}
                                            >
                                                <span>{match shadow_type {
                                                    "glow" => "✨",
                                                    "drop-shadow" => "🌑",
                                                    "outline" => "📝",
                                                    "neon" => "🌈",
                                                    _ => "✨"
                                                }}</span>
                                                {shadow_type.to_uppercase()}
                                            </button>
                                        }
                                    }).collect::<Html>()}
                                </div>
                            </div>

                            // Shadow Color
                            <div>
                                <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                                    {"Shadow Color"}
                                </label>
                                <input 
                                    type="color"
                                    value={customization.text_shadow_color.clone()}
                                    onchange={{
                                        let update_customization = update_customization.clone();
                                        Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                            update_customization.emit(vec![("text_shadow_color".to_string(), target.value())]);
                                        })
                                    }}
                                    style="
                                        width: 100%;
                                        height: 40px;
                                        border: 2px solid #e9ecef;
                                        border-radius: 8px;
                                        cursor: pointer;
                                    "
                                />
                            </div>

                            // Shadow Intensity and Blur Controls
                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px;">
                                <div>
                                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                                        {"Intensity: "}{&customization.text_shadow_intensity}{"%"}
                                    </label>
                                    <input
                                        type="range"
                                        min="0"
                                        max="100"
                                        value={customization.text_shadow_intensity.clone()}
                                        oninput={{
                                            let update_customization = update_customization.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                update_customization.emit(vec![("text_shadow_intensity".to_string(), target.value())]);
                                            })
                                        }}
                                        style="
                                            width: 100%;
                                            height: 6px;
                                            border-radius: 3px;
                                            background: linear-gradient(to right, #e2e8f0, #8b5cf6);
                                            outline: none;
                                            cursor: pointer;
                                        "
                                    />
                                </div>
                                <div>
                                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                                        {"Blur: "}{&customization.text_shadow_blur}{"px"}
                                    </label>
                                    <input
                                        type="range"
                                        min="0"
                                        max="20"
                                        value={customization.text_shadow_blur.clone()}
                                        oninput={{
                                            let update_customization = update_customization.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                update_customization.emit(vec![("text_shadow_blur".to_string(), target.value())]);
                                            })
                                        }}
                                        style="
                                            width: 100%;
                                            height: 6px;
                                            border-radius: 3px;
                                            background: linear-gradient(to right, #e2e8f0, #8b5cf6);
                                            outline: none;
                                            cursor: pointer;
                                        "
                                    />
                                </div>
                            </div>

                            // Shadow Offset Controls
                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px;">
                                <div>
                                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                                        {"X Offset: "}{&customization.text_shadow_offset_x}{"px"}
                                    </label>
                                    <input
                                        type="range"
                                        min="-10"
                                        max="10"
                                        value={customization.text_shadow_offset_x.clone()}
                                        oninput={{
                                            let update_customization = update_customization.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                update_customization.emit(vec![("text_shadow_offset_x".to_string(), target.value())]);
                                            })
                                        }}
                                        style="
                                            width: 100%;
                                            height: 6px;
                                            border-radius: 3px;
                                            background: linear-gradient(to right, #e2e8f0, #8b5cf6);
                                            outline: none;
                                            cursor: pointer;
                                        "
                                    />
                                </div>
                                <div>
                                    <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 8px; font-size: 14px;">
                                        {"Y Offset: "}{&customization.text_shadow_offset_y}{"px"}
                                    </label>
                                    <input
                                        type="range"
                                        min="-10"
                                        max="10"
                                        value={customization.text_shadow_offset_y.clone()}
                                        oninput={{
                                            let update_customization = update_customization.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                update_customization.emit(vec![("text_shadow_offset_y".to_string(), target.value())]);
                                            })
                                        }}
                                        style="
                                            width: 100%;
                                            height: 6px;
                                            border-radius: 3px;
                                            background: linear-gradient(to right, #e2e8f0, #8b5cf6);
                                            outline: none;
                                            cursor: pointer;
                                        "
                                    />
                                </div>
                            </div>
                        </>
                    }
                } else { html! {} }}
            </div>
        </div>
    }
}

