use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, InputEvent};
use crate::services::user_service::{get_users, create_user, promote_user, delete_user, CreateUserRequest};
use crate::services::auth_service::{User, AuthError};
use crate::components::simple_notification::SimpleNotification;

#[derive(Clone, PartialEq)]
pub enum UserManagementView {
    List,
    Create,
}

#[derive(Clone, PartialEq)]
pub enum NotificationType {
    Success,
    Error,
    #[allow(dead_code)]
    Info,
}

#[derive(Clone, PartialEq)]
pub struct UserForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

impl Default for UserForm {
    fn default() -> Self {
        Self {
            username: String::new(),
            email: String::new(),
            password: String::new(),
            role: "user".to_string(),
        }
    }
}

#[function_component(EnhancedUserManagement)]
pub fn enhanced_user_management() -> Html {
    let users = use_state(Vec::<User>::new);
    let loading = use_state(|| true);
    
    // Initialize view based on current URL
    let initial_view = if let Some(window) = window() {
        if let Ok(pathname) = window.location().pathname() {
            if pathname == "/admin/users/create" {
                UserManagementView::Create
            } else {
                UserManagementView::List
            }
        } else {
            UserManagementView::List
        }
    } else {
        UserManagementView::List
    };
    
    let current_view = use_state(|| initial_view);
    let user_form = use_state(UserForm::default);
    let form_loading = use_state(|| false);
    let notification = use_state(|| None::<(String, NotificationType)>);

    let clear_notification = {
        let notification = notification.clone();
        Callback::from(move |_| {
            notification.set(None);
        })
    };

    // Load users
    {
        let users = users.clone();
        let loading = loading.clone();
        let notification = notification.clone();

        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                match get_users().await {
                    Ok(fetched_users) => {
                        users.set(fetched_users);
                        loading.set(false);
                    }
                    Err(e) => {
                        notification.set(Some((format!("Failed to load users: {}", e), NotificationType::Error)));
                        loading.set(false);
                    }
                }
            });
        });
    }

    let reload_users = {
        let users = users.clone();
        let notification = notification.clone();
        Callback::from(move |_| {
            let users = users.clone();
            let notification = notification.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match get_users().await {
                    Ok(fetched_users) => {
                        users.set(fetched_users);
                    }
                    Err(e) => {
                        notification.set(Some((format!("Failed to reload users: {}", e), NotificationType::Error)));
                    }
                }
            });
        })
    };

    let on_create_user = {
        let current_view = current_view.clone();
        let user_form = user_form.clone();
        Callback::from(move |_| {
            user_form.set(UserForm::default());
            current_view.set(UserManagementView::Create);
            
            // Update URL
            if let Some(window) = window() {
                if let Ok(history) = window.history() {
                    let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some("/admin/users/create"));
                }
            }
        })
    };

    let on_promote_user = {
        let notification = notification.clone();
        let reload_users = reload_users.clone();
        Callback::from(move |(user_id, new_role): (i32, String)| {
            let notification = notification.clone();
            let reload_users = reload_users.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match promote_user(user_id, &new_role).await {
                    Ok(_) => {
                        let action = if new_role == "editor" { "promoted to" } else { "demoted to" };
                        notification.set(Some((format!("User {} {} role", action, new_role), NotificationType::Success)));
                        reload_users.emit(());
                    }
                    Err(AuthError::ServerError(msg)) => {
                        notification.set(Some((msg, NotificationType::Error)));
                    }
                    Err(e) => {
                        notification.set(Some((format!("Failed to change user role: {}", e), NotificationType::Error)));
                    }
                }
            });
        })
    };

    let on_delete_user = {
        let notification = notification.clone();
        let reload_users = reload_users.clone();
        Callback::from(move |user_id: i32| {
            let notification = notification.clone();
            let reload_users = reload_users.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match delete_user(user_id).await {
                    Ok(_) => {
                        notification.set(Some(("User deleted successfully".to_string(), NotificationType::Success)));
                        reload_users.emit(());
                    }
                    Err(AuthError::ServerError(msg)) => {
                        notification.set(Some((msg, NotificationType::Error)));
                    }
                    Err(e) => {
                        notification.set(Some((format!("Failed to delete user: {}", e), NotificationType::Error)));
                    }
                }
            });
        })
    };

    let on_save_user = {
        let current_view = current_view.clone();
        let user_form = user_form.clone();
        let form_loading = form_loading.clone();
        let notification = notification.clone();
        let reload_users = reload_users.clone();

        Callback::from(move |_| {
            let current_view = current_view.clone();
            let user_form = (*user_form).clone();
            let form_loading = form_loading.clone();
            let notification = notification.clone();
            let reload_users = reload_users.clone();

            form_loading.set(true);

            wasm_bindgen_futures::spawn_local(async move {
                let create_request = CreateUserRequest {
                    username: user_form.username,
                    email: if user_form.email.is_empty() { None } else { Some(user_form.email) },
                    password: user_form.password,
                    role: Some(user_form.role),
                };

                match create_user(&create_request).await {
                    Ok(_) => {
                        notification.set(Some(("User created successfully".to_string(), NotificationType::Success)));
                        current_view.set(UserManagementView::List);
                        reload_users.emit(());
                        
                        // Update URL back to list
                        if let Some(window) = window() {
                            if let Ok(history) = window.history() {
                                let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some("/admin/users"));
                            }
                        }
                    }
                    Err(AuthError::ServerError(msg)) => {
                        notification.set(Some((msg, NotificationType::Error)));
                    }
                    Err(e) => {
                        notification.set(Some((format!("Failed to create user: {}", e), NotificationType::Error)));
                    }
                }
                form_loading.set(false);
            });
        })
    };

    let on_cancel = {
        let current_view = current_view.clone();
        Callback::from(move |_| {
            current_view.set(UserManagementView::List);
            
            // Update URL back to list
            if let Some(window) = window() {
                if let Ok(history) = window.history() {
                    let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some("/admin/users"));
                }
            }
        })
    };

    let on_username_change = {
        let user_form = user_form.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
            let mut form = (*user_form).clone();
            form.username = target.value();
            user_form.set(form);
        })
    };

    let on_email_change = {
        let user_form = user_form.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
            let mut form = (*user_form).clone();
            form.email = target.value();
            user_form.set(form);
        })
    };

    let on_password_change = {
        let user_form = user_form.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
            let mut form = (*user_form).clone();
            form.password = target.value();
            user_form.set(form);
        })
    };

    let on_role_change = {
        let user_form = user_form.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
            let mut form = (*user_form).clone();
            form.role = target.value();
            user_form.set(form);
        })
    };

    match *current_view {
        UserManagementView::List => {
            if *loading {
                html! {
                    <div class="user-management">
                        <div class="page-header">
                            <div>
                                <h1>{"User Management"}</h1>
                                <p>{"Manage users, roles, and permissions"}</p>
                            </div>
                        </div>
                        <div class="loading">{"Loading users..."}</div>
                    </div>
                }
            } else {
                html! {
                    <div class="user-management">
                        <div class="page-header">
                            <div>
                                <h1>{"User Management"}</h1>
                                <p>{"Manage users, roles, and permissions. New signups get 'user' role by default."}</p>
                            </div>
                            <div class="header-actions">
                                <button class="btn btn-primary" onclick={on_create_user}>{"Add New User"}</button>
                            </div>
                        </div>

                        {
                            if let Some((message, notification_type)) = (*notification).clone() {
                                let class = match notification_type {
                                    NotificationType::Success => "notification-success",
                                    NotificationType::Error => "notification-error",
                                    NotificationType::Info => "notification-info",
                                };
                                html! {
                                    <SimpleNotification 
                                        message={message} 
                                        notification_type={class} 
                                        on_close={clear_notification.clone()}
                                    />
                                }
                            } else {
                                html! {}
                            }
                        }

                        <div class="admin-table-container">
                            <table>
                                <thead>
                                    <tr>
                                        <th>{"Username"}</th>
                                        <th>{"Email"}</th>
                                        <th>{"Role"}</th>
                                        <th>{"Status"}</th>
                                        <th>{"Email Verified"}</th>
                                        <th>{"Actions"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {(*users).iter().map(|user| {
                                        let user_id = user.id;
                                        
                                        let on_promote_to_editor = {
                                            let on_promote_user = on_promote_user.clone();
                                            Callback::from(move |_| on_promote_user.emit((user_id, "editor".to_string())))
                                        };

                                        let on_demote_to_user = {
                                            let on_promote_user = on_promote_user.clone();
                                            Callback::from(move |_| on_promote_user.emit((user_id, "user".to_string())))
                                        };

                                        let on_delete = {
                                            let on_delete_user = on_delete_user.clone();
                                            Callback::from(move |_| {
                                                if web_sys::window().unwrap().confirm_with_message("Are you sure you want to delete this user?").unwrap() {
                                                    on_delete_user.emit(user_id);
                                                }
                                            })
                                        };

                                        let status_class = match user.status.as_str() {
                                            "active" => "status-badge active",
                                            "inactive" => "status-badge inactive",
                                            "pending_verification" => "status-badge pending",
                                            _ => "status-badge pending",
                                        };

                                        let email_verified = user.email_verified.unwrap_or(false);

                                        html! {
                                            <tr key={user_id}>
                                                <td>{&user.username}</td>
                                                <td>{&user.email}</td>
                                                <td>
                                                    <span class={format!("role-badge {}", user.role.to_lowercase())}>
                                                        {&user.role}
                                                    </span>
                                                </td>
                                                <td><span class={status_class}>{&user.status}</span></td>
                                                <td>
                                                    <span class={if email_verified { "verified-badge" } else { "unverified-badge" }}>
                                                        {if email_verified { "✓ Verified" } else { "✗ Unverified" }}
                                                    </span>
                                                </td>
                                                <td class="actions">
                                                    {
                                                        if user.role == "user" {
                                                            html! {
                                                                <button 
                                                                    class="btn btn-small btn-success" 
                                                                    onclick={on_promote_to_editor}
                                                                    title="Promote to Editor"
                                                                >
                                                                    {"Promote to Editor"}
                                                                </button>
                                                            }
                                                        } else if user.role == "editor" {
                                                            html! {
                                                                <button 
                                                                    class="btn btn-small btn-warning" 
                                                                    onclick={on_demote_to_user}
                                                                    title="Demote to User"
                                                                >
                                                                    {"Demote to User"}
                                                                </button>
                                                            }
                                                        } else {
                                                            html! {
                                                                <span class="admin-label">{"Admin"}</span>
                                                            }
                                                        }
                                                    }
                                                    {
                                                        if user.role != "admin" {
                                                            html! {
                                                                <button 
                                                                    class="btn btn-small btn-danger" 
                                                                    onclick={on_delete}
                                                                >
                                                                    {"Delete"}
                                                                </button>
                                                            }
                                                        } else {
                                                            html! {}
                                                        }
                                                    }
                                                </td>
                                            </tr>
                                        }
                                    }).collect::<Html>()}
                                </tbody>
                            </table>
                        </div>

                        {
                            if (*users).is_empty() {
                                html! {
                                    <div class="empty-state">
                                        <p>{"No users found."}</p>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                }
            }
        }
        UserManagementView::Create => {
            html! {
                <div class="user-management">
                    <div class="page-header">
                        <h1>{"Create New User"}</h1>
                        <p>{"Create a new user account. They will be automatically verified."}</p>
                    </div>

                    {
                        if let Some((message, notification_type)) = (*notification).clone() {
                            let class = match notification_type {
                                NotificationType::Success => "notification-success",
                                NotificationType::Error => "notification-error",
                                NotificationType::Info => "notification-info",
                            };
                            html! {
                                <SimpleNotification 
                                    message={message} 
                                    notification_type={class} 
                                    on_close={clear_notification.clone()}
                                />
                            }
                        } else {
                            html! {}
                        }
                    }

                    <div class="user-form">
                        <div class="form-group">
                            <label for="username">{"Username *"}</label>
                            <input
                                type="text"
                                id="username"
                                value={(*user_form).username.clone()}
                                oninput={on_username_change}
                                placeholder="Enter username"
                                required=true
                                disabled={*form_loading}
                            />
                        </div>

                        <div class="form-group">
                            <label for="email">{"Email"}</label>
                            <input
                                type="email"
                                id="email"
                                value={(*user_form).email.clone()}
                                oninput={on_email_change}
                                placeholder="Enter email (optional)"
                                disabled={*form_loading}
                            />
                        </div>

                        <div class="form-group">
                            <label for="password">{"Password *"}</label>
                            <input
                                type="password"
                                id="password"
                                value={(*user_form).password.clone()}
                                oninput={on_password_change}
                                placeholder="Enter password"
                                required=true
                                disabled={*form_loading}
                            />
                            <small class="help-text">{"Password must be at least 6 characters long"}</small>
                        </div>

                        <div class="form-group">
                            <label for="role">{"Role"}</label>
                            <select 
                                id="role" 
                                value={(*user_form).role.clone()} 
                                onchange={on_role_change}
                                disabled={*form_loading}
                            >
                                <option value="user">{"User (can comment on posts)"}</option>
                                <option value="editor">{"Editor (can create and edit content)"}</option>
                            </select>
                        </div>

                        <div class="form-actions">
                            <button 
                                class="btn btn-primary" 
                                onclick={on_save_user}
                                disabled={*form_loading || (*user_form).username.is_empty() || (*user_form).password.is_empty()}
                            >
                                {if *form_loading { "Creating..." } else { "Create User" }}
                            </button>
                            <button 
                                class="btn btn-secondary" 
                                onclick={on_cancel}
                                disabled={*form_loading}
                            >
                                {"Cancel"}
                            </button>
                        </div>
                    </div>
                </div>
            }
        }
    }
}
