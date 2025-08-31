use yew::prelude::*;
use wasm_bindgen::JsCast;
use crate::services::api_service::{
    get_system_info, SystemInfo, get_backups, get_data_snapshot, create_backup,
    BackupInfo, DataSnapshot, BackupRequest, get_settings, Setting, update_settings, SettingData
};

#[derive(Clone, PartialEq, Debug)]
pub struct SiteSettings {
    pub site_title: String,
    pub site_description: String,
    pub site_url: String,
    pub admin_email: String,
    pub posts_per_page: i32,
    pub allow_comments: bool,
    pub moderate_comments: bool,
    pub login_button_visible: bool,
    pub theme: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct EmailSettings {
    pub smtp_server: String,
    pub smtp_port: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
    pub base_url: String,
}

#[function_component(SystemSettings)]
pub fn system_settings() -> Html {
    // State management for all tabs
    let active_tab = use_state(|| "site".to_string());
    let loading = use_state(|| false);
    let error_message = use_state(|| None::<String>);
    let system_info = use_state(|| None::<SystemInfo>);
    let backups_list = use_state(|| None::<Vec<BackupInfo>>);
    let data_snapshot = use_state(|| None::<DataSnapshot>);
    let settings_list = use_state(|| None::<Vec<Setting>>);
    let saving = use_state(|| false);
    let save_message = use_state(|| None::<String>);
    
    // Site settings state
    let site_settings = use_state(|| SiteSettings {
        site_title: "My Rust CMS".to_string(),
        site_description: "A modern content management system built with Rust".to_string(),
        site_url: "http://localhost:8080".to_string(),
        admin_email: "admin@example.com".to_string(),
        posts_per_page: 10,
        allow_comments: true,
        moderate_comments: true,
        login_button_visible: true,
        theme: "Modern".to_string(),
    });
    
    // Email settings state
    let email_settings = use_state(|| EmailSettings {
        smtp_server: "".to_string(),
        smtp_port: "587".to_string(),
        smtp_username: "".to_string(),
        smtp_password: "".to_string(),
        from_email: "".to_string(),
        from_name: "CMS System".to_string(),
        base_url: "http://localhost:3000".to_string(),
    });
    
    // Track if email settings have been loaded
    let email_settings_loaded = use_state(|| false);
    
    // API Callbacks for each tab
    let load_system_info = {
        let loading = loading.clone();
        let error_message = error_message.clone();
        let system_info = system_info.clone();
        
        Callback::from(move |_| {
            let loading = loading.clone();
            let error_message = error_message.clone();
            let system_info = system_info.clone();
            
            error_message.set(None);
            loading.set(true);
            
            wasm_bindgen_futures::spawn_local(async move {
                match get_system_info().await {
                    Ok(info) => {
                        system_info.set(Some(info));
                        web_sys::console::log_1(&"✅ System info loaded successfully".into());
                    },
                    Err(e) => {
                        error_message.set(Some(format!("Unable to load system info: {}", e)));
                        web_sys::console::warn_1(&format!("⚠️ System info API error: {}", e).into());
                    }
                }
                loading.set(false);
            });
        })
    };

    let load_backups = {
        let loading = loading.clone();
        let error_message = error_message.clone();
        let backups_list = backups_list.clone();
        
        Callback::from(move |_| {
            let loading = loading.clone();
            let error_message = error_message.clone();
            let backups_list = backups_list.clone();
            
            error_message.set(None);
            loading.set(true);
            
            wasm_bindgen_futures::spawn_local(async move {
                match get_backups().await {
                    Ok(backups) => {
                        backups_list.set(Some(backups));
                        web_sys::console::log_1(&"✅ Backups loaded successfully".into());
                    },
                    Err(e) => {
                        error_message.set(Some(format!("Unable to load backups: {}", e)));
                        web_sys::console::warn_1(&format!("⚠️ Backups API error: {}", e).into());
                    }
                }
                loading.set(false);
            });
        })
    };

    let load_data_snapshot = {
        let loading = loading.clone();
        let error_message = error_message.clone();
        let data_snapshot = data_snapshot.clone();
        
        Callback::from(move |_| {
            let loading = loading.clone();
            let error_message = error_message.clone();
            let data_snapshot = data_snapshot.clone();
            
            error_message.set(None);
            loading.set(true);
            
            wasm_bindgen_futures::spawn_local(async move {
                match get_data_snapshot().await {
                    Ok(snapshot) => {
                        data_snapshot.set(Some(snapshot));
                        web_sys::console::log_1(&"✅ Data snapshot loaded successfully".into());
                    },
                    Err(e) => {
                        error_message.set(Some(format!("Unable to load data snapshot: {}", e)));
                        web_sys::console::warn_1(&format!("⚠️ Data snapshot API error: {}", e).into());
                    }
                }
                loading.set(false);
            });
        })
    };

    let create_backup_handler = {
        let loading = loading.clone();
        let error_message = error_message.clone();
        let backups_list = backups_list.clone();
        
        Callback::from(move |_| {
            let loading = loading.clone();
            let error_message = error_message.clone();
            let backups_list = backups_list.clone();
            
            error_message.set(None);
            loading.set(true);
            
            wasm_bindgen_futures::spawn_local(async move {
                let backup_request = BackupRequest {
                    backup_type: "full".to_string(),
                    description: Some("Manual backup created from admin interface".to_string()),
                };
                
                match create_backup(backup_request).await {
                    Ok(_) => {
                        web_sys::console::log_1(&"✅ Backup created successfully".into());
                        // Refresh backups list
                        match get_backups().await {
                            Ok(backups) => backups_list.set(Some(backups)),
                            Err(e) => web_sys::console::warn_1(&format!("⚠️ Error refreshing backups: {}", e).into()),
                        }
                    },
                    Err(e) => {
                        error_message.set(Some(format!("Failed to create backup: {}", e)));
                        web_sys::console::warn_1(&format!("⚠️ Backup creation error: {}", e).into());
                    }
                }
                loading.set(false);
            });
        })
    };

    let load_settings = {
        let loading = loading.clone();
        let error_message = error_message.clone();
        let settings_list = settings_list.clone();
        
        Callback::from(move |_| {
            let loading = loading.clone();
            let error_message = error_message.clone();
            let settings_list = settings_list.clone();
            
            error_message.set(None);
            loading.set(true);
            
            wasm_bindgen_futures::spawn_local(async move {
                match get_settings(None).await {
                    Ok(settings) => {
                        settings_list.set(Some(settings));
                        web_sys::console::log_1(&"✅ Settings loaded successfully".into());
                    },
                    Err(e) => {
                        error_message.set(Some(format!("Unable to load settings: {}", e)));
                        web_sys::console::warn_1(&format!("⚠️ Settings API error: {}", e).into());
                    }
                }
                loading.set(false);
            });
        })
    };

    // Site settings save callback
    let save_site_settings = {
        let site_settings = site_settings.clone();
        let saving = saving.clone();
        let save_message = save_message.clone();
        
        Callback::from(move |_| {
            let settings = (*site_settings).clone();
            let saving = saving.clone();
            let save_message = save_message.clone();
            
            saving.set(true);
            save_message.set(None);
            
            wasm_bindgen_futures::spawn_local(async move {
                web_sys::console::log_1(&format!("Saving site settings: {:?}", settings).into());
                
                // Convert site settings to API format
                let settings_data = vec![
                    SettingData {
                        key: "site_title".to_string(),
                        value: settings.site_title,
                        setting_type: "site".to_string(),
                        description: Some("Site title displayed in navigation".to_string()),
                    },
                    SettingData {
                        key: "site_description".to_string(),
                        value: settings.site_description,
                        setting_type: "site".to_string(),
                        description: Some("Brief description of the site".to_string()),
                    },
                    SettingData {
                        key: "site_url".to_string(),
                        value: settings.site_url,
                        setting_type: "site".to_string(),
                        description: Some("Base URL of the site".to_string()),
                    },
                    SettingData {
                        key: "admin_email".to_string(),
                        value: settings.admin_email,
                        setting_type: "site".to_string(),
                        description: Some("Administrator email address".to_string()),
                    },
                    SettingData {
                        key: "posts_per_page".to_string(),
                        value: settings.posts_per_page.to_string(),
                        setting_type: "site".to_string(),
                        description: Some("Number of posts to display per page".to_string()),
                    },
                    SettingData {
                        key: "allow_comments".to_string(),
                        value: settings.allow_comments.to_string(),
                        setting_type: "site".to_string(),
                        description: Some("Allow comments on posts".to_string()),
                    },
                    SettingData {
                        key: "moderate_comments".to_string(),
                        value: settings.moderate_comments.to_string(),
                        setting_type: "site".to_string(),
                        description: Some("Require comment moderation".to_string()),
                    },
                    SettingData {
                        key: "login_button_visible".to_string(),
                        value: settings.login_button_visible.to_string(),
                        setting_type: "site".to_string(),
                        description: Some("Show login button in public navigation".to_string()),
                    },
                    SettingData {
                        key: "theme".to_string(),
                        value: settings.theme,
                        setting_type: "site".to_string(),
                        description: Some("Site theme".to_string()),
                    },
                ];
                
                match update_settings(settings_data).await {
                    Ok(_) => {
                        saving.set(false);
                        save_message.set(Some("Settings saved successfully!".to_string()));
                        web_sys::console::log_1(&"Settings saved successfully".into());
                    }
                    Err(e) => {
                        saving.set(false);
                        save_message.set(Some(format!("Error saving settings: {}", e)));
                        web_sys::console::error_1(&format!("Failed to save settings: {}", e).into());
                    }
                }
                
                // Clear message after 3 seconds
                let save_message = save_message.clone();
                gloo_timers::future::TimeoutFuture::new(3000).await;
                save_message.set(None);
            });
        })
    };

    // Email settings save callback
    let save_email_settings = {
        let email_settings = email_settings.clone();
        let saving = saving.clone();
        let save_message = save_message.clone();
        
        Callback::from(move |_| {
            let settings = (*email_settings).clone();
            let saving = saving.clone();
            let save_message = save_message.clone();
            
            saving.set(true);
            save_message.set(None);
            
            wasm_bindgen_futures::spawn_local(async move {
                web_sys::console::log_1(&format!("Saving email settings: {:?}", settings).into());
                
                let settings_data = vec![
                    SettingData {
                        key: "smtp_server".to_string(),
                        value: settings.smtp_server,
                        setting_type: "email".to_string(),
                        description: Some("SMTP server hostname".to_string()),
                    },
                    SettingData {
                        key: "smtp_port".to_string(),
                        value: settings.smtp_port,
                        setting_type: "email".to_string(),
                        description: Some("SMTP server port".to_string()),
                    },
                    SettingData {
                        key: "smtp_username".to_string(),
                        value: settings.smtp_username,
                        setting_type: "email".to_string(),
                        description: Some("SMTP username".to_string()),
                    },
                    SettingData {
                        key: "smtp_password".to_string(),
                        value: settings.smtp_password,
                        setting_type: "email".to_string(),
                        description: Some("SMTP password".to_string()),
                    },
                    SettingData {
                        key: "from_email".to_string(),
                        value: settings.from_email,
                        setting_type: "email".to_string(),
                        description: Some("From email address".to_string()),
                    },
                    SettingData {
                        key: "from_name".to_string(),
                        value: settings.from_name,
                        setting_type: "email".to_string(),
                        description: Some("From name".to_string()),
                    },
                    SettingData {
                        key: "base_url".to_string(),
                        value: settings.base_url,
                        setting_type: "email".to_string(),
                        description: Some("Base URL for email links".to_string()),
                    },
                ];
                
                match update_settings(settings_data).await {
                    Ok(_) => {
                        saving.set(false);
                        save_message.set(Some("Email settings saved successfully!".to_string()));
                        web_sys::console::log_1(&"Email settings saved successfully".into());
                    }
                    Err(e) => {
                        saving.set(false);
                        save_message.set(Some(format!("Error saving email settings: {}", e)));
                        web_sys::console::error_1(&format!("Failed to save email settings: {}", e).into());
                    }
                }
                
                // Clear message after 3 seconds
                let save_message = save_message.clone();
                gloo_timers::future::TimeoutFuture::new(3000).await;
                save_message.set(None);
            });
        })
    };

    // Function to load site settings
    let do_load_site_settings = {
        let loading = loading.clone();
        let error_message = error_message.clone();
        let site_settings = site_settings.clone();
        
        move || {
            let loading = loading.clone();
            let error_message = error_message.clone();
            let site_settings = site_settings.clone();
            
            // Prevent multiple simultaneous loads
            if *loading {
                return;
            }
            
            error_message.set(None);
            loading.set(true);
            
            wasm_bindgen_futures::spawn_local(async move {
                match get_settings(Some("site")).await {
                    Ok(settings) => {
                        // Parse the settings and update the site_settings state
                        let mut site_config = (*site_settings).clone();
                        
                        for setting in settings {
                            match setting.setting_key.as_str() {
                                "site_title" => site_config.site_title = setting.setting_value.unwrap_or_default(),
                                "site_description" => site_config.site_description = setting.setting_value.unwrap_or_default(),
                                "site_url" => site_config.site_url = setting.setting_value.unwrap_or_default(),
                                "admin_email" => site_config.admin_email = setting.setting_value.unwrap_or_default(),
                                "posts_per_page" => {
                                    if let Some(value) = setting.setting_value {
                                        if let Ok(parsed) = value.parse::<i32>() {
                                            site_config.posts_per_page = parsed;
                                        }
                                    }
                                },
                                "allow_comments" => {
                                    if let Some(value) = setting.setting_value {
                                        site_config.allow_comments = value.parse::<bool>().unwrap_or(true);
                                    }
                                },
                                "moderate_comments" => {
                                    if let Some(value) = setting.setting_value {
                                        site_config.moderate_comments = value.parse::<bool>().unwrap_or(true);
                                    }
                                },
                                "login_button_visible" => {
                                    if let Some(value) = setting.setting_value {
                                        site_config.login_button_visible = value.parse::<bool>().unwrap_or(true);
                                    }
                                },
                                "theme" => site_config.theme = setting.setting_value.unwrap_or_default(),
                                _ => {}
                            }
                        }
                        
                        site_settings.set(site_config);
                        web_sys::console::log_1(&"✅ Site settings loaded successfully".into());
                    },
                    Err(e) => {
                        error_message.set(Some(format!("Unable to load site settings: {}", e)));
                        web_sys::console::warn_1(&format!("⚠️ Site settings API error: {}", e).into());
                    }
                }
                loading.set(false);
            });
        }
    };

    // Function to load email settings
    let do_load_email_settings = {
        let loading = loading.clone();
        let error_message = error_message.clone();
        let email_settings = email_settings.clone();
        let email_settings_loaded = email_settings_loaded.clone();
        
        move || {
            let loading = loading.clone();
            let error_message = error_message.clone();
            let email_settings = email_settings.clone();
            let email_settings_loaded = email_settings_loaded.clone();
            
            // Prevent multiple simultaneous loads
            if *loading {
                return;
            }
            
            error_message.set(None);
            loading.set(true);
            
            wasm_bindgen_futures::spawn_local(async move {
                match get_settings(Some("email")).await {
                    Ok(settings) => {
                        // Parse the settings and update the email_settings state
                        let mut email_config = (*email_settings).clone();
                        
                        for setting in settings {
                            match setting.setting_key.as_str() {
                                "smtp_server" => email_config.smtp_server = setting.setting_value.unwrap_or_default(),
                                "smtp_port" => email_config.smtp_port = setting.setting_value.unwrap_or_default(),
                                "smtp_username" => email_config.smtp_username = setting.setting_value.unwrap_or_default(),
                                "smtp_password" => email_config.smtp_password = setting.setting_value.unwrap_or_default(),
                                "from_email" => email_config.from_email = setting.setting_value.unwrap_or_default(),
                                "from_name" => email_config.from_name = setting.setting_value.unwrap_or_default(),
                                "base_url" => email_config.base_url = setting.setting_value.unwrap_or_default(),
                                _ => {}
                            }
                        }
                        
                        email_settings.set(email_config);
                        email_settings_loaded.set(true);
                        web_sys::console::log_1(&"✅ Email settings loaded successfully".into());
                    },
                    Err(e) => {
                        error_message.set(Some(format!("Unable to load email settings: {}", e)));
                        web_sys::console::warn_1(&format!("⚠️ Email settings API error: {}", e).into());
                    }
                }
                loading.set(false);
            });
        }
    };

    // Load site settings callback for button
    let _load_site_settings = {
        let do_load = do_load_site_settings.clone();
        Callback::from(move |_: MouseEvent| {
            do_load();
        })
    };

    // Load email settings callback for button
    let load_email_settings = {
        let do_load = do_load_email_settings.clone();
        Callback::from(move |_: MouseEvent| {
            do_load();
        })
    };
    
    // Auto-load site settings when component mounts
    {
        let do_load_site_settings = do_load_site_settings.clone();
        
        use_effect_with_deps(move |_| {
            web_sys::console::log_1(&"Auto-loading site settings on component mount".into());
            do_load_site_settings();
            || () // cleanup function
        }, ());
    }

    // Auto-load email settings when switching to email tab
    {
        let active_tab = active_tab.clone();
        let email_settings_loaded = email_settings_loaded.clone();
        let do_load_email_settings = do_load_email_settings.clone();
        
        use_effect_with_deps(move |tab| {
            if tab.as_str() == "email" && !*email_settings_loaded {
                // Only auto-load if we haven't loaded email settings yet
                web_sys::console::log_1(&"Auto-loading email settings for email tab".into());
                do_load_email_settings();
            }
            || () // cleanup function
        }, active_tab.clone());
    }
    
    web_sys::console::log_1(&"Settings component with full API integration".into());

    html! {
        <div class="system-settings">
            <div class="page-header">
                <h1>{"Settings"}</h1>
                <p>{"Configure your CMS settings and preferences"}</p>
            </div>

            // Show any errors or save messages
            if let Some(error) = error_message.as_ref() {
                <div class="notification error">
                    <span>{error}</span>
                </div>
            }
            
            if let Some(message) = save_message.as_ref() {
                <div class="notification success">
                    <span>{message}</span>
                </div>
            }

            <div class="settings-content">
                <div class="settings-tabs">
                    <button 
                        class={if *active_tab == "site" { "tab-button active" } else { "tab-button" }}
                        onclick={let active_tab = active_tab.clone(); Callback::from(move |_| active_tab.set("site".to_string()))}
                    >
                        {"Site Configuration"}
                    </button>
                    <button 
                        class={if *active_tab == "email" { "tab-button active" } else { "tab-button" }}
                        onclick={let active_tab = active_tab.clone(); Callback::from(move |_| active_tab.set("email".to_string()))}
                    >
                        {"Email Settings"}
                    </button>
                    <button 
                        class={if *active_tab == "overview" { "tab-button active" } else { "tab-button" }}
                        onclick={let active_tab = active_tab.clone(); Callback::from(move |_| active_tab.set("overview".to_string()))}
                    >
                        {"System Overview"}
                    </button>
                    <button 
                        class={if *active_tab == "settings" { "tab-button active" } else { "tab-button" }}
                        onclick={let active_tab = active_tab.clone(); Callback::from(move |_| active_tab.set("settings".to_string()))}
                    >
                        {"System Settings"}
                    </button>
                    <button 
                        class={if *active_tab == "backups" { "tab-button active" } else { "tab-button" }}
                        onclick={let active_tab = active_tab.clone(); Callback::from(move |_| active_tab.set("backups".to_string()))}
                    >
                        {"Backups"}
                    </button>
                    <button 
                        class={if *active_tab == "snapshot" { "tab-button active" } else { "tab-button" }}
                        onclick={let active_tab = active_tab.clone(); Callback::from(move |_| active_tab.set("snapshot".to_string()))}
                    >
                        {"Data Snapshot"}
                    </button>
                </div>

                <div class="tab-content">
                    {match active_tab.as_str() {
                        "site" => html! {
                            <div class="settings-section">
                                <h3>{"Site Configuration"}</h3>
                                <div class="form-grid">
                                    <div class="form-group">
                                        <label>{"Site Title"}</label>
                                        <input 
                                            type="text" 
                                            value={site_settings.site_title.clone()}
                                            onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let mut settings = (*site_settings).clone();
                                                settings.site_title = target.value();
                                                site_settings.set(settings);
                                            })}
                                            placeholder="My Website"
                                        />
                                    </div>
                                    
                                    <div class="form-group">
                                        <label>{"Site Description"}</label>
                                        <textarea 
                                            value={site_settings.site_description.clone()}
                                            onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlTextAreaElement>();
                                                let mut settings = (*site_settings).clone();
                                                settings.site_description = target.value();
                                                site_settings.set(settings);
                                            })}
                                            placeholder="A brief description of your site"
                                            rows="3"
                                        />
                                    </div>
                                    
                                    <div class="form-group">
                                        <label>{"Site URL"}</label>
                                        <input 
                                            type="url" 
                                            value={site_settings.site_url.clone()}
                                            onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let mut settings = (*site_settings).clone();
                                                settings.site_url = target.value();
                                                site_settings.set(settings);
                                            })}
                                            placeholder="https://example.com"
                                        />
                                    </div>
                                    
                                    <div class="form-group">
                                        <label>{"Admin Email"}</label>
                                        <input 
                                            type="email" 
                                            value={site_settings.admin_email.clone()}
                                            onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let mut settings = (*site_settings).clone();
                                                settings.admin_email = target.value();
                                                site_settings.set(settings);
                                            })}
                                            placeholder="admin@example.com"
                                        />
                                    </div>
                                </div>

                                <h3>{"Content Settings"}</h3>
                                <div class="form-grid">
                                    <div class="form-group">
                                        <label>{"Posts per Page"}</label>
                                        <input 
                                            type="number" 
                                            value={site_settings.posts_per_page.to_string()}
                                            onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                if let Ok(value) = target.value().parse::<i32>() {
                                                    let mut settings = (*site_settings).clone();
                                                    settings.posts_per_page = value;
                                                    site_settings.set(settings);
                                                }
                                            })}
                                            min="1"
                                            max="100"
                                        />
                                    </div>
                                    
                                    <div class="form-group">
                                        <label>{"Theme"}</label>
                                        <select 
                                            value={site_settings.theme.clone()}
                                            onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                let mut settings = (*site_settings).clone();
                                                settings.theme = target.value();
                                                site_settings.set(settings);
                                            })}
                                        >
                                            <option value="default">{"Default"}</option>
                                            <option value="dark">{"Dark"}</option>
                                            <option value="minimal">{"Minimal"}</option>
                                            <option value="modern">{"Modern"}</option>
                                        </select>
                                    </div>
                                </div>

                                <h3>{"Comment Settings"}</h3>
                                <div class="form-grid">
                                    <div class="form-group checkbox-group">
                                        <label>
                                            <input 
                                                type="checkbox" 
                                                checked={site_settings.allow_comments}
                                                onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                    let mut settings = (*site_settings).clone();
                                                    settings.allow_comments = target.checked();
                                                    site_settings.set(settings);
                                                })}
                                            />
                                            {"Allow Comments"}
                                        </label>
                                    </div>
                                    
                                    <div class="form-group checkbox-group">
                                        <label>
                                            <input 
                                                type="checkbox" 
                                                checked={site_settings.moderate_comments}
                                                onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                    let mut settings = (*site_settings).clone();
                                                    settings.moderate_comments = target.checked();
                                                    site_settings.set(settings);
                                                })}
                                            />
                                            {"Moderate Comments"}
                                        </label>
                                    </div>
                                </div>

                                <h3>{"Navigation Settings"}</h3>
                                <div class="form-grid">
                                    <div class="form-group checkbox-group">
                                        <label>
                                            <input 
                                                type="checkbox" 
                                                checked={site_settings.login_button_visible}
                                                onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                    let mut settings = (*site_settings).clone();
                                                    settings.login_button_visible = target.checked();
                                                    site_settings.set(settings);
                                                })}
                                            />
                                            {"Show Login Button in Public Navigation"}
                                        </label>
                                    </div>
                                </div>

                                <div class="form-actions">
                                    <button 
                                        class="btn" 
                                        onclick={save_site_settings.clone()}
                                        disabled={*saving}
                                    >
                                        {if *saving { "Saving..." } else { "Save Settings" }}
                                    </button>
                                </div>
                            </div>
                        },
                        "email" => html! {
                            <div class="settings-section">
                                <div class="settings-header">
                                    <h3>{"Email Configuration"}</h3>
                                    <button 
                                        class="btn btn-secondary"
                                        onclick={load_email_settings.clone()}
                                        disabled={*loading}
                                    >
                                        {if *loading { "Loading..." } else { if *email_settings_loaded { "Reload Settings" } else { "Load Settings" } }}
                                    </button>
                                </div>
                                <p class="settings-description">{"Configure SMTP settings for sending emails including user verification and notifications."}</p>
                                
                                <div class="form-grid">
                                    <div class="form-group">
                                        <label>{"SMTP Server"}</label>
                                        <input 
                                            type="text" 
                                            value={email_settings.smtp_server.clone()}
                                            onchange={let email_settings = email_settings.clone(); Callback::from(move |e: Event| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let mut settings = (*email_settings).clone();
                                                settings.smtp_server = target.value();
                                                email_settings.set(settings);
                                            })}
                                            placeholder="smtp.gmail.com"
                                        />
                                        <small class="form-help">{"The hostname of your SMTP server"}</small>
                                    </div>
                                    
                                    <div class="form-group">
                                        <label>{"SMTP Port"}</label>
                                        <input 
                                            type="text" 
                                            value={email_settings.smtp_port.clone()}
                                            onchange={let email_settings = email_settings.clone(); Callback::from(move |e: Event| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let mut settings = (*email_settings).clone();
                                                settings.smtp_port = target.value();
                                                email_settings.set(settings);
                                            })}
                                            placeholder="587"
                                        />
                                        <small class="form-help">{"Usually 587 for TLS or 465 for SSL"}</small>
                                    </div>
                                    
                                    <div class="form-group">
                                        <label>{"SMTP Username"}</label>
                                        <input 
                                            type="text" 
                                            value={email_settings.smtp_username.clone()}
                                            onchange={let email_settings = email_settings.clone(); Callback::from(move |e: Event| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let mut settings = (*email_settings).clone();
                                                settings.smtp_username = target.value();
                                                email_settings.set(settings);
                                            })}
                                            placeholder="your-email@gmail.com"
                                        />
                                        <small class="form-help">{"Your email account username"}</small>
                                    </div>
                                    
                                    <div class="form-group">
                                        <label>{"SMTP Password"}</label>
                                        <input 
                                            type="password" 
                                            value={email_settings.smtp_password.clone()}
                                            onchange={let email_settings = email_settings.clone(); Callback::from(move |e: Event| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let mut settings = (*email_settings).clone();
                                                settings.smtp_password = target.value();
                                                email_settings.set(settings);
                                            })}
                                            placeholder="your-app-password"
                                        />
                                        <small class="form-help">{"Use an app-specific password for Gmail"}</small>
                                    </div>
                                    
                                    <div class="form-group">
                                        <label>{"From Email"}</label>
                                        <input 
                                            type="email" 
                                            value={email_settings.from_email.clone()}
                                            onchange={let email_settings = email_settings.clone(); Callback::from(move |e: Event| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let mut settings = (*email_settings).clone();
                                                settings.from_email = target.value();
                                                email_settings.set(settings);
                                            })}
                                            placeholder="noreply@yourdomain.com"
                                        />
                                        <small class="form-help">{"Email address that appears as sender"}</small>
                                    </div>
                                    
                                    <div class="form-group">
                                        <label>{"From Name"}</label>
                                        <input 
                                            type="text" 
                                            value={email_settings.from_name.clone()}
                                            onchange={let email_settings = email_settings.clone(); Callback::from(move |e: Event| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let mut settings = (*email_settings).clone();
                                                settings.from_name = target.value();
                                                email_settings.set(settings);
                                            })}
                                            placeholder="CMS System"
                                        />
                                        <small class="form-help">{"Display name for outgoing emails"}</small>
                                    </div>
                                    
                                    <div class="form-group">
                                        <label>{"Base URL"}</label>
                                        <input 
                                            type="url" 
                                            value={email_settings.base_url.clone()}
                                            onchange={let email_settings = email_settings.clone(); Callback::from(move |e: Event| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let mut settings = (*email_settings).clone();
                                                settings.base_url = target.value();
                                                email_settings.set(settings);
                                            })}
                                            placeholder="https://yourdomain.com"
                                        />
                                        <small class="form-help">{"Base URL for verification links in emails"}</small>
                                    </div>
                                </div>

                                <div class="form-actions">
                                    <button 
                                        class="btn btn-primary" 
                                        onclick={save_email_settings.clone()}
                                        disabled={*saving}
                                    >
                                        {if *saving { "Saving..." } else { "Save Email Settings" }}
                                    </button>
                                </div>
                                
                                <div class="email-info">
                                    <h4>{"Setup Instructions"}</h4>
                                    <ul>
                                        <li>{"For Gmail, enable 2-factor authentication and create an App Password"}</li>
                                        <li>{"Use port 587 for TLS encryption (recommended)"}</li>
                                        <li>{"The Base URL should match your domain for proper email links"}</li>
                                        <li>{"Test the configuration by creating a new user account"}</li>
                                    </ul>
                                </div>
                            </div>
                        },
                        "overview" => html! {
                            <div class="overview-content">
                                <div class="overview-header">
                                    <h2>{"System Information"}</h2>
                                    <button 
                                        class="btn btn-primary"
                                        onclick={load_system_info}
                                        disabled={*loading}
                                    >
                                        {if *loading { "Loading..." } else { "Load System Info" }}
                                    </button>
                                </div>
                                
                                {if let Some(info) = system_info.as_ref() {
                                    html! {
                                        <div class="system-info-grid">
                                            <div class="info-card">
                                                <h4>{"System Overview"}</h4>
                                                <div class="info-item"><span class="label">{"Rust Version:"}</span><span class="value">{&info.rust_version}</span></div>
                                                <div class="info-item"><span class="label">{"Database Version:"}</span><span class="value">{&info.database_version}</span></div>
                                                <div class="info-item"><span class="label">{"Uptime:"}</span><span class="value">{&info.uptime}</span></div>
                                                <div class="info-item"><span class="label">{"Memory Usage:"}</span><span class="value">{&info.memory_usage}</span></div>
                                                <div class="info-item"><span class="label">{"CPU Usage:"}</span><span class="value">{&info.cpu_usage}</span></div>
                                                <div class="info-item"><span class="label">{"Disk Usage:"}</span><span class="value">{&info.disk_usage}</span></div>
                                            </div>
                                            <div class="info-card">
                                                <h4>{"Database Statistics"}</h4>
                                                <div class="info-item"><span class="label">{"Active Sessions:"}</span><span class="value">{info.active_sessions}</span></div>
                                                <div class="info-item"><span class="label">{"Total Posts:"}</span><span class="value">{info.total_posts}</span></div>
                                                <div class="info-item"><span class="label">{"Total Users:"}</span><span class="value">{info.total_users}</span></div>
                                                <div class="info-item"><span class="label">{"Total Media Files:"}</span><span class="value">{info.total_media}</span></div>
                                            </div>
                                        </div>
                                    }
                                } else if *loading {
                                    html! { <div class="loading-spinner">{"Loading system information..."}</div> }
                                } else {
                                    html! { 
                                        <div class="placeholder-content">
                                            <p>{"Click 'Load System Info' to fetch current system statistics."}</p>
                                        </div>
                                    }
                                }}
                            </div>
                        },
                        "settings" => html! {
                            <div class="settings-content">
                                <div class="settings-header">
                                    <h2>{"System Settings"}</h2>
                                    <button 
                                        class="btn btn-primary"
                                        onclick={load_settings}
                                        disabled={*loading}
                                    >
                                        {if *loading { "Loading..." } else { "Load Settings" }}
                                    </button>
                                </div>
                                
                                {if let Some(settings) = settings_list.as_ref() {
                                    if settings.is_empty() {
                                        html! { <div class="no-data">{"No system settings found."}</div> }
                                    } else {
                                        html! {
                                            <div class="settings-table">
                                                <table class="admin-table">
                                                    <thead>
                                                        <tr>
                                                            <th>{"Setting Key"}</th>
                                                            <th>{"Current Value"}</th>
                                                            <th>{"Setting Type"}</th>
                                                            <th>{"Description"}</th>
                                                            <th>{"Actions"}</th>
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        {for settings.iter().map(|setting| {
                                                            html! {
                                                                <tr key={setting.setting_key.clone()}>
                                                                    <td class="setting-key">{&setting.setting_key}</td>
                                                                    <td class="setting-value">{setting.setting_value.as_ref().unwrap_or(&"Not set".to_string())}</td>
                                                                    <td class="setting-type">{&setting.setting_type}</td>
                                                                    <td class="setting-description">{setting.description.as_ref().unwrap_or(&"No description".to_string())}</td>
                                                                    <td>
                                                                        <button class="btn btn-sm btn-primary">{"Edit"}</button>
                                                                    </td>
                                                                </tr>
                                                            }
                                                        })}
                                                    </tbody>
                                                </table>
                                            </div>
                                        }
                                    }
                                } else if *loading {
                                    html! { <div class="loading-spinner">{"Loading system settings..."}</div> }
                                } else {
                                    html! { 
                                        <div class="placeholder-content">
                                            <p>{"Click 'Load Settings' to view and manage system configuration."}</p>
                                        </div>
                                    }
                                }}
                            </div>
                        },
                        "backups" => html! {
                            <div class="backups-content">
                                <div class="backups-header">
                                    <h2>{"Backup Management"}</h2>
                                    <div class="backup-actions">
                                        <button 
                                            class="btn btn-success"
                                            onclick={create_backup_handler}
                                            disabled={*loading}
                                        >
                                            {if *loading { "Creating..." } else { "Create New Backup" }}
                                        </button>
                                        <button 
                                            class="btn btn-primary"
                                            onclick={load_backups}
                                            disabled={*loading}
                                        >
                                            {if *loading { "Loading..." } else { "Refresh Backups" }}
                                        </button>
                                    </div>
                                </div>
                                
                                {if let Some(backups) = backups_list.as_ref() {
                                    if backups.is_empty() {
                                        html! { <div class="no-data">{"No backups found. Create your first backup!"}</div> }
                                    } else {
                                        html! {
                                            <div class="backups-table">
                                                <table class="admin-table">
                                                    <thead>
                                                        <tr>
                                                            <th>{"Backup ID"}</th>
                                                            <th>{"Description"}</th>
                                                            <th>{"Created"}</th>
                                                            <th>{"Size"}</th>
                                                            <th>{"Actions"}</th>
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        {for backups.iter().map(|backup| {
                                                            html! {
                                                                <tr key={backup.id.clone()}>
                                                                    <td>{&backup.id}</td>
                                                                    <td>{backup.description.as_ref().unwrap_or(&"No description".to_string())}</td>
                                                                    <td>{&backup.created_at}</td>
                                                                    <td>{format!("{} bytes", backup.size)}</td>
                                                                    <td>
                                                                        <button class="btn btn-sm btn-primary">{"Restore"}</button>
                                                                        <button class="btn btn-sm btn-danger">{"Delete"}</button>
                                                                    </td>
                                                                </tr>
                                                            }
                                                        })}
                                                    </tbody>
                                                </table>
                                            </div>
                                        }
                                    }
                                } else if *loading {
                                    html! { <div class="loading-spinner">{"Loading backups..."}</div> }
                                } else {
                                    html! { 
                                        <div class="placeholder-content">
                                            <p>{"Click 'Refresh Backups' to load available backups."}</p>
                                        </div>
                                    }
                                }}
                            </div>
                        },
                        "snapshot" => html! {
                            <div class="snapshot-content">
                                <div class="snapshot-header">
                                    <h2>{"Data Snapshot & Integrity"}</h2>
                                    <button 
                                        class="btn btn-primary"
                                        onclick={load_data_snapshot}
                                        disabled={*loading}
                                    >
                                        {if *loading { "Loading..." } else { "Generate Data Snapshot" }}
                                    </button>
                                </div>
                                
                                {if let Some(snapshot) = data_snapshot.as_ref() {
                                    html! {
                                        <div class="snapshot-data">
                                            <div class="snapshot-overview">
                                                <h3>{"Data Integrity Overview"}</h3>
                                                <div class="snapshot-stats">
                                                    <div class="stat-item">
                                                        <span class="label">{"Total Rows:"}</span>
                                                        <span class="value">{snapshot.total_rows}</span>
                                                    </div>
                                                    <div class="stat-item">
                                                        <span class="label">{"Generated:"}</span>
                                                        <span class="value">{&snapshot.timestamp}</span>
                                                    </div>
                                                    <div class="stat-item">
                                                        <span class="label">{"Data Hash:"}</span>
                                                        <span class="value code">{&snapshot.data_hash}</span>
                                                    </div>
                                                    <div class="stat-item">
                                                        <span class="label">{"Integrity:"}</span>
                                                        <span class="value">{if snapshot.integrity_verified { "✅ Verified" } else { "❌ Failed" }}</span>
                                                    </div>
                                                </div>
                                            </div>
                                            
                                            <div class="tables-snapshot">
                                                <h4>{"Table Data Summary"}</h4>
                                                <div class="tables-grid">
                                                    {for snapshot.tables.iter().map(|table| {
                                                        html! {
                                                            <div key={table.table_name.clone()} class="table-card">
                                                                <h5>{&table.table_name}</h5>
                                                                <div class="table-stats">
                                                                    <div class="stat"><span>{"Rows:"}</span><span>{table.row_count}</span></div>
                                                                    <div class="stat"><span>{"Hash:"}</span><span class="code">{&table.table_hash}</span></div>
                                                                    <div class="stat">
                                                                        <span>{"Modified:"}</span>
                                                                        <span>{table.last_modified.as_ref().unwrap_or(&"Unknown".to_string())}</span>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        }
                                                    })}
                                                </div>
                                            </div>
                                        </div>
                                    }
                                } else if *loading {
                                    html! { <div class="loading-spinner">{"Generating data snapshot..."}</div> }
                                } else {
                                    html! { 
                                        <div class="placeholder-content">
                                            <p>{"Click 'Generate Data Snapshot' to analyze current data integrity."}</p>
                                        </div>
                                    }
                                }}
                            </div>
                        },
                        _ => html! { <div>{"Unknown tab"}</div> }
                    }}
                </div>
            </div>
        </div>
    }
}