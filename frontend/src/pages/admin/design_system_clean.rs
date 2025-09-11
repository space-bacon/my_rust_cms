use yew::prelude::*;
use crate::services::api_service::{get_settings, update_settings, SettingData};
use crate::pages::admin::TypographySystem;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminColorScheme {
    pub name: String,
    pub primary: String,
    pub primary_gradient: String,
    pub secondary: String,
    pub secondary_gradient: String,
    pub success: String,
    pub warning: String,
    pub danger: String,
    pub info: String,
    pub background: String,
    pub surface: String,
    pub border: String,
    pub header_border_color: String,
    pub header_shadow: String,
    pub sidebar_border_color: String,
    pub sidebar_shadow: String,
    pub sidebar_section_title_color: String,
    pub sidebar_section_border_color: String,
    pub nav_link_hover_bg: String,
    pub nav_link_active_shadow: String,
    pub nav_link_public_hover_bg: String,
    pub card_bg: String,
    pub shadow_color: String,
    pub accent_color: String,
    pub header_gradient: String,
    pub sidebar_bg: String,
    pub header_logo_gradient: String,
    pub nav_link_active_bg: String,
    pub admin_title_color: String,
    pub admin_user_bg: String,
    pub admin_user_text: String,
    pub admin_user_border: String,
}

impl Default for AdminColorScheme {
    fn default() -> Self {
        Self {
            name: "Light Theme".to_string(),
            primary: "#3b82f6".to_string(),
            primary_gradient: "linear-gradient(135deg, #3b82f6, #1d4ed8)".to_string(),
            secondary: "#6b7280".to_string(),
            secondary_gradient: "linear-gradient(135deg, #6b7280, #4b5563)".to_string(),
            success: "#10b981".to_string(),
            warning: "#f59e0b".to_string(),
            danger: "#ef4444".to_string(),
            info: "#06b6d4".to_string(),
            background: "#ffffff".to_string(),
            surface: "#f9fafb".to_string(),
            border: "#e5e7eb".to_string(),
            header_border_color: "#e5e7eb".to_string(),
            header_shadow: "0 1px 3px rgba(0, 0, 0, 0.1)".to_string(),
            sidebar_border_color: "#e5e7eb".to_string(),
            sidebar_shadow: "0 1px 3px rgba(0, 0, 0, 0.1)".to_string(),
            sidebar_section_title_color: "#374151".to_string(),
            sidebar_section_border_color: "#e5e7eb".to_string(),
            nav_link_hover_bg: "#f3f4f6".to_string(),
            nav_link_active_shadow: "0 0 0 2px #3b82f6".to_string(),
            nav_link_public_hover_bg: "rgba(255, 255, 255, 0.1)".to_string(),
            card_bg: "#ffffff".to_string(),
            shadow_color: "rgba(0, 0, 0, 0.1)".to_string(),
            accent_color: "#3b82f6".to_string(),
            header_gradient: "linear-gradient(135deg, #3b82f6, #1d4ed8)".to_string(),
            sidebar_bg: "#ffffff".to_string(),
            header_logo_gradient: "linear-gradient(135deg, #3b82f6, #1d4ed8)".to_string(),
            nav_link_active_bg: "#eff6ff".to_string(),
            admin_title_color: "#ffffff".to_string(),
            admin_user_bg: "rgba(255, 255, 255, 0.15)".to_string(),
            admin_user_text: "rgba(255, 255, 255, 0.9)".to_string(),
            admin_user_border: "rgba(255, 255, 255, 0.2)".to_string(),
        }
    }
}

impl AdminColorScheme {
    pub fn dark_mode() -> Self {
        Self {
            name: "Dark Theme".to_string(),
            primary: "#3b82f6".to_string(),
            primary_gradient: "linear-gradient(135deg, #3b82f6, #1e40af)".to_string(),
            secondary: "#6b7280".to_string(),
            secondary_gradient: "linear-gradient(135deg, #6b7280, #374151)".to_string(),
            success: "#10b981".to_string(),
            warning: "#f59e0b".to_string(),
            danger: "#ef4444".to_string(),
            info: "#06b6d4".to_string(),
            background: "#111827".to_string(),
            surface: "#1f2937".to_string(),
            border: "#374151".to_string(),
            header_border_color: "#374151".to_string(),
            header_shadow: "0 1px 3px rgba(0, 0, 0, 0.3)".to_string(),
            sidebar_border_color: "#374151".to_string(),
            sidebar_shadow: "0 1px 3px rgba(0, 0, 0, 0.3)".to_string(),
            sidebar_section_title_color: "#d1d5db".to_string(),
            sidebar_section_border_color: "#374151".to_string(),
            nav_link_hover_bg: "#374151".to_string(),
            nav_link_active_shadow: "0 0 0 2px #3b82f6".to_string(),
            nav_link_public_hover_bg: "rgba(255, 255, 255, 0.1)".to_string(),
            card_bg: "#1f2937".to_string(),
            shadow_color: "rgba(0, 0, 0, 0.3)".to_string(),
            accent_color: "#3b82f6".to_string(),
            header_gradient: "linear-gradient(135deg, #1f2937, #111827)".to_string(),
            sidebar_bg: "#1f2937".to_string(),
            header_logo_gradient: "linear-gradient(135deg, #3b82f6, #1d4ed8)".to_string(),
            nav_link_active_bg: "#1e40af".to_string(),
            admin_title_color: "#ffffff".to_string(),
            admin_user_bg: "rgba(255, 255, 255, 0.1)".to_string(),
            admin_user_text: "rgba(255, 255, 255, 0.85)".to_string(),
            admin_user_border: "rgba(255, 255, 255, 0.15)".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PublicColorScheme {
    pub link_primary: String,
    pub link_primary_gradient: String,
    pub link_hover: String,
    pub link_visited: String,
    pub link_active: String,
    pub success: String,
    pub warning: String,
    pub danger: String,
    pub info: String,
    pub border_light: String,
    pub background_light: String,
    pub hero_bg: String,
    pub card_shadow: String,
}

impl Default for PublicColorScheme {
    fn default() -> Self {
        Self {
            link_primary: "#3b82f6".to_string(),
            link_primary_gradient: "linear-gradient(135deg, #3b82f6, #1d4ed8)".to_string(),
            link_hover: "#1d4ed8".to_string(),
            link_visited: "#7c3aed".to_string(),
            link_active: "#1e40af".to_string(),
            success: "#10b981".to_string(),
            warning: "#f59e0b".to_string(),
            danger: "#ef4444".to_string(),
            info: "#06b6d4".to_string(),
            border_light: "#e5e7eb".to_string(),
            background_light: "#f9fafb".to_string(),
            hero_bg: "#f9fafb".to_string(),
            card_shadow: "0 1px 3px rgba(0, 0, 0, 0.1)".to_string(),
        }
    }
}

pub fn apply_admin_css_variables(scheme: &AdminColorScheme) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            // Remove existing admin theme style element
            if let Some(existing_style) = document.query_selector("#admin-theme-settings").ok().flatten() {
                let _ = existing_style.remove();
            }

            // Create new style element
            if let Ok(style_element) = document.create_element("style") {
                style_element.set_id("admin-theme-settings");
                
                let css_content = format!(
                    r#"
                    :root {{
                        --admin-primary: {};
                        --admin-primary-gradient: {};
                        --admin-secondary: {};
                        --admin-secondary-gradient: {};
                        --admin-success: {};
                        --admin-warning: {};
                        --admin-danger: {};
                        --admin-info: {};
                        --admin-background: {};
                        --admin-surface: {};
                        --admin-border: {};
                        --admin-header-border: {};
                        --admin-header-shadow: {};
                        --admin-sidebar-border: {};
                        --admin-sidebar-shadow: {};
                        --admin-nav-link-hover-bg: {};
                        --admin-card-bg: {};
                        --admin-shadow-color: {};
                        --admin-accent-color: {};
                        --admin-header-gradient: {};
                        --admin-sidebar-bg: {};
                        --admin-title-color: {};
                        --admin-header-logo-gradient: {};
                        --admin-user-bg: {};
                        --admin-user-text: {};
                        --admin-user-border: {};
                    }}
                    
                    .admin-page {{
                        background-color: {} !important;
                    }}
                    
                    .admin-sidebar {{
                        background-color: {} !important;
                        border-color: {} !important;
                        box-shadow: {} !important;
                    }}
                    
                    .admin-header {{
                        background: {} !important;
                        border-color: {} !important;
                        box-shadow: {} !important;
                    }}
                    
                    .admin-nav-link:hover {{
                        background-color: {} !important;
                    }}
                    
                    .admin-card {{
                        background-color: {} !important;
                        border-color: {} !important;
                        box-shadow: 0 1px 3px {} !important;
                    }}
                    
                    .btn-primary {{
                        background-color: {} !important;
                        border-color: {} !important;
                    }}
                    
                    .btn-success {{
                        background-color: {} !important;
                        border-color: {} !important;
                    }}
                    
                    .btn-warning {{
                        background-color: {} !important;
                        border-color: {} !important;
                    }}
                    
                    .btn-danger {{
                        background-color: {} !important;
                        border-color: {} !important;
                    }}
                    
                    .admin-header .admin-title {{
                        color: {} !important;
                    }}
                    
                    .admin-header .admin-title::before {{
                        background: {} !important;
                    }}
                    
                    .admin-user {{
                        background: {} !important;
                        color: {} !important;
                        border-color: {} !important;
                    }}
                    "#,
                    scheme.primary, scheme.primary_gradient, scheme.secondary, scheme.secondary_gradient, 
                    scheme.success, scheme.warning, scheme.danger, scheme.info, scheme.background, scheme.surface,
                    scheme.border, scheme.header_border_color, scheme.header_shadow,
                    scheme.sidebar_border_color, scheme.sidebar_shadow, scheme.nav_link_hover_bg, scheme.card_bg,
                    scheme.shadow_color, scheme.accent_color, scheme.primary_gradient,
                    scheme.sidebar_bg, scheme.admin_title_color, scheme.header_logo_gradient,
                    scheme.admin_user_bg, scheme.admin_user_text, scheme.admin_user_border,
                    scheme.background, scheme.sidebar_bg, scheme.sidebar_border_color, scheme.sidebar_shadow,
                    scheme.primary_gradient, scheme.header_border_color, scheme.header_shadow, 
                    scheme.nav_link_hover_bg, scheme.card_bg, scheme.border, scheme.shadow_color,
                    scheme.primary, scheme.primary, scheme.success, scheme.success,
                    scheme.warning, scheme.warning, scheme.danger, scheme.danger,
                    scheme.admin_title_color, scheme.header_logo_gradient,
                    scheme.admin_user_bg, scheme.admin_user_text, scheme.admin_user_border
                );
                
                style_element.set_text_content(Some(&css_content));
                
                if let Some(head) = document.head() {
                    let _ = head.append_child(&style_element);
                }
                
                log::info!("Applied admin theme: {}", scheme.name);
            }
        }
    }
}

pub fn apply_public_css_variables(scheme: &PublicColorScheme) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            // Remove existing public theme style element
            if let Some(existing_style) = document.query_selector("#public-theme-settings").ok().flatten() {
                let _ = existing_style.remove();
            }

            // Create new style element
            if let Ok(style_element) = document.create_element("style") {
                style_element.set_id("public-theme-settings");
                
                let css_content = format!(
                    r#"
                    :root {{
                        --public-link-primary: {};
                        --public-link-primary-gradient: {};
                        --public-link-hover: {};
                        --public-link-visited: {};
                        --public-link-active: {};
                        --public-success: {};
                        --public-warning: {};
                        --public-danger: {};
                        --public-info: {};
                        --public-border-light: {};
                        --public-background-light: {};
                        --public-hero-bg: {};
                        --public-card-shadow: {};
                    }}
                    
                    a {{
                        color: {} !important;
                    }}
                    
                    a:hover {{
                        color: {} !important;
                    }}
                    
                    a:visited {{
                        color: {} !important;
                    }}
                    
                    a:active {{
                        color: {} !important;
                    }}
                    
                    .hero-section {{
                        background-color: {} !important;
                    }}
                    
                    .card {{
                        background-color: {} !important;
                        border-color: {} !important;
                        box-shadow: {} !important;
                    }}
                    
                    .alert-success {{
                        background-color: {} !important;
                    }}
                    
                    .alert-warning {{
                        background-color: {} !important;
                    }}
                    
                    .alert-danger {{
                        background-color: {} !important;
                    }}
                    
                    .alert-info {{
                        background-color: {} !important;
                    }}
                    "#,
                    scheme.link_primary, scheme.link_primary_gradient, scheme.link_hover,
                    scheme.link_visited, scheme.link_active, scheme.success,
                    scheme.warning, scheme.danger, scheme.info, scheme.border_light,
                    scheme.background_light, scheme.hero_bg, scheme.card_shadow,
                    scheme.link_primary, scheme.link_hover, scheme.link_visited, scheme.link_active,
                    scheme.hero_bg, scheme.background_light, scheme.border_light, scheme.card_shadow,
                    scheme.success, scheme.warning, scheme.danger, scheme.info
                );
                
                style_element.set_text_content(Some(&css_content));
                
                if let Some(head) = document.head() {
                    let _ = head.append_child(&style_element);
                }
                
                log::info!("Applied public theme");
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AdminPreviewProps {
    pub scheme: AdminColorScheme,
}

#[function_component(AdminPreview)]
pub fn admin_preview(props: &AdminPreviewProps) -> Html {
    html! {
        <div class="admin-preview" style={format!("background: {}; padding: 1rem; border-radius: 8px; border: 1px solid {};", 
            props.scheme.background, props.scheme.border)}>
            
        <div class="admin-header" style={format!("background: {}; padding: 0.5rem 1rem; border-bottom: 1px solid {}; display: flex; justify-content: space-between; align-items: center;", 
            props.scheme.header_gradient, props.scheme.header_border_color)}>
            <div class="admin-title" style={format!("color: {}; font-weight: bold;", props.scheme.admin_title_color)}>
                {"Admin Panel"}
            </div>
            <div class="admin-user" style={format!("background: {}; color: {}; padding: 0.25rem 0.5rem; border-radius: 4px; border: 1px solid {};", 
                props.scheme.admin_user_bg, props.scheme.admin_user_text, props.scheme.admin_user_border)}>
                {"User"}
            </div>
        </div>
        
        <div class="admin-body" style="display: flex;">
            <div class="admin-sidebar" style={format!("background: {}; border-right: 1px solid {}; padding: 1rem; width: 200px;", 
                props.scheme.sidebar_bg, props.scheme.sidebar_border_color)}>
                
                <div class="admin-nav-link" style="padding: 0.5rem; margin: 0.25rem 0; border-radius: 4px; cursor: pointer;">
                    {"Dashboard"}
                </div>
                <div class="admin-nav-link" style={format!("background: {}; padding: 0.5rem; margin: 0.25rem 0; border-radius: 4px; cursor: pointer;", 
                    props.scheme.nav_link_hover_bg)}>
                    {"Posts (Active)"}
                </div>
            </div>
            
            <div class="admin-content" style={format!("background: {}; padding: 1rem; border-radius: 4px;", 
                props.scheme.background)}>
                <h5 style="margin: 0 0 0.5rem 0;">{"Content Area"}</h5>
                <p style="margin: 0 0 1rem 0;">{"This is how your admin interface will look."}</p>
                
                <div class="button-group" style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
                    <button class="btn-primary" style={format!("background: {}; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;", 
                        props.scheme.primary)}>
                        {"Primary"}
                    </button>
                    <button class="btn-success" style={format!("background: {}; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;", 
                        props.scheme.success)}>
                        {"Success"}
                    </button>
                    <button class="btn-warning" style={format!("background: {}; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;", 
                        props.scheme.warning)}>
                        {"Warning"}
                    </button>
                    <button class="btn-danger" style={format!("background: {}; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;", 
                        props.scheme.danger)}>
                        {"Danger"}
                    </button>
                </div>
            </div>
        </div>
    </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PublicPreviewProps {
    pub scheme: PublicColorScheme,
}

#[function_component(PublicPreview)]
pub fn public_preview(props: &PublicPreviewProps) -> Html {
    html! {
        <div class="public-preview" style={format!("background: {}; padding: 1rem; border-radius: 8px; border: 1px solid {};", 
            props.scheme.background_light, props.scheme.border_light)}>
            
        <div class="public-header" style={format!("background: {}; color: white; padding: 0.75rem 1rem; display: flex; justify-content: space-between; align-items: center;", 
                props.scheme.link_primary)}>
            <div class="site-title" style="font-weight: bold;">{"Your Site"}</div>
            <div class="nav-links" style="display: flex; gap: 1rem;">
                <a href="#" style="color: white; text-decoration: none;">{"Home"}</a>
                <a href="#" style="color: rgba(255,255,255,0.8); text-decoration: none;">{"About"}</a>
                <a href="#" style="color: white; text-decoration: none;">{"Contact"}</a>
            </div>
        </div>
        
        <div class="public-content" style="padding: 1rem;">
            <div class="hero-section" style={format!("background: {}; padding: 1rem; margin-bottom: 1rem; border-radius: 4px;", 
                props.scheme.hero_bg)}>
                <h1 style="font-weight: 700; margin: 0 0 0.5rem 0;">{"Welcome to Your Site"}</h1>
                <h2 style="font-weight: 600; margin: 0 0 0.5rem 0;">{"Subheading"}</h2>
                <p style="margin: 0 0 1rem 0;">{"This is how your public site will look with the selected theme."}</p>
                <p style="font-size: 0.9rem; margin: 0;">{"Secondary text and descriptions will appear like this."}</p>
            </div>
            
            <div class="content-area" style="margin-bottom: 1rem;">
                <h3 style="font-weight: 600; margin: 0 0 0.5rem 0;">{"Content Section"}</h3>
                <p style="margin: 0 0 0.5rem 0;">
                    {"Here's a sample paragraph with a "}
                    <a href="#" style={format!("color: {}; text-decoration: underline;", props.scheme.link_primary)}>{"primary link"}</a>
                    {" and some meta information."}
                </p>
                <p style="font-size: 0.8rem; margin: 0;">{"Light text for captions and less important information."}</p>
            </div>
            
            <div class="alerts-demo" style="display: flex; flex-direction: column; gap: 0.5rem;">
                <div class="alert-success" style={format!("background: {}; color: white; padding: 0.5rem; border-radius: 4px; font-size: 0.8rem;", 
                    props.scheme.success)}>
                    {"✓ Success message"}
                </div>
                <div class="alert-warning" style={format!("background: {}; color: white; padding: 0.5rem; border-radius: 4px; font-size: 0.8rem;", 
                    props.scheme.warning)}>
                    {"⚠ Warning message"}
                </div>
                <div class="alert-danger" style={format!("background: {}; color: white; padding: 0.5rem; border-radius: 4px; font-size: 0.8rem;", 
                    props.scheme.danger)}>
                    {"✗ Error message"}
                </div>
                <div class="alert-info" style={format!("background: {}; color: white; padding: 0.5rem; border-radius: 4px; font-size: 0.8rem;", 
                    props.scheme.info)}>
                    {"ℹ Info message"}
                </div>
            </div>
        </div>
        
        <div class="public-footer" style={format!("background: {}; color: white; padding: 0.5rem 1rem; text-align: center; margin-top: 1rem;", 
                props.scheme.link_primary)}>
            <p style="color: rgba(255,255,255,0.8); font-size: 0.8rem;">{"Footer text and links"}</p>
        </div>
    </div>
    }
}

#[function_component(DesignSystem)]
pub fn design_system() -> Html {
    let admin_scheme = use_state(AdminColorScheme::default);
    let public_scheme = use_state(PublicColorScheme::default);
    let selected_preset = use_state(|| "Light Preset".to_string());

    // Apply initial themes
    use_effect_with((), {
        let admin_scheme = admin_scheme.clone();
        let public_scheme = public_scheme.clone();
        move |_| {
            apply_admin_css_variables(&admin_scheme);
            apply_public_css_variables(&public_scheme);
            || ()
        }
    });

    let on_admin_color_change = {
        let admin_scheme = admin_scheme.clone();
        Callback::from(move |event: web_sys::Event| {
            let input = event.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            let color_name = input.get_attribute("data-color").unwrap();
            let value = input.value();
            
            let mut scheme = (*admin_scheme).clone();
            match color_name.as_str() {
                "primary" => scheme.primary = value,
                "secondary" => scheme.secondary = value,
                "success" => scheme.success = value,
                "warning" => scheme.warning = value,
                "danger" => scheme.danger = value,
                "info" => scheme.info = value,
                "background" => scheme.background = value,
                "surface" => scheme.surface = value,
                "border" => scheme.border = value,
                _ => {}
            }
            admin_scheme.set(scheme.clone());
            apply_admin_css_variables(&scheme);
        })
    };

    let on_public_color_change = {
        let public_scheme = public_scheme.clone();
        Callback::from(move |event: web_sys::Event| {
            let input = event.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            let color_name = input.get_attribute("data-color").unwrap();
            let value = input.value();
            
            let mut scheme = (*public_scheme).clone();
            match color_name.as_str() {
                "link_primary" => scheme.link_primary = value,
                "link_hover" => scheme.link_hover = value,
                "link_visited" => scheme.link_visited = value,
                "link_active" => scheme.link_active = value,
                "success" => scheme.success = value,
                "warning" => scheme.warning = value,
                "danger" => scheme.danger = value,
                "info" => scheme.info = value,
                _ => {}
            }
            public_scheme.set(scheme.clone());
            apply_public_css_variables(&scheme);
        })
    };

    let render_admin_color_input = |label: &str, color_name: &str, value: &str, callback: &Callback<web_sys::Event>| {
        html! {
            <div class="color-input-group" style="margin-bottom: 0.5rem;">
                <label style="display: block; font-size: 0.8rem; margin-bottom: 0.25rem;">{label}</label>
                <input 
                    type="color" 
                    value={value} 
                    data-color={color_name}
                    onchange={callback.clone()}
                    style="width: 100%; height: 2rem; border: 1px solid #ccc; border-radius: 4px; cursor: pointer;"
                />
            </div>
        }
    };

    let render_public_color_input = |label: &str, color_name: &str, value: &str, callback: &Callback<web_sys::Event>| {
        html! {
            <div class="color-input-group" style="margin-bottom: 0.5rem;">
                <label style="display: block; font-size: 0.8rem; margin-bottom: 0.25rem;">{label}</label>
                <input 
                    type="color" 
                    value={value} 
                    data-color={color_name}
                    onchange={callback.clone()}
                    style="width: 100%; height: 2rem; border: 1px solid #ccc; border-radius: 4px; cursor: pointer;"
                />
            </div>
        }
    };

    html! {
        <div class="design-system-page" style="padding: 2rem;">
            <div class="page-header" style="margin-bottom: 2rem;">
                <h1 style="margin: 0 0 0.5rem 0; font-size: 2rem; font-weight: 700;">{"Design System"}</h1>
                <p style="margin: 0; color: #6b7280;">{"Customize your site's visual appearance and branding"}</p>
            </div>

            <div class="design-tabs" style="display: flex; gap: 2rem;">
                <div class="admin-theme-section" style="flex: 1;">
                    <h2 style="margin: 0 0 1rem 0; font-size: 1.5rem; font-weight: 600;">{"Admin Theme"}</h2>
                    
                    <div class="theme-controls" style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem;">
                        <div class="color-controls">
                            <h3 style="margin: 0 0 1rem 0; font-size: 1.2rem; font-weight: 500;">{"Colors"}</h3>
                            
                            <div class="color-groups" style="display: grid; gap: 1rem;">
                                <div class="color-group">
                                    <h4 style="margin: 0 0 0.5rem 0; font-size: 1rem; font-weight: 500;">{"Primary Colors"}</h4>
                                    {render_admin_color_input("Primary", "primary", &(*admin_scheme).primary, &on_admin_color_change)}
                                    {render_admin_color_input("Secondary", "secondary", &(*admin_scheme).secondary, &on_admin_color_change)}
                                </div>
                                
                                <div class="color-group">
                                    <h4 style="margin: 0 0 0.5rem 0; font-size: 1rem; font-weight: 500;">{"Status Colors"}</h4>
                                    {render_admin_color_input("Success", "success", &(*admin_scheme).success, &on_admin_color_change)}
                                    {render_admin_color_input("Warning", "warning", &(*admin_scheme).warning, &on_admin_color_change)}
                                    {render_admin_color_input("Danger", "danger", &(*admin_scheme).danger, &on_admin_color_change)}
                                    {render_admin_color_input("Info", "info", &(*admin_scheme).info, &on_admin_color_change)}
                                </div>
                                
                                <div class="color-group">
                                    <h4 style="margin: 0 0 0.5rem 0; font-size: 1rem; font-weight: 500;">{"Layout Colors"}</h4>
                                    {render_admin_color_input("Background", "background", &(*admin_scheme).background, &on_admin_color_change)}
                                    {render_admin_color_input("Surface", "surface", &(*admin_scheme).surface, &on_admin_color_change)}
                                    {render_admin_color_input("Border", "border", &(*admin_scheme).border, &on_admin_color_change)}
                                </div>
                            </div>
                        </div>
                        
                        <div class="preview-section">
                            <h3 style="margin: 0 0 1rem 0; font-size: 1.2rem; font-weight: 500;">{"Preview"}</h3>
                            <AdminPreview scheme={(*admin_scheme).clone()} />
                        </div>
                    </div>
                </div>
                
                <div class="public-theme-section" style="flex: 1;">
                    <h2 style="margin: 0 0 1rem 0; font-size: 1.5rem; font-weight: 600;">{"Public Theme"}</h2>
                    
                    <div class="theme-controls" style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem;">
                        <div class="color-controls">
                            <h3 style="margin: 0 0 1rem 0; font-size: 1.2rem; font-weight: 500;">{"Colors"}</h3>
                            
                            <div class="color-groups" style="display: grid; gap: 1rem;">
                                <div class="color-group">
                                    <h4 style="margin: 0 0 0.5rem 0; font-size: 1rem; font-weight: 500;">{"Link Colors"}</h4>
                                    {render_public_color_input("Primary Link", "link_primary", &(*public_scheme).link_primary, &on_public_color_change)}
                                    {render_public_color_input("Link Hover", "link_hover", &(*public_scheme).link_hover, &on_public_color_change)}
                                    {render_public_color_input("Link Visited", "link_visited", &(*public_scheme).link_visited, &on_public_color_change)}
                                    {render_public_color_input("Link Active", "link_active", &(*public_scheme).link_active, &on_public_color_change)}
                                </div>
                                
                                <div class="color-group">
                                    <h4 style="margin: 0 0 0.5rem 0; font-size: 1rem; font-weight: 500;">{"Status Colors"}</h4>
                                    {render_public_color_input("Success", "success", &(*public_scheme).success, &on_public_color_change)}
                                    {render_public_color_input("Warning", "warning", &(*public_scheme).warning, &on_public_color_change)}
                                    {render_public_color_input("Danger", "danger", &(*public_scheme).danger, &on_public_color_change)}
                                    {render_public_color_input("Info", "info", &(*public_scheme).info, &on_public_color_change)}
                                </div>
                            </div>
                        </div>
                        
                        <div class="preview-section">
                            <h3 style="margin: 0 0 1rem 0; font-size: 1.2rem; font-weight: 500;">{"Preview"}</h3>
                            <PublicPreview scheme={(*public_scheme).clone()} />
                        </div>
                    </div>
                </div>
            </div>

            <div class="typography-section" style="margin-top: 3rem; padding-top: 2rem; border-top: 1px solid #e5e7eb;">
                <h2 style="margin: 0 0 1rem 0; font-size: 1.5rem; font-weight: 600;">{"Typography System"}</h2>
                <p style="margin: 0 0 1.5rem 0; color: #6b7280;">{"All typography colors and styling are now managed through the dedicated Typography System."}</p>
                <TypographySystem />
            </div>
        </div>
    }
}


