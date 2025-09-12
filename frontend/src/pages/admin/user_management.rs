use yew::prelude::*;
use crate::services::api_service::{get_users, create_user, update_user, delete_user, User};
use wasm_bindgen::JsCast;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum UserManagementView {
    List,
    Create,
    Edit(User),
}

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub struct UserForm {
    pub username: String,
    pub email: String,
    pub role: String,
    pub status: String,
}

impl Default for UserForm {
    fn default() -> Self {
        Self {
            username: String::new(),
            email: String::new(),
            role: "user".to_string(),
            status: "active".to_string(),
        }
    }
}

#[function_component(UserManagement)]
pub fn user_management() -> Html {
    let users = use_state(Vec::<User>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let current_view = use_state(|| UserManagementView::List);
    let user_form = use_state(UserForm::default);
    let form_loading = use_state(|| false);

    // Load users
    {
        let users = users.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match get_users().await {
                    Ok(fetched_users) => {
                        users.set(fetched_users);
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e.to_string()));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, ());
    }

    let on_create_user = {
        let current_view = current_view.clone();
        Callback::from(move |_| {
            current_view.set(UserManagementView::Create);
        })
    };

    let on_edit_user = {
        let current_view = current_view.clone();
        Callback::from(move |user: User| {
            let _form = UserForm {
                username: user.username.clone(),
                email: user.email.clone(),
                role: user.role.clone(),
                status: user.status.clone(),
            };
            current_view.set(UserManagementView::Edit(user));
        })
    };

    let on_delete_user = {
        let users = users.clone();
        let error = error.clone();
        Callback::from(move |user_id: i32| {
            let users = users.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match delete_user(user_id).await {
                    Ok(_) => {
                        let mut current_users = (*users).clone();
                        current_users.retain(|user| user.id != Some(user_id));
                        users.set(current_users);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to delete user: {}", e)));
                    }
                }
            });
        })
    };

    let on_save_user = {
        let users = users.clone();
        let current_view = current_view.clone();
        let user_form = user_form.clone();
        let form_loading = form_loading.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let users = users.clone();
            let current_view = current_view.clone();
            let user_form = (*user_form).clone();
            let form_loading = form_loading.clone();
            let error = error.clone();

            form_loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let new_user = User {
                    id: None,
                    username: user_form.username,
                    email: user_form.email,
                    role: user_form.role,
                    status: user_form.status,
                };

                match create_user(&new_user).await {
                    Ok(created_user) => {
                        let mut current_users = (*users).clone();
                        current_users.push(created_user);
                        users.set(current_users);
                        current_view.set(UserManagementView::List);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to create user: {}", e)));
                    }
                }
                form_loading.set(false);
            });
        })
    };

    let on_update_user = {
        let users = users.clone();
        let current_view = current_view.clone();
        let user_form = user_form.clone();
        let form_loading = form_loading.clone();
        let error = error.clone();

        Callback::from(move |user_id: i32| {
            let users = users.clone();
            let current_view = current_view.clone();
            let user_form = (*user_form).clone();
            let form_loading = form_loading.clone();
            let error = error.clone();

            form_loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let updated_user = User {
                    id: Some(user_id),
                    username: user_form.username,
                    email: user_form.email,
                    role: user_form.role,
                    status: user_form.status,
                };

                match update_user(user_id, &updated_user).await {
                    Ok(saved_user) => {
                        let mut current_users = (*users).clone();
                        if let Some(index) = current_users.iter().position(|u| u.id == Some(user_id)) {
                            current_users[index] = saved_user;
                            users.set(current_users);
                        }
                        current_view.set(UserManagementView::List);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to update user: {}", e)));
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
        })
    };

    let on_username_change = {
        let user_form = user_form.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
            let mut form = (*user_form).clone();
            form.username = target.value();
            user_form.set(form);
        })
    };

    let on_email_change = {
        let user_form = user_form.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
            let mut form = (*user_form).clone();
            form.email = target.value();
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

    let on_status_change = {
        let user_form = user_form.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
            let mut form = (*user_form).clone();
            form.status = target.value();
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
                                <p>{"Manage users, roles, and permissions"}</p>
                            </div>
                            <div class="header-actions">
                                <button class="btn btn-primary" onclick={on_create_user}>{"Add New User"}</button>
                            </div>
                        </div>

                        if let Some(ref error_msg) = *error {
                            <div class="error-message">{"Error: "}{error_msg}</div>
                        }

                        <div class="admin-table-container">
                            <table>
                                <thead>
                                    <tr>
                                        <th>{"Username"}</th>
                                        <th>{"Email"}</th>
                                        <th>{"Role"}</th>
                                        <th>{"Status"}</th>
                                        <th>{"Actions"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {(*users).iter().map(|user| {
                                        let user_id = user.id.unwrap_or(0);
                                        
                                        let on_edit = {
                                            let on_edit_user = on_edit_user.clone();
                                            let user = user.clone();
                                            Callback::from(move |_| on_edit_user.emit(user.clone()))
                                        };

                                        let on_delete = {
                                            let on_delete_user = on_delete_user.clone();
                                            let user_id = user_id;
                                            Callback::from(move |_| on_delete_user.emit(user_id))
                                        };

                                        let status_class = match user.status.as_str() {
                                            "active" => "status-badge active",
                                            "inactive" => "status-badge inactive",
                                            _ => "status-badge pending",
                                        };

                                        html! {
                                            <tr key={user_id}>
                                                <td>{&user.username}</td>
                                                <td>{&user.email}</td>
                                                <td>{&user.role}</td>
                                                <td><span class={status_class}>{&user.status}</span></td>
                                                <td class="actions">
                                                    <button class="btn btn-small btn-secondary" onclick={on_edit}>{"Edit"}</button>
                                                    if user.role != "admin" {
                                                        <button class="btn btn-small btn-danger" onclick={on_delete}>{"Delete"}</button>
                                                    }
                                                </td>
                                            </tr>
                                        }
                                    }).collect::<Html>()}
                                </tbody>
                            </table>
                        </div>

                        if (*users).is_empty() {
                            <div class="empty-state">
                                <p>{"No users found."}</p>
                            </div>
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
                    </div>

                    if let Some(ref error_msg) = *error {
                        <div class="error-message">{"Error: "}{error_msg}</div>
                    }

                    <div class="user-form">
                        <div class="form-group">
                            <label for="username">{"Username"}</label>
                            <input
                                type="text"
                                id="username"
                                value={(*user_form).username.clone()}
                                onchange={on_username_change}
                                placeholder="Enter username"
                                required=true
                            />
                        </div>

                        <div class="form-group">
                            <label for="email">{"Email"}</label>
                            <input
                                type="email"
                                id="email"
                                value={(*user_form).email.clone()}
                                onchange={on_email_change}
                                placeholder="Enter email"
                                required=true
                            />
                        </div>

                        <div class="form-group">
                            <label for="role">{"Role"}</label>
                            <select id="role" value={(*user_form).role.clone()} onchange={on_role_change}>
                                <option value="user">{"User"}</option>
                                <option value="editor">{"Editor"}</option>
                                <option value="admin">{"Administrator"}</option>
                            </select>
                        </div>

                        <div class="form-group">
                            <label for="status">{"Status"}</label>
                            <select id="status" value={(*user_form).status.clone()} onchange={on_status_change}>
                                <option value="active">{"Active"}</option>
                                <option value="inactive">{"Inactive"}</option>
                                <option value="pending">{"Pending"}</option>
                            </select>
                        </div>

                        <div class="form-actions">
                            <button 
                                class="btn btn-primary" 
                                onclick={on_save_user}
                                disabled={*form_loading}
                            >
                                {if *form_loading { "Creating..." } else { "Create User" }}
                            </button>
                            <button class="btn btn-secondary" onclick={on_cancel}>{"Cancel"}</button>
                        </div>
                    </div>
                </div>
            }
        }
        UserManagementView::Edit(ref user) => {
            let user_id = user.id.unwrap_or(0);
            
            let on_update = {
                let on_update_user = on_update_user.clone();
                let user_id = user_id;
                Callback::from(move |_| on_update_user.emit(user_id))
            };

            html! {
                <div class="user-management">
                    <div class="page-header">
                        <h1>{"Edit User"}</h1>
                    </div>

                    if let Some(ref error_msg) = *error {
                        <div class="error-message">{"Error: "}{error_msg}</div>
                    }

                    <div class="user-form">
                        <div class="form-group">
                            <label for="username">{"Username"}</label>
                            <input
                                type="text"
                                id="username"
                                value={(*user_form).username.clone()}
                                onchange={on_username_change}
                                placeholder="Enter username"
                                required=true
                            />
                        </div>

                        <div class="form-group">
                            <label for="email">{"Email"}</label>
                            <input
                                type="email"
                                id="email"
                                value={(*user_form).email.clone()}
                                onchange={on_email_change}
                                placeholder="Enter email"
                                required=true
                            />
                        </div>

                        <div class="form-group">
                            <label for="role">{"Role"}</label>
                            <select id="role" value={(*user_form).role.clone()} onchange={on_role_change}>
                                <option value="user">{"User"}</option>
                                <option value="editor">{"Editor"}</option>
                                <option value="admin">{"Administrator"}</option>
                            </select>
                        </div>

                        <div class="form-group">
                            <label for="status">{"Status"}</label>
                            <select id="status" value={(*user_form).status.clone()} onchange={on_status_change}>
                                <option value="active">{"Active"}</option>
                                <option value="inactive">{"Inactive"}</option>
                                <option value="pending">{"Pending"}</option>
                            </select>
                        </div>

                        <div class="form-actions">
                            <button 
                                class="btn btn-primary" 
                                onclick={on_update}
                                disabled={*form_loading}
                            >
                                {if *form_loading { "Updating..." } else { "Update User" }}
                            </button>
                            <button class="btn btn-secondary" onclick={on_cancel}>{"Cancel"}</button>
                        </div>
                    </div>
                </div>
            }
        }
    }
} 