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
    pub text_primary: String,
    pub text_secondary: String,
    pub header_text_color: String,
    pub header_border_color: String,
    pub header_shadow: String,
    pub sidebar_border_color: String,
    pub sidebar_shadow: String,
    pub sidebar_section_title_color: String,
    pub sidebar_section_border_color: String,
    pub nav_link_text_color: String,
    pub nav_link_hover_bg: String,
    pub nav_link_hover_text: String,
    pub nav_link_active_shadow: String,
    pub nav_link_public_text: String,
    pub nav_link_public_hover_bg: String,
    pub nav_link_public_hover_text: String,
    pub card_bg: String,
    pub shadow_color: String,
    pub accent_color: String,
    pub header_gradient: String,
    pub sidebar_bg: String,
    pub header_logo_gradient: String,
    pub nav_link_active_bg: String,
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
            text_primary: "#111827".to_string(),
            text_secondary: "#6b7280".to_string(),
            header_text_color: "#ffffff".to_string(),
            header_border_color: "#e5e7eb".to_string(),
            header_shadow: "0 1px 3px rgba(0, 0, 0, 0.1)".to_string(),
            sidebar_border_color: "#e5e7eb".to_string(),
            sidebar_shadow: "0 1px 3px rgba(0, 0, 0, 0.1)".to_string(),
            sidebar_section_title_color: "#374151".to_string(),
            sidebar_section_border_color: "#e5e7eb".to_string(),
            nav_link_text_color: "#374151".to_string(),
            nav_link_hover_bg: "#f3f4f6".to_string(),
            nav_link_hover_text: "#111827".to_string(),
            nav_link_active_shadow: "0 0 0 2px #3b82f6".to_string(),
            nav_link_public_text: "#ffffff".to_string(),
            nav_link_public_hover_bg: "rgba(255, 255, 255, 0.1)".to_string(),
            nav_link_public_hover_text: "#ffffff".to_string(),
            card_bg: "#ffffff".to_string(),
            shadow_color: "rgba(0, 0, 0, 0.1)".to_string(),
            accent_color: "#3b82f6".to_string(),
            header_gradient: "linear-gradient(135deg, #3b82f6, #1d4ed8)".to_string(),
            sidebar_bg: "#ffffff".to_string(),
            header_logo_gradient: "linear-gradient(135deg, #3b82f6, #1d4ed8)".to_string(),
            nav_link_active_bg: "#eff6ff".to_string(),
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
            text_primary: "#f9fafb".to_string(),
            text_secondary: "#d1d5db".to_string(),
            header_text_color: "#ffffff".to_string(),
            header_border_color: "#374151".to_string(),
            header_shadow: "0 1px 3px rgba(0, 0, 0, 0.3)".to_string(),
            sidebar_border_color: "#374151".to_string(),
            sidebar_shadow: "0 1px 3px rgba(0, 0, 0, 0.3)".to_string(),
            sidebar_section_title_color: "#d1d5db".to_string(),
            sidebar_section_border_color: "#374151".to_string(),
            nav_link_text_color: "#d1d5db".to_string(),
            nav_link_hover_bg: "#374151".to_string(),
            nav_link_hover_text: "#f9fafb".to_string(),
            nav_link_active_shadow: "0 0 0 2px #3b82f6".to_string(),
            nav_link_public_text: "#ffffff".to_string(),
            nav_link_public_hover_bg: "rgba(255, 255, 255, 0.1)".to_string(),
            nav_link_public_hover_text: "#ffffff".to_string(),
            card_bg: "#1f2937".to_string(),
            shadow_color: "rgba(0, 0, 0, 0.3)".to_string(),
            accent_color: "#3b82f6".to_string(),
            header_gradient: "linear-gradient(135deg, #1f2937, #111827)".to_string(),
            sidebar_bg: "#1f2937".to_string(),
            header_logo_gradient: "linear-gradient(135deg, #3b82f6, #1d4ed8)".to_string(),
            nav_link_active_bg: "#1e40af".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PublicColorScheme {
    pub text_primary: String,
    pub text_secondary: String,
    pub text_meta: String,
    pub text_light: String,
    pub text_muted: String,
    pub heading_h1: String,
    pub heading_h2: String,
    pub heading_h3: String,
    pub heading_h4: String,
    pub link_primary: String,
    pub link_primary_gradient: String,
    pub link_hover: String,
    pub link_visited: String,
    pub link_active: String,
    pub header_text: String,
    pub header_text_hover: String,
    pub footer_text: String,
    pub footer_text_muted: String,
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
            text_primary: "#1f2937".to_string(),
            text_secondary: "#4b5563".to_string(),
            text_meta: "#6b7280".to_string(),
            text_light: "#9ca3af".to_string(),
            text_muted: "#d1d5db".to_string(),
            heading_h1: "#111827".to_string(),
            heading_h2: "#1f2937".to_string(),
            heading_h3: "#374151".to_string(),
            heading_h4: "#4b5563".to_string(),
            link_primary: "#3b82f6".to_string(),
            link_primary_gradient: "linear-gradient(135deg, #3b82f6, #1d4ed8)".to_string(),
            link_hover: "#1d4ed8".to_string(),
            link_visited: "#7c3aed".to_string(),
            link_active: "#1e40af".to_string(),
            header_text: "#ffffff".to_string(),
            header_text_hover: "#f3f4f6".to_string(),
            footer_text: "#d1d5db".to_string(),
            footer_text_muted: "#9ca3af".to_string(),
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
                        --admin-text-primary: {};
                        --admin-text-secondary: {};
                        --admin-header-text: {};
                        --admin-header-border: {};
                        --admin-header-shadow: {};
                        --admin-sidebar-border: {};
                        --admin-sidebar-shadow: {};
                        --admin-nav-link-text: {};
                        --admin-nav-link-hover-bg: {};
                        --admin-nav-link-hover-text: {};
                        --admin-card-bg: {};
                        --admin-shadow-color: {};
                        --admin-accent-color: {};
                        --admin-header-gradient: {};
                        --admin-sidebar-bg: {};
                    }}
                    
                    .admin-page {{
                        background-color: {} !important;
                        color: {} !important;
                    }}
                    
                    .admin-sidebar {{
                        background-color: {} !important;
                        border-color: {} !important;
                        box-shadow: {} !important;
                    }}
                    
                    .admin-header {{
                        background: {} !important;
                        color: {} !important;
                        border-color: {} !important;
                        box-shadow: {} !important;
                    }}
                    
                    .admin-nav-link {{
                        color: {} !important;
                    }}
                    
                    .admin-nav-link:hover {{
                        background-color: {} !important;
                        color: {} !important;
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
                    "#,
                    scheme.primary, scheme.primary_gradient, scheme.secondary, scheme.secondary_gradient, scheme.success, scheme.warning,
                    scheme.danger, scheme.info, scheme.background, scheme.surface,
                    scheme.border, scheme.text_primary, scheme.text_secondary,
                    scheme.header_text_color, scheme.header_border_color, scheme.header_shadow,
                    scheme.sidebar_border_color, scheme.sidebar_shadow, scheme.nav_link_text_color,
                    scheme.nav_link_hover_bg, scheme.nav_link_hover_text, scheme.card_bg,
                    scheme.shadow_color, scheme.accent_color, scheme.header_gradient,
                    scheme.sidebar_bg, scheme.background, scheme.text_primary,
                    scheme.sidebar_bg, scheme.sidebar_border_color, scheme.sidebar_shadow,
                    scheme.header_gradient, scheme.header_text_color, scheme.header_border_color,
                    scheme.header_shadow, scheme.nav_link_text_color, scheme.nav_link_hover_bg,
                    scheme.nav_link_hover_text, scheme.card_bg, scheme.border, scheme.shadow_color,
                    scheme.primary, scheme.primary, scheme.success, scheme.success,
                    scheme.warning, scheme.warning, scheme.danger, scheme.danger
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
                        --public-text-primary: {};
                        --public-text-secondary: {};
                        --public-text-meta: {};
                        --public-text-light: {};
                        --public-text-muted: {};
                        --public-heading-h1: {};
                        --public-heading-h2: {};
                        --public-heading-h3: {};
                        --public-heading-h4: {};
                        --public-link-primary: {};
                        --public-link-primary-gradient: {};
                        --public-link-hover: {};
                        --public-link-visited: {};
                        --public-link-active: {};
                        --public-header-text: {};
                        --public-header-text-hover: {};
                        --public-footer-text: {};
                        --public-footer-text-muted: {};
                        --public-success: {};
                        --public-warning: {};
                        --public-danger: {};
                        --public-info: {};
                        --public-border-light: {};
                        --public-background-light: {};
                        --public-hero-bg: {};
                        --public-card-shadow: {};
                    }}
                    
                    body {{
                        color: {} !important;
                    }}
                    
                    h1 {{
                        color: {} !important;
                    }}
                    
                    h2 {{
                        color: {} !important;
                    }}
                    
                    h3 {{
                        color: {} !important;
                    }}
                    
                    h4 {{
                        color: {} !important;
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
                    
                    .text-muted {{
                        color: {} !important;
                    }}
                    
                    .text-light {{
                        color: {} !important;
                    }}
                    
                    .text-meta {{
                        color: {} !important;
                    }}
                    
                    .alert-success {{
                        color: {} !important;
                        border-color: {} !important;
                    }}
                    
                    .alert-warning {{
                        color: {} !important;
                        border-color: {} !important;
                    }}
                    
                    .alert-danger {{
                        color: {} !important;
                        border-color: {} !important;
                    }}
                    
                    .alert-info {{
                        color: {} !important;
                        border-color: {} !important;
                    }}
                    "#,
                    scheme.text_primary, scheme.text_secondary, scheme.text_meta,
                    scheme.text_light, scheme.text_muted, scheme.heading_h1,
                    scheme.heading_h2, scheme.heading_h3, scheme.heading_h4,
                    scheme.link_primary, scheme.link_primary_gradient, scheme.link_hover, scheme.link_visited,
                    scheme.link_active, scheme.header_text, scheme.header_text_hover,
                    scheme.footer_text, scheme.footer_text_muted, scheme.success,
                    scheme.warning, scheme.danger, scheme.info, scheme.border_light,
                    scheme.background_light, scheme.hero_bg, scheme.card_shadow, scheme.text_primary,
                    scheme.heading_h1, scheme.heading_h2, scheme.heading_h3,
                    scheme.heading_h4, scheme.link_primary, scheme.link_hover,
                    scheme.link_visited, scheme.link_active, 
                    scheme.hero_bg, scheme.background_light, scheme.border_light,
                    scheme.card_shadow, scheme.text_muted, scheme.text_light,
                    scheme.text_meta, scheme.success, scheme.success,
                    scheme.warning, scheme.warning, scheme.danger, scheme.danger,
                    scheme.info, scheme.info
                );
                
                style_element.set_text_content(Some(&css_content));
                
                if let Some(head) = document.head() {
                    let _ = head.append_child(&style_element);
                }
                
                log::info!("✅ Applied comprehensive public theme: Modern Editorial - All text colors updated in DOM");
            }
        }
    }
}

#[function_component(AdminPreview)]
pub fn admin_preview(props: &AdminPreviewProps) -> Html {
    html! {
        <div class="admin-preview-container" style="padding: 1rem; border-radius: 8px; border: 1px solid #e5e7eb;">
            <div class="admin-header" style={format!("background: {}; color: {}; padding: 1rem; margin-bottom: 1rem; border-radius: 4px;", 
                props.scheme.header_gradient, props.scheme.header_text_color)}>
                <h4 style="margin: 0;">{"Admin Panel Header"}</h4>
            </div>
            
            <div class="admin-sidebar" style={format!("background: {}; padding: 1rem; margin-bottom: 1rem; border-radius: 4px; border: 1px solid {};", 
                props.scheme.sidebar_bg, props.scheme.sidebar_border_color)}>
                <h5 style={format!("color: {}; margin: 0 0 0.5rem 0;", props.scheme.sidebar_section_title_color)}>{"Navigation"}</h5>
                <div class="admin-nav-link" style={format!("color: {}; padding: 0.5rem; margin: 0.25rem 0; border-radius: 4px; cursor: pointer;", 
                    props.scheme.nav_link_text_color)}>
                    {"Dashboard"}
                </div>
                <div class="admin-nav-link" style={format!("color: {}; background: {}; padding: 0.5rem; margin: 0.25rem 0; border-radius: 4px; cursor: pointer;", 
                    props.scheme.nav_link_hover_text, props.scheme.nav_link_hover_bg)}>
                    {"Posts (Active)"}
                </div>
            </div>
            
            <div class="admin-content" style={format!("background: {}; color: {}; padding: 1rem; border-radius: 4px;", 
                props.scheme.background, props.scheme.text_primary)}>
                <h5 style={format!("color: {}; margin: 0 0 0.5rem 0;", props.scheme.text_primary)}>{"Content Area"}</h5>
                <p style={format!("color: {}; margin: 0 0 1rem 0;", props.scheme.text_secondary)}>{"This is how your admin interface will look."}</p>
                
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
    }
}

#[derive(Properties, PartialEq)]
pub struct AdminPreviewProps {
    pub scheme: AdminColorScheme,
}

#[function_component(PublicPreview)]
pub fn public_preview(props: &PublicPreviewProps) -> Html {
    html! {
        <div class="public-preview-container" style="padding: 1rem; border-radius: 8px; border: 1px solid #e5e7eb;">
            <div class="site-header" style={format!("color: {}; padding: 1rem; margin-bottom: 1rem; border-radius: 4px; border: 1px solid #e5e7eb;", 
                props.scheme.header_text)}>
                <h4 style="margin: 0;">{"Site Header"}</h4>
                <nav style="margin-top: 0.5rem;">
                    <a href="#" style={format!("color: {}; margin-right: 1rem; text-decoration: none;", props.scheme.header_text)}>{"Home"}</a>
                    <a href="#" style={format!("color: {}; margin-right: 1rem; text-decoration: none;", props.scheme.header_text_hover)}>{"About"}</a>
                    <a href="#" style={format!("color: {}; text-decoration: none;", props.scheme.header_text)}>{"Contact"}</a>
                </nav>
            </div>
            
            <div class="hero-section" style={format!("background: {}; padding: 1rem; margin-bottom: 1rem; border-radius: 4px;", 
                props.scheme.hero_bg)}>
                <h1 style={format!("color: {}; font-weight: 700; margin: 0 0 0.5rem 0;", props.scheme.heading_h1)}>{"Welcome to Your Site"}</h1>
                <h2 style={format!("color: {}; font-weight: 600; margin: 0 0 0.5rem 0;", props.scheme.heading_h2)}>{"Subheading"}</h2>
                <p style={format!("color: {}; margin: 0 0 1rem 0;", props.scheme.text_primary)}>{"This is how your public site will look with the selected theme."}</p>
                <p style={format!("color: {}; font-size: 0.9rem; margin: 0;", props.scheme.text_secondary)}>{"Secondary text and descriptions will appear like this."}</p>
            </div>
            
            <div class="content-area" style="margin-bottom: 1rem;">
                <h3 style={format!("color: {}; font-weight: 600; margin: 0 0 0.5rem 0;", props.scheme.heading_h3)}>{"Content Section"}</h3>
                <p style={format!("color: {}; margin: 0 0 0.5rem 0;", props.scheme.text_primary)}>
                    {"Here's a sample paragraph with a "}
                    <a href="#" style={format!("color: {}; text-decoration: underline;", props.scheme.link_primary)}>{"primary link"}</a>
                    {" and some "}
                    <span style={format!("color: {};", props.scheme.text_meta)}>{"meta information"}</span>
                    {"."}
                </p>
                <p style={format!("color: {}; font-size: 0.8rem; margin: 0;", props.scheme.text_light)}>{"Light text for captions and less important information."}</p>
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
            
            <div class="site-footer" style={format!("color: {}; padding: 1rem; margin-top: 1rem; border-radius: 4px; border: 1px solid #e5e7eb;", 
                props.scheme.footer_text)}>
                <p style="margin: 0; font-size: 0.9rem;">{"Site Footer"}</p>
                <p style={format!("color: {}; margin: 0; font-size: 0.8rem;", props.scheme.footer_text_muted)}>{"Footer text and links"}</p>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PublicPreviewProps {
    pub scheme: PublicColorScheme,
}

#[function_component(DesignSystemPage)]
pub fn design_system_page() -> Html {
    let current_tab = use_state(|| "admin".to_string());
    let admin_scheme = use_state(|| AdminColorScheme::default());
    let public_scheme = use_state(|| PublicColorScheme::default());
    let selected_preset = use_state(|| "Light Preset".to_string());
    let saved_themes = use_state(|| vec!["Light Preset".to_string(), "Dark Preset".to_string()]);
    let theme_name_input = use_state(|| String::new());

    // Load theme settings on mount
    {
        let admin_scheme = admin_scheme.clone();
        let public_scheme = public_scheme.clone();
        let selected_preset = selected_preset.clone();
        
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // Load admin theme
                match get_settings(Some("theme")).await {
                    Ok(theme_settings) => {
                        let mut current_theme = String::new();
                        let mut found_current_theme = false;

                        for setting in theme_settings {
                            if setting.setting_key == "current_admin_theme" {
                                if let Some(theme_name) = setting.setting_value {
                                    current_theme = theme_name;
                                    found_current_theme = true;
                                }
                            }
                        }

                        if found_current_theme {
                            let scheme = match current_theme.as_str() {
                                "Dark Preset" => AdminColorScheme::dark_mode(),
                                _ => AdminColorScheme::default(),
                            };
                            selected_preset.set(current_theme.clone());
                            admin_scheme.set(scheme.clone());
                            apply_admin_css_variables(&scheme);
                        } else {
                            let default_scheme = AdminColorScheme::default();
                            admin_scheme.set(default_scheme.clone());
                            apply_admin_css_variables(&default_scheme);
                        }
                    },
                    Err(_) => {
                        let default_scheme = AdminColorScheme::default();
                        admin_scheme.set(default_scheme.clone());
                        apply_admin_css_variables(&default_scheme);
                    }
                }

                // Apply default public theme
                let default_public = PublicColorScheme::default();
                public_scheme.set(default_public.clone());
                apply_public_css_variables(&default_public);
            });
            || ()
        }, ());
    }

    let on_tab_change = {
        let current_tab = current_tab.clone();
        Callback::from(move |tab: String| {
            current_tab.set(tab);
        })
    };

    let on_admin_color_change = {
        let admin_scheme = admin_scheme.clone();
        Callback::from(move |(field, value): (String, String)| {
            let mut scheme = (*admin_scheme).clone();
            match field.as_str() {
                "primary" => scheme.primary = value,
                "primary_gradient" => scheme.primary_gradient = value,
                "secondary" => scheme.secondary = value,
                "secondary_gradient" => scheme.secondary_gradient = value,
                "success" => scheme.success = value,
                "warning" => scheme.warning = value,
                "danger" => scheme.danger = value,
                "info" => scheme.info = value,
                "background" => scheme.background = value,
                "surface" => scheme.surface = value,
                "border" => scheme.border = value,
                "text_primary" => scheme.text_primary = value,
                "text_secondary" => scheme.text_secondary = value,
                "header_text_color" => scheme.header_text_color = value,
                "sidebar_bg" => scheme.sidebar_bg = value,
                "card_bg" => scheme.card_bg = value,
                _ => {}
            }
            admin_scheme.set(scheme.clone());
            apply_admin_css_variables(&scheme);
        })
    };

    let on_public_color_change = {
        let public_scheme = public_scheme.clone();
        Callback::from(move |(field, value): (String, String)| {
            let mut scheme = (*public_scheme).clone();
            match field.as_str() {
                "text_primary" => scheme.text_primary = value,
                "text_secondary" => scheme.text_secondary = value,
                "text_meta" => scheme.text_meta = value,
                "text_light" => scheme.text_light = value,
                "heading_h1" => scheme.heading_h1 = value,
                "heading_h2" => scheme.heading_h2 = value,
                "heading_h3" => scheme.heading_h3 = value,
                "link_primary" => scheme.link_primary = value,
                "link_primary_gradient" => scheme.link_primary_gradient = value,
                "link_hover" => scheme.link_hover = value,

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

    let on_preset_change = {
        let admin_scheme = admin_scheme.clone();
        let selected_preset = selected_preset.clone();
        Callback::from(move |event: web_sys::Event| {
            let select = event.target().unwrap().dyn_into::<web_sys::HtmlSelectElement>().unwrap();
            let theme_name = select.value();
            
            let scheme = match theme_name.as_str() {
                "Dark Preset" => AdminColorScheme::dark_mode(),
                _ => AdminColorScheme::default(),
            };
            
            selected_preset.set(theme_name.clone());
            admin_scheme.set(scheme.clone());
            apply_admin_css_variables(&scheme);
            
            // Save to database
            let theme_name_clone = theme_name.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let setting = SettingData {
                    key: "current_admin_theme".to_string(),
                    value: theme_name_clone,
                    setting_type: "theme".to_string(),
                    description: Some("Current admin theme".to_string()),
                };
                let _ = update_settings(vec![setting]).await;
            });
        })
    };

    let on_save_theme = {
        let theme_name_input = theme_name_input.clone();
        let admin_scheme = admin_scheme.clone();
        let saved_themes = saved_themes.clone();
        Callback::from(move |_| {
            let theme_name = (*theme_name_input).clone();
            if !theme_name.is_empty() {
                let scheme = (*admin_scheme).clone();
                
                // Add to saved themes
                let mut themes = (*saved_themes).clone();
                if !themes.contains(&theme_name) {
                    themes.push(theme_name.clone());
                    saved_themes.set(themes);
                }
                
                // Save to database
                let theme_name_clone = theme_name.clone();
                let scheme_clone = scheme.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let theme_data = serde_json::to_string(&scheme_clone).unwrap_or_default();
                    let setting = SettingData {
                        key: format!("admin_theme_{}", theme_name_clone),
                        value: theme_data,
                        setting_type: "theme".to_string(),
                        description: Some(format!("Custom admin theme: {}", theme_name_clone)),
                    };
                    let _ = update_settings(vec![setting]).await;
                });
                
                theme_name_input.set(String::new());
            }
        })
    };

    html! {
        <div class="admin-page">
            <div class="design-system-header" style="margin-bottom: 2rem;">
                <h2>{"🎨 Design System"}</h2>
                <p>{"Comprehensive theme and typography management for your CMS"}</p>
            </div>

            <div class="design-tabs" style="display: flex; gap: 0.5rem; margin-bottom: 2rem; border-bottom: 1px solid #e5e7eb;">
                <button 
                    class={if (*current_tab).as_str() == "admin" { "tab-button active" } else { "tab-button" }}
                    style={format!("padding: 0.75rem 1.5rem; border: none; background: {}; color: {}; cursor: pointer; border-radius: 4px 4px 0 0;", 
                        if (*current_tab).as_str() == "admin" { "#3b82f6" } else { "transparent" },
                        if (*current_tab).as_str() == "admin" { "white" } else { "#6b7280" })}
                    onclick={let on_tab_change = on_tab_change.clone(); Callback::from(move |_| on_tab_change.emit("admin".to_string()))}
                >
                    {"Admin Theme"}
                </button>
                <button 
                    class={if (*current_tab).as_str() == "public" { "tab-button active" } else { "tab-button" }}
                    style={format!("padding: 0.75rem 1.5rem; border: none; background: {}; color: {}; cursor: pointer; border-radius: 4px 4px 0 0;", 
                        if (*current_tab).as_str() == "public" { "#3b82f6" } else { "transparent" },
                        if (*current_tab).as_str() == "public" { "white" } else { "#6b7280" })}
                    onclick={let on_tab_change = on_tab_change.clone(); Callback::from(move |_| on_tab_change.emit("public".to_string()))}
                >
                    {"Public Theme"}
                </button>
                <button 
                    class={if (*current_tab).as_str() == "typography" { "tab-button active" } else { "tab-button" }}
                    style={format!("padding: 0.75rem 1.5rem; border: none; background: {}; color: {}; cursor: pointer; border-radius: 4px 4px 0 0;", 
                        if (*current_tab).as_str() == "typography" { "#3b82f6" } else { "transparent" },
                        if (*current_tab).as_str() == "typography" { "white" } else { "#6b7280" })}
                    onclick={let on_tab_change = on_tab_change.clone(); Callback::from(move |_| on_tab_change.emit("typography".to_string()))}
                >
                    {"Typography"}
                </button>
            </div>

            <div class="design-content">
                {match (*current_tab).as_str() {
                    "admin" => html! {
                        <div class="admin-theme-section">
                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem;">
                                <div class="theme-controls">
                                    <h3>{"Admin Theme Settings"}</h3>
                                    
                                    <div class="preset-section" style="margin-bottom: 2rem;">
                                        <h4>{"Theme Presets"}</h4>
                                        <div style="display: flex; gap: 1rem; align-items: center; margin-bottom: 1rem;">
                                            <select 
                                                value={(*selected_preset).clone()}
                                                onchange={on_preset_change}
                                                style="padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 4px; min-width: 150px;"
                                            >
                                                {for saved_themes.iter().map(|theme| {
                                                    html! {
                                                        <option value={theme.clone()}>{theme.clone()}</option>
                                                    }
                                                })}
                                            </select>
                                        </div>
                                        
                                        <div style="display: flex; gap: 0.5rem; align-items: center;">
                                            <input 
                                                type="text"
                                                placeholder="New theme name"
                                                value={(*theme_name_input).clone()}
                                                oninput={let theme_name_input = theme_name_input.clone(); Callback::from(move |e: InputEvent| {
                                                    let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                                    theme_name_input.set(input.value());
                                                })}
                                                style="padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 4px; flex: 1;"
                                            />
                                            <button 
                                                onclick={on_save_theme}
                                                style="padding: 0.5rem 1rem; background: #10b981; color: white; border: none; border-radius: 4px; cursor: pointer;"
                                            >
                                                {"Save Theme"}
                                            </button>
                                        </div>
                                    </div>
                                    
                                    <div class="color-controls" style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
                                        <div class="color-group">
                                            <h5>{"Brand Colors"}</h5>
                                            {render_color_input("Primary", "primary", &(*admin_scheme).primary, &on_admin_color_change)}
                                            {render_gradient_input("Primary Gradient", "primary_gradient", &(*admin_scheme).primary_gradient, &on_admin_color_change)}
                                            {render_color_input("Secondary", "secondary", &(*admin_scheme).secondary, &on_admin_color_change)}
                                            {render_gradient_input("Secondary Gradient", "secondary_gradient", &(*admin_scheme).secondary_gradient, &on_admin_color_change)}
                                            {render_color_input("Success", "success", &(*admin_scheme).success, &on_admin_color_change)}
                                            {render_color_input("Warning", "warning", &(*admin_scheme).warning, &on_admin_color_change)}
                                            {render_color_input("Danger", "danger", &(*admin_scheme).danger, &on_admin_color_change)}
                                            {render_color_input("Info", "info", &(*admin_scheme).info, &on_admin_color_change)}
                                        </div>
                                        
                                        <div class="color-group">
                                            <h5>{"Layout Colors"}</h5>
                                            {render_color_input("Background", "background", &(*admin_scheme).background, &on_admin_color_change)}
                                            {render_color_input("Surface", "surface", &(*admin_scheme).surface, &on_admin_color_change)}
                                            {render_color_input("Border", "border", &(*admin_scheme).border, &on_admin_color_change)}
                                            {render_color_input("Text Primary", "text_primary", &(*admin_scheme).text_primary, &on_admin_color_change)}
                                            {render_color_input("Text Secondary", "text_secondary", &(*admin_scheme).text_secondary, &on_admin_color_change)}
                                            {render_color_input("Header Text", "header_text_color", &(*admin_scheme).header_text_color, &on_admin_color_change)}
                                        </div>
                                        
                                        <div class="color-group">
                                            <h5>{"Component Colors"}</h5>
                                            {render_color_input("Sidebar Background", "sidebar_bg", &(*admin_scheme).sidebar_bg, &on_admin_color_change)}
                                            {render_color_input("Card Background", "card_bg", &(*admin_scheme).card_bg, &on_admin_color_change)}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    },
                    "public" => html! {
                        <div class="public-theme-section">
                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem;">
                                <div class="theme-controls">
                                    <h3>{"Public Theme Settings"}</h3>
                                    
                                    <div class="color-controls" style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
                                        <div class="color-group">
                                            <h5>{"Text Colors"}</h5>
                                            {render_public_color_input("Primary Text", "text_primary", &(*public_scheme).text_primary, &on_public_color_change)}
                                            {render_public_color_input("Secondary Text", "text_secondary", &(*public_scheme).text_secondary, &on_public_color_change)}
                                            {render_public_color_input("Meta Text", "text_meta", &(*public_scheme).text_meta, &on_public_color_change)}
                                            {render_public_color_input("Light Text", "text_light", &(*public_scheme).text_light, &on_public_color_change)}
                                        </div>
                                        
                                        <div class="color-group">
                                            <h5>{"Heading Colors"}</h5>
                                            {render_public_color_input("H1 Headings", "heading_h1", &(*public_scheme).heading_h1, &on_public_color_change)}
                                            {render_public_color_input("H2 Headings", "heading_h2", &(*public_scheme).heading_h2, &on_public_color_change)}
                                            {render_public_color_input("H3 Headings", "heading_h3", &(*public_scheme).heading_h3, &on_public_color_change)}
                                        </div>
                                        
                                        <div class="color-group">
                                            <h5>{"Link Colors"}</h5>
                                            {render_public_color_input("Primary Links", "link_primary", &(*public_scheme).link_primary, &on_public_color_change)}
                                            {render_gradient_input("Link Gradient", "link_primary_gradient", &(*public_scheme).link_primary_gradient, &on_public_color_change)}
                                            {render_public_color_input("Link Hover", "link_hover", &(*public_scheme).link_hover, &on_public_color_change)}
                                        </div>
                                        

                                        
                                        <div class="color-group">
                                            <h5>{"Alert Colors"}</h5>
                                            {render_public_color_input("Success", "success", &(*public_scheme).success, &on_public_color_change)}
                                            {render_public_color_input("Warning", "warning", &(*public_scheme).warning, &on_public_color_change)}
                                            {render_public_color_input("Danger", "danger", &(*public_scheme).danger, &on_public_color_change)}
                                            {render_public_color_input("Info", "info", &(*public_scheme).info, &on_public_color_change)}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    },
                    "typography" => html! {
                        <div class="typography-section">
                            <TypographySystem />
                        </div>
                    },
                    _ => html! { <div>{"Unknown tab"}</div> }
                }}
            </div>
        </div>
    }
}

fn render_gradient_input(label: &str, field: &str, value: &str, on_change: &Callback<(String, String)>) -> Html {
    let field = field.to_string();
    let on_change = on_change.clone();
    
    html! {
        <div style="margin-bottom: 1rem;">
            <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151;">{label}</label>
            <div style="display: flex; gap: 0.5rem; align-items: center;">
                <input
                    type="text"
                    value={value.to_string()}
                    placeholder="linear-gradient(135deg, #color1, #color2)"
                    style="flex: 1; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.875rem; font-family: monospace;"
                    oninput={
                        let on_change = on_change.clone();
                        let field = field.clone();
                        Callback::from(move |e: web_sys::InputEvent| {
                            let input = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
                            on_change.emit((field.clone(), input.value()));
                        })
                    }
                />
                <div 
                    style={format!("width: 2.5rem; height: 2.5rem; border-radius: 0.375rem; border: 1px solid #d1d5db; background: {}; flex-shrink: 0;", value)}
                    title="Gradient Preview"
                ></div>
            </div>
            <div style="font-size: 0.75rem; color: #6b7280; margin-top: 0.25rem;">
                {"Example: linear-gradient(135deg, #3b82f6, #1d4ed8)"}
            </div>
        </div>
    }
}

fn render_color_input(label: &str, field: &str, value: &str, on_change: &Callback<(String, String)>) -> Html {
    let field = field.to_string();
    let on_change = on_change.clone();
    
    html! {
        <div style="margin-bottom: 1rem;">
            <label style="display: block; margin-bottom: 0.25rem; font-weight: 500; font-size: 0.875rem;">
                {label}
            </label>
            <div style="display: flex; gap: 0.5rem; align-items: center;">
                <input 
                    type="color"
                    value={value.to_string()}
                    onchange={let field = field.clone(); let on_change = on_change.clone(); Callback::from(move |e: Event| {
                        let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        on_change.emit((field.clone(), input.value()));
                    })}
                    style="width: 40px; height: 32px; border: 1px solid #d1d5db; border-radius: 4px; cursor: pointer;"
                />
                <input 
                    type="text"
                    value={value.to_string()}
                    oninput={let field = field.clone(); Callback::from(move |e: InputEvent| {
                        let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        on_change.emit((field.clone(), input.value()));
                    })}
                    style="flex: 1; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 4px; font-family: monospace; font-size: 0.875rem;"
                />
            </div>
        </div>
    }
}

fn render_public_color_input(label: &str, field: &str, value: &str, on_change: &Callback<(String, String)>) -> Html {
    let field = field.to_string();
    let on_change = on_change.clone();
    
    html! {
        <div style="margin-bottom: 1rem;">
            <label style="display: block; margin-bottom: 0.25rem; font-weight: 500; font-size: 0.875rem;">
                {label}
            </label>
            <div style="display: flex; gap: 0.5rem; align-items: center;">
                <input 
                    type="color"
                    value={value.to_string()}
                    onchange={let field = field.clone(); let on_change = on_change.clone(); Callback::from(move |e: Event| {
                        let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        on_change.emit((field.clone(), input.value()));
                    })}
                    style="width: 40px; height: 32px; border: 1px solid #d1d5db; border-radius: 4px; cursor: pointer;"
                />
                <input 
                    type="text"
                    value={value.to_string()}
                    oninput={let field = field.clone(); Callback::from(move |e: InputEvent| {
                        let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        on_change.emit((field.clone(), input.value()));
                    })}
                    style="flex: 1; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 4px; font-family: monospace; font-size: 0.875rem;"
                />
            </div>
        </div>
    }
}

pub fn design_system_page() -> Html {
    html! { <DesignSystemPage /> }
}