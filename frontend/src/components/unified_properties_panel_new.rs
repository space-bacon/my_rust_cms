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

    let has_unsaved_changes = use_state(|| false);
    let saving = use_state(|| false);

    // Update working template data when props change
    {
        let working_template_data = working_template_data.clone();
        let has_unsaved_changes_state = has_unsaved_changes.clone();
        use_effect_with_deps(move |deps| {
            let (template_data_opt, currently_has_unsaved) = deps;
            
            if !currently_has_unsaved && template_data_opt.is_some() {
                if let Some(data) = template_data_opt.clone() {
                    working_template_data.set(data);
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
                        // Handle template property updates
                        let mut data = (*working_template_data).clone();
                        data[&name] = serde_json::Value::String(value.clone());
                        working_template_data.set(data.clone());
                        
                        // Apply real-time preview for template properties
                        let component_type = match panel_type {
                            PanelType::HeaderTemplate => "header",
                            PanelType::FooterTemplate => "footer",
                            PanelType::ContainerTemplate => "container",
                            _ => "",
                        };
                        
                        if !component_type.is_empty() {
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
                        
                        // Save to database
                        let template_for_api = template.clone();
                        let has_unsaved_changes_for_api = has_unsaved_changes.clone();
                        let saving_for_api = saving.clone();
                        let callback_for_api = callback.clone();
                        
                        wasm_bindgen_futures::spawn_local(async move {
                            match crate::services::navigation_service::update_component_template(template_for_api.id, &template_for_api).await {
                                Ok(saved_template) => {
                                    callback_for_api.emit(saved_template);
                                    has_unsaved_changes_for_api.set(false);
                                }
                                Err(e) => {
                                    web_sys::console::log_1(&format!("Failed to save template: {:?}", e).into());
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
    working_properties: &UseStateHandle<ComponentProperties>, 
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
            {render_component_specific_properties(component_properties, on_change.clone())}
            
            // Common styling properties
            {render_common_styling_properties(component_properties, on_change.clone())}
        </div>
    }
}

// Include all the header properties rendering from the original file
fn render_header_properties(template_data: &UseStateHandle<serde_json::Value>, on_change: Callback<InputEvent>) -> Html {
    // This would include all the header property rendering logic from the original file
    html! { <div>{"Header properties (implementation from original file)"}</div> }
}

fn render_footer_properties(template_data: &UseStateHandle<serde_json::Value>, on_change: Callback<InputEvent>) -> Html {
    render_header_properties(template_data, on_change)
}

fn render_container_properties(template_data: &UseStateHandle<serde_json::Value>, on_change: Callback<InputEvent>) -> Html {
    html! { <div>{"Container properties"}</div> }
}

// Component-specific property rendering functions
fn render_component_specific_properties(properties: &ComponentProperties, on_change: Callback<InputEvent>) -> Html {
    html! {
        <div class="component-specific-properties">
            // Image Properties
            {if !properties.image_url.is_empty() || !properties.image_alt.is_empty() {
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
            
            // Button Properties
            {if !properties.button_text.is_empty() || !properties.button_url.is_empty() {
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
            
            // Video Properties
            {if !properties.video_url.is_empty() {
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
            
            // Hero Properties
            {if !properties.hero_title.is_empty() || !properties.hero_subtitle.is_empty() {
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
            // Animation Properties
            <div class="property-section" style="margin-bottom: 16px;">
                <h4 style="margin: 0 0 8px 0; font-size: 14px; color: #555; font-weight: 600;">{"Animation & Effects"}</h4>
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
        
        // Animation properties
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
