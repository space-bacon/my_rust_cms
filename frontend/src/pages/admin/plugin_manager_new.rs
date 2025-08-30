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
                error.set(None);
                
                let action_request = crate::services::plugin_service::PluginActionRequest {
                    action: action.clone(),
                    parameters: None,
                };
                
                match plugin_action(plugin_id, &action_request).await {
                    Ok(_) => {
                        let action_msg = match action.as_str() {
                            "activate" => "Plugin activated successfully",
                            "deactivate" => "Plugin deactivated successfully",
                            _ => "Plugin action completed successfully",
                        };
                        success_message.set(Some(action_msg.to_string()));
                        reload_plugins.emit(());
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to {} plugin: {}", action, e)));
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
                error.set(None);
                
                match delete_plugin(plugin_id).await {
                    Ok(_) => {
                        success_message.set(Some("Plugin deleted successfully".to_string()));
                        reload_plugins.emit(());
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to delete plugin: {}", e)));
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

    html! {
        <div class="plugin-manager">
            <div class="page-header">
                <h1 class="page-title">{"Plugin Manager"}</h1>
                <p class="page-description">{"Manage and configure plugins for your CMS"}</p>
            </div>

            // Success/Error Messages
            if let Some(success_msg) = success_message.as_ref() {
                <div class="alert alert-success">
                    <strong>{"Success!"}</strong> {" "} {success_msg}
                    <button 
                        class="alert-close" 
                        onclick={Callback::from({
                            let success_message = success_message.clone();
                            move |_| success_message.set(None)
                        })}
                    >
                        {"×"}
                    </button>
                </div>
            }
            
            if let Some(error_msg) = error.as_ref() {
                <div class="alert alert-error">
                    <strong>{"Error!"}</strong> {" "} {error_msg}
                    <button 
                        class="alert-close" 
                        onclick={Callback::from({
                            let error = error.clone();
                            move |_| error.set(None)
                        })}
                    >
                        {"×"}
                    </button>
                </div>
            }

            // Action Bar
            <div class="action-bar">
                <div class="action-bar-left">
                    <h2 class="section-title">
                        {"Installed Plugins"} 
                        <span class="plugin-count">{"("}{plugins.len()}{" plugins)"}</span>
                    </h2>
                </div>
                <div class="action-bar-right">
                    <button 
                        class="btn btn-primary"
                        onclick={open_plugin_modal.clone()}
                        disabled={*loading}
                    >
                        <span class="btn-icon">{"+"}</span>
                        {"Add Plugin"}
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
                    
                    <button 
                        class="btn btn-secondary"
                        onclick={Callback::from({
                            let reload_plugins = reload_plugins.clone();
                            move |_| reload_plugins.emit(())
                        })}
                        disabled={*loading}
                    >
                        <span class="btn-icon">{"🔄"}</span>
                        {"Refresh"}
                    </button>
                </div>
            </div>

            // Loading State
            if *loading {
                <div class="loading-container">
                    <div class="loading-spinner"></div>
                    <p>{"Loading plugins..."}</p>
                </div>
            } else if plugins.is_empty() {
                // Empty State
                <div class="empty-state">
                    <div class="empty-icon">{"🔌"}</div>
                    <h3>{"No Plugins Installed"}</h3>
                    <p>{"Get started by adding your first plugin or try some sample plugins to see how they work."}</p>
                    <div class="empty-actions">
                        <button 
                            class="btn btn-primary"
                            onclick={open_plugin_modal.clone()}
                        >
                            <span class="btn-icon">{"+"}</span>
                            {"Add Your First Plugin"}
                        </button>
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
                    </div>
                </div>
            } else {
                // Plugin Grid
                <div class="plugins-grid">
                    {for plugins.iter().map(|plugin| {
                        let plugin_id = plugin.id;
                        let is_processing = *processing_action == Some(plugin_id);
                        
                        html! {
                            <div class="plugin-card" key={plugin.id}>
                                <div class="plugin-header">
                                    <div class="plugin-info">
                                        <h3 class="plugin-name">{&plugin.display_name}</h3>
                                        <p class="plugin-version">{"v"}{&plugin.version}</p>
                                    </div>
                                    <div class={format!("plugin-status {}", if plugin.is_active { "active" } else { "inactive" })}>
                                        {if plugin.is_active { "Active" } else { "Inactive" }}
                                    </div>
                                </div>
                                
                                if let Some(description) = &plugin.description {
                                    <p class="plugin-description">{description}</p>
                                }
                                
                                <div class="plugin-meta">
                                    if let Some(author) = &plugin.author {
                                        <span class="plugin-author">{"by "}{author}</span>
                                    }
                                    <span class="plugin-id">{"ID: "}{&plugin.name}</span>
                                </div>
                                
                                <div class="plugin-actions">
                                    if plugin.is_active {
                                        <button 
                                            class="btn btn-warning btn-sm"
                                            onclick={Callback::from({
                                                let handle_plugin_action = handle_plugin_action.clone();
                                                move |_| handle_plugin_action.emit((plugin_id, "deactivate".to_string()))
                                            })}
                                            disabled={is_processing}
                                        >
                                            if is_processing {
                                                <span class="btn-spinner"></span>
                                            }
                                            {"Deactivate"}
                                        </button>
                                    } else {
                                        <button 
                                            class="btn btn-success btn-sm"
                                            onclick={Callback::from({
                                                let handle_plugin_action = handle_plugin_action.clone();
                                                move |_| handle_plugin_action.emit((plugin_id, "activate".to_string()))
                                            })}
                                            disabled={is_processing}
                                        >
                                            if is_processing {
                                                <span class="btn-spinner"></span>
                                            }
                                            {"Activate"}
                                        </button>
                                    }
                                    
                                    <button 
                                        class="btn btn-secondary btn-sm"
                                        disabled={is_processing}
                                    >
                                        {"Settings"}
                                    </button>
                                    
                                    <button 
                                        class="btn btn-danger btn-sm"
                                        onclick={Callback::from({
                                            let handle_delete_plugin = handle_delete_plugin.clone();
                                            move |_| {
                                                if web_sys::window()
                                                    .unwrap()
                                                    .confirm_with_message(&format!("Are you sure you want to delete the '{}' plugin? This action cannot be undone.", plugin.display_name))
                                                    .unwrap_or(false)
                                                {
                                                    handle_delete_plugin.emit(plugin_id);
                                                }
                                            }
                                        })}
                                        disabled={is_processing}
                                    >
                                        if is_processing {
                                            <span class="btn-spinner"></span>
                                        }
                                        {"Delete"}
                                    </button>
                                </div>
                            </div>
                        }
                    })}
                </div>
            }

            // Plugin Modal
            <PluginModal 
                show={*show_plugin_modal}
                on_close={close_plugin_modal}
                on_success={on_plugin_created}
            />
        </div>
    }
}

