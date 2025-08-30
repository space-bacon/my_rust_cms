use yew::prelude::*;
use crate::services::api_service::{get_settings, update_settings, SettingData};
use web_sys::HtmlSelectElement;
use wasm_bindgen::JsCast;
use std::collections::HashMap;
use gloo_timers;

#[derive(Clone, PartialEq)]
pub struct TypographySettings {
    // Universal font family
    pub font_family: String,
    
    // Paragraph settings
    pub paragraph_size: i32,
    pub paragraph_line_height: f64,
    pub paragraph_weight: i32,
    pub paragraph_spacing: f64,
    
    // Heading settings
    pub h1_size: i32,
    pub h1_line_height: f64,
    pub h1_weight: i32,
    pub h1_spacing: f64,
    
    pub h2_size: i32,
    pub h2_line_height: f64,
    pub h2_weight: i32,
    pub h2_spacing: f64,
    
    pub h3_size: i32,
    pub h3_line_height: f64,
    pub h3_weight: i32,
    pub h3_spacing: f64,
    
    // Navigation settings
    pub nav_size: i32,
    pub nav_line_height: f64,
    pub nav_weight: i32,
    pub nav_spacing: f64,
    
    // Button settings
    pub button_size: i32,
    pub button_line_height: f64,
    pub button_weight: i32,
    pub button_spacing: f64,
    
    // Alert settings
    pub alert_size: i32,
    pub alert_line_height: f64,
    pub alert_weight: i32,
    pub alert_spacing: f64,
}

impl Default for TypographySettings {
    fn default() -> Self {
        Self {
            font_family: "system".to_string(),
            
            // Paragraph defaults
            paragraph_size: 16,
            paragraph_line_height: 1.6,
            paragraph_weight: 400,
            paragraph_spacing: 1.0,
            
            // Heading defaults (proper hierarchy)
            h1_size: 32,
            h1_line_height: 1.2,
            h1_weight: 700,
            h1_spacing: 1.5,
            
            h2_size: 28,
            h2_line_height: 1.3,
            h2_weight: 600,
            h2_spacing: 1.3,
            
            h3_size: 24,
            h3_line_height: 1.4,
            h3_weight: 600,
            h3_spacing: 1.2,
            
            // Navigation defaults
            nav_size: 16,
            nav_line_height: 1.4,
            nav_weight: 500,
            nav_spacing: 0.8,
            
            // Button defaults
            button_size: 16,
            button_line_height: 1.2,
            button_weight: 500,
            button_spacing: 0.5,
            
            // Alert defaults
            alert_size: 14,
            alert_line_height: 1.4,
            alert_weight: 400,
            alert_spacing: 0.8,
        }
    }
}

// Function to get CSS font family string
fn get_font_family_css(font_family: &str) -> String {
    match font_family {
        "system" => "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif".to_string(),
        "inter" => "'Inter', -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
        "roboto" => "'Roboto', -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
        "open-sans" => "'Open Sans', -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
        "lato" => "'Lato', -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
        "montserrat" => "'Montserrat', -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
        "poppins" => "'Poppins', -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
        "nunito" => "'Nunito', -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
        "source-sans-pro" => "'Source Sans Pro', -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
        "ubuntu" => "'Ubuntu', -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
        "raleway" => "'Raleway', -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
        "oswald" => "'Oswald', -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
        "pt-sans" => "'PT Sans', -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
        "merriweather" => "'Merriweather', Georgia, serif".to_string(),
        "playfair-display" => "'Playfair Display', Georgia, serif".to_string(),
        "lora" => "'Lora', Georgia, serif".to_string(),
        "crimson-text" => "'Crimson Text', Georgia, serif".to_string(),
        "eb-garamond" => "'EB Garamond', Georgia, serif".to_string(),
        "jetbrains-mono" => "'JetBrains Mono', 'Fira Code', Consolas, monospace".to_string(),
        "fira-code" => "'Fira Code', Consolas, monospace".to_string(),
        "source-code-pro" => "'Source Code Pro', Consolas, monospace".to_string(),
        "inconsolata" => "'Inconsolata', Consolas, monospace".to_string(),
        "courier-prime" => "'Courier Prime', 'Courier New', monospace".to_string(),
        "press-start-2p" => "'Press Start 2P', monospace".to_string(),
        "pixel-operator" => "'Pixel Operator', monospace".to_string(),
        "pixelify-sans" => "'Pixelify Sans', monospace".to_string(),
        "orbitron" => "'Orbitron', monospace".to_string(),
        "space-grotesk" => "'Space Grotesk', -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
        _ => "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif".to_string(),
    }
}

// Function to load and apply saved typography settings
pub fn load_and_apply_typography_settings() {
    web_sys::console::log_1(&"🚨 GLOBAL TYPOGRAPHY: load_and_apply_typography_settings called!".into());
    wasm_bindgen_futures::spawn_local(async {
        log::info!("🔍 Loading typography settings from database...");
        web_sys::console::log_1(&"🚨 GLOBAL TYPOGRAPHY: Inside async block, about to call get_settings".into());
        web_sys::console::log_1(&"🚨 GLOBAL TYPOGRAPHY: Querying ALL settings to check if typography exists".into());
        match crate::services::api_service::get_settings(None).await {
            Ok(settings) => {
                web_sys::console::log_1(&format!("🚨 GLOBAL TYPOGRAPHY: Database returned {} settings", settings.len()).into());
                log::info!("📥 Found {} typography settings", settings.len());
                
                // Debug: Show what settings we actually got
                let mut typography_count = 0;
                for (i, setting) in settings.iter().enumerate() {
                    if setting.setting_type == "typography" {
                        typography_count += 1;
                        web_sys::console::log_1(&format!("🎯 TYPOGRAPHY Setting {}: key='{}', value={:?}, type='{}'", 
                            typography_count, setting.setting_key, setting.setting_value, 
                            &setting.setting_type).into());
                    } else if i < 5 {
                        // Show first 5 non-typography settings for context
                        web_sys::console::log_1(&format!("🔍 Other Setting {}: key='{}', value={:?}, type='{}'", 
                            i, setting.setting_key, setting.setting_value, 
                            &setting.setting_type).into());
                    }
                }
                web_sys::console::log_1(&format!("🎯 FOUND {} TYPOGRAPHY SETTINGS out of {} total", typography_count, settings.len()).into());
                let mut font_family = "system".to_string();
                let mut font_size = 16;
                let mut line_height = 1.5;
                let mut font_weight = 400;

                // Filter and process only typography settings
                let typography_settings: Vec<_> = settings.into_iter()
                    .filter(|s| s.setting_type == "typography")
                    .collect();
                
                web_sys::console::log_1(&format!("🎯 PROCESSING {} typography settings", typography_settings.len()).into());
                
                // If no typography settings found in database, try localStorage
                if typography_settings.is_empty() {
                    web_sys::console::log_1(&"🔧 FALLBACK: No typography settings in database, checking localStorage".into());
                    if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
                        let mut found_local_storage = false;
                        let mut font_family = "system".to_string();
                        let mut font_size = 16;
                        let mut line_height = 1.5;
                        let mut font_weight = 400;
                        
                        if let Ok(Some(value)) = storage.get_item("typography_font_family") {
                            font_family = value;
                            found_local_storage = true;
                        }
                        if let Ok(Some(value)) = storage.get_item("typography_font_size") {
                            if let Ok(size) = value.parse::<i32>() {
                                font_size = size;
                            }
                        }
                        if let Ok(Some(value)) = storage.get_item("typography_line_height") {
                            if let Ok(height) = value.parse::<f64>() {
                                line_height = height;
                            }
                        }
                        if let Ok(Some(value)) = storage.get_item("typography_font_weight") {
                            if let Ok(weight) = value.parse::<i32>() {
                                font_weight = weight;
                            }
                        }
                        
                        if found_local_storage {
                            web_sys::console::log_1(&format!("✅ FALLBACK: Found typography settings in localStorage: font={}", font_family).into());
                            let local_storage_settings = TypographySettings {
                                font_family: font_family.clone(),
                                paragraph_size: font_size,
                                paragraph_line_height: line_height,
                                paragraph_weight: font_weight,
                                ..TypographySettings::default()
                            };
                            apply_typography_settings(&local_storage_settings);
                            log::info!("🎨 Loaded and applied typography settings from localStorage: {} {}px {}lh {}fw", font_family, font_size, line_height, font_weight);
                            return;
                        }
                    }
                }
                
                for setting in typography_settings {
                    web_sys::console::log_1(&format!("🔧 Processing typography setting: {} = {:?}", setting.setting_key, setting.setting_value).into());
                    match setting.setting_key.as_str() {
                        "typography_font_family" => {
                            if let Some(value) = setting.setting_value {
                                font_family = value.clone();
                                log::info!("✅ Set font family to: {}", value);
                            }
                        },
                        "typography_font_size" => {
                            if let Some(value) = setting.setting_value {
                                if let Ok(size) = value.parse::<i32>() {
                                    font_size = size;
                                }
                            }
                        },
                        "typography_line_height" => {
                            if let Some(value) = setting.setting_value {
                                if let Ok(height) = value.parse::<f64>() {
                                    line_height = height;
                                }
                            }
                        },
                        "typography_font_weight" => {
                            if let Some(value) = setting.setting_value {
                                if let Ok(weight) = value.parse::<i32>() {
                                    font_weight = weight;
                                }
                            }
                        },
                        _ => {}
                    }
                }

                let typography_settings = TypographySettings {
                    font_family: font_family.clone(),
                    paragraph_size: font_size,
                    paragraph_line_height: line_height,
                    paragraph_weight: font_weight,
                    paragraph_spacing: 1.0,
                    ..TypographySettings::default()
                };
                apply_typography_settings(&typography_settings);
                log::info!("🎨 Loaded and applied typography settings: {} {}px {}lh {}fw", font_family, font_size, line_height, font_weight);
            },
            Err(err) => {
                web_sys::console::log_1(&format!("🚨 GLOBAL TYPOGRAPHY: Database error: {:?}", err).into());
                log::error!("❌ Failed to load typography settings: {:?}", err);
                
                // Try localStorage as fallback
                web_sys::console::log_1(&"🔧 FALLBACK: Checking localStorage for typography settings".into());
                if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
                        let mut found_localStorage = false;
                        let mut font_family = "system".to_string();
                        let mut font_size = 16;
                        let mut line_height = 1.5;
                        let mut font_weight = 400;
                        
                        if let Ok(Some(value)) = storage.get_item("typography_font_family") {
                            font_family = value;
                            found_localStorage = true;
                        }
                        if let Ok(Some(value)) = storage.get_item("typography_font_size") {
                            if let Ok(size) = value.parse::<i32>() {
                                font_size = size;
                            }
                        }
                        if let Ok(Some(value)) = storage.get_item("typography_line_height") {
                            if let Ok(height) = value.parse::<f64>() {
                                line_height = height;
                            }
                        }
                        if let Ok(Some(value)) = storage.get_item("typography_font_weight") {
                            if let Ok(weight) = value.parse::<i32>() {
                                font_weight = weight;
                            }
                        }
                        
                        if found_localStorage {
                            web_sys::console::log_1(&format!("✅ FALLBACK: Found typography settings in localStorage: font={}", font_family).into());
                            let localStorage_settings = TypographySettings {
                                font_family,
                                paragraph_size: font_size,
                                paragraph_line_height: line_height,
                                paragraph_weight: font_weight,
                                ..TypographySettings::default()
                            };
                            apply_typography_settings(&localStorage_settings);
                            return;
                        }
                }
                
                // Apply default typography settings
                let default_settings = TypographySettings::default();
                apply_typography_settings(&default_settings);
                log::info!("🎨 Applied default comprehensive typography settings");
            }
        }
    });
}

// Function to apply comprehensive typography settings
fn apply_typography_settings(settings: &TypographySettings) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            // Remove existing typography style element
            if let Some(existing_style) = document.query_selector("#typography-settings").ok().flatten() {
                let _ = existing_style.remove();
            }

            // Create new style element
            if let Ok(style_element) = document.create_element("style") {
                style_element.set_id("typography-settings");
                
                let font_css = get_font_family_css(&settings.font_family);
                let css_content = format!(
                    r#"
                    :root {{
                        --global-font-family: {};
                    }}
                    
                    /* Universal font-family application */
                    * {{
                        font-family: {} !important;
                    }}
                    
                    /* Paragraph typography */
                    p, .paragraph, .text-content, .content, .description, .excerpt, .summary,
                    .post-content, .page-content, .comment-text, .comment-content, .comment-body,
                    .article-content, .blog-content, .main-content, .body-text, .text,
                    .post-excerpt, .page-excerpt, .content-text, .rich-text, .markdown-content,
                    div, span, li, td, th, blockquote, pre, code {{
                        font-size: {}px !important;
                        line-height: {} !important;
                        font-weight: {} !important;
                        margin-bottom: {}em !important;
                    }}
                    
                    /* Heading 1 typography */
                    h1, .h1, .page-title, .main-title, .post-title, .article-title, .entry-title {{
                        font-size: {}px !important;
                        line-height: {} !important;
                        font-weight: {} !important;
                        margin-bottom: {}em !important;
                    }}
                    
                    /* Heading 2 typography */
                    h2, .h2, .section-title, .major-heading, .post-subtitle, .section-heading {{
                        font-size: {}px !important;
                        line-height: {} !important;
                        font-weight: {} !important;
                        margin-bottom: {}em !important;
                    }}
                    
                    /* Heading 3 typography */
                    h3, .h3, .subsection-title, .component-title, .widget-title, .sidebar-title, .section-title {{
                        font-size: {}px !important;
                        line-height: {} !important;
                        font-weight: {} !important;
                        margin-bottom: {}em !important;
                    }}
                    
                    /* Navigation typography */
                    nav, .navigation, .nav-menu, .menu, .menu-item, .nav-item, .nav-link, .menu-link,
                    .navbar, .header-nav, .main-nav, .primary-nav, .secondary-nav, .breadcrumb,
                    .breadcrumb-item, .pagination, .pagination-item, .tab, .tab-item, .mobile-menu,
                    .hamburger-menu, .dropdown-menu, .dropdown-item, .site-nav, .navigation-link {{
                        font-size: {}px !important;
                        line-height: {} !important;
                        font-weight: {} !important;
                        margin-bottom: {}em !important;
                    }}
                    
                    /* Button typography */
                    button, .button, .btn, input[type="submit"], input[type="button"], input[type="reset"],
                    .cta-button, .action-button, .primary-button, .secondary-button, .link-button,
                    .comment-submit, .form-submit, .search-button, .toggle-button, .menu-toggle,
                    .read-more, .load-more, .view-more, .btn-primary, .btn-secondary, .btn-outline {{
                        font-size: {}px !important;
                        line-height: {} !important;
                        font-weight: {} !important;
                        padding: {}em 1.5em !important;
                    }}
                    
                    /* Alert typography */
                    .alert, .notification, .toast, .message, .warning, .error, .success, .info {{
                        font-size: {}px !important;
                        line-height: {} !important;
                        font-weight: {} !important;
                        padding: {}em 1em !important;
                    }}
                    
                    /* Comments typography */
                    .comments, .comment, .comment-item, .comment-list, .comment-thread,
                    .comment-author, .comment-meta, .comment-date, .comment-reply,
                    .comment-form, .comment-input, .comment-textarea, .reply-form,
                    .reply-content, .comments-section, .comments-header, .comments-count {{
                        font-size: {}px !important;
                        line-height: {} !important;
                        font-weight: {} !important;
                    }}
                    
                    /* Form elements typography */
                    input, textarea, select, label, .form-control, .input, .textarea,
                    .form-group, .form-field, .field-label, .field-input, .search-input,
                    .contact-form, .subscription-form, .newsletter-form {{
                        font-size: {}px !important;
                        line-height: {} !important;
                        font-weight: {} !important;
                    }}
                    
                    /* Links typography */
                    a, .link, .text-link, .content-link, .post-link, .page-link,
                    .category-link, .tag-link, .archive-link {{
                        font-size: inherit !important;
                        line-height: inherit !important;
                        font-weight: inherit !important;
                    }}
                    
                    /* Post/Page meta information */
                    .post-meta, .page-meta, .entry-meta, .article-meta, .meta-info,
                    .post-date, .post-author, .post-category, .post-tags, .read-time {{
                        font-size: {}px !important;
                        line-height: {} !important;
                        font-weight: {} !important;
                    }}
                    "#,
                    font_css, font_css,
                    
                    // Paragraph
                    settings.paragraph_size, settings.paragraph_line_height, settings.paragraph_weight, settings.paragraph_spacing,
                    
                    // H1
                    settings.h1_size, settings.h1_line_height, settings.h1_weight, settings.h1_spacing,
                    
                    // H2
                    settings.h2_size, settings.h2_line_height, settings.h2_weight, settings.h2_spacing,
                    
                    // H3
                    settings.h3_size, settings.h3_line_height, settings.h3_weight, settings.h3_spacing,
                    
                    // Navigation
                    settings.nav_size, settings.nav_line_height, settings.nav_weight, settings.nav_spacing,
                    
                    // Button
                    settings.button_size, settings.button_line_height, settings.button_weight, settings.button_spacing,
                    
                    // Alert
                    settings.alert_size, settings.alert_line_height, settings.alert_weight, settings.alert_spacing,
                    
                    // Comments (use paragraph settings)
                    settings.paragraph_size, settings.paragraph_line_height, settings.paragraph_weight,
                    
                    // Form elements (use paragraph settings)
                    settings.paragraph_size, settings.paragraph_line_height, settings.paragraph_weight,
                    
                    // Post/Page meta (use smaller size)
                    (settings.paragraph_size as f32 * 0.875) as i32, settings.paragraph_line_height, settings.paragraph_weight
                );
                
                style_element.set_text_content(Some(&css_content));
                
                if let Some(head) = document.head() {
                    let _ = head.append_child(&style_element);
                }
                
                log::info!("Applied comprehensive typography settings with {} font family", settings.font_family);
            }
        }
    }
}

#[function_component(TypographySystem)]
pub fn typography_system() -> Html {
    let current_font_family = use_state(|| "system".to_string());
    let current_font_size = use_state(|| 16);
    let current_line_height = use_state(|| 1.6);
    let current_font_weight = use_state(|| 400);
    
    // Additional comprehensive settings
    let h1_size = use_state(|| 32);
    let h1_line_height = use_state(|| 1.2);
    let h1_weight = use_state(|| 700);
    let h1_spacing = use_state(|| 1.5);
    
    let h2_size = use_state(|| 28);
    let h2_line_height = use_state(|| 1.3);
    let h2_weight = use_state(|| 600);
    let h2_spacing = use_state(|| 1.3);
    
    let h3_size = use_state(|| 24);
    let h3_line_height = use_state(|| 1.4);
    let h3_weight = use_state(|| 600);
    let h3_spacing = use_state(|| 1.2);
    
    let nav_size = use_state(|| 16);
    let nav_line_height = use_state(|| 1.4);
    let nav_weight = use_state(|| 500);
    let nav_spacing = use_state(|| 0.8);
    
    let button_size = use_state(|| 16);
    let button_line_height = use_state(|| 1.2);
    let button_weight = use_state(|| 500);
    let button_spacing = use_state(|| 0.5);
    
    let alert_size = use_state(|| 14);
    let alert_line_height = use_state(|| 1.4);
    let alert_weight = use_state(|| 400);
    let alert_spacing = use_state(|| 0.8);
    
    let paragraph_spacing = use_state(|| 1.0);
    
    // UI state for notifications and saving
    let is_saving = use_state(|| false);
    let save_message = use_state(|| None::<String>);
    let has_unsaved_changes = use_state(|| false);

    // Load typography settings from database on mount
    {
        let current_font_family = current_font_family.clone();
        let current_font_size = current_font_size.clone();
        let current_line_height = current_line_height.clone();
        let current_font_weight = current_font_weight.clone();
        let paragraph_spacing = paragraph_spacing.clone();
        let h1_size = h1_size.clone();
        let h1_line_height = h1_line_height.clone();
        let h1_weight = h1_weight.clone();
        let h1_spacing = h1_spacing.clone();
        let h2_size = h2_size.clone();
        let h2_line_height = h2_line_height.clone();
        let h2_weight = h2_weight.clone();
        let h2_spacing = h2_spacing.clone();
        let h3_size = h3_size.clone();
        let h3_line_height = h3_line_height.clone();
        let h3_weight = h3_weight.clone();
        let h3_spacing = h3_spacing.clone();
        let nav_size = nav_size.clone();
        let nav_line_height = nav_line_height.clone();
        let nav_weight = nav_weight.clone();
        let nav_spacing = nav_spacing.clone();
        let button_size = button_size.clone();
        let button_line_height = button_line_height.clone();
        let button_weight = button_weight.clone();
        let button_spacing = button_spacing.clone();
        let alert_size = alert_size.clone();
        let alert_line_height = alert_line_height.clone();
        let alert_weight = alert_weight.clone();
        let alert_spacing = alert_spacing.clone();
        
        use_effect_with_deps(move |_| {
            web_sys::console::log_1(&"🔍 Typography Component: use_effect_with_deps triggered".into());
            // Add a small delay to ensure DOM is ready
            let current_font_family = current_font_family.clone();
            let current_font_size = current_font_size.clone();
            let current_line_height = current_line_height.clone();
            let current_font_weight = current_font_weight.clone();
            
            gloo_timers::callback::Timeout::new(100, move || {
                web_sys::console::log_1(&"🔍 Typography Component: Timeout callback triggered".into());
                wasm_bindgen_futures::spawn_local(async move {
                    web_sys::console::log_1(&"🔍 Typography Component: About to start async loading".into());
                    log::info!("🔍 Typography Component: Loading settings from database...");
                    match get_settings(None).await {
                        Ok(all_settings) => {
                            web_sys::console::log_1(&format!("🔍 Typography Component: Found {} total settings in database", all_settings.len()).into());
                            
                            // Filter for typography settings
                            let typography_settings: Vec<_> = all_settings.iter()
                                .filter(|setting| &setting.setting_type == "typography")
                                .collect();
                            
                            web_sys::console::log_1(&format!("🔍 Typography Component: Found {} typography settings", typography_settings.len()).into());
                            log::info!("🔍 Typography Component: Found {} typography settings in database", typography_settings.len());
                            let mut font_family = "system".to_string();
                            let mut font_size = 16;
                            let mut line_height = 1.6;
                            let mut font_weight = 400;
                            let mut para_spacing = 1.0;
                            let mut h1_sz = 32;
                            let mut h1_lh = 1.2;
                            let mut h1_wt = 700;
                            let mut h1_sp = 1.5;
                            let mut h2_sz = 28;
                            let mut h2_lh = 1.3;
                            let mut h2_wt = 600;
                            let mut h2_sp = 1.3;
                            let mut h3_sz = 24;
                            let mut h3_lh = 1.4;
                            let mut h3_wt = 600;
                            let mut h3_sp = 1.2;
                            let mut nav_sz = 16;
                            let mut nav_lh = 1.4;
                            let mut nav_wt = 500;
                            let mut nav_sp = 0.8;
                            let mut btn_sz = 16;
                            let mut btn_lh = 1.2;
                            let mut btn_wt = 500;
                            let mut btn_sp = 0.5;
                            let mut alt_sz = 14;
                            let mut alt_lh = 1.4;
                            let mut alt_wt = 400;
                            let mut alt_sp = 0.8;
                            
                            // If no database settings, try localStorage
                            if typography_settings.is_empty() {
                                web_sys::console::log_1(&"🔧 Typography Component: No database settings, checking localStorage".into());
                                if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
                                    if let Ok(Some(value)) = storage.get_item("typography_font_family") {
                                        font_family = value;
                                        web_sys::console::log_1(&format!("✅ Typography Component: Loaded font from localStorage: {}", font_family).into());
                                    }
                                    if let Ok(Some(value)) = storage.get_item("typography_font_size") {
                                        if let Ok(size) = value.parse::<i32>() {
                                            font_size = size;
                                        }
                                    }
                                    if let Ok(Some(value)) = storage.get_item("typography_line_height") {
                                        if let Ok(height) = value.parse::<f64>() {
                                            line_height = height;
                                        }
                                    }
                                    if let Ok(Some(value)) = storage.get_item("typography_font_weight") {
                                        if let Ok(weight) = value.parse::<i32>() {
                                            font_weight = weight;
                                        }
                                    }
                                }
                            }
                            
                            for setting in &typography_settings {
                                log::info!("🔍 Typography Component: Processing setting: {} = {:?}", setting.setting_key, setting.setting_value);
                                match setting.setting_key.as_str() {
                                    "typography_font_family" => {
                                        if let Some(value) = &setting.setting_value {
                                            font_family = value.clone();
                                            log::info!("✅ Typography Component: Set font family to: {}", value);
                                        }
                                    },
                                    "typography_font_size" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(size) = value.parse::<i32>() {
                                                font_size = size;
                                                log::info!("✅ Typography Component: Set font size to: {}px", size);
                                            }
                                        }
                                    },
                                    "typography_line_height" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(height) = value.parse::<f64>() {
                                                line_height = height;
                                                log::info!("✅ Typography Component: Set line height to: {}", height);
                                            }
                                        }
                                    },
                                    "typography_font_weight" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(weight) = value.parse::<i32>() {
                                                font_weight = weight;
                                                log::info!("✅ Typography Component: Set font weight to: {}", weight);
                                            }
                                        }
                                    },
                                    "typography_paragraph_spacing" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(spacing) = value.parse::<f64>() {
                                                para_spacing = spacing;
                                            }
                                        }
                                    },
                                    "typography_h1_size" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(size) = value.parse::<i32>() {
                                                h1_sz = size;
                                            }
                                        }
                                    },
                                    "typography_h1_line_height" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(height) = value.parse::<f64>() {
                                                h1_lh = height;
                                            }
                                        }
                                    },
                                    "typography_h1_weight" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(weight) = value.parse::<i32>() {
                                                h1_wt = weight;
                                            }
                                        }
                                    },
                                    "typography_h1_spacing" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(spacing) = value.parse::<f64>() {
                                                h1_sp = spacing;
                                            }
                                        }
                                    },
                                    "typography_h2_size" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(size) = value.parse::<i32>() {
                                                h2_sz = size;
                                            }
                                        }
                                    },
                                    "typography_h2_line_height" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(height) = value.parse::<f64>() {
                                                h2_lh = height;
                                            }
                                        }
                                    },
                                    "typography_h2_weight" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(weight) = value.parse::<i32>() {
                                                h2_wt = weight;
                                            }
                                        }
                                    },
                                    "typography_h2_spacing" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(spacing) = value.parse::<f64>() {
                                                h2_sp = spacing;
                                            }
                                        }
                                    },
                                    "typography_h3_size" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(size) = value.parse::<i32>() {
                                                h3_sz = size;
                                            }
                                        }
                                    },
                                    "typography_h3_line_height" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(height) = value.parse::<f64>() {
                                                h3_lh = height;
                                            }
                                        }
                                    },
                                    "typography_h3_weight" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(weight) = value.parse::<i32>() {
                                                h3_wt = weight;
                                            }
                                        }
                                    },
                                    "typography_h3_spacing" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(spacing) = value.parse::<f64>() {
                                                h3_sp = spacing;
                                            }
                                        }
                                    },
                                    "typography_nav_size" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(size) = value.parse::<i32>() {
                                                nav_sz = size;
                                            }
                                        }
                                    },
                                    "typography_nav_line_height" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(height) = value.parse::<f64>() {
                                                nav_lh = height;
                                            }
                                        }
                                    },
                                    "typography_nav_weight" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(weight) = value.parse::<i32>() {
                                                nav_wt = weight;
                                            }
                                        }
                                    },
                                    "typography_nav_spacing" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(spacing) = value.parse::<f64>() {
                                                nav_sp = spacing;
                                            }
                                        }
                                    },
                                    "typography_button_size" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(size) = value.parse::<i32>() {
                                                btn_sz = size;
                                            }
                                        }
                                    },
                                    "typography_button_line_height" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(height) = value.parse::<f64>() {
                                                btn_lh = height;
                                            }
                                        }
                                    },
                                    "typography_button_weight" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(weight) = value.parse::<i32>() {
                                                btn_wt = weight;
                                            }
                                        }
                                    },
                                    "typography_button_spacing" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(spacing) = value.parse::<f64>() {
                                                btn_sp = spacing;
                                            }
                                        }
                                    },
                                    "typography_alert_size" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(size) = value.parse::<i32>() {
                                                alt_sz = size;
                                            }
                                        }
                                    },
                                    "typography_alert_line_height" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(height) = value.parse::<f64>() {
                                                alt_lh = height;
                                            }
                                        }
                                    },
                                    "typography_alert_weight" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(weight) = value.parse::<i32>() {
                                                alt_wt = weight;
                                            }
                                        }
                                    },
                                    "typography_alert_spacing" => {
                                        if let Some(value) = &setting.setting_value {
                                            if let Ok(spacing) = value.parse::<f64>() {
                                                alt_sp = spacing;
                                            }
                                        }
                                    },
                                    _ => {}
                                }
                            }
                            
                            // Update component state
                            log::info!("🔄 Typography Component: Updating component state...");
                            web_sys::console::log_1(&format!("🔧 Typography Component: Setting UI state - font: {}, size: {}", font_family, font_size).into());
                            current_font_family.set(font_family.clone());
                            current_font_size.set(font_size);
                            current_line_height.set(line_height);
                            current_font_weight.set(font_weight);
                            paragraph_spacing.set(para_spacing);
                            h1_size.set(h1_sz);
                            h1_line_height.set(h1_lh);
                            h1_weight.set(h1_wt);
                            h1_spacing.set(h1_sp);
                            h2_size.set(h2_sz);
                            h2_line_height.set(h2_lh);
                            h2_weight.set(h2_wt);
                            h2_spacing.set(h2_sp);
                            h3_size.set(h3_sz);
                            h3_line_height.set(h3_lh);
                            h3_weight.set(h3_wt);
                            h3_spacing.set(h3_sp);
                            nav_size.set(nav_sz);
                            nav_line_height.set(nav_lh);
                            nav_weight.set(nav_wt);
                            nav_spacing.set(nav_sp);
                            button_size.set(btn_sz);
                            button_line_height.set(btn_lh);
                            button_weight.set(btn_wt);
                            button_spacing.set(btn_sp);
                            alert_size.set(alt_sz);
                            alert_line_height.set(alt_lh);
                            alert_weight.set(alt_wt);
                            alert_spacing.set(alt_sp);
                            web_sys::console::log_1(&"✅ Typography Component: UI state updated successfully".into());
                            
                            // Apply typography settings to DOM
                            let typography_settings = TypographySettings {
                                font_family: font_family.clone(),
                                paragraph_size: font_size,
                                paragraph_line_height: line_height,
                                paragraph_weight: font_weight,
                                paragraph_spacing: 1.0,
                                ..TypographySettings::default()
                            };
                            apply_typography_settings(&typography_settings);
                            
                            log::info!("✅ Typography Component: Loaded and applied settings: {} {}px {}lh {}fw", font_family, font_size, line_height, font_weight);
                        },
                        Err(err) => {
                            log::error!("❌ Typography Component: Failed to load typography settings: {:?}", err);
                        }
                    }
                });
            }).forget();
            || ()
        }, ());
    }

    // Helper function to apply all typography settings
    let apply_all_typography = {
        let current_font_family = current_font_family.clone();
        let current_font_size = current_font_size.clone();
        let current_line_height = current_line_height.clone();
        let current_font_weight = current_font_weight.clone();
        let paragraph_spacing = paragraph_spacing.clone();
        let h1_size = h1_size.clone();
        let h1_line_height = h1_line_height.clone();
        let h1_weight = h1_weight.clone();
        let h1_spacing = h1_spacing.clone();
        let h2_size = h2_size.clone();
        let h2_line_height = h2_line_height.clone();
        let h2_weight = h2_weight.clone();
        let h2_spacing = h2_spacing.clone();
        let h3_size = h3_size.clone();
        let h3_line_height = h3_line_height.clone();
        let h3_weight = h3_weight.clone();
        let h3_spacing = h3_spacing.clone();
        let nav_size = nav_size.clone();
        let nav_line_height = nav_line_height.clone();
        let nav_weight = nav_weight.clone();
        let nav_spacing = nav_spacing.clone();
        let button_size = button_size.clone();
        let button_line_height = button_line_height.clone();
        let button_weight = button_weight.clone();
        let button_spacing = button_spacing.clone();
        let alert_size = alert_size.clone();
        let alert_line_height = alert_line_height.clone();
        let alert_weight = alert_weight.clone();
        let alert_spacing = alert_spacing.clone();
        
        move || {
            let typography_settings = TypographySettings {
                font_family: (*current_font_family).clone(),
                paragraph_size: *current_font_size,
                paragraph_line_height: *current_line_height,
                paragraph_weight: *current_font_weight,
                paragraph_spacing: *paragraph_spacing,
                h1_size: *h1_size,
                h1_line_height: *h1_line_height,
                h1_weight: *h1_weight,
                h1_spacing: *h1_spacing,
                h2_size: *h2_size,
                h2_line_height: *h2_line_height,
                h2_weight: *h2_weight,
                h2_spacing: *h2_spacing,
                h3_size: *h3_size,
                h3_line_height: *h3_line_height,
                h3_weight: *h3_weight,
                h3_spacing: *h3_spacing,
                nav_size: *nav_size,
                nav_line_height: *nav_line_height,
                nav_weight: *nav_weight,
                nav_spacing: *nav_spacing,
                button_size: *button_size,
                button_line_height: *button_line_height,
                button_weight: *button_weight,
                button_spacing: *button_spacing,
                alert_size: *alert_size,
                alert_line_height: *alert_line_height,
                alert_weight: *alert_weight,
                alert_spacing: *alert_spacing,
            };
            apply_typography_settings(&typography_settings);
        }
    };

    // Event handlers
    let on_font_family_change = {
        let current_font_family = current_font_family.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::Event| {
            let select = event.target().unwrap().dyn_into::<HtmlSelectElement>().unwrap();
            let font_family = select.value();
            log::info!("🎯 Font family changed to: {}", font_family);
            current_font_family.set(font_family.clone());
            has_unsaved_changes.set(true);
            apply_all_typography();
        })
    };

    let on_font_size_change = {
        let current_font_size = current_font_size.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(size) = input.value().parse::<i32>() {
                current_font_size.set(size);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_line_height_change = {
        let current_line_height = current_line_height.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(height) = input.value().parse::<f64>() {
                current_line_height.set(height);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_font_weight_change = {
        let current_font_weight = current_font_weight.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(weight) = input.value().parse::<i32>() {
                current_font_weight.set(weight);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    // Paragraph spacing event handler
    let on_paragraph_spacing_change = {
        let paragraph_spacing = paragraph_spacing.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(spacing) = input.value().parse::<f64>() {
                paragraph_spacing.set(spacing);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    // H1 event handlers
    let on_h1_size_change = {
        let h1_size = h1_size.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(size) = input.value().parse::<i32>() {
                h1_size.set(size);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_h1_line_height_change = {
        let h1_line_height = h1_line_height.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(height) = input.value().parse::<f64>() {
                h1_line_height.set(height);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_h1_weight_change = {
        let h1_weight = h1_weight.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(weight) = input.value().parse::<i32>() {
                h1_weight.set(weight);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_h1_spacing_change = {
        let h1_spacing = h1_spacing.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(spacing) = input.value().parse::<f64>() {
                h1_spacing.set(spacing);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    // H2 event handlers
    let on_h2_size_change = {
        let h2_size = h2_size.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(size) = input.value().parse::<i32>() {
                h2_size.set(size);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_h2_line_height_change = {
        let h2_line_height = h2_line_height.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(height) = input.value().parse::<f64>() {
                h2_line_height.set(height);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_h2_weight_change = {
        let h2_weight = h2_weight.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(weight) = input.value().parse::<i32>() {
                h2_weight.set(weight);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_h2_spacing_change = {
        let h2_spacing = h2_spacing.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(spacing) = input.value().parse::<f64>() {
                h2_spacing.set(spacing);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    // H3 event handlers
    let on_h3_size_change = {
        let h3_size = h3_size.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(size) = input.value().parse::<i32>() {
                h3_size.set(size);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_h3_line_height_change = {
        let h3_line_height = h3_line_height.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(height) = input.value().parse::<f64>() {
                h3_line_height.set(height);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_h3_weight_change = {
        let h3_weight = h3_weight.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(weight) = input.value().parse::<i32>() {
                h3_weight.set(weight);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_h3_spacing_change = {
        let h3_spacing = h3_spacing.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(spacing) = input.value().parse::<f64>() {
                h3_spacing.set(spacing);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    // Navigation event handlers
    let on_nav_size_change = {
        let nav_size = nav_size.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(size) = input.value().parse::<i32>() {
                nav_size.set(size);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_nav_line_height_change = {
        let nav_line_height = nav_line_height.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(height) = input.value().parse::<f64>() {
                nav_line_height.set(height);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_nav_weight_change = {
        let nav_weight = nav_weight.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(weight) = input.value().parse::<i32>() {
                nav_weight.set(weight);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_nav_spacing_change = {
        let nav_spacing = nav_spacing.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(spacing) = input.value().parse::<f64>() {
                nav_spacing.set(spacing);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    // Button event handlers
    let on_button_size_change = {
        let button_size = button_size.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(size) = input.value().parse::<i32>() {
                button_size.set(size);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_button_line_height_change = {
        let button_line_height = button_line_height.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(height) = input.value().parse::<f64>() {
                button_line_height.set(height);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_button_weight_change = {
        let button_weight = button_weight.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(weight) = input.value().parse::<i32>() {
                button_weight.set(weight);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_button_spacing_change = {
        let button_spacing = button_spacing.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(spacing) = input.value().parse::<f64>() {
                button_spacing.set(spacing);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    // Alert event handlers
    let on_alert_size_change = {
        let alert_size = alert_size.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(size) = input.value().parse::<i32>() {
                alert_size.set(size);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_alert_line_height_change = {
        let alert_line_height = alert_line_height.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(height) = input.value().parse::<f64>() {
                alert_line_height.set(height);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_alert_weight_change = {
        let alert_weight = alert_weight.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(weight) = input.value().parse::<i32>() {
                alert_weight.set(weight);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };

    let on_alert_spacing_change = {
        let alert_spacing = alert_spacing.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        let apply_all_typography = apply_all_typography.clone();
        
        Callback::from(move |event: web_sys::InputEvent| {
            let input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            if let Ok(spacing) = input.value().parse::<f64>() {
                alert_spacing.set(spacing);
                has_unsaved_changes.set(true);
                apply_all_typography();
            }
        })
    };
    
    // Save button handler
    let on_save_click = {
        let current_font_family = current_font_family.clone();
        let current_font_size = current_font_size.clone();
        let current_line_height = current_line_height.clone();
        let current_font_weight = current_font_weight.clone();
        let paragraph_spacing = paragraph_spacing.clone();
        let h1_size = h1_size.clone();
        let h1_line_height = h1_line_height.clone();
        let h1_weight = h1_weight.clone();
        let h1_spacing = h1_spacing.clone();
        let h2_size = h2_size.clone();
        let h2_line_height = h2_line_height.clone();
        let h2_weight = h2_weight.clone();
        let h2_spacing = h2_spacing.clone();
        let h3_size = h3_size.clone();
        let h3_line_height = h3_line_height.clone();
        let h3_weight = h3_weight.clone();
        let h3_spacing = h3_spacing.clone();
        let nav_size = nav_size.clone();
        let nav_line_height = nav_line_height.clone();
        let nav_weight = nav_weight.clone();
        let nav_spacing = nav_spacing.clone();
        let button_size = button_size.clone();
        let button_line_height = button_line_height.clone();
        let button_weight = button_weight.clone();
        let button_spacing = button_spacing.clone();
        let alert_size = alert_size.clone();
        let alert_line_height = alert_line_height.clone();
        let alert_weight = alert_weight.clone();
        let alert_spacing = alert_spacing.clone();
        let is_saving = is_saving.clone();
        let save_message = save_message.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        
        Callback::from(move |_| {
            let current_font_family = current_font_family.clone();
            let current_font_size = current_font_size.clone();
            let current_line_height = current_line_height.clone();
            let current_font_weight = current_font_weight.clone();
            let paragraph_spacing = paragraph_spacing.clone();
            let h1_size = h1_size.clone();
            let h1_line_height = h1_line_height.clone();
            let h1_weight = h1_weight.clone();
            let h1_spacing = h1_spacing.clone();
            let h2_size = h2_size.clone();
            let h2_line_height = h2_line_height.clone();
            let h2_weight = h2_weight.clone();
            let h2_spacing = h2_spacing.clone();
            let h3_size = h3_size.clone();
            let h3_line_height = h3_line_height.clone();
            let h3_weight = h3_weight.clone();
            let h3_spacing = h3_spacing.clone();
            let nav_size = nav_size.clone();
            let nav_line_height = nav_line_height.clone();
            let nav_weight = nav_weight.clone();
            let nav_spacing = nav_spacing.clone();
            let button_size = button_size.clone();
            let button_line_height = button_line_height.clone();
            let button_weight = button_weight.clone();
            let button_spacing = button_spacing.clone();
            let alert_size = alert_size.clone();
            let alert_line_height = alert_line_height.clone();
            let alert_weight = alert_weight.clone();
            let alert_spacing = alert_spacing.clone();
            let is_saving = is_saving.clone();
            let save_message = save_message.clone();
            let has_unsaved_changes = has_unsaved_changes.clone();
            
            is_saving.set(true);
            save_message.set(None);
            
            wasm_bindgen_futures::spawn_local(async move {
                log::info!("💾 Saving all typography settings...");
                web_sys::console::log_1(&"🚨 SAVE: About to create settings_data vector".into());
                
                let settings_data = vec![
                    SettingData {
                        key: "typography_font_family".to_string(),
                        value: (*current_font_family).clone(),
                        setting_type: "typography".to_string(),
                        description: Some("Global font family setting".to_string()),
                    },
                    SettingData {
                        key: "typography_font_size".to_string(),
                        value: (*current_font_size).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("Global base font size setting".to_string()),
                    },
                    SettingData {
                        key: "typography_line_height".to_string(),
                        value: (*current_line_height).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("Global line height setting".to_string()),
                    },
                    SettingData {
                        key: "typography_font_weight".to_string(),
                        value: (*current_font_weight).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("Global font weight setting".to_string()),
                    },
                    SettingData {
                        key: "typography_paragraph_spacing".to_string(),
                        value: (*paragraph_spacing).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("Paragraph spacing setting".to_string()),
                    },
                    SettingData {
                        key: "typography_h1_size".to_string(),
                        value: (*h1_size).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("H1 font size setting".to_string()),
                    },
                    SettingData {
                        key: "typography_h1_line_height".to_string(),
                        value: (*h1_line_height).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("H1 line height setting".to_string()),
                    },
                    SettingData {
                        key: "typography_h1_weight".to_string(),
                        value: (*h1_weight).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("H1 font weight setting".to_string()),
                    },
                    SettingData {
                        key: "typography_h1_spacing".to_string(),
                        value: (*h1_spacing).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("H1 spacing setting".to_string()),
                    },
                    SettingData {
                        key: "typography_h2_size".to_string(),
                        value: (*h2_size).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("H2 font size setting".to_string()),
                    },
                    SettingData {
                        key: "typography_h2_line_height".to_string(),
                        value: (*h2_line_height).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("H2 line height setting".to_string()),
                    },
                    SettingData {
                        key: "typography_h2_weight".to_string(),
                        value: (*h2_weight).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("H2 font weight setting".to_string()),
                    },
                    SettingData {
                        key: "typography_h2_spacing".to_string(),
                        value: (*h2_spacing).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("H2 spacing setting".to_string()),
                    },
                    SettingData {
                        key: "typography_h3_size".to_string(),
                        value: (*h3_size).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("H3 font size setting".to_string()),
                    },
                    SettingData {
                        key: "typography_h3_line_height".to_string(),
                        value: (*h3_line_height).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("H3 line height setting".to_string()),
                    },
                    SettingData {
                        key: "typography_h3_weight".to_string(),
                        value: (*h3_weight).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("H3 font weight setting".to_string()),
                    },
                    SettingData {
                        key: "typography_h3_spacing".to_string(),
                        value: (*h3_spacing).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("H3 spacing setting".to_string()),
                    },
                    SettingData {
                        key: "typography_nav_size".to_string(),
                        value: (*nav_size).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("Navigation font size setting".to_string()),
                    },
                    SettingData {
                        key: "typography_nav_line_height".to_string(),
                        value: (*nav_line_height).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("Navigation line height setting".to_string()),
                    },
                    SettingData {
                        key: "typography_nav_weight".to_string(),
                        value: (*nav_weight).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("Navigation font weight setting".to_string()),
                    },
                    SettingData {
                        key: "typography_nav_spacing".to_string(),
                        value: (*nav_spacing).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("Navigation spacing setting".to_string()),
                    },
                    SettingData {
                        key: "typography_button_size".to_string(),
                        value: (*button_size).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("Button font size setting".to_string()),
                    },
                    SettingData {
                        key: "typography_button_line_height".to_string(),
                        value: (*button_line_height).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("Button line height setting".to_string()),
                    },
                    SettingData {
                        key: "typography_button_weight".to_string(),
                        value: (*button_weight).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("Button font weight setting".to_string()),
                    },
                    SettingData {
                        key: "typography_button_spacing".to_string(),
                        value: (*button_spacing).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("Button spacing setting".to_string()),
                    },
                    SettingData {
                        key: "typography_alert_size".to_string(),
                        value: (*alert_size).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("Alert font size setting".to_string()),
                    },
                    SettingData {
                        key: "typography_alert_line_height".to_string(),
                        value: (*alert_line_height).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("Alert line height setting".to_string()),
                    },
                    SettingData {
                        key: "typography_alert_weight".to_string(),
                        value: (*alert_weight).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("Alert font weight setting".to_string()),
                    },
                    SettingData {
                        key: "typography_alert_spacing".to_string(),
                        value: (*alert_spacing).to_string(),
                        setting_type: "typography".to_string(),
                        description: Some("Alert spacing setting".to_string()),
                    },
                ];
                
                web_sys::console::log_1(&format!("🚨 SAVE: Created {} settings to save", settings_data.len()).into());
                for (i, setting) in settings_data.iter().enumerate() {
                    web_sys::console::log_1(&format!("🚨 SAVE Setting {}: key='{}', value='{}', type='{}'", 
                        i, setting.key, setting.value, setting.setting_type).into());
                }
                
                web_sys::console::log_1(&"🚨 SAVE: About to call update_settings API".into());
                match update_settings(settings_data).await {
                    Ok(_) => {
                        web_sys::console::log_1(&"🚨 SAVE: API call succeeded!".into());
                        log::info!("✅ All typography settings saved successfully!");
                        
                        // Immediately verify the save by checking the database
                        web_sys::console::log_1(&"🔍 VERIFICATION: Checking if settings were actually saved...".into());
                        let current_font_family_for_verification = current_font_family.clone();
                        let current_font_size_for_verification = current_font_size.clone();
                        let current_line_height_for_verification = current_line_height.clone();
                        let current_font_weight_for_verification = current_font_weight.clone();
                        
                        wasm_bindgen_futures::spawn_local(async move {
                            match crate::services::api_service::get_settings(None).await {
                                Ok(verify_settings) => {
                                    let typography_count = verify_settings.iter()
                                        .filter(|s| s.setting_type == "typography")
                                        .count();
                                    web_sys::console::log_1(&format!("🔍 VERIFICATION: Found {} typography settings after save", typography_count).into());
                                    
                                    if typography_count == 0 {
                                        web_sys::console::log_1(&"❌ VERIFICATION FAILED: No typography settings found after successful save!".into());
                                        web_sys::console::log_1(&"🔧 WORKAROUND: Saving to localStorage until backend is fixed".into());
                                        
                                        // Temporary localStorage workaround
                                        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
                                        
                                        let _ = storage.set_item("typography_font_family", &(*current_font_family_for_verification));
                                        let _ = storage.set_item("typography_font_size", &(*current_font_size_for_verification).to_string());
                                        let _ = storage.set_item("typography_line_height", &(*current_line_height_for_verification).to_string());
                                        let _ = storage.set_item("typography_font_weight", &(*current_font_weight_for_verification).to_string());
                                        
                                        web_sys::console::log_1(&"✅ WORKAROUND: Typography settings saved to localStorage".into());
                                    } else {
                                        web_sys::console::log_1(&"✅ VERIFICATION PASSED: Typography settings found in database".into());
                                    }
                                },
                                Err(e) => {
                                    web_sys::console::log_1(&format!("❌ VERIFICATION ERROR: {:?}", e).into());
                                }
                            }
                        });
                        
                        is_saving.set(false);
                        has_unsaved_changes.set(false);
                        save_message.set(Some("✅ Typography settings saved successfully!".to_string()));
                        
                        // Clear success message after 3 seconds
                        let save_message_clear = save_message.clone();
                        gloo_timers::callback::Timeout::new(3000, move || {
                            save_message_clear.set(None);
                        }).forget();
                    },
                    Err(e) => {
                        web_sys::console::log_1(&format!("🚨 SAVE: API call failed: {:?}", e).into());
                        log::error!("❌ Failed to save typography settings: {:?}", e);
                        is_saving.set(false);
                        save_message.set(Some("❌ Failed to save settings. Please try again.".to_string()));
                        
                        // Clear error message after 5 seconds
                        let save_message_clear = save_message.clone();
                        gloo_timers::callback::Timeout::new(5000, move || {
                            save_message_clear.set(None);
                        }).forget();
                    }
                }
            });
        })
    };

    // Debug: Log current state when component renders
    web_sys::console::log_1(&format!("🎨 RENDER: Typography component rendering with font_family: {}", *current_font_family).into());
    
    html! {
        <div class="typography-system">
            <div style="background: white; border-radius: 12px; padding: 2rem; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem;">
                    <h2 style="margin: 0; color: #1f2937; font-size: 1.5rem; font-weight: 600;">
                        {"Typography System"}
                    </h2>
                    
                    // Save button and status
                    <div style="display: flex; align-items: center; gap: 1rem;">
                        {
                            if let Some(message) = (*save_message).as_ref() {
                                html! {
                                    <div style={format!(
                                        "padding: 0.5rem 1rem; border-radius: 0.5rem; font-size: 0.875rem; font-weight: 500; {}",
                                        if message.starts_with("✅") {
                                            "background: #dcfce7; color: #166534; border: 1px solid #bbf7d0;"
                                        } else {
                                            "background: #fef2f2; color: #991b1b; border: 1px solid #fecaca;"
                                        }
                                    )}>
                                        {message}
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                        
                        <button 
                            onclick={on_save_click}
                            disabled={!*has_unsaved_changes || *is_saving}
                            style={format!(
                                "padding: 0.75rem 1.5rem; border-radius: 0.5rem; font-weight: 500; border: none; cursor: {}; transition: all 0.2s; {}",
                                if !*has_unsaved_changes || *is_saving { "not-allowed" } else { "pointer" },
                                if !*has_unsaved_changes || *is_saving {
                                    "background: #e5e7eb; color: #9ca3af;"
                                } else {
                                    "background: #3b82f6; color: white; box-shadow: 0 2px 4px rgba(59, 130, 246, 0.3);"
                                }
                            )}
                        >
                            {
                                if *is_saving {
                                    "Saving..."
                                } else if *has_unsaved_changes {
                                    "Save Changes"
                                } else {
                                    "Saved"
                                }
                            }
                        </button>
                    </div>
                </div>
                
                <div style="display: grid; gap: 2rem;">
                    // Font Family
                    <div>
                        <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151;">
                            {format!("Font Family (Current: {})", *current_font_family)}
                        </label>
                        <select 
                            value={(*current_font_family).clone()}
                            onchange={on_font_family_change}
                            style="width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.5rem; font-size: 1rem;"
                        >
                            <option value="system">{"System Default"}</option>
                            <optgroup label="Sans Serif">
                                <option value="inter">{"Inter"}</option>
                                <option value="roboto">{"Roboto"}</option>
                                <option value="open-sans">{"Open Sans"}</option>
                                <option value="lato">{"Lato"}</option>
                                <option value="montserrat">{"Montserrat"}</option>
                                <option value="poppins">{"Poppins"}</option>
                                <option value="nunito">{"Nunito"}</option>
                                <option value="source-sans-pro">{"Source Sans Pro"}</option>
                                <option value="ubuntu">{"Ubuntu"}</option>
                                <option value="raleway">{"Raleway"}</option>
                                <option value="oswald">{"Oswald"}</option>
                                <option value="pt-sans">{"PT Sans"}</option>
                                <option value="space-grotesk">{"Space Grotesk"}</option>
                            </optgroup>
                            <optgroup label="Serif">
                                <option value="merriweather">{"Merriweather"}</option>
                                <option value="playfair-display">{"Playfair Display"}</option>
                                <option value="lora">{"Lora"}</option>
                                <option value="crimson-text">{"Crimson Text"}</option>
                                <option value="eb-garamond">{"EB Garamond"}</option>
                            </optgroup>
                            <optgroup label="Monospace">
                                <option value="jetbrains-mono">{"JetBrains Mono"}</option>
                                <option value="fira-code">{"Fira Code"}</option>
                                <option value="source-code-pro">{"Source Code Pro"}</option>
                                <option value="inconsolata">{"Inconsolata"}</option>
                                <option value="courier-prime">{"Courier Prime"}</option>
                            </optgroup>
                            <optgroup label="Pixel/Gaming">
                                <option value="press-start-2p">{"Press Start 2P"}</option>
                                <option value="pixel-operator">{"Pixel Operator"}</option>
                                <option value="pixelify-sans">{"Pixelify Sans"}</option>
                                <option value="orbitron">{"Orbitron"}</option>
                            </optgroup>
                        </select>
                    </div>

                    // Paragraph Typography Section
                    <div style="background: #fefefe; padding: 1.5rem; border-radius: 0.75rem; border: 2px solid #e5e7eb;">
                        <h3 style="margin: 0 0 1rem 0; color: #1f2937; font-size: 1.25rem; font-weight: 600;">
                            {"📝 Paragraph Typography"}
                        </h3>
                        <p style="margin: 0 0 1rem 0; color: #6b7280; font-size: 0.875rem;">
                            {"Body text, descriptions, and general content"}
                        </p>
                        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;">
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Size: {}px", *current_font_size)}
                                </label>
                                <input 
                                    type="range"
                                    min="10"
                                    max="24"
                                    step="1"
                                    value={(*current_font_size).to_string()}
                                    oninput={on_font_size_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Line Height: {:.1}", *current_line_height)}
                                </label>
                                <input 
                                    type="range"
                                    min="1.0"
                                    max="2.0"
                                    step="0.1"
                                    value={(*current_line_height).to_string()}
                                    oninput={on_line_height_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Weight: {}", *current_font_weight)}
                                </label>
                                <input 
                                    type="range"
                                    min="100"
                                    max="900"
                                    step="100"
                                    value={(*current_font_weight).to_string()}
                                    oninput={on_font_weight_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Spacing: {:.1}em", *paragraph_spacing)}
                                </label>
                                <input 
                                    type="range"
                                    min="0.5"
                                    max="2.0"
                                    step="0.1"
                                    value={(*paragraph_spacing).to_string()}
                                    oninput={on_paragraph_spacing_change}
                                    style="width: 100%;"
                                />
                            </div>
                        </div>
                    </div>

                    // Heading 1 Typography Section
                    <div style="background: #fef7ff; padding: 1.5rem; border-radius: 0.75rem; border: 2px solid #e9d5ff;">
                        <h3 style="margin: 0 0 1rem 0; color: #7c2d12; font-size: 1.25rem; font-weight: 600;">
                            {"🎯 Heading 1 Typography"}
                        </h3>
                        <p style="margin: 0 0 1rem 0; color: #a855f7; font-size: 0.875rem;">
                            {"Main page titles and primary headings"}
                        </p>
                        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;">
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Size: {}px", *h1_size)}
                                </label>
                                <input 
                                    type="range"
                                    min="24"
                                    max="48"
                                    step="2"
                                    value={(*h1_size).to_string()}
                                    oninput={on_h1_size_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Line Height: {:.1}", *h1_line_height)}
                                </label>
                                <input 
                                    type="range"
                                    min="1.0"
                                    max="1.6"
                                    step="0.1"
                                    value={(*h1_line_height).to_string()}
                                    oninput={on_h1_line_height_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Weight: {}", *h1_weight)}
                                </label>
                                <input 
                                    type="range"
                                    min="400"
                                    max="900"
                                    step="100"
                                    value={(*h1_weight).to_string()}
                                    oninput={on_h1_weight_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Spacing: {:.1}em", *h1_spacing)}
                                </label>
                                <input 
                                    type="range"
                                    min="0.5"
                                    max="2.5"
                                    step="0.1"
                                    value={(*h1_spacing).to_string()}
                                    oninput={on_h1_spacing_change}
                                    style="width: 100%;"
                                />
                            </div>
                        </div>
                    </div>

                    // Heading 2 Typography Section
                    <div style="background: #f0f9ff; padding: 1.5rem; border-radius: 0.75rem; border: 2px solid #bae6fd;">
                        <h3 style="margin: 0 0 1rem 0; color: #0c4a6e; font-size: 1.25rem; font-weight: 600;">
                            {"🎯 Heading 2 Typography"}
                        </h3>
                        <p style="margin: 0 0 1rem 0; color: #0284c7; font-size: 0.875rem;">
                            {"Section titles and major headings"}
                        </p>
                        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;">
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Size: {}px", *h2_size)}
                                </label>
                                <input 
                                    type="range"
                                    min="20"
                                    max="36"
                                    step="2"
                                    value={(*h2_size).to_string()}
                                    oninput={on_h2_size_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Line Height: {:.1}", *h2_line_height)}
                                </label>
                                <input 
                                    type="range"
                                    min="1.0"
                                    max="1.6"
                                    step="0.1"
                                    value={(*h2_line_height).to_string()}
                                    oninput={on_h2_line_height_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Weight: {}", *h2_weight)}
                                </label>
                                <input 
                                    type="range"
                                    min="400"
                                    max="900"
                                    step="100"
                                    value={(*h2_weight).to_string()}
                                    oninput={on_h2_weight_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Spacing: {:.1}em", *h2_spacing)}
                                </label>
                                <input 
                                    type="range"
                                    min="0.5"
                                    max="2.5"
                                    step="0.1"
                                    value={(*h2_spacing).to_string()}
                                    oninput={on_h2_spacing_change}
                                    style="width: 100%;"
                                />
                            </div>
                        </div>
                    </div>

                    // Heading 3 Typography Section
                    <div style="background: #f0fdf4; padding: 1.5rem; border-radius: 0.75rem; border: 2px solid #bbf7d0;">
                        <h3 style="margin: 0 0 1rem 0; color: #14532d; font-size: 1.25rem; font-weight: 600;">
                            {"🎯 Heading 3 Typography"}
                        </h3>
                        <p style="margin: 0 0 1rem 0; color: #16a34a; font-size: 0.875rem;">
                            {"Subsection titles and component headings"}
                        </p>
                        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;">
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Size: {}px", *h3_size)}
                                </label>
                                <input 
                                    type="range"
                                    min="16"
                                    max="32"
                                    step="2"
                                    value={(*h3_size).to_string()}
                                    oninput={on_h3_size_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Line Height: {:.1}", *h3_line_height)}
                                </label>
                                <input 
                                    type="range"
                                    min="1.0"
                                    max="1.6"
                                    step="0.1"
                                    value={(*h3_line_height).to_string()}
                                    oninput={on_h3_line_height_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Weight: {}", *h3_weight)}
                                </label>
                                <input 
                                    type="range"
                                    min="400"
                                    max="900"
                                    step="100"
                                    value={(*h3_weight).to_string()}
                                    oninput={on_h3_weight_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Spacing: {:.1}em", *h3_spacing)}
                                </label>
                                <input 
                                    type="range"
                                    min="0.5"
                                    max="2.5"
                                    step="0.1"
                                    value={(*h3_spacing).to_string()}
                                    oninput={on_h3_spacing_change}
                                    style="width: 100%;"
                                />
                            </div>
                        </div>
                    </div>

                    // Navigation Typography Section
                    <div style="background: #fffbeb; padding: 1.5rem; border-radius: 0.75rem; border: 2px solid #fed7aa;">
                        <h3 style="margin: 0 0 1rem 0; color: #9a3412; font-size: 1.25rem; font-weight: 600;">
                            {"🧭 Navigation Typography"}
                        </h3>
                        <p style="margin: 0 0 1rem 0; color: #ea580c; font-size: 0.875rem;">
                            {"Menu items, navigation links, and breadcrumbs"}
                        </p>
                        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;">
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Size: {}px", *nav_size)}
                                </label>
                                <input 
                                    type="range"
                                    min="12"
                                    max="20"
                                    step="1"
                                    value={(*nav_size).to_string()}
                                    oninput={on_nav_size_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Line Height: {:.1}", *nav_line_height)}
                                </label>
                                <input 
                                    type="range"
                                    min="1.0"
                                    max="1.8"
                                    step="0.1"
                                    value={(*nav_line_height).to_string()}
                                    oninput={on_nav_line_height_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Weight: {}", *nav_weight)}
                                </label>
                                <input 
                                    type="range"
                                    min="300"
                                    max="700"
                                    step="100"
                                    value={(*nav_weight).to_string()}
                                    oninput={on_nav_weight_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Spacing: {:.1}em", *nav_spacing)}
                                </label>
                                <input 
                                    type="range"
                                    min="0.2"
                                    max="1.5"
                                    step="0.1"
                                    value={(*nav_spacing).to_string()}
                                    oninput={on_nav_spacing_change}
                                    style="width: 100%;"
                                />
                            </div>
                        </div>
                    </div>

                    // Button Typography Section
                    <div style="background: #f3e8ff; padding: 1.5rem; border-radius: 0.75rem; border: 2px solid #c4b5fd;">
                        <h3 style="margin: 0 0 1rem 0; color: #581c87; font-size: 1.25rem; font-weight: 600;">
                            {"🔘 Button Typography"}
                        </h3>
                        <p style="margin: 0 0 1rem 0; color: #7c3aed; font-size: 0.875rem;">
                            {"Buttons, CTAs, and interactive elements"}
                        </p>
                        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;">
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Size: {}px", *button_size)}
                                </label>
                                <input 
                                    type="range"
                                    min="12"
                                    max="20"
                                    step="1"
                                    value={(*button_size).to_string()}
                                    oninput={on_button_size_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Line Height: {:.1}", *button_line_height)}
                                </label>
                                <input 
                                    type="range"
                                    min="1.0"
                                    max="1.6"
                                    step="0.1"
                                    value={(*button_line_height).to_string()}
                                    oninput={on_button_line_height_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Weight: {}", *button_weight)}
                                </label>
                                <input 
                                    type="range"
                                    min="400"
                                    max="700"
                                    step="100"
                                    value={(*button_weight).to_string()}
                                    oninput={on_button_weight_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Spacing: {:.1}em", *button_spacing)}
                                </label>
                                <input 
                                    type="range"
                                    min="0.2"
                                    max="1.0"
                                    step="0.1"
                                    value={(*button_spacing).to_string()}
                                    oninput={on_button_spacing_change}
                                    style="width: 100%;"
                                />
                            </div>
                        </div>
                    </div>

                    // Alert Typography Section
                    <div style="background: #fef2f2; padding: 1.5rem; border-radius: 0.75rem; border: 2px solid #fecaca;">
                        <h3 style="margin: 0 0 1rem 0; color: #991b1b; font-size: 1.25rem; font-weight: 600;">
                            {"⚠️ Alert Typography"}
                        </h3>
                        <p style="margin: 0 0 1rem 0; color: #dc2626; font-size: 0.875rem;">
                            {"Notifications, warnings, and status messages"}
                        </p>
                        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;">
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Size: {}px", *alert_size)}
                                </label>
                                <input 
                                    type="range"
                                    min="10"
                                    max="18"
                                    step="1"
                                    value={(*alert_size).to_string()}
                                    oninput={on_alert_size_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Line Height: {:.1}", *alert_line_height)}
                                </label>
                                <input 
                                    type="range"
                                    min="1.0"
                                    max="1.8"
                                    step="0.1"
                                    value={(*alert_line_height).to_string()}
                                    oninput={on_alert_line_height_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Weight: {}", *alert_weight)}
                                </label>
                                <input 
                                    type="range"
                                    min="300"
                                    max="600"
                                    step="100"
                                    value={(*alert_weight).to_string()}
                                    oninput={on_alert_weight_change}
                                    style="width: 100%;"
                                />
                            </div>
                            <div>
                                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151; font-size: 0.875rem;">
                                    {format!("Spacing: {:.1}em", *alert_spacing)}
                                </label>
                                <input 
                                    type="range"
                                    min="0.2"
                                    max="1.5"
                                    step="0.1"
                                    value={(*alert_spacing).to_string()}
                                    oninput={on_alert_spacing_change}
                                    style="width: 100%;"
                                />
                            </div>
                        </div>
                    </div>

                    // Comprehensive Preview Section
                    <div style="margin-top: 2rem; padding: 2rem; background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%); border-radius: 1rem; border: 2px solid #e2e8f0;">
                        <h3 style="margin: 0 0 1.5rem 0; color: #1e293b; font-size: 1.5rem; font-weight: 700; text-align: center;">
                            {"🎨 Comprehensive Typography Preview"}
                        </h3>
                        
                        <div style="display: grid; gap: 2rem;">
                            // Heading Samples
                            <div style="background: white; padding: 1.5rem; border-radius: 0.75rem; border: 1px solid #e2e8f0;">
                                <h4 style="margin: 0 0 1rem 0; color: #64748b; font-size: 0.875rem; font-weight: 500; text-transform: uppercase; letter-spacing: 0.05em;">{"Heading Hierarchy"}</h4>
                                <h1 style="margin: 0 0 0.5rem 0;">{"Heading 1 - Main Page Title"}</h1>
                                <h2 style="margin: 0 0 0.5rem 0;">{"Heading 2 - Section Title"}</h2>
                                <h3 style="margin: 0 0 0.5rem 0;">{"Heading 3 - Subsection Title"}</h3>
                            </div>
                            
                            // Body Text Sample
                            <div style="background: white; padding: 1.5rem; border-radius: 0.75rem; border: 1px solid #e2e8f0;">
                                <h4 style="margin: 0 0 1rem 0; color: #64748b; font-size: 0.875rem; font-weight: 500; text-transform: uppercase; letter-spacing: 0.05em;">{"Body Content"}</h4>
                                <p style="margin: 0 0 1rem 0;">
                                    {"This is a comprehensive paragraph that demonstrates how your typography settings will appear throughout your website. "}
                                    <strong>{"This text is bold"}</strong>{" and "}
                                    <em>{"this text is italic"}</em>{". The spacing, line height, and font weight all contribute to readability and visual hierarchy."}
                                </p>
                                <ul style="margin: 0; padding-left: 1.5rem;">
                                    <li>{"First list item with proper spacing"}</li>
                                    <li>{"Second list item showing consistency"}</li>
                                    <li>{"Third list item demonstrating flow"}</li>
                                </ul>
                            </div>
                            
                            // Navigation Sample
                            <div style="background: white; padding: 1.5rem; border-radius: 0.75rem; border: 1px solid #e2e8f0;">
                                <h4 style="margin: 0 0 1rem 0; color: #64748b; font-size: 0.875rem; font-weight: 500; text-transform: uppercase; letter-spacing: 0.05em;">{"Navigation Elements"}</h4>
                                <nav style="display: flex; gap: 2rem; flex-wrap: wrap;">
                                    <a href="#" style="text-decoration: none; color: #3b82f6;">{"Home"}</a>
                                    <a href="#" style="text-decoration: none; color: #3b82f6;">{"About"}</a>
                                    <a href="#" style="text-decoration: none; color: #3b82f6;">{"Services"}</a>
                                    <a href="#" style="text-decoration: none; color: #3b82f6;">{"Contact"}</a>
                                </nav>
                            </div>
                            
                            // Interactive Elements Sample
                            <div style="background: white; padding: 1.5rem; border-radius: 0.75rem; border: 1px solid #e2e8f0;">
                                <h4 style="margin: 0 0 1rem 0; color: #64748b; font-size: 0.875rem; font-weight: 500; text-transform: uppercase; letter-spacing: 0.05em;">{"Interactive Elements"}</h4>
                                <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                                    <button style="padding: 0.75rem 1.5rem; background: #3b82f6; color: white; border: none; border-radius: 0.5rem; cursor: pointer;">
                                        {"Primary Button"}
                                    </button>
                                    <button style="padding: 0.75rem 1.5rem; background: transparent; color: #3b82f6; border: 2px solid #3b82f6; border-radius: 0.5rem; cursor: pointer;">
                                        {"Secondary Button"}
                                    </button>
                                </div>
                            </div>
                            
                            // Alert Sample
                            <div style="background: white; padding: 1.5rem; border-radius: 0.75rem; border: 1px solid #e2e8f0;">
                                <h4 style="margin: 0 0 1rem 0; color: #64748b; font-size: 0.875rem; font-weight: 500; text-transform: uppercase; letter-spacing: 0.05em;">{"Alert Messages"}</h4>
                                <div style="display: grid; gap: 0.75rem;">
                                    <div class="alert" style="padding: 0.75rem 1rem; background: #dbeafe; color: #1e40af; border-radius: 0.5rem; border-left: 4px solid #3b82f6;">
                                        {"This is an informational alert message with proper typography."}
                                    </div>
                                    <div class="alert" style="padding: 0.75rem 1rem; background: #dcfce7; color: #166534; border-radius: 0.5rem; border-left: 4px solid #22c55e;">
                                        {"This is a success alert message showing consistent styling."}
                                    </div>
                                    <div class="alert" style="padding: 0.75rem 1rem; background: #fef2f2; color: #991b1b; border-radius: 0.5rem; border-left: 4px solid #ef4444;">
                                        {"This is an error alert message with appropriate typography."}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}