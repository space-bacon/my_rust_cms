use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::services::plugin_service::{PluginInfo, get_plugins, plugin_action, delete_plugin};
use crate::services::plugin_seeder_service::seed_sample_plugins;
use crate::components::PluginModal;

#[function_component(PluginManager)]
pub fn plugin_manager() -> Html {
    let plugins = use_state(Vec::<PluginInfo>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let success_message = use_state(|| None::<String>);
    let show_plugin_modal = use_state(|| false);
    let processing_action = use_state(|| None::<i32>);
    let seeding_samples = use_state(|| false);

    // Load plugins on component mount
    {
        let plugins = plugins.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            spawn_local(async move {
                loading.set(true);
                
                // Check authentication first
                if let Err(_) = crate::services::auth_service::get_auth_token() {
                    error.set(Some("Please log in to access the plugin manager".to_string()));
                    loading.set(false);
                    return;
                }
                
                match get_plugins().await {
                    Ok(plugin_list) => {
                        plugins.set(plugin_list);
                        error.set(None);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load plugins: {}", e)));
                    }
                }
                loading.set(false);
            });
            || ()
        }, ());
    }

    let reload_plugins = {
        let plugins = plugins.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        Callback::from(move |_| {
            let plugins = plugins.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            spawn_local(async move {
                loading.set(true);
                match get_plugins().await {
                    Ok(plugin_list) => {
                        plugins.set(plugin_list);
                        error.set(None);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to reload plugins: {}", e)));
                    }
                }
                loading.set(false);
            });
        })
    };

    let handle_plugin_action = {
        let processing_action = processing_action.clone();
        let success_message = success_message.clone();
        let error = error.clone();
        let reload_plugins = reload_plugins.clone();
        
        Callback::from(move |(plugin_id, action): (i32, String)| {
            let processing_action = processing_action.clone();
            let success_message = success_message.clone();
            let error = error.clone();
            let reload_plugins = reload_plugins.clone();
            
            spawn_local(async move {
                processing_action.set(Some(plugin_id));
                
                match plugin_action(plugin_id, &action).await {
                    Ok(response) => {
                        if response.success {
                            success_message.set(Some(response.message));
                            error.set(None);
                            reload_plugins.emit(());
                        } else {
                            error.set(Some(response.message));
                            success_message.set(None);
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Action failed: {}", e)));
                        success_message.set(None);
                    }
                }
                
                processing_action.set(None);
            });
        })
    };

    let handle_delete_plugin = {
        let processing_action = processing_action.clone();
        let success_message = success_message.clone();
        let error = error.clone();
        let reload_plugins = reload_plugins.clone();
        
        Callback::from(move |plugin_id: i32| {
            let processing_action = processing_action.clone();
            let success_message = success_message.clone();
            let error = error.clone();
            let reload_plugins = reload_plugins.clone();
            
            spawn_local(async move {
                processing_action.set(Some(plugin_id));
                
                match delete_plugin(plugin_id).await {
                    Ok(_) => {
                        success_message.set(Some("Plugin deleted successfully".to_string()));
                        error.set(None);
                        reload_plugins.emit(());
                    }
                    Err(e) => {
                        error.set(Some(format!("Delete failed: {}", e)));
                        success_message.set(None);
                    }
                }
                
                processing_action.set(None);
            });
        })
    };

    let open_plugin_modal = {
        let show_plugin_modal = show_plugin_modal.clone();
        Callback::from(move |_| {
            show_plugin_modal.set(true);
        })
    };

    let close_plugin_modal = {
        let show_plugin_modal = show_plugin_modal.clone();
        Callback::from(move |_| {
            show_plugin_modal.set(false);
        })
    };

    let on_plugin_created = {
        let reload_plugins = reload_plugins.clone();
        let show_plugin_modal = show_plugin_modal.clone();
        let success_message = success_message.clone();
        
        Callback::from(move |_| {
            show_plugin_modal.set(false);
            success_message.set(Some("Plugin created successfully! 🎉".to_string()));
            reload_plugins.emit(());
        })
    };

    let handle_seed_samples = {
        let seeding_samples = seeding_samples.clone();
        let success_message = success_message.clone();
        let error = error.clone();
        let reload_plugins = reload_plugins.clone();
        
        Callback::from(move |_| {
            let seeding_samples = seeding_samples.clone();
            let success_message = success_message.clone();
            let error = error.clone();
            let reload_plugins = reload_plugins.clone();
            
            spawn_local(async move {
                seeding_samples.set(true);
                
                match seed_sample_plugins().await {
                    Ok(_) => {
                        success_message.set(Some("Sample plugins seeded successfully! 🎉".to_string()));
                        error.set(None);
                        reload_plugins.emit(());
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to seed sample plugins: {}", e)));
                        success_message.set(None);
                    }
                }
                
                seeding_samples.set(false);
            });
        })
    };

    let handle_create_plugin = {
        let form_name = form_name.clone();
        let form_display_name = form_display_name.clone();
        let form_description = form_description.clone();
        let form_version = form_version.clone();
        let form_author = form_author.clone();
        let form_author_email = form_author_email.clone();
        let form_homepage_url = form_homepage_url.clone();
        let form_repository_url = form_repository_url.clone();
        let form_license = form_license.clone();
        let show_create_form = show_create_form.clone();
        let success_message = success_message.clone();
        let error = error.clone();
        let reload_plugins = reload_plugins.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let form_name = form_name.clone();
            let form_display_name = form_display_name.clone();
            let form_description = form_description.clone();
            let form_version = form_version.clone();
            let form_author = form_author.clone();
            let form_author_email = form_author_email.clone();
            let form_homepage_url = form_homepage_url.clone();
            let form_repository_url = form_repository_url.clone();
            let form_license = form_license.clone();
            let show_create_form = show_create_form.clone();
            let success_message = success_message.clone();
            let error = error.clone();
            let reload_plugins = reload_plugins.clone();
            
            spawn_local(async move {
                let create_request = CreatePluginRequest {
                    name: (*form_name).clone(),
                    display_name: (*form_display_name).clone(),
                    description: if form_description.is_empty() { None } else { Some((*form_description).clone()) },
                    version: (*form_version).clone(),
                    author: if form_author.is_empty() { None } else { Some((*form_author).clone()) },
                    author_email: if form_author_email.is_empty() { None } else { Some((*form_author_email).clone()) },
                    homepage_url: if form_homepage_url.is_empty() { None } else { Some((*form_homepage_url).clone()) },
                    repository_url: if form_repository_url.is_empty() { None } else { Some((*form_repository_url).clone()) },
                    license: if form_license.is_empty() { None } else { Some((*form_license).clone()) },
                    capabilities: None,
                    dependencies: None,
                    config_schema: None,
                    config_data: None,
                    manifest_data: None,
                };
                
                match create_plugin(create_request).await {
                    Ok(_) => {
                        success_message.set(Some("Plugin created successfully".to_string()));
                        error.set(None);
                        show_create_form.set(false);
                        
                        // Clear form
                        form_name.set(String::new());
                        form_display_name.set(String::new());
                        form_description.set(String::new());
                        form_version.set(String::new());
                        form_author.set(String::new());
                        form_author_email.set(String::new());
                        form_homepage_url.set(String::new());
                        form_repository_url.set(String::new());
                        form_license.set(String::new());
                        
                        reload_plugins.emit(());
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to create plugin: {}", e)));
                        success_message.set(None);
                    }
                }
            });
        })
    };

    // Input handlers
    let on_name_change = {
        let form_name = form_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            form_name.set(input.value());
        })
    };

    let on_display_name_change = {
        let form_display_name = form_display_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            form_display_name.set(input.value());
        })
    };

    let on_description_change = {
        let form_description = form_description.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            form_description.set(textarea.value());
        })
    };

    let on_version_change = {
        let form_version = form_version.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            form_version.set(input.value());
        })
    };

    let on_author_change = {
        let form_author = form_author.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            form_author.set(input.value());
        })
    };

    let on_author_email_change = {
        let form_author_email = form_author_email.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            form_author_email.set(input.value());
        })
    };

    let on_homepage_url_change = {
        let form_homepage_url = form_homepage_url.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            form_homepage_url.set(input.value());
        })
    };

    let on_repository_url_change = {
        let form_repository_url = form_repository_url.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            form_repository_url.set(input.value());
        })
    };

    let on_license_change = {
        let form_license = form_license.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            form_license.set(input.value());
        })
    };

    html! {
        <div class="plugin-manager">
            <div class="page-header">
                <h1>{"Plugin Management"}</h1>
                <p class="page-description">
                    {"Manage plugins to extend your CMS functionality. Install, activate, and configure plugins to add new features."}
                </p>
            </div>

            // Success/Error Messages
            if let Some(success) = success_message.as_ref() {
                <div class="alert alert-success">
                    <strong>{"Success!"}</strong> {" "} {success}
                </div>
            }

            if let Some(error_msg) = error.as_ref() {
                <div class="alert alert-error">
                    <strong>{"Error!"}</strong> {" "} {error_msg}
                </div>
            }

            // Action Bar
            <div class="action-bar">
                <button 
                    class="btn btn-primary"
                    onclick={toggle_create_form.clone()}
                >
                    <span class="btn-icon">{"+"}</span>
                    {"Add Plugin"}
                </button>
                <button 
                    class="btn btn-secondary"
                    onclick={move |_| reload_plugins.emit(())}
                    disabled={*loading}
                >
                    <span class="btn-icon">{"↻"}</span>
                    {"Refresh"}
                </button>
                if plugins.is_empty() && !*loading {
                    <button 
                        class="btn btn-success"
                        onclick={handle_seed_samples.clone()}
                        disabled={*seeding_samples}
                    >
                        <span class="btn-icon">{"🌱"}</span>
                        if *seeding_samples {
                            {"Seeding..."}
                        } else {
                            {"Seed Sample Plugins"}
                        }
                    </button>
                }
            </div>

            // Create Plugin Form
            if *show_create_form {
                <div class="create-plugin-form">
                    <div class="form-card">
                        <h3>{"Add New Plugin"}</h3>
                        <form onsubmit={handle_create_plugin}>
                            <div class="form-row">
                                <div class="form-group">
                                    <label for="plugin-name">{"Plugin Name*"}</label>
                                    <input 
                                        type="text" 
                                        id="plugin-name"
                                        value={(*form_name).clone()}
                                        oninput={on_name_change}
                                        placeholder="e.g., my-awesome-plugin"
                                        required=true
                                    />
                                    <small class="form-help">{"Unique identifier for the plugin (lowercase, hyphens allowed)"}</small>
                                </div>
                                <div class="form-group">
                                    <label for="plugin-display-name">{"Display Name*"}</label>
                                    <input 
                                        type="text" 
                                        id="plugin-display-name"
                                        value={(*form_display_name).clone()}
                                        oninput={on_display_name_change}
                                        placeholder="e.g., My Awesome Plugin"
                                        required=true
                                    />
                                </div>
                            </div>

                            <div class="form-group">
                                <label for="plugin-description">{"Description"}</label>
                                <textarea 
                                    id="plugin-description"
                                    value={(*form_description).clone()}
                                    oninput={on_description_change}
                                    placeholder="Brief description of what this plugin does..."
                                    rows="3"
                                ></textarea>
                            </div>

                            <div class="form-row">
                                <div class="form-group">
                                    <label for="plugin-version">{"Version*"}</label>
                                    <input 
                                        type="text" 
                                        id="plugin-version"
                                        value={(*form_version).clone()}
                                        oninput={on_version_change}
                                        placeholder="e.g., 1.0.0"
                                        required=true
                                    />
                                </div>
                                <div class="form-group">
                                    <label for="plugin-author">{"Author"}</label>
                                    <input 
                                        type="text" 
                                        id="plugin-author"
                                        value={(*form_author).clone()}
                                        oninput={on_author_change}
                                        placeholder="Author name"
                                    />
                                </div>
                            </div>

                            <div class="form-row">
                                <div class="form-group">
                                    <label for="plugin-author-email">{"Author Email"}</label>
                                    <input 
                                        type="email" 
                                        id="plugin-author-email"
                                        value={(*form_author_email).clone()}
                                        oninput={on_author_email_change}
                                        placeholder="author@example.com"
                                    />
                                </div>
                                <div class="form-group">
                                    <label for="plugin-license">{"License"}</label>
                                    <input 
                                        type="text" 
                                        id="plugin-license"
                                        value={(*form_license).clone()}
                                        oninput={on_license_change}
                                        placeholder="e.g., MIT, GPL-3.0"
                                    />
                                </div>
                            </div>

                            <div class="form-row">
                                <div class="form-group">
                                    <label for="plugin-homepage">{"Homepage URL"}</label>
                                    <input 
                                        type="url" 
                                        id="plugin-homepage"
                                        value={(*form_homepage_url).clone()}
                                        oninput={on_homepage_url_change}
                                        placeholder="https://example.com"
                                    />
                                </div>
                                <div class="form-group">
                                    <label for="plugin-repository">{"Repository URL"}</label>
                                    <input 
                                        type="url" 
                                        id="plugin-repository"
                                        value={(*form_repository_url).clone()}
                                        oninput={on_repository_url_change}
                                        placeholder="https://github.com/user/repo"
                                    />
                                </div>
                            </div>

                            <div class="form-actions">
                                <button type="submit" class="btn btn-primary">{"Create Plugin"}</button>
                                <button 
                                    type="button" 
                                    class="btn btn-secondary"
                                    onclick={toggle_create_form.clone()}
                                >
                                    {"Cancel"}
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            }

            // Plugin List
            <div class="plugin-list">
                if *loading {
                    <div class="loading-state">
                        <div class="spinner"></div>
                        <p>{"Loading plugins..."}</p>
                    </div>
                } else if plugins.is_empty() {
                    <div class="empty-state">
                        <div class="empty-icon">{"🔌"}</div>
                        <h3>{"No Plugins Installed"}</h3>
                        <p>{"Get started by adding your first plugin to extend your CMS functionality."}</p>
                        <div class="empty-actions">
                            <button 
                                class="btn btn-success"
                                onclick={handle_seed_samples.clone()}
                                disabled={*seeding_samples}
                            >
                                <span class="btn-icon">{"🌱"}</span>
                                if *seeding_samples {
                                    {"Seeding Sample Plugins..."}
                                } else {
                                    {"Try Sample Plugins"}
                                }
                            </button>
                            <button 
                                class="btn btn-primary"
                                onclick={toggle_create_form.clone()}
                            >
                                {"Add Custom Plugin"}
                            </button>
                        </div>
                    </div>
                } else {
                    <div class="plugins-grid">
                        { for plugins.iter().map(|plugin| {
                            let plugin_id = plugin.id;
                            let is_processing = processing_action.as_ref() == Some(&plugin_id);
                            
                            let activate_click = {
                                let handle_plugin_action = handle_plugin_action.clone();
                                Callback::from(move |_| {
                                    handle_plugin_action.emit((plugin_id, "activate".to_string()));
                                })
                            };
                            
                            let deactivate_click = {
                                let handle_plugin_action = handle_plugin_action.clone();
                                Callback::from(move |_| {
                                    handle_plugin_action.emit((plugin_id, "deactivate".to_string()));
                                })
                            };
                            
                            let delete_click = {
                                let handle_delete_plugin = handle_delete_plugin.clone();
                                Callback::from(move |_| {
                                    if web_sys::window()
                                        .unwrap()
                                        .confirm_with_message("Are you sure you want to delete this plugin? This action cannot be undone.")
                                        .unwrap_or(false)
                                    {
                                        handle_delete_plugin.emit(plugin_id);
                                    }
                                })
                            };

                            html! {
                                <div class={classes!("plugin-card", plugin.status.clone())}>
                                    <div class="plugin-header">
                                        <div class="plugin-info">
                                            <h3 class="plugin-name">{&plugin.display_name}</h3>
                                            <p class="plugin-version">{"v"}{&plugin.version}</p>
                                        </div>
                                        <div class={classes!("plugin-status", format!("status-{}", plugin.status))}>
                                            {plugin.status.to_uppercase()}
                                        </div>
                                    </div>
                                    
                                    if let Some(description) = &plugin.description {
                                        <p class="plugin-description">{description}</p>
                                    }
                                    
                                    <div class="plugin-meta">
                                        if let Some(author) = &plugin.author {
                                            <div class="meta-item">
                                                <span class="meta-label">{"Author:"}</span>
                                                <span class="meta-value">{author}</span>
                                            </div>
                                        }
                                        if let Some(license) = &plugin.license {
                                            <div class="meta-item">
                                                <span class="meta-label">{"License:"}</span>
                                                <span class="meta-value">{license}</span>
                                            </div>
                                        }
                                        if !plugin.capabilities.is_empty() {
                                            <div class="meta-item">
                                                <span class="meta-label">{"Capabilities:"}</span>
                                                <div class="capabilities">
                                                    { for plugin.capabilities.iter().map(|cap| {
                                                        html! { <span class="capability-tag">{cap}</span> }
                                                    })}
                                                </div>
                                            </div>
                                        }
                                    </div>
                                    
                                    if plugin.has_errors {
                                        <div class="plugin-error">
                                            <strong>{"Error:"}</strong> {" "}
                                            {plugin.last_error.as_ref().unwrap_or(&"Unknown error".to_string())}
                                        </div>
                                    }
                                    
                                    <div class="plugin-actions">
                                        if plugin.status == "active" {
                                            <button 
                                                class="btn btn-warning btn-sm"
                                                onclick={deactivate_click}
                                                disabled={is_processing || plugin.is_system}
                                            >
                                                if is_processing {
                                                    {"Deactivating..."}
                                                } else {
                                                    {"Deactivate"}
                                                }
                                            </button>
                                        } else {
                                            <button 
                                                class="btn btn-success btn-sm"
                                                onclick={activate_click}
                                                disabled={is_processing}
                                            >
                                                if is_processing {
                                                    {"Activating..."}
                                                } else {
                                                    {"Activate"}
                                                }
                                            </button>
                                        }
                                        
                                        if !plugin.is_system {
                                            <button 
                                                class="btn btn-danger btn-sm"
                                                onclick={delete_click}
                                                disabled={is_processing || plugin.status == "active"}
                                            >
                                                if is_processing {
                                                    {"Deleting..."}
                                                } else {
                                                    {"Delete"}
                                                }
                                            </button>
                                        }
                                        
                                        if let Some(homepage_url) = &plugin.homepage_url {
                                            <a 
                                                href={homepage_url.clone()} 
                                                target="_blank" 
                                                class="btn btn-secondary btn-sm"
                                            >
                                                {"Homepage"}
                                            </a>
                                        }
                                        
                                        if let Some(repository_url) = &plugin.repository_url {
                                            <a 
                                                href={repository_url.clone()} 
                                                target="_blank" 
                                                class="btn btn-secondary btn-sm"
                                            >
                                                {"Repository"}
                                            </a>
                                        }
                                    </div>
                                </div>
                            }
                        })}
                    </div>
                }
            </div>
        </div>
    }
}
