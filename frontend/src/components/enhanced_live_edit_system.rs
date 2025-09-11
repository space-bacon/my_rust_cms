use yew::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{window, Element, MouseEvent};

// Helper functions for class manipulation
fn add_class(element: &Element, class_name: &str) {
    let current_class = element.class_name();
    if !current_class.contains(class_name) {
        let new_class = if current_class.is_empty() {
            class_name.to_string()
        } else {
            format!("{} {}", current_class, class_name)
        };
        element.set_class_name(&new_class);
    }
}

fn remove_class(element: &Element, class_name: &str) {
    let current_class = element.class_name();
    if current_class.contains(class_name) {
        let new_class = current_class
            .split_whitespace()
            .filter(|&c| c != class_name)
            .collect::<Vec<&str>>()
            .join(" ");
        element.set_class_name(&new_class);
    }
}

fn has_class(element: &Element, class_name: &str) -> bool {
    element.class_name().split_whitespace().any(|c| c == class_name)
}

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
                    // Add live-edit-mode class to body for global styling
                    if let Some(body) = doc.body() {
                        if let Ok(element) = body.dyn_into::<Element>() {
                            add_class(&element, "live-edit-mode");
                        }
                    }
                    
                    // Add highlighting to header, footer, and container
                    for (id, target_type) in [
                        ("site-header", EditTarget::Header),
                        ("site-footer", EditTarget::Footer),
                        ("site-container", EditTarget::Container),
                    ] {
                        if let Some(el) = doc.get_element_by_id(id) {
                            let _ = el.set_attribute("data-live-editable", "true");
                            
                            // Add click handler
                            let selected_target_clone = selected_target.clone();
                            let show_properties_panel_clone = show_properties_panel.clone();
                            let target_type_clone = target_type.clone();
                            
                            let closure: Closure<dyn FnMut(web_sys::Event)> = Closure::wrap(Box::new(move |e: web_sys::Event| {
                                e.stop_propagation();
                                
                                // Remove selected class from all elements
                                if let Some(doc) = window().and_then(|w| w.document()) {
                                    let selected_elements = doc.query_selector_all("[data-live-editable].selected, [data-live-component-editable].selected").unwrap();
                                    for i in 0..selected_elements.length() {
                                        if let Some(node) = selected_elements.item(i) {
                                            if let Ok(el) = node.dyn_into::<Element>() {
                                                remove_class(&el, "selected");
                                            }
                                        }
                                    }
                                    
                                    // Add selected class to clicked element
                                    if let Some(target) = e.target() {
                                        if let Ok(el) = target.dyn_into::<Element>() {
                                            add_class(&el, "selected");
                                        }
                                    }
                                }
                                
                                selected_target_clone.set(Some(target_type_clone.clone()));
                                show_properties_panel_clone.set(true);
                            }) as Box<dyn FnMut(_)>);
                            
                            let _ = el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
                            closure.forget();
                        }
                    }

                    // Add highlighting to page builder components, excluding hidden ones
                    let components_with_class = doc.query_selector_all(".component, .canvas-component").unwrap();
                    web_sys::console::log_1(&format!("Live Edit: Found {} components to make editable", components_with_class.length()).into());
                    
                    for i in 0..components_with_class.length() {
                        if let Some(node) = components_with_class.item(i) {
                            if let Ok(el) = node.dyn_into::<Element>() {
                                // Skip hidden elements
                                if let Some(style) = el.get_attribute("style") {
                                    if style.contains("display: none") || style.contains("display:none") {
                                        web_sys::console::log_1(&format!("Live Edit: Skipping hidden component {}", i).into());
                                        continue;
                                    }
                                }
                                
                                let _ = el.set_attribute("data-live-component-editable", "true");
                                
                                // Apply consistent highlighting style (same as header/footer)
                                // Also add a debug background for Hero components to make them visible
                                let debug_style = if el.class_name().contains("hero-section") {
                                    "outline: 2px dashed rgba(255,0,0,0.8); outline-offset: -2px; cursor: pointer; background: rgba(255,0,0,0.1) !important;"
                                } else {
                                    "outline: 2px dashed rgba(0,150,255,0.8); outline-offset: -2px; cursor: pointer;"
                                };
                                
                                if let Some(existing) = el.get_attribute("style") {
                                    let _ = el.set_attribute("style", &format!("{}; {}", existing, debug_style));
                                } else {
                                    let _ = el.set_attribute("style", debug_style);
                                }

                                // Add component index for selection
                                let component_index = i.to_string();
                                let _ = el.set_attribute("data-component-index", &component_index);
                                
                                // Debug logging for each component
                                let class_name = el.class_name();
                                let tag_name = el.tag_name();
                                web_sys::console::log_1(&format!("Live Edit: Made component {} editable - class: '{}', tag: '{}'", 
                                    component_index, class_name, tag_name).into());
                                
                                // Add click handler for page components
                                let selected_target_clone = selected_target.clone();
                                let show_properties_panel_clone = show_properties_panel.clone();
                                let page_components_clone = page_components.clone();
                                
                                let closure: Closure<dyn FnMut(MouseEvent)> = Closure::wrap(Box::new(move |e: MouseEvent| {
                                    e.stop_propagation();
                                    
                                    web_sys::console::log_1(&"Live Edit: Component clicked!".into());
                                    
                                    // Remove selected class from all elements first
                                    if let Some(doc) = window().and_then(|w| w.document()) {
                                        let selected_elements = doc.query_selector_all("[data-live-editable].selected, [data-live-component-editable].selected, .canvas-component.selected").unwrap();
                                        for j in 0..selected_elements.length() {
                                            if let Some(node) = selected_elements.item(j) {
                                                if let Ok(el) = node.dyn_into::<Element>() {
                                                    remove_class(&el, "selected");
                                                }
                                            }
                                        }
                                    }
                                    
                                    if let Some(target_element) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                                        web_sys::console::log_1(&format!("Live Edit: Click target - class: '{}', tag: '{}'", 
                                            target_element.class_name(), target_element.tag_name()).into());
                                        
                                        // Try to find the component index from the clicked element or its parents
                                        let mut current_element = Some(target_element.clone());
                                        let mut component_index: Option<usize> = None;
                                        let mut selected_element: Option<web_sys::Element> = None;
                                        
                                        while let Some(element) = current_element {
                                            // Check if this element has component data or is a canvas component
                                            let has_canvas_class = has_class(&element, "canvas-component");
                                            
                                            if element.has_attribute("data-component-index") || 
                                               has_canvas_class ||
                                               element.has_attribute("data-live-component-editable") {
                                                selected_element = Some(element.clone());
                                                
                                                if let Some(index_str) = element.get_attribute("data-component-index") {
                                                    web_sys::console::log_1(&format!("Live Edit: Found component index: {}", index_str).into());
                                                    if let Ok(index) = index_str.parse::<usize>() {
                                                        component_index = Some(index);
                                                        break;
                                                    }
                                                }
                                            }
                                            current_element = element.parent_element();
                                        }
                                        
                                        // Add selected class to the found element
                                        if let Some(el) = selected_element {
                                            add_class(&el, "selected");
                                        }
                                        
                                        if component_index.is_none() {
                                            web_sys::console::log_1(&"Live Edit: No component index found, searching parent elements".into());
                                        }
                                        
                                        // Try to find the actual component from the page components list
                                        web_sys::console::log_1(&format!("Live Edit: Total page components available: {}", page_components_clone.len()).into());
                                        
                                        // Debug: show all available components
                                        for (idx, comp) in page_components_clone.iter().enumerate() {
                                            web_sys::console::log_1(&format!("Live Edit: Component {}: {:?} ({}), content: '{}'", 
                                                idx, comp.component_type, comp.id, comp.content.chars().take(50).collect::<String>()).into());
                                        }
                                        
                                        if let Some(index) = component_index {
                                            web_sys::console::log_1(&format!("Live Edit: Looking for component at index: {}", index).into());
                                            if let Some(actual_component) = find_component_by_index(&page_components_clone, index) {
                                                // Debug logging
                                                web_sys::console::log_1(&format!("Live Edit: Found actual component at index {}: {:?} ({})", 
                                                    index, actual_component.component_type, actual_component.id).into());
                                                selected_target_clone.set(Some(EditTarget::PageComponent(actual_component)));
                                                show_properties_panel_clone.set(true);
                                                return;
                                            }
                                        }
                                        
                                        // Fallback: create a sample component if we can't find the actual one
                                        let component_type = detect_component_type_from_element(&e);
                                        let component_id = format!("live-component-{}", component_index.unwrap_or(0));
                                        let component = create_sample_component_from_element(component_type, &component_id, &target_element);
                                        
                                        // Debug logging for fallback
                                        web_sys::console::log_1(&format!("Live Edit: Using fallback component: {:?} ({})", 
                                            component.component_type, component.id).into());
                                        
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
                    // Remove live-edit-mode class from body
                    if let Some(body) = doc.body() {
                        if let Ok(element) = body.dyn_into::<Element>() {
                            remove_class(&element, "live-edit-mode");
                        }
                    }
                    
                    // Remove selected class from all elements
                    let selected_elements = doc.query_selector_all("[data-live-editable].selected, [data-live-component-editable].selected, .canvas-component.selected").unwrap();
                    for i in 0..selected_elements.length() {
                        if let Some(node) = selected_elements.item(i) {
                            if let Ok(el) = node.dyn_into::<Element>() {
                                remove_class(&el, "selected");
                            }
                        }
                    }
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
                                let _ = el.remove_attribute("data-component-index");
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
            
            // Apply live preview immediately
            apply_template_style_preview(&updated_template.component_type, &updated_template.template_data);
            
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
        // First check for specific component classes that the page builder uses
        if let Some(class_name) = target.get_attribute("class") {
            // Check for specific component classes
            if class_name.contains("hero-section") || class_name.contains("hero-component") {
                return ComponentType::Hero;
            } else if class_name.contains("card-component") || class_name.contains("feature-card") {
                return ComponentType::Card;
            } else if class_name.contains("two-column") || class_name.contains("columns-2") {
                return ComponentType::TwoColumn;
            } else if class_name.contains("three-column") || class_name.contains("columns-3") {
                return ComponentType::ThreeColumn;
            } else if class_name.contains("posts-list") || class_name.contains("post-list") {
                return ComponentType::PostsList;
            } else if class_name.contains("quote-component") || class_name.contains("blockquote") {
                return ComponentType::Quote;
            } else if class_name.contains("button-component") {
                return ComponentType::Button;
            } else if class_name.contains("image-component") || class_name.contains("img") {
                return ComponentType::Image;
            } else if class_name.contains("video-component") {
                return ComponentType::Video;
            } else if class_name.contains("divider-component") {
                return ComponentType::Divider;
            }
        }
        
        // Check for data attributes that might indicate component type
        if let Some(component_type) = target.get_attribute("data-component-type") {
            match component_type.as_str() {
                "hero" => return ComponentType::Hero,
                "card" => return ComponentType::Card,
                "two-column" => return ComponentType::TwoColumn,
                "three-column" => return ComponentType::ThreeColumn,
                "posts-list" => return ComponentType::PostsList,
                "quote" => return ComponentType::Quote,
                "button" => return ComponentType::Button,
                "image" => return ComponentType::Image,
                "video" => return ComponentType::Video,
                "divider" => return ComponentType::Divider,
                "heading" => return ComponentType::Heading,
                "subheading" => return ComponentType::Subheading,
                _ => {}
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
            "BLOCKQUOTE" => ComponentType::Quote,
            _ => {
                // Check parent elements for component context
                let mut current_element = Some(target.clone());
                while let Some(element) = current_element {
                    if let Some(class_name) = element.get_attribute("class") {
                        if class_name.contains("component") || class_name.contains("canvas-component") {
                            // This is likely a component wrapper, try to determine type from content
                            if element.query_selector("h1, h2, h3").ok().flatten().is_some() {
                                return ComponentType::Heading;
                            } else if element.query_selector("h4, h5, h6").ok().flatten().is_some() {
                                return ComponentType::Subheading;
                            } else if element.query_selector("blockquote").ok().flatten().is_some() {
                                return ComponentType::Quote;
                            } else if element.query_selector(".card, .feature-card").ok().flatten().is_some() {
                                return ComponentType::Card;
                            } else if element.query_selector(".columns, .two-column").ok().flatten().is_some() {
                                return ComponentType::TwoColumn;
                            } else if element.query_selector(".posts-list, .post-list").ok().flatten().is_some() {
                                return ComponentType::PostsList;
                            }
                            break;
                        }
                    }
                    current_element = element.parent_element();
                }
                ComponentType::Text
            }
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
                "container" => "site-container",
                _ => return,
            };
            
            if let Some(element) = document.get_element_by_id(element_id) {
                // CRITICAL: Disable scroll effects during live editing to prevent interference
                if component_type == "header" {
                    let _ = element.set_attribute("data-live-edit-active", "true");
                    web_sys::console::log_1(&"🚫 Live Edit: Temporarily disabled scroll effects for header".into());
                }
                
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
                    
                    web_sys::console::log_1(&format!("🎨 Live Preview: Background type is '{}' for {}", bg_type, component_type).into());
                    web_sys::console::log_1(&format!("🎨 Live Preview: Template data: {}", serde_json::to_string_pretty(template_data).unwrap_or_else(|_| "Failed to serialize".to_string())).into());
                    
                    match bg_type {
                        "color" => {
                            if let Some(mut bg_color) = template_data.get("bg_color").and_then(|v| v.as_str()) {
                                // For header, coerce white to black per default theme requirement
                                if component_type == "header" && bg_color.trim().eq_ignore_ascii_case("#ffffff") {
                                    bg_color = "#000000";
                                }
                                styles.push(format!("background-color: {} !important", bg_color));
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
                            // Check for custom gradient first
                            if let Some(custom_gradient) = template_data.get("bg_gradient_custom").and_then(|v| v.as_str()) {
                                if !custom_gradient.trim().is_empty() {
                                    web_sys::console::log_1(&format!("🎨 Live Preview: Using custom gradient: {}", custom_gradient).into());
                                    styles.push(format!("background: {} !important", custom_gradient));
                                } else {
                                    // Fallback to individual fields if custom is empty
                                    let start_color = template_data.get("bg_gradient_start").and_then(|v| v.as_str()).unwrap_or("#667eea");
                                    let end_color = template_data.get("bg_gradient_end").and_then(|v| v.as_str()).unwrap_or("#764ba2");
                                    let direction = template_data.get("bg_gradient_direction").and_then(|v| v.as_str()).unwrap_or("135deg");
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
                                    web_sys::console::log_1(&format!("🎨 Live Preview: Using individual gradient fields: {}", gradient).into());
                                    styles.push(format!("background: {} !important", gradient));
                                }
                            } else {
                                // Fallback to individual fields if custom field is not present
                                let start_color = template_data.get("bg_gradient_start").and_then(|v| v.as_str()).unwrap_or("#667eea");
                                let end_color = template_data.get("bg_gradient_end").and_then(|v| v.as_str()).unwrap_or("#764ba2");
                                let direction = template_data.get("bg_gradient_direction").and_then(|v| v.as_str()).unwrap_or("135deg");
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
                                web_sys::console::log_1(&format!("🎨 Live Preview: Using fallback gradient fields: {}", gradient).into());
                                styles.push(format!("background: {} !important", gradient));
                            }
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
                    
                    // Handle header-specific position property
                    if component_type == "header" {
                        if let Some(position) = template_data.get("position").and_then(|v| v.as_str()) {
                            styles.push(format!("position: {} !important", position));
                            
                            // Adjust top property based on position
                            match position {
                                "fixed" | "sticky" => {
                                    styles.push("top: 0 !important".to_string());
                                }
                                "static" => {
                                    styles.push("top: auto !important".to_string());
                                    styles.push("position: static !important".to_string()); // Ensure static overrides
                                }
                                _ => {}
                            }
                            
                            // Add additional properties to ensure position works correctly
                            match position {
                                "static" => {
                                    // For static positioning, remove any transforms or z-index that might interfere
                                    styles.push("transform: none !important".to_string());
                                    styles.push("z-index: auto !important".to_string());
                                    // Remove any margin that might create gaps
                                    styles.push("margin-bottom: 0 !important".to_string());
                                }
                                "sticky" => {
                                    // Ensure sticky has proper z-index and no conflicting transforms
                                    styles.push("z-index: 100 !important".to_string());
                                    styles.push("transform: none !important".to_string());
                                }
                                "fixed" => {
                                    // Fixed positioning needs proper z-index and full width
                                    styles.push("z-index: 1000 !important".to_string());
                                    styles.push("left: 0 !important".to_string());
                                    styles.push("right: 0 !important".to_string());
                                    styles.push("width: 100% !important".to_string());
                                }
                                _ => {}
                            }
                            
                            web_sys::console::log_1(&format!("🎯 Live Preview: Setting header position to {} with enhanced properties", position).into());
                            
                            // Also adjust the main content area based on header position
                            if let Some(main_element) = document.get_element_by_id("site-main") {
                                let main_styles = match position {
                                    "static" => {
                                        // For static header, reduce top padding to avoid gap
                                        vec!["padding-top: 1rem !important".to_string()]
                                    }
                                    "sticky" | "fixed" => {
                                        // For sticky/fixed header, restore normal padding
                                        vec!["padding-top: 2rem !important".to_string()]
                                    }
                                    _ => vec![]
                                };
                                
                                if !main_styles.is_empty() {
                                    let current_main_style = main_element.get_attribute("style").unwrap_or_default();
                                    let mut existing_main_styles: Vec<String> = current_main_style
                                        .split(';')
                                        .filter(|s| !s.trim().is_empty())
                                        .filter(|s| !s.trim().starts_with("padding-top:"))
                                        .map(|s| s.trim().to_string())
                                        .collect();
                                    
                                    existing_main_styles.extend(main_styles);
                                    let new_main_style = existing_main_styles.join("; ");
                                    let _ = main_element.set_attribute("style", &new_main_style);
                                    
                                    web_sys::console::log_1(&format!("🎯 Live Preview: Adjusted main content padding for {} header", position).into());
                                }
                            }
                        }
                        
                        // Handle header scroll effects properties
                        if let Some(scroll_effect) = template_data.get("scroll_effect").and_then(|v| v.as_str()) {
                            if scroll_effect != "none" {
                                // Set CSS variables for scroll effects
                                if let Some(shrink_logo_scale) = template_data.get("shrink_logo_scale").and_then(|v| v.as_str()) {
                                    let scale_value = shrink_logo_scale.parse::<f64>().unwrap_or(80.0) / 100.0;
                                    
                                    // Store the logo scale as a CSS variable but don't apply it immediately
                                    // The scroll handler will apply it when the scroll threshold is reached
                                    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                                        if let Some(header) = document.get_element_by_id("site-header") {
                                            let current_style = header.get_attribute("style").unwrap_or_default();
                                            let mut style_parts: Vec<String> = current_style
                                                .split(';')
                                                .filter(|s| !s.trim().is_empty())
                                                .map(|s| s.trim().to_string())
                                                .collect();
                                            
                                            // Remove existing shrink logo scale variable
                                            style_parts.retain(|s| !s.starts_with("--shrink-logo-scale:"));
                                            
                                            // Store the target logo scale as a CSS variable for the scroll handler to use
                                            style_parts.push(format!("--shrink-logo-scale: {}", scale_value));
                                            
                                            // Check current scroll position to determine if we should apply the scale now
                                            let scroll_y = web_sys::window().and_then(|w| w.scroll_y().ok()).unwrap_or(0.0);
                                            let scroll_trigger = template_data.get("scroll_trigger")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("100")
                                                .parse::<f64>()
                                                .unwrap_or(100.0);
                                            
                                            // Only apply the logo scale if we're currently past the scroll trigger
                                            if scroll_y > scroll_trigger {
                                                // Remove existing logo scale
                                                style_parts.retain(|s| !s.starts_with("--logo-scale:"));
                                                // Apply the shrink scale since we're past the trigger
                                                style_parts.push(format!("--logo-scale: {}", scale_value));
                                            } else {
                                                // Reset to normal scale since we're above the trigger
                                                style_parts.retain(|s| !s.starts_with("--logo-scale:"));
                                                style_parts.push("--logo-scale: 1".to_string());
                                            }
                                            
                                            let new_style = style_parts.join("; ");
                                            let _ = header.set_attribute("style", &new_style);
                                            
                                            web_sys::console::log_1(&format!("🎨 Live Edit: Set header style to: {}", new_style).into());
                                        }
                                    }
                                    
                                    web_sys::console::log_1(&format!("🎨 Live Preview: Updated shrink logo scale to {}% (scale: {}) - will apply at scroll trigger", shrink_logo_scale, scale_value).into());
                                }
                                
                                // Handle scroll trigger
                                if let Some(scroll_trigger) = template_data.get("scroll_trigger").and_then(|v| v.as_str()) {
                                    styles.push(format!("--scroll-trigger: {}px", scroll_trigger));
                                    web_sys::console::log_1(&format!("🎨 Live Preview: Updated scroll trigger to {}px", scroll_trigger).into());
                                }
                                
                                // Handle scroll duration
                                if let Some(scroll_duration) = template_data.get("scroll_duration").and_then(|v| v.as_str()) {
                                    styles.push(format!("--scroll-duration: {}ms", scroll_duration));
                                    web_sys::console::log_1(&format!("🎨 Live Preview: Updated scroll duration to {}ms", scroll_duration).into());
                                }
                                
                                // Handle animation properties
                                if let Some(animation_type) = template_data.get("animation_type").and_then(|v| v.as_str()) {
                                    if animation_type != "none" {
                                        // Add animation CSS variables for pure CSS animations
                                        if let Some(duration) = template_data.get("animation_duration").and_then(|v| v.as_str()) {
                                            styles.push(format!("--intro-duration: {}", duration));
                                        }
                                        
                                        if let Some(delay) = template_data.get("animation_delay").and_then(|v| v.as_str()) {
                                            styles.push(format!("--intro-delay: {}", delay));
                                        }
                                        
                                        // Set easing to ease-out for smooth animations
                                        styles.push("--intro-easing: ease-out".to_string());
                                        
                                        web_sys::console::log_1(&format!("🎬 Live Edit: Adding {} CSS animation to {} template", animation_type, component_type).into());
                                    }
                                }
                                
                                // Handle scroll easing
                                if let Some(scroll_easing) = template_data.get("scroll_easing").and_then(|v| v.as_str()) {
                                    let css_easing = match scroll_easing {
                                        "linear" => "linear",
                                        "ease" => "ease",
                                        "ease-in" => "ease-in",
                                        "ease-out" => "ease-out", 
                                        "ease-in-out" => "ease-in-out",
                                        "elastic" => "cubic-bezier(0.68, -0.55, 0.265, 1.55)",
                                        "bounce" => "cubic-bezier(0.175, 0.885, 0.32, 1.275)",
                                        _ => "cubic-bezier(0.68, -0.55, 0.265, 1.55)"
                                    };
                                    styles.push(format!("--scroll-easing: {}", css_easing));
                                    web_sys::console::log_1(&format!("🎨 Live Preview: Updated scroll easing to {}", scroll_easing).into());
                                }
                                
                                // Handle shrink height for shrink scroll effect
                                if scroll_effect == "shrink" {
                                    if let Some(shrink_height) = template_data.get("shrink_height").and_then(|v| v.as_str()) {
                                        // Apply shrink height immediately for live preview if currently in shrunk state
                                        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                                            if let Some(header) = document.get_element_by_id("site-header") {
                                                let scroll_y = web_sys::window().and_then(|w| w.scroll_y().ok()).unwrap_or(0.0);
                                                let scroll_trigger = template_data.get("scroll_trigger")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("100")
                                                    .parse::<f64>()
                                                    .unwrap_or(100.0);
                                                
                                                // If currently scrolled past trigger, apply the new shrink height immediately
                                                if scroll_y > scroll_trigger {
                                                    let current_style = header.get_attribute("style").unwrap_or_default();
                                                    let mut style_parts: Vec<String> = current_style
                                                        .split(';')
                                                        .filter(|s| !s.trim().is_empty())
                                                        .map(|s| s.trim().to_string())
                                                        .collect();
                                                    
                                                    // Remove existing height
                                                    style_parts.retain(|s| !s.starts_with("height:"));
                                                    
                                                    // Add new shrink height
                                                    style_parts.push(format!("height: {}px", shrink_height));
                                                    
                                                    let new_style = style_parts.join("; ");
                                                    let _ = header.set_attribute("style", &new_style);
                                                }
                                            }
                                        }
                                        
                                        web_sys::console::log_1(&format!("🎨 Live Preview: Updated shrink height to {}px", shrink_height).into());
                                    }
                                }
                                
                                // Add scroll effect class for CSS targeting
                                if let Some(html_element) = document.get_element_by_id("site-header") {
                                    let current_class = html_element.get_attribute("class").unwrap_or_default();
                                    if !current_class.contains(&format!("scroll-effect-{}", scroll_effect)) {
                                        let new_class = format!("{} scroll-effect-{}", current_class, scroll_effect);
                                        let _ = html_element.set_attribute("class", &new_class);
                                    }
                                }
                                
                                web_sys::console::log_1(&format!("🎯 Live Preview: Applied scroll effect: {}", scroll_effect).into());
                            }
                        }
                        
                        // Handle logo properties for header
                        let logo_height = template_data.get("logo_height").and_then(|v| v.as_str());
                        let logo_width = template_data.get("logo_width").and_then(|v| v.as_str());
                        
                        if logo_height.is_some() || logo_width.is_some() {
                            // Find the logo container and apply dimensions
                            if let Some(logo_element) = document.query_selector(".site-logo").ok().flatten() {
                                if let Ok(logo_html) = logo_element.dyn_into::<HtmlElement>() {
                                    let current_style = logo_html.get_attribute("style").unwrap_or_default();
                                    
                                    // Parse existing styles and update dimensions
                                    let mut style_parts: Vec<String> = current_style
                                        .split(';')
                                        .filter(|s| {
                                            let s = s.trim();
                                            !s.is_empty() && !s.starts_with("height:") && !s.starts_with("width:")
                                        })
                                        .map(|s| s.trim().to_string())
                                        .collect();
                                    
                                    // Add the new dimensions
                                    if let Some(height) = logo_height {
                                        style_parts.push(format!("height: {}", height));
                                        web_sys::console::log_1(&format!("🎨 Live Preview: Updated logo height to {}", height).into());
                                    }
                                    if let Some(width) = logo_width {
                                        style_parts.push(format!("width: {}", width));
                                        web_sys::console::log_1(&format!("🎨 Live Preview: Updated logo width to {}", width).into());
                                    }
                                    
                                    let new_style = style_parts.join("; ");
                                    let _ = logo_html.set_attribute("style", &new_style);
                                }
                            }
                        }
                        
                        // Handle logo URL changes for header
                        if let Some(logo_url) = template_data.get("logo_url").and_then(|v| v.as_str()) {
                            // Find the logo image and update src
                            if let Some(logo_img) = document.query_selector(".site-logo img").ok().flatten() {
                                let _ = logo_img.set_attribute("src", logo_url);
                                web_sys::console::log_1(&format!("🎨 Live Preview: Updated logo URL to {}", logo_url).into());
                            }
                        }
                        
                        // Handle logo effects for header
                        if let Some(logo_effect) = template_data.get("logo_effect").and_then(|v| v.as_str()) {
                            if logo_effect == "pulsate" {
                                // Find the logo element and apply effects
                                if let Some(logo_element) = document.query_selector(".site-logo").ok().flatten() {
                                    // Check if logo is SVG and get src first (before consuming logo_element)
                                    let (is_svg, logo_src) = if let Some(img) = logo_element.query_selector("img").ok().flatten() {
                                        if let Some(src) = img.get_attribute("src") {
                                            let is_svg = src.to_lowercase().ends_with(".svg") || src.to_lowercase().contains("image/svg+xml");
                                            (is_svg, Some(src))
                                        } else { (false, None) }
                                    } else { (false, None) };
                                    
                                    if let Ok(logo_html) = logo_element.dyn_into::<HtmlElement>() {
                                        let mut logo_classes = vec!["site-logo"];
                                        let mut logo_css_vars = Vec::new();
                                        
                                        if is_svg {
                                            logo_classes.push("logo-effect-pulsate");
                                            logo_classes.push("has-svg-logo");
                                            
                                            // Add CSS variables for pulsate effect
                                            if let Some(frequency) = template_data.get("logo_pulsate_frequency").and_then(|v| v.as_str()) {
                                                logo_css_vars.push(format!("--logo-pulsate-frequency: {}", frequency));
                                            }
                                            if let Some(decay) = template_data.get("logo_pulsate_decay").and_then(|v| v.as_str()) {
                                                logo_css_vars.push(format!("--logo-pulsate-decay: {}", decay));
                                            }
                                            if let Some(opacity) = template_data.get("logo_pulsate_opacity").and_then(|v| v.as_str()) {
                                                let opacity_decimal = opacity.parse::<f32>().unwrap_or(70.0) / 100.0;
                                                logo_css_vars.push(format!("--logo-pulsate-opacity: {}", opacity_decimal));
                                            }
                                            if let Some(duration) = template_data.get("logo_pulsate_duration").and_then(|v| v.as_str()) {
                                                logo_css_vars.push(format!("--logo-pulsate-duration: {}", duration));
                                            }
                                            if let Some(anim_freq) = template_data.get("logo_pulsate_anim_frequency").and_then(|v| v.as_str()) {
                                                logo_css_vars.push(format!("--logo-pulsate-anim-frequency: {}", anim_freq));
                                            }
                                            
                                            // Add SVG URL for JavaScript processing
                                            if let Some(ref src) = logo_src {
                                                logo_css_vars.push(format!("--logo-svg-url: {}", src));
                                            }
                                            
                                            // Apply classes and styles to logo
                                            let logo_class_str = logo_classes.join(" ");
                                            let _ = logo_html.set_attribute("class", &logo_class_str);
                                            
                                            if !logo_css_vars.is_empty() {
                                                let current_logo_style = logo_html.get_attribute("style").unwrap_or_default();
                                                
                                                // Parse existing styles to preserve important properties like --logo-scale
                                                let mut style_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
                                                
                                                // Parse existing styles
                                                for style_pair in current_logo_style.split(';') {
                                                    if let Some((key, value)) = style_pair.split_once(':') {
                                                        let key = key.trim().to_lowercase();
                                                        let value = value.trim();
                                                        if !key.is_empty() && !value.is_empty() {
                                                            style_map.insert(key, value.to_string());
                                                        }
                                                    }
                                                }
                                                
                                                // Add new pulsate CSS variables
                                                for var in &logo_css_vars {
                                                    if let Some((key, value)) = var.split_once(':') {
                                                        let key = key.trim().to_lowercase();
                                                        let value = value.trim();
                                                        if !key.is_empty() && !value.is_empty() {
                                                            style_map.insert(key, value.to_string());
                                                        }
                                                    }
                                                }
                                                
                                                // Remove any --logo-scale from logo element so it inherits from header
                                                style_map.remove("--logo-scale");
                                                
                                                // Rebuild style string
                                                let new_logo_style: Vec<String> = style_map.iter()
                                                    .map(|(key, value)| format!("{}: {}", key, value))
                                                    .collect();
                                                let final_logo_style = new_logo_style.join("; ");
                                                
                                                let _ = logo_html.set_attribute("style", &final_logo_style);
                                                web_sys::console::log_1(&format!("🎨 Logo pulsate style applied: {}", final_logo_style).into());
                                            }
                                            
                                            // Create SVG ripples if we have an SVG logo
                                            if let Some(ref src) = logo_src {
                                                create_svg_ripples(&logo_html, src);
                                            }
                                            
                                            web_sys::console::log_1(&format!("🎯 Live Preview: Applied logo pulsate effect with vars: {:?}", logo_css_vars).into());
                                        } else {
                                            web_sys::console::log_1(&"⚠️ Live Preview: Logo effect only works with SVG images".into());
                                        }
                                    }
                                }
                            } else if logo_effect == "none" {
                                // Remove logo effects
                                if let Some(logo_element) = document.query_selector(".site-logo").ok().flatten() {
                                    if let Ok(logo_html) = logo_element.dyn_into::<HtmlElement>() {
                                        // Remove effect classes but keep site-logo class
                                        let current_class = logo_html.get_attribute("class").unwrap_or_default();
                                        let cleaned_classes: Vec<&str> = current_class
                                            .split_whitespace()
                                            .filter(|&class| class == "site-logo")
                                            .collect();
                                        let _ = logo_html.set_attribute("class", &cleaned_classes.join(" "));
                                        
                                        // Remove logo effect CSS variables and ripples
                                        let current_style = logo_html.get_attribute("style").unwrap_or_default();
                                        let cleaned_style = current_style
                                            .split(';')
                                            .filter(|s| !s.trim().starts_with("--logo-pulsate") && !s.trim().starts_with("--logo-svg"))
                                            .collect::<Vec<_>>()
                                            .join("; ");
                                        let _ = logo_html.set_attribute("style", &cleaned_style);
                                        
                                        // Remove existing ripples
                                        if let Some(ripples) = logo_html.query_selector(".logo-ripples").ok().flatten() {
                                            let _ = ripples.remove();
                                        }
                                        
                                        web_sys::console::log_1(&"🎯 Live Preview: Removed logo effects".into());
                                    }
                                }
                            }
                        }
                    }
                    
                    // Handle container-specific overlay properties
                    if component_type == "container" {
                        let mut has_overlay_settings = false;
                        
                        // Check for overlay properties
                        if let Some(overlay_color) = template_data.get("overlay_color").and_then(|v| v.as_str()) {
                            if !overlay_color.is_empty() {
                                styles.push(format!("--container-overlay-color: {}", overlay_color));
                                has_overlay_settings = true;
                            }
                        }
                        if let Some(overlay_opacity) = template_data.get("overlay_opacity").and_then(|v| v.as_str()) {
                            if !overlay_opacity.is_empty() && overlay_opacity != "0" {
                                styles.push(format!("--container-overlay-opacity: {}", overlay_opacity));
                                has_overlay_settings = true;
                            }
                        }
                        
                        // Check if we have background media (image or video)
                        let has_background_media = match bg_type {
                            "image" | "video" => true,
                            _ => false
                        };
                        
                        // Enable overlay display only if we have both overlay settings and background media
                        if has_overlay_settings && has_background_media {
                            styles.push("--container-overlay-display: block".to_string());
                        } else {
                            styles.push("--container-overlay-display: none".to_string());
                        }
                        
                        // Handle container width properties
                        if let Some(width_type) = template_data.get("width_type").and_then(|v| v.as_str()) {
                            styles.push(format!("--container-width-type: {}", width_type));
                        }
                        if let Some(max_width) = template_data.get("max_width").and_then(|v| v.as_str()) {
                            styles.push(format!("max-width: {}", max_width));
                        }
                        if let Some(padding) = template_data.get("padding").and_then(|v| v.as_str()) {
                            styles.push(format!("padding: {}", padding));
                        }
                        if let Some(margin) = template_data.get("margin").and_then(|v| v.as_str()) {
                            styles.push(format!("margin: {}", margin));
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
                            // Remove existing properties that might conflict with our new ones
                            let s = s.trim();
                            let should_keep = !s.starts_with("height:") && 
                                !s.starts_with("background:") && 
                                !s.starts_with("background-color:") && 
                                !s.starts_with("background-image:") && 
                                !s.starts_with("background-size:") && 
                                !s.starts_with("background-position:") && 
                                !s.starts_with("background-repeat:") && 
                                !s.starts_with("--header-text:") && 
                                !s.starts_with("--footer-text:") &&
                                // Remove position-related properties to avoid conflicts
                                !s.starts_with("position:") &&
                                !s.starts_with("top:") &&
                                !s.starts_with("left:") &&
                                !s.starts_with("right:") &&
                                !s.starts_with("z-index:") &&
                                !s.starts_with("width:") &&
                                !s.starts_with("transform:");
                            
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
                    
                                            // Handle header effects and positioning properly
                        if component_type == "header" {
                            let has_effects = template_data.get("effects").and_then(|v| v.as_str()).unwrap_or("none") != "none";
                            let position = template_data.get("position").and_then(|v| v.as_str()).unwrap_or("sticky");
                            
                            // If user explicitly sets position and it conflicts with effects, respect user choice
                            if template_data.get("position").is_some() && (position == "static" || position == "sticky") && has_effects {
                                let current_class = html_element.get_attribute("class").unwrap_or_default();
                                let mut classes: Vec<&str> = current_class.split_whitespace().collect();
                                
                                // Remove effect classes that force position: fixed
                                classes.retain(|&class| !class.starts_with("effect-"));
                                
                                let new_class = classes.join(" ");
                                if new_class != current_class {
                                    web_sys::console::log_1(&format!("🎯 Live Preview: Removing effect classes to allow position change: {} -> {}", current_class, new_class).into());
                                    let _ = html_element.set_attribute("class", &new_class);
                                }
                                
                                // Remove header-has-effects class from body since we're overriding effects
                                if let Some(window) = web_sys::window() {
                                    if let Some(document) = window.document() {
                                        if let Some(body) = document.body() {
                                            let current_body_class = body.class_name();
                                            let new_body_class = current_body_class.replace("header-has-effects", "").trim().to_string();
                                            if new_body_class != current_body_class {
                                                body.set_class_name(&new_body_class);
                                                // Remove the margin-top from main content
                                                if let Some(main_element) = document.query_selector("main").ok().flatten() {
                                                    let current_main_style = main_element.get_attribute("style").unwrap_or_default();
                                                    let new_main_style = current_main_style
                                                        .split(';')
                                                        .filter(|s| !s.trim().starts_with("margin-top:"))
                                                        .collect::<Vec<_>>()
                                                        .join("; ");
                                                    let _ = main_element.set_attribute("style", &new_main_style);
                                                }
                                                web_sys::console::log_1(&"🎯 Live Preview: Removed header-has-effects class and main margin".into());
                                            }
                                        }
                                    }
                                }
                            } else if has_effects && (position == "fixed" || !template_data.get("position").is_some()) {
                                // When effects are applied and position allows it, ensure proper body class and spacing
                                if let Some(window) = web_sys::window() {
                                    if let Some(document) = window.document() {
                                        if let Some(body) = document.body() {
                                            let current_body_class = body.class_name();
                                            if !current_body_class.contains("header-has-effects") {
                                                body.set_class_name(&format!("{} header-has-effects", current_body_class));
                                                
                                                // Add proper margin to main content
                                                if let Some(main_element) = document.query_selector("main").ok().flatten() {
                                                    let header_height = template_data.get("height")
                                                        .and_then(|v| v.as_str())
                                                        .unwrap_or("120px")
                                                        .trim_end_matches("px");
                                                    
                                                    let current_main_style = main_element.get_attribute("style").unwrap_or_default();
                                                    let mut main_styles: Vec<String> = current_main_style
                                                        .split(';')
                                                        .filter(|s| !s.trim().is_empty() && !s.trim().starts_with("margin-top:"))
                                                        .map(|s| s.trim().to_string())
                                                        .collect();
                                                    
                                                    main_styles.push(format!("margin-top: {}px", header_height));
                                                    let new_main_style = main_styles.join("; ");
                                                    let _ = main_element.set_attribute("style", &new_main_style);
                                                }
                                                web_sys::console::log_1(&"🎯 Live Preview: Added header-has-effects class and main margin".into());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    
                    let _ = html_element.set_attribute("style", &new_style);
                    
                    // Handle animation classes - apply them to the element
                    if let Some(animation_type) = template_data.get("animation_type").and_then(|v| v.as_str()) {
                        if animation_type != "none" {
                            // Remove any existing animation classes first
                            let current_class = html_element.class_name();
                            let cleaned_classes = current_class
                                .split_whitespace()
                                .filter(|&c| !c.starts_with("intro-") && c != "intro-animation" && c != "animate")
                                .collect::<Vec<&str>>()
                                .join(" ");
                            
                            // Add new animation classes (no need for 'animate' class with keyframe animations)
                            let new_classes = format!("{} intro-animation intro-{}", cleaned_classes, animation_type).trim().to_string();
                            html_element.set_class_name(&new_classes);
                            
                            web_sys::console::log_1(&format!("🎬 Live Edit: Applied animation classes to {}: {}", component_type, new_classes).into());
                        } else {
                            // Remove animation classes if animation is set to "none"
                            let current_class = html_element.class_name();
                            let cleaned_classes = current_class
                                .split_whitespace()
                                .filter(|&c| !c.starts_with("intro-") && c != "intro-animation" && c != "animate")
                                .collect::<Vec<&str>>()
                                .join(" ");
                            html_element.set_class_name(&cleaned_classes);
                            
                            web_sys::console::log_1(&format!("🎬 Live Edit: Removed animation classes from {}", component_type).into());
                        }
                    }
                    
                    // Verify the styles were applied
                    if let Some(applied_style) = html_element.get_attribute("style") {
                        web_sys::console::log_1(&format!("🔍 Live Preview: Verified applied styles: {}", applied_style).into());
                    } else {
                        web_sys::console::log_1(&format!("❌ Live Preview: No styles found after application").into());
                    }
                }
            }
        }
    }
}

// Helper function to find component by index
fn find_component_by_index(components: &[PageComponent], index: usize) -> Option<PageComponent> {
    components.get(index).cloned()
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

// Function to create SVG ripples that match the logo shape
pub fn create_svg_ripples(logo_container: &web_sys::HtmlElement, svg_url: &str) {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, RequestMode, Response};
    
    // Remove existing ripples first
    if let Some(existing_ripples) = logo_container.query_selector(".logo-ripples").ok().flatten() {
        let _ = existing_ripples.remove();
    }
    
    let logo_container = logo_container.clone();
    let svg_url = svg_url.to_string();
    
    wasm_bindgen_futures::spawn_local(async move {
        // Fetch the SVG content
        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);
        
        if let Ok(request) = Request::new_with_str_and_init(&svg_url, &opts) {
            if let Some(window) = web_sys::window() {
                if let Ok(resp_value) = JsFuture::from(window.fetch_with_request(&request)).await {
                    if let Ok(resp) = resp_value.dyn_into::<Response>() {
                        if let Ok(text_promise) = resp.text() {
                            if let Ok(text_value) = JsFuture::from(text_promise).await {
                                if let Some(svg_content) = text_value.as_string() {
                                    // Create ripples container
                                    if let Ok(document) = window.document().ok_or("No document") {
                                        if let Some(ripples_container) = document.create_element("div").ok() {
                                            let _ = ripples_container.set_attribute("class", "logo-ripples");
                                            
                                            // Create 3 ripple layers
                                            for i in 1..=3 {
                                                if let Some(ripple_div) = document.create_element("div").ok() {
                                                    let _ = ripple_div.set_attribute("class", &format!("logo-ripple ripple-{}", i));
                                                    
                                                    // Parse and modify SVG content
                                                    let modified_svg = modify_svg_for_ripple(&svg_content);
                                                    ripple_div.set_inner_html(&modified_svg);
                                                    
                                                    let _ = ripples_container.append_child(&ripple_div);
                                                }
                                            }
                                            
                                            // Add ripples to logo container
                                            let _ = logo_container.append_child(&ripples_container);
                                            
                                            web_sys::console::log_1(&"🎯 Created SVG ripples successfully".into());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}

// Function to modify SVG content for ripple effect
fn modify_svg_for_ripple(svg_content: &str) -> String {
    // Remove any existing fill attributes and add stroke-only styling
    let mut modified = svg_content.to_string();
    
    // Remove fill attributes
    modified = modified.replace(r#"fill="[^"]*""#, "");
    modified = modified.replace(r#"fill='[^']*'"#, "");
    
    // Add stroke-only styling to all shape elements
    let shapes = ["path", "circle", "rect", "polygon", "ellipse", "line", "polyline"];
    for shape in &shapes {
        let pattern = format!(r#"<{}"#, shape);
        let replacement = format!(r#"<{} fill="none" stroke="currentColor""#, shape);
        modified = modified.replace(&pattern, &replacement);
    }
    
    // Ensure SVG has proper attributes for ripple effect
    if modified.contains("<svg") {
        // Add preserveAspectRatio and other attributes if not present
        if !modified.contains("preserveAspectRatio") {
            modified = modified.replace("<svg", r#"<svg preserveAspectRatio="xMidYMid meet""#);
        }
        if !modified.contains("vector-effect") {
            modified = modified.replace("stroke=\"currentColor\"", r#"stroke="currentColor" vector-effect="non-scaling-stroke""#);
        }
    }
    
    modified
}
