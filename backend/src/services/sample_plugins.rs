use crate::services::plugin_interface::*;
use async_trait::async_trait;
use serde_json::json;

/// Analytics Plugin - Adds Google Analytics integration
pub struct AnalyticsPlugin {
    context: Option<PluginContext>,
}

impl AnalyticsPlugin {
    pub fn new() -> Self {
        Self { context: None }
    }
}

#[async_trait]
impl Plugin for AnalyticsPlugin {
    fn manifest(&self) -> PluginManifest {
        PluginManifest {
            name: "google-analytics".to_string(),
            display_name: "Google Analytics".to_string(),
            description: Some("Integrates Google Analytics tracking into your CMS".to_string()),
            version: "1.2.0".to_string(),
            author: Some("CMS Team".to_string()),
            author_email: Some("team@myrustcms.dev".to_string()),
            homepage_url: Some("https://myrustcms.dev/plugins/analytics".to_string()),
            repository_url: Some("https://github.com/myrustcms/plugin-analytics".to_string()),
            license: Some("MIT".to_string()),
            min_cms_version: Some("1.0.0".to_string()),
            max_cms_version: None,
            capabilities: vec![
                PluginCapability::FrontendComponents,
                PluginCapability::AdminPages,
                PluginCapability::ContentFilters,
            ],
            dependencies: vec![],
            config_schema: Some(json!({
                "type": "object",
                "properties": {
                    "tracking_id": {
                        "type": "string",
                        "pattern": "^GA-[A-Z0-9-]+$",
                        "description": "Google Analytics tracking ID (e.g., GA-123456789-1)"
                    },
                    "anonymize_ip": {
                        "type": "boolean",
                        "default": true,
                        "description": "Anonymize visitor IP addresses"
                    },
                    "track_downloads": {
                        "type": "boolean",
                        "default": false,
                        "description": "Track file downloads"
                    },
                    "enhanced_ecommerce": {
                        "type": "boolean",
                        "default": false,
                        "description": "Enable enhanced ecommerce tracking"
                    }
                },
                "required": ["tracking_id"]
            })),
            default_config: Some(json!({
                "anonymize_ip": true,
                "track_downloads": false,
                "enhanced_ecommerce": false
            })),
        }
    }
    
    async fn initialize(&mut self, context: PluginContext) -> Result<(), PluginError> {
        context.logger.info("Google Analytics plugin initialized");
        
        // Validate tracking ID format
        if let Some(tracking_id) = context.config.get("tracking_id").and_then(|v| v.as_str()) {
            if !tracking_id.starts_with("GA-") {
                return Err(PluginError::ConfigError("Invalid Google Analytics tracking ID format".to_string()));
            }
        }
        
        self.context = Some(context);
        Ok(())
    }
    
    async fn handle_event(&mut self, event: PluginEvent) -> Result<(), PluginError> {
        if let Some(ref context) = self.context {
            match event {
                PluginEvent::Activate => {
                    context.logger.info("Analytics tracking activated");
                }
                PluginEvent::Deactivate => {
                    context.logger.info("Analytics tracking deactivated");
                }
                PluginEvent::ConfigUpdate => {
                    context.logger.info("Analytics configuration updated");
                }
                _ => {}
            }
        }
        Ok(())
    }
    
    fn validate_config(&self, config: &serde_json::Value) -> Result<(), PluginError> {
        if let Some(tracking_id) = config.get("tracking_id").and_then(|v| v.as_str()) {
            if tracking_id.is_empty() || !tracking_id.starts_with("GA-") {
                return Err(PluginError::ConfigError("Valid Google Analytics tracking ID is required".to_string()));
            }
        } else {
            return Err(PluginError::ConfigError("tracking_id is required".to_string()));
        }
        Ok(())
    }
}

/// SEO Plugin - Adds SEO optimization features
pub struct SeoPlugin {
    context: Option<PluginContext>,
}

impl SeoPlugin {
    pub fn new() -> Self {
        Self { context: None }
    }
}

#[async_trait]
impl Plugin for SeoPlugin {
    fn manifest(&self) -> PluginManifest {
        PluginManifest {
            name: "seo-optimizer".to_string(),
            display_name: "SEO Optimizer".to_string(),
            description: Some("Advanced SEO optimization tools including meta tags, sitemaps, and schema markup".to_string()),
            version: "2.1.3".to_string(),
            author: Some("SEO Experts Inc".to_string()),
            author_email: Some("contact@seoexperts.com".to_string()),
            homepage_url: Some("https://seoexperts.com/cms-plugin".to_string()),
            repository_url: Some("https://github.com/seoexperts/rustcms-seo".to_string()),
            license: Some("GPL-3.0".to_string()),
            min_cms_version: Some("1.0.0".to_string()),
            max_cms_version: None,
            capabilities: vec![
                PluginCapability::ContentFilters,
                PluginCapability::AdminPages,
                PluginCapability::ApiEndpoints,
                PluginCapability::BackgroundTasks,
            ],
            dependencies: vec![],
            config_schema: Some(json!({
                "type": "object",
                "properties": {
                    "auto_meta_description": {
                        "type": "boolean",
                        "default": true,
                        "description": "Automatically generate meta descriptions"
                    },
                    "sitemap_enabled": {
                        "type": "boolean",
                        "default": true,
                        "description": "Generate XML sitemap"
                    },
                    "schema_markup": {
                        "type": "boolean",
                        "default": true,
                        "description": "Add JSON-LD schema markup"
                    },
                    "focus_keyword_analysis": {
                        "type": "boolean",
                        "default": false,
                        "description": "Enable focus keyword analysis"
                    },
                    "readability_analysis": {
                        "type": "boolean",
                        "default": false,
                        "description": "Enable content readability analysis"
                    }
                }
            })),
            default_config: Some(json!({
                "auto_meta_description": true,
                "sitemap_enabled": true,
                "schema_markup": true,
                "focus_keyword_analysis": false,
                "readability_analysis": false
            })),
        }
    }
    
    async fn initialize(&mut self, context: PluginContext) -> Result<(), PluginError> {
        context.logger.info("SEO Optimizer plugin initialized");
        self.context = Some(context);
        Ok(())
    }
    
    async fn handle_event(&mut self, event: PluginEvent) -> Result<(), PluginError> {
        if let Some(ref context) = self.context {
            match event {
                PluginEvent::Activate => {
                    context.logger.info("SEO optimization features activated");
                }
                PluginEvent::Deactivate => {
                    context.logger.info("SEO optimization features deactivated");
                }
                _ => {}
            }
        }
        Ok(())
    }
    
    async fn health_check(&self) -> Result<PluginHealthStatus, PluginError> {
        // Simulate health check
        Ok(PluginHealthStatus::Healthy)
    }
}

/// Backup Plugin - Automated backup system
pub struct BackupPlugin {
    context: Option<PluginContext>,
}

impl BackupPlugin {
    pub fn new() -> Self {
        Self { context: None }
    }
}

#[async_trait]
impl Plugin for BackupPlugin {
    fn manifest(&self) -> PluginManifest {
        PluginManifest {
            name: "auto-backup".to_string(),
            display_name: "Automated Backup".to_string(),
            description: Some("Automatically backup your content and database on a schedule".to_string()),
            version: "1.0.5".to_string(),
            author: Some("Backup Solutions Ltd".to_string()),
            author_email: Some("support@backupsolutions.com".to_string()),
            homepage_url: Some("https://backupsolutions.com/rustcms".to_string()),
            repository_url: Some("https://github.com/backupsolutions/rustcms-backup".to_string()),
            license: Some("Commercial".to_string()),
            min_cms_version: Some("1.0.0".to_string()),
            max_cms_version: None,
            capabilities: vec![
                PluginCapability::BackgroundTasks,
                PluginCapability::AdminPages,
                PluginCapability::DatabaseAccess,
                PluginCapability::FileSystemAccess,
                PluginCapability::ExternalRequests,
            ],
            dependencies: vec![],
            config_schema: Some(json!({
                "type": "object",
                "properties": {
                    "schedule": {
                        "type": "string",
                        "enum": ["daily", "weekly", "monthly"],
                        "default": "weekly",
                        "description": "Backup frequency"
                    },
                    "retention_days": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 365,
                        "default": 30,
                        "description": "Number of days to keep backups"
                    },
                    "include_media": {
                        "type": "boolean",
                        "default": true,
                        "description": "Include media files in backup"
                    },
                    "cloud_storage": {
                        "type": "object",
                        "properties": {
                            "enabled": {
                                "type": "boolean",
                                "default": false
                            },
                            "provider": {
                                "type": "string",
                                "enum": ["aws_s3", "google_cloud", "azure"]
                            },
                            "bucket": {
                                "type": "string"
                            }
                        }
                    }
                }
            })),
            default_config: Some(json!({
                "schedule": "weekly",
                "retention_days": 30,
                "include_media": true,
                "cloud_storage": {
                    "enabled": false
                }
            })),
        }
    }
    
    async fn initialize(&mut self, context: PluginContext) -> Result<(), PluginError> {
        context.logger.info("Automated Backup plugin initialized");
        self.context = Some(context);
        Ok(())
    }
    
    async fn handle_event(&mut self, event: PluginEvent) -> Result<(), PluginError> {
        if let Some(ref context) = self.context {
            match event {
                PluginEvent::Activate => {
                    context.logger.info("Automated backups enabled");
                    // Schedule backup tasks
                }
                PluginEvent::Deactivate => {
                    context.logger.info("Automated backups disabled");
                    // Cancel scheduled tasks
                }
                _ => {}
            }
        }
        Ok(())
    }
    
    fn validate_config(&self, config: &serde_json::Value) -> Result<(), PluginError> {
        if let Some(retention_days) = config.get("retention_days").and_then(|v| v.as_i64()) {
            if retention_days < 1 || retention_days > 365 {
                return Err(PluginError::ConfigError("Retention days must be between 1 and 365".to_string()));
            }
        }
        Ok(())
    }
}

/// Social Media Plugin - Social sharing and integration
pub struct SocialMediaPlugin {
    context: Option<PluginContext>,
}

impl SocialMediaPlugin {
    pub fn new() -> Self {
        Self { context: None }
    }
}

#[async_trait]
impl Plugin for SocialMediaPlugin {
    fn manifest(&self) -> PluginManifest {
        PluginManifest {
            name: "social-media-integration".to_string(),
            display_name: "Social Media Integration".to_string(),
            description: Some("Add social sharing buttons, auto-posting, and social login features".to_string()),
            version: "3.2.1".to_string(),
            author: Some("Social Connect Inc".to_string()),
            author_email: Some("hello@socialconnect.io".to_string()),
            homepage_url: Some("https://socialconnect.io/cms-plugin".to_string()),
            repository_url: Some("https://github.com/socialconnect/rustcms-social".to_string()),
            license: Some("MIT".to_string()),
            min_cms_version: Some("1.0.0".to_string()),
            max_cms_version: None,
            capabilities: vec![
                PluginCapability::FrontendComponents,
                PluginCapability::AdminPages,
                PluginCapability::ExternalRequests,
                PluginCapability::AuthenticationHooks,
                PluginCapability::ApiEndpoints,
            ],
            dependencies: vec![],
            config_schema: Some(json!({
                "type": "object",
                "properties": {
                    "sharing_buttons": {
                        "type": "array",
                        "items": {
                            "type": "string",
                            "enum": ["facebook", "twitter", "linkedin", "pinterest", "reddit", "whatsapp"]
                        },
                        "default": ["facebook", "twitter", "linkedin"]
                    },
                    "auto_posting": {
                        "type": "object",
                        "properties": {
                            "enabled": {
                                "type": "boolean",
                                "default": false
                            },
                            "platforms": {
                                "type": "array",
                                "items": {
                                    "type": "string",
                                    "enum": ["facebook", "twitter", "linkedin"]
                                }
                            }
                        }
                    },
                    "social_login": {
                        "type": "object",
                        "properties": {
                            "enabled": {
                                "type": "boolean",
                                "default": false
                            },
                            "providers": {
                                "type": "array",
                                "items": {
                                    "type": "string",
                                    "enum": ["google", "facebook", "github", "twitter"]
                                }
                            }
                        }
                    }
                }
            })),
            default_config: Some(json!({
                "sharing_buttons": ["facebook", "twitter", "linkedin"],
                "auto_posting": {
                    "enabled": false,
                    "platforms": []
                },
                "social_login": {
                    "enabled": false,
                    "providers": []
                }
            })),
        }
    }
    
    async fn initialize(&mut self, context: PluginContext) -> Result<(), PluginError> {
        context.logger.info("Social Media Integration plugin initialized");
        self.context = Some(context);
        Ok(())
    }
    
    async fn handle_event(&mut self, event: PluginEvent) -> Result<(), PluginError> {
        if let Some(ref context) = self.context {
            match event {
                PluginEvent::Activate => {
                    context.logger.info("Social media features activated");
                }
                PluginEvent::Deactivate => {
                    context.logger.info("Social media features deactivated");
                }
                _ => {}
            }
        }
        Ok(())
    }
}

/// Email Newsletter Plugin - Email marketing integration
pub struct NewsletterPlugin {
    context: Option<PluginContext>,
}

impl NewsletterPlugin {
    pub fn new() -> Self {
        Self { context: None }
    }
}

#[async_trait]
impl Plugin for NewsletterPlugin {
    fn manifest(&self) -> PluginManifest {
        PluginManifest {
            name: "email-newsletter".to_string(),
            display_name: "Email Newsletter".to_string(),
            description: Some("Collect email subscribers and send newsletters with integration to popular email services".to_string()),
            version: "1.4.2".to_string(),
            author: Some("Email Marketing Pro".to_string()),
            author_email: Some("support@emailmarketingpro.com".to_string()),
            homepage_url: Some("https://emailmarketingpro.com/cms-plugin".to_string()),
            repository_url: Some("https://github.com/emailmarketingpro/rustcms-newsletter".to_string()),
            license: Some("GPL-2.0".to_string()),
            min_cms_version: Some("1.0.0".to_string()),
            max_cms_version: None,
            capabilities: vec![
                PluginCapability::FrontendComponents,
                PluginCapability::AdminPages,
                PluginCapability::EmailSending,
                PluginCapability::ExternalRequests,
                PluginCapability::DatabaseAccess,
            ],
            dependencies: vec![],
            config_schema: Some(json!({
                "type": "object",
                "properties": {
                    "email_service": {
                        "type": "string",
                        "enum": ["mailchimp", "sendgrid", "mailgun", "smtp"],
                        "description": "Email service provider"
                    },
                    "api_key": {
                        "type": "string",
                        "description": "API key for email service"
                    },
                    "double_opt_in": {
                        "type": "boolean",
                        "default": true,
                        "description": "Require email confirmation for subscriptions"
                    },
                    "signup_form_style": {
                        "type": "string",
                        "enum": ["inline", "popup", "sidebar"],
                        "default": "inline",
                        "description": "Newsletter signup form style"
                    }
                },
                "required": ["email_service", "api_key"]
            })),
            default_config: Some(json!({
                "double_opt_in": true,
                "signup_form_style": "inline"
            })),
        }
    }
    
    async fn initialize(&mut self, context: PluginContext) -> Result<(), PluginError> {
        context.logger.info("Email Newsletter plugin initialized");
        
        // Validate email service configuration
        if context.config.get("api_key").and_then(|v| v.as_str()).map_or(true, |s| s.is_empty()) {
            return Err(PluginError::ConfigError("Email service API key is required".to_string()));
        }
        
        self.context = Some(context);
        Ok(())
    }
    
    async fn handle_event(&mut self, event: PluginEvent) -> Result<(), PluginError> {
        if let Some(ref context) = self.context {
            match event {
                PluginEvent::Activate => {
                    context.logger.info("Email newsletter features activated");
                }
                PluginEvent::Deactivate => {
                    context.logger.info("Email newsletter features deactivated");
                }
                _ => {}
            }
        }
        Ok(())
    }
    
    fn validate_config(&self, config: &serde_json::Value) -> Result<(), PluginError> {
        if config.get("api_key").and_then(|v| v.as_str()).map_or(true, |s| s.is_empty()) {
            return Err(PluginError::ConfigError("API key is required".to_string()));
        }
        
        if config.get("email_service").and_then(|v| v.as_str()).is_none() {
            return Err(PluginError::ConfigError("Email service must be specified".to_string()));
        }
        
        Ok(())
    }
}

/// Function to get all sample plugins
pub fn get_sample_plugins() -> Vec<Box<dyn Plugin>> {
    vec![
        Box::new(AnalyticsPlugin::new()),
        Box::new(SeoPlugin::new()),
        Box::new(BackupPlugin::new()),
        Box::new(SocialMediaPlugin::new()),
        Box::new(NewsletterPlugin::new()),
    ]
}

