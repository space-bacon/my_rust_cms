use crate::{
    models::{
        category::{Category, NewCategory},
        post::{Post, NewPost},
        page::{Page, NewPage},
        navigation::{Navigation, NewNavigation, NewComponentTemplate},
        setting::{Setting, NewSetting},
        user::{User, NewUser},
    },
    database::DbPool,
};
use diesel::prelude::*;
use serde_json::json;
use std::sync::Arc;

/// Seed the database with default CMS data
/// This includes all the current customizations, posts, and settings
pub async fn seed_default_cms_data(db_pool: Arc<DbPool>) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = db_pool.get()?;
    
    println!("🌱 Seeding default CMS data...");
    
    // Create default admin user if none exists
    let users = User::list(&mut conn)?;
    if users.is_empty() {
        let hashed_password = bcrypt::hash("admin", bcrypt::DEFAULT_COST)
            .map_err(|e| format!("Password hashing failed: {}", e))?;
        let admin_user = NewUser {
            username: "admin".to_string(),
            password: hashed_password,
            email: Some("admin@example.com".to_string()),
            role: "admin".to_string(),
            status: "active".to_string(),
            email_verified: Some(true),
            email_verification_token: None,
            email_verification_expires_at: None,
        };
        User::create(&mut conn, admin_user)?;
        println!("✅ Created admin user");
    }
    
    // Create default category
    if Category::list(&mut conn)?.is_empty() {
        let general_category = NewCategory {
            name: "General".to_string(),
        };
        Category::create(&mut conn, general_category)?;
        println!("✅ Created General category");
    }
    
    // Create default navigation items
    if Navigation::list(&mut conn)?.is_empty() {
        let home_nav = NewNavigation {
            title: "Home".to_string(),
            url: "/".to_string(),
            order_position: 1,
            is_active: true,
            menu_area: "header".to_string(),
            parent_id: None,
            icon: Some("home".to_string()),
            css_class: None,
            target: Some("_self".to_string()),
            mobile_visible: true,
            description: Some("Homepage link".to_string()),
        };
        Navigation::create(&mut conn, home_nav)?;
        
        let posts_nav = NewNavigation {
            title: "Posts".to_string(),
            url: "/posts".to_string(),
            order_position: 2,
            is_active: true,
            menu_area: "header".to_string(),
            parent_id: None,
            icon: Some("article".to_string()),
            css_class: None,
            target: Some("_self".to_string()),
            mobile_visible: true,
            description: Some("View all posts".to_string()),
        };
        Navigation::create(&mut conn, posts_nav)?;
        println!("✅ Created navigation items");
    }
    
    // Create default component templates
    seed_component_templates(&mut conn)?;
    
    // Create default settings
    seed_default_settings(&mut conn)?;
    
    // Create sample posts
    seed_sample_posts(&mut conn)?;
    
    // Create default pages
    seed_default_pages(&mut conn)?;
    
    println!("🎉 Default CMS data seeding completed!");
    
    Ok(())
}

fn seed_component_templates(conn: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    use crate::schema::component_templates::dsl::*;
    
    let existing_count: i64 = component_templates.count().get_result(conn)?;
    if existing_count > 0 {
        println!("⏭️  Component templates already exist, skipping...");
        return Ok(());
    }
    
    // Fixed Header Template (with gradient and current customizations)
    let fixed_header = NewComponentTemplate {
        name: "Fixed Header".to_string(),
        component_type: "header".to_string(),
        template_data: json!({
            "bg_color": "#66f0a2",
            "bg_gradient_direction": "to right",
            "bg_gradient_end": "#764ba2",
            "bg_gradient_start": "#667eea",
            "bg_type": "gradient",
            "background": "inherit",
            "border": "none",
            "effects": "none",
            "height": "429px",
            "logo_font_size": "1.5rem",
            "logo_text": "My Site",
            "logo_type": "text",
            "nav_underline_animation": "slide",
            "padding": "1rem 0",
            "position": "sticky",
            "scroll_effect": "shrink",
            "scroll_trigger": "100",
            "shape_mask_lower": "tilt",
            "shape_mask_lower_amplitude": "20",
            "shape_mask_lower_degrees": "2",
            "shape_mask_lower_direction": "left",
            "shape_mask_lower_frequency": "1.5",
            "shape_mask_lower_scale": "100",
            "shape_mask_upper": "none",
            "shape_mask_upper_amplitude": "20",
            "shape_mask_upper_degrees": "0",
            "shape_mask_upper_direction": "right",
            "shape_mask_upper_frequency": "1",
            "shape_mask_upper_scale": "100",
            "shrink_height": "135",
            "shrink_logo_scale": "70",
            "site_title_color": "#ffffff",
            "text_color": "#db3d3d"
        }),
        breakpoints: json!({}),
        width_setting: None,
        max_width: None,
        is_default: true,
        is_active: true,
    };
    diesel::insert_into(crate::schema::component_templates::table)
        .values(&fixed_header)
        .execute(conn)?;
    
    // Default Footer Template
    let default_footer = NewComponentTemplate {
        name: "Default Footer".to_string(),
        component_type: "footer".to_string(),
        template_data: json!({
            "background": "inherit",
            "padding": "2rem 0",
            "border": "none"
        }),
        breakpoints: json!({}),
        width_setting: None,
        max_width: None,
        is_default: true,
        is_active: true,
    };
    diesel::insert_into(crate::schema::component_templates::table)
        .values(&default_footer)
        .execute(conn)?;
    
    println!("✅ Created component templates");
    Ok(())
}

fn seed_default_settings(conn: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    use crate::schema::settings::dsl::*;
    
    let existing_count: i64 = settings.count().get_result(conn)?;
    if existing_count > 0 {
        println!("⏭️  Settings already exist, skipping...");
        return Ok(());
    }
    
    let default_settings = vec![
        ("admin_button_visible", "true", "site"),
        ("login_button_visible", "true", "site"),
        ("site_title", "My Rust CMS", "site"),
        ("site_description", "Modern Content Management System built with Rust", "site"),
        
        // Typography settings
        ("typography_font_family", "pixelify-sans", "typography"),
        ("typography_font_size", "16", "typography"),
        ("typography_line_height", "1.6", "typography"),
        ("typography_font_weight", "400", "typography"),
        
        // Container settings  
        ("container_background_type", "none", "container"),
        ("container_background_color", "#ffffff", "container"),
        ("container_border_color", "#000000", "container"),
        ("container_animation", "none", "container"),
    ];
    
    for (key, value, type_str) in default_settings {
        let new_setting = NewSetting {
            setting_key: key.to_string(),
            setting_value: Some(value.to_string()),
            setting_type: type_str.to_string(),
            description: None,
        };
        Setting::create(conn, new_setting)?;
    }
    
    println!("✅ Created default settings");
    Ok(())
}

fn seed_sample_posts(conn: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    use crate::schema::posts::dsl::*;
    
    let existing_count: i64 = posts.count().get_result(conn)?;
    if existing_count > 0 {
        println!("⏭️  Posts already exist, skipping...");
        return Ok(());
    }
    
    let sample_posts = vec![
        ("Welcome to My Rust CMS", "Welcome to your new Rust-powered content management system! This CMS is built for performance, security, and scalability."),
        ("Getting Started Guide", "Learn how to customize your CMS, create content, and manage your website with this comprehensive getting started guide."),
        ("Modern Content Management", "Experience the power of modern web technologies with Rust on the backend and cutting-edge frontend frameworks."),
        ("Thank you for your attention on this matter", "Thank you for your attention on this matter"),
    ];
    
    for (title_str, content_str) in sample_posts {
        let new_post = NewPost {
            title: title_str.to_string(),
            content: content_str.to_string(),
            category_id: None,
            user_id: Some(1), // Admin user ID
        };
        Post::create(conn, new_post)?;
    }
    
    println!("✅ Created sample posts");
    Ok(())
}

fn seed_default_pages(conn: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    use crate::schema::pages::dsl::*;
    
    // Check if home page exists
    let home_exists = pages
        .filter(slug.eq("home"))
        .first::<Page>(conn)
        .is_ok();
    
    if !home_exists {
        let home_page = NewPage {
            title: "Home".to_string(),
            content: "[{\"id\": \"hero-section\", \"component_type\": \"hero\", \"content\": \"# Modern Content Management\\n\\nBuilt for Performance and Scalability\", \"properties\": {\"background_type\": \"gradient\", \"background_gradient_start\": \"#667eea\", \"background_gradient_end\": \"#764ba2\", \"text_align\": \"center\", \"padding\": \"4rem 2rem\"}}, {\"id\": \"features-section\", \"component_type\": \"text\", \"content\": \"## Built for Performance and Scalability\\n\\nOur Rust-powered CMS delivers exceptional speed and reliability.\", \"properties\": {\"text_align\": \"center\", \"padding\": \"2rem\"}}, {\"id\": \"cta-section\", \"component_type\": \"text\", \"content\": \"Thank you for your attention on this matter\", \"properties\": {\"text_align\": \"center\", \"padding\": \"2rem\"}}]".to_string(),
            slug: "home".to_string(),
            status: "published".to_string(),
            user_id: Some(1), // Admin user ID
        };
        Page::create(conn, home_page)?;
        println!("✅ Created home page");
    }
    
    // Check if posts page exists
    let posts_exists = pages
        .filter(slug.eq("posts"))
        .first::<Page>(conn)
        .is_ok();
        
    if !posts_exists {
        let posts_page = NewPage {
            title: "Posts".to_string(),
            content: "".to_string(), // Posts page uses dynamic content
            slug: "posts".to_string(),
            status: "published".to_string(),
            user_id: Some(1), // Admin user ID
        };
        Page::create(conn, posts_page)?;
        println!("✅ Created posts page");
    }
    
    Ok(())
}
