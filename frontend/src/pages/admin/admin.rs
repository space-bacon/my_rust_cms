use yew::prelude::*;
use crate::components::admin::{AdminSidebar, AdminHeader};
use crate::components::admin::sidebar::AdminTab;
use crate::pages::admin::{dashboard::AdminDashboard, post_list::PostList, post_editor::PostEditor, page_builder::PageBuilder, media_library::MediaLibrary, enhanced_user_management::EnhancedUserManagement, comment_moderation::CommentModeration, navigation_manager::NavigationManager, template_manager::TemplateManager, analytics::Analytics, system_settings::SystemSettings, design_system::design_system_page, plugin_manager::PluginManager};
use crate::services::migrate_pages::create_essential_pages;
use crate::services::navigation_service::get_component_templates;
use crate::services::api_service::get_settings;
use crate::services::auth_service::User;
use crate::pages::admin::design_system::{AdminColorScheme, apply_admin_css_variables};
use crate::pages::admin::typography_system::load_and_apply_typography_settings;

#[derive(Properties, PartialEq)]
pub struct AdminProps {
    pub on_tab_change: Callback<AdminTab>,
    pub current_tab: AdminTab,
    pub on_public_click: Callback<()>,
    pub on_logout: Callback<()>,
    pub current_user: Option<User>,
}

#[function_component(Admin)]
pub fn admin(props: &AdminProps) -> Html {
    log::info!("🚀 ADMIN COMPONENT: Rendering with tab: {:?}", props.current_tab);
    let on_tab_change = props.on_tab_change.clone();

    // Load essential pages and container settings
    use_effect_with_deps(|_| {
        log::info!("🚀 ADMIN LAYOUT: Starting initialization...");
        wasm_bindgen_futures::spawn_local(async {
            // Ensure essential pages exist on first load
            if let Err(err) = create_essential_pages().await {
                log::warn!("Failed to create essential pages (may already exist): {:?}", err);
            }
            
            // Load container settings and inject as CSS variables
            match get_settings(Some("container")).await {
                Ok(container_settings) => {
                    let mut vars: Vec<String> = Vec::new();
                    for setting in container_settings {
                        if let Some(value) = setting.setting_value.clone() {
                            if let Some(rest) = setting.setting_key.strip_prefix("container_") {
                                vars.push(format!("--container-{}: {} !important;", rest, value));
                            }
                        }
                    }
                    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                        if let Some(style_el) = document.query_selector("style#container-settings-overrides").ok().flatten() {
                            let _ = style_el.remove();
                        }
                        if let Some(head) = document.head() {
                            if let Ok(style_el) = document.create_element("style") {
                                style_el.set_id("container-settings-overrides");
                                let css_text = format!(":root {{\n    {}\n}}", vars.join("\n    "));
                                style_el.set_text_content(Some(&css_text));
                                let _ = head.append_child(&style_el);
                                log::info!("🧩 Injected container CSS variables ({} settings)", vars.len());
                            }
                        }
                    }
                },
                Err(err) => {
                    log::warn!("Could not load container settings: {:?}", err);
                }
            }

            // Auto-apply default template if available
            let _ = get_component_templates().await;
            
            // Load and apply admin theme
            match get_settings(Some("theme")).await {
                Ok(theme_settings) => {
                    let mut current_theme = String::new();
                    let mut saved_admin_schemes: std::collections::HashMap<String, AdminColorScheme> = std::collections::HashMap::new();
                    let mut found_current_theme = false;

                    for setting in theme_settings {
                        if setting.setting_key == "theme_current_admin" {
                            if let Some(theme_name) = setting.setting_value {
                                current_theme = theme_name;
                                found_current_theme = true;
                            }
                        } else if setting.setting_key.starts_with("admin_theme_") {
                            if let Some(theme_data) = setting.setting_value {
                                if let Ok(scheme) = serde_json::from_str::<AdminColorScheme>(&theme_data) {
                                    let theme_name = setting.setting_key.strip_prefix("admin_theme_").unwrap_or("Unknown").to_string();
                                    saved_admin_schemes.insert(theme_name, scheme);
                                }
                            }
                        }
                    }

                    if found_current_theme {
                        let scheme = match current_theme.as_str() {
                            "Dark Preset" => AdminColorScheme::dark_mode(),
                            "Light Preset" => AdminColorScheme::default(),
                            custom_name => {
                                saved_admin_schemes.get(custom_name)
                                    .cloned()
                                    .unwrap_or_else(|| {
                                        log::warn!("🎨 Custom theme '{}' not found in admin, using light preset", custom_name);
                                        AdminColorScheme::default()
                                    })
                            }
                        };
                        apply_admin_css_variables(&scheme);
                        log::info!("🎨 Applied admin theme: {}", current_theme);
                    } else {
                        let default_scheme = AdminColorScheme::default();
                        apply_admin_css_variables(&default_scheme);
                        log::info!("🎨 No current theme found, using default admin theme");
                    }
                },
                Err(_) => {
                    let default_scheme = AdminColorScheme::default();
                    apply_admin_css_variables(&default_scheme);
                    log::info!("🎨 No theme settings found, using default admin theme");
                }
            }

            // Load and apply typography settings for admin
            log::info!("🎨 ADMIN: Loading typography settings...");
            web_sys::console::log_1(&"🚨 ADMIN LAYOUT: About to call load_and_apply_typography_settings".into());
            load_and_apply_typography_settings();
            
            // Also apply typography with a slight delay to ensure DOM is ready
            let timeout = gloo_timers::callback::Timeout::new(100, move || {
                log::info!("🎨 ADMIN: Re-applying typography settings after delay...");
                load_and_apply_typography_settings();
            });
            timeout.forget();
            
            log::info!("✅ ADMIN: Initialization complete with theme and typography");
        });
        
        // Cleanup function
        || {
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                if let Some(body) = document.body() {
                    let existing_class = body.class_name();
                    let new_class = existing_class.replace("admin-body", "").trim().to_string();
                    body.set_class_name(&new_class);
                }
            }
        }
    }, ());

    html! {
        <div class="admin-layout">
            <AdminHeader 
                on_public_click={props.on_public_click.clone()}
                on_logout={props.on_logout.clone()}
                current_user={props.current_user.clone()}
            />
            <div class="admin-content">
                <AdminSidebar 
                    active_tab={props.current_tab.clone()}
                    on_tab_click={on_tab_change.clone()}
                    on_public_click={props.on_public_click.clone()}
                />
                <main class="admin-main">
                    {match props.current_tab {
                        AdminTab::Dashboard => html! { <AdminDashboard on_navigate={on_tab_change.clone()} /> },
                        AdminTab::Posts => html! { <PostList /> },
                        AdminTab::PostCreate => html! { <PostEditor on_save={Callback::noop()} on_cancel={Callback::noop()} /> },
                        AdminTab::Pages => html! { <PageBuilder /> },
                        AdminTab::Media => html! { <MediaLibrary /> },
                        AdminTab::Users => html! { <EnhancedUserManagement /> },
                        AdminTab::Comments => html! { <CommentModeration /> },
                        AdminTab::Navigation => html! { <NavigationManager /> },
                        AdminTab::Templates => html! { <TemplateManager /> },
                        AdminTab::Analytics => html! { <Analytics /> },
                        AdminTab::Plugins => html! { <PluginManager /> },
                        AdminTab::DesignSystem => design_system_page(),
                        AdminTab::SystemSettings => html! { <SystemSettings /> },
                    }}
                </main>
            </div>
        </div>
    }
}