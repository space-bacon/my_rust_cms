use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement, InputEvent};
use crate::components::page_builder::drag_drop_builder::{PageComponent, ComponentProperties, ComponentType};
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

    let has_unsaved_changes = use_state(|| false);
    let saving = use_state(|| false);

    // Update working states when props change
    {
        let working_properties = working_properties.clone();
        let working_content = working_content.clone();
        let _working_template_data = working_template_data.clone();
        
        use_effect_with_deps(move |component_opt| {
            if let Some(component) = component_opt {
                working_properties.set(component.properties.clone());
                working_content.set(component.content.clone());
            }
            || ()
        }, props.component.clone());
    }

    // Enhanced state management for template data
    // Only update working data when props change AND there are no unsaved changes
    // This prevents loss of user inputs during live editing
    {
        let working_template_data = working_template_data.clone();
        let _has_unsaved_changes_state = has_unsaved_changes.clone();
        use_effect_with_deps(move |deps| {
            let (template_data_opt, currently_has_unsaved) = deps;
            
            // Only reset working data if:
            // 1. There are no unsaved changes AND
            // 2. Template data is provided AND
            // 3. The template data has actually changed
            if !currently_has_unsaved {
                if let Some(new_data) = template_data_opt.clone() {
                    // Check if the data has actually changed to avoid unnecessary resets
                    if *working_template_data != new_data {
                        working_template_data.set(new_data);
                    }
                }
            }
            || ()
        }, (props.template_data.clone(), *has_unsaved_changes));
    }

    // Handle property changes for both templates and components
    let on_property_change = {
        let working_template_data = working_template_data.clone();
        let working_properties = working_properties.clone();
        let working_content = working_content.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let panel_type = props.panel_type.clone();
        
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                let (name, value, is_checkbox) = if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                    (input.name(), input.value(), input.type_() == "checkbox")
                } else if let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
                    (select.name(), select.value(), false)
                } else {
                    return;
                };
                
                // Handle component properties vs template properties
                match panel_type {
                    PanelType::PageComponent => {
                        // Handle component property updates
                        if name == "content" {
                            working_content.set(value.clone());
                        } else {
                            // Update component properties
                            let mut props = (*working_properties).clone();
                            update_component_property(&mut props, &name, &value, is_checkbox);
                            working_properties.set(props);
                        }
                    }
                    _ => {
                        // Handle template property updates with enhanced change accumulation
                        let mut data = (*working_template_data).clone();
                        
                        // Handle special property types that need processing
                        let processed_value = match name.as_str() {
                            // Height values - ensure they have px units if numeric
                            "height" => {
                                if value.chars().all(|c| c.is_numeric()) {
                                    format!("{}px", value)
                                } else {
                                    value.clone()
                                }
                            },
                            // Range values that should remain as strings
                            "effects_intensity" | "overlay_opacity" | "shrink_height" | "logo_scale" => value.clone(),
                            // All other values
                            _ => value.clone()
                        };
                        
                        // Update the working data with processed value
                        data[&name] = serde_json::Value::String(processed_value);
                        working_template_data.set(data.clone());
                        
                        // Apply real-time preview for template properties
                        let component_type = match panel_type {
                            PanelType::HeaderTemplate => "header",
                            PanelType::FooterTemplate => "footer",
                            PanelType::ContainerTemplate => "container",
                            _ => "",
                        };
                        
                        if !component_type.is_empty() {
                            // Apply preview with accumulated changes
                            crate::components::enhanced_live_edit_system::apply_template_style_preview(component_type, &data);
                        }
                    }
                }
                    
                    // Mark that user has made changes
                    has_unsaved_changes.set(true);
            }
        })
    };

    // Save changes
    let on_save = {
        let working_template_data = working_template_data.clone();
        let working_content = working_content.clone();
        let working_properties = working_properties.clone();
        let props_component = props.component.clone();
        let props_on_component_updated = props.on_component_updated.clone();
        let props_on_template_updated = props.on_template_updated.clone();
        let panel_type = props.panel_type.clone();
        let template_id = props.template_id;
        let template_name = props.template_name.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let saving = saving.clone();
        
        Callback::from(move |_| {
            if *saving {
                return;
            }
            
            saving.set(true);
            match panel_type {
                PanelType::PageComponent => {
                    if let (Some(mut component), Some(callback)) = (props_component.clone(), &props_on_component_updated) {
                        component.content = (*working_content).clone();
                        component.properties = (*working_properties).clone();
                        callback.emit(component);
                    }
                    saving.set(false);
                }
                PanelType::HeaderTemplate | PanelType::FooterTemplate | PanelType::ContainerTemplate => {
                    if let Some(callback) = &props_on_template_updated {
                        let template = ComponentTemplate {
                            id: template_id.unwrap_or(1),
                            name: template_name.clone().unwrap_or_else(|| "Template".to_string()),
                            component_type: match panel_type {
                                PanelType::HeaderTemplate => "header",
                                PanelType::FooterTemplate => "footer",
                                PanelType::ContainerTemplate => "container",
                                _ => "unknown",
                            }.to_string(),
                            template_data: (*working_template_data).clone(),
                            breakpoints: serde_json::json!({}),
                            width_setting: None,
                            max_width: None,
                            is_default: true,
                            is_active: true,
                        };
                        
                        // Save to database with enhanced error handling and feedback
                        let template_for_api = template.clone();
                        let has_unsaved_changes_for_api = has_unsaved_changes.clone();
                        let saving_for_api = saving.clone();
                        let callback_for_api = callback.clone();
                        
                        // Log the changes being saved for debugging
                        web_sys::console::log_1(&format!("💾 Live Edit: Saving {} template with data: {}", 
                            template_for_api.component_type,
                            serde_json::to_string_pretty(&template_for_api.template_data).unwrap_or_else(|_| "Failed to serialize".to_string())
                        ).into());
                        
                        wasm_bindgen_futures::spawn_local(async move {
                            match crate::services::navigation_service::update_component_template(template_for_api.id, &template_for_api).await {
                                Ok(saved_template) => {
                                    web_sys::console::log_1(&"🎉 Live Edit: Successfully saved all accumulated changes to database!".into());
                                    callback_for_api.emit(saved_template);
                                    has_unsaved_changes_for_api.set(false);
                                    
                                    // Re-enable scroll effects after successful save for header templates
                                    if template_for_api.component_type == "header" {
                                        if let Some(window) = web_sys::window() {
                                            if let Some(document) = window.document() {
                                                if let Some(header) = document.get_element_by_id("site-header") {
                                                    header.remove_attribute("data-live-edit-active").ok();
                                                    web_sys::console::log_1(&"✅ Live Edit: Re-enabled scroll effects after save".into());
                                                }
                                            }
                                        }
                                    }
                                    
                                    // Show success feedback to user
                                    if let Some(window) = web_sys::window() {
                                        let _ = window.alert_with_message("✅ Changes saved successfully!");
                                    }
                                }
                                Err(e) => {
                                    web_sys::console::log_1(&format!("❌ Live Edit: Failed to save template: {:?}", e).into());
                                    
                                    // Show error feedback to user  
                                    if let Some(window) = web_sys::window() {
                                        let _ = window.alert_with_message(&format!("❌ Failed to save changes: {}", e));
                                    }
                                }
                            }
                            saving_for_api.set(false);
                        });
                    }
                }
            }
        })
    };

    let on_close = {
        let props_on_close = props.on_close.clone();
        let panel_type = props.panel_type.clone();
        Callback::from(move |_| {
            // Re-enable scroll effects when closing live edit panel
            if matches!(panel_type, PanelType::HeaderTemplate) {
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(header) = document.get_element_by_id("site-header") {
                            header.remove_attribute("data-live-edit-active").ok();
                            web_sys::console::log_1(&"✅ Live Edit: Re-enabled scroll effects for header".into());
                        }
                    }
                }
            }
            props_on_close.emit(())
        })
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
                    {if *has_unsaved_changes {
                        html! {
                            <span style="
                                margin-left: 8px;
                                background: #ff6b35;
                                color: white;
                                font-size: 10px;
                                padding: 2px 6px;
                                border-radius: 10px;
                                font-weight: bold;
                            ">{"UNSAVED"}</span>
                        }
                    } else {
                        html! {}
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
                        let component_type = props.component.as_ref().map(|c| &c.component_type);
                        render_component_properties(&working_content, &working_properties, component_type, on_property_change.clone())
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
                        disabled={*saving || !*has_unsaved_changes}
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
                            transition: all 0.2s ease;
                        ", 
                        if *saving { 
                            "#6c757d" 
                        } else if *has_unsaved_changes { 
                            "#28a745" 
                        } else { 
                            "#6c757d" 
                        },
                        if *saving || !*has_unsaved_changes { "not-allowed" } else { "pointer" },
                        if *saving || !*has_unsaved_changes { "0.6" } else { "1.0" }
                    )}>
                        {if *saving { 
                            "💾 Saving..." 
                        } else if *has_unsaved_changes { 
                            "💾 Save All Changes" 
                        } else { 
                            "✅ All Changes Saved" 
                        }}
                    </button>
                    
                    {if *has_unsaved_changes && !*saving {
                        html! {
                            <div style="
                                margin-top: 8px;
                                padding: 8px;
                                background: #fff3cd;
                                border: 1px solid #ffeaa7;
                                border-radius: 4px;
                                font-size: 12px;
                                color: #856404;
                                text-align: center;
                            ">
                                {"⚠️ You have unsaved changes that will be lost if you close this panel"}
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        </div>
    }
}

fn render_component_properties(
    working_content: &UseStateHandle<String>, 
    working_properties: &UseStateHandle<ComponentProperties>, 
    component_type: Option<&ComponentType>,
    on_change: Callback<InputEvent>
) -> Html {
    let component_properties = &**working_properties;
    
    html! {
        <div>
            // Content Section (for most components)
            <div style="margin-bottom: 16px;">
                <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{"Content"}</label>
                <textarea 
                    name="content"
                    value={(**working_content).clone()}
                    oninput={on_change.clone()}
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
            
            // Component-specific properties
            {render_component_specific_properties(component_properties, component_type, on_change.clone())}
            
            // Common styling properties
            {render_common_styling_properties(component_properties, on_change.clone())}
        </div>
    }
}

fn render_header_properties(template_data: &UseStateHandle<serde_json::Value>, on_change: Callback<InputEvent>) -> Html {
    // Extract current values
    let text_color = template_data.get("text_color").and_then(|v| v.as_str()).unwrap_or("#ffffff");
    let height = template_data.get("height").and_then(|v| v.as_str()).unwrap_or("110px").trim_end_matches("px");
    let bg_type = template_data.get("bg_type").and_then(|v| v.as_str()).unwrap_or("color");
    let bg_color = template_data.get("bg_color").and_then(|v| v.as_str()).unwrap_or("#333333");
    let bg_image = template_data.get("bg_image").and_then(|v| v.as_str()).unwrap_or("");
    let bg_video = template_data.get("bg_video").and_then(|v| v.as_str()).unwrap_or("");
    
    // Shape mask properties
    let shape_mask_upper = template_data.get("shape_mask_upper").and_then(|v| v.as_str()).unwrap_or("none");
    let shape_mask_upper_scale = template_data.get("shape_mask_upper_scale").and_then(|v| v.as_str()).unwrap_or("100");
    let shape_mask_lower = template_data.get("shape_mask_lower").and_then(|v| v.as_str()).unwrap_or("none");
    let shape_mask_lower_scale = template_data.get("shape_mask_lower_scale").and_then(|v| v.as_str()).unwrap_or("100");
    
    // Effects properties
    let effects = template_data.get("effects").and_then(|v| v.as_str()).unwrap_or("none");

    html! {
        <>
            // Basic Properties
            <div class="property-section">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Basic Properties"}</h4>
                {render_color_field("Text Color", "text_color", text_color, on_change.clone())}
                {render_select_field("Position", "position", 
                    template_data.get("position").and_then(|v| v.as_str()).unwrap_or("sticky"), 
                    vec![("static", "Static"), ("sticky", "Sticky"), ("fixed", "Fixed")], on_change.clone())}
                {render_select_field("Logo Type", "logo_type", 
                    template_data.get("logo_type").and_then(|v| v.as_str()).unwrap_or("text"), 
                    vec![("text", "Text Logo"), ("image", "Image Logo")], on_change.clone())}
                
                {match template_data.get("logo_type").and_then(|v| v.as_str()).unwrap_or("text") {
                    "image" => html! {
                        <>
                            {render_input_field("Logo Image URL", "logo_url", 
                                template_data.get("logo_url").and_then(|v| v.as_str()).unwrap_or(""), on_change.clone())}
                            {render_input_field("Logo Height", "logo_height", 
                                template_data.get("logo_height").and_then(|v| v.as_str()).unwrap_or("40px"), on_change.clone())}
                            {render_input_field("Logo Width", "logo_width", 
                                template_data.get("logo_width").and_then(|v| v.as_str()).unwrap_or("auto"), on_change.clone())}
                        </>
                    },
                    _ => html! {
                        <>
                            {render_input_field("Logo Text", "logo_text", 
                                template_data.get("logo_text").and_then(|v| v.as_str()).unwrap_or("My Site"), on_change.clone())}
                            {render_input_field("Logo Font Size", "logo_size", 
                                template_data.get("logo_size").and_then(|v| v.as_str()).unwrap_or("1.5rem"), on_change.clone())}
                        </>
                    }
                }}
                
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
                    "gradient" => html! {
                        <>
                            {render_color_field("Gradient Start Color", "bg_gradient_start", 
                                template_data.get("bg_gradient_start").and_then(|v| v.as_str()).unwrap_or("#667eea"), on_change.clone())}
                            {render_color_field("Gradient End Color", "bg_gradient_end", 
                                template_data.get("bg_gradient_end").and_then(|v| v.as_str()).unwrap_or("#764ba2"), on_change.clone())}
                            {render_select_field("Gradient Direction", "bg_gradient_direction", 
                                template_data.get("bg_gradient_direction").and_then(|v| v.as_str()).unwrap_or("to-right"), 
                                vec![
                                    ("to-right", "Left to Right"),
                                    ("to-left", "Right to Left"),
                                    ("to-bottom", "Top to Bottom"),
                                    ("to-top", "Bottom to Top"),
                                    ("to-bottom-right", "Top-Left to Bottom-Right"),
                                    ("to-bottom-left", "Top-Right to Bottom-Left"),
                                    ("to-top-right", "Bottom-Left to Top-Right"),
                                    ("to-top-left", "Bottom-Right to Top-Left"),
                                    ("135deg", "Diagonal (135°)"),
                                    ("45deg", "Diagonal (45°)")
                                ], on_change.clone())}
                            {render_input_field("Custom Gradient (CSS)", "bg_gradient_custom", 
                                template_data.get("bg_gradient_custom").and_then(|v| v.as_str()).unwrap_or(""), on_change.clone())}
                        </>
                    },
                    "video" => render_input_field("Background Video URL", "bg_video", bg_video, on_change.clone()),
                    _ => html! {}
                }}
            </div>
            
            // Navigation Properties
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Navigation"}</h4>
                {render_color_field("Navigation Hover Color", "nav_hover_color", 
                    template_data.get("nav_hover_color").and_then(|v| v.as_str()).unwrap_or("#f7fafc"), on_change.clone())}
                {render_color_field("Underline Color", "nav_underline_color", 
                    template_data.get("nav_underline_color").and_then(|v| v.as_str()).unwrap_or("#ffffff"), on_change.clone())}
                {render_input_field("Underline Thickness", "nav_underline_thickness", 
                    template_data.get("nav_underline_thickness").and_then(|v| v.as_str()).unwrap_or("2px"), on_change.clone())}
                {render_select_field("Underline Animation", "nav_underline_animation", 
                    template_data.get("nav_underline_animation").and_then(|v| v.as_str()).unwrap_or("none"), 
                    vec![("none", "None"), ("slide", "Slide"), ("fade", "Fade")], on_change.clone())}
            </div>

            // Button Properties
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Buttons"}</h4>
                {render_input_field("Primary Button Background", "button_primary_bg", 
                    template_data.get("button_primary_bg").and_then(|v| v.as_str()).unwrap_or("linear-gradient(135deg, rgba(255,255,255,0.2), rgba(255,255,255,0.1))"), on_change.clone())}
                {render_input_field("Primary Button Hover Background", "button_primary_hover_bg", 
                    template_data.get("button_primary_hover_bg").and_then(|v| v.as_str()).unwrap_or("linear-gradient(135deg, rgba(255,255,255,0.3), rgba(255,255,255,0.2))"), on_change.clone())}
                {render_color_field("Primary Button Text Color", "button_primary_text", 
                    template_data.get("button_primary_text").and_then(|v| v.as_str()).unwrap_or("#ffffff"), on_change.clone())}
            </div>
            
            // Effects Properties
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Effects"}</h4>
                {render_select_field("Visual Effects", "effects", effects, vec![
                    ("none", "None"),
                    ("glassmorphism", "Glassmorphism"),
                    ("neumorphism", "Neumorphism"),
                    ("claymorphism", "Claymorphism"),
                    ("cybermorphism", "Cybermorphism")
                ], on_change.clone())}
                
                {if effects != "none" {
                    html! {
                        {render_range_field("Multiply Intensity (%)", "effects_intensity", 
                            template_data.get("effects_intensity").and_then(|v| v.as_str()).unwrap_or("50"), 
                            "0", "100", on_change.clone())}
                    }
                } else { html! {} }}
            </div>

            // Scroll Effects Properties
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Scroll Effects"}</h4>
                {render_select_field("Scroll Effect", "scroll_effect", 
                    template_data.get("scroll_effect").and_then(|v| v.as_str()).unwrap_or("none"), 
                    vec![
                        ("none", "None"),
                        ("shrink", "Shrink Effect"),
                        ("fade", "Fade Effect"),
                        ("slide", "Slide Effect"),
                        ("blur", "Blur Effect")
                    ], on_change.clone())}
                
                {if template_data.get("scroll_effect").and_then(|v| v.as_str()).unwrap_or("none") != "none" {
                    html! {
                        <>
                            {render_range_field("Scroll Trigger (px)", "scroll_trigger", 
                                template_data.get("scroll_trigger").and_then(|v| v.as_str()).unwrap_or("100"), 
                                "50", "500", on_change.clone())}
                            {render_range_field("Animation Duration (ms)", "scroll_duration", 
                                template_data.get("scroll_duration").and_then(|v| v.as_str()).unwrap_or("300"), 
                                "100", "2500", on_change.clone())}
                            {render_select_field("Easing", "scroll_easing", 
                                template_data.get("scroll_easing").and_then(|v| v.as_str()).unwrap_or("elastic"), 
                                vec![
                                    ("linear", "Linear"),
                                    ("ease", "Ease"),
                                    ("ease-in", "Ease In"),
                                    ("ease-out", "Ease Out"),
                                    ("ease-in-out", "Ease In Out"),
                                    ("elastic", "Elastic"),
                                    ("bounce", "Bounce")
                                ], on_change.clone())}
                        </>
                    }
                } else { html! {} }}
                
                {if template_data.get("scroll_effect").and_then(|v| v.as_str()).unwrap_or("none") == "shrink" {
                    html! {
                        <>
                            {render_range_field("Shrink Height (px)", "shrink_height", 
                                template_data.get("shrink_height").and_then(|v| v.as_str()).unwrap_or("500"), 
                                "40", "600", on_change.clone())}
                            {render_range_field("Logo Scale (%)", "shrink_logo_scale", 
                                template_data.get("shrink_logo_scale").and_then(|v| v.as_str()).unwrap_or("80"), 
                                "10", "100", on_change.clone())}
                        </>
                    }
                } else { html! {} }}
            </div>

            // Shape Mask Properties
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Shape Masks"}</h4>
                {render_select_field("Upper Shape", "shape_mask_upper", shape_mask_upper, vec![
                        ("none", "None"),
                        ("wave", "Wave"),
                        ("curve", "Curve"),
                        ("triangle", "Triangle"),
                    ("tilt", "Tilt"),
                    ("zigzag", "Zigzag")
                    ], on_change.clone())}
                    
                    {if shape_mask_upper != "none" {
                        html! {
                        <>
                            {render_range_field("Upper Scale (%)", "shape_mask_upper_scale", shape_mask_upper_scale, "10", "200", on_change.clone())}
                            {if matches!(shape_mask_upper, "wave" | "triangle" | "zigzag") {
                                render_range_field("Upper Frequency", "shape_mask_upper_frequency", 
                                    template_data.get("shape_mask_upper_frequency").and_then(|v| v.as_str()).unwrap_or("2"), 
                                    "1", "10", on_change.clone())
                            } else { html! {} }}
                            {if matches!(shape_mask_upper, "curve" | "tilt") {
                                render_select_field("Upper Direction", "shape_mask_upper_direction", 
                                    template_data.get("shape_mask_upper_direction").and_then(|v| v.as_str()).unwrap_or("positive"), 
                                    vec![("positive", "Positive"), ("negative", "Negative")], on_change.clone())
                            } else { html! {} }}
                            {if shape_mask_upper == "curve" {
                                render_range_field("Upper Amplitude", "shape_mask_upper_amplitude", 
                                    template_data.get("shape_mask_upper_amplitude").and_then(|v| v.as_str()).unwrap_or("50"), 
                                    "10", "100", on_change.clone())
                            } else { html! {} }}
                            {if shape_mask_upper == "tilt" {
                                render_range_field("Upper Degrees", "shape_mask_upper_degrees", 
                                    template_data.get("shape_mask_upper_degrees").and_then(|v| v.as_str()).unwrap_or("15"), 
                                    "1", "45", on_change.clone())
                            } else { html! {} }}
                        </>
                    }
                } else { html! {} }}

                {render_select_field("Lower Shape", "shape_mask_lower", shape_mask_lower, vec![
                    ("none", "None"),
                    ("wave", "Wave"),
                    ("curve", "Curve"),
                    ("triangle", "Triangle"),
                    ("tilt", "Tilt"),
                    ("zigzag", "Zigzag")
                ], on_change.clone())}
                
                {if shape_mask_lower != "none" {
                    html! {
                        <>
                            {render_range_field("Lower Scale (%)", "shape_mask_lower_scale", shape_mask_lower_scale, "10", "200", on_change.clone())}
                            {if matches!(shape_mask_lower, "wave" | "triangle" | "zigzag") {
                                render_range_field("Lower Frequency", "shape_mask_lower_frequency", 
                                    template_data.get("shape_mask_lower_frequency").and_then(|v| v.as_str()).unwrap_or("2"), 
                                    "1", "10", on_change.clone())
                            } else { html! {} }}
                            {if matches!(shape_mask_lower, "curve" | "tilt") {
                                render_select_field("Lower Direction", "shape_mask_lower_direction", 
                                    template_data.get("shape_mask_lower_direction").and_then(|v| v.as_str()).unwrap_or("positive"), 
                                    vec![("positive", "Positive"), ("negative", "Negative")], on_change.clone())
                            } else { html! {} }}
                            {if shape_mask_lower == "curve" {
                                render_range_field("Lower Amplitude", "shape_mask_lower_amplitude", 
                                    template_data.get("shape_mask_lower_amplitude").and_then(|v| v.as_str()).unwrap_or("50"), 
                                    "10", "100", on_change.clone())
                            } else { html! {} }}
                            {if shape_mask_lower == "tilt" {
                                render_range_field("Lower Degrees", "shape_mask_lower_degrees", 
                                    template_data.get("shape_mask_lower_degrees").and_then(|v| v.as_str()).unwrap_or("15"), 
                                    "1", "45", on_change.clone())
                            } else { html! {} }}
                        </>
                    }
                } else { html! {} }}
            </div>

            // Logo Effects Properties (SVG Only)
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Logo Effects"}</h4>
                <div style="margin-bottom: 8px; padding: 8px; background: #f8f9fa; border-radius: 4px; font-size: 12px; color: #666;">
                    {"⚠️ Logo effects only work with SVG images"}
                </div>
                
                {render_select_field("Logo Effect", "logo_effect", 
                    template_data.get("logo_effect").and_then(|v| v.as_str()).unwrap_or("none"), 
                    vec![
                        ("none", "None"),
                        ("pulsate", "Pulsate SVG Edges")
                    ], on_change.clone())}
                
                {if template_data.get("logo_effect").and_then(|v| v.as_str()).unwrap_or("none") == "pulsate" {
                    html! {
                        <>
                            {render_decimal_range_field("Pulsate Frequency (Hz)", "logo_pulsate_frequency", 
                                template_data.get("logo_pulsate_frequency").and_then(|v| v.as_str()).unwrap_or("1.5"), 
                                "0.1", "5.0", "0.1", on_change.clone())}
                            {render_decimal_range_field("Decay Rate", "logo_pulsate_decay", 
                                template_data.get("logo_pulsate_decay").and_then(|v| v.as_str()).unwrap_or("0.8"), 
                                "0.1", "1.0", "0.1", on_change.clone())}
                            {render_range_field("Opacity (%)", "logo_pulsate_opacity", 
                                template_data.get("logo_pulsate_opacity").and_then(|v| v.as_str()).unwrap_or("70"), 
                                "10", "100", on_change.clone())}
                            {render_range_field("Duration (s)", "logo_pulsate_duration", 
                                template_data.get("logo_pulsate_duration").and_then(|v| v.as_str()).unwrap_or("3"), 
                                "1", "10", on_change.clone())}
                            {render_range_field("Speed Multiplier", "logo_pulsate_anim_frequency", 
                                template_data.get("logo_pulsate_anim_frequency").and_then(|v| v.as_str()).unwrap_or("2"), 
                                "1", "8", on_change.clone())}
                        </>
                    }
                } else { html! {} }}
            </div>

            // Intro Animation Properties (as requested - this is the "legacy animation" section)
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Intro Animation"}</h4>
                {render_select_field("Animation Type", "animation_type", 
                    template_data.get("animation_type").and_then(|v| v.as_str()).unwrap_or("none"), vec![
                    ("none", "None"),
                    ("fade-in", "Fade In"),
                    ("slide-up", "Slide Up"),
                    ("slide-down", "Slide Down"),
                    ("slide-left", "Slide Left"),
                    ("slide-right", "Slide Right"),
                    ("zoom-in", "Zoom In"),
                    ("zoom-out", "Zoom Out")
                ], on_change.clone())}
                {render_input_field("Duration", "animation_duration", 
                    template_data.get("animation_duration").and_then(|v| v.as_str()).unwrap_or("0.6s"), on_change.clone())}
                {render_input_field("Delay", "animation_delay", 
                    template_data.get("animation_delay").and_then(|v| v.as_str()).unwrap_or("0s"), on_change.clone())}
            </div>
        </>
    }
}

fn render_footer_properties(template_data: &UseStateHandle<serde_json::Value>, on_change: Callback<InputEvent>) -> Html {
    // Footer properties are similar to header but with footer-specific styling
    let bg_color = template_data.get("bg_color").and_then(|v| v.as_str()).unwrap_or("#000000");
    let text_color = template_data.get("text_color").and_then(|v| v.as_str()).unwrap_or("#ffffff");
    let text_muted = template_data.get("text_muted").and_then(|v| v.as_str()).unwrap_or("#e2e8f0");
    let height = template_data.get("height").and_then(|v| v.as_str()).unwrap_or("auto");
    
    // Shape mask properties
    let shape_mask_upper = template_data.get("shape_mask_upper").and_then(|v| v.as_str()).unwrap_or("none");
    let shape_mask_upper_scale = template_data.get("shape_mask_upper_scale").and_then(|v| v.as_str()).unwrap_or("100");
    let shape_mask_lower = template_data.get("shape_mask_lower").and_then(|v| v.as_str()).unwrap_or("none");
    let shape_mask_lower_scale = template_data.get("shape_mask_lower_scale").and_then(|v| v.as_str()).unwrap_or("100");
    
    // Effects properties
    let effects = template_data.get("effects").and_then(|v| v.as_str()).unwrap_or("none");

    html! {
        <>
            // Basic Properties
            <div class="property-section">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Basic Properties"}</h4>
                {render_color_field("Background Color", "bg_color", bg_color, on_change.clone())}
                {render_color_field("Text Color", "text_color", text_color, on_change.clone())}
                {render_color_field("Muted Text Color", "text_muted", text_muted, on_change.clone())}
                {render_input_field("Height", "height", height, on_change.clone())}
                            </div>

            // Effects Properties
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Effects"}</h4>
                {render_select_field("Visual Effects", "effects", effects, vec![
                    ("none", "None"),
                    ("glassmorphism", "Glassmorphism"),
                    ("neumorphism", "Neumorphism"),
                    ("claymorphism", "Claymorphism"),
                    ("cybermorphism", "Cybermorphism")
                ], on_change.clone())}
                
                {if effects != "none" {
                    html! {
                        {render_range_field("Multiply Intensity (%)", "effects_intensity", 
                            template_data.get("effects_intensity").and_then(|v| v.as_str()).unwrap_or("50"), 
                            "0", "100", on_change.clone())}
                    }
                } else { html! {} }}
                </div>
                
            // Shape Mask Properties
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Shape Masks"}</h4>
                {render_select_field("Upper Shape", "shape_mask_upper", shape_mask_upper, vec![
                        ("none", "None"),
                        ("wave", "Wave"),
                        ("curve", "Curve"),
                        ("triangle", "Triangle"),
                    ("tilt", "Tilt"),
                    ("zigzag", "Zigzag")
                ], on_change.clone())}
                
                {if shape_mask_upper != "none" {
                    html! {
                        <>
                            {render_range_field("Upper Scale (%)", "shape_mask_upper_scale", shape_mask_upper_scale, "10", "200", on_change.clone())}
                            {if matches!(shape_mask_upper, "wave" | "triangle" | "zigzag") {
                                render_range_field("Upper Frequency", "shape_mask_upper_frequency", 
                                    template_data.get("shape_mask_upper_frequency").and_then(|v| v.as_str()).unwrap_or("2"), 
                                    "1", "10", on_change.clone())
                            } else { html! {} }}
                            {if matches!(shape_mask_upper, "curve" | "tilt") {
                                render_select_field("Upper Direction", "shape_mask_upper_direction", 
                                    template_data.get("shape_mask_upper_direction").and_then(|v| v.as_str()).unwrap_or("positive"), 
                                    vec![("positive", "Positive"), ("negative", "Negative")], on_change.clone())
                            } else { html! {} }}
                            {if shape_mask_upper == "curve" {
                                render_range_field("Upper Amplitude", "shape_mask_upper_amplitude", 
                                    template_data.get("shape_mask_upper_amplitude").and_then(|v| v.as_str()).unwrap_or("50"), 
                                    "10", "100", on_change.clone())
                            } else { html! {} }}
                            {if shape_mask_upper == "tilt" {
                                render_range_field("Upper Degrees", "shape_mask_upper_degrees", 
                                    template_data.get("shape_mask_upper_degrees").and_then(|v| v.as_str()).unwrap_or("15"), 
                                    "1", "45", on_change.clone())
                            } else { html! {} }}
                        </>
                    }
                } else { html! {} }}

                {render_select_field("Lower Shape", "shape_mask_lower", shape_mask_lower, vec![
                    ("none", "None"),
                    ("wave", "Wave"),
                    ("curve", "Curve"),
                    ("triangle", "Triangle"),
                    ("tilt", "Tilt"),
                    ("zigzag", "Zigzag")
                    ], on_change.clone())}
                    
                    {if shape_mask_lower != "none" {
                        html! {
                        <>
                            {render_range_field("Lower Scale (%)", "shape_mask_lower_scale", shape_mask_lower_scale, "10", "200", on_change.clone())}
                            {if matches!(shape_mask_lower, "wave" | "triangle" | "zigzag") {
                                render_range_field("Lower Frequency", "shape_mask_lower_frequency", 
                                    template_data.get("shape_mask_lower_frequency").and_then(|v| v.as_str()).unwrap_or("2"), 
                                    "1", "10", on_change.clone())
                            } else { html! {} }}
                            {if matches!(shape_mask_lower, "curve" | "tilt") {
                                render_select_field("Lower Direction", "shape_mask_lower_direction", 
                                    template_data.get("shape_mask_lower_direction").and_then(|v| v.as_str()).unwrap_or("positive"), 
                                    vec![("positive", "Positive"), ("negative", "Negative")], on_change.clone())
                            } else { html! {} }}
                            {if shape_mask_lower == "curve" {
                                render_range_field("Lower Amplitude", "shape_mask_lower_amplitude", 
                                    template_data.get("shape_mask_lower_amplitude").and_then(|v| v.as_str()).unwrap_or("50"), 
                                    "10", "100", on_change.clone())
                            } else { html! {} }}
                            {if shape_mask_lower == "tilt" {
                                render_range_field("Lower Degrees", "shape_mask_lower_degrees", 
                                    template_data.get("shape_mask_lower_degrees").and_then(|v| v.as_str()).unwrap_or("15"), 
                                    "1", "45", on_change.clone())
                            } else { html! {} }}
                        </>
                    }
                } else { html! {} }}
            </div>

            // Intro Animation Properties
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Intro Animation"}</h4>
                {render_select_field("Animation Type", "intro_animation_type", 
                    template_data.get("intro_animation_type").and_then(|v| v.as_str()).unwrap_or("none"), vec![
                    ("none", "None"),
                    ("fade-in", "Fade In"),
                    ("fade-in-up", "Fade In Up"),
                    ("fade-in-down", "Fade In Down"),
                    ("fade-in-left", "Fade In Left"),
                    ("fade-in-right", "Fade In Right"),
                    ("slide-up", "Slide Up"),
                    ("slide-down", "Slide Down"),
                    ("slide-left", "Slide Left"),
                    ("slide-right", "Slide Right"),
                    ("zoom-in", "Zoom In"),
                    ("zoom-out", "Zoom Out"),
                    ("bounce-in", "Bounce In"),
                    ("flip-in-x", "Flip In X"),
                    ("flip-in-y", "Flip In Y"),
                    ("rotate-in", "Rotate In"),
                    ("scale-in", "Scale In")
                ], on_change.clone())}
                {render_input_field("Duration", "intro_animation_duration", 
                    template_data.get("intro_animation_duration").and_then(|v| v.as_str()).unwrap_or("0.6s"), on_change.clone())}
                {render_input_field("Delay", "intro_animation_delay", 
                    template_data.get("intro_animation_delay").and_then(|v| v.as_str()).unwrap_or("0s"), on_change.clone())}
                {render_select_field("Easing", "intro_animation_easing", 
                    template_data.get("intro_animation_easing").and_then(|v| v.as_str()).unwrap_or("ease-out"), vec![
                    ("ease", "Ease"),
                    ("ease-in", "Ease In"),
                    ("ease-out", "Ease Out"),
                    ("ease-in-out", "Ease In Out"),
                    ("linear", "Linear"),
                    ("cubic-bezier(0.68, -0.55, 0.265, 1.55)", "Bounce"),
                    ("cubic-bezier(0.25, 0.46, 0.45, 0.94)", "Smooth")
                ], on_change.clone())}
                {render_select_field("Trigger", "intro_animation_trigger", 
                    template_data.get("intro_animation_trigger").and_then(|v| v.as_str()).unwrap_or("scroll"), vec![
                    ("load", "On Load"),
                    ("scroll", "On Scroll"),
                    ("hover", "On Hover"),
                    ("click", "On Click")
                ], on_change.clone())}
                {if template_data.get("intro_animation_trigger").and_then(|v| v.as_str()).unwrap_or("scroll") == "scroll" {
                    html! {
                        {render_input_field("Scroll Offset", "intro_animation_offset", 
                            template_data.get("intro_animation_offset").and_then(|v| v.as_str()).unwrap_or("100px"), on_change.clone())}
                    }
                } else {
                    html! {}
                }}
                {render_checkbox_field("Repeat Animation", "intro_animation_repeat", 
                    template_data.get("intro_animation_repeat").and_then(|v| v.as_bool()).unwrap_or(false), on_change.clone())}
                
                // Legacy Animation (keeping for backward compatibility)
                <div style="margin-top: 12px; padding-top: 12px; border-top: 1px solid #eee;">
                    <h5 style="margin: 0 0 8px 0; font-size: 12px; color: #666; font-weight: 600;">{"Legacy Animation"}</h5>
                    {render_select_field("Animation Type", "animation_type", 
                        template_data.get("animation_type").and_then(|v| v.as_str()).unwrap_or("none"), vec![
                        ("none", "None"),
                        ("fade-in", "Fade In"),
                        ("slide-up", "Slide Up"),
                        ("slide-down", "Slide Down"),
                        ("slide-left", "Slide Left"),
                        ("slide-right", "Slide Right"),
                        ("zoom-in", "Zoom In"),
                        ("zoom-out", "Zoom Out")
                    ], on_change.clone())}
                    {render_input_field("Duration", "animation_duration", 
                        template_data.get("animation_duration").and_then(|v| v.as_str()).unwrap_or("0.6s"), on_change.clone())}
                    {render_input_field("Delay", "animation_delay", 
                        template_data.get("animation_delay").and_then(|v| v.as_str()).unwrap_or("0s"), on_change.clone())}
                </div>
            </div>
        </>
    }
}

fn render_container_properties(template_data: &UseStateHandle<serde_json::Value>, on_change: Callback<InputEvent>) -> Html {
    // Container properties
    let max_width = template_data.get("max_width").and_then(|v| v.as_str()).unwrap_or("1200px");
    let padding = template_data.get("padding").and_then(|v| v.as_str()).unwrap_or("1rem");
    let margin = template_data.get("margin").and_then(|v| v.as_str()).unwrap_or("0 auto");
    let width_type = template_data.get("width_type").and_then(|v| v.as_str()).unwrap_or("fixed");
    
    // Background & Media Overlay properties
    let bg_type = template_data.get("bg_type").and_then(|v| v.as_str()).unwrap_or("color");
    let bg_color = template_data.get("bg_color").and_then(|v| v.as_str()).unwrap_or("#ffffff");
    let bg_image = template_data.get("bg_image").and_then(|v| v.as_str()).unwrap_or("");
    let bg_video = template_data.get("bg_video").and_then(|v| v.as_str()).unwrap_or("");
    let overlay_color = template_data.get("overlay_color").and_then(|v| v.as_str()).unwrap_or("#000000");
    let overlay_opacity = template_data.get("overlay_opacity").and_then(|v| v.as_str()).unwrap_or("0.3");
    
    // Animation properties
    let _animation = template_data.get("animation").and_then(|v| v.as_str()).unwrap_or("none");
    
    // Shape mask properties
    let shape_mask_upper = template_data.get("shape_mask_upper").and_then(|v| v.as_str()).unwrap_or("none");
    let shape_mask_upper_scale = template_data.get("shape_mask_upper_scale").and_then(|v| v.as_str()).unwrap_or("100");
    let shape_mask_lower = template_data.get("shape_mask_lower").and_then(|v| v.as_str()).unwrap_or("none");
    let shape_mask_lower_scale = template_data.get("shape_mask_lower_scale").and_then(|v| v.as_str()).unwrap_or("100");
    
    // Effects properties
    let effects = template_data.get("effects").and_then(|v| v.as_str()).unwrap_or("none");

    html! {
        <>
            // Basic Properties
            <div class="property-section">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Container Properties"}</h4>
                {render_select_field("Width Type", "width_type", width_type, vec![
                    ("fixed", "Fixed Width"),
                    ("fluid", "Fluid Width"),
                    ("full", "Full Width")
                ], on_change.clone())}
                {render_input_field("Max Width", "max_width", max_width, on_change.clone())}
                {render_input_field("Padding", "padding", padding, on_change.clone())}
                {render_input_field("Margin", "margin", margin, on_change.clone())}
            </div>

            // Background & Media Overlay Properties
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Background & Media Overlay"}</h4>
                {render_select_field("Background Type", "bg_type", bg_type, vec![
                    ("color", "Color"),
                    ("image", "Image"),
                    ("gradient", "Gradient"),
                    ("video", "Video")
                ], on_change.clone())}
                
                {match bg_type {
                    "color" => render_color_field("Background Color", "bg_color", bg_color, on_change.clone()),
                    "image" => render_input_field("Background Image URL", "bg_image", bg_image, on_change.clone()),
                    "gradient" => html! {
                        <>
                            {render_color_field("Gradient Start Color", "bg_gradient_start", 
                                template_data.get("bg_gradient_start").and_then(|v| v.as_str()).unwrap_or("#667eea"), on_change.clone())}
                            {render_color_field("Gradient End Color", "bg_gradient_end", 
                                template_data.get("bg_gradient_end").and_then(|v| v.as_str()).unwrap_or("#764ba2"), on_change.clone())}
                            {render_select_field("Gradient Direction", "bg_gradient_direction", 
                                template_data.get("bg_gradient_direction").and_then(|v| v.as_str()).unwrap_or("to-right"), 
                                vec![
                                    ("to-right", "Left to Right"),
                                    ("to-left", "Right to Left"),
                                    ("to-bottom", "Top to Bottom"),
                                    ("to-top", "Bottom to Top"),
                                    ("to-bottom-right", "Top-Left to Bottom-Right"),
                                    ("to-bottom-left", "Top-Right to Bottom-Left"),
                                    ("to-top-right", "Bottom-Left to Top-Right"),
                                    ("to-top-left", "Bottom-Right to Top-Left"),
                                    ("135deg", "Diagonal (135°)"),
                                    ("45deg", "Diagonal (45°)")
                                ], on_change.clone())}
                            {render_input_field("Custom Gradient (CSS)", "bg_gradient_custom", 
                                template_data.get("bg_gradient_custom").and_then(|v| v.as_str()).unwrap_or(""), on_change.clone())}
                        </>
                    },
                    "video" => render_input_field("Background Video URL", "bg_video", bg_video, on_change.clone()),
                    _ => html! {}
                }}
                
                // Overlay Properties (shown for all background types, but only applied to image/video)
                <div style="margin-top: 12px; padding-top: 12px; border-top: 1px solid #eee;">
                    <h5 style="margin: 0 0 8px 0; font-size: 12px; color: #666; font-weight: 600;">{"Media Overlay"}</h5>
                    {render_color_field("Overlay Color", "overlay_color", overlay_color, on_change.clone())}
                    {render_range_field("Overlay Opacity", "overlay_opacity", overlay_opacity, "0", "1", on_change.clone())}
                    {if !matches!(bg_type, "image" | "video") {
                        html! {
                            <p style="font-size: 11px; color: #999; margin: 4px 0 0 0; font-style: italic;">
                                {"Note: Overlay only applies to image and video backgrounds"}
                            </p>
                        }
                    } else { html! {} }}
                </div>
            </div>

            // Intro Animation Properties
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Intro Animation"}</h4>
                {render_select_field("Animation Type", "intro_animation_type", 
                    template_data.get("intro_animation_type").and_then(|v| v.as_str()).unwrap_or("none"), vec![
                    ("none", "None"),
                    ("fade-in", "Fade In"),
                    ("fade-in-up", "Fade In Up"),
                    ("fade-in-down", "Fade In Down"),
                    ("fade-in-left", "Fade In Left"),
                    ("fade-in-right", "Fade In Right"),
                    ("slide-up", "Slide Up"),
                    ("slide-down", "Slide Down"),
                    ("slide-left", "Slide Left"),
                    ("slide-right", "Slide Right"),
                    ("zoom-in", "Zoom In"),
                    ("zoom-out", "Zoom Out"),
                    ("bounce-in", "Bounce In"),
                    ("flip-in-x", "Flip In X"),
                    ("flip-in-y", "Flip In Y"),
                    ("rotate-in", "Rotate In"),
                    ("scale-in", "Scale In")
                ], on_change.clone())}
                {render_input_field("Duration", "intro_animation_duration", 
                    template_data.get("intro_animation_duration").and_then(|v| v.as_str()).unwrap_or("0.6s"), on_change.clone())}
                {render_input_field("Delay", "intro_animation_delay", 
                    template_data.get("intro_animation_delay").and_then(|v| v.as_str()).unwrap_or("0s"), on_change.clone())}
                {render_select_field("Easing", "intro_animation_easing", 
                    template_data.get("intro_animation_easing").and_then(|v| v.as_str()).unwrap_or("ease-out"), vec![
                    ("ease", "Ease"),
                    ("ease-in", "Ease In"),
                    ("ease-out", "Ease Out"),
                    ("ease-in-out", "Ease In Out"),
                    ("linear", "Linear"),
                    ("cubic-bezier(0.68, -0.55, 0.265, 1.55)", "Bounce"),
                    ("cubic-bezier(0.25, 0.46, 0.45, 0.94)", "Smooth")
                ], on_change.clone())}
                {render_select_field("Trigger", "intro_animation_trigger", 
                    template_data.get("intro_animation_trigger").and_then(|v| v.as_str()).unwrap_or("scroll"), vec![
                    ("load", "On Load"),
                    ("scroll", "On Scroll"),
                    ("hover", "On Hover"),
                    ("click", "On Click")
                ], on_change.clone())}
                {if template_data.get("intro_animation_trigger").and_then(|v| v.as_str()).unwrap_or("scroll") == "scroll" {
                    html! {
                        {render_input_field("Scroll Offset", "intro_animation_offset", 
                            template_data.get("intro_animation_offset").and_then(|v| v.as_str()).unwrap_or("100px"), on_change.clone())}
                    }
                } else {
                    html! {}
                }}
                {render_checkbox_field("Repeat Animation", "intro_animation_repeat", 
                    template_data.get("intro_animation_repeat").and_then(|v| v.as_bool()).unwrap_or(false), on_change.clone())}
                
                // Legacy Animation (keeping for backward compatibility)
                <div style="margin-top: 12px; padding-top: 12px; border-top: 1px solid #eee;">
                    <h5 style="margin: 0 0 8px 0; font-size: 12px; color: #666; font-weight: 600;">{"Legacy Animation"}</h5>
                    {render_select_field("Animation Type", "animation_type", 
                        template_data.get("animation_type").and_then(|v| v.as_str()).unwrap_or("none"), vec![
                        ("none", "None"),
                        ("fade-in", "Fade In"),
                        ("slide-up", "Slide Up"),
                        ("slide-down", "Slide Down"),
                        ("slide-left", "Slide Left"),
                        ("slide-right", "Slide Right"),
                        ("zoom-in", "Zoom In"),
                        ("zoom-out", "Zoom Out")
                    ], on_change.clone())}
                    {render_input_field("Duration", "animation_duration", 
                        template_data.get("animation_duration").and_then(|v| v.as_str()).unwrap_or("0.6s"), on_change.clone())}
                    {render_input_field("Delay", "animation_delay", 
                        template_data.get("animation_delay").and_then(|v| v.as_str()).unwrap_or("0s"), on_change.clone())}
                </div>
            </div>

            // Effects Properties
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Effects"}</h4>
                {render_select_field("Visual Effects", "effects", effects, vec![
                    ("none", "None"),
                    ("glassmorphism", "Glassmorphism"),
                    ("neumorphism", "Neumorphism"),
                    ("claymorphism", "Claymorphism"),
                    ("cybermorphism", "Cybermorphism")
                ], on_change.clone())}
                
                {if effects != "none" {
                    html! {
                        {render_range_field("Multiply Intensity (%)", "effects_intensity", 
                            template_data.get("effects_intensity").and_then(|v| v.as_str()).unwrap_or("50"), 
                            "0", "100", on_change.clone())}
                    }
                } else { html! {} }}
            </div>

            // Shape Mask Properties
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Shape Masks"}</h4>
                {render_select_field("Upper Shape", "shape_mask_upper", shape_mask_upper, vec![
                    ("none", "None"),
                    ("wave", "Wave"),
                    ("curve", "Curve"),
                    ("triangle", "Triangle"),
                    ("tilt", "Tilt"),
                    ("zigzag", "Zigzag")
                ], on_change.clone())}
                
                {if shape_mask_upper != "none" {
                    html! {
                        <>
                            {render_range_field("Upper Scale (%)", "shape_mask_upper_scale", shape_mask_upper_scale, "10", "200", on_change.clone())}
                            {if matches!(shape_mask_upper, "wave" | "triangle" | "zigzag") {
                                render_range_field("Upper Frequency", "shape_mask_upper_frequency", 
                                    template_data.get("shape_mask_upper_frequency").and_then(|v| v.as_str()).unwrap_or("2"), 
                                    "1", "10", on_change.clone())
                            } else { html! {} }}
                            {if matches!(shape_mask_upper, "curve" | "tilt") {
                                render_select_field("Upper Direction", "shape_mask_upper_direction", 
                                    template_data.get("shape_mask_upper_direction").and_then(|v| v.as_str()).unwrap_or("positive"), 
                                    vec![("positive", "Positive"), ("negative", "Negative")], on_change.clone())
                            } else { html! {} }}
                            {if shape_mask_upper == "curve" {
                                render_range_field("Upper Amplitude", "shape_mask_upper_amplitude", 
                                    template_data.get("shape_mask_upper_amplitude").and_then(|v| v.as_str()).unwrap_or("50"), 
                                    "10", "100", on_change.clone())
                            } else { html! {} }}
                            {if shape_mask_upper == "tilt" {
                                render_range_field("Upper Degrees", "shape_mask_upper_degrees", 
                                    template_data.get("shape_mask_upper_degrees").and_then(|v| v.as_str()).unwrap_or("15"), 
                                    "1", "45", on_change.clone())
                            } else { html! {} }}
                        </>
                    }
                } else { html! {} }}

                {render_select_field("Lower Shape", "shape_mask_lower", shape_mask_lower, vec![
                    ("none", "None"),
                    ("wave", "Wave"),
                    ("curve", "Curve"),
                    ("triangle", "Triangle"),
                    ("tilt", "Tilt"),
                    ("zigzag", "Zigzag")
                ], on_change.clone())}
                
                {if shape_mask_lower != "none" {
                    html! {
                        <>
                            {render_range_field("Lower Scale (%)", "shape_mask_lower_scale", shape_mask_lower_scale, "10", "200", on_change.clone())}
                            {if matches!(shape_mask_lower, "wave" | "triangle" | "zigzag") {
                                render_range_field("Lower Frequency", "shape_mask_lower_frequency", 
                                    template_data.get("shape_mask_lower_frequency").and_then(|v| v.as_str()).unwrap_or("2"), 
                                    "1", "10", on_change.clone())
                            } else { html! {} }}
                            {if matches!(shape_mask_lower, "curve" | "tilt") {
                                render_select_field("Lower Direction", "shape_mask_lower_direction", 
                                    template_data.get("shape_mask_lower_direction").and_then(|v| v.as_str()).unwrap_or("positive"), 
                                    vec![("positive", "Positive"), ("negative", "Negative")], on_change.clone())
                            } else { html! {} }}
                            {if shape_mask_lower == "curve" {
                                render_range_field("Lower Amplitude", "shape_mask_lower_amplitude", 
                                    template_data.get("shape_mask_lower_amplitude").and_then(|v| v.as_str()).unwrap_or("50"), 
                                    "10", "100", on_change.clone())
                            } else { html! {} }}
                            {if shape_mask_lower == "tilt" {
                                render_range_field("Lower Degrees", "shape_mask_lower_degrees", 
                                    template_data.get("shape_mask_lower_degrees").and_then(|v| v.as_str()).unwrap_or("15"), 
                                    "1", "45", on_change.clone())
                            } else { html! {} }}
                        </>
                    }
                } else { html! {} }}
            </div>
        </>
    }
}

// Component-specific property rendering functions
fn render_component_specific_properties(properties: &ComponentProperties, component_type: Option<&ComponentType>, on_change: Callback<InputEvent>) -> Html {
    html! {
        <div class="component-specific-properties">
            // Image Properties - only show for Image components
            {if matches!(component_type, Some(ComponentType::Image)) {
                html! {
                    <div class="property-section" style="margin-bottom: 16px;">
                        <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Image Properties"}</h4>
                        {render_input_field("Image URL", "image_url", &properties.image_url, on_change.clone())}
                        {render_input_field("Alt Text", "image_alt", &properties.image_alt, on_change.clone())}
                        {render_input_field("Title", "image_title", &properties.image_title, on_change.clone())}
                        {render_checkbox_field("Lazy Load", "image_lazy_load", properties.image_lazy_load, on_change.clone())}
                    </div>
                }
            } else {
                html! {}
            }}
            
            // Button Properties - only show for Button components
            {if matches!(component_type, Some(ComponentType::Button)) {
                html! {
                    <div class="property-section" style="margin-bottom: 16px;">
                        <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Button Properties"}</h4>
                        {render_input_field("Button Text", "button_text", &properties.button_text, on_change.clone())}
                        {render_input_field("Button URL", "button_url", &properties.button_url, on_change.clone())}
                        {render_select_field("Target", "button_target", &properties.button_target, vec![
                            ("_self", "Same Window"),
                            ("_blank", "New Window"),
                            ("_parent", "Parent Frame"),
                            ("_top", "Top Frame")
                        ], on_change.clone())}
                        {render_select_field("Size", "button_size", &properties.button_size, vec![
                            ("small", "Small"),
                            ("medium", "Medium"),
                            ("large", "Large")
                        ], on_change.clone())}
                        {render_select_field("Variant", "button_variant", &properties.button_variant, vec![
                            ("primary", "Primary"),
                            ("secondary", "Secondary"),
                            ("outline", "Outline"),
                            ("ghost", "Ghost")
                        ], on_change.clone())}
                        {render_input_field("Icon", "button_icon", &properties.button_icon, on_change.clone())}
                            </div>
                        }
                    } else {
                        html! {}
                    }}
            
            // Video Properties - only show for Video components
            {if matches!(component_type, Some(ComponentType::Video)) {
                html! {
                    <div class="property-section" style="margin-bottom: 16px;">
                        <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Video Properties"}</h4>
                        {render_input_field("Video URL", "video_url", &properties.video_url, on_change.clone())}
                        {render_checkbox_field("Autoplay", "video_autoplay", properties.video_autoplay, on_change.clone())}
                        {render_checkbox_field("Show Controls", "video_controls", properties.video_controls, on_change.clone())}
                        {render_checkbox_field("Muted", "video_muted", properties.video_muted, on_change.clone())}
                        {render_checkbox_field("Loop", "video_loop", properties.video_loop, on_change.clone())}
                </div>
                }
            } else {
                html! {}
            }}
            
            // Hero Properties - only show for Hero components
            {if matches!(component_type, Some(ComponentType::Hero)) {
                html! {
                    <div class="property-section" style="margin-bottom: 16px;">
                        <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Hero Properties"}</h4>
                        {render_input_field("Badge Text", "hero_badge_text", &properties.hero_badge_text, on_change.clone())}
                        {render_input_field("Title", "hero_title", &properties.hero_title, on_change.clone())}
                        {render_input_field("Subtitle", "hero_subtitle", &properties.hero_subtitle, on_change.clone())}
                        {render_input_field("Description", "hero_description", &properties.hero_description, on_change.clone())}
                        {render_select_field("Background Type", "hero_background_type", &properties.hero_background_type, vec![
                            ("solid", "Solid Color"),
                            ("gradient", "Gradient"),
                            ("image", "Image")
                        ], on_change.clone())}
                        {render_color_field("Background Color", "hero_background_color", &properties.hero_background_color, on_change.clone())}
                        {render_color_field("Text Color", "hero_text_color", &properties.hero_text_color, on_change.clone())}
                        {render_select_field("Alignment", "hero_alignment", &properties.hero_alignment, vec![
                            ("left", "Left"),
                            ("center", "Center"),
                            ("right", "Right")
                        ], on_change.clone())}
                        {render_input_field("Min Height", "hero_min_height", &properties.hero_min_height, on_change.clone())}
                        {render_checkbox_field("Show Badge", "hero_show_badge", properties.hero_show_badge, on_change.clone())}
                        {render_checkbox_field("Show Primary Button", "hero_show_primary_button", properties.hero_show_primary_button, on_change.clone())}
                        {render_checkbox_field("Show Secondary Button", "hero_show_secondary_button", properties.hero_show_secondary_button, on_change.clone())}
                        {if properties.hero_show_primary_button {
                            html! {
                                <>
                                    {render_input_field("Primary Button Text", "hero_primary_button_text", &properties.hero_primary_button_text, on_change.clone())}
                                    {render_input_field("Primary Button URL", "hero_primary_button_url", &properties.hero_primary_button_url, on_change.clone())}
                                </>
                            }
                        } else {
                            html! {}
                        }}
                        {if properties.hero_show_secondary_button {
                            html! {
                                <>
                                    {render_input_field("Secondary Button Text", "hero_secondary_button_text", &properties.hero_secondary_button_text, on_change.clone())}
                                    {render_input_field("Secondary Button URL", "hero_secondary_button_url", &properties.hero_secondary_button_url, on_change.clone())}
                                </>
                            }
                        } else {
                            html! {}
                        }}
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}

fn render_common_styling_properties(properties: &ComponentProperties, on_change: Callback<InputEvent>) -> Html {
    html! {
        <div class="common-styling-properties">
            // Intro Animation Properties
            <div class="property-section" style="margin-bottom: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Intro Animation"}</h4>
                {render_select_field("Animation Type", "intro_animation_type", &properties.intro_animation_type, vec![
                    ("none", "None"),
                    ("fade-in", "Fade In"),
                    ("fade-in-up", "Fade In Up"),
                    ("fade-in-down", "Fade In Down"),
                    ("fade-in-left", "Fade In Left"),
                    ("fade-in-right", "Fade In Right"),
                    ("slide-up", "Slide Up"),
                    ("slide-down", "Slide Down"),
                    ("slide-left", "Slide Left"),
                    ("slide-right", "Slide Right"),
                    ("zoom-in", "Zoom In"),
                    ("zoom-out", "Zoom Out"),
                    ("bounce-in", "Bounce In"),
                    ("flip-in-x", "Flip In X"),
                    ("flip-in-y", "Flip In Y"),
                    ("rotate-in", "Rotate In"),
                    ("scale-in", "Scale In")
                ], on_change.clone())}
                {render_input_field("Duration", "intro_animation_duration", &properties.intro_animation_duration, on_change.clone())}
                {render_input_field("Delay", "intro_animation_delay", &properties.intro_animation_delay, on_change.clone())}
                {render_select_field("Easing", "intro_animation_easing", &properties.intro_animation_easing, vec![
                    ("ease", "Ease"),
                    ("ease-in", "Ease In"),
                    ("ease-out", "Ease Out"),
                    ("ease-in-out", "Ease In Out"),
                    ("linear", "Linear"),
                    ("cubic-bezier(0.68, -0.55, 0.265, 1.55)", "Bounce"),
                    ("cubic-bezier(0.25, 0.46, 0.45, 0.94)", "Smooth")
                ], on_change.clone())}
                {render_select_field("Trigger", "intro_animation_trigger", &properties.intro_animation_trigger, vec![
                    ("load", "On Load"),
                    ("scroll", "On Scroll"),
                    ("hover", "On Hover"),
                    ("click", "On Click")
                ], on_change.clone())}
                {if properties.intro_animation_trigger == "scroll" {
                    html! {
                        {render_input_field("Scroll Offset", "intro_animation_offset", &properties.intro_animation_offset, on_change.clone())}
                    }
                } else {
                    html! {}
                }}
                {render_checkbox_field("Repeat Animation", "intro_animation_repeat", properties.intro_animation_repeat, on_change.clone())}
            </div>

            // Legacy Animation Properties (for backward compatibility)
            <div class="property-section" style="margin-bottom: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Legacy Animation"}</h4>
                {render_select_field("Animation Type", "animation_type", &properties.animation_type, vec![
                    ("none", "None"),
                    ("fade-in", "Fade In"),
                    ("slide-up", "Slide Up"),
                    ("slide-down", "Slide Down"),
                    ("slide-left", "Slide Left"),
                    ("slide-right", "Slide Right"),
                    ("zoom-in", "Zoom In"),
                    ("zoom-out", "Zoom Out")
                ], on_change.clone())}
                {render_input_field("Duration", "animation_duration", &properties.animation_duration, on_change.clone())}
                {render_input_field("Delay", "animation_delay", &properties.animation_delay, on_change.clone())}
            </div>
            
            // SEO Properties
            <div class="property-section" style="margin-bottom: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"SEO & Accessibility"}</h4>
                {render_input_field("SEO Title", "seo_title", &properties.seo_title, on_change.clone())}
                {render_input_field("SEO Description", "seo_description", &properties.seo_description, on_change.clone())}
                {render_input_field("ARIA Label", "aria_label", &properties.aria_label, on_change.clone())}
                {render_input_field("ARIA Description", "aria_description", &properties.aria_description, on_change.clone())}
            </div>
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

fn render_checkbox_field(label: &str, name: &str, checked: bool, on_change: Callback<InputEvent>) -> Html {
    let label = label.to_string();
    let name = name.to_string();
    
    html! {
        <div style="margin-bottom: 12px;">
            <label style="display: flex; align-items: center; font-weight: 600; font-size: 12px; color: #555;">
                <input 
                    type="checkbox"
                    name={name}
                    checked={checked}
                    oninput={on_change}
                    style="margin-right: 8px;"
                />
                {label}
            </label>
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

fn render_decimal_range_field(label: &str, name: &str, value: &str, min: &str, max: &str, step: &str, on_change: Callback<InputEvent>) -> Html {
    let label = label.to_string();
    let name = name.to_string();
    let value = value.to_string();
    let min = min.to_string();
    let max = max.to_string();
    let step = step.to_string();
    
    html! {
        <div style="margin-bottom: 12px;">
            <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{format!("{} ({})", label, value)}</label>
            <input 
                type="range"
                name={name}
                value={value}
                min={min}
                max={max}
                step={step}
                oninput={on_change}
                style="
                    width: 100%;
                    margin-bottom: 4px;
                "
            />
        </div>
    }
}

// Helper function to update component properties
fn update_component_property(props: &mut ComponentProperties, name: &str, value: &str, is_checkbox: bool) {
    match name {
        // Image properties
        "image_url" => props.image_url = value.to_string(),
        "image_alt" => props.image_alt = value.to_string(),
        "image_title" => props.image_title = value.to_string(),
        "image_lazy_load" => props.image_lazy_load = is_checkbox,
        
        // Button properties
        "button_text" => props.button_text = value.to_string(),
        "button_url" => props.button_url = value.to_string(),
        "button_target" => props.button_target = value.to_string(),
        "button_size" => props.button_size = value.to_string(),
        "button_variant" => props.button_variant = value.to_string(),
        "button_icon" => props.button_icon = value.to_string(),
        
        // Video properties
        "video_url" => props.video_url = value.to_string(),
        "video_autoplay" => props.video_autoplay = is_checkbox,
        "video_controls" => props.video_controls = is_checkbox,
        "video_muted" => props.video_muted = is_checkbox,
        "video_loop" => props.video_loop = is_checkbox,
        
        // Hero properties
        "hero_badge_text" => props.hero_badge_text = value.to_string(),
        "hero_title" => props.hero_title = value.to_string(),
        "hero_subtitle" => props.hero_subtitle = value.to_string(),
        "hero_description" => props.hero_description = value.to_string(),
        "hero_background_type" => props.hero_background_type = value.to_string(),
        "hero_background_color" => props.hero_background_color = value.to_string(),
        "hero_text_color" => props.hero_text_color = value.to_string(),
        "hero_alignment" => props.hero_alignment = value.to_string(),
        "hero_min_height" => props.hero_min_height = value.to_string(),
        "hero_show_badge" => props.hero_show_badge = is_checkbox,
        "hero_show_primary_button" => props.hero_show_primary_button = is_checkbox,
        "hero_show_secondary_button" => props.hero_show_secondary_button = is_checkbox,
        "hero_primary_button_text" => props.hero_primary_button_text = value.to_string(),
        "hero_primary_button_url" => props.hero_primary_button_url = value.to_string(),
        "hero_secondary_button_text" => props.hero_secondary_button_text = value.to_string(),
        "hero_secondary_button_url" => props.hero_secondary_button_url = value.to_string(),
        
        // Intro Animation properties
        "intro_animation_type" => props.intro_animation_type = value.to_string(),
        "intro_animation_duration" => props.intro_animation_duration = value.to_string(),
        "intro_animation_delay" => props.intro_animation_delay = value.to_string(),
        "intro_animation_easing" => props.intro_animation_easing = value.to_string(),
        "intro_animation_trigger" => props.intro_animation_trigger = value.to_string(),
        "intro_animation_offset" => props.intro_animation_offset = value.to_string(),
        "intro_animation_repeat" => props.intro_animation_repeat = if is_checkbox { value == "true" } else { value.parse().unwrap_or(false) },
        
        // Legacy Animation properties (for backward compatibility)
        "animation_type" => props.animation_type = value.to_string(),
        "animation_duration" => props.animation_duration = value.to_string(),
        "animation_delay" => props.animation_delay = value.to_string(),
        
        // SEO properties
        "seo_title" => props.seo_title = value.to_string(),
        "seo_description" => props.seo_description = value.to_string(),
        "aria_label" => props.aria_label = value.to_string(),
        "aria_description" => props.aria_description = value.to_string(),
        
        _ => {
            // Log unknown property for debugging
            web_sys::console::log_1(&format!("Unknown component property: {}", name).into());
        }
    }
}


