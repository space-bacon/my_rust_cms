use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement, InputEvent};
use crate::components::page_builder::drag_drop_builder::{PageComponent, ComponentProperties};
use crate::services::navigation_service::ComponentTemplate;

#[derive(Properties, PartialEq)]
pub struct UnifiedPropertiesPanelProps {
    pub panel_type: PanelType,
    pub component: Option<PageComponent>,
    pub template_data: Option<serde_json::Value>,
    pub template_id: Option<i32>,
    pub template_name: Option<String>,
    pub on_close: Callback<()>,
    pub on_component_updated: Option<Callback<PageComponent>>,
    pub on_template_updated: Option<Callback<ComponentTemplate>>,
}

#[derive(Clone, PartialEq)]
pub enum PanelType {
    PageComponent,
    HeaderTemplate,
    FooterTemplate,
    ContainerTemplate,
}

#[function_component(UnifiedPropertiesPanel)]
pub fn unified_properties_panel(props: &UnifiedPropertiesPanelProps) -> Html {
    let working_template_data = use_state(|| {
        props.template_data.clone().unwrap_or_else(|| serde_json::json!({}))
    });

    let working_properties = use_state(|| {
        props.component.as_ref()
            .map(|c| c.properties.clone())
            .unwrap_or_default()
    });

    let working_content = use_state(|| {
        props.component.as_ref()
            .map(|c| c.content.clone())
            .unwrap_or_default()
    });

    // Track if user has made changes (to prevent overwriting unsaved changes)
    let has_unsaved_changes = use_state(|| false);
    let has_unsaved_changes_ref = use_node_ref();
    let saving = use_state(|| false);

    // Update working template data when props change, but only if no unsaved changes
    {
        let working_template_data = working_template_data.clone();
        let has_unsaved_changes_state = has_unsaved_changes.clone();
        use_effect_with_deps(move |deps| {
            let (template_data_opt, currently_has_unsaved) = deps;
            
            // Only sync if no unsaved changes and template data exists
            if !currently_has_unsaved && template_data_opt.is_some() {
                if let Some(data) = template_data_opt.clone() {
                    web_sys::console::log_1(&format!("✅ Syncing template data from props (unsaved: {})", currently_has_unsaved).into());
                    working_template_data.set(data);
                }
            } else if *currently_has_unsaved {
                web_sys::console::log_1(&"⚠️ Skipping template data sync - user has unsaved changes".into());
            }
            || ()
        }, (props.template_data.clone(), *has_unsaved_changes));
    }

    // Handle property changes for templates
    let on_property_change = {
        let working_template_data = working_template_data.clone();
        let props_on_template_updated = props.on_template_updated.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let panel_type = props.panel_type.clone();
        
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                    let name = input.name();
                    let value = input.value();
                    
                    let mut data = (*working_template_data).clone();
                    data[&name] = serde_json::Value::String(value.clone());
                    working_template_data.set(data.clone());
                    
                    // Mark that user has made changes
                    has_unsaved_changes.set(true);
                    
                    // Apply real-time preview for all template properties
                    let component_type = match panel_type {
                        PanelType::HeaderTemplate => "header",
                        PanelType::FooterTemplate => "footer",
                        PanelType::ContainerTemplate => "container",
                        _ => "",
                    };
                    
                    if !component_type.is_empty() {
                        web_sys::console::log_1(&format!("🔧 Properties Panel: Calling apply_template_style_preview for {} with field '{}' = '{}'", component_type, name, value).into());
                        crate::components::enhanced_live_edit_system::apply_template_style_preview(component_type, &data);
                    }
                    
                    // Apply real-time shape mask preview
                    if name.starts_with("shape_mask") {
                        // Determine which element to target based on panel type
                        let element_id = match panel_type {
                            PanelType::HeaderTemplate => "site-header",
                            PanelType::FooterTemplate => "site-footer",
                            _ => "site-header", // Default fallback
                        };
                        
                        if let Some(element) = web_sys::window()
                            .and_then(|w| w.document())
                            .and_then(|d| d.get_element_by_id(element_id))
                            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
                        {
                            let shape_mask_upper = data.get("shape_mask_upper").and_then(|v| v.as_str()).unwrap_or("none");
                            let shape_mask_upper_scale = data.get("shape_mask_upper_scale").and_then(|v| v.as_str()).unwrap_or("100");
                            let shape_mask_upper_frequency = data.get("shape_mask_upper_frequency").and_then(|v| v.as_str()).unwrap_or("2");
                            let shape_mask_upper_amplitude = data.get("shape_mask_upper_amplitude").and_then(|v| v.as_str()).unwrap_or("50");
                            let shape_mask_upper_curve_depth = data.get("shape_mask_upper_curve_depth").and_then(|v| v.as_str()).unwrap_or("50");
                            let shape_mask_lower = data.get("shape_mask_lower").and_then(|v| v.as_str()).unwrap_or("none");
                            let shape_mask_lower_scale = data.get("shape_mask_lower_scale").and_then(|v| v.as_str()).unwrap_or("100");
                            let shape_mask_lower_frequency = data.get("shape_mask_lower_frequency").and_then(|v| v.as_str()).unwrap_or("2");
                            let shape_mask_lower_amplitude = data.get("shape_mask_lower_amplitude").and_then(|v| v.as_str()).unwrap_or("50");
                            let shape_mask_lower_curve_depth = data.get("shape_mask_lower_curve_depth").and_then(|v| v.as_str()).unwrap_or("50");
                            
                            crate::components::enhanced_live_edit_system::apply_shape_masks_to_element(
                                &element,
                                shape_mask_upper, shape_mask_upper_scale, shape_mask_upper_frequency, 
                                &data.get("shape_mask_upper_direction").and_then(|v| v.as_str()).unwrap_or("positive"), shape_mask_upper_amplitude, 
                                &data.get("shape_mask_upper_degrees").and_then(|v| v.as_str()).unwrap_or("15"),
                                shape_mask_lower, shape_mask_lower_scale, shape_mask_lower_frequency, 
                                &data.get("shape_mask_lower_direction").and_then(|v| v.as_str()).unwrap_or("positive"), shape_mask_lower_amplitude, 
                                &data.get("shape_mask_lower_degrees").and_then(|v| v.as_str()).unwrap_or("15")
                            );
                        }
                    }
                    
                    // Apply real-time preview for template properties
                    if let Some(callback) = &props_on_template_updated {
                        let template = ComponentTemplate {
                            id: 1, // This would be the actual template ID
                            name: "Header".to_string(), // This would be the actual template name
                            component_type: "header".to_string(),
                            template_data: data,
                            breakpoints: serde_json::json!({}),
                            width_setting: None,
                            max_width: None,
                            is_default: false,
                            is_active: true,
                        };
                        callback.emit(template);
                    }
                } else if let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
                    let name = select.name();
                    let value = select.value();
                    
                    let mut data = (*working_template_data).clone();
                    data[&name] = serde_json::Value::String(value);
                    working_template_data.set(data.clone());
                    
                    // Mark that user has made changes
                    has_unsaved_changes.set(true);
                    
                    // Apply real-time shape mask preview
                    if name.starts_with("shape_mask") {
                        // Determine which element to target based on panel type
                        let element_id = match panel_type {
                            PanelType::HeaderTemplate => "site-header",
                            PanelType::FooterTemplate => "site-footer",
                            _ => "site-header", // Default fallback
                        };
                        
                        if let Some(element) = web_sys::window()
                            .and_then(|w| w.document())
                            .and_then(|d| d.get_element_by_id(element_id))
                            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
                        {
                            let shape_mask_upper = data.get("shape_mask_upper").and_then(|v| v.as_str()).unwrap_or("none");
                            let shape_mask_upper_scale = data.get("shape_mask_upper_scale").and_then(|v| v.as_str()).unwrap_or("100");
                            let shape_mask_upper_frequency = data.get("shape_mask_upper_frequency").and_then(|v| v.as_str()).unwrap_or("2");
                            let shape_mask_upper_amplitude = data.get("shape_mask_upper_amplitude").and_then(|v| v.as_str()).unwrap_or("50");
                            let shape_mask_upper_curve_depth = data.get("shape_mask_upper_curve_depth").and_then(|v| v.as_str()).unwrap_or("50");
                            let shape_mask_lower = data.get("shape_mask_lower").and_then(|v| v.as_str()).unwrap_or("none");
                            let shape_mask_lower_scale = data.get("shape_mask_lower_scale").and_then(|v| v.as_str()).unwrap_or("100");
                            let shape_mask_lower_frequency = data.get("shape_mask_lower_frequency").and_then(|v| v.as_str()).unwrap_or("2");
                            let shape_mask_lower_amplitude = data.get("shape_mask_lower_amplitude").and_then(|v| v.as_str()).unwrap_or("50");
                            let shape_mask_lower_curve_depth = data.get("shape_mask_lower_curve_depth").and_then(|v| v.as_str()).unwrap_or("50");
                            
                            crate::components::enhanced_live_edit_system::apply_shape_masks_to_element(
                                &element,
                                shape_mask_upper, shape_mask_upper_scale, shape_mask_upper_frequency, 
                                &data.get("shape_mask_upper_direction").and_then(|v| v.as_str()).unwrap_or("positive"), shape_mask_upper_amplitude, 
                                &data.get("shape_mask_upper_degrees").and_then(|v| v.as_str()).unwrap_or("15"),
                                shape_mask_lower, shape_mask_lower_scale, shape_mask_lower_frequency, 
                                &data.get("shape_mask_lower_direction").and_then(|v| v.as_str()).unwrap_or("positive"), shape_mask_lower_amplitude, 
                                &data.get("shape_mask_lower_degrees").and_then(|v| v.as_str()).unwrap_or("15")
                            );
                        }
                    }
                    
                    // Apply real-time preview for template properties
                    if let Some(callback) = &props_on_template_updated {
                        let template = ComponentTemplate {
                            id: 1, // This would be the actual template ID
                            name: "Header".to_string(), // This would be the actual template name
                            component_type: "header".to_string(),
                            template_data: data,
                            breakpoints: serde_json::json!({}),
                            width_setting: None,
                            max_width: None,
                            is_default: false,
                            is_active: true,
                        };
                        callback.emit(template);
                    }
                }
            }
        })
    };

    // Save changes
    let on_save = {
        let working_template_data = working_template_data.clone();
        let working_content = working_content.clone();
        let props_component = props.component.clone();
        let props_on_component_updated = props.on_component_updated.clone();
        let props_on_template_updated = props.on_template_updated.clone();
        let panel_type = props.panel_type.clone();
        let template_id = props.template_id;
        let template_name = props.template_name.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let saving = saving.clone();
        
        Callback::from(move |_| {
            // Prevent multiple saves
            if *saving {
                return;
            }
            
            saving.set(true);
            match panel_type {
                PanelType::PageComponent => {
                    if let (Some(mut component), Some(callback)) = (props_component.clone(), &props_on_component_updated) {
                        component.content = (*working_content).clone();
                        callback.emit(component);
                    }
                }
                PanelType::HeaderTemplate | PanelType::FooterTemplate | PanelType::ContainerTemplate => {
                    if let Some(callback) = &props_on_template_updated {
                        // Clean up shape mask parameters based on current shape selections
                        // BUT preserve all settings for non-none shapes to prevent interference
                        let mut clean_template_data = (*working_template_data).clone();
                        
                        web_sys::console::log_1(&format!("🔧 Save: Before cleanup - Upper: {:?}, Lower: {:?}", 
                            clean_template_data.get("shape_mask_upper"), 
                            clean_template_data.get("shape_mask_lower")).into());
                        
                        // Only clean up parameters if shape is explicitly "none" - preserve all others
                        if let Some(upper_shape) = clean_template_data.get("shape_mask_upper").and_then(|v| v.as_str()) {
                            if upper_shape == "none" {
                                // Remove all upper shape parameters only for "none"
                                clean_template_data.as_object_mut().map(|obj| {
                                    obj.remove("shape_mask_upper_frequency");
                                    obj.remove("shape_mask_upper_scale");
                                    obj.remove("shape_mask_upper_direction");
                                    obj.remove("shape_mask_upper_amplitude");
                                    obj.remove("shape_mask_upper_degrees");
                                });
                                web_sys::console::log_1(&"🧹 Cleaned up upper shape parameters (shape was 'none')".into());
                            } else {
                                web_sys::console::log_1(&format!("✅ Preserving upper shape '{}' parameters", upper_shape).into());
                            }
                        }
                        
                        // Only clean up parameters if shape is explicitly "none" - preserve all others
                        if let Some(lower_shape) = clean_template_data.get("shape_mask_lower").and_then(|v| v.as_str()) {
                            if lower_shape == "none" {
                                // Remove all lower shape parameters only for "none"
                                clean_template_data.as_object_mut().map(|obj| {
                                    obj.remove("shape_mask_lower_frequency");
                                    obj.remove("shape_mask_lower_scale");
                                    obj.remove("shape_mask_lower_direction");
                                    obj.remove("shape_mask_lower_amplitude");
                                    obj.remove("shape_mask_lower_degrees");
                                });
                                web_sys::console::log_1(&"🧹 Cleaned up lower shape parameters (shape was 'none')".into());
                            } else {
                                web_sys::console::log_1(&format!("✅ Preserving lower shape '{}' parameters", lower_shape).into());
                            }
                        }
                        
                        // Ensure template data includes ALL current working data (including shape masks)
                        let mut final_template_data = clean_template_data.clone();
                        
                        // Merge in any current working template data to ensure nothing is lost
                        if let Some(current_data) = working_template_data.as_object() {
                            if let Some(final_data) = final_template_data.as_object_mut() {
                                for (key, value) in current_data {
                                    // Only add if not already present (clean_template_data takes precedence)
                                    if !final_data.contains_key(key) {
                                        final_data.insert(key.clone(), value.clone());
                                    }
                                }
                            }
                        }
                        
                        let template = ComponentTemplate {
                            id: template_id.unwrap_or(1),
                            name: template_name.clone().unwrap_or_else(|| "Template".to_string()),
                            component_type: match panel_type {
                                PanelType::HeaderTemplate => "header",
                                PanelType::FooterTemplate => "footer",
                                PanelType::ContainerTemplate => "container",
                                _ => "unknown",
                            }.to_string(),
                            template_data: final_template_data,
                            breakpoints: serde_json::json!({}),
                            width_setting: None,
                            max_width: None,
                            is_default: true,  // Mark as default so it gets used
                            is_active: true,   // Ensure it's active so it appears in public endpoint
                        };
                        // Debug logging
                        web_sys::console::log_1(&format!("💾 Saving template with ID: {}, name: {}", template.id, template.name).into());
                        web_sys::console::log_1(&format!("💾 Clean template data: {}", serde_json::to_string_pretty(&clean_template_data).unwrap_or_default()).into());
                        web_sys::console::log_1(&format!("💾 Working template data: {}", serde_json::to_string_pretty(&*working_template_data).unwrap_or_default()).into());
                        web_sys::console::log_1(&format!("💾 Final template data being saved: {}", serde_json::to_string_pretty(&template.template_data).unwrap_or_default()).into());
                        
                        // Actually save to database via API
                        let template_for_api = template.clone();
                        let has_unsaved_changes_for_api = has_unsaved_changes.clone();
                        let saving_for_api = saving.clone();
                        let callback_for_api = callback.clone();
                        
                        wasm_bindgen_futures::spawn_local(async move {
                            web_sys::console::log_1(&format!("💾 Starting API save to database for template ID: {}", template_for_api.id).into());
                            web_sys::console::log_1(&format!("💾 Template is_active: {}, is_default: {}", template_for_api.is_active, template_for_api.is_default).into());
                            
                            // First, clear the default flag from other templates of the same type
                            let component_type_for_api = template_for_api.component_type.clone();
                            web_sys::console::log_1(&format!("🔄 Clearing default flag from other {} templates...", component_type_for_api).into());
                            match crate::services::navigation_service::get_component_templates().await {
                                Ok(all_templates) => {
                                    for other_template in all_templates {
                                        if other_template.component_type == component_type_for_api 
                                           && other_template.id != template_for_api.id 
                                           && other_template.is_default {
                                            web_sys::console::log_1(&format!("🔄 Clearing default flag from template ID: {}", other_template.id).into());
                                            let mut updated_template = other_template.clone();
                                            updated_template.is_default = false;
                                            let _ = crate::services::navigation_service::update_component_template(updated_template.id, &updated_template).await;
                                        }
                                    }
                                }
                                Err(e) => {
                                    web_sys::console::log_1(&format!("⚠️ Could not load templates to clear defaults: {:?}", e).into());
                                }
                            }
                            
                            match crate::services::navigation_service::update_component_template(template_for_api.id, &template_for_api).await {
                                Ok(saved_template) => {
                                    web_sys::console::log_1(&"✅ Template successfully saved to database!".into());
                                    
                                    // Show success notification
                                    if let Some(window) = web_sys::window() {
                                        if let Some(document) = window.document() {
                                            if let Some(body) = document.body() {
                                                let notification = document.create_element("div").unwrap();
                                                notification.set_class_name("save-notification success");
                                                notification.set_inner_html("✅ Changes saved successfully!");
                                                
                                                // Style the notification
                                                let _ = notification.set_attribute("style", 
                                                    "position: fixed; top: 20px; right: 20px; background: #4CAF50; color: white; \
                                                     padding: 12px 20px; border-radius: 4px; box-shadow: 0 2px 8px rgba(0,0,0,0.2); \
                                                     z-index: 10000; font-weight: 600; animation: slideInRight 0.3s ease-out;"
                                                );
                                                
                                                let _ = body.append_child(&notification);
                                                
                                                // Remove notification after 3 seconds
                                                let notification_clone = notification.clone();
                                                wasm_bindgen_futures::spawn_local(async move {
                                                    gloo_timers::future::TimeoutFuture::new(3000).await;
                                                    let _ = notification_clone.remove();
                                                });
                                            }
                                        }
                                    }
                                    
                                    // Emit the callback with the saved template
                                    callback_for_api.emit(saved_template.clone());
                                    
                                    // Force reload component templates to ensure changes are visible immediately
                                    let saved_template_clone = saved_template.clone();
                                    wasm_bindgen_futures::spawn_local(async move {
                                        // Small delay to ensure database transaction is committed
                                        gloo_timers::future::TimeoutFuture::new(500).await;
                                        
                                        // Verify the template is now available in the public endpoint
                                        web_sys::console::log_1(&"🔄 Verifying template is available in public endpoint...".into());
                                        match crate::services::navigation_service::get_component_templates().await {
                                            Ok(templates) => {
                                                let found = templates.iter().find(|t| t.id == saved_template_clone.id);
                                                if found.is_some() {
                                                    web_sys::console::log_1(&"✅ Template found in public endpoint - changes should be visible!".into());
                                                } else {
                                                    web_sys::console::log_1(&"⚠️ Template not found in public endpoint - may need page refresh".into());
                                                }
                                            }
                                            Err(e) => {
                                                web_sys::console::log_1(&format!("❌ Error verifying template: {:?}", e).into());
                                            }
                                        }
                                    });
                                    
                                    // Reset unsaved changes flag after successful save
                                    has_unsaved_changes_for_api.set(false);
                                    
                                    // Reset saving state
                                    saving_for_api.set(false);
                                }
                                Err(e) => {
                                    web_sys::console::log_1(&format!("❌ Failed to save template to database: {:?}", e).into());
                                    
                                    // Show error notification
                                    if let Some(window) = web_sys::window() {
                                        if let Some(document) = window.document() {
                                            if let Some(body) = document.body() {
                                                let notification = document.create_element("div").unwrap();
                                                notification.set_class_name("save-notification error");
                                                notification.set_inner_html(&format!("❌ Failed to save changes: {}", e));
                                                
                                                // Style the error notification
                                                let _ = notification.set_attribute("style", 
                                                    "position: fixed; top: 20px; right: 20px; background: #f44336; color: white; \
                                                     padding: 12px 20px; border-radius: 4px; box-shadow: 0 2px 8px rgba(0,0,0,0.2); \
                                                     z-index: 10000; font-weight: 600; animation: slideInRight 0.3s ease-out;"
                                                );
                                                
                                                let _ = body.append_child(&notification);
                                                
                                                // Remove error notification after 5 seconds
                                                let notification_clone = notification.clone();
                                                wasm_bindgen_futures::spawn_local(async move {
                                                    gloo_timers::future::TimeoutFuture::new(5000).await;
                                                    let _ = notification_clone.remove();
                                                });
                                            }
                                        }
                                    }
                                    
                                    // Reset saving state even on error
                                    saving_for_api.set(false);
                                }
                            }
                        });
                        
                        // Reapply shape masks after save to ensure visual consistency
                        if panel_type == PanelType::HeaderTemplate || panel_type == PanelType::FooterTemplate {
                            let element_id = match panel_type {
                                PanelType::HeaderTemplate => "site-header",
                                PanelType::FooterTemplate => "site-footer",
                                _ => "site-header",
                            };
                            
                            if let Some(element) = web_sys::window()
                                .and_then(|w| w.document())
                                .and_then(|d| d.get_element_by_id(element_id))
                                .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
                            {
                                // Use the clean_template_data that was just saved to ensure consistency
                                let data = &clean_template_data;
                                web_sys::console::log_1(&format!("🔄 Reapplying shapes after save - Upper: {:?}, Lower: {:?}", 
                                    data.get("shape_mask_upper"), data.get("shape_mask_lower")).into());
                                
                                crate::components::enhanced_live_edit_system::apply_shape_masks_to_element(
                                    &element,
                                    &data.get("shape_mask_upper").and_then(|v| v.as_str()).unwrap_or("none"),
                                    &data.get("shape_mask_upper_scale").and_then(|v| v.as_str()).unwrap_or("100"),
                                    &data.get("shape_mask_upper_frequency").and_then(|v| v.as_str()).unwrap_or("2"),
                                    &data.get("shape_mask_upper_direction").and_then(|v| v.as_str()).unwrap_or("positive"),
                                    &data.get("shape_mask_upper_amplitude").and_then(|v| v.as_str()).unwrap_or("50"),
                                    &data.get("shape_mask_upper_degrees").and_then(|v| v.as_str()).unwrap_or("15"),
                                    &data.get("shape_mask_lower").and_then(|v| v.as_str()).unwrap_or("none"),
                                    &data.get("shape_mask_lower_scale").and_then(|v| v.as_str()).unwrap_or("100"),
                                    &data.get("shape_mask_lower_frequency").and_then(|v| v.as_str()).unwrap_or("2"),
                                    &data.get("shape_mask_lower_direction").and_then(|v| v.as_str()).unwrap_or("positive"),
                                    &data.get("shape_mask_lower_amplitude").and_then(|v| v.as_str()).unwrap_or("50"),
                                    &data.get("shape_mask_lower_degrees").and_then(|v| v.as_str()).unwrap_or("15")
                                );
                                
                                web_sys::console::log_1(&"🔄 Shape reapplication completed".into());
                            }
                        }
                    }
                }
            }
        })
    };

    let on_close = {
        let props_on_close = props.on_close.clone();
        Callback::from(move |_| props_on_close.emit(()))
    };

    html! {
        <div class="properties-panel" style="
            position: fixed;
            top: 20px;
            right: 20px;
            width: 320px;
            max-height: 80vh;
            background: white;
            border: 1px solid #ddd;
            border-radius: 8px;
            box-shadow: 0 4px 12px rgba(0,0,0,0.15);
            z-index: 1000;
            overflow-y: auto;
        ">
            <div class="panel-header" style="
                padding: 16px;
                border-bottom: 1px solid #eee;
                display: flex;
                justify-content: space-between;
                align-items: center;
                background: #f8f9fa;
                border-radius: 8px 8px 0 0;
            ">
                <h3 style="margin: 0; font-size: 16px; color: #333;">
                    {match props.panel_type {
                        PanelType::PageComponent => "Component Properties",
                        PanelType::HeaderTemplate => "Header Properties",
                        PanelType::FooterTemplate => "Footer Properties",
                        PanelType::ContainerTemplate => "Container Properties",
                    }}
                </h3>
                <button onclick={on_close} style="
                    background: none;
                    border: none;
                    font-size: 18px;
                    cursor: pointer;
                    color: #666;
                    padding: 4px;
                    border-radius: 4px;
                ">{"×"}</button>
            </div>
            
            <div class="panel-content" style="padding: 16px;">
                {match props.panel_type {
                    PanelType::PageComponent => {
                        render_component_properties(&working_content, &working_properties, on_property_change.clone())
                    }
                    PanelType::HeaderTemplate => {
                        render_header_properties(&working_template_data, on_property_change.clone())
                    }
                    PanelType::FooterTemplate => {
                        render_footer_properties(&working_template_data, on_property_change.clone())
                    }
                    PanelType::ContainerTemplate => {
                        render_container_properties(&working_template_data, on_property_change.clone())
                    }
                }}
                
                <div class="panel-actions" style="margin-top: 20px; padding-top: 16px; border-top: 1px solid #eee;">
                    <button 
                        onclick={on_save} 
                        disabled={*saving}
                        style={format!("
                            background: {};
                            color: white;
                            border: none;
                            padding: 8px 16px;
                            border-radius: 4px;
                            cursor: {};
                            font-size: 14px;
                            width: 100%;
                            opacity: {};
                        ", 
                        if *saving { "#6c757d" } else { "#007bff" },
                        if *saving { "not-allowed" } else { "pointer" },
                        if *saving { "0.6" } else { "1.0" }
                    )}>
                        {if *saving { "💾 Saving..." } else { "Save Changes" }}
                    </button>
                </div>
            </div>
        </div>
    }
}

fn render_component_properties(
    working_content: &UseStateHandle<String>, 
    _working_properties: &UseStateHandle<ComponentProperties>, 
    _on_change: Callback<InputEvent>
) -> Html {
    html! {
        <div>
            <div style="margin-bottom: 16px;">
                <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{"Content"}</label>
                <textarea 
                    value={(**working_content).clone()}
                    style="
                        width: 100%;
                        min-height: 100px;
                        padding: 8px;
                        border: 1px solid #ddd;
                        border-radius: 4px;
                        font-family: inherit;
                        font-size: 14px;
                        resize: vertical;
                    "
                    placeholder="Enter content..."
                />
            </div>
        </div>
    }
}

fn render_header_properties(template_data: &UseStateHandle<serde_json::Value>, on_change: Callback<InputEvent>) -> Html {
    // Extract current values
    let text_color = template_data.get("text_color").and_then(|v| v.as_str()).unwrap_or("#ffffff");
    let logo_url = template_data.get("logo_url").and_then(|v| v.as_str()).unwrap_or("");
    let height = template_data.get("height").and_then(|v| v.as_str()).unwrap_or("110px").trim_end_matches("px");
    let bg_type = template_data.get("bg_type").and_then(|v| v.as_str()).unwrap_or("color");
    let bg_color = template_data.get("bg_color").and_then(|v| v.as_str()).unwrap_or("#333333");
    let bg_image = template_data.get("bg_image").and_then(|v| v.as_str()).unwrap_or("");
    let bg_gradient = template_data.get("bg_gradient").and_then(|v| v.as_str()).unwrap_or("linear-gradient(135deg, #667eea 0%, #764ba2 100%)");
    let bg_video = template_data.get("bg_video").and_then(|v| v.as_str()).unwrap_or("");
    
    // Shape mask properties
    let shape_mask_upper = template_data.get("shape_mask_upper").and_then(|v| v.as_str()).unwrap_or("none");
    let shape_mask_upper_scale = template_data.get("shape_mask_upper_scale").and_then(|v| v.as_str()).unwrap_or("100");
    let shape_mask_lower = template_data.get("shape_mask_lower").and_then(|v| v.as_str()).unwrap_or("none");
    let shape_mask_lower_scale = template_data.get("shape_mask_lower_scale").and_then(|v| v.as_str()).unwrap_or("100");

    html! {
        <>
            // Basic Properties
            <div class="property-section">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Basic Properties"}</h4>
                {render_color_field("Text Color", "text_color", text_color, on_change.clone())}
                {render_input_field("Logo URL", "logo_url", logo_url, on_change.clone())}
                {render_range_field("Height (px)", "height", height, "60", "2600", on_change.clone())}
            </div>
            
            // Background Properties
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Background"}</h4>
                {render_select_field("Background Type", "bg_type", bg_type, vec![
                    ("color", "Color"),
                    ("image", "Image"),
                    ("gradient", "Gradient"),
                    ("video", "Video")
                ], on_change.clone())}
                
                {match bg_type {
                    "color" => render_color_field("Background Color", "bg_color", bg_color, on_change.clone()),
                    "image" => render_input_field("Background Image URL", "bg_image", bg_image, on_change.clone()),
                    "gradient" => render_input_field("Background Gradient", "bg_gradient", bg_gradient, on_change.clone()),
                    "video" => render_input_field("Background Video URL", "bg_video", bg_video, on_change.clone()),
                    _ => html! {}
                }}
            </div>
            
            // Shape Mask Properties
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Shape Mask"}</h4>
                
                // Upper Shape Mask
                <div style="margin-bottom: 16px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{"Upper Shape"}</label>
                    {render_select_field("", "shape_mask_upper", shape_mask_upper, vec![
                        ("none", "None"),
                        ("wave", "Wave"),
                        ("curve", "Curve"),
                        ("triangle", "Triangle"),
                        ("zigzag", "Zigzag"),
                        ("tilt", "Tilt")
                    ], on_change.clone())}
                    
                    {if shape_mask_upper != "none" {
                        html! {
                            <div style="margin-top: 8px;">
                                {match shape_mask_upper {
                                    "wave" | "triangle" | "zigzag" => html! {
                                        <>
                                            {render_range_field("Frequency", "shape_mask_upper_frequency", 
                                                &template_data.get("shape_mask_upper_frequency").and_then(|v| v.as_str()).unwrap_or("2"), 
                                                "1", "8", on_change.clone())}
                                            {render_range_field("Scale (%)", "shape_mask_upper_scale", shape_mask_upper_scale, "10", "100", on_change.clone())}
                                        </>
                                    },
                                    "curve" => html! {
                                        <>
                                            {render_select_field("Direction", "shape_mask_upper_direction", 
                                                &template_data.get("shape_mask_upper_direction").and_then(|v| v.as_str()).unwrap_or("positive"), 
                                                vec![("positive", "Positive"), ("negative", "Negative")], on_change.clone())}
                                            {render_range_field("Amplitude (%)", "shape_mask_upper_amplitude", 
                                                &template_data.get("shape_mask_upper_amplitude").and_then(|v| v.as_str()).unwrap_or("50"), 
                                                "10", "100", on_change.clone())}
                                            {render_range_field("Scale (%)", "shape_mask_upper_scale", shape_mask_upper_scale, "10", "100", on_change.clone())}
                                        </>
                                    },
                                    "tilt" => html! {
                                        <>
                                            {render_select_field("Direction", "shape_mask_upper_direction", 
                                                &template_data.get("shape_mask_upper_direction").and_then(|v| v.as_str()).unwrap_or("right"), 
                                                vec![("left", "Left"), ("right", "Right")], on_change.clone())}
                                            {render_range_field("Degrees", "shape_mask_upper_degrees", 
                                                &template_data.get("shape_mask_upper_degrees").and_then(|v| v.as_str()).unwrap_or("15"), 
                                                "0", "45", on_change.clone())}
                                        </>
                                    },
                                    _ => html! {
                                        {render_range_field("Scale (%)", "shape_mask_upper_scale", shape_mask_upper_scale, "10", "100", on_change.clone())}
                                    }
                                }}
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                </div>
                
                // Lower Shape Mask
                <div>
                    <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{"Lower Shape"}</label>
                    {render_select_field("", "shape_mask_lower", shape_mask_lower, vec![
                        ("none", "None"),
                        ("wave", "Wave"),
                        ("curve", "Curve"),
                        ("triangle", "Triangle"),
                        ("zigzag", "Zigzag"),
                        ("tilt", "Tilt")
                    ], on_change.clone())}
                    
                    {if shape_mask_lower != "none" {
                        html! {
                            <div style="margin-top: 8px;">
                                {match shape_mask_lower {
                                    "wave" | "triangle" | "zigzag" => html! {
                                        <>
                                            {render_range_field("Frequency", "shape_mask_lower_frequency", 
                                                &template_data.get("shape_mask_lower_frequency").and_then(|v| v.as_str()).unwrap_or("2"), 
                                                "1", "8", on_change.clone())}
                                            {render_range_field("Scale (%)", "shape_mask_lower_scale", shape_mask_lower_scale, "10", "100", on_change.clone())}
                                        </>
                                    },
                                    "curve" => html! {
                                        <>
                                            {render_select_field("Direction", "shape_mask_lower_direction", 
                                                &template_data.get("shape_mask_lower_direction").and_then(|v| v.as_str()).unwrap_or("positive"), 
                                                vec![("positive", "Positive"), ("negative", "Negative")], on_change.clone())}
                                            {render_range_field("Amplitude (%)", "shape_mask_lower_amplitude", 
                                                &template_data.get("shape_mask_lower_amplitude").and_then(|v| v.as_str()).unwrap_or("50"), 
                                                "10", "100", on_change.clone())}
                                            {render_range_field("Scale (%)", "shape_mask_lower_scale", shape_mask_lower_scale, "10", "100", on_change.clone())}
                                        </>
                                    },
                                    "tilt" => html! {
                                        <>
                                            {render_select_field("Direction", "shape_mask_lower_direction", 
                                                &template_data.get("shape_mask_lower_direction").and_then(|v| v.as_str()).unwrap_or("right"), 
                                                vec![("left", "Left"), ("right", "Right")], on_change.clone())}
                                            {render_range_field("Degrees", "shape_mask_lower_degrees", 
                                                &template_data.get("shape_mask_lower_degrees").and_then(|v| v.as_str()).unwrap_or("15"), 
                                                "0", "45", on_change.clone())}
                                        </>
                                    },
                                    _ => html! {
                                        {render_range_field("Scale (%)", "shape_mask_lower_scale", shape_mask_lower_scale, "10", "100", on_change.clone())}
                                    }
                                }}
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        </>
    }
}

fn render_footer_properties(template_data: &UseStateHandle<serde_json::Value>, on_change: Callback<InputEvent>) -> Html {
    // Same structure as header properties
    render_header_properties(template_data, on_change)
}

fn render_container_properties(template_data: &UseStateHandle<serde_json::Value>, on_change: Callback<InputEvent>) -> Html {
    // Basic container properties
    let bg_color = template_data.get("bg_color").and_then(|v| v.as_str()).unwrap_or("#ffffff");
    let text_color = template_data.get("text_color").and_then(|v| v.as_str()).unwrap_or("#333333");
    
    html! {
        <div class="property-section">
            <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Container Properties"}</h4>
            {render_color_field("Background Color", "bg_color", bg_color, on_change.clone())}
            {render_color_field("Text Color", "text_color", text_color, on_change.clone())}
        </div>
    }
}

// Helper functions for rendering form fields
fn render_input_field(label: &str, name: &str, value: &str, on_change: Callback<InputEvent>) -> Html {
    let label = label.to_string();
    let name = name.to_string();
    let value = value.to_string();
    
    html! {
        <div style="margin-bottom: 12px;">
            <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{label}</label>
            <input 
                type="text"
                name={name}
                value={value}
                oninput={on_change}
                style="
                    width: 100%;
                    padding: 6px 8px;
                    border: 1px solid #ddd;
                    border-radius: 4px;
                    font-size: 14px;
                "
            />
        </div>
    }
}

fn render_range_field(label: &str, name: &str, value: &str, min: &str, max: &str, on_change: Callback<InputEvent>) -> Html {
    let label = label.to_string();
    let name = name.to_string();
    let value = value.to_string();
    let min = min.to_string();
    let max = max.to_string();
    
    html! {
        <div style="margin-bottom: 12px;">
            <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{format!("{} ({})", label, value)}</label>
            <input 
                type="range"
                name={name}
                value={value}
                min={min}
                max={max}
                oninput={on_change}
                style="
                    width: 100%;
                    margin-bottom: 4px;
                "
            />
        </div>
    }
}

fn render_select_field(label: &str, name: &str, value: &str, options: Vec<(&str, &str)>, on_change: Callback<InputEvent>) -> Html {
    let label = label.to_string();
    let name = name.to_string();
    let value = value.to_string();
    let value_for_comparison = value.clone();
    
    html! {
        <div style="margin-bottom: 12px;">
            {if !label.is_empty() {
                html! {
                    <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{label}</label>
                }
            } else {
                html! {}
            }}
            <select 
                name={name}
                value={value}
                oninput={on_change}
                style="
                    width: 100%;
                    padding: 6px 8px;
                    border: 1px solid #ddd;
                    border-radius: 4px;
                    font-size: 14px;
                    background: white;
                "
            >
                {for options.into_iter().map(|(val, text)| {
                    let val_string = val.to_string();
                    let text_string = text.to_string();
                    html! {
                        <option value={val_string.clone()} selected={val_string == value_for_comparison}>{text_string}</option>
                    }
                })}
            </select>
        </div>
    }
}

fn render_color_field(label: &str, name: &str, value: &str, on_change: Callback<InputEvent>) -> Html {
    let label = label.to_string();
    let name = name.to_string();
    let value = value.to_string();
    
    html! {
        <div style="margin-bottom: 12px;">
            <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{label}</label>
            <div style="display: flex; gap: 8px; align-items: center;">
                <input 
                    type="color"
                    name={name.clone()}
                    value={value.clone()}
                    oninput={on_change.clone()}
                    style="
                        width: 40px;
                        height: 32px;
                        border: 1px solid #ddd;
                        border-radius: 4px;
                        cursor: pointer;
                    "
                />
                <input 
                    type="text"
                    name={name}
                    value={value}
                    oninput={on_change}
                    style="
                        flex: 1;
                        padding: 6px 8px;
                        border: 1px solid #ddd;
                        border-radius: 4px;
                        font-size: 14px;
                        font-family: monospace;
                    "
                />
            </div>
        </div>
    }
}
