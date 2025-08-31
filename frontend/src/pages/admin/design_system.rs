use yew::prelude::*;
use crate::services::api_service::{get_settings, update_settings, SettingData};
use crate::pages::admin::TypographySystem;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use gloo_timers;

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
    
    // Button system colors
    pub button_primary_bg: String,
    pub button_primary_text: String,
    pub button_primary_hover_bg: String,
    pub button_primary_border: String,
    pub button_primary_hover_border: String,
    
    pub button_secondary_bg: String,
    pub button_secondary_text: String,
    pub button_secondary_hover_bg: String,
    pub button_secondary_border: String,
    pub button_secondary_hover_border: String,
    
    pub button_outline_bg: String,
    pub button_outline_text: String,
    pub button_outline_hover_bg: String,
    pub button_outline_hover_text: String,
    pub button_outline_border: String,
    
    pub button_ghost_bg: String,
    pub button_ghost_text: String,
    pub button_ghost_hover_bg: String,
    pub button_ghost_border: String,
    
    // Card system colors
    pub card_bg: String,
    pub card_border: String,
    pub card_hover_shadow: String,
    
    // Box system colors
    pub box_bg: String,
    pub box_border: String,
    
    // Alert system colors
    pub alert_success_bg: String,
    pub alert_success_border: String,
    pub alert_success_text: String,
    
    pub alert_warning_bg: String,
    pub alert_warning_border: String,
    pub alert_warning_text: String,
    
    pub alert_error_bg: String,
    pub alert_error_border: String,
    pub alert_error_text: String,
    
    pub alert_info_bg: String,
    pub alert_info_border: String,
    pub alert_info_text: String,
    
    // Accent color system
    pub accent_primary: String,
    pub accent_secondary: String,
    pub accent_tertiary: String,
    
    // Component-specific accents
    pub post_card_accent: String,
    pub quote_accent: String,
    pub blockquote_accent: String,
    pub card_top_accent: String,
    pub card_side_accent: String,
    
    // Metric card accents
    pub metric_card_posts_accent: String,
    pub metric_card_users_accent: String,
    pub metric_card_comments_accent: String,
    pub metric_card_media_accent: String,
    
    // Login button colors
    pub login_button_bg: String,
    pub login_button_text: String,
    pub login_button_border: String,
    pub login_button_hover_bg: String,
    pub login_button_hover_text: String,
    pub login_button_hover_border: String,
    pub login_button_font_size: String,
    pub login_button_font_weight: String,
    pub login_button_padding: String,
    pub login_button_border_radius: String,
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
            
            // Button system defaults
            button_primary_bg: "#3182ce".to_string(),
            button_primary_text: "#ffffff".to_string(),
            button_primary_hover_bg: "#2c5aa0".to_string(),
            button_primary_border: "#3182ce".to_string(),
            button_primary_hover_border: "#2c5aa0".to_string(),
            
            button_secondary_bg: "transparent".to_string(),
            button_secondary_text: "#3182ce".to_string(),
            button_secondary_hover_bg: "#f0f9ff".to_string(),
            button_secondary_border: "#3182ce".to_string(),
            button_secondary_hover_border: "#2c5aa0".to_string(),
            
            button_outline_bg: "transparent".to_string(),
            button_outline_text: "#3182ce".to_string(),
            button_outline_hover_bg: "#3182ce".to_string(),
            button_outline_hover_text: "#ffffff".to_string(),
            button_outline_border: "#3182ce".to_string(),
            
            button_ghost_bg: "transparent".to_string(),
            button_ghost_text: "#3182ce".to_string(),
            button_ghost_hover_bg: "rgba(49, 130, 206, 0.1)".to_string(),
            button_ghost_border: "transparent".to_string(),
            
            // Card system defaults
            card_bg: "#ffffff".to_string(),
            card_border: "#e2e8f0".to_string(),
            card_hover_shadow: "0 4px 12px rgba(0, 0, 0, 0.15)".to_string(),
            
            // Box system defaults
            box_bg: "#f8fafc".to_string(),
            box_border: "#e2e8f0".to_string(),
            
            // Alert system defaults
            alert_success_bg: "#f0fdf4".to_string(),
            alert_success_border: "#bbf7d0".to_string(),
            alert_success_text: "#166534".to_string(),
            
            alert_warning_bg: "#fffbeb".to_string(),
            alert_warning_border: "#fed7aa".to_string(),
            alert_warning_text: "#92400e".to_string(),
            
            alert_error_bg: "#fef2f2".to_string(),
            alert_error_border: "#fecaca".to_string(),
            alert_error_text: "#991b1b".to_string(),
            
            alert_info_bg: "#f0f9ff".to_string(),
            alert_info_border: "#bae6fd".to_string(),
            alert_info_text: "#1e40af".to_string(),
            
            // Accent color system defaults
            accent_primary: "#667eea".to_string(),
            accent_secondary: "#764ba2".to_string(),
            accent_tertiary: "#f093fb".to_string(),
            
            // Component-specific accent defaults
            post_card_accent: "#667eea".to_string(),
            quote_accent: "#667eea".to_string(),
            blockquote_accent: "#667eea".to_string(),
            card_top_accent: "#667eea".to_string(),
            card_side_accent: "#667eea".to_string(),
            
            // Metric card accent defaults
            metric_card_posts_accent: "#8b5cf6".to_string(),
            metric_card_users_accent: "#10b981".to_string(),
            metric_card_comments_accent: "#f59e0b".to_string(),
            metric_card_media_accent: "#ef4444".to_string(),
            
            // Login button defaults
            login_button_bg: "#ffffff".to_string(),
            login_button_text: "#1a1a1a".to_string(),
            login_button_border: "#e2e8f0".to_string(),
            login_button_hover_bg: "#1a1a1a".to_string(),
            login_button_hover_text: "#ffffff".to_string(),
            login_button_hover_border: "#1a1a1a".to_string(),
            login_button_font_size: "14px".to_string(),
            login_button_font_weight: "500".to_string(),
            login_button_padding: "0.5rem 1rem".to_string(),
            login_button_border_radius: "4px".to_string(),
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
    // Debug log to see what color we're trying to apply
    web_sys::console::log_1(&format!("🎨 Applying post-card accent: {}", scheme.post_card_accent).into());
    
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
                        
                        /* Button system variables */
                        --public-button-primary-bg: {};
                        --public-button-primary-text: {};
                        --public-button-primary-hover-bg: {};
                        --public-button-primary-border: {};
                        --public-button-primary-hover-border: {};
                        
                        --public-button-secondary-bg: {};
                        --public-button-secondary-text: {};
                        --public-button-secondary-hover-bg: {};
                        --public-button-secondary-border: {};
                        --public-button-secondary-hover-border: {};
                        
                        --public-button-outline-bg: {};
                        --public-button-outline-text: {};
                        --public-button-outline-hover-bg: {};
                        --public-button-outline-hover-text: {};
                        --public-button-outline-border: {};
                        
                        --public-button-ghost-bg: {};
                        --public-button-ghost-text: {};
                        --public-button-ghost-hover-bg: {};
                        --public-button-ghost-border: {};
                        
                        /* Card system variables */
                        --public-card-bg: {};
                        --public-card-border: {};
                        --public-card-hover-shadow: {};
                        
                        /* Box system variables */
                        --public-box-bg: {};
                        --public-box-border: {};
                        
                        /* Alert system variables */
                        --public-alert-success-bg: {};
                        --public-alert-success-border: {};
                        --public-alert-success-text: {};
                        
                        --public-alert-warning-bg: {};
                        --public-alert-warning-border: {};
                        --public-alert-warning-text: {};
                        
                        --public-alert-error-bg: {};
                        --public-alert-error-border: {};
                        --public-alert-error-text: {};
                        
                        --public-alert-info-bg: {};
                        --public-alert-info-border: {};
                        --public-alert-info-text: {};
                        
                        /* Accent color system variables */
                        --public-accent-primary: {};
                        --public-accent-secondary: {};
                        --public-accent-tertiary: {};
                        
                        /* Component-specific accent variables */
                        --public-post-card-accent: {};
                        --public-quote-accent: {};
                        --public-blockquote-accent: {};
                        --public-card-top-accent: {};
                        --public-card-side-accent: {};
                        
                        /* Metric card accent variables */
                        --public-metric-card-posts-accent: {};
                        --public-metric-card-users-accent: {};
                        --public-metric-card-comments-accent: {};
                        --public-metric-card-media-accent: {};
                        
                        /* Login button variables */
                        --login-button-bg: {};
                        --login-button-text: {};
                        --login-button-border: {};
                        --login-button-hover-bg: {};
                        --login-button-hover-text: {};
                        --login-button-hover-border: {};
                        --login-button-font-size: {};
                        --login-button-font-weight: {};
                        --login-button-padding: {};
                        --login-button-border-radius: {};
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
                    
                    // Button system variables
                    scheme.button_primary_bg, scheme.button_primary_text, scheme.button_primary_hover_bg,
                    scheme.button_primary_border, scheme.button_primary_hover_border,
                    
                    scheme.button_secondary_bg, scheme.button_secondary_text, scheme.button_secondary_hover_bg,
                    scheme.button_secondary_border, scheme.button_secondary_hover_border,
                    
                    scheme.button_outline_bg, scheme.button_outline_text, scheme.button_outline_hover_bg,
                    scheme.button_outline_hover_text, scheme.button_outline_border,
                    
                    scheme.button_ghost_bg, scheme.button_ghost_text, scheme.button_ghost_hover_bg,
                    scheme.button_ghost_border,
                    
                    // Card system variables
                    scheme.card_bg, scheme.card_border, scheme.card_hover_shadow,
                    
                    // Box system variables
                    scheme.box_bg, scheme.box_border,
                    
                    // Alert system variables
                    scheme.alert_success_bg, scheme.alert_success_border, scheme.alert_success_text,
                    scheme.alert_warning_bg, scheme.alert_warning_border, scheme.alert_warning_text,
                    scheme.alert_error_bg, scheme.alert_error_border, scheme.alert_error_text,
                    scheme.alert_info_bg, scheme.alert_info_border, scheme.alert_info_text,
                    
                    // Accent color system variables
                    scheme.accent_primary, scheme.accent_secondary, scheme.accent_tertiary,
                    
                    // Component-specific accent variables
                    scheme.post_card_accent, scheme.quote_accent, scheme.blockquote_accent,
                    scheme.card_top_accent, scheme.card_side_accent,
                    
                    // Metric card accent variables
                    scheme.metric_card_posts_accent, scheme.metric_card_users_accent,
                    scheme.metric_card_comments_accent, scheme.metric_card_media_accent,
                    
                    // Login button variables
                    scheme.login_button_bg, scheme.login_button_text, scheme.login_button_border,
                    scheme.login_button_hover_bg, scheme.login_button_hover_text, scheme.login_button_hover_border,
                    scheme.login_button_font_size, scheme.login_button_font_weight,
                    scheme.login_button_padding, scheme.login_button_border_radius,
                    
                    // Existing variables for backwards compatibility
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



pub fn design_system_page() -> Html {
    html! { <DesignSystemPage /> }
}

#[function_component(DesignSystemPage)]
fn design_system_page_component() -> Html {
    let admin_scheme = use_state(AdminColorScheme::default);
    let public_scheme = use_state(PublicColorScheme::default);
    let _selected_preset = use_state(|| "Light Preset".to_string());
    let _ = _selected_preset; // Suppress unused warning
    
    // State for saving/loading
    let saving = use_state(|| false);
    let loading = use_state(|| false);
    let save_message = use_state(|| None::<String>);
    let settings_loaded = use_state(|| false);

    // Load saved settings on component mount
    use_effect({
        let public_scheme = public_scheme.clone();
        let admin_scheme = admin_scheme.clone();
        let loading = loading.clone();
        let settings_loaded = settings_loaded.clone();
        
        move || {
            if !*settings_loaded {
                loading.set(true);
                settings_loaded.set(true);
                
                wasm_bindgen_futures::spawn_local(async move {
                    match get_settings(Some("design_system")).await {
                        Ok(settings) => {
                            let mut updated_public_scheme = (*public_scheme).clone();
                            let mut updated_admin_scheme = (*admin_scheme).clone();
                            
                            // Load saved design system settings
                            for setting in &settings {
                                match setting.setting_key.as_str() {
                                    // Accent colors
                                    "public_accent_primary" => updated_public_scheme.accent_primary = setting.setting_value.clone().unwrap_or_default(),
                                    "public_accent_secondary" => updated_public_scheme.accent_secondary = setting.setting_value.clone().unwrap_or_default(),
                                    "public_accent_tertiary" => updated_public_scheme.accent_tertiary = setting.setting_value.clone().unwrap_or_default(),
                                    
                                    // Component accents
                                    "public_post_card_accent" => updated_public_scheme.post_card_accent = setting.setting_value.clone().unwrap_or_default(),
                                    "public_quote_accent" => updated_public_scheme.quote_accent = setting.setting_value.clone().unwrap_or_default(),
                                    "public_blockquote_accent" => updated_public_scheme.blockquote_accent = setting.setting_value.clone().unwrap_or_default(),
                                    "public_card_top_accent" => updated_public_scheme.card_top_accent = setting.setting_value.clone().unwrap_or_default(),
                                    "public_card_side_accent" => updated_public_scheme.card_side_accent = setting.setting_value.clone().unwrap_or_default(),
                                    
                                    // Metric card accents
                                    "public_metric_card_posts_accent" => updated_public_scheme.metric_card_posts_accent = setting.setting_value.clone().unwrap_or_default(),
                                    "public_metric_card_users_accent" => updated_public_scheme.metric_card_users_accent = setting.setting_value.clone().unwrap_or_default(),
                                    "public_metric_card_comments_accent" => updated_public_scheme.metric_card_comments_accent = setting.setting_value.clone().unwrap_or_default(),
                                    "public_metric_card_media_accent" => updated_public_scheme.metric_card_media_accent = setting.setting_value.clone().unwrap_or_default(),
                                    
                                    // Button colors (key ones)
                                    "public_button_primary_bg" => updated_public_scheme.button_primary_bg = setting.setting_value.clone().unwrap_or_default(),
                                    "public_button_primary_text" => updated_public_scheme.button_primary_text = setting.setting_value.clone().unwrap_or_default(),
                                    
                                    // Link colors
                                    "public_link_primary" => updated_public_scheme.link_primary = setting.setting_value.clone().unwrap_or_default(),
                                    "public_link_hover" => updated_public_scheme.link_hover = setting.setting_value.clone().unwrap_or_default(),
                                    
                                    // Status colors
                                    "public_success" => updated_public_scheme.success = setting.setting_value.clone().unwrap_or_default(),
                                    "public_warning" => updated_public_scheme.warning = setting.setting_value.clone().unwrap_or_default(),
                                    "public_danger" => updated_public_scheme.danger = setting.setting_value.clone().unwrap_or_default(),
                                    "public_info" => updated_public_scheme.info = setting.setting_value.clone().unwrap_or_default(),
                                    
                                    // Admin colors
                                    "admin_primary" => updated_admin_scheme.primary = setting.setting_value.clone().unwrap_or_default(),
                                    "admin_secondary" => updated_admin_scheme.secondary = setting.setting_value.clone().unwrap_or_default(),
                                    "admin_success" => updated_admin_scheme.success = setting.setting_value.clone().unwrap_or_default(),
                                    "admin_warning" => updated_admin_scheme.warning = setting.setting_value.clone().unwrap_or_default(),
                                    "admin_danger" => updated_admin_scheme.danger = setting.setting_value.clone().unwrap_or_default(),
                                    "admin_info" => updated_admin_scheme.info = setting.setting_value.clone().unwrap_or_default(),
                                    
                                    _ => {}
                                }
                            }
                            
                            // Update state and apply CSS
                            public_scheme.set(updated_public_scheme.clone());
                            admin_scheme.set(updated_admin_scheme.clone());
                            apply_public_css_variables(&updated_public_scheme);
                            apply_admin_css_variables(&updated_admin_scheme);
                            
                            log::info!("Loaded design system settings");
                        }
                        Err(_) => {
                            // Apply default themes if no settings found
                            apply_admin_css_variables(&admin_scheme);
                            apply_public_css_variables(&public_scheme);
                        }
                    }
                    loading.set(false);
                });
            }
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
                
                // Button system colors
                "button_primary_bg" => scheme.button_primary_bg = value,
                "button_primary_text" => scheme.button_primary_text = value,
                "button_primary_hover_bg" => scheme.button_primary_hover_bg = value,
                "button_primary_border" => scheme.button_primary_border = value,
                "button_primary_hover_border" => scheme.button_primary_hover_border = value,
                
                "button_secondary_bg" => scheme.button_secondary_bg = value,
                "button_secondary_text" => scheme.button_secondary_text = value,
                "button_secondary_hover_bg" => scheme.button_secondary_hover_bg = value,
                "button_secondary_border" => scheme.button_secondary_border = value,
                "button_secondary_hover_border" => scheme.button_secondary_hover_border = value,
                
                "button_outline_bg" => scheme.button_outline_bg = value,
                "button_outline_text" => scheme.button_outline_text = value,
                "button_outline_hover_bg" => scheme.button_outline_hover_bg = value,
                "button_outline_hover_text" => scheme.button_outline_hover_text = value,
                "button_outline_border" => scheme.button_outline_border = value,
                
                "button_ghost_bg" => scheme.button_ghost_bg = value,
                "button_ghost_text" => scheme.button_ghost_text = value,
                "button_ghost_hover_bg" => scheme.button_ghost_hover_bg = value,
                "button_ghost_border" => scheme.button_ghost_border = value,
                
                // Card system colors
                "card_bg" => scheme.card_bg = value,
                "card_border" => scheme.card_border = value,
                "card_hover_shadow" => scheme.card_hover_shadow = value,
                
                // Box system colors
                "box_bg" => scheme.box_bg = value,
                "box_border" => scheme.box_border = value,
                
                // Alert system colors
                "alert_success_bg" => scheme.alert_success_bg = value,
                "alert_success_border" => scheme.alert_success_border = value,
                "alert_success_text" => scheme.alert_success_text = value,
                
                "alert_warning_bg" => scheme.alert_warning_bg = value,
                "alert_warning_border" => scheme.alert_warning_border = value,
                "alert_warning_text" => scheme.alert_warning_text = value,
                
                "alert_error_bg" => scheme.alert_error_bg = value,
                "alert_error_border" => scheme.alert_error_border = value,
                "alert_error_text" => scheme.alert_error_text = value,
                
                "alert_info_bg" => scheme.alert_info_bg = value,
                "alert_info_border" => scheme.alert_info_border = value,
                "alert_info_text" => scheme.alert_info_text = value,
                
                // Accent color system
                "accent_primary" => scheme.accent_primary = value,
                "accent_secondary" => scheme.accent_secondary = value,
                "accent_tertiary" => scheme.accent_tertiary = value,
                
                // Component-specific accents
                "post_card_accent" => scheme.post_card_accent = value,
                "quote_accent" => scheme.quote_accent = value,
                "blockquote_accent" => scheme.blockquote_accent = value,
                "card_top_accent" => scheme.card_top_accent = value,
                "card_side_accent" => scheme.card_side_accent = value,
                
                // Metric card accents
                "metric_card_posts_accent" => scheme.metric_card_posts_accent = value,
                "metric_card_users_accent" => scheme.metric_card_users_accent = value,
                "metric_card_comments_accent" => scheme.metric_card_comments_accent = value,
                "metric_card_media_accent" => scheme.metric_card_media_accent = value,
                
                // Login button colors
                "login_button_bg" => scheme.login_button_bg = value,
                "login_button_text" => scheme.login_button_text = value,
                "login_button_border" => scheme.login_button_border = value,
                "login_button_hover_bg" => scheme.login_button_hover_bg = value,
                "login_button_hover_text" => scheme.login_button_hover_text = value,
                "login_button_hover_border" => scheme.login_button_hover_border = value,
                "login_button_font_size" => scheme.login_button_font_size = value,
                "login_button_font_weight" => scheme.login_button_font_weight = value,
                "login_button_padding" => scheme.login_button_padding = value,
                "login_button_border_radius" => scheme.login_button_border_radius = value,
                
                _ => {}
            }
            public_scheme.set(scheme.clone());
            apply_public_css_variables(&scheme);
        })
    };

    // Save design system settings
    let save_design_system = {
        let public_scheme = public_scheme.clone();
        let admin_scheme = admin_scheme.clone();
        let saving = saving.clone();
        let save_message = save_message.clone();
        
        Callback::from(move |_: web_sys::MouseEvent| {
            let public_scheme = (*public_scheme).clone();
            let admin_scheme = (*admin_scheme).clone();
            let saving = saving.clone();
            let save_message = save_message.clone();
            
            saving.set(true);
            save_message.set(None);
            
            wasm_bindgen_futures::spawn_local(async move {
                // Convert color schemes to settings data
                let mut settings_data = vec![
                    // Public accent colors
                    SettingData {
                        key: "public_accent_primary".to_string(),
                        value: public_scheme.accent_primary,
                        setting_type: "design_system".to_string(),
                        description: Some("Primary accent color for interface elements".to_string()),
                    },
                    SettingData {
                        key: "public_accent_secondary".to_string(),
                        value: public_scheme.accent_secondary,
                        setting_type: "design_system".to_string(),
                        description: Some("Secondary accent color for interface elements".to_string()),
                    },
                    SettingData {
                        key: "public_accent_tertiary".to_string(),
                        value: public_scheme.accent_tertiary,
                        setting_type: "design_system".to_string(),
                        description: Some("Tertiary accent color for interface elements".to_string()),
                    },
                    
                    // Component accents
                    SettingData {
                        key: "public_post_card_accent".to_string(),
                        value: public_scheme.post_card_accent,
                        setting_type: "design_system".to_string(),
                        description: Some("Accent color for post card animations".to_string()),
                    },
                    SettingData {
                        key: "public_quote_accent".to_string(),
                        value: public_scheme.quote_accent,
                        setting_type: "design_system".to_string(),
                        description: Some("Accent color for quote components".to_string()),
                    },
                    SettingData {
                        key: "public_blockquote_accent".to_string(),
                        value: public_scheme.blockquote_accent,
                        setting_type: "design_system".to_string(),
                        description: Some("Accent color for blockquote side bars".to_string()),
                    },
                    SettingData {
                        key: "public_card_top_accent".to_string(),
                        value: public_scheme.card_top_accent,
                        setting_type: "design_system".to_string(),
                        description: Some("Accent color for card top bars".to_string()),
                    },
                    SettingData {
                        key: "public_card_side_accent".to_string(),
                        value: public_scheme.card_side_accent,
                        setting_type: "design_system".to_string(),
                        description: Some("Accent color for card side bars".to_string()),
                    },
                    
                    // Metric card accents
                    SettingData {
                        key: "public_metric_card_posts_accent".to_string(),
                        value: public_scheme.metric_card_posts_accent,
                        setting_type: "design_system".to_string(),
                        description: Some("Accent color for posts metric cards".to_string()),
                    },
                    SettingData {
                        key: "public_metric_card_users_accent".to_string(),
                        value: public_scheme.metric_card_users_accent,
                        setting_type: "design_system".to_string(),
                        description: Some("Accent color for users metric cards".to_string()),
                    },
                    SettingData {
                        key: "public_metric_card_comments_accent".to_string(),
                        value: public_scheme.metric_card_comments_accent,
                        setting_type: "design_system".to_string(),
                        description: Some("Accent color for comments metric cards".to_string()),
                    },
                    SettingData {
                        key: "public_metric_card_media_accent".to_string(),
                        value: public_scheme.metric_card_media_accent,
                        setting_type: "design_system".to_string(),
                        description: Some("Accent color for media metric cards".to_string()),
                    },
                    
                    // Button colors
                    SettingData {
                        key: "public_button_primary_bg".to_string(),
                        value: public_scheme.button_primary_bg,
                        setting_type: "design_system".to_string(),
                        description: Some("Primary button background color".to_string()),
                    },
                    SettingData {
                        key: "public_button_primary_text".to_string(),
                        value: public_scheme.button_primary_text,
                        setting_type: "design_system".to_string(),
                        description: Some("Primary button text color".to_string()),
                    },
                    
                    // Link colors
                    SettingData {
                        key: "public_link_primary".to_string(),
                        value: public_scheme.link_primary,
                        setting_type: "design_system".to_string(),
                        description: Some("Primary link color".to_string()),
                    },
                    SettingData {
                        key: "public_link_hover".to_string(),
                        value: public_scheme.link_hover,
                        setting_type: "design_system".to_string(),
                        description: Some("Link hover color".to_string()),
                    },
                    
                    // Status colors
                    SettingData {
                        key: "public_success".to_string(),
                        value: public_scheme.success,
                        setting_type: "design_system".to_string(),
                        description: Some("Success color".to_string()),
                    },
                    SettingData {
                        key: "public_warning".to_string(),
                        value: public_scheme.warning,
                        setting_type: "design_system".to_string(),
                        description: Some("Warning color".to_string()),
                    },
                    SettingData {
                        key: "public_danger".to_string(),
                        value: public_scheme.danger,
                        setting_type: "design_system".to_string(),
                        description: Some("Danger color".to_string()),
                    },
                    SettingData {
                        key: "public_info".to_string(),
                        value: public_scheme.info,
                        setting_type: "design_system".to_string(),
                        description: Some("Info color".to_string()),
                    },
                    
                    // Admin colors
                    SettingData {
                        key: "admin_primary".to_string(),
                        value: admin_scheme.primary,
                        setting_type: "design_system".to_string(),
                        description: Some("Admin primary color".to_string()),
                    },
                    SettingData {
                        key: "admin_secondary".to_string(),
                        value: admin_scheme.secondary,
                        setting_type: "design_system".to_string(),
                        description: Some("Admin secondary color".to_string()),
                    },
                    SettingData {
                        key: "admin_success".to_string(),
                        value: admin_scheme.success,
                        setting_type: "design_system".to_string(),
                        description: Some("Admin success color".to_string()),
                    },
                    SettingData {
                        key: "admin_warning".to_string(),
                        value: admin_scheme.warning,
                        setting_type: "design_system".to_string(),
                        description: Some("Admin warning color".to_string()),
                    },
                    SettingData {
                        key: "admin_danger".to_string(),
                        value: admin_scheme.danger,
                        setting_type: "design_system".to_string(),
                        description: Some("Admin danger color".to_string()),
                    },
                    SettingData {
                        key: "admin_info".to_string(),
                        value: admin_scheme.info,
                        setting_type: "design_system".to_string(),
                        description: Some("Admin info color".to_string()),
                    },
                    
                    // Login button settings
                    SettingData {
                        key: "public_login_button_bg".to_string(),
                        value: public_scheme.login_button_bg,
                        setting_type: "design_system".to_string(),
                        description: Some("Login button background color".to_string()),
                    },
                    SettingData {
                        key: "public_login_button_text".to_string(),
                        value: public_scheme.login_button_text,
                        setting_type: "design_system".to_string(),
                        description: Some("Login button text color".to_string()),
                    },
                    SettingData {
                        key: "public_login_button_border".to_string(),
                        value: public_scheme.login_button_border,
                        setting_type: "design_system".to_string(),
                        description: Some("Login button border color".to_string()),
                    },
                    SettingData {
                        key: "public_login_button_hover_bg".to_string(),
                        value: public_scheme.login_button_hover_bg,
                        setting_type: "design_system".to_string(),
                        description: Some("Login button hover background color".to_string()),
                    },
                    SettingData {
                        key: "public_login_button_hover_text".to_string(),
                        value: public_scheme.login_button_hover_text,
                        setting_type: "design_system".to_string(),
                        description: Some("Login button hover text color".to_string()),
                    },
                    SettingData {
                        key: "public_login_button_hover_border".to_string(),
                        value: public_scheme.login_button_hover_border,
                        setting_type: "design_system".to_string(),
                        description: Some("Login button hover border color".to_string()),
                    },
                    SettingData {
                        key: "public_login_button_font_size".to_string(),
                        value: public_scheme.login_button_font_size,
                        setting_type: "design_system".to_string(),
                        description: Some("Login button font size".to_string()),
                    },
                    SettingData {
                        key: "public_login_button_font_weight".to_string(),
                        value: public_scheme.login_button_font_weight,
                        setting_type: "design_system".to_string(),
                        description: Some("Login button font weight".to_string()),
                    },
                    SettingData {
                        key: "public_login_button_padding".to_string(),
                        value: public_scheme.login_button_padding,
                        setting_type: "design_system".to_string(),
                        description: Some("Login button padding".to_string()),
                    },
                    SettingData {
                        key: "public_login_button_border_radius".to_string(),
                        value: public_scheme.login_button_border_radius,
                        setting_type: "design_system".to_string(),
                        description: Some("Login button border radius".to_string()),
                    },
                ];
                
                match update_settings(settings_data).await {
                    Ok(_) => {
                        saving.set(false);
                        save_message.set(Some("Design system saved successfully!".to_string()));
                        log::info!("Design system settings saved successfully");
                    }
                    Err(e) => {
                        saving.set(false);
                        save_message.set(Some(format!("Error saving design system: {}", e)));
                        log::error!("Failed to save design system settings: {}", e);
                    }
                }
                
                // Clear message after 3 seconds
                let save_message = save_message.clone();
                gloo_timers::future::TimeoutFuture::new(3000).await;
                save_message.set(None);
            });
        })
    };

    let render_admin_color_input = |label: String, color_name: String, value: String, callback: Callback<web_sys::Event>| {
        html! {
            <div class="color-input-group" style="margin-bottom: 0.5rem;">
                <label style="display: block; font-size: 0.8rem; margin-bottom: 0.25rem;">{label}</label>
                <input 
                    type="color" 
                    value={value} 
                    data-color={color_name}
                    onchange={callback}
                    style="width: 100%; height: 2rem; border: 1px solid #ccc; border-radius: 4px; cursor: pointer;"
                />
            </div>
        }
    };

    let render_public_color_input = |label: String, color_name: String, value: String, callback: Callback<web_sys::Event>| {
        html! {
            <div class="color-input-group" style="margin-bottom: 0.5rem;">
                <label style="display: block; font-size: 0.8rem; margin-bottom: 0.25rem;">{label}</label>
                <input 
                    type="color" 
                    value={value} 
                    data-color={color_name}
                    onchange={callback}
                    style="width: 100%; height: 2rem; border: 1px solid #ccc; border-radius: 4px; cursor: pointer;"
                />
            </div>
        }
    };

    let active_tab = use_state(|| "admin".to_string());

    let on_tab_click = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: String| {
            active_tab.set(tab);
        })
    };

    html! {
        <div class="design-system-page">
            <div class="page-header">
                <h1>{"Design System"}</h1>
                <p>{"Customize your site's visual appearance and branding"}</p>
                
                // Save button and status
                <div class="design-system-actions" style="margin-top: 1rem; display: flex; align-items: center; gap: 1rem;">
                    <button 
                        class="btn btn-primary"
                        onclick={save_design_system}
                        disabled={*saving || *loading}
                        style="padding: 0.75rem 1.5rem; font-weight: 600;"
                    >
                        {if *saving { "Saving..." } else { "Save Design System" }}
                    </button>
                    
                    {if *loading {
                        html! { <span style="color: #666; font-size: 0.875rem;">{"Loading saved settings..."}</span> }
                    } else { html! {} }}
                    
                    {if let Some(message) = (*save_message).as_ref() {
                        html! { 
                            <span style={format!("color: {}; font-size: 0.875rem; font-weight: 500;", 
                                if message.contains("Error") { "#ef4444" } else { "#10b981" }
                            )}>
                                {message}
                            </span> 
                        }
                    } else { html! {} }}
                </div>
            </div>

            <div class="design-system-tabs">
                <button 
                    class={if *active_tab == "admin" { "tab-button active" } else { "tab-button" }}
                    onclick={let tab = on_tab_click.clone(); Callback::from(move |_| tab.emit("admin".to_string()))}
                >
                    {"Admin Theme"}
                </button>
                <button 
                    class={if *active_tab == "public" { "tab-button active" } else { "tab-button" }}
                    onclick={let tab = on_tab_click.clone(); Callback::from(move |_| tab.emit("public".to_string()))}
                >
                    {"Public Theme"}
                </button>
                <button 
                    class={if *active_tab == "typography" { "tab-button active" } else { "tab-button" }}
                    onclick={let tab = on_tab_click.clone(); Callback::from(move |_| tab.emit("typography".to_string()))}
                >
                    {"Typography"}
                </button>
            </div>

            <div class="tab-content">
                {match (*active_tab).as_str() {
                    "admin" => html! {
                        <div class="colors-tab">
                            <div class="color-editor-layout">
                                <div class="color-controls">
                                    <div class="color-groups">
                                        <div class="color-group">
                                            <h3>{"Primary Colors"}</h3>
                                            {render_admin_color_input("Primary".to_string(), "primary".to_string(), (*admin_scheme).primary.clone(), on_admin_color_change.clone())}
                                            {render_admin_color_input("Secondary".to_string(), "secondary".to_string(), (*admin_scheme).secondary.clone(), on_admin_color_change.clone())}
                                        </div>
                                        
                                        <div class="color-group">
                                            <h3>{"Status Colors"}</h3>
                                            {render_admin_color_input("Success".to_string(), "success".to_string(), (*admin_scheme).success.clone(), on_admin_color_change.clone())}
                                            {render_admin_color_input("Warning".to_string(), "warning".to_string(), (*admin_scheme).warning.clone(), on_admin_color_change.clone())}
                                            {render_admin_color_input("Danger".to_string(), "danger".to_string(), (*admin_scheme).danger.clone(), on_admin_color_change.clone())}
                                            {render_admin_color_input("Info".to_string(), "info".to_string(), (*admin_scheme).info.clone(), on_admin_color_change.clone())}
                                        </div>
                                        
                                        <div class="color-group">
                                            <h3>{"Layout Colors"}</h3>
                                            {render_admin_color_input("Background".to_string(), "background".to_string(), (*admin_scheme).background.clone(), on_admin_color_change.clone())}
                                            {render_admin_color_input("Surface".to_string(), "surface".to_string(), (*admin_scheme).surface.clone(), on_admin_color_change.clone())}
                                            {render_admin_color_input("Border".to_string(), "border".to_string(), (*admin_scheme).border.clone(), on_admin_color_change.clone())}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    },
                    "public" => html! {
                        <div class="colors-tab">
                            <div class="public-theme-layout">
                                <div class="color-controls">
                                    <div class="color-groups">
                                        <div class="color-group">
                                            <h3>{"Link Colors"}</h3>
                                            {render_public_color_input("Primary Link".to_string(), "link_primary".to_string(), (*public_scheme).link_primary.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Link Hover".to_string(), "link_hover".to_string(), (*public_scheme).link_hover.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Link Visited".to_string(), "link_visited".to_string(), (*public_scheme).link_visited.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Link Active".to_string(), "link_active".to_string(), (*public_scheme).link_active.clone(), on_public_color_change.clone())}
                                        </div>
                                        
                                        <div class="color-group">
                                            <h3>{"Status Colors"}</h3>
                                            {render_public_color_input("Success".to_string(), "success".to_string(), (*public_scheme).success.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Warning".to_string(), "warning".to_string(), (*public_scheme).warning.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Danger".to_string(), "danger".to_string(), (*public_scheme).danger.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Info".to_string(), "info".to_string(), (*public_scheme).info.clone(), on_public_color_change.clone())}
                                        </div>
                                        
                                        <div class="color-group">
                                            <h3>{"Button Colors"}</h3>
                                            <h4 style="font-size: 0.9rem; margin: 0.5rem 0 0.25rem 0; color: #666;">{"Primary Buttons"}</h4>
                                            {render_public_color_input("Background".to_string(), "button_primary_bg".to_string(), (*public_scheme).button_primary_bg.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Text".to_string(), "button_primary_text".to_string(), (*public_scheme).button_primary_text.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Hover Background".to_string(), "button_primary_hover_bg".to_string(), (*public_scheme).button_primary_hover_bg.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Border".to_string(), "button_primary_border".to_string(), (*public_scheme).button_primary_border.clone(), on_public_color_change.clone())}
                                            
                                            <h4 style="font-size: 0.9rem; margin: 0.5rem 0 0.25rem 0; color: #666;">{"Secondary Buttons"}</h4>
                                            {render_public_color_input("Background".to_string(), "button_secondary_bg".to_string(), (*public_scheme).button_secondary_bg.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Text".to_string(), "button_secondary_text".to_string(), (*public_scheme).button_secondary_text.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Hover Background".to_string(), "button_secondary_hover_bg".to_string(), (*public_scheme).button_secondary_hover_bg.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Border".to_string(), "button_secondary_border".to_string(), (*public_scheme).button_secondary_border.clone(), on_public_color_change.clone())}
                                            
                                            <h4 style="font-size: 0.9rem; margin: 0.5rem 0 0.25rem 0; color: #666;">{"Outline Buttons"}</h4>
                                            {render_public_color_input("Text".to_string(), "button_outline_text".to_string(), (*public_scheme).button_outline_text.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Hover Background".to_string(), "button_outline_hover_bg".to_string(), (*public_scheme).button_outline_hover_bg.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Hover Text".to_string(), "button_outline_hover_text".to_string(), (*public_scheme).button_outline_hover_text.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Border".to_string(), "button_outline_border".to_string(), (*public_scheme).button_outline_border.clone(), on_public_color_change.clone())}
                                        </div>
                                        
                                        <div class="color-group">
                                            <h3>{"Card & Layout Colors"}</h3>
                                            <h4 style="font-size: 0.9rem; margin: 0.5rem 0 0.25rem 0; color: #666;">{"Cards"}</h4>
                                            {render_public_color_input("Card Background".to_string(), "card_bg".to_string(), (*public_scheme).card_bg.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Card Border".to_string(), "card_border".to_string(), (*public_scheme).card_border.clone(), on_public_color_change.clone())}
                                            
                                            <h4 style="font-size: 0.9rem; margin: 0.5rem 0 0.25rem 0; color: #666;">{"Boxes"}</h4>
                                            {render_public_color_input("Box Background".to_string(), "box_bg".to_string(), (*public_scheme).box_bg.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Box Border".to_string(), "box_border".to_string(), (*public_scheme).box_border.clone(), on_public_color_change.clone())}
                                        </div>
                                        
                                        <div class="color-group">
                                            <h3>{"Alert Colors"}</h3>
                                            <h4 style="font-size: 0.9rem; margin: 0.5rem 0 0.25rem 0; color: #666;">{"Success Alerts"}</h4>
                                            {render_public_color_input("Background".to_string(), "alert_success_bg".to_string(), (*public_scheme).alert_success_bg.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Border".to_string(), "alert_success_border".to_string(), (*public_scheme).alert_success_border.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Text".to_string(), "alert_success_text".to_string(), (*public_scheme).alert_success_text.clone(), on_public_color_change.clone())}
                                            
                                            <h4 style="font-size: 0.9rem; margin: 0.5rem 0 0.25rem 0; color: #666;">{"Warning Alerts"}</h4>
                                            {render_public_color_input("Background".to_string(), "alert_warning_bg".to_string(), (*public_scheme).alert_warning_bg.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Border".to_string(), "alert_warning_border".to_string(), (*public_scheme).alert_warning_border.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Text".to_string(), "alert_warning_text".to_string(), (*public_scheme).alert_warning_text.clone(), on_public_color_change.clone())}
                                            
                                            <h4 style="font-size: 0.9rem; margin: 0.5rem 0 0.25rem 0; color: #666;">{"Error Alerts"}</h4>
                                            {render_public_color_input("Background".to_string(), "alert_error_bg".to_string(), (*public_scheme).alert_error_bg.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Border".to_string(), "alert_error_border".to_string(), (*public_scheme).alert_error_border.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Text".to_string(), "alert_error_text".to_string(), (*public_scheme).alert_error_text.clone(), on_public_color_change.clone())}
                                            
                                            <h4 style="font-size: 0.9rem; margin: 0.5rem 0 0.25rem 0; color: #666;">{"Info Alerts"}</h4>
                                            {render_public_color_input("Background".to_string(), "alert_info_bg".to_string(), (*public_scheme).alert_info_bg.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Border".to_string(), "alert_info_border".to_string(), (*public_scheme).alert_info_border.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Text".to_string(), "alert_info_text".to_string(), (*public_scheme).alert_info_text.clone(), on_public_color_change.clone())}
                                        </div>
                                        
                                        <div class="color-group">
                                            <h3>{"Accent Colors"}</h3>
                                            <h4 style="font-size: 0.9rem; margin: 0.5rem 0 0.25rem 0; color: #666;">{"Primary Accent System"}</h4>
                                            {render_public_color_input("Primary Accent".to_string(), "accent_primary".to_string(), (*public_scheme).accent_primary.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Secondary Accent".to_string(), "accent_secondary".to_string(), (*public_scheme).accent_secondary.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Tertiary Accent".to_string(), "accent_tertiary".to_string(), (*public_scheme).accent_tertiary.clone(), on_public_color_change.clone())}
                                            
                                            <h4 style="font-size: 0.9rem; margin: 0.5rem 0 0.25rem 0; color: #666;">{"Component Accents"}</h4>
                                            {render_public_color_input("Post Card Accent".to_string(), "post_card_accent".to_string(), (*public_scheme).post_card_accent.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Quote Accent".to_string(), "quote_accent".to_string(), (*public_scheme).quote_accent.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Blockquote Accent".to_string(), "blockquote_accent".to_string(), (*public_scheme).blockquote_accent.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Card Top Accent".to_string(), "card_top_accent".to_string(), (*public_scheme).card_top_accent.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Card Side Accent".to_string(), "card_side_accent".to_string(), (*public_scheme).card_side_accent.clone(), on_public_color_change.clone())}
                                            
                                            <h4 style="font-size: 0.9rem; margin: 0.5rem 0 0.25rem 0; color: #666;">{"Analytics Card Accents"}</h4>
                                            {render_public_color_input("Posts Cards".to_string(), "metric_card_posts_accent".to_string(), (*public_scheme).metric_card_posts_accent.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Users Cards".to_string(), "metric_card_users_accent".to_string(), (*public_scheme).metric_card_users_accent.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Comments Cards".to_string(), "metric_card_comments_accent".to_string(), (*public_scheme).metric_card_comments_accent.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Media Cards".to_string(), "metric_card_media_accent".to_string(), (*public_scheme).metric_card_media_accent.clone(), on_public_color_change.clone())}
                                        </div>
                                    </div>
                                    
                                    // Login Button Section
                                    <div class="color-section">
                                        <h3 style="font-size: 1rem; margin: 0 0 0.5rem 0; color: #333; border-bottom: 1px solid #eee; padding-bottom: 0.25rem;">{"🔐 Login Button"}</h3>
                                        <div class="color-grid">
                                            <h4 style="font-size: 0.9rem; margin: 0.5rem 0 0.25rem 0; color: #666;">{"Normal State"}</h4>
                                            {render_public_color_input("Background".to_string(), "login_button_bg".to_string(), (*public_scheme).login_button_bg.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Text Color".to_string(), "login_button_text".to_string(), (*public_scheme).login_button_text.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Border Color".to_string(), "login_button_border".to_string(), (*public_scheme).login_button_border.clone(), on_public_color_change.clone())}
                                            
                                            <h4 style="font-size: 0.9rem; margin: 0.5rem 0 0.25rem 0; color: #666;">{"Hover State"}</h4>
                                            {render_public_color_input("Hover Background".to_string(), "login_button_hover_bg".to_string(), (*public_scheme).login_button_hover_bg.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Hover Text".to_string(), "login_button_hover_text".to_string(), (*public_scheme).login_button_hover_text.clone(), on_public_color_change.clone())}
                                            {render_public_color_input("Hover Border".to_string(), "login_button_hover_border".to_string(), (*public_scheme).login_button_hover_border.clone(), on_public_color_change.clone())}
                                            
                                            <h4 style="font-size: 0.9rem; margin: 0.5rem 0 0.25rem 0; color: #666;">{"Typography & Spacing"}</h4>
                                            <div class="form-group">
                                                <label>{"Font Size"}</label>
                                                <input 
                                                    type="text" 
                                                    value={(*public_scheme).login_button_font_size.clone()}
                                                    data-color="login_button_font_size"
                                                    oninput={let callback = on_public_color_change.clone(); Callback::from(move |e: InputEvent| {
                                                        let event: Event = e.dyn_into().unwrap();
                                                        callback.emit(event);
                                                    })}
                                                />
                                            </div>
                                            <div class="form-group">
                                                <label>{"Font Weight"}</label>
                                                <input 
                                                    type="text" 
                                                    value={(*public_scheme).login_button_font_weight.clone()}
                                                    data-color="login_button_font_weight"
                                                    oninput={let callback = on_public_color_change.clone(); Callback::from(move |e: InputEvent| {
                                                        let event: Event = e.dyn_into().unwrap();
                                                        callback.emit(event);
                                                    })}
                                                />
                                            </div>
                                            <div class="form-group">
                                                <label>{"Padding"}</label>
                                                <input 
                                                    type="text" 
                                                    value={(*public_scheme).login_button_padding.clone()}
                                                    data-color="login_button_padding"
                                                    oninput={let callback = on_public_color_change.clone(); Callback::from(move |e: InputEvent| {
                                                        let event: Event = e.dyn_into().unwrap();
                                                        callback.emit(event);
                                                    })}
                                                />
                                            </div>
                                            <div class="form-group">
                                                <label>{"Border Radius"}</label>
                                                <input 
                                                    type="text" 
                                                    value={(*public_scheme).login_button_border_radius.clone()}
                                                    data-color="login_button_border_radius"
                                                    oninput={let callback = on_public_color_change.clone(); Callback::from(move |e: InputEvent| {
                                                        let event: Event = e.dyn_into().unwrap();
                                                        callback.emit(event);
                                                    })}
                                                />
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    },
                    "typography" => html! {
                        <div class="typography-tab">
                            <TypographySystem />
                        </div>
                    },
                    _ => html! {}
                }}
            </div>
        </div>
    }
}
