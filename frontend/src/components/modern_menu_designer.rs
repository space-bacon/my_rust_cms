use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlSelectElement, MouseEvent, FocusEvent, KeyboardEvent, InputEvent};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MenuStyle {
    pub id: String,
    pub name: String,
    pub properties: MenuProperties,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MenuProperties {
    // Core Colors
    pub background: String,
    pub text_color: String,
    pub hover_color: String,
    pub active_color: String,
    pub hover_background: String,
    pub active_background: String,
    
    // Layout & Spacing
    pub layout: String, // "horizontal", "vertical"
    pub alignment: String, // "left", "center", "right"
    pub item_spacing: String, // CSS value like "16px", "1rem"
    pub padding: String, // CSS value like "8px 16px"
    
    // Visual Style
    pub border_radius: String, // CSS value like "4px", "8px"
    pub border: String, // CSS border value like "1px solid #ccc"
    pub shadow: String, // CSS box-shadow value
    
    // Typography
    pub font_size: String, // CSS value like "14px", "1rem"
    pub font_weight: String, // CSS value like "400", "500", "600"
    pub text_transform: String, // "none", "uppercase", "lowercase", "capitalize"
    
    // Hover Effects & Animations
    pub hover_transition: String, // CSS transition value
    pub hover_transform: String, // CSS transform value
    pub hover_animation_type: String, // "none", "lift", "scale", "glow", "slide", "bounce", "pulse"
    pub hover_animation_duration: String, // CSS duration like "0.3s", "300ms"
    pub hover_animation_easing: String, // CSS easing like "ease", "cubic-bezier(0.4, 0, 0.2, 1)"
    pub hover_scale: String, // Scale factor like "1.05", "1.1"
    pub hover_lift_distance: String, // Transform Y distance like "2px", "4px"
    pub hover_glow_color: String, // Color for glow effect
    pub hover_glow_intensity: String, // Box-shadow blur radius like "8px", "15px"
    pub hover_border_color: String, // Border color on hover
    pub hover_border_width: String, // Border width on hover
    
    // Simple Animation Effects
    pub enable_underline_animation: bool, // Animated underline on hover
    pub underline_color: String, // Color of animated underline
    pub underline_thickness: String, // Thickness of underline
    
    // Mobile Responsive
    pub mobile_breakpoint: String, // CSS media query breakpoint
    pub mobile_layout: String, // "hamburger", "stack", "scroll"
}

impl Default for MenuProperties {
    fn default() -> Self {
        Self {
            // Conservative minimal colors
            background: "transparent".to_string(),
            text_color: "#374151".to_string(),
            hover_color: "#1f2937".to_string(),
            active_color: "#111827".to_string(),
            hover_background: "rgba(31, 41, 55, 0.05)".to_string(),
            active_background: "rgba(17, 24, 39, 0.1)".to_string(),
            
            // Clean layout
            layout: "horizontal".to_string(),
            alignment: "left".to_string(),
            item_spacing: "24px".to_string(),
            padding: "8px 16px".to_string(),
            
            // Minimal styling
            border_radius: "4px".to_string(),
            border: "none".to_string(),
            shadow: "none".to_string(),
            
            // Clean typography
            font_size: "15px".to_string(),
            font_weight: "500".to_string(),
            text_transform: "none".to_string(),
            
            // Subtle effects & animations
            hover_transition: "all 0.2s ease".to_string(),
            hover_transform: "none".to_string(),
            hover_animation_type: "none".to_string(),
            hover_animation_duration: "0.3s".to_string(),
            hover_animation_easing: "cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
            hover_scale: "1.0".to_string(),
            hover_lift_distance: "0px".to_string(),
            hover_glow_color: "rgba(59, 130, 246, 0.3)".to_string(),
            hover_glow_intensity: "8px".to_string(),
            hover_border_color: "transparent".to_string(),
            hover_border_width: "1px".to_string(),
            
            // Simple animation defaults
            enable_underline_animation: false,
            underline_color: "#3b82f6".to_string(),
            underline_thickness: "2px".to_string(),
            
            // Responsive
            mobile_breakpoint: "768px".to_string(),
            mobile_layout: "hamburger".to_string(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ModernMenuDesignerProps {
    pub menu_area: String,
    pub current_style: Option<MenuStyle>,
    pub on_style_change: Callback<MenuStyle>,
    pub on_save_and_close: Option<Callback<()>>,
    pub on_close: Callback<()>,
}

#[function_component(ModernMenuDesigner)]
pub fn modern_menu_designer(props: &ModernMenuDesignerProps) -> Html {
    let current_style = use_state(|| {
        props.current_style.clone().unwrap_or_else(|| MenuStyle {
            id: format!("{}_style", props.menu_area),
            name: format!("{} Menu Style", props.menu_area),
            properties: MenuProperties::default(),
        })
    });

    // Sync local state with props changes
    {
        let current_style = current_style.clone();
        let props_style = props.current_style.clone();
        use_effect_with_deps(move |_| {
            if let Some(new_style) = props_style {
                current_style.set(new_style);
            }
            || {}
        }, props.current_style.clone());
    }

    let active_tab = use_state(|| "colors".to_string());

    // Update property callback
    let update_property = {
        let current_style = current_style.clone();
        let on_style_change = props.on_style_change.clone();
        
        Callback::from(move |updates: Vec<(String, String)>| {
            let mut style = (*current_style).clone();
            
            web_sys::console::log_1(&format!("🎨 UPDATE_PROPERTY: Received updates: {:?}", updates).into());
            
            for (field, value) in updates {
                match field.as_str() {
                    "background" => style.properties.background = value,
                    "text_color" => style.properties.text_color = value,
                    "hover_color" => style.properties.hover_color = value,
                    "active_color" => style.properties.active_color = value,
                    "hover_background" => style.properties.hover_background = value,
                    "active_background" => style.properties.active_background = value,
                    "layout" => style.properties.layout = value,
                    "alignment" => style.properties.alignment = value,
                    "item_spacing" => style.properties.item_spacing = value,
                    "padding" => style.properties.padding = value,
                    "border_radius" => style.properties.border_radius = value,
                    "border" => style.properties.border = value,
                    "shadow" => style.properties.shadow = value,
                    "font_size" => style.properties.font_size = value,
                    "font_weight" => style.properties.font_weight = value,
                    "text_transform" => style.properties.text_transform = value,
                    "hover_transition" => style.properties.hover_transition = value,
                    "hover_transform" => style.properties.hover_transform = value,
                    "hover_animation_type" => style.properties.hover_animation_type = value,
                    "hover_animation_duration" => style.properties.hover_animation_duration = value,
                    "hover_animation_easing" => style.properties.hover_animation_easing = value,
                    "hover_scale" => style.properties.hover_scale = value,
                    "hover_lift_distance" => style.properties.hover_lift_distance = value,
                    "hover_glow_color" => style.properties.hover_glow_color = value,
                    "hover_glow_intensity" => style.properties.hover_glow_intensity = value,
                    "hover_border_color" => style.properties.hover_border_color = value,
                    "hover_border_width" => style.properties.hover_border_width = value,
                    "enable_underline_animation" => style.properties.enable_underline_animation = value == "true",
                    "underline_color" => style.properties.underline_color = value,
                    "underline_thickness" => style.properties.underline_thickness = value,
                    "mobile_breakpoint" => style.properties.mobile_breakpoint = value,
                    "mobile_layout" => style.properties.mobile_layout = value,
                    _ => {}
                }
            }
            
            current_style.set(style.clone());
            on_style_change.emit(style);
        })
    };

    // Save callback - always save current style and close if save_and_close is provided
    let save_callback = {
        let on_save_and_close = props.on_save_and_close.clone();
        let on_style_change = props.on_style_change.clone();
        let current_style = current_style.clone();
        let on_close = props.on_close.clone();
        
        Callback::from(move |_| {
            let style = (*current_style).clone();
            
            if on_save_and_close.is_some() {
                // Save the current style and close the modal
                on_style_change.emit(style);
                // Small delay to ensure save completes before closing
                let on_close = on_close.clone();
                gloo_timers::callback::Timeout::new(100, move || {
                    on_close.emit(());
                }).forget();
            } else {
                // Just save without closing (fallback behavior)
                on_style_change.emit(style);
            }
        })
    };

    // Modal backdrop click handler - only close when clicking the backdrop, not the modal content
    let backdrop_click = {
        let on_close = props.on_close.clone();
        
        Callback::from(move |e: MouseEvent| {
            // Only close if the click target is exactly the backdrop element
            if let Some(target) = e.target() {
                if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                    if let Some(class_name) = element.get_attribute("class") {
                        // Only close if clicking exactly on the backdrop (not child elements)
                        if class_name == "modern-menu-designer" {
                            on_close.emit(());
                        }
                    }
                }
            }
        })
    };

    // Prevent modal content clicks from bubbling to backdrop
    let modal_content_click = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });

    // Handle Escape key to close modal
    let keydown_handler = {
        let on_close = props.on_close.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Escape" {
                e.prevent_default();
                e.stop_propagation();
                on_close.emit(());
            }
        })
    };

    // Set up keyboard event listener
    {
        let keydown_handler = keydown_handler.clone();
        use_effect_with_deps(move |_| {
            let handler = keydown_handler.clone();
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                handler.emit(e.into());
            }) as Box<dyn FnMut(_)>);
            
            if let Some(window) = web_sys::window() {
                let _ = window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            }
            
            move || {
                if let Some(window) = web_sys::window() {
                    let _ = window.remove_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
                }
            }
        }, ());
    }



    html! {
        <div 
            class="modern-menu-designer" 
            onclick={backdrop_click}
            style="
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: rgba(0, 0, 0, 0.5);
                z-index: 10000;
                display: flex;
                align-items: center;
                justify-content: center;
                padding: 20px;
            "
        >
            <div 
                class="designer-modal" 
                onclick={modal_content_click}
                style="
                    background: white;
                    border-radius: 12px;
                    box-shadow: 0 25px 50px rgba(0, 0, 0, 0.25);
                    width: 100%;
                    max-width: 1200px;
                    max-height: 90vh;
                    display: flex;
                    flex-direction: column;
                    overflow: hidden;
                "
            >
                // Header
                <div class="designer-header" style="
                    background: #f8fafc;
                    border-bottom: 1px solid #e2e8f0;
                    padding: 24px 32px;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                ">
                    <div>
                        <h2 style="margin: 0; color: #1e293b; font-size: 20px; font-weight: 600;">
                            {format!("Customize {} Menu", props.menu_area.to_uppercase())}
                        </h2>
                        <p style="margin: 4px 0 0 0; color: #64748b; font-size: 14px;">
                            {"Configure your menu appearance and behavior"}
                        </p>
                    </div>
                    <button 
                        onclick={{
                            let on_close = props.on_close.clone();
                            Callback::from(move |e: MouseEvent| {
                                e.stop_propagation();
                                on_close.emit(());
                            })
                        }}
                        style="
                            background: none;
                            border: none;
                            font-size: 24px;
                            color: #64748b;
                            cursor: pointer;
                            padding: 8px;
                            border-radius: 6px;
                            transition: all 0.2s ease;
                        "
                    >
                        {"×"}
                    </button>
                </div>

                // Main Content
                <div class="designer-content" style="
                    flex: 1;
                    display: flex;
                    overflow: hidden;
                ">
                    // Properties Panel
                    <div class="properties-panel" style="
                        width: 400px;
                        background: #fafbfc;
                        border-right: 1px solid #e2e8f0;
                        display: flex;
                        flex-direction: column;
                    ">
                        // Tabs
                        <div class="tabs" style="
                            display: flex;
                            background: white;
                            border-bottom: 1px solid #e2e8f0;
                        ">
                            {render_tab("colors", "Colors", &active_tab)}
                            {render_tab("animations", "Animations", &active_tab)}
                            {render_tab("layout", "Layout", &active_tab)}
                            {render_tab("style", "Style", &active_tab)}
                            {render_tab("mobile", "Mobile", &active_tab)}
                        </div>

                        // Tab Content
                        <div class="tab-content" style="
                            flex: 1;
                            padding: 24px;
                            overflow-y: auto;
                        ">
                            {match (*active_tab).as_str() {
                                "colors" => render_colors_tab(&current_style.properties, update_property.clone()),
                                "animations" => render_animations_tab(&current_style.properties, update_property.clone()),
                                "layout" => render_layout_tab(&current_style.properties, update_property.clone()),
                                "style" => render_style_tab(&current_style.properties, update_property.clone()),
                                "mobile" => render_mobile_tab(&current_style.properties, update_property.clone()),
                                _ => html! {}
                            }}
                        </div>
                    </div>

                    // Preview Panel
                    <div class="preview-panel" style="
                        flex: 1;
                        background: #f1f5f9;
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        justify-content: center;
                        padding: 40px;
                    ">
                        <div class="preview-container" style="
                            background: white;
                            border-radius: 12px;
                            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
                            padding: 32px;
                            width: 100%;
                            max-width: 600px;
                        ">
                            <h3 style="margin: 0 0 24px 0; color: #374151; text-align: center;">
                                {"Live Preview"}
                            </h3>
                            {render_menu_preview(&current_style.properties, &props.menu_area)}
                        </div>
                    </div>
                </div>

                // Footer
                <div class="designer-footer" style="
                    background: white;
                    border-top: 1px solid #e2e8f0;
                    padding: 20px 32px;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                ">
                    <div style="color: #6b7280; font-size: 14px;">
                        {"Changes are applied automatically"}
                    </div>
                    <div style="display: flex; gap: 12px;">
                        <button 
                            onclick={{
                                let on_close = props.on_close.clone();
                                Callback::from(move |e: MouseEvent| {
                                    e.stop_propagation();
                                    on_close.emit(());
                                })
                            }}
                            style="
                                background: #f1f5f9;
                                border: 1px solid #d1d5db;
                                color: #374151;
                                padding: 10px 20px;
                                border-radius: 6px;
                                font-size: 14px;
                                font-weight: 500;
                                cursor: pointer;
                                transition: all 0.2s ease;
                            "
                        >
                            {"Cancel"}
                        </button>
                        <button 
                            onclick={{
                                let save_callback = save_callback.clone();
                                Callback::from(move |e: MouseEvent| {
                                    e.stop_propagation();
                                    save_callback.emit(());
                                })
                            }}
                            style="
                                background: #3b82f6;
                                border: 1px solid #3b82f6;
                                color: white;
                                padding: 10px 20px;
                                border-radius: 6px;
                                font-size: 14px;
                                font-weight: 500;
                                cursor: pointer;
                                transition: all 0.2s ease;
                            "
                        >
                            {"Save Changes"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn render_tab(tab_id: &str, label: &str, active_tab: &UseStateHandle<String>) -> Html {
    let is_active = **active_tab == tab_id;
    let active_tab = active_tab.clone();
    let tab_id = tab_id.to_string();
    
    html! {
        <button
            onclick={Callback::from(move |e: MouseEvent| {
                e.stop_propagation();
                active_tab.set(tab_id.clone());
            })}
            style={format!("
                flex: 1;
                padding: 12px 16px;
                border: none;
                background: {};
                color: {};
                font-size: 14px;
                font-weight: 500;
                cursor: pointer;
                border-bottom: 2px solid {};
                transition: all 0.2s ease;
            ", 
                if is_active { "#ffffff" } else { "transparent" },
                if is_active { "#3b82f6" } else { "#6b7280" },
                if is_active { "#3b82f6" } else { "transparent" }
            )}
        >
            {label}
        </button>
    }
}

fn render_colors_tab(properties: &MenuProperties, update_property: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="colors-tab">
            <div class="section" style="margin-bottom: 24px;">
                <h4 style="margin: 0 0 16px 0; color: #374151; font-size: 16px; font-weight: 600;">
                    {"Text Colors"}
                </h4>
                <div style="display: grid; gap: 16px;">
                    {render_color_input("Default Text", "text_color", &properties.text_color, update_property.clone())}
                    {render_color_input("Hover Text", "hover_color", &properties.hover_color, update_property.clone())}
                    {render_color_input("Active Text", "active_color", &properties.active_color, update_property.clone())}
                </div>
            </div>

            <div class="section" style="margin-bottom: 24px;">
                <h4 style="margin: 0 0 16px 0; color: #374151; font-size: 16px; font-weight: 600;">
                    {"Background Colors"}
                </h4>
                <div style="display: grid; gap: 16px;">
                    {render_color_input("Menu Background", "background", &properties.background, update_property.clone())}
                    {render_color_input("Hover Background", "hover_background", &properties.hover_background, update_property.clone())}
                    {render_color_input("Active Background", "active_background", &properties.active_background, update_property.clone())}
                </div>
            </div>
        </div>
    }
}

fn render_layout_tab(properties: &MenuProperties, update_property: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="layout-tab">
            <div class="section" style="margin-bottom: 24px;">
                <h4 style="margin: 0 0 16px 0; color: #374151; font-size: 16px; font-weight: 600;">
                    {"Layout"}
                </h4>
                <div style="display: grid; gap: 16px;">
                    {render_select_input("Direction", "layout", &properties.layout, 
                        vec![("horizontal", "Horizontal"), ("vertical", "Vertical")], 
                        update_property.clone())}
                    {render_select_input("Alignment", "alignment", &properties.alignment,
                        vec![("left", "Left"), ("center", "Center"), ("right", "Right")],
                        update_property.clone())}
                </div>
            </div>

            <div class="section" style="margin-bottom: 24px;">
                <h4 style="margin: 0 0 16px 0; color: #374151; font-size: 16px; font-weight: 600;">
                    {"Spacing"}
                </h4>
                <div style="display: grid; gap: 16px;">
                    {render_text_input("Item Spacing", "item_spacing", &properties.item_spacing, "e.g., 16px, 1rem", update_property.clone())}
                    {render_text_input("Padding", "padding", &properties.padding, "e.g., 8px 16px", update_property.clone())}
                </div>
            </div>
        </div>
    }
}

fn render_style_tab(properties: &MenuProperties, update_property: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="style-tab">
            <div class="section" style="margin-bottom: 24px;">
                <h4 style="margin: 0 0 16px 0; color: #374151; font-size: 16px; font-weight: 600;">
                    {"Visual Style"}
                </h4>
                <div style="display: grid; gap: 16px;">
                    {render_text_input("Border Radius", "border_radius", &properties.border_radius, "e.g., 4px, 8px", update_property.clone())}
                    {render_text_input("Border", "border", &properties.border, "e.g., 1px solid #ccc", update_property.clone())}
                    {render_text_input("Shadow", "shadow", &properties.shadow, "e.g., 0 2px 4px rgba(0,0,0,0.1)", update_property.clone())}
                </div>
            </div>

            <div class="section" style="margin-bottom: 24px;">
                <h4 style="margin: 0 0 16px 0; color: #374151; font-size: 16px; font-weight: 600;">
                    {"Typography"}
                </h4>
                <div style="display: grid; gap: 16px;">
                    {render_text_input("Font Size", "font_size", &properties.font_size, "e.g., 14px, 1rem", update_property.clone())}
                    {render_select_input("Font Weight", "font_weight", &properties.font_weight,
                        vec![("400", "Normal"), ("500", "Medium"), ("600", "Semi-bold"), ("700", "Bold")],
                        update_property.clone())}
                    {render_select_input("Text Transform", "text_transform", &properties.text_transform,
                        vec![("none", "None"), ("uppercase", "Uppercase"), ("lowercase", "Lowercase"), ("capitalize", "Capitalize")],
                        update_property.clone())}
                </div>
            </div>

            <div class="section" style="margin-bottom: 24px;">
                <h4 style="margin: 0 0 16px 0; color: #374151; font-size: 16px; font-weight: 600;">
                    {"Hover Effects"}
                </h4>
                <div style="display: grid; gap: 16px;">
                    {render_text_input("Transition", "hover_transition", &properties.hover_transition, "e.g., all 0.2s ease", update_property.clone())}
                    {render_text_input("Transform", "hover_transform", &properties.hover_transform, "e.g., scale(1.05)", update_property.clone())}
                </div>
            </div>
        </div>
    }
}

fn render_mobile_tab(properties: &MenuProperties, update_property: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="mobile-tab">
            <div class="section" style="margin-bottom: 24px;">
                <h4 style="margin: 0 0 16px 0; color: #374151; font-size: 16px; font-weight: 600;">
                    {"Mobile Behavior"}
                </h4>
                <div style="display: grid; gap: 16px;">
                    {render_text_input("Breakpoint", "mobile_breakpoint", &properties.mobile_breakpoint, "e.g., 768px", update_property.clone())}
                    {render_select_input("Mobile Layout", "mobile_layout", &properties.mobile_layout,
                        vec![("hamburger", "Hamburger Menu"), ("stack", "Stacked"), ("scroll", "Horizontal Scroll")],
                        update_property.clone())}
                </div>
            </div>
        </div>
    }
}

fn render_animations_tab(properties: &MenuProperties, update_property: Callback<Vec<(String, String)>>) -> Html {
    html! {
        <div class="animations-tab">


            // Hover Animation Effects
            <div class="section" style="margin-bottom: 24px;">
                <h4 style="margin: 0 0 16px 0; color: #374151; font-size: 16px; font-weight: 600;">
                    {"Hover Animation"}
                </h4>
                <div style="display: grid; gap: 16px;">
                    {render_select_input("Animation Type", "hover_animation_type", &properties.hover_animation_type,
                        vec![
                            ("none", "None"),
                            ("lift", "Lift Up"),
                            ("scale", "Scale"),
                            ("glow", "Glow Effect")
                        ],
                        update_property.clone())}
                    {render_text_input("Duration", "hover_animation_duration", &properties.hover_animation_duration, "e.g., 0.3s", update_property.clone())}
                    {render_select_input("Easing", "hover_animation_easing", &properties.hover_animation_easing,
                        vec![
                            ("ease", "Ease"),
                            ("ease-in", "Ease In"),
                            ("ease-out", "Ease Out"),
                            ("ease-in-out", "Ease In Out"),
                            ("cubic-bezier(0.4, 0, 0.2, 1)", "Smooth"),
                            ("cubic-bezier(0.68, -0.55, 0.265, 1.55)", "Bouncy")
                        ],
                        update_property.clone())}
                </div>
            </div>

            // Transform Effects
            <div class="section" style="margin-bottom: 24px;">
                <h4 style="margin: 0 0 16px 0; color: #374151; font-size: 16px; font-weight: 600;">
                    {"Transform Effects"}
                </h4>
                <div style="display: grid; gap: 16px;">
                    {render_text_input("Scale Factor", "hover_scale", &properties.hover_scale, "e.g., 1.05", update_property.clone())}
                    {render_text_input("Lift Distance", "hover_lift_distance", &properties.hover_lift_distance, "e.g., 2px", update_property.clone())}
                </div>
            </div>

            // Glow & Border Effects
            <div class="section" style="margin-bottom: 24px;">
                <h4 style="margin: 0 0 16px 0; color: #374151; font-size: 16px; font-weight: 600;">
                    {"Glow & Border Effects"}
                </h4>
                <div style="display: grid; gap: 16px;">
                    {render_color_input("Glow Color", "hover_glow_color", &properties.hover_glow_color, update_property.clone())}
                    {render_text_input("Glow Intensity", "hover_glow_intensity", &properties.hover_glow_intensity, "e.g., 8px", update_property.clone())}
                    {render_color_input("Hover Border Color", "hover_border_color", &properties.hover_border_color, update_property.clone())}
                    {render_text_input("Border Width", "hover_border_width", &properties.hover_border_width, "e.g., 1px", update_property.clone())}
                </div>
            </div>

            // Simple Effects
            <div class="section" style="margin-bottom: 24px;">
                <h4 style="margin: 0 0 16px 0; color: #374151; font-size: 16px; font-weight: 600;">
                    {"Simple Effects"}
                </h4>
                <div style="display: grid; gap: 16px;">
                    {render_checkbox_input("Enable Underline Animation", "enable_underline_animation", properties.enable_underline_animation, update_property.clone())}
                    {render_color_input("Underline Color", "underline_color", &properties.underline_color, update_property.clone())}
                    {render_text_input("Underline Thickness", "underline_thickness", &properties.underline_thickness, "e.g., 2px", update_property.clone())}
                </div>
            </div>


        </div>
    }
}

fn render_color_input(label: &str, field: &str, value: &str, update_property: Callback<Vec<(String, String)>>) -> Html {
    let field = field.to_string();
    let original_value = value.to_string();
    let label = label.to_string();
    let update_property_clone = update_property.clone();
    
    // Convert color value to hex format for the color input
    let hex_value = convert_to_hex_color(&original_value);

    let onchange = {
        let update_property_clone = update_property_clone.clone();
        let field = field.clone();
        Callback::from(move |e: Event| {
            e.stop_propagation();
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let new_value = input.value();
                web_sys::console::log_1(&format!("🎨 Color change detected: {} = {}", field, new_value).into());
                update_property_clone.emit(vec![(field.clone(), new_value)]);
            }
        })
    };

    // Prevent clicks on the color input container from bubbling
    let container_click = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });

    html! {
        <div class="input-group">
            <label style="
                display: block;
                margin-bottom: 6px;
                color: #374151;
                font-size: 14px;
                font-weight: 500;
            ">
                {label}
            </label>
            <div 
                onclick={container_click}
                style="display: flex; gap: 8px; align-items: center;"
            >
                <input
                    type="color"
                    value={hex_value.clone()}
                    onchange={onchange.clone()}
                    oninput={{
                        let update_property_clone = update_property_clone.clone();
                        let field = field.clone();
                        Callback::from(move |e: InputEvent| {
                            e.stop_propagation();
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                let new_value = input.value();
                                web_sys::console::log_1(&format!("🎨 Color input change: {} = {}", field, new_value).into());
                                update_property_clone.emit(vec![(field.clone(), new_value)]);
                            }
                        })
                    }}
                    onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                    style="
                        width: 40px;
                        height: 32px;
                        border: 1px solid #d1d5db;
                        border-radius: 4px;
                        cursor: pointer;
                    "
                />
                <input
                    type="text"
                    value={original_value.clone()}
                    onchange={onchange}
                    onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                    onfocus={Callback::from(|e: FocusEvent| e.stop_propagation())}
                    style="
                        flex: 1;
                        padding: 8px 12px;
                        border: 1px solid #d1d5db;
                        border-radius: 4px;
                        font-size: 14px;
                        font-family: monospace;
                    "
                />
                <div style={format!("
                    width: 24px;
                    height: 24px;
                    border-radius: 50%;
                    background: {};
                    border: 2px solid #e5e7eb;
                    flex-shrink: 0;
                ", original_value)} title="Current color preview"></div>
            </div>
        </div>
    }
}

fn render_text_input(label: &str, field: &str, value: &str, placeholder: &str, update_property: Callback<Vec<(String, String)>>) -> Html {
    let field = field.to_string();
    let value = value.to_string();
    let label = label.to_string();
    let placeholder = placeholder.to_string();
    
    html! {
        <div class="input-group">
            <label style="
                display: block;
                margin-bottom: 6px;
                color: #374151;
                font-size: 14px;
                font-weight: 500;
            ">
                {label}
            </label>
            <input
                type="text"
                value={value}
                placeholder={placeholder}
                onchange={update_property.reform(move |e: Event| {
                    e.stop_propagation();
                    if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                        vec![(field.clone(), input.value())]
                    } else {
                        vec![]
                    }
                })}
                onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                onfocus={Callback::from(|e: FocusEvent| e.stop_propagation())}
                style="
                    width: 100%;
                    padding: 8px 12px;
                    border: 1px solid #d1d5db;
                    border-radius: 4px;
                    font-size: 14px;
                    font-family: monospace;
                "
            />
        </div>
    }
}

fn render_select_input(label: &str, field: &str, value: &str, options: Vec<(&str, &str)>, update_property: Callback<Vec<(String, String)>>) -> Html {
    let field = field.to_string();
    let value = value.to_string();
    let label = label.to_string();
    
    html! {
        <div class="input-group">
            <label style="
                display: block;
                margin-bottom: 6px;
                color: #374151;
                font-size: 14px;
                font-weight: 500;
            ">
                {label}
            </label>
            <select
                value={value.clone()}
                onchange={update_property.reform(move |e: Event| {
                    e.stop_propagation();
                    if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                        vec![(field.clone(), select.value())]
                    } else {
                        vec![]
                    }
                })}
                onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                onfocus={Callback::from(|e: FocusEvent| e.stop_propagation())}
                style="
                    width: 100%;
                    padding: 8px 12px;
                    border: 1px solid #d1d5db;
                    border-radius: 4px;
                    font-size: 14px;
                    background: white;
                "
            >
                {for options.iter().map(|(val, opt_label)| {
                    html! {
                        <option value={val.to_string()} selected={*val == value}>
                            {opt_label}
                        </option>
                    }
                })}
            </select>
        </div>
    }
}

fn render_menu_preview(properties: &MenuProperties, _menu_area: &str) -> Html {
    let menu_items = vec!["Home", "About", "Services", "Contact"];
    
    // Generate unique CSS class for this preview
    let preview_id = format!("menu-preview-{}", js_sys::Math::random().to_string().replace("0.", ""));
    
    // Generate CSS for interactive preview
    let preview_css = generate_preview_css(&preview_id, properties);
    inject_preview_css(&preview_css);
    
    html! {
        <div class={format!("menu-preview {}", preview_id)} style="
            display: flex;
            justify-content: center;
            padding: 20px;
            background: #f8fafc;
            border-radius: 8px;
            border: 1px solid #e2e8f0;
        ">
            <nav class="preview-nav" style={format!("
                display: flex;
                flex-direction: {};
                align-items: center;
                justify-content: {};
                gap: {};
                background: {};
                padding: {};
                border-radius: {};
                border: {};
                box-shadow: {};
            ", 
                if properties.layout == "vertical" { "column" } else { "row" },
                match properties.alignment.as_str() {
                    "center" => "center",
                    "right" => "flex-end",
                    _ => "flex-start"
                },
                properties.item_spacing,
                properties.background,
                properties.padding,
                properties.border_radius,
                properties.border,
                properties.shadow
            )}>
                {for menu_items.iter().map(|item| {
                    html! {
                        <a href="#" class="preview-item" style={format!("
                            color: {};
                            text-decoration: none;
                            font-size: {};
                            font-weight: {};
                            text-transform: {};
                            transition: {};
                            padding: 4px 8px;
                            border-radius: {};
                        ", 
                            properties.text_color,
                            properties.font_size,
                            properties.font_weight,
                            properties.text_transform,
                            properties.hover_transition,
                            properties.border_radius
                        )}>
                            {item}
                        </a>
                    }
                })}
            </nav>
        </div>
    }
}

fn generate_preview_css(preview_id: &str, properties: &MenuProperties) -> String {
    format!(r#"
        .{} .preview-item:hover {{
            color: {} !important;
            background: {} !important;
            transform: {} !important;
        }}
        
        .{} .preview-item:active {{
            color: {} !important;
            background: {} !important;
        }}
    "#, 
        preview_id, properties.hover_color, properties.hover_background, properties.hover_transform,
        preview_id, properties.active_color, properties.active_background
    )
}

fn inject_preview_css(css: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(head) = document.head() {
                // Remove existing preview styles
                if let Some(existing) = document.get_element_by_id("menu-preview-styles") {
                    existing.remove();
                }
                
                // Create new style element
                if let Ok(style_element) = document.create_element("style") {
                    style_element.set_id("menu-preview-styles");
                    style_element.set_text_content(Some(css));
                    let _ = head.append_child(&style_element);
                }
            }
        }
    }
}

// Re-export for compatibility with existing code
#[allow(dead_code)]
pub type MenuPreset = MenuProperties;

// Helper function to convert various color formats to hex for color input
fn convert_to_hex_color(color: &str) -> String {
    let color = color.trim();
    
    // If already hex, return as-is
    if color.starts_with('#') && color.len() == 7 {
        return color.to_string();
    }
    
    // Handle transparent
    if color == "transparent" {
        return "#000000".to_string(); // Default to black for transparent
    }
    
    // Handle rgba() colors - extract RGB values and ignore alpha
    if color.starts_with("rgba(") {
        if let Some(inner) = color.strip_prefix("rgba(").and_then(|s| s.strip_suffix(")")) {
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() >= 3 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    parts[0].trim().parse::<u8>(),
                    parts[1].trim().parse::<u8>(),
                    parts[2].trim().parse::<u8>(),
                ) {
                    return format!("#{:02x}{:02x}{:02x}", r, g, b);
                }
            }
        }
    }
    
    // Handle rgb() colors
    if color.starts_with("rgb(") {
        if let Some(inner) = color.strip_prefix("rgb(").and_then(|s| s.strip_suffix(")")) {
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() >= 3 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    parts[0].trim().parse::<u8>(),
                    parts[1].trim().parse::<u8>(),
                    parts[2].trim().parse::<u8>(),
                ) {
                    return format!("#{:02x}{:02x}{:02x}", r, g, b);
                }
            }
        }
    }
    
    // Handle named colors (basic set)
    match color.to_lowercase().as_str() {
        "black" => "#000000".to_string(),
        "white" => "#ffffff".to_string(),
        "red" => "#ff0000".to_string(),
        "green" => "#008000".to_string(),
        "blue" => "#0000ff".to_string(),
        "yellow" => "#ffff00".to_string(),
        "cyan" => "#00ffff".to_string(),
        "magenta" => "#ff00ff".to_string(),
        "gray" | "grey" => "#808080".to_string(),
        _ => "#000000".to_string(), // Default fallback
    }
}



fn render_checkbox_input(label: &str, field: &str, value: bool, update_property: Callback<Vec<(String, String)>>) -> Html {
    let field = field.to_string();
    let update_property_clone = update_property.clone();
    
    let onchange = Callback::from(move |e: Event| {
        e.stop_propagation();
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            let new_value = if input.checked() { "true" } else { "false" };
            update_property_clone.emit(vec![(field.clone(), new_value.to_string())]);
        }
    });

    html! {
        <div class="input-group">
            <label style="
                display: flex;
                align-items: center;
                gap: 8px;
                font-size: 14px;
                color: #374151;
                font-weight: 500;
                cursor: pointer;
            ">
                <input
                    type="checkbox"
                    checked={value}
                    onchange={onchange}
                    style="
                        width: 16px;
                        height: 16px;
                        accent-color: #3b82f6;
                        cursor: pointer;
                    "
                />
                {label}
            </label>
        </div>
    }
}