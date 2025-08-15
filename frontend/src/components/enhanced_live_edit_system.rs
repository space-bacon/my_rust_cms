use yew::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{window, Element, MouseEvent};

use crate::services::navigation_service::ComponentTemplate;
use crate::services::api_service::PageItem;
use crate::services::page_service::{get_page_by_slug, update_page_content};
use crate::services::auth_service::{get_auth_token, is_authenticated};
use crate::components::page_builder::drag_drop_builder::{PageComponent, ComponentType, ComponentProperties};
use crate::components::unified_properties_panel::{UnifiedPropertiesPanel, PanelType};

#[derive(Properties, PartialEq, Clone)]
pub struct EnhancedLiveEditSystemProps {
    pub enabled: bool,
    pub component_templates: Vec<ComponentTemplate>,
    pub on_templates_updated: Callback<Vec<ComponentTemplate>>,
    pub page_components: Vec<PageComponent>,
    pub on_page_components_updated: Callback<Vec<PageComponent>>,
    pub current_page: Option<PageItem>,
}

#[derive(Clone, PartialEq)]
enum EditTarget {
    Header,
    Footer,
    Container,
    PageComponent(PageComponent),
}

#[function_component(EnhancedLiveEditSystem)]
pub fn enhanced_live_edit_system(props: &EnhancedLiveEditSystemProps) -> Html {
    let selected_target = use_state(|| Option::<EditTarget>::None);
    let show_properties_panel = use_state(|| false);

    // Setup highlighting and click handlers when enabled
    {
        let selected_target = selected_target.clone();
        let show_properties_panel = show_properties_panel.clone();
        let page_components = props.page_components.clone();
        
        use_effect_with_deps(move |enabled| {
            if *enabled {
                if let Some(doc) = window().and_then(|w| w.document()) {
                    // Add highlighting to header, footer, and container
                    for (id, target_type) in [
                        ("site-header", EditTarget::Header),
                        ("site-footer", EditTarget::Footer),
                        ("site-container", EditTarget::Container),
                    ] {
                        if let Some(el) = doc.get_element_by_id(id) {
                            let _ = el.set_attribute("data-live-editable", "true");
                            
                            // Apply consistent highlighting style
                            if let Some(existing) = el.get_attribute("style") {
                                let _ = el.set_attribute("style", &format!("{}; outline: 2px dashed rgba(0,150,255,0.8); outline-offset: -2px; cursor: pointer;", existing));
                            } else {
                                let _ = el.set_attribute("style", "outline: 2px dashed rgba(0,150,255,0.8); outline-offset: -2px; cursor: pointer;");
                            }
                            
                            // Add click handler
                            let selected_target_clone = selected_target.clone();
                            let show_properties_panel_clone = show_properties_panel.clone();
                            let target_type_clone = target_type.clone();
                            
                            let closure: Closure<dyn FnMut(web_sys::Event)> = Closure::wrap(Box::new(move |e: web_sys::Event| {
                                e.stop_propagation();
                                selected_target_clone.set(Some(target_type_clone.clone()));
                                show_properties_panel_clone.set(true);
                            }) as Box<dyn FnMut(_)>);
                            
                            let _ = el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
                            closure.forget();
                        }
                    }

                    // Add highlighting to page builder components
                    let components_with_class = doc.query_selector_all(".component, .canvas-component").unwrap();
                    for i in 0..components_with_class.length() {
                        if let Some(node) = components_with_class.item(i) {
                            if let Ok(el) = node.dyn_into::<Element>() {
                                let _ = el.set_attribute("data-live-component-editable", "true");
                                
                                // Apply consistent highlighting style (same as header/footer)
                                if let Some(existing) = el.get_attribute("style") {
                                    let _ = el.set_attribute("style", &format!("{}; outline: 2px dashed rgba(0,150,255,0.8); outline-offset: -2px; cursor: pointer;", existing));
                                } else {
                                    let _ = el.set_attribute("style", "outline: 2px dashed rgba(0,150,255,0.8); outline-offset: -2px; cursor: pointer;");
                                }

                                // Add component ID for selection
                                let component_id = format!("live-component-{}", i);
                                let _ = el.set_attribute("data-component-id", &component_id);
                                
                                // Add click handler for page components
                                let selected_target_clone = selected_target.clone();
                                let show_properties_panel_clone = show_properties_panel.clone();
                                let page_components_clone = page_components.clone();
                                
                                let closure: Closure<dyn FnMut(MouseEvent)> = Closure::wrap(Box::new(move |e: MouseEvent| {
                                    e.stop_propagation();
                                    
                                    if let Some(target_element) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) {
                                        // Extract actual content from the clicked element
                                        let component_type = detect_component_type_from_element(&e);
                                        let component = create_sample_component_from_element(component_type, &component_id, &target_element);
                                        
                                        selected_target_clone.set(Some(EditTarget::PageComponent(component)));
                                        show_properties_panel_clone.set(true);
                                    }
                                }) as Box<dyn FnMut(_)>);
                                
                                let _ = el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
                                closure.forget();
                            }
                        }
                    }
                }
            }

            // Cleanup function
            || {
                if let Some(doc) = window().and_then(|w| w.document()) {
                    // Remove highlighting from header, footer, container
                    for id in ["site-header", "site-footer", "site-container"] {
                        if let Some(el) = doc.get_element_by_id(id) {
                            let _ = el.remove_attribute("data-live-editable");
                            if let Some(existing) = el.get_attribute("style") {
                                let cleaned = existing
                                    .replace("outline: 2px dashed rgba(0,150,255,0.8);", "")
                                    .replace("outline-offset: -2px;", "")
                                    .replace("cursor: pointer;", "");
                                let _ = el.set_attribute("style", cleaned.trim());
                            }
                        }
                    }

                    // Remove highlighting from page components
                    let components_with_class = doc.query_selector_all("[data-live-component-editable]").unwrap();
                    for i in 0..components_with_class.length() {
                        if let Some(node) = components_with_class.item(i) {
                            if let Ok(el) = node.dyn_into::<Element>() {
                                let _ = el.remove_attribute("data-live-component-editable");
                                let _ = el.remove_attribute("data-component-id");
                                if let Some(existing) = el.get_attribute("style") {
                                    let cleaned = existing
                                        .replace("outline: 2px dashed rgba(0,150,255,0.8);", "")
                                        .replace("outline-offset: -2px;", "")
                                        .replace("cursor: pointer;", "");
                                    let _ = el.set_attribute("style", cleaned.trim());
                                }
                            }
                        }
                    }
                }
            }
        }, props.enabled);
    }

    let on_close_panel = {
        let selected_target = selected_target.clone();
        let show_properties_panel = show_properties_panel.clone();
        Callback::from(move |_| {
            selected_target.set(None);
            show_properties_panel.set(false);
        })
    };

    let on_component_updated = {
        let selected_target = selected_target.clone();
        let show_properties_panel = show_properties_panel.clone();
        
        Callback::from(move |updated_component: PageComponent| {
            // For text components, update the DOM directly
            if matches!(updated_component.component_type, ComponentType::Text | ComponentType::Heading | ComponentType::Subheading) {
                // Find the DOM element that was clicked and update its content
                if let Some(window) = window() {
                    if let Some(document) = window.document() {
                        // Look for the element with the component ID
                        if let Some(element) = document.query_selector(&format!("[data-component-id='{}']", updated_component.id)).ok().flatten() {
                            // Find the actual content element (h1, h2, p, etc.) within the selected element
                            let content_element = match updated_component.component_type {
                                ComponentType::Heading => {
                                    // Look for h1, h2, h3, etc. within the element
                                    if let Some(heading) = element.query_selector("h1, h2, h3, h4, h5, h6").ok().flatten() {
                                        Some(heading)
                                    } else if element.tag_name().to_lowercase().starts_with('h') {
                                        Some(element.clone())
                                    } else {
                                        None
                                    }
                                }
                                ComponentType::Subheading => {
                                    // Look for heading elements, preferring h2-h6
                                    if let Some(heading) = element.query_selector("h2, h3, h4, h5, h6, h1").ok().flatten() {
                                        Some(heading)
                                    } else if element.tag_name().to_lowercase().starts_with('h') {
                                        Some(element.clone())
                                    } else {
                                        None
                                    }
                                }
                                ComponentType::Text => {
                                    // Look for p, div, or other text containers
                                    if let Some(text_elem) = element.query_selector("p, div, span").ok().flatten() {
                                        Some(text_elem)
                                    } else if matches!(element.tag_name().to_lowercase().as_str(), "p" | "div" | "span") {
                                        Some(element.clone())
                                    } else {
                                        Some(element.clone()) // Fallback to the element itself
                                    }
                                }
                                _ => Some(element.clone())
                            };
                            
                            // Update the text content of the found element
                            if let Some(target_element) = content_element {
                                target_element.set_text_content(Some(&updated_component.content));
                                web_sys::console::log_1(&format!("Updated {} element with new content: '{}'", target_element.tag_name(), updated_component.content).into());
                            } else {
                                // Fallback: update the original element
                                element.set_text_content(Some(&updated_component.content));
                                web_sys::console::log_1(&format!("Updated element (fallback) with new content: '{}'", updated_component.content).into());
                            }
                        }
                    }
                }
            }
            
            // Now persist the changes to the backend
            let updated_component_clone = updated_component.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // Check authentication first
                if !is_authenticated() {
                    web_sys::console::log_1(&"⚠️ Cannot save changes: User not authenticated. Please log in to the admin panel first.".into());
                    return;
                }
                
                match get_auth_token() {
                    Ok(token) => {
                        web_sys::console::log_1(&format!("✅ Authentication token found, proceeding with save...").into());
                    }
                    Err(_) => {
                        web_sys::console::log_1(&"❌ Failed to get authentication token. Please log in to the admin panel.".into());
                        return;
                    }
                }
                
                // Get the current page slug from the URL
                if let Some(window) = window() {
                    if let Some(location) = window.location().pathname().ok() {
                        let slug = if location == "/" { "home" } else { location.trim_start_matches('/') };
                        
                        web_sys::console::log_1(&format!("💾 Saving changes for page slug: {}", slug).into());
                        
                        // Fetch the current page data
                        match get_page_by_slug(slug).await {
                            Ok(page) => {
                                // Parse the current page content to get components
                                match serde_json::from_str::<Vec<PageComponent>>(&page.content) {
                                    Ok(mut components) => {
                                        // Find and update the component
                                        if let Some(component_index) = components.iter().position(|c| c.id == updated_component_clone.id) {
                                            components[component_index] = updated_component_clone.clone();
                                            
                                            // Serialize back to JSON
                                            match serde_json::to_string(&components) {
                                                Ok(updated_content) => {
                                                    // Save to backend using the new content-only endpoint
                                                    if let Some(page_id) = page.id {
                                                        match update_page_content(page_id, &updated_content).await {
                                                            Ok(_) => {
                                                                web_sys::console::log_1(&"🎉 Successfully saved component changes to backend! Changes will persist on page reload.".into());
                                                            }
                                                            Err(e) => {
                                                                web_sys::console::log_1(&format!("❌ Failed to save to backend: {:?}. Make sure you're logged in.", e).into());
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    web_sys::console::log_1(&format!("Failed to serialize updated content: {:?}", e).into());
                                                }
                                            }
                                        } else {
                                            web_sys::console::log_1(&format!("Component with ID {} not found in page content", updated_component_clone.id).into());
                                        }
                                    }
                                    Err(e) => {
                                        web_sys::console::log_1(&format!("Failed to parse page content JSON: {:?}", e).into());
                                    }
                                }
                            }
                            Err(e) => {
                                web_sys::console::log_1(&format!("Failed to fetch current page: {:?}", e).into());
                            }
                        }
                    }
                }
            });
            
            // Close the properties panel after saving
            selected_target.set(None);
            show_properties_panel.set(false);
        })
    };

    let on_template_updated = {
        let component_templates = props.component_templates.clone();
        let on_templates_updated = props.on_templates_updated.clone();
        
        Callback::from(move |updated_template: ComponentTemplate| {
            web_sys::console::log_1(&format!("Template update callback received: ID {}, type {}", updated_template.id, updated_template.component_type).into());
            
            let mut updated_templates = component_templates.clone();
            
            // Find and update the template in the list
            if let Some(index) = updated_templates.iter().position(|t| t.id == updated_template.id) {
                web_sys::console::log_1(&format!("Found template at index {}, updating...", index).into());
                updated_templates[index] = updated_template.clone();
            } else {
                web_sys::console::log_1(&format!("Template with ID {} not found in list!", updated_template.id).into());
            }
            
            // Notify parent of changes
            web_sys::console::log_1(&"Emitting updated templates to parent".into());
            on_templates_updated.emit(updated_templates);
        })
    };

    if !props.enabled {
        return html! {};
    }

    html! {
        <div class="enhanced-live-edit-system">
            // Live edit indicator
            <div style="position: fixed; top: 10px; right: 10px; z-index: 10000;">
                <div style="background: rgba(0,123,255,0.9); color: white; padding: 8px 12px; border-radius: 6px; font-size: 12px; font-weight: 600; box-shadow: 0 2px 8px rgba(0,0,0,0.2);">
                    {"🎨 Enhanced Live Edit Mode - Click any component to edit"}
                </div>
            </div>

            // Properties panel
            {
                if *show_properties_panel {
                    if let Some(target) = &*selected_target {
                        match target {
                            EditTarget::Header => {
                                if let Some(template) = props.component_templates.iter().find(|t| t.component_type == "header" && t.is_active) {
                                    html! {
                                        <UnifiedPropertiesPanel
                                            component={None}
                                            template_data={Some(template.template_data.clone())}
                                            template_id={Some(template.id)}
                                            template_name={Some(template.name.clone())}
                                            panel_type={PanelType::HeaderTemplate}
                                            on_component_updated={None::<Callback<PageComponent>>}
                                            on_template_updated={Some(on_template_updated.clone())}
                                            on_close={on_close_panel.clone()}
                                        />
                                    }
                                } else {
                                    html! { <p>{"No header template found"}</p> }
                                }
                            }
                            EditTarget::Footer => {
                                if let Some(template) = props.component_templates.iter().find(|t| t.component_type == "footer" && t.is_active) {
                                    html! {
                                        <UnifiedPropertiesPanel
                                            component={None}
                                            template_data={Some(template.template_data.clone())}
                                            template_id={Some(template.id)}
                                            template_name={Some(template.name.clone())}
                                            panel_type={PanelType::FooterTemplate}
                                            on_component_updated={None::<Callback<PageComponent>>}
                                            on_template_updated={Some(on_template_updated.clone())}
                                            on_close={on_close_panel.clone()}
                                        />
                                    }
                                } else {
                                    html! { <p>{"No footer template found"}</p> }
                                }
                            }
                            EditTarget::Container => {
                                if let Some(template) = props.component_templates.iter().find(|t| t.component_type == "container" && t.is_active) {
                                    html! {
                                        <UnifiedPropertiesPanel
                                            component={None}
                                            template_data={Some(template.template_data.clone())}
                                            template_id={Some(template.id)}
                                            template_name={Some(template.name.clone())}
                                            panel_type={PanelType::ContainerTemplate}
                                            on_component_updated={None::<Callback<PageComponent>>}
                                            on_template_updated={Some(on_template_updated.clone())}
                                            on_close={on_close_panel.clone()}
                                        />
                                    }
                                } else {
                                    html! { <p>{"No container template found"}</p> }
                                }
                            }
                            EditTarget::PageComponent(component) => {
                                html! {
                                    <UnifiedPropertiesPanel
                                        component={Some(component.clone())}
                                                                                    template_data={None}
                                            template_id={None::<i32>}
                                            template_name={None::<String>}
                                            panel_type={PanelType::PageComponent}
                                            on_component_updated={Some(on_component_updated.clone())}
                                            on_template_updated={None::<Callback<ComponentTemplate>>}
                                            on_close={on_close_panel.clone()}
                                    />
                                }
                            }
                        }
                    } else {
                        html! { <span></span> }
                    }
                } else {
                    html! { <span></span> }
                }
            }
        </div>
    }
}

// Helper function to detect component type from DOM element
fn detect_component_type_from_element(event: &MouseEvent) -> ComponentType {
    if let Some(target) = event.target().and_then(|t| t.dyn_into::<Element>().ok()) {
        if let Some(class_name) = target.get_attribute("class") {
            if class_name.contains("hero") {
                return ComponentType::Hero;
            } else if class_name.contains("card") {
                return ComponentType::Card;
            } else if class_name.contains("button") {
                return ComponentType::Button;
            } else if class_name.contains("image") || class_name.contains("img") {
                return ComponentType::Image;
            } else if class_name.contains("video") {
                return ComponentType::Video;
            } else if class_name.contains("divider") {
                return ComponentType::Divider;
            } else if class_name.contains("heading") || target.tag_name() == "H1" || target.tag_name() == "H2" {
                return ComponentType::Heading;
            }
        }
        
        // Check tag name as fallback
        match target.tag_name().as_str() {
            "IMG" => ComponentType::Image,
            "VIDEO" => ComponentType::Video,
            "BUTTON" | "A" => ComponentType::Button,
            "H1" | "H2" | "H3" => ComponentType::Heading,
            "H4" | "H5" | "H6" => ComponentType::Subheading,
            "HR" => ComponentType::Divider,
            _ => ComponentType::Text,
        }
    } else {
        ComponentType::Text
    }
}

// Helper function to apply real-time template style updates
pub fn apply_template_style_preview(component_type: &str, template_data: &serde_json::Value) {
    use wasm_bindgen::JsCast;
    use web_sys::{window, HtmlElement};
    
    if let Some(window) = window() {
        if let Ok(document) = window.document().ok_or("No document") {
            let element_id = match component_type {
                "header" => "site-header",
                "footer" => "site-footer",
                _ => return,
            };
            
            if let Some(element) = document.get_element_by_id(element_id) {
                if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
                    let mut styles = Vec::new();
                    
                    // Handle height property
                    if let Some(height) = template_data.get("height").and_then(|v| v.as_str()) {
                        let mut h = height.to_string();
                        
                        // Ensure height has px units
                        if !h.ends_with("px") && !h.ends_with("%") && !h.ends_with("em") && !h.ends_with("rem") {
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
                        
                        // Debug logging for height changes
                        web_sys::console::log_1(&format!("🎨 Live Preview: Setting {} height to {}", component_type, h).into());
                        
                        styles.push(format!("height: {}", h));
                    }
                    
                    // Handle background properties
                    let bg_type = template_data.get("bg_type").and_then(|v| v.as_str()).unwrap_or("color");
                    
                    match bg_type {
                        "color" => {
                            if let Some(mut bg_color) = template_data.get("bg_color").and_then(|v| v.as_str()) {
                                // For header, coerce white to black per default theme requirement
                                if component_type == "header" && bg_color.trim().eq_ignore_ascii_case("#ffffff") {
                                    bg_color = "#000000";
                                }
                                styles.push(format!("background-color: {}", bg_color));
                            }
                        },
                        "image" => {
                            if let Some(bg_image) = template_data.get("bg_image").and_then(|v| v.as_str()) {
                                styles.push(format!("background-image: url({})", bg_image));
                                styles.push("background-size: cover".to_string());
                                styles.push("background-position: center".to_string());
                                styles.push("background-repeat: no-repeat".to_string());
                            }
                        },
                        "gradient" => {
                            let start_color = template_data.get("bg_gradient_start").and_then(|v| v.as_str()).unwrap_or("#ffffff");
                            let end_color = template_data.get("bg_gradient_end").and_then(|v| v.as_str()).unwrap_or("#f0f0f0");
                            let direction = template_data.get("bg_gradient_direction").and_then(|v| v.as_str()).unwrap_or("to-right");
                            styles.push(format!("background: linear-gradient({}, {}, {})", direction, start_color, end_color));
                        },
                        "video" => {
                            if let Some(bg_video) = template_data.get("bg_video").and_then(|v| v.as_str()) {
                                styles.push("background-color: #000000".to_string());
                                styles.push("position: relative".to_string());
                                styles.push(format!("--bg-video-url: '{}'", bg_video));
                            }
                        },
                        _ => {}
                    }
                    
                    // Handle text color
                    if let Some(text_color) = template_data.get("text_color").and_then(|v| v.as_str()) {
                        if component_type == "header" {
                            styles.push(format!("--header-text: {}", text_color));
                        } else if component_type == "footer" {
                            styles.push(format!("--footer-text: {}", text_color));
                        }
                    }
                    
                    // Handle shape masks with improved approach
                    let shape_mask_upper = template_data.get("shape_mask_upper").and_then(|v| v.as_str()).unwrap_or("none");
                    let shape_mask_upper_scale = template_data.get("shape_mask_upper_scale").and_then(|v| v.as_str()).unwrap_or("100");
                    let shape_mask_lower = template_data.get("shape_mask_lower").and_then(|v| v.as_str()).unwrap_or("none");
                    let shape_mask_lower_scale = template_data.get("shape_mask_lower_scale").and_then(|v| v.as_str()).unwrap_or("100");
                    
                    // Debug logging
                    web_sys::console::log_1(&format!("Shape mask preview - Upper: {}, Scale: {}, Lower: {}, Scale: {}", 
                        shape_mask_upper, shape_mask_upper_scale, shape_mask_lower, shape_mask_lower_scale).into());
                    
                    // Get additional shape parameters
                    let shape_mask_upper_frequency = template_data.get("shape_mask_upper_frequency").and_then(|v| v.as_str()).unwrap_or("2");
                    let shape_mask_upper_direction = template_data.get("shape_mask_upper_direction").and_then(|v| v.as_str()).unwrap_or("positive");
                    let shape_mask_upper_amplitude = template_data.get("shape_mask_upper_amplitude").and_then(|v| v.as_str()).unwrap_or("50");
                    let shape_mask_upper_degrees = template_data.get("shape_mask_upper_degrees").and_then(|v| v.as_str()).unwrap_or("15");
                    let shape_mask_lower_frequency = template_data.get("shape_mask_lower_frequency").and_then(|v| v.as_str()).unwrap_or("2");
                    let shape_mask_lower_direction = template_data.get("shape_mask_lower_direction").and_then(|v| v.as_str()).unwrap_or("positive");
                    let shape_mask_lower_amplitude = template_data.get("shape_mask_lower_amplitude").and_then(|v| v.as_str()).unwrap_or("50");
                    let shape_mask_lower_degrees = template_data.get("shape_mask_lower_degrees").and_then(|v| v.as_str()).unwrap_or("15");
                    
                    // Apply shape masks using direct DOM manipulation
                    apply_shape_masks_to_element(
                        &html_element, 
                        shape_mask_upper, shape_mask_upper_scale, shape_mask_upper_frequency, shape_mask_upper_direction, shape_mask_upper_amplitude, shape_mask_upper_degrees,
                        shape_mask_lower, shape_mask_lower_scale, shape_mask_lower_frequency, shape_mask_lower_direction, shape_mask_lower_amplitude, shape_mask_lower_degrees
                    );
                    
                    // Apply the styles
                    let current_style = html_element.get_attribute("style").unwrap_or_default();
                    web_sys::console::log_1(&format!("🔍 Live Preview: Existing styles for {}: {}", element_id, current_style).into());
                    
                    let mut existing_styles: Vec<String> = current_style
                        .split(';')
                        .filter(|s| !s.trim().is_empty())
                        .filter(|s| {
                            // Remove existing height, background, color, and shape mask properties to avoid conflicts
                            let s = s.trim();
                            let should_keep = !s.starts_with("height:") && 
                                !s.starts_with("background:") && 
                                !s.starts_with("background-color:") && 
                                !s.starts_with("background-image:") && 
                                !s.starts_with("background-size:") && 
                                !s.starts_with("background-position:") && 
                                !s.starts_with("background-repeat:") && 
                                !s.starts_with("--header-text:") && 
                                !s.starts_with("--footer-text:");
                            
                            if !should_keep {
                                web_sys::console::log_1(&format!("🗑️ Live Preview: Removing conflicting style: {}", s).into());
                            }
                            should_keep
                        })
                        .map(|s| s.to_string())
                        .collect();
                    
                    // Add new styles
                    web_sys::console::log_1(&format!("➕ Live Preview: Adding new styles: {:?}", styles).into());
                    existing_styles.extend(styles.clone());
                    
                    let new_style = existing_styles.join("; ");
                    
                    // Debug logging for applied styles
                    web_sys::console::log_1(&format!("✅ Live Preview: Final styles for {}: {}", element_id, new_style).into());
                    
                    let _ = html_element.set_attribute("style", &new_style);
                }
            }
        }
    }
}

// Helper function to create a sample component for editing
fn create_sample_component_from_element(component_type: ComponentType, component_id: &str, element: &Element) -> PageComponent {
    // Extract actual text content from the element
    let content = element.text_content().unwrap_or_else(|| "Sample content".to_string());
    
    PageComponent {
        id: component_id.to_string(),
        component_type,
        content,
        styles: Default::default(),
        position: Default::default(),
        properties: ComponentProperties::default(),
    }
}

// Enhanced shape mask application using CSS clip-path for proper cutout effects
pub fn apply_shape_masks_to_element(
    element: &web_sys::HtmlElement, 
    shape_upper: &str, 
    scale_upper: &str, 
    frequency_upper: &str,
    direction_upper: &str,
    amplitude_upper: &str,
    degrees_upper: &str,
    shape_lower: &str, 
    scale_lower: &str,
    frequency_lower: &str,
    direction_lower: &str,
    amplitude_lower: &str,
    degrees_lower: &str
) {
    // Set data attributes for CSS targeting
    if shape_upper != "none" {
        let _ = element.set_attribute("data-shape-top", "active");
    } else {
        let _ = element.remove_attribute("data-shape-top");
    }
    
    if shape_lower != "none" {
        let _ = element.set_attribute("data-shape-bottom", "active");
    } else {
        let _ = element.remove_attribute("data-shape-bottom");
    }
    
    // Debug logging before generating clip-path
    web_sys::console::log_1(&format!("🎭 Applying shapes - Upper: '{}', Lower: '{}'", shape_upper, shape_lower).into());
    
    // Clear any existing clip-path first to prevent interference
    let current_style = element.get_attribute("style").unwrap_or_default();
    let cleaned_style = current_style
        .split(';')
        .filter(|s| {
            let trimmed = s.trim();
            !trimmed.starts_with("clip-path") && 
            !trimmed.starts_with("-webkit-clip-path") && 
            !trimmed.is_empty()
        })
        .collect::<Vec<&str>>()
        .join("; ");
    
    // Force clear any cached clip-path by temporarily setting it to none
    let _ = element.set_attribute("style", &format!("{};clip-path:none;-webkit-clip-path:none", cleaned_style));
    
    // Also clear any data attributes that might be caching shape info
    let _ = element.remove_attribute("data-shape-cache");
    let _ = element.remove_attribute("data-last-clip-path");
    
    // Force a style recalculation by toggling a harmless property
    let _ = element.set_attribute("data-shape-clearing", "true");
    let _ = element.remove_attribute("data-shape-clearing");
    
    web_sys::console::log_1(&"🧹 Aggressively cleared existing clip-path and cache".into());
    
    // Generate clip-path for the element
    let clip_path = generate_clip_path(
        shape_upper, scale_upper, frequency_upper, direction_upper, amplitude_upper, degrees_upper,
        shape_lower, scale_lower, frequency_lower, direction_lower, amplitude_lower, degrees_lower
    );
    
    web_sys::console::log_1(&format!("🎭 Generated clip-path: '{}'", clip_path).into());
    
    // Apply clip-path directly to the element using the cleaned style
    let mut existing_styles: Vec<String> = if cleaned_style.is_empty() {
        Vec::new()
    } else {
        cleaned_style
            .split(';')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect()
    };
    
    if !clip_path.is_empty() {
        existing_styles.push(format!("clip-path: {}", clip_path));
    }
    
    let new_style = existing_styles.join("; ");
    let _ = element.set_attribute("style", &new_style);
    
    // Debug logging
    web_sys::console::log_1(&format!("🎭 Shape Mask Debug - Element: {}", element.id()).into());
    web_sys::console::log_1(&format!("  Upper: {} (freq: {}, scale: {})", shape_upper, frequency_upper, scale_upper).into());
    web_sys::console::log_1(&format!("  Lower: {} (freq: {}, scale: {})", shape_lower, frequency_lower, scale_lower).into());
    web_sys::console::log_1(&format!("  Generated clip-path: {}", clip_path).into());
}

// Generate CSS clip-path specifically for tilt shapes, completely isolated from curve logic
fn generate_tilt_only_clip_path(
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
            (tilt_amount, 0.0)
        } else {
            (0.0, tilt_amount)
        };
        
        polygon_points.push(format!("0% {}%", left_y));
        polygon_points.push(format!("100% {}%", right_y));
        
        web_sys::console::log_1(&format!("🎯 PURE TILT UPPER: degrees={}, left_y={}%, right_y={}%", degrees, left_y, right_y).into());
    } else {
        // Normal flat top
        polygon_points.push("0% 0%".to_string());
        polygon_points.push("100% 0%".to_string());
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
            (100.0, 100.0 - tilt_amount)
        } else {
            (100.0 - tilt_amount, 100.0)
        };
        
        polygon_points.push(format!("100% {}%", right_y));
        polygon_points.push(format!("0% {}%", left_y));
        
        web_sys::console::log_1(&format!("🎯 PURE TILT LOWER: degrees={}, right_y={}%, left_y={}%", degrees, right_y, left_y).into());
    } else {
        // Normal flat bottom
        polygon_points.push("100% 100%".to_string());
        polygon_points.push("0% 100%".to_string());
    }
    
    let clip_path = format!("polygon({})", polygon_points.join(", "));
    web_sys::console::log_1(&format!("🎯 PURE TILT CLIP-PATH: {}", clip_path).into());
    clip_path
}

// Generate CSS clip-path for shape masks
fn generate_clip_path(
    shape_upper: &str, scale_upper: &str, frequency_upper: &str, direction_upper: &str, amplitude_upper: &str, degrees_upper: &str,
    shape_lower: &str, scale_lower: &str, frequency_lower: &str, direction_lower: &str, amplitude_lower: &str, degrees_lower: &str
) -> String {
    // CRITICAL: If either shape is tilt, completely bypass all curve logic
    if shape_upper == "tilt" || shape_lower == "tilt" {
        web_sys::console::log_1(&format!("🎯 TILT DETECTED - Bypassing all curve logic. Upper: '{}', Lower: '{}'", shape_upper, shape_lower).into());
        return generate_tilt_only_clip_path(shape_upper, degrees_upper, direction_upper, shape_lower, degrees_lower, direction_lower);
    }
    
    // Create proper polygon without duplicate points
    let mut polygon_points = vec!["0% 0%".to_string()]; // Start at top-left
    
    // Add upper shape points (left to right along top edge) - but skip tilt shapes
    if shape_upper != "none" && !shape_upper.is_empty() && shape_upper != "tilt" {
        let scale_factor = scale_upper.parse::<f32>().unwrap_or(100.0) / 100.0;
        let frequency = frequency_upper.parse::<f32>().unwrap_or(2.0);
        let amplitude = amplitude_upper.parse::<f32>().ok();
        let degrees = degrees_upper.parse::<f32>().unwrap_or(15.0);
        let upper_points = generate_shape_points_new(shape_upper, scale_factor, frequency, true, Some(direction_upper), amplitude, Some(degrees));
        polygon_points.extend(upper_points);
        
        // Debug logging for upper shape
        web_sys::console::log_1(&format!("🔺 Upper shape applied: {} (scale: {}, freq: {}, dir: {}, amp: {:?}, deg: {})", 
            shape_upper, scale_factor, frequency, direction_upper, amplitude, degrees).into());
    } else {
        web_sys::console::log_1(&format!("🔺 Upper shape skipped: '{}'", shape_upper).into());
    }
    
    // Handle tilt shapes specially for upper edge
    if shape_upper == "tilt" {
        let degrees = degrees_upper.parse::<f32>().unwrap_or(15.0);
        let is_left = direction_upper == "left";
        
        // Special handling for edge cases
        let tilt_amount = if degrees <= 0.5 {
            // For very small angles (including 0°), ensure we get a flat line, not a curve
            0.0
        } else {
            // Use a linear relationship for more predictable tilt
            degrees.min(45.0) * 0.8 // Scale down slightly, max 36% at 45°
        };
        
        // Always use positive values and adjust the polygon structure instead
        let (left_y, right_y) = if is_left {
            (tilt_amount, 0.0) // Left corner goes down, right stays at 0
        } else {
            (0.0, tilt_amount) // Left stays at 0, right corner goes down
        };
        
        polygon_points.pop(); // Remove the "0% 0%" we added initially
        polygon_points.push(format!("0% {}%", left_y)); // Left corner
        polygon_points.push(format!("100% {}%", right_y)); // Right corner
        
        web_sys::console::log_1(&format!("🔺 TILT OVERRIDE: degrees={}, tilt_amount={}, left_y={}%, right_y={}%", 
            degrees, tilt_amount, left_y, right_y).into());
    } else {
        // Add normal top-right corner
        polygon_points.push("100% 0%".to_string());
    }
    
    // Handle tilt shapes specially for lower edge  
    if shape_lower == "tilt" {
        let degrees = degrees_lower.parse::<f32>().unwrap_or(15.0);
        let is_left = direction_lower == "left";
        
        // Special handling for edge cases
        let tilt_amount = if degrees <= 0.5 {
            // For very small angles (including 0°), ensure we get a flat line, not a curve
            0.0
        } else {
            // Use a linear relationship for more predictable tilt
            degrees.min(45.0) * 0.8 // Scale down slightly, max 36% at 45°
        };
        
        // Always use positive values relative to 100%
        let (right_y, left_y) = if is_left {
            (100.0, 100.0 - tilt_amount) // Right stays at 100%, left goes up
        } else {
            (100.0 - tilt_amount, 100.0) // Right goes up, left stays at 100%
        };
        
        // Add right edge to bottom-right corner
        polygon_points.push(format!("100% {}%", right_y));
        
        // Add tilted bottom-left corner
        polygon_points.push(format!("0% {}%", left_y));
        
        web_sys::console::log_1(&format!("🔻 TILT OVERRIDE: degrees={}, tilt_amount={}, right_y={}%, left_y={}%", 
            degrees, tilt_amount, right_y, left_y).into());
    } else {
        // Add right edge to bottom-right
        polygon_points.push("100% 100%".to_string());
        
        // Add lower shape points (right to left along bottom edge) - but skip tilt shapes
        if shape_lower != "none" && !shape_lower.is_empty() && shape_lower != "tilt" {
            let scale_factor = scale_lower.parse::<f32>().unwrap_or(100.0) / 100.0;
            let frequency = frequency_lower.parse::<f32>().unwrap_or(2.0);
            let amplitude = amplitude_lower.parse::<f32>().ok();
            let degrees = degrees_lower.parse::<f32>().unwrap_or(15.0);
            let mut lower_points = generate_shape_points_new(shape_lower, scale_factor, frequency, false, Some(direction_lower), amplitude, Some(degrees));
            lower_points.reverse(); // Reverse for right-to-left traversal
            polygon_points.extend(lower_points);
            
            // Debug logging for lower shape
            web_sys::console::log_1(&format!("🔻 Lower shape applied: {} (scale: {}, freq: {}, dir: {}, amp: {:?}, deg: {})", 
                shape_lower, scale_factor, frequency, direction_lower, amplitude, degrees).into());
        } else {
            web_sys::console::log_1(&format!("🔻 Lower shape skipped: '{}'", shape_lower).into());
        }
        
        // Add normal bottom-left corner to close the polygon
        polygon_points.push("0% 100%".to_string());
    }
    
    if shape_upper != "none" || shape_lower != "none" {
        format!("polygon({})", polygon_points.join(", "))
    } else {
        String::new()
    }
}

// Generate shape points for CSS polygon clip-path
fn generate_shape_points(shape_type: &str, scale: f32, frequency: f32, is_top: bool) -> Vec<String> {
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
        "triangle" => {
            vec![
                if is_top {
                    format!("50% {}%", depth)
                } else {
                    format!("50% {}%", 100.0 - depth)
                }
            ]
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

// New shape generation function with enhanced parameters
fn generate_shape_points_new(
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
                
                // For negative curves, we need to invert the curve direction properly
                let y = if is_positive {
                    // Positive curve: normal behavior
                    if is_top { base_y + curve_height } else { base_y - curve_height }
                } else {
                    // Negative curve: invert the curve direction
                    if is_top { base_y - curve_height } else { base_y + curve_height }
                };
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
            let tilt_degrees = degrees.unwrap_or(15.0);
            let is_left = direction.unwrap_or("right") == "left";
            
            // Debug logging for tilt
            web_sys::console::log_1(&format!("🔧 Tilt generation: degrees={}, direction={}, is_top={}, is_left={}", 
                tilt_degrees, direction.unwrap_or("right"), is_top, is_left).into());
            
            // For tilt, we return EMPTY points because tilt needs special handling in the polygon construction
            // The tilt will be handled by modifying the corner points directly
            
            Vec::new()
        },
        _ => Vec::new(),
    }
}
