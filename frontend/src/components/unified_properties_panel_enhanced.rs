use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, InputEvent};
use crate::components::page_builder::drag_drop_builder::{PageComponent, ComponentProperties};
use crate::services::navigation_service::{ComponentTemplate, update_component_template};

#[derive(Properties, PartialEq)]
pub struct UnifiedPropertiesPanelProps {
    pub panel_type: PanelType,
    pub component: Option<PageComponent>,
    pub template_data: Option<serde_json::Value>,
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

    // Handle property changes for templates
    let on_property_change = {
        let working_template_data = working_template_data.clone();
        let props_on_template_updated = props.on_template_updated.clone();
        
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                    let name = input.name();
                    let value = input.value();
                    
                    let mut data = (*working_template_data).clone();
                    data[&name] = serde_json::Value::String(value);
                    working_template_data.set(data.clone());
                    
                    // Apply real-time preview for template properties
                    if let Some(callback) = &props_on_template_updated {
                        let template = ComponentTemplate {
                            id: 1, // This would be the actual template ID
                            name: "Header".to_string(), // This would be the actual template name
                            template_type: "header".to_string(),
                            template_data: data,
                            created_at: chrono::Utc::now().naive_utc(),
                            updated_at: chrono::Utc::now().naive_utc(),
                        };
                        callback.emit(template);
                    }
                } else if let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
                    let name = select.name();
                    let value = select.value();
                    
                    let mut data = (*working_template_data).clone();
                    data[&name] = serde_json::Value::String(value);
                    working_template_data.set(data.clone());
                    
                    // Apply real-time preview for template properties
                    if let Some(callback) = &props_on_template_updated {
                        let template = ComponentTemplate {
                            id: 1, // This would be the actual template ID
                            name: "Header".to_string(), // This would be the actual template name
                            template_type: "header".to_string(),
                            template_data: data,
                            created_at: chrono::Utc::now().naive_utc(),
                            updated_at: chrono::Utc::now().naive_utc(),
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
        
        Callback::from(move |_| {
            match panel_type {
                PanelType::PageComponent => {
                    if let (Some(mut component), Some(callback)) = (props_component.clone(), &props_on_component_updated) {
                        component.content = (*working_content).clone();
                        callback.emit(component);
                    }
                }
                PanelType::HeaderTemplate | PanelType::FooterTemplate | PanelType::ContainerTemplate => {
                    if let Some(callback) = &props_on_template_updated {
                        let template = ComponentTemplate {
                            id: 1, // This would be the actual template ID
                            name: "Template".to_string(), // This would be the actual template name
                            template_type: match panel_type {
                                PanelType::HeaderTemplate => "header",
                                PanelType::FooterTemplate => "footer",
                                PanelType::ContainerTemplate => "container",
                                _ => "unknown",
                            }.to_string(),
                            template_data: (*working_template_data).clone(),
                            created_at: chrono::Utc::now().naive_utc(),
                            updated_at: chrono::Utc::now().naive_utc(),
                        };
                        callback.emit(template);
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
                    <button onclick={on_save} style="
                        background: #007bff;
                        color: white;
                        border: none;
                        padding: 8px 16px;
                        border-radius: 4px;
                        cursor: pointer;
                        font-size: 14px;
                        width: 100%;
                    ">{"Save Changes"}</button>
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
                        ("arrow", "Arrow"),
                        ("tilt", "Tilt")
                    ], on_change.clone())}
                    
                    {if shape_mask_upper != "none" {
                        html! {
                            <div style="margin-top: 8px;">
                                {render_range_field("Scale (%)", "shape_mask_upper_scale", shape_mask_upper_scale, "50", "200", on_change.clone())}
                                
                                {if shape_mask_upper == "wave" {
                                    html! {
                                        <>
                                            {render_range_field("Frequency", "shape_mask_upper_frequency", 
                                                &template_data.get("shape_mask_upper_frequency").and_then(|v| v.as_str()).unwrap_or("2"), 
                                                "1", "8", on_change.clone())}
                                            {render_range_field("Amplitude (%)", "shape_mask_upper_amplitude", 
                                                &template_data.get("shape_mask_upper_amplitude").and_then(|v| v.as_str()).unwrap_or("50"), 
                                                "10", "100", on_change.clone())}
                                        </>
                                    }
                                } else if shape_mask_upper == "curve" {
                                    html! {
                                        {render_range_field("Curve Depth (%)", "shape_mask_upper_curve_depth", 
                                            &template_data.get("shape_mask_upper_curve_depth").and_then(|v| v.as_str()).unwrap_or("50"), 
                                            "10", "100", on_change.clone())}
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
                            <div style="margin-top: 8px;">
                                {render_range_field("Scale (%)", "shape_mask_lower_scale", shape_mask_lower_scale, "50", "200", on_change.clone())}
                                
                                {if shape_mask_lower == "wave" {
                                    html! {
                                        <>
                                            {render_range_field("Frequency", "shape_mask_lower_frequency", 
                                                &template_data.get("shape_mask_lower_frequency").and_then(|v| v.as_str()).unwrap_or("2"), 
                                                "1", "8", on_change.clone())}
                                            {render_range_field("Amplitude (%)", "shape_mask_lower_amplitude", 
                                                &template_data.get("shape_mask_lower_amplitude").and_then(|v| v.as_str()).unwrap_or("50"), 
                                                "10", "100", on_change.clone())}
                                        </>
                                    }
                                } else if shape_mask_lower == "curve" {
                                    html! {
                                        {render_range_field("Curve Depth (%)", "shape_mask_lower_curve_depth", 
                                            &template_data.get("shape_mask_lower_curve_depth").and_then(|v| v.as_str()).unwrap_or("50"), 
                                            "10", "100", on_change.clone())}
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
                    html! {
                        <option value={val} selected={val == value}>{text}</option>
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
