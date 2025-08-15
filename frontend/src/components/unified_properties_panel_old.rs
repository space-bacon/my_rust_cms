use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, InputEvent, Event};
use crate::components::page_builder::drag_drop_builder::{PageComponent, ComponentProperties};
use crate::services::navigation_service::{ComponentTemplate, update_component_template};
use crate::components::enhanced_live_edit_system::apply_template_style_preview;
use serde_json::json;

#[derive(Properties, PartialEq, Clone)]
pub struct UnifiedPropertiesPanelProps {
    pub component: Option<PageComponent>,
    pub component_template: Option<ComponentTemplate>,
    pub panel_type: PanelType,
    pub on_component_updated: Option<Callback<PageComponent>>,
    pub on_template_updated: Option<Callback<ComponentTemplate>>,
    pub on_close: Callback<()>,
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
    let working_properties = use_state(|| ComponentProperties::default());
    let working_template_data = use_state(|| serde_json::Value::Object(serde_json::Map::new()));
    let working_content = use_state(|| String::new());

    // Initialize working values when component/template changes
    {
        let working_properties = working_properties.clone();
        let working_template_data = working_template_data.clone();
        let working_content = working_content.clone();
        let component = props.component.clone();
        let template = props.component_template.clone();
        
        use_effect_with_deps(move |(comp, tmpl)| {
            if let Some(component) = comp {
                working_properties.set(component.properties.clone());
                working_content.set(component.content.clone());
            } else if let Some(template) = tmpl {
                working_template_data.set(template.template_data.clone());
            }
            || ()
        }, (component, template));
    }

    let on_property_change = {
        let working_properties = working_properties.clone();
        let working_template_data = working_template_data.clone();
        let working_content = working_content.clone();
        let panel_type = props.panel_type.clone();
        
        Callback::from(move |event: InputEvent| {
            if let Some(target) = event.target() {
                let property_name = if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                    let name = input.name();
                    let value = if input.type_() == "checkbox" {
                        if input.checked() { "true" } else { "false" }.to_string()
                    } else {
                        input.value()
                    };
                    (name, value)
                } else if let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
                    (select.name(), select.value())
                } else if let Ok(textarea) = target.clone().dyn_into::<HtmlTextAreaElement>() {
                    (textarea.name(), textarea.value())
                } else {
                    return;
                };

                let (property_name, value) = property_name;

                match panel_type {
                    PanelType::PageComponent => {
                        if property_name == "component_content" {
                            working_content.set(value);
                        } else {
                            let mut props = (*working_properties).clone();
                            update_component_property(&mut props, &property_name, &value);
                            working_properties.set(props);
                        }
                    }
                    PanelType::HeaderTemplate | PanelType::FooterTemplate | PanelType::ContainerTemplate => {
                        let mut data = (*working_template_data).clone();
                        if let Some(obj) = data.as_object_mut() {
                            // Handle height values - ensure they have px units
                            let processed_value = if property_name == "height" {
                                if value.ends_with("px") {
                                    value
                                } else {
                                    format!("{}px", value)
                                }
                            } else {
                                value
                            };
                            obj.insert(property_name, json!(processed_value));
                        }
                        
                        // Apply real-time preview
                        let component_type = match panel_type {
                            PanelType::HeaderTemplate => "header",
                            PanelType::FooterTemplate => "footer",
                            PanelType::ContainerTemplate => "container",
                            _ => "",
                        };
                        
                        if !component_type.is_empty() {
                            apply_template_style_preview(component_type, &data);
                        }
                        
                        working_template_data.set(data);
                    }
                }
            }
        })
    };

    let on_save = {
        let working_properties = working_properties.clone();
        let working_template_data = working_template_data.clone();
        let working_content = working_content.clone();
        let on_component_updated = props.on_component_updated.clone();
        let on_template_updated = props.on_template_updated.clone();
        let component = props.component.clone();
        let template = props.component_template.clone();
        let panel_type = props.panel_type.clone();
        
        Callback::from(move |_| {
            match panel_type {
                PanelType::PageComponent => {
                    if let (Some(mut comp), Some(callback)) = (component.clone(), on_component_updated.clone()) {
                        comp.properties = (*working_properties).clone();
                        comp.content = (*working_content).clone();
                        callback.emit(comp);
                    }
                }
                PanelType::HeaderTemplate | PanelType::FooterTemplate | PanelType::ContainerTemplate => {
                    if let (Some(mut template_clone), Some(callback)) = (template.clone(), on_template_updated.clone()) {
                        template_clone.template_data = (*working_template_data).clone();
                        
                        // Save to backend
                        let template_id = template_clone.id;
                        let template_for_update = template_clone.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            match update_component_template(template_id, &template_for_update).await {
                                Ok(_) => {
                                    web_sys::console::log_1(&"Template updated successfully".into());
                                }
                                Err(e) => {
                                    web_sys::console::log_1(&format!("Failed to update template: {:?}", e).into());
                                }
                            }
                        });
                        
                        callback.emit(template_clone);
                    }
                }
            }
        })
    };

    html! {
        <div class="properties-panel" style="position: fixed; top: 0; right: 0; width: 300px; height: 100vh; background: white; border-left: 1px solid #ddd; padding: 16px; overflow-y: auto; z-index: 1000; box-shadow: -2px 0 8px rgba(0,0,0,0.1);">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; padding-bottom: 12px; border-bottom: 1px solid #eee;">
                <h3 style="margin: 0; font-size: 16px; color: #333; font-weight: 600;">
                    {match props.panel_type {
                        PanelType::PageComponent => "Component Properties",
                        PanelType::HeaderTemplate => "Header Properties",
                        PanelType::FooterTemplate => "Footer Properties",
                        PanelType::ContainerTemplate => "Container Properties",
                    }}
                </h3>
                <button 
                    onclick={{
                        let on_close = props.on_close.clone();
                        Callback::from(move |_| on_close.emit(()))
                    }}
                    style="background: none; border: none; font-size: 18px; cursor: pointer; color: #666; padding: 4px;"
                    title="Close"
                >
                    {"×"}
                </button>
            </div>

            {match props.panel_type {
                PanelType::PageComponent => {
                    render_component_properties(&working_properties, &props.component, &working_content, on_property_change.clone())
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

            <div style="margin-top: 20px; padding-top: 16px; border-top: 1px solid #eee;">
                <button 
                    onclick={on_save}
                    style="width: 100%; padding: 10px; background: #007cba; color: white; border: none; border-radius: 4px; cursor: pointer; font-weight: 600; font-size: 14px;"
                >
                    {"Save Changes"}
                </button>
            </div>
        </div>
    }
}

fn render_component_properties(
    working_properties: &UseStateHandle<ComponentProperties>, 
    _component: &Option<PageComponent>, 
    working_content: &UseStateHandle<String>, 
    on_change: Callback<InputEvent>
) -> Html {
    html! {
        <div>
            <div class="property-group">
                <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{"Content"}</label>
                <textarea
                    name="component_content"
                    value={(**working_content).clone()}
                    oninput={on_change.clone()}
                    style="width: 100%; min-height: 100px; padding: 8px; border: 1px solid #ddd; border-radius: 4px; font-family: monospace; font-size: 12px; resize: vertical;"
                    placeholder="Enter component content..."
                />
            </div>
        </div>
    }
}

fn render_header_properties(template_data: &UseStateHandle<serde_json::Value>, on_change: Callback<InputEvent>) -> Html {
    // Basic properties
    let text_color = template_data.get("text_color").and_then(|v| v.as_str()).unwrap_or("#ffffff");
    let logo_url = template_data.get("logo_url").and_then(|v| v.as_str()).unwrap_or("");
    let height = template_data.get("height").and_then(|v| v.as_str()).unwrap_or("110px");
    
    // Strip px from height for display in range slider
    let height_value = height.strip_suffix("px").unwrap_or(height);
    
    // Background properties
    let bg_type = template_data.get("bg_type").and_then(|v| v.as_str()).unwrap_or("color");
    let bg_color = template_data.get("bg_color").and_then(|v| v.as_str()).unwrap_or("#000000");
    let bg_image = template_data.get("bg_image").and_then(|v| v.as_str()).unwrap_or("");
    let bg_video = template_data.get("bg_video").and_then(|v| v.as_str()).unwrap_or("");
    let bg_gradient_start = template_data.get("bg_gradient_start").and_then(|v| v.as_str()).unwrap_or("#ffffff");
    let bg_gradient_end = template_data.get("bg_gradient_end").and_then(|v| v.as_str()).unwrap_or("#f0f0f0");
    let bg_gradient_direction = template_data.get("bg_gradient_direction").and_then(|v| v.as_str()).unwrap_or("to-right");
    
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
                {render_input_field("Text Color", "text_color", text_color, on_change.clone())}
                {render_input_field("Logo URL", "logo_url", logo_url, on_change.clone())}
                {render_range_field("Height (px)", "height", height_value, "40", "2600", on_change.clone())}
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
                    "color" => html! {
                        {render_color_field("Background Color", "bg_color", bg_color, on_change.clone())}
                    },
                    "image" => html! {
                        {render_input_field("Background Image URL", "bg_image", bg_image, on_change.clone())}
                    },
                    "gradient" => html! {
                        <>
                            {render_color_field("Start Color", "bg_gradient_start", bg_gradient_start, on_change.clone())}
                            {render_color_field("End Color", "bg_gradient_end", bg_gradient_end, on_change.clone())}
                            {render_select_field("Direction", "bg_gradient_direction", bg_gradient_direction, vec![
                                ("to-right", "Left to Right"),
                                ("to-left", "Right to Left"),
                                ("to-bottom", "Top to Bottom"),
                                ("to-top", "Bottom to Top")
                            ], on_change.clone())}
                        </>
                    },
                    "video" => html! {
                        {render_input_field("Background Video URL", "bg_video", bg_video, on_change.clone())}
                    },
                    _ => html! {}
                }}
            </div>
            
            // Shape Mask Properties
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Shape Mask"}</h4>
                
                // Upper Shape Mask
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{"Upper Shape"}</label>
                    {render_select_field("", "shape_mask_upper", shape_mask_upper, vec![
                        ("none", "None"),
                        ("wave", "Wave"),
                        ("curve", "Curve"),
                        ("triangle", "Triangle"),
                        ("zigzag", "Zigzag"),
                        ("arrow", "Arrow"),
                        ("tilt", "Tilt")
                    ], on_change.clone())}
                    {if shape_mask_upper != "none" {
                        html! {
                            {render_range_field("Upper Scale (%)", "shape_mask_upper_scale", shape_mask_upper_scale, "50", "200", on_change.clone())}
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
                        ("arrow", "Arrow"),
                        ("tilt", "Tilt")
                    ], on_change.clone())}
                    {if shape_mask_lower != "none" {
                        html! {
                            {render_range_field("Lower Scale (%)", "shape_mask_lower_scale", shape_mask_lower_scale, "50", "200", on_change.clone())}
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
    // Basic properties
    let text_color = template_data.get("text_color").and_then(|v| v.as_str()).unwrap_or("#ffffff");
    let copyright_text = template_data.get("copyright_text").and_then(|v| v.as_str()).unwrap_or("© 2024 My Site");
    let height = template_data.get("height").and_then(|v| v.as_str()).unwrap_or("80px");
    
    // Strip px from height for display in range slider
    let height_value = height.strip_suffix("px").unwrap_or(height);
    
    // Background properties
    let bg_type = template_data.get("bg_type").and_then(|v| v.as_str()).unwrap_or("color");
    let bg_color = template_data.get("bg_color").and_then(|v| v.as_str()).unwrap_or("#333333");
    let bg_image = template_data.get("bg_image").and_then(|v| v.as_str()).unwrap_or("");
    let bg_video = template_data.get("bg_video").and_then(|v| v.as_str()).unwrap_or("");
    let bg_gradient_start = template_data.get("bg_gradient_start").and_then(|v| v.as_str()).unwrap_or("#333333");
    let bg_gradient_end = template_data.get("bg_gradient_end").and_then(|v| v.as_str()).unwrap_or("#000000");
    let bg_gradient_direction = template_data.get("bg_gradient_direction").and_then(|v| v.as_str()).unwrap_or("to-right");
    
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
                {render_input_field("Text Color", "text_color", text_color, on_change.clone())}
                {render_input_field("Copyright Text", "copyright_text", copyright_text, on_change.clone())}
                {render_range_field("Height (px)", "height", height_value, "60", "2600", on_change.clone())}
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
                    "color" => html! {
                        {render_color_field("Background Color", "bg_color", bg_color, on_change.clone())}
                    },
                    "image" => html! {
                        {render_input_field("Background Image URL", "bg_image", bg_image, on_change.clone())}
                    },
                    "gradient" => html! {
                        <>
                            {render_color_field("Start Color", "bg_gradient_start", bg_gradient_start, on_change.clone())}
                            {render_color_field("End Color", "bg_gradient_end", bg_gradient_end, on_change.clone())}
                            {render_select_field("Direction", "bg_gradient_direction", bg_gradient_direction, vec![
                                ("to-right", "Left to Right"),
                                ("to-left", "Right to Left"),
                                ("to-bottom", "Top to Bottom"),
                                ("to-top", "Bottom to Top")
                            ], on_change.clone())}
                        </>
                    },
                    "video" => html! {
                        {render_input_field("Background Video URL", "bg_video", bg_video, on_change.clone())}
                    },
                    _ => html! {}
                }}
            </div>
            
            // Shape Mask Properties
            <div class="property-section" style="margin-top: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Shape Mask"}</h4>
                
                // Upper Shape Mask
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{"Upper Shape"}</label>
                    {render_select_field("", "shape_mask_upper", shape_mask_upper, vec![
                        ("none", "None"),
                        ("wave", "Wave"),
                        ("curve", "Curve"),
                        ("triangle", "Triangle"),
                        ("zigzag", "Zigzag"),
                        ("arrow", "Arrow"),
                        ("tilt", "Tilt")
                    ], on_change.clone())}
                    {if shape_mask_upper != "none" {
                        html! {
                            {render_range_field("Upper Scale (%)", "shape_mask_upper_scale", shape_mask_upper_scale, "50", "200", on_change.clone())}
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
                        ("arrow", "Arrow"),
                        ("tilt", "Tilt")
                    ], on_change.clone())}
                    {if shape_mask_lower != "none" {
                        html! {
                            {render_range_field("Lower Scale (%)", "shape_mask_lower_scale", shape_mask_lower_scale, "50", "200", on_change.clone())}
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        </>
    }
}

fn render_container_properties(template_data: &UseStateHandle<serde_json::Value>, on_change: Callback<InputEvent>) -> Html {
    let bg_type = template_data.get("bg_type").and_then(|v| v.as_str()).unwrap_or("none");
    let bg_color = template_data.get("bg_color").and_then(|v| v.as_str()).unwrap_or("#ffffff");
    let bg_image = template_data.get("bg_image").and_then(|v| v.as_str()).unwrap_or("");
    let bg_video = template_data.get("bg_video").and_then(|v| v.as_str()).unwrap_or("");
    let bg_gradient_start = template_data.get("bg_gradient_start").and_then(|v| v.as_str()).unwrap_or("#ffffff");
    let bg_gradient_end = template_data.get("bg_gradient_end").and_then(|v| v.as_str()).unwrap_or("#f0f0f0");
    let bg_gradient_direction = template_data.get("bg_gradient_direction").and_then(|v| v.as_str()).unwrap_or("to-right");

    html! {
        <>
            // Background Properties
            <div class="property-section">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Background"}</h4>
                {render_select_field("Background Type", "bg_type", bg_type, vec![
                    ("none", "None"),
                    ("color", "Color"),
                    ("image", "Image"),
                    ("gradient", "Gradient"),
                    ("video", "Video")
                ], on_change.clone())}
                
                {match bg_type {
                    "color" => html! {
                        {render_color_field("Background Color", "bg_color", bg_color, on_change.clone())}
                    },
                    "image" => html! {
                        {render_input_field("Background Image URL", "bg_image", bg_image, on_change.clone())}
                    },
                    "gradient" => html! {
                        <>
                            {render_color_field("Start Color", "bg_gradient_start", bg_gradient_start, on_change.clone())}
                            {render_color_field("End Color", "bg_gradient_end", bg_gradient_end, on_change.clone())}
                            {render_select_field("Direction", "bg_gradient_direction", bg_gradient_direction, vec![
                                ("to-right", "Left to Right"),
                                ("to-left", "Right to Left"),
                                ("to-bottom", "Top to Bottom"),
                                ("to-top", "Bottom to Top")
                            ], on_change.clone())}
                        </>
                    },
                    "video" => html! {
                        {render_input_field("Background Video URL", "bg_video", bg_video, on_change.clone())}
                    },
                    _ => html! {}
                }}
            </div>
        </>
    }
}

fn update_component_property(properties: &mut ComponentProperties, property_name: &str, value: &str) {
    match property_name {
        "hero_title" => properties.hero_title = value.to_string(),
        "hero_subtitle" => properties.hero_subtitle = value.to_string(),
        "hero_primary_button_text" => properties.hero_primary_button_text = value.to_string(),
        "hero_primary_button_url" => properties.hero_primary_button_url = value.to_string(),
        "hero_background_image" => properties.hero_background_image = value.to_string(),
        "card_title" => properties.card_title = value.to_string(),
        "card_description" => properties.card_description = value.to_string(),
        "card_image" => properties.card_image = value.to_string(),
        "card_button_url" => properties.card_button_url = value.to_string(),
        "image_url" => properties.image_url = value.to_string(),
        "image_alt" => properties.image_alt = value.to_string(),
        "video_url" => properties.video_url = value.to_string(),
        "button_text" => properties.button_text = value.to_string(),
        "button_url" => properties.button_url = value.to_string(),
        "form_action" => properties.form_action = value.to_string(),
        "form_method" => properties.form_method = value.to_string(),
        "divider_style" => properties.divider_style = value.to_string(),
        _ => {}
    }
}

// Helper functions for rendering form fields
fn render_input_field(label: &str, name: &str, value: &str, on_change: Callback<InputEvent>) -> Html {
    let name = name.to_string();
    let value = value.to_string();
    
    html! {
        <div class="property-group">
            <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{label}</label>
            <input
                type="text"
                name={name}
                value={value}
                oninput={on_change}
                style="width: 100%; padding: 6px 8px; border: 1px solid #ddd; border-radius: 4px; font-size: 12px;"
            />
        </div>
    }
}

fn render_textarea_field(label: &str, name: &str, value: &str, on_change: Callback<InputEvent>) -> Html {
    let name = name.to_string();
    let value = value.to_string();
    
    html! {
        <div class="property-group">
            <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{label}</label>
            <textarea
                name={name}
                value={value}
                oninput={on_change}
                style="width: 100%; min-height: 60px; padding: 6px 8px; border: 1px solid #ddd; border-radius: 4px; font-size: 12px; resize: vertical;"
            />
        </div>
    }
}

fn render_select_field(label: &str, name: &str, value: &str, options: Vec<(&str, &str)>, on_change: Callback<InputEvent>) -> Html {
    let name = name.to_string();
    
    html! {
        <div class="property-group">
            {if !label.is_empty() {
                html! { <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{label}</label> }
            } else {
                html! {}
            }}
            <select
                name={name}
                value={value.to_string()}
                oninput={on_change}
                style="width: 100%; padding: 6px 8px; border: 1px solid #ddd; border-radius: 4px; font-size: 12px; background: white;"
            >
                {for options.iter().map(|(val, text)| {
                    html! {
                        <option value={val.to_string()} selected={*val == value}>
                            {text}
                        </option>
                    }
                })}
            </select>
        </div>
    }
}

fn render_checkbox_field(label: &str, name: &str, checked: bool, on_change: Callback<InputEvent>) -> Html {
    let name = name.to_string();
    
    html! {
        <div class="property-group">
            <label style="display: flex; align-items: center; gap: 8px; font-weight: 600; font-size: 12px; color: #555; cursor: pointer;">
                <input
                    type="checkbox"
                    name={name}
                    checked={checked}
                    oninput={on_change}
                    style="margin: 0;"
                />
                {label}
            </label>
        </div>
    }
}

fn render_range_field(label: &str, name: &str, value: &str, min: &str, max: &str, on_change: Callback<InputEvent>) -> Html {
    let name = name.to_string();
    let value = value.to_string();
    let min = min.to_string();
    let max = max.to_string();

    html! {
        <div class="property-group">
            <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{label}</label>
            <div style="display: flex; align-items: center; gap: 8px;">
                <input
                    type="range"
                    name={name.clone()}
                    value={value.clone()}
                    min={min}
                    max={max}
                    oninput={on_change.clone()}
                    style="flex: 1;"
                />
                <span style="font-size: 11px; color: #666; min-width: 40px; text-align: right;">{value}</span>
            </div>
        </div>
    }
}

fn render_color_field(label: &str, name: &str, value: &str, on_change: Callback<InputEvent>) -> Html {
    let name = name.to_string();
    let value = value.to_string();
    
    html! {
        <div class="property-group">
            <label style="display: block; margin-bottom: 4px; font-weight: 600; font-size: 12px; color: #555;">{label}</label>
            <div style="display: flex; align-items: center; gap: 8px;">
                <input
                    type="color"
                    name={name.clone()}
                    value={value.clone()}
                    oninput={on_change.clone()}
                    style="width: 40px; height: 32px; border: 1px solid #ddd; border-radius: 4px; cursor: pointer;"
                />
                <input
                    type="text"
                    name={name}
                    value={value}
                    oninput={on_change}
                    style="flex: 1; padding: 6px 8px; border: 1px solid #ddd; border-radius: 4px; font-size: 12px; font-family: monospace;"
                />
            </div>
        </div>
    }
}
