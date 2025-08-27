use crate::components::page_builder::drag_drop_builder::{PageComponent, ComponentType, ComponentStyles, Position, ComponentProperties};
use crate::services::api_service::{create_page, get_pages, PageItem, ApiServiceError};
use uuid::Uuid;

// Helper function to create a component with default properties
#[allow(dead_code)]
fn create_component(component_type: ComponentType, content: &str) -> PageComponent {
    PageComponent {
        id: Uuid::new_v4().to_string(),
        component_type,
        content: content.to_string(),
        styles: ComponentStyles::default(),
        position: Position::default(),
        properties: ComponentProperties::default(),
    }
}

fn generate_component_id() -> String {
    Uuid::new_v4().to_string()
}

fn default_component_styles() -> ComponentStyles {
    ComponentStyles {
        background_color: "transparent".to_string(),
        text_color: "inherit".to_string(),
        padding: "16px".to_string(),
        margin: "8px".to_string(),
        border_radius: "8px".to_string(),
        font_size: "16px".to_string(),
        font_weight: "normal".to_string(),
        text_align: "left".to_string(),
        border_width: "0px".to_string(),
        border_color: "transparent".to_string(),
        border_style: "solid".to_string(),
        box_shadow: "none".to_string(),
        opacity: 1.0,
        z_index: 1,
        font_family: "system-ui, -apple-system, sans-serif".to_string(),
        line_height: "1.5".to_string(),
        letter_spacing: "normal".to_string(),
        text_decoration: "none".to_string(),
        text_transform: "none".to_string(),
        background_image: "none".to_string(),
        background_size: "cover".to_string(),
        background_position: "center".to_string(),
        background_repeat: "no-repeat".to_string(),
    }
}

fn default_position() -> Position {
    Position {
        x: 0.0,
        y: 0.0,
        width: "100%".to_string(),
        height: "auto".to_string(),
    }
}

fn default_properties() -> ComponentProperties {
    ComponentProperties {
        image_url: "".to_string(),
        image_alt: "".to_string(),
        image_title: "".to_string(),
        image_lazy_load: true,
        button_text: "Click Here".to_string(),
        button_url: "#".to_string(),
        button_target: "_self".to_string(),
        button_size: "large".to_string(),
        button_variant: "primary".to_string(),
        button_icon: "".to_string(),
        form_action: "".to_string(),
        form_method: "POST".to_string(),
        form_fields: vec![],
        video_url: "".to_string(),
        video_autoplay: false,
        video_controls: true,
        video_muted: false,
        video_loop: false,
        gallery_images: vec![],
        gallery_layout: "grid".to_string(),
        gallery_columns: 3,
        gallery_gap: 16,
        gallery_border_radius: 8,
        gallery_show_captions: true,
        gallery_enable_lightbox: true,
        gallery_enable_drag_reorder: true,

        container_max_width: "1200px".to_string(),
        container_align: "center".to_string(),
        // Sidebar
        sidebar_position: "right".to_string(),
        sidebar_width: "300px".to_string(),
        sidebar_sticky: true,
        divider_style: "solid".to_string(),
        divider_thickness: "1px".to_string(),
        divider_color: "var(--public-border-light, #ddd)".to_string(),
        divider_margin: "20px".to_string(),
        divider_width: "100%".to_string(),
        animation_type: "none".to_string(),
        animation_duration: "0.3s".to_string(),
        animation_delay: "0s".to_string(),
                        effects: "none".to_string(),
                effects_intensity: "50".to_string(),
        seo_title: "".to_string(),
        seo_description: "".to_string(),
        seo_keywords: vec![],
        aria_label: "".to_string(),
        aria_description: "".to_string(),
        tab_index: 0,
        
        // Card specific properties
        card_title: "Feature Highlight".to_string(),
        card_description: "Add your feature description here. This card component uses the same styling as post cards for a consistent design.".to_string(),
        card_image: "".to_string(),
        card_image_alt: "".to_string(),
        card_background: "#ffffff".to_string(),
        card_border_radius: "4px".to_string(),
        card_shadow: "medium".to_string(),
        card_padding: "1.5rem".to_string(),
        card_meta_text: "".to_string(),
        card_button_text: "Learn More".to_string(),
        card_button_url: "#".to_string(),
        card_button_show: true,
        
                    // Hero specific properties
            hero_badge_text: "🚀 Welcome to the Future".to_string(),
            hero_title: "Transform Your Ideas Into Reality".to_string(),
            hero_subtitle: "".to_string(),
            hero_description: "Experience the power of modern web technology with our comprehensive content management system. Built for creators, designed for success.".to_string(),
            hero_background_type: "gradient".to_string(),
            hero_background_color: "#3b82f6".to_string(),
            hero_background_gradient_start: "#3b82f6".to_string(),
            hero_background_gradient_end: "#1d4ed8".to_string(),
            hero_background_image: "".to_string(),
            hero_text_color: "#ffffff".to_string(),
            hero_alignment: "center".to_string(),
            hero_padding: "80px 40px".to_string(),
            hero_min_height: "500px".to_string(),
            hero_primary_button_text: "Get Started".to_string(),
            hero_primary_button_url: "#get-started".to_string(),
            hero_secondary_button_text: "📖 Learn More".to_string(),
            hero_secondary_button_url: "#learn-more".to_string(),
            hero_show_primary_button: true,
            hero_show_secondary_button: true,
            hero_show_badge: true,
            hero_show_stats: true,
            hero_stat1_number: "1000+".to_string(),
            hero_stat1_label: "Happy Users".to_string(),
            hero_stat2_number: "99.9%".to_string(),
            hero_stat2_label: "Uptime".to_string(),
            hero_stat3_number: "24/7".to_string(),
            hero_stat3_label: "Support".to_string(),
            
            // List specific properties
            list_items: vec![
                crate::components::page_builder::drag_drop_builder::ListItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: "Drag & Drop Page Builder".to_string(),
                    description: "Build beautiful pages without code using our intuitive visual editor".to_string(),
                    icon: "✓".to_string(),
                },
                crate::components::page_builder::drag_drop_builder::ListItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: "Lightning Fast Performance".to_string(),
                    description: "Rust-powered backend delivers exceptional speed and reliability".to_string(),
                    icon: "⚡".to_string(),
                },
                crate::components::page_builder::drag_drop_builder::ListItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: "Enterprise Security".to_string(),
                    description: "Advanced security features to protect your content and users".to_string(),
                    icon: "🔒".to_string(),
                },
            ],
            list_background: "#ffffff".to_string(),
            list_border_radius: "12px".to_string(),
            list_padding: "32px".to_string(),
            list_item_spacing: "16px".to_string(),
            list_text_color: "#666666".to_string(),
            list_show_icons: true,
            
            // PostsList specific properties
        posts_list_card_background: "var(--public-background-light, #f8fafc)".to_string(),
        posts_list_grid_gap: "24px".to_string(),
        posts_list_card_radius: "8px".to_string(),
        posts_list_card_shadow: "medium".to_string(),
        posts_list_title_color: "var(--public-text-primary, #0f172a)".to_string(),
        posts_list_meta_color: "var(--public-text-meta, #64748b)".to_string(),
        posts_list_link_color: "var(--public-link-primary, #2563eb)".to_string(),
        posts_list_columns: 3,
        posts_list_count: 6,
        posts_list_excerpt_length: 200,
        posts_list_show_author: true,
        posts_list_show_date: true,
        posts_list_show_excerpt: true,
        posts_list_show_view_all: true,
        
        // Nested components for layout containers
        nested_components: vec![],
        column_1_components: vec![],
        column_2_components: vec![],
        column_3_components: vec![],
        
        // Comments specific properties
        comments_enabled: true,
        comments_per_page: 20,
        comments_avatar_size: 48,
        comments_show_auth_prompt: true,
        comments_post_id: 1,
        
        // Page Title specific properties
        page_title_tag: "h1".to_string(),
        page_title_show_prefix: false,
        page_title_prefix: "".to_string(),
        page_title_show_suffix: false,
        page_title_suffix: "".to_string(),
        
        // Published Date specific properties
        published_date_format: "Month DD, YYYY".to_string(),
        published_date_show_prefix: true,
        published_date_prefix: "Published on".to_string(),
        published_date_show_time: false,
        published_date_relative: false,
    }
}

pub fn create_home_page_components() -> Vec<PageComponent> {
    vec![
        PageComponent {
            id: generate_component_id(),
            component_type: ComponentType::Hero,
            content: "# My Rust CMS\n\nFast. Secure. Extensible. Built with Rust & WebAssembly.".to_string(),
            styles: ComponentStyles {
                background_color: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string(),
                text_color: "white".to_string(),
                padding: "4rem 2rem".to_string(),
                text_align: "center".to_string(),
                ..default_component_styles()
            },
            position: default_position(),
            properties: default_properties(),
        },
        PageComponent {
            id: generate_component_id(),
            component_type: ComponentType::List,
            content: "Core Benefits".to_string(),
            styles: ComponentStyles {
                padding: "2rem".to_string(),
                text_align: "left".to_string(),
                font_size: "18px".to_string(),
                ..default_component_styles()
            },
            position: default_position(),
            properties: default_properties(),
        },
        PageComponent {
            id: generate_component_id(),
            component_type: ComponentType::PostsList,
            content: "Recent Posts".to_string(),
            styles: ComponentStyles {
                padding: "2rem".to_string(),
                ..default_component_styles()
            },
            position: default_position(),
            properties: default_properties(),
        },
    ]
}

pub fn create_posts_page_components() -> Vec<PageComponent> {
    vec![
        PageComponent {
            id: generate_component_id(),
            component_type: ComponentType::Heading,
            content: "# All Posts".to_string(),
            styles: ComponentStyles {
                padding: "2rem".to_string(),
                text_align: "center".to_string(),
                ..default_component_styles()
            },
            position: default_position(),
            properties: default_properties(),
        },
        PageComponent {
            id: generate_component_id(),
            component_type: ComponentType::PostsList,
            content: "All Posts".to_string(),
            styles: ComponentStyles {
                padding: "2rem".to_string(),
                ..default_component_styles()
            },
            position: default_position(),
            properties: default_properties(),
        },
    ]
}

pub fn create_why_mrcms_page_components() -> Vec<PageComponent> {
    vec![
        PageComponent {
            id: generate_component_id(),
            component_type: ComponentType::Heading,
            content: "# Why My Rust CMS".to_string(),
            styles: ComponentStyles { text_align: "center".to_string(), padding: "2rem".to_string(), ..default_component_styles() },
            position: default_position(),
            properties: default_properties(),
        },
        PageComponent {
            id: generate_component_id(),
            component_type: ComponentType::Text,
            content: "Built with Rust for performance and safety. Yew/WASM for a seamless admin. Modular templates, page builder, and enterprise security.".to_string(),
            styles: ComponentStyles { padding: "1rem 2rem".to_string(), ..default_component_styles() },
            position: default_position(),
            properties: default_properties(),
        },
        PageComponent {
            id: generate_component_id(),
            component_type: ComponentType::List,
            content: "Highlights".to_string(),
            styles: ComponentStyles { padding: "1rem 2rem".to_string(), ..default_component_styles() },
            position: default_position(),
            properties: default_properties(),
        },
    ]
}

pub async fn create_essential_pages() -> Result<Vec<PageItem>, ApiServiceError> {
    let mut created_pages = Vec::new();
    
    // Check if essential pages already exist
    let existing_pages = get_pages().await?;
    let has_home = existing_pages.iter().any(|p| p.slug == "home");
    let has_posts = existing_pages.iter().any(|p| p.slug == "posts");
    let has_why = existing_pages.iter().any(|p| p.slug == "why-my-rust-cms");
    
    // Create home page if it doesn't exist
    if !has_home {
        let home_components = create_home_page_components();
        let home_content = serde_json::to_string(&home_components).unwrap_or_default();
        
        let home_page = PageItem {
            id: None,
            title: "Home".to_string(),
            slug: "home".to_string(),
            content: home_content,
            status: "published".to_string(),
            created_at: None,
            updated_at: None,
        };
        
        match create_page(&home_page).await {
            Ok(page) => {
                gloo::console::log!("✅ Created home page with component structure");
                created_pages.push(page);
            }
            Err(e) => {
                gloo::console::error!("❌ Failed to create home page:", &format!("{:?}", e));
                return Err(e);
            }
        }
    } else {
        gloo::console::log!("ℹ️ Home page already exists, skipping creation");
    }
    
    // Create posts page if it doesn't exist
    if !has_posts {
        let posts_components = create_posts_page_components();
        let posts_content = serde_json::to_string(&posts_components).unwrap_or_default();
        
        let posts_page = PageItem {
            id: None,
            title: "Posts".to_string(),
            slug: "posts".to_string(),
            content: posts_content,
            status: "published".to_string(),
            created_at: None,
            updated_at: None,
        };
        
        match create_page(&posts_page).await {
            Ok(page) => {
                gloo::console::log!("✅ Created posts page with component structure");
                created_pages.push(page);
            }
            Err(e) => {
                gloo::console::error!("❌ Failed to create posts page:", &format!("{:?}", e));
                return Err(e);
            }
        }
    } else {
        gloo::console::log!("ℹ️ Posts page already exists, skipping creation");
    }
    
    // Create Why My Rust CMS page if it doesn't exist
    if !has_why {
        let comps = create_why_mrcms_page_components();
        let content = serde_json::to_string(&comps).unwrap_or_default();
        let why_page = PageItem { id: None, title: "Why My Rust CMS".to_string(), slug: "why-my-rust-cms".to_string(), content, status: "published".to_string(), created_at: None, updated_at: None };
        match create_page(&why_page).await {
            Ok(page) => { gloo::console::log!("✅ Created Why My Rust CMS page"); created_pages.push(page); }
            Err(e) => { gloo::console::error!("❌ Failed to create Why page:", &format!("{:?}", e)); return Err(e); }
        }
    } else {
        gloo::console::log!("ℹ️ Why My Rust CMS page already exists, skipping creation");
    }
    
    Ok(created_pages)
}