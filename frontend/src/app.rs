use yew::prelude::*;
use crate::services::auth_context::{use_auth, logout_and_update_context};
use crate::pages::public::{PublicRouter, PublicPage};
use crate::components::admin::sidebar::AdminTab;
use web_sys::window;
use std::ops::Deref;

#[derive(Clone, PartialEq)]
pub enum AppView {
    Public,
    Login,
    Signup,
    VerifyEmail,
    Admin(AdminTab),
}

#[derive(Clone, PartialEq)]
pub enum AppRoute {
    Public(PublicPage),
    Admin(AdminTab),
    Login,
    Signup,
    VerifyEmail,
}

// Helper function to convert AdminTab to URL path
fn admin_tab_to_path(tab: &AdminTab) -> String {
    match tab {
        AdminTab::Dashboard => "/admin".to_string(),
        AdminTab::Posts => "/admin/posts".to_string(),
        AdminTab::PostCreate => "/admin/posts/create".to_string(),
        AdminTab::Pages => "/admin/pages".to_string(),
        AdminTab::Media => "/admin/media".to_string(),
        AdminTab::Users => "/admin/users".to_string(),
        AdminTab::Comments => "/admin/comments".to_string(),
        AdminTab::Navigation => "/admin/navigation".to_string(),
        AdminTab::Templates => "/admin/templates".to_string(),
        AdminTab::Analytics => "/admin/analytics".to_string(),
        AdminTab::Plugins => "/admin/plugins".to_string(),
        AdminTab::SystemSettings => "/admin/settings".to_string(),
        AdminTab::DesignSystem => "/admin/design".to_string(),
    }
}

// Helper function to convert URL path to AdminTab
fn path_to_admin_tab(path: &str) -> Option<AdminTab> {
    match path {
        "/admin" | "/admin/" => Some(AdminTab::Dashboard),
        "/admin/dashboard" => Some(AdminTab::Dashboard),
        "/admin/posts" => Some(AdminTab::Posts),
        "/admin/posts/create" => Some(AdminTab::PostCreate),
        "/admin/pages" => Some(AdminTab::Pages),
        "/admin/media" => Some(AdminTab::Media),
        "/admin/users" | "/admin/users/create" => Some(AdminTab::Users),
        "/admin/comments" => Some(AdminTab::Comments),
        "/admin/navigation" => Some(AdminTab::Navigation),
        "/admin/templates" => Some(AdminTab::Templates),
        "/admin/analytics" => Some(AdminTab::Analytics),
        "/admin/plugins" => Some(AdminTab::Plugins),
        "/admin/settings" => Some(AdminTab::SystemSettings),
        "/admin/design" => Some(AdminTab::DesignSystem),
        _ => None,
    }
}

// Function to parse the current URL and determine the initial route
fn parse_current_url() -> AppRoute {
    if let Some(window) = window() {
        if let Ok(location) = window.location().pathname() {
            web_sys::console::log_1(&format!("App: Parsing URL path: {}", location).into());
            
            // Check for admin routes first
            if location.starts_with("/admin") {
                if let Some(admin_tab) = path_to_admin_tab(&location) {
                    return AppRoute::Admin(admin_tab);
                } else {
                    web_sys::console::log_1(&format!("App: Unknown admin path, defaulting to Dashboard: {}", location).into());
                    return AppRoute::Admin(AdminTab::Dashboard);
                }
            }

            // Check for auth routes
            if location == "/login" {
                return AppRoute::Login;
            }
            if location == "/signup" {
                return AppRoute::Signup;
            }
            if location == "/verify-email" {
                return AppRoute::VerifyEmail;
            }
            
            // Handle public routes
            let public_page = match location.as_str() {
                "/" => PublicPage::Home,
                "/posts" => PublicPage::Posts,
                path if path.starts_with("/post/") => {
                    if let Ok(id) = path.trim_start_matches("/post/").parse::<i32>() {
                        web_sys::console::log_1(&format!("App: Parsed post ID: {}", id).into());
                        PublicPage::Post(id)
                    } else {
                        web_sys::console::log_1(&format!("App: Failed to parse post ID from: {}", path).into());
                        PublicPage::Home
                    }
                }
                path if path.starts_with("/page/") => {
                    let slug = path.trim_start_matches("/page/");
                    PublicPage::Page(slug.to_string())
                }
                _ => {
                    web_sys::console::log_1(&format!("App: Unknown path, defaulting to Home: {}", location).into());
                    PublicPage::Home
                }
            };
            
            AppRoute::Public(public_page)
        } else {
            web_sys::console::log_1(&"App: Failed to get pathname, defaulting to Home".into());
            AppRoute::Public(PublicPage::Home)
        }
    } else {
        web_sys::console::log_1(&"App: No window object, defaulting to Home".into());
        AppRoute::Public(PublicPage::Home)
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let auth = use_auth();
    let current_route = use_state(|| parse_current_url());
    
    // Determine current view from route
    let current_view = match current_route.deref() {
        AppRoute::Public(_) => AppView::Public,
        AppRoute::Admin(tab) => AppView::Admin(tab.clone()),
        AppRoute::Login => AppView::Login,
        AppRoute::Signup => AppView::Signup,
        AppRoute::VerifyEmail => AppView::VerifyEmail,
    };

    // TODO: Add browser back/forward navigation support later

    let switch_to_admin = {
        let current_route = current_route.clone();
        let auth = auth.clone();
        Callback::from(move |tab: Option<AdminTab>| {
            if auth.is_authenticated {
                let admin_tab = tab.unwrap_or(AdminTab::Dashboard);
                let new_route = AppRoute::Admin(admin_tab.clone());
                
                // Update URL
                if let Some(window) = window() {
                    if let Ok(history) = window.history() {
                        let url = admin_tab_to_path(&admin_tab);
                        if let Err(e) = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url)) {
                            web_sys::console::warn_1(&format!("Failed to update admin URL: {:?}", e).into());
                        } else {
                            web_sys::console::log_1(&format!("App: Updated admin URL to: {}", url).into());
                        }
                    }
                }
                
                current_route.set(new_route);
            } else {
                // Update URL to login
                if let Some(window) = window() {
                    if let Ok(history) = window.history() {
                        if let Err(e) = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some("/login")) {
                            web_sys::console::warn_1(&format!("Failed to update login URL: {:?}", e).into());
                        }
                    }
                }
                current_route.set(AppRoute::Login);
            }
        })
    };

    let switch_to_public = {
        let current_route = current_route.clone();
        Callback::from(move |page: Option<PublicPage>| {
            let public_page = page.unwrap_or(PublicPage::Home);
            let new_route = AppRoute::Public(public_page.clone());
            
            // Update URL
            if let Some(window) = window() {
                if let Ok(history) = window.history() {
                    let url = match &public_page {
                        PublicPage::Home => "/".to_string(),
                        PublicPage::Posts => "/posts".to_string(),
                        PublicPage::Post(id) => format!("/post/{}", id),
                        PublicPage::Page(slug) => format!("/page/{}", slug),
                    };
                    
                    if let Err(e) = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url)) {
                        web_sys::console::warn_1(&format!("Failed to update public URL: {:?}", e).into());
                    } else {
                        web_sys::console::log_1(&format!("App: Updated public URL to: {}", url).into());
                    }
                }
            }
            
            current_route.set(new_route);
        })
    };

    let on_login_success = {
        let current_route = current_route.clone();
        Callback::from(move |_| {
            let new_route = AppRoute::Admin(AdminTab::Dashboard);
            
            // Update URL
            if let Some(window) = window() {
                if let Ok(history) = window.history() {
                    if let Err(e) = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some("/admin")) {
                        web_sys::console::warn_1(&format!("Failed to update admin URL after login: {:?}", e).into());
                    }
                }
            }
            
            current_route.set(new_route);
        })
    };

    let on_logout = {
        let current_route = current_route.clone();
        let auth = auth.clone();
        Callback::from(move |_| {
            let current_route = current_route.clone();
            let auth = auth.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = logout_and_update_context(&auth).await;
                
                // Update URL
                if let Some(window) = window() {
                    if let Ok(history) = window.history() {
                        if let Err(e) = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some("/")) {
                            web_sys::console::warn_1(&format!("Failed to update URL after logout: {:?}", e).into());
                        }
                    }
                }
                
                current_route.set(AppRoute::Public(PublicPage::Home));
            });
        })
    };

    let on_public_navigate = {
        let current_route = current_route.clone();
        Callback::from(move |page: PublicPage| {
            web_sys::console::log_1(&format!("App: Navigating to page: {:?}", page).into());
            
            let new_route = AppRoute::Public(page.clone());
            
            // Update the browser URL to match the current page
            if let Some(window) = window() {
                if let Ok(history) = window.history() {
                    let url = match &page {
                        PublicPage::Home => "/".to_string(),
                        PublicPage::Posts => "/posts".to_string(),
                        PublicPage::Post(id) => format!("/post/{}", id),
                        PublicPage::Page(slug) => format!("/page/{}", slug),
                    };
                    
                    if let Err(e) = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url)) {
                        web_sys::console::warn_1(&format!("Failed to update URL: {:?}", e).into());
                    } else {
                        web_sys::console::log_1(&format!("App: Updated URL to: {}", url).into());
                    }
                }
            }
            
            current_route.set(new_route);
        })
    };

    let on_admin_navigate = {
        let current_route = current_route.clone();
        Callback::from(move |tab: AdminTab| {
            web_sys::console::log_1(&format!("App: Navigating to admin tab: {:?}", tab).into());
            
            let new_route = AppRoute::Admin(tab.clone());
            
            // Update the browser URL to match the current tab
            if let Some(window) = window() {
                if let Ok(history) = window.history() {
                    let url = admin_tab_to_path(&tab);
                    
                    if let Err(e) = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url)) {
                        web_sys::console::warn_1(&format!("Failed to update admin URL: {:?}", e).into());
                    } else {
                        web_sys::console::log_1(&format!("App: Updated admin URL to: {}", url).into());
                    }
                }
            }
            
            current_route.set(new_route);
        })
    };

    if auth.loading {
        html! {
            <div class="loading-screen">
                <div class="loading-spinner">{"Loading..."}</div>
            </div>
        }
    } else {
        html! {
            <div>
                {match current_view {
                    AppView::Public => {
                        if let AppRoute::Public(public_page) = current_route.deref() {
                            html! {
                                <PublicRouter 
                                    current_page={public_page.clone()}
                                    on_admin_click={Callback::from({
                                        let switch_to_admin = switch_to_admin.clone();
                                        move |_| switch_to_admin.emit(None)
                                    })}
                                    on_navigate={on_public_navigate}
                                />
                            }
                        } else {
                            html! { <div>{"Error: Invalid route state"}</div> }
                        }
                    },
                    AppView::Login => html! {
                        <div>
                            <crate::pages::auth::Login on_login_success={on_login_success} />
                        </div>
                    },
                    AppView::Signup => html! {
                        <div>
                            <crate::pages::auth::SignupPage />
                        </div>
                    },
                    AppView::VerifyEmail => html! {
                        <div>
                            <crate::pages::auth::VerifyEmailPage />
                        </div>
                    },
                    AppView::Admin(admin_tab) => html! {
                        <crate::components::AdminGuard>
                            <crate::pages::admin::Admin 
                                current_tab={admin_tab.clone()}
                                on_public_click={Callback::from({
                                    let switch_to_public = switch_to_public.clone();
                                    move |_| switch_to_public.emit(None)
                                })}
                                on_logout={on_logout}
                                on_tab_change={on_admin_navigate}
                                current_user={auth.user.clone()}
                            />
                        </crate::components::AdminGuard>
                    },
                }}
            </div>
        }
    }
} 