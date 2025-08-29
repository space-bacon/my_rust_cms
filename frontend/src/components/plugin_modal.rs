use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement, HtmlSelectElement, File, MouseEvent};
use wasm_bindgen_futures::spawn_local;
use crate::services::plugin_service::{create_plugin, CreatePluginRequest, upload_plugin_zip, download_base_plugin_template};

#[derive(Properties, PartialEq)]
pub struct PluginModalProps {
    pub show: bool,
    pub on_close: Callback<()>,
    pub on_success: Callback<()>,
}

#[derive(Clone, PartialEq)]
pub enum PluginInstallMethod {
    Manual,
    ZipUpload,
    Repository,
}

#[derive(Clone, PartialEq)]
pub enum ModalTab {
    Install,
    Documentation,
    Examples,
}

#[function_component(PluginModal)]
pub fn plugin_modal(props: &PluginModalProps) -> Html {
    let current_tab = use_state(|| ModalTab::Install);
    let install_method = use_state(|| PluginInstallMethod::Manual);
    let uploading = use_state(|| false);
    let error = use_state(|| None::<String>);
    let success_message = use_state(|| None::<String>);
    
    // Manual form fields
    let form_name = use_state(String::new);
    let form_display_name = use_state(String::new);
    let form_description = use_state(String::new);
    let form_version = use_state(String::new);
    let form_author = use_state(String::new);
    let form_author_email = use_state(String::new);
    let form_homepage_url = use_state(String::new);
    let form_repository_url = use_state(String::new);
    let form_license = use_state(String::new);
    
    // ZIP upload fields
    let selected_file = use_state(|| None::<File>);
    
    // Repository fields
    let repo_url = use_state(String::new);
    let repo_branch = use_state(|| "main".to_string());

    let close_modal = {
        let on_close = props.on_close.clone();
        let error = error.clone();
        let success_message = success_message.clone();
        let current_tab = current_tab.clone();
        
        Callback::from(move |_| {
            error.set(None);
            success_message.set(None);
            current_tab.set(ModalTab::Install);
            on_close.emit(());
        })
    };

    let switch_tab = {
        let current_tab = current_tab.clone();
        let error = error.clone();
        
        Callback::from(move |tab: ModalTab| {
            error.set(None);
            current_tab.set(tab);
        })
    };

    let download_template = {
        let error = error.clone();
        let success_message = success_message.clone();
        
        Callback::from(move |_: MouseEvent| {
            let error = error.clone();
            let success_message = success_message.clone();
            
            spawn_local(async move {
                match download_base_plugin_template().await {
                    Ok(blob) => {
                        // Create download link
                        if let Some(window) = web_sys::window() {
                            if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                                if let Some(document) = window.document() {
                                    if let Ok(element) = document.create_element("a") {
                                        let _ = element.set_attribute("href", &url);
                                        let _ = element.set_attribute("download", "base-plugin-template.zip");
                                        let anchor = element.dyn_into::<web_sys::HtmlElement>().unwrap();
                                        anchor.click();
                                        let _ = web_sys::Url::revoke_object_url(&url);
                                        success_message.set(Some("Plugin template downloaded successfully!".to_string()));
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to download template: {}", e)));
                    }
                }
            });
        })
    };

    let handle_method_change = {
        let install_method = install_method.clone();
        let error = error.clone();
        
        Callback::from(move |e: Event| {
            error.set(None);
            if let Some(select) = e.target().and_then(|t| t.dyn_into::<HtmlSelectElement>().ok()) {
                let method = match select.value().as_str() {
                    "manual" => PluginInstallMethod::Manual,
                    "zip" => PluginInstallMethod::ZipUpload,
                    "repo" => PluginInstallMethod::Repository,
                    _ => PluginInstallMethod::Manual,
                };
                install_method.set(method);
            }
        })
    };

    let handle_file_change = {
        let selected_file = selected_file.clone();
        let error = error.clone();
        
        Callback::from(move |e: Event| {
            error.set(None);
            if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                if let Some(files) = input.files() {
                    if files.length() > 0 {
                        if let Some(file) = files.get(0) {
                            // Validate file type
                            if file.name().ends_with(".zip") {
                                selected_file.set(Some(file));
                            } else {
                                error.set(Some("Please select a ZIP file".to_string()));
                                selected_file.set(None);
                            }
                        }
                    }
                }
            }
        })
    };

    let handle_manual_submit = {
        let form_name = form_name.clone();
        let form_display_name = form_display_name.clone();
        let form_description = form_description.clone();
        let form_version = form_version.clone();
        let form_author = form_author.clone();
        let form_author_email = form_author_email.clone();
        let form_homepage_url = form_homepage_url.clone();
        let form_repository_url = form_repository_url.clone();
        let form_license = form_license.clone();
        let uploading = uploading.clone();
        let error = error.clone();
        let success_message = success_message.clone();
        let on_success = props.on_success.clone();
        
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
            let uploading = uploading.clone();
            let error = error.clone();
            let success_message = success_message.clone();
            let on_success = on_success.clone();
            
            spawn_local(async move {
                uploading.set(true);
                error.set(None);
                
                let request = CreatePluginRequest {
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
                
                match create_plugin(request).await {
                    Ok(_) => {
                        success_message.set(Some("Plugin created successfully!".to_string()));
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
                        
                        on_success.emit(());
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to create plugin: {}", e)));
                    }
                }
                
                uploading.set(false);
            });
        })
    };

    let handle_zip_submit = {
        let selected_file = selected_file.clone();
        let uploading = uploading.clone();
        let error = error.clone();
        let success_message = success_message.clone();
        let on_success = props.on_success.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            if let Some(file) = (*selected_file).clone() {
                let uploading = uploading.clone();
                let error = error.clone();
                let success_message = success_message.clone();
                let on_success = on_success.clone();
                
                spawn_local(async move {
                    uploading.set(true);
                    error.set(None);
                    
                    match upload_plugin_zip(file).await {
                        Ok(_) => {
                            success_message.set(Some("Plugin uploaded and installed successfully! 🎉".to_string()));
                            on_success.emit(());
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to upload plugin: {}", e)));
                        }
                    }
                    
                    uploading.set(false);
                });
            } else {
                error.set(Some("Please select a ZIP file".to_string()));
            }
        })
    };

    let handle_repo_submit = {
        let repo_url = repo_url.clone();
        let repo_branch = repo_branch.clone();
        let uploading = uploading.clone();
        let error = error.clone();
        let success_message = success_message.clone();
        let on_success = props.on_success.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            if !repo_url.is_empty() {
                let uploading = uploading.clone();
                let error = error.clone();
                let success_message = success_message.clone();
                let on_success = on_success.clone();
                
                spawn_local(async move {
                    uploading.set(true);
                    error.set(None);
                    
                    // TODO: Implement repository clone functionality
                    // For now, show a placeholder message
                    success_message.set(Some("Repository installation functionality coming soon!".to_string()));
                    on_success.emit(());
                    
                    uploading.set(false);
                });
            } else {
                error.set(Some("Please enter a repository URL".to_string()));
            }
        })
    };

    if !props.show {
        return html! {};
    }

    html! {
        <div class="modal-overlay" onclick={close_modal.clone()}>
            <div class="modal-content plugin-modal" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <div class="modal-header">
                    <h2 class="modal-title">{"Plugin Manager"}</h2>
                    <button class="modal-close" onclick={close_modal.clone()}>
                        <span class="close-icon">{"×"}</span>
                    </button>
                </div>
                
                // Tab Navigation
                <div class="tab-navigation">
                    <button 
                        class={if *current_tab == ModalTab::Install { "tab-button active" } else { "tab-button" }}
                        onclick={{
                            let switch_tab = switch_tab.clone();
                            Callback::from(move |_| switch_tab.emit(ModalTab::Install))
                        }}
                    >
                        {"Install Plugin"}
                    </button>
                    <button 
                        class={if *current_tab == ModalTab::Documentation { "tab-button active" } else { "tab-button" }}
                        onclick={{
                            let switch_tab = switch_tab.clone();
                            Callback::from(move |_| switch_tab.emit(ModalTab::Documentation))
                        }}
                    >
                        {"Documentation"}
                    </button>
                    <button 
                        class={if *current_tab == ModalTab::Examples { "tab-button active" } else { "tab-button" }}
                        onclick={{
                            let switch_tab = switch_tab.clone();
                            Callback::from(move |_| switch_tab.emit(ModalTab::Examples))
                        }}
                    >
                        {"Examples & Templates"}
                    </button>
                </div>
                
                <div class="modal-body">
                    {match *current_tab {
                        ModalTab::Install => html! {
                            <div class="tab-content install-tab">
                    // Success/Error Messages
                    if let Some(success_msg) = success_message.as_ref() {
                        <div class="alert alert-success">
                            <strong>{"Success!"}</strong> {" "} {success_msg}
                        </div>
                    }
                    
                    if let Some(error_msg) = error.as_ref() {
                        <div class="alert alert-error">
                            <strong>{"Error!"}</strong> {" "} {error_msg}
                        </div>
                    }
                    
                    // Installation Method Selector
                    <div class="form-group">
                        <label class="form-label">{"Installation Method"}</label>
                        <select class="form-select" onchange={handle_method_change}>
                            <option value="manual" selected={*install_method == PluginInstallMethod::Manual}>
                                {"Manual Entry"}
                            </option>
                            <option value="zip" selected={*install_method == PluginInstallMethod::ZipUpload}>
                                {"ZIP Upload"}
                            </option>
                            <option value="repo" selected={*install_method == PluginInstallMethod::Repository}>
                                {"Git Repository"}
                            </option>
                        </select>
                        <small class="form-help">{"Choose how you want to install the plugin"}</small>
                    </div>
                    
                    // Method-specific forms
                    {match *install_method {
                        PluginInstallMethod::Manual => html! {
                            <form onsubmit={handle_manual_submit}>
                                <div class="form-row">
                                    <div class="form-group">
                                        <label class="form-label required">{"Plugin Name"}</label>
                                        <input
                                            type="text"
                                            class="form-input"
                                            placeholder="e.g., my-awesome-plugin"
                                            value={(*form_name).clone()}
                                            oninput={Callback::from({
                                                let form_name = form_name.clone();
                                                move |e: InputEvent| {
                                                    if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                                                        form_name.set(input.value());
                                                    }
                                                }
                                            })}
                                            required=true
                                        />
                                        <small class="form-help">{"Unique identifier for the plugin (lowercase, hyphens only)"}</small>
                                    </div>
                                    
                                    <div class="form-group">
                                        <label class="form-label required">{"Display Name"}</label>
                                        <input
                                            type="text"
                                            class="form-input"
                                            placeholder="e.g., My Awesome Plugin"
                                            value={(*form_display_name).clone()}
                                            oninput={Callback::from({
                                                let form_display_name = form_display_name.clone();
                                                move |e: InputEvent| {
                                                    if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                                                        form_display_name.set(input.value());
                                                    }
                                                }
                                            })}
                                            required=true
                                        />
                                    </div>
                                </div>
                                
                                <div class="form-group">
                                    <label class="form-label">{"Description"}</label>
                                    <textarea
                                        class="form-textarea"
                                        placeholder="Brief description of what this plugin does..."
                                        rows="3"
                                        value={(*form_description).clone()}
                                        oninput={Callback::from({
                                            let form_description = form_description.clone();
                                            move |e: InputEvent| {
                                                if let Some(textarea) = e.target().and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok()) {
                                                    form_description.set(textarea.value());
                                                }
                                            }
                                        })}
                                    />
                                </div>
                                
                                <div class="form-row">
                                    <div class="form-group">
                                        <label class="form-label required">{"Version"}</label>
                                        <input
                                            type="text"
                                            class="form-input"
                                            placeholder="e.g., 1.0.0"
                                            value={(*form_version).clone()}
                                            oninput={Callback::from({
                                                let form_version = form_version.clone();
                                                move |e: InputEvent| {
                                                    if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                                                        form_version.set(input.value());
                                                    }
                                                }
                                            })}
                                            required=true
                                        />
                                    </div>
                                    
                                    <div class="form-group">
                                        <label class="form-label">{"License"}</label>
                                        <input
                                            type="text"
                                            class="form-input"
                                            placeholder="e.g., MIT, GPL-3.0"
                                            value={(*form_license).clone()}
                                            oninput={Callback::from({
                                                let form_license = form_license.clone();
                                                move |e: InputEvent| {
                                                    if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                                                        form_license.set(input.value());
                                                    }
                                                }
                                            })}
                                        />
                                    </div>
                                </div>
                                
                                <div class="form-row">
                                    <div class="form-group">
                                        <label class="form-label">{"Author"}</label>
                                        <input
                                            type="text"
                                            class="form-input"
                                            placeholder="Plugin author name"
                                            value={(*form_author).clone()}
                                            oninput={Callback::from({
                                                let form_author = form_author.clone();
                                                move |e: InputEvent| {
                                                    if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                                                        form_author.set(input.value());
                                                    }
                                                }
                                            })}
                                        />
                                    </div>
                                    
                                    <div class="form-group">
                                        <label class="form-label">{"Author Email"}</label>
                                        <input
                                            type="email"
                                            class="form-input"
                                            placeholder="author@example.com"
                                            value={(*form_author_email).clone()}
                                            oninput={Callback::from({
                                                let form_author_email = form_author_email.clone();
                                                move |e: InputEvent| {
                                                    if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                                                        form_author_email.set(input.value());
                                                    }
                                                }
                                            })}
                                        />
                                    </div>
                                </div>
                                
                                <div class="form-row">
                                    <div class="form-group">
                                        <label class="form-label">{"Homepage URL"}</label>
                                        <input
                                            type="url"
                                            class="form-input"
                                            placeholder="https://example.com/plugin"
                                            value={(*form_homepage_url).clone()}
                                            oninput={Callback::from({
                                                let form_homepage_url = form_homepage_url.clone();
                                                move |e: InputEvent| {
                                                    if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                                                        form_homepage_url.set(input.value());
                                                    }
                                                }
                                            })}
                                        />
                                    </div>
                                    
                                    <div class="form-group">
                                        <label class="form-label">{"Repository URL"}</label>
                                        <input
                                            type="url"
                                            class="form-input"
                                            placeholder="https://github.com/user/plugin"
                                            value={(*form_repository_url).clone()}
                                            oninput={Callback::from({
                                                let form_repository_url = form_repository_url.clone();
                                                move |e: InputEvent| {
                                                    if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                                                        form_repository_url.set(input.value());
                                                    }
                                                }
                                            })}
                                        />
                                    </div>
                                </div>
                                
                                <div class="modal-actions">
                                    <button type="button" class="btn btn-secondary" onclick={close_modal.clone()}>
                                        {"Cancel"}
                                    </button>
                                    <button type="submit" class="btn btn-primary" disabled={*uploading}>
                                        if *uploading {
                                            <span class="btn-spinner"></span>
                                            {"Creating..."}
                                        } else {
                                            {"Create Plugin"}
                                        }
                                    </button>
                                </div>
                            </form>
                        },
                        
                        PluginInstallMethod::ZipUpload => html! {
                            <form onsubmit={handle_zip_submit}>
                                <div class="upload-area">
                                    <div class="upload-icon">{"📦"}</div>
                                    <h3>{"Upload Plugin ZIP"}</h3>
                                    <p>{"Select a ZIP file containing your plugin"}</p>
                                    
                                    <input
                                        type="file"
                                        id="plugin-zip"
                                        class="file-input"
                                        accept=".zip"
                                        onchange={handle_file_change}
                                    />
                                    <label for="plugin-zip" class="file-label">
                                        if let Some(file) = selected_file.as_ref() {
                                            {format!("Selected: {}", file.name())}
                                        } else {
                                            {"Choose ZIP File"}
                                        }
                                    </label>
                                    
                                    <div class="upload-requirements">
                                        <h4>{"Requirements:"}</h4>
                                        <ul>
                                            <li>{"ZIP file containing plugin source code"}</li>
                                            <li>{"Must include plugin.json manifest file"}</li>
                                            <li>{"Maximum file size: 50MB"}</li>
                                        </ul>
                                    </div>
                                </div>
                                
                                <div class="modal-actions">
                                    <button type="button" class="btn btn-secondary" onclick={close_modal.clone()}>
                                        {"Cancel"}
                                    </button>
                                    <button type="submit" class="btn btn-primary" disabled={*uploading || selected_file.is_none()}>
                                        if *uploading {
                                            <span class="btn-spinner"></span>
                                            {"Uploading..."}
                                        } else {
                                            {"Install Plugin"}
                                        }
                                    </button>
                                </div>
                            </form>
                        },
                        
                        PluginInstallMethod::Repository => html! {
                            <form onsubmit={handle_repo_submit}>
                                <div class="form-group">
                                    <label class="form-label required">{"Repository URL"}</label>
                                    <input
                                        type="url"
                                        class="form-input"
                                        placeholder="https://github.com/user/plugin.git"
                                        value={(*repo_url).clone()}
                                        oninput={Callback::from({
                                            let repo_url = repo_url.clone();
                                            move |e: InputEvent| {
                                                if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                                                    repo_url.set(input.value());
                                                }
                                            }
                                        })}
                                        required=true
                                    />
                                    <small class="form-help">{"Git repository URL containing the plugin"}</small>
                                </div>
                                
                                <div class="form-group">
                                    <label class="form-label">{"Branch"}</label>
                                    <input
                                        type="text"
                                        class="form-input"
                                        placeholder="main"
                                        value={(*repo_branch).clone()}
                                        oninput={Callback::from({
                                            let repo_branch = repo_branch.clone();
                                            move |e: InputEvent| {
                                                if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                                                    repo_branch.set(input.value());
                                                }
                                            }
                                        })}
                                    />
                                    <small class="form-help">{"Branch to clone (default: main)"}</small>
                                </div>
                                
                                <div class="repo-requirements">
                                    <h4>{"Requirements:"}</h4>
                                    <ul>
                                        <li>{"Public Git repository"}</li>
                                        <li>{"Must contain plugin.json manifest"}</li>
                                        <li>{"Repository must be accessible"}</li>
                                    </ul>
                                </div>
                                
                                <div class="modal-actions">
                                    <button type="button" class="btn btn-secondary" onclick={close_modal.clone()}>
                                        {"Cancel"}
                                    </button>
                                    <button type="submit" class="btn btn-primary" disabled={*uploading || repo_url.is_empty()}>
                                        if *uploading {
                                            <span class="btn-spinner"></span>
                                            {"Cloning..."}
                                        } else {
                                            {"Install Plugin"}
                                        }
                                    </button>
                                </div>
                            </form>
                        }
                    }}
                            </div>
                        },
                        ModalTab::Documentation => html! {
                            <div class="tab-content documentation-tab">
                                <div class="documentation-section">
                                    <h3>{"Plugin Architecture Guide"}</h3>
                                    <p>{"Complete system overview and technical specifications"}</p>
                                    
                                    <div class="doc-content">
                                        <h4>{"Core Components"}</h4>
                                        <ul>
                                            <li><strong>{"Plugin Manifest"}</strong>{" - "}<code>{"plugin.json"}</code>{" metadata and configuration"}</li>
                                            <li><strong>{"Entry Point"}</strong>{" - "}<code>{"src/lib.rs"}</code>{" main plugin code"}</li>
                                            <li><strong>{"Hook System"}</strong>{" - Event-driven plugin execution"}</li>
                                            <li><strong>{"Security Layer"}</strong>{" - Sandboxing and validation"}</li>
                                        </ul>
                                        
                                        <h4>{"Plugin Capabilities"}</h4>
                                        <div class="capability-grid">
                                            <div class="capability-item">
                                                <strong>{"Content Management"}</strong>
                                                <ul>
                                                    <li><code>{"content_filter"}</code>{" - Modify content before display"}</li>
                                                    <li><code>{"content_save"}</code>{" - Process content before saving"}</li>
                                                    <li><code>{"media_handler"}</code>{" - Handle media files"}</li>
                                                </ul>
                                            </div>
                                            <div class="capability-item">
                                                <strong>{"Administration"}</strong>
                                                <ul>
                                                    <li><code>{"admin_menu"}</code>{" - Add admin menu items"}</li>
                                                    <li><code>{"admin_dashboard"}</code>{" - Add dashboard widgets"}</li>
                                                    <li><code>{"user_management"}</code>{" - Manage users"}</li>
                                                </ul>
                                            </div>
                                            <div class="capability-item">
                                                <strong>{"API & Integration"}</strong>
                                                <ul>
                                                    <li><code>{"api_endpoint"}</code>{" - Provide API endpoints"}</li>
                                                    <li><code>{"webhook_handler"}</code>{" - Handle webhooks"}</li>
                                                    <li><code>{"external_service"}</code>{" - External integrations"}</li>
                                                </ul>
                                            </div>
                                        </div>
                                        
                                        <h4>{"Hook System"}</h4>
                                        <p>{"Plugins can register hooks to respond to CMS events:"}</p>
                                        <ul>
                                            <li><strong>{"Content Hooks:"}</strong>{" content_render, content_save, content_delete"}</li>
                                            <li><strong>{"User Hooks:"}</strong>{" user_login, user_logout, user_register"}</li>
                                            <li><strong>{"Admin Hooks:"}</strong>{" admin_init, admin_menu, admin_dashboard"}</li>
                                            <li><strong>{"System Hooks:"}</strong>{" system_startup, plugin_activate"}</li>
                                        </ul>
                                        
                                        <h4>{"Security Model"}</h4>
                                        <div class="security-info">
                                            <p><strong>{"Merkle Tree Verification:"}</strong>{" Cryptographic integrity checking"}</p>
                                            <p><strong>{"Digital Signatures:"}</strong>{" Plugin signing and verification"}</p>
                                            <p><strong>{"Sandboxing:"}</strong>{" Isolated plugin execution"}</p>
                                            <p><strong>{"Permissions:"}</strong>{" Granular access control"}</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        },
                        ModalTab::Examples => html! {
                            <div class="tab-content examples-tab">
                                <div class="examples-section">
                                    <h3>{"Plugin Templates & Examples"}</h3>
                                    <p>{"Get started with plugin development using our templates and examples"}</p>
                                    
                                    <div class="template-download">
                                        <div class="template-card">
                                            <h4>{"Base Plugin Template"}</h4>
                                            <p>{"A complete, functional plugin template with all the essentials"}</p>
                                            <div class="template-features">
                                                <span class="feature-tag">{"Rust Project"}</span>
                                                <span class="feature-tag">{"Complete Manifest"}</span>
                                                <span class="feature-tag">{"Hook Examples"}</span>
                                                <span class="feature-tag">{"Documentation"}</span>
                                            </div>
                                            <button class="btn btn-primary" onclick={download_template}>
                                                {"Download Template ZIP"}
                                            </button>
                                        </div>
                                    </div>
                                    
                                    <div class="manifest-example">
                                        <h4>{"Plugin Manifest Example"}</h4>
                                        <pre class="code-block">{r#"{
  "name": "my-awesome-plugin",
  "display_name": "My Awesome Plugin",
  "version": "1.0.0",
  "description": "A sample plugin",
  "author": "Your Name",
  "capabilities": [
    "content_filter",
    "admin_menu"
  ],
  "hooks": [
    "content_render",
    "admin_init"
  ],
  "config_schema": {
    "type": "object",
    "properties": {
      "enabled": {
        "type": "boolean",
        "default": true
      }
    }
  }
}"#}</pre>
                                    </div>
                                    
                                    <div class="plugin-code-example">
                                        <h4>{"Basic Plugin Code"}</h4>
                                        <pre class="code-block">{r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyPlugin {
    config: PluginConfig,
}

impl MyPlugin {
    pub fn new() -> Self {
        Self {
            config: PluginConfig::default(),
        }
    }
    
    pub fn on_content_render(&self, content: &str) -> String {
        if self.config.enabled {
            format!("{}\n<!-- Enhanced by MyPlugin -->", content)
        } else {
            content.to_string()
        }
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> *mut MyPlugin {
    Box::into_raw(Box::new(MyPlugin::new()))
}"#}</pre>
                                    </div>
                                    
                                    <div class="development-steps">
                                        <h4>{"Development Steps"}</h4>
                                        <ol>
                                            <li>{"Download the base template"}</li>
                                            <li>{"Customize plugin.json with your details"}</li>
                                            <li>{"Implement your functionality in src/lib.rs"}</li>
                                            <li>{"Test with "}<code>{"cargo test"}</code></li>
                                            <li>{"Build with "}<code>{"cargo build --release"}</code></li>
                                            <li>{"Create ZIP and upload to CMS"}</li>
                                        </ol>
                                    </div>
                                </div>
                            </div>
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
