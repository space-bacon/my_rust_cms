use yew::prelude::*;
use web_sys::{DragEvent, Element, MouseEvent, HtmlInputElement, HtmlTextAreaElement, InputEvent, KeyboardEvent};
use wasm_bindgen::JsCast;
use serde::{Deserialize, Serialize};
use crate::components::markdown_editor::MarkdownEditor;
use crate::components::MediaPicker;
use crate::services::api_service::MediaItem;
use crate::services::navigation_service::{get_component_templates, get_all_component_templates_admin, ComponentTemplate};
use crate::components::{EnhancedGallery, EnhancedGalleryImage};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PageComponent {
    pub id: String,
    pub component_type: ComponentType,
    pub content: String,
    pub styles: ComponentStyles,
    pub position: Position,
    pub properties: ComponentProperties,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ComponentProperties {
    // Image specific
    pub image_url: String,
    pub image_alt: String,
    pub image_title: String,
    pub image_lazy_load: bool,
    
    // Button/Link specific
    pub button_text: String,
    pub button_url: String,
    pub button_target: String,
    pub button_size: String,
    pub button_variant: String,
    pub button_icon: String,
    
    // Form specific
    pub form_action: String,
    pub form_method: String,
    pub form_fields: Vec<FormField>,
    
    // Video specific
    pub video_url: String,
    pub video_autoplay: bool,
    pub video_controls: bool,
    pub video_muted: bool,
    pub video_loop: bool,
    
    // Gallery specific
    pub gallery_images: Vec<GalleryImage>,
    pub gallery_layout: String,
    pub gallery_columns: i32,
    pub gallery_gap: i32,
    pub gallery_border_radius: i32,
    pub gallery_show_captions: bool,
    pub gallery_enable_lightbox: bool,
    pub gallery_enable_drag_reorder: bool,
    

    
    // Container specific
    pub container_max_width: String,
    pub container_align: String,
    // Sidebar specific
    pub sidebar_position: String, // "left" | "right"
    pub sidebar_width: String,    // e.g., "300px"
    pub sidebar_sticky: bool,
    
    // Divider specific
    pub divider_style: String,
    pub divider_thickness: String,
    pub divider_color: String,
    pub divider_margin: String,
    pub divider_width: String,
    
    // Intro Animation
    pub intro_animation_type: String,
    pub intro_animation_duration: String,
    pub intro_animation_delay: String,
    pub intro_animation_easing: String,
    pub intro_animation_trigger: String, // "load", "scroll", "hover", "click"
    pub intro_animation_offset: String, // scroll offset for scroll trigger
    pub intro_animation_repeat: bool,
    
    // Legacy Animation (keeping for backward compatibility)
    pub animation_type: String,
    pub animation_duration: String,
    pub animation_delay: String,
    
    // Effects
    pub effects: String,
    pub effects_intensity: String,
    
    // SEO
    pub seo_title: String,
    pub seo_description: String,
    pub seo_keywords: Vec<String>,
    
    // Accessibility
    pub aria_label: String,
    pub aria_description: String,
    pub tab_index: i32,
    
    // Nested components for layout containers
    pub nested_components: Vec<PageComponent>,
    pub column_1_components: Vec<PageComponent>,
    pub column_2_components: Vec<PageComponent>,
    pub column_3_components: Vec<PageComponent>,
    
    // Card specific properties
    pub card_title: String,
    pub card_description: String,
    pub card_image: String,
    pub card_image_alt: String,
    pub card_background: String,
    pub card_border_radius: String,
    pub card_shadow: String,
    pub card_padding: String,
    pub card_meta_text: String,
    pub card_button_text: String,
    pub card_button_url: String,
    pub card_button_show: bool,
    
    // Hero specific properties
    pub hero_badge_text: String,
    pub hero_title: String,
    pub hero_subtitle: String,
    pub hero_description: String,
    pub hero_background_type: String, // "gradient", "solid", "image"
    pub hero_background_color: String,
    pub hero_background_gradient_start: String,
    pub hero_background_gradient_end: String,
    pub hero_background_image: String,
    pub hero_text_color: String,
    pub hero_alignment: String, // "left", "center", "right"
    pub hero_padding: String,
    pub hero_min_height: String,
    pub hero_primary_button_text: String,
    pub hero_primary_button_url: String,
    pub hero_secondary_button_text: String,
    pub hero_secondary_button_url: String,
    pub hero_show_primary_button: bool,
    pub hero_show_secondary_button: bool,
    pub hero_show_badge: bool,
    pub hero_show_stats: bool,
    pub hero_stat1_number: String,
    pub hero_stat1_label: String,
    pub hero_stat2_number: String,
    pub hero_stat2_label: String,
    pub hero_stat3_number: String,
    pub hero_stat3_label: String,
    
    // List specific properties
    pub list_items: Vec<ListItem>,
    pub list_background: String,
    pub list_border_radius: String,
    pub list_padding: String,
    pub list_item_spacing: String,
    pub list_text_color: String,
    pub list_show_icons: bool,
    
    // PostsList specific properties
    pub posts_list_card_background: String,
    pub posts_list_grid_gap: String,
    pub posts_list_card_radius: String,
    pub posts_list_card_shadow: String,
    pub posts_list_title_color: String,
    pub posts_list_meta_color: String,
    pub posts_list_link_color: String,
    pub posts_list_columns: i32,
    pub posts_list_count: i32,
    pub posts_list_excerpt_length: i32,
    pub posts_list_show_author: bool,
    pub posts_list_show_date: bool,
    pub posts_list_show_excerpt: bool,
    pub posts_list_show_view_all: bool,
    
    // Comments specific properties
    pub comments_enabled: bool,
    pub comments_per_page: i32,
    pub comments_avatar_size: i32,
    pub comments_show_auth_prompt: bool,
    pub comments_post_id: i32,
    
    // Page Title specific properties
    pub page_title_tag: String, // h1, h2, h3, etc.
    pub page_title_show_prefix: bool,
    pub page_title_prefix: String,
    pub page_title_show_suffix: bool,
    pub page_title_suffix: String,
    
    // Published Date specific properties
    pub published_date_format: String, // "YYYY-MM-DD", "Month DD, YYYY", etc.
    pub published_date_show_prefix: bool,
    pub published_date_prefix: String,
    pub published_date_show_time: bool,
    pub published_date_relative: bool, // "2 days ago" vs absolute date
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct FormField {
    pub field_type: String,
    pub name: String,
    pub label: String,
    pub placeholder: String,
    pub required: bool,
    pub validation: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct GalleryImage {
    pub url: String,
    pub alt: String,
    pub caption: String,
    pub title: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ComponentType {
    Text,
    Heading,
    Subheading,
    Image,
    Button,
    Link,
    Container,
    Sidebar,
    TwoColumn,
    ThreeColumn,
    Hero,
    Card,
    List,
    Quote,
    Video,
    Spacer,
    Divider,
    ContactForm,
    Newsletter,
    Map,
    Gallery,
    PostsList,
    Comments,
    PageTitle,
    PublishedDate,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentStyles {
    pub background_color: String,
    pub text_color: String,
    pub padding: String,
    pub margin: String,
    pub border_radius: String,
    pub font_size: String,
    pub font_weight: String,
    pub text_align: String,
    pub border_width: String,
    pub border_color: String,
    pub border_style: String,
    pub box_shadow: String,
    pub opacity: f32,
    pub z_index: i32,
    pub font_family: String,
    pub line_height: String,
    pub letter_spacing: String,
    pub text_decoration: String,
    pub text_transform: String,
    pub background_image: String,
    pub background_size: String,
    pub background_position: String,
    pub background_repeat: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub width: String,
    pub height: String,
}

impl Default for ComponentStyles {
    fn default() -> Self {
        Self {
            background_color: "transparent".to_string(),
            text_color: "var(--public-text-primary, #000000)".to_string(),
            padding: "1rem".to_string(),  // Reduced from 16px to 1rem (16px) for consistency
            margin: "0.5rem".to_string(), // Reduced from 8px to 0.5rem (8px) for consistency
            border_radius: "0.5rem".to_string(), // Changed to rem units for consistency
            font_size: "1rem".to_string(), // Changed to rem for consistency
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
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: "100%".to_string(),
            height: "auto".to_string(),
        }
    }
}

impl Default for ComponentProperties {
    fn default() -> Self {
        Self {
            // Image specific
            image_url: "".to_string(),
            image_alt: "".to_string(),
            image_title: "".to_string(),
            image_lazy_load: true,
            
            // Button/Link specific
            button_text: "Button".to_string(),
            button_url: "#".to_string(),
            button_target: "_self".to_string(),
            button_size: "large".to_string(),
            button_variant: "primary".to_string(),
            button_icon: "".to_string(),
            
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
                ListItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: "Drag & Drop Page Builder".to_string(),
                    description: "Build beautiful pages without code using our intuitive visual editor".to_string(),
                    icon: "✓".to_string(),
                },
                ListItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: "Lightning Fast Performance".to_string(),
                    description: "Rust-powered backend delivers exceptional speed and reliability".to_string(),
                    icon: "⚡".to_string(),
                },
                ListItem {
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
            
            // Form specific
            form_action: "/submit".to_string(),
            form_method: "POST".to_string(),
            form_fields: vec![],
            
            // Video specific
            video_url: "".to_string(),
            video_autoplay: false,
            video_controls: true,
            video_muted: false,
            video_loop: false,
            
            // Gallery specific
            gallery_images: vec![],
            gallery_layout: "grid".to_string(),
            gallery_columns: 3,
            gallery_gap: 16,
            gallery_border_radius: 8,
            gallery_show_captions: true,
            gallery_enable_lightbox: true,
            gallery_enable_drag_reorder: true,
            

            
            // Container specific
            container_max_width: "1200px".to_string(),
            container_align: "center".to_string(),
            // Sidebar specific
            sidebar_position: "right".to_string(),
            sidebar_width: "300px".to_string(),
            sidebar_sticky: true,
            
            // Divider specific
            divider_style: "solid".to_string(),
            divider_thickness: "1px".to_string(),
            divider_color: "var(--public-border-light, #ddd)".to_string(),
            divider_margin: "20px".to_string(),
            divider_width: "100%".to_string(),
            
            // Intro Animation
            intro_animation_type: "none".to_string(),
            intro_animation_duration: "0.6s".to_string(),
            intro_animation_delay: "0s".to_string(),
            intro_animation_easing: "ease-out".to_string(),
            intro_animation_trigger: "scroll".to_string(),
            intro_animation_offset: "100px".to_string(),
            intro_animation_repeat: false,
            
            // Legacy Animation (keeping for backward compatibility)
            animation_type: "none".to_string(),
            animation_duration: "0.3s".to_string(),
            animation_delay: "0s".to_string(),
            
            // Effects
            effects: "none".to_string(),
            effects_intensity: "50".to_string(),
            
            // SEO
            seo_title: "".to_string(),
            seo_description: "".to_string(),
            seo_keywords: vec![],
            
            // Accessibility
            aria_label: "".to_string(),
            aria_description: "".to_string(),
            tab_index: 0,
            
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
}

impl ComponentType {
    pub fn display_name(&self) -> &'static str {
        match self {
            ComponentType::Text => "Text Block",
            ComponentType::Heading => "Heading",
            ComponentType::Subheading => "Subheading",
            ComponentType::Image => "Image",
            ComponentType::Button => "Button",
            ComponentType::Link => "Link",
            ComponentType::Container => "Container",
            ComponentType::Sidebar => "Sidebar Layout",
            ComponentType::TwoColumn => "Two Columns",
            ComponentType::ThreeColumn => "Three Columns",
            ComponentType::Hero => "Hero Section",
            ComponentType::Card => "Card",
            ComponentType::List => "List",
            ComponentType::Quote => "Quote",
            ComponentType::Video => "Video",
            ComponentType::Spacer => "Spacer",
            ComponentType::Divider => "Divider",
            ComponentType::ContactForm => "Contact Form",
            ComponentType::Newsletter => "Newsletter",
            ComponentType::Map => "Map",
            ComponentType::Gallery => "Gallery",
            ComponentType::PostsList => "Posts List",
            ComponentType::Comments => "Comments",
            ComponentType::PageTitle => "Page Title",
            ComponentType::PublishedDate => "Published Date",
        }
    }

    pub fn default_content(&self) -> String {
        match self {
            ComponentType::Text => "Welcome to our comprehensive content management system built with Rust and modern web technologies. This platform provides powerful tools for creating, managing, and publishing content with ease.".to_string(),
            ComponentType::Heading => "# Modern Content Management".to_string(),
            ComponentType::Subheading => "## Built for Performance and Scalability".to_string(),
            ComponentType::Image => "![Rust CMS Dashboard](https://via.placeholder.com/800x400/4299e1/ffffff?text=Rust+CMS+Dashboard)".to_string(),
            ComponentType::Button => "[Start Building 🚀](/page-builder)".to_string(),
            ComponentType::Link => "[Link Text](#)".to_string(),
            ComponentType::Container => "This container holds structured content that can be easily customized and styled to match your brand.".to_string(),
            ComponentType::Sidebar => "Sidebar layout with configurable left/right position and width.".to_string(),
            ComponentType::TwoColumn => "## 🚀 Performance First\n\nBuilt with Rust for maximum performance and reliability. Our backend delivers lightning-fast responses and handles high traffic with ease.\n\n## 🎨 Beautiful Design\n\nModern, responsive design that looks great on all devices. Clean interfaces and intuitive user experience.".to_string(),
            ComponentType::ThreeColumn => "## ⚡ Fast\n\nRust-powered backend delivers exceptional performance\n\n## 🔒 Secure\n\nBuilt-in security features and best practices\n\n## 🎯 Flexible\n\nCustomizable components and layouts".to_string(),
            ComponentType::Hero => "# Welcome to the Future of Content Management\n\nExperience the power of Rust-based CMS with WebAssembly frontend. Create stunning websites with our drag-and-drop page builder and comprehensive content management tools.\n\n[Get Started →](/register) [View Demo →](/demo)".to_string(),
            ComponentType::Card => "## 🌟 Feature Highlight\n\nDrag-and-drop page builder with real-time preview. Create professional pages without coding knowledge.\n\n**Key Benefits:**\n- Visual editing\n- Real-time preview\n- Mobile responsive\n- SEO optimized\n\n[Try Page Builder →](/page-builder)".to_string(),
            ComponentType::List => "✅ **Rust-powered backend** for maximum performance\n✅ **WebAssembly frontend** for modern user experience\n✅ **Drag-and-drop page builder** for easy content creation\n✅ **Media management** with file upload and organization\n✅ **User authentication** and role-based access\n✅ **Responsive design** that works on all devices".to_string(),
            ComponentType::Quote => "> \"This Rust CMS has revolutionized how we manage our content. The performance is incredible and the page builder makes it easy for our team to create beautiful pages without technical knowledge.\"\n\n*— Sarah Johnson, Content Manager*".to_string(),
            ComponentType::Video => "".to_string(),
            ComponentType::Spacer => "".to_string(),
            ComponentType::Divider => "---".to_string(),
            ComponentType::ContactForm => "## 📧 Get in Touch\n\nReady to transform your content management? We'd love to hear from you and help you get started.\n\n**Why choose our CMS?**\n- Built with modern Rust technology\n- Intuitive drag-and-drop interface\n- Enterprise-grade security\n- 24/7 support\n\n[Contact Form - Name, Email, Message fields would appear here]".to_string(),
            ComponentType::Newsletter => "## 📮 Stay Updated\n\nGet the latest updates about new features, best practices, and industry insights delivered to your inbox.\n\n**What you'll receive:**\n- Monthly feature updates\n- Content management tips\n- Industry insights\n- Exclusive tutorials\n\n[Email Signup Form - Email field and Subscribe button would appear here]".to_string(),
            ComponentType::Map => "## 🗺️ Visit Our Office\n\n**Rust CMS Headquarters**\n123 Innovation Drive\nTech Valley, CA 94000\n\nOffice Hours: Monday - Friday, 9 AM - 6 PM PST\nPhone: (555) 123-4567\n\n[Interactive Map showing our location would appear here]".to_string(),
            ComponentType::Gallery => "## 🖼️ Showcase Gallery\n\nExplore examples of websites built with our CMS. From simple blogs to complex e-commerce sites, see what's possible.\n\n[Image gallery with sample websites would appear here]".to_string(),
            ComponentType::PostsList => "## 📄 Latest Posts\n\nDiscover our latest articles and insights. This dynamic list automatically displays your most recent blog posts.\n\n[This will show a list of your published posts]".to_string(),
            ComponentType::Comments => "## 💬 Join the Conversation\n\nShare your thoughts and engage with our community. Sign in or create an account to leave your comments.\n\n[Comments section with text message style bubbles and Gravatar avatars will appear here]".to_string(),
            ComponentType::PageTitle => "".to_string(), // Will be populated dynamically from page data
            ComponentType::PublishedDate => "".to_string(), // Will be populated dynamically from page data
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ComponentType::Text => "📝",
            ComponentType::Heading => "📰",
            ComponentType::Subheading => "📄",
            ComponentType::Image => "🖼️",
            ComponentType::Button => "🔘",
            ComponentType::Link => "🔗",
            ComponentType::Container => "📦",
            ComponentType::Sidebar => "📚",
            ComponentType::TwoColumn => "📑",
            ComponentType::ThreeColumn => "📊",
            ComponentType::Hero => "🌟",
            ComponentType::Card => "🃏",
            ComponentType::List => "📋",
            ComponentType::Quote => "💬",
            ComponentType::Video => "🎥",
            ComponentType::Spacer => "⬜",
            ComponentType::Divider => "➖",
            ComponentType::ContactForm => "📧",
            ComponentType::Newsletter => "📮",
            ComponentType::Map => "🗺️",
            ComponentType::Gallery => "🖼️",
            ComponentType::PostsList => "📄",
            ComponentType::Comments => "💬",
            ComponentType::PageTitle => "📋",
            ComponentType::PublishedDate => "📅",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DragDropPageBuilderProps {
    pub page_id: Option<i32>,
    pub on_save: Callback<Vec<PageComponent>>,
    #[prop_or_default]
    pub initial_components: Vec<PageComponent>,
}

// Helper function to find a nested component by ID and return its parent and location
#[allow(dead_code)]
fn find_nested_component_location(components: &[PageComponent], target_id: &str) -> Option<(String, String)> {
    for component in components {
        // Check nested_components
        if component.properties.nested_components.iter().any(|c| c.id == target_id) {
            return Some((component.id.clone(), "nested_components".to_string()));
        }
        // Check column_1_components
        if component.properties.column_1_components.iter().any(|c| c.id == target_id) {
            return Some((component.id.clone(), "column_1_components".to_string()));
        }
        // Check column_2_components
        if component.properties.column_2_components.iter().any(|c| c.id == target_id) {
            return Some((component.id.clone(), "column_2_components".to_string()));
        }
        // Check column_3_components
        if component.properties.column_3_components.iter().any(|c| c.id == target_id) {
            return Some((component.id.clone(), "column_3_components".to_string()));
        }
    }
    None
}

// Helper function to remove a nested component
fn remove_nested_component(components: &mut Vec<PageComponent>, target_id: &str) -> bool {
    // First check if it's a top-level component
    if let Some(pos) = components.iter().position(|c| c.id == target_id) {
        components.remove(pos);
        return true;
    }
    
    // Otherwise, look for it in nested components
    for component in components.iter_mut() {
        if component.properties.nested_components.iter().any(|c| c.id == target_id) {
            component.properties.nested_components.retain(|c| c.id != target_id);
            return true;
        }
        if component.properties.column_1_components.iter().any(|c| c.id == target_id) {
            component.properties.column_1_components.retain(|c| c.id != target_id);
            return true;
        }
        if component.properties.column_2_components.iter().any(|c| c.id == target_id) {
            component.properties.column_2_components.retain(|c| c.id != target_id);
            return true;
        }
        if component.properties.column_3_components.iter().any(|c| c.id == target_id) {
            component.properties.column_3_components.retain(|c| c.id != target_id);
            return true;
        }
    }
    false
}

// Helper function to duplicate a nested component
fn duplicate_nested_component(components: &mut Vec<PageComponent>, target_id: &str) -> bool {
    // First check if it's a top-level component
    if let Some(pos) = components.iter().position(|c| c.id == target_id) {
        if let Some(component) = components.get(pos).cloned() {
            let mut new_component = component;
            new_component.id = uuid::Uuid::new_v4().to_string();
            components.insert(pos + 1, new_component);
            return true;
        }
    }
    
    // Otherwise, look for it in nested components
    for component in components.iter_mut() {
        if let Some(pos) = component.properties.nested_components.iter().position(|c| c.id == target_id) {
            if let Some(nested_comp) = component.properties.nested_components.get(pos).cloned() {
                let mut new_component = nested_comp;
                new_component.id = uuid::Uuid::new_v4().to_string();
                component.properties.nested_components.insert(pos + 1, new_component);
                return true;
            }
        }
        if let Some(pos) = component.properties.column_1_components.iter().position(|c| c.id == target_id) {
            if let Some(nested_comp) = component.properties.column_1_components.get(pos).cloned() {
                let mut new_component = nested_comp;
                new_component.id = uuid::Uuid::new_v4().to_string();
                component.properties.column_1_components.insert(pos + 1, new_component);
                return true;
            }
        }
        if let Some(pos) = component.properties.column_2_components.iter().position(|c| c.id == target_id) {
            if let Some(nested_comp) = component.properties.column_2_components.get(pos).cloned() {
                let mut new_component = nested_comp;
                new_component.id = uuid::Uuid::new_v4().to_string();
                component.properties.column_2_components.insert(pos + 1, new_component);
                return true;
            }
        }
        if let Some(pos) = component.properties.column_3_components.iter().position(|c| c.id == target_id) {
            if let Some(nested_comp) = component.properties.column_3_components.get(pos).cloned() {
                let mut new_component = nested_comp;
                new_component.id = uuid::Uuid::new_v4().to_string();
                component.properties.column_3_components.insert(pos + 1, new_component);
                return true;
            }
        }
    }
    false
}

// Helper function to update nested component content
fn update_nested_component_content(components: &mut Vec<PageComponent>, target_id: &str, new_content: String) -> bool {
    // First check if it's a top-level component
    if let Some(component) = components.iter_mut().find(|c| c.id == target_id) {
        component.content = new_content;
        return true;
    }
    
    // Otherwise, look for it in nested components
    for component in components.iter_mut() {
        if let Some(nested_comp) = component.properties.nested_components.iter_mut().find(|c| c.id == target_id) {
            nested_comp.content = new_content;
            return true;
        }
        if let Some(nested_comp) = component.properties.column_1_components.iter_mut().find(|c| c.id == target_id) {
            nested_comp.content = new_content;
            return true;
        }
        if let Some(nested_comp) = component.properties.column_2_components.iter_mut().find(|c| c.id == target_id) {
            nested_comp.content = new_content;
            return true;
        }
        if let Some(nested_comp) = component.properties.column_3_components.iter_mut().find(|c| c.id == target_id) {
            nested_comp.content = new_content;
            return true;
        }
    }
    false
}

// Helper function to find any component by ID (including nested components)
fn find_component_by_id<'a>(components: &'a [PageComponent], target_id: &str) -> Option<&'a PageComponent> {
    // First check if it's a top-level component
    if let Some(component) = components.iter().find(|c| c.id == target_id) {
        return Some(component);
    }
    
    // Otherwise, look for it in nested components
    for component in components {
        if let Some(nested_comp) = component.properties.nested_components.iter().find(|c| c.id == target_id) {
            return Some(nested_comp);
        }
        if let Some(nested_comp) = component.properties.column_1_components.iter().find(|c| c.id == target_id) {
            return Some(nested_comp);
        }
        if let Some(nested_comp) = component.properties.column_2_components.iter().find(|c| c.id == target_id) {
            return Some(nested_comp);
        }
        if let Some(nested_comp) = component.properties.column_3_components.iter().find(|c| c.id == target_id) {
            return Some(nested_comp);
        }
    }
    None
}

// Helper function to generate animation classes and data attributes
#[allow(dead_code)]
fn get_animation_classes_and_attributes(component: &PageComponent) -> (String, Vec<(&'static str, String)>) {
    let mut classes = Vec::new();
    let mut attributes = Vec::new();
    
    // Only add animation classes if animation type is not "none"
    if component.properties.intro_animation_type != "none" && !component.properties.intro_animation_type.is_empty() {
        // Add the base animation class
        classes.push("intro-animation".to_string());
        
        // Add the specific animation type class
        classes.push(format!("intro-{}", component.properties.intro_animation_type));
        
        // Add easing class if specified
        if !component.properties.intro_animation_easing.is_empty() && component.properties.intro_animation_easing != "ease-out" {
            let easing_class = match component.properties.intro_animation_easing.as_str() {
                "ease" => "intro-ease",
                "ease-in" => "intro-ease-in",
                "ease-out" => "intro-ease-out",
                "ease-in-out" => "intro-ease-in-out",
                "linear" => "intro-linear",
                "cubic-bezier(0.68, -0.55, 0.265, 1.55)" => "intro-bounce",
                "cubic-bezier(0.25, 0.46, 0.45, 0.94)" => "intro-smooth",
                _ => "intro-ease-out"
            };
            classes.push(easing_class.to_string());
        }
        
        // Add data attributes for JavaScript
        attributes.push(("data-intro-trigger", component.properties.intro_animation_trigger.clone()));
        attributes.push(("data-intro-duration", component.properties.intro_animation_duration.clone()));
        attributes.push(("data-intro-delay", component.properties.intro_animation_delay.clone()));
        attributes.push(("data-intro-easing", component.properties.intro_animation_easing.clone()));
        
        if component.properties.intro_animation_trigger == "scroll" {
            attributes.push(("data-intro-offset", component.properties.intro_animation_offset.clone()));
        }
        
        if component.properties.intro_animation_repeat {
            attributes.push(("data-intro-repeat", "true".to_string()));
        }
    }
    
    (classes.join(" "), attributes)
}

#[function_component(DragDropPageBuilder)]
pub fn drag_drop_page_builder(props: &DragDropPageBuilderProps) -> Html {
    let components = use_state(Vec::<PageComponent>::new);
    let selected_component = use_state(|| None::<String>);
    let editing_component = use_state(|| None::<String>);
    let drag_over = use_state(|| false);
    let dragging_component = use_state(|| None::<ComponentType>);
    
    // Individual drag-over states for nested drop zones  
    let container_drag_over = use_state(|| None::<String>); // stores container ID when dragging over
    let column_drag_over = use_state(|| None::<(String, String)>); // stores (container_id, column) when dragging over
    
    // New state for dragging existing components for reordering
    let dragging_existing_component = use_state(|| None::<String>); // stores component ID being dragged
    let drop_zone_hover = use_state(|| None::<String>); // stores drop zone ID being hovered over
    let show_media_picker = use_state(|| false);
    let media_picker_target_component = use_state(|| None::<String>);
    let media_picker_images_only = use_state(|| true);

    // Cache component templates for default config lookups (e.g., Sidebar)
    let component_templates = use_state(Vec::<ComponentTemplate>::new);

    {
        let component_templates = component_templates.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // Prefer admin endpoint (includes inactive templates and latest edits)
                match get_all_component_templates_admin().await {
                    Ok(templates) => component_templates.set(templates),
                    Err(_) => {
                        // Fallback to public endpoint
                        if let Ok(public_templates) = get_component_templates().await {
                            component_templates.set(public_templates);
                        }
                    }
                }
            });
            || ()
        }, ());
    }

    // Load initial components when provided (always update, even if empty)
    {
        let components = components.clone();
        let initial_components = props.initial_components.clone();
        let selected_component = selected_component.clone();
        let editing_component = editing_component.clone();
        
        use_effect_with_deps(move |_| {
            // Always set components to reflect the current page, even if empty
            components.set(initial_components);
            // Clear any open modals when switching pages to prevent navigation blocking
            selected_component.set(None);
            editing_component.set(None);
            || ()
        }, (props.initial_components.clone(),));
    }

    // Auto-sync components back to parent when they change
    {
        let components = components.clone();
        let on_save = props.on_save.clone();
        
        use_effect_with_deps(move |current_components| {
            // Automatically notify parent of component changes
            on_save.emit(current_components.clone());
            || ()
        }, (*components).clone());
    }

    // Modal-specific escape key handling is implemented in the properties modal
    // (see onkeydown handler in the modal component below)

    let component_types = vec![
        ComponentType::Text,
        ComponentType::Heading,
        ComponentType::Subheading,
        ComponentType::Image,
        ComponentType::Button,
        ComponentType::Link,
        ComponentType::Container,
        ComponentType::Sidebar,
        ComponentType::TwoColumn,
        ComponentType::ThreeColumn,
        ComponentType::Hero,
        ComponentType::Card,
        ComponentType::List,
        ComponentType::Quote,
        ComponentType::Video,
        ComponentType::Spacer,
        ComponentType::Divider,
        ComponentType::ContactForm,
        ComponentType::Newsletter,
        ComponentType::Map,
        ComponentType::Gallery,
        ComponentType::PostsList,
        ComponentType::Comments,
        ComponentType::PageTitle,
        ComponentType::PublishedDate,
    ];

    let on_drag_start = {
        let dragging_component = dragging_component.clone();
        Callback::from(move |e: DragEvent| {
            if let Some(target) = e.target() {
                if let Ok(element) = target.dyn_into::<Element>() {
                    if let Some(component_type_str) = element.get_attribute("data-component-type") {
                        let component_type = match component_type_str.as_str() {
                            "Text" => ComponentType::Text,
                            "Heading" => ComponentType::Heading,
                            "Subheading" => ComponentType::Subheading,
                            "Image" => ComponentType::Image,
                            "Button" => ComponentType::Button,
                            "Link" => ComponentType::Link,
                            "Container" => ComponentType::Container,
                            "Sidebar" => ComponentType::Sidebar,
                            "TwoColumn" => ComponentType::TwoColumn,
                            "ThreeColumn" => ComponentType::ThreeColumn,
                            "Hero" => ComponentType::Hero,
                            "Card" => ComponentType::Card,
                            "List" => ComponentType::List,
                            "Quote" => ComponentType::Quote,
                            "Video" => ComponentType::Video,
                            "Spacer" => ComponentType::Spacer,
                            "Divider" => ComponentType::Divider,
                            "ContactForm" => ComponentType::ContactForm,
                            "Newsletter" => ComponentType::Newsletter,
                            "Map" => ComponentType::Map,
                            "Gallery" => ComponentType::Gallery,
                            "PostsList" => ComponentType::PostsList,
                            "Comments" => ComponentType::Comments,
                            "PageTitle" => ComponentType::PageTitle,
                            "PublishedDate" => ComponentType::PublishedDate,
                            _ => ComponentType::Text,
                        };
                        dragging_component.set(Some(component_type));
                    }
                }
            }
        })
    };

    let on_drag_over = {
        let drag_over = drag_over.clone();
        let dragging_component = dragging_component.clone();
        let dragging_existing_component = dragging_existing_component.clone();
        Callback::from(move |e: DragEvent| {
            // Only prevent default if we have something being dragged
            if (*dragging_component).is_some() || (*dragging_existing_component).is_some() {
                e.prevent_default();
                drag_over.set(true);
            }
        })
    };

    let on_drag_leave = {
        let drag_over = drag_over.clone();
        let container_drag_over = container_drag_over.clone();
        let column_drag_over = column_drag_over.clone();
        Callback::from(move |_: DragEvent| {
            drag_over.set(false);
            // Clear nested drag states when leaving the main builder area
            container_drag_over.set(None);
            column_drag_over.set(None);
        })
    };

    // on_drop will be defined after on_component_reorder

    // Callback for handling drops into nested container components
    let on_nested_drop = {
        let components = components.clone();
        let dragging_component = dragging_component.clone();
        let container_drag_over = container_drag_over.clone();
        let column_drag_over = column_drag_over.clone();
        Callback::from(move |(container_id, drop_zone): (String, String)| {
            // Clear drag-over states first
            container_drag_over.set(None);
            column_drag_over.set(None);
            
            if let Some(component_type) = (*dragging_component).clone() {
                let new_component = PageComponent {
                    id: uuid::Uuid::new_v4().to_string(),
                    component_type: component_type.clone(),
                    content: component_type.default_content(),
                    styles: ComponentStyles::default(),
                    position: Position::default(),
                    properties: ComponentProperties::default(),
                };
                
                let mut current_components = (*components).clone();
                
                // Find the container component and add the new component to the appropriate nested area
                if let Some(container) = current_components.iter_mut().find(|c| c.id == container_id) {
                    match drop_zone.as_str() {
                        "container" => container.properties.nested_components.push(new_component),
                        "column-1" => container.properties.column_1_components.push(new_component),
                        "column-2" => container.properties.column_2_components.push(new_component),
                        "column-3" => container.properties.column_3_components.push(new_component),
                        _ => {
                            // Log error for invalid drop zone but don't crash
                            web_sys::console::log_1(&format!("Invalid drop zone: {}", drop_zone).into());
                        }
                    }
                    
                    components.set(current_components);
                } else {
                    // Log error if container not found but don't crash
                    web_sys::console::log_1(&format!("Container not found: {}", container_id).into());
                }
                
                dragging_component.set(None);
            } else {
                // Log warning if no component is being dragged
                web_sys::console::log_1(&"No component being dragged".into());
                dragging_component.set(None);
            }
        })
    };

    let on_component_click = {
        let selected_component = selected_component.clone();
        Callback::from(move |component_id: String| {
            selected_component.set(Some(component_id));
        })
    };

    let on_component_edit = {
        let editing_component = editing_component.clone();
        Callback::from(move |component_id: String| {
            editing_component.set(Some(component_id));
        })
    };

    let on_component_delete = {
        let components = components.clone();
        let selected_component = selected_component.clone();
        Callback::from(move |component_id: String| {
            let mut current_components = (*components).clone();
            if remove_nested_component(&mut current_components, &component_id) {
                components.set(current_components);
                selected_component.set(None);
            }
        })
    };

    let on_component_duplicate = {
        let components = components.clone();
        Callback::from(move |component_id: String| {
            let mut current_components = (*components).clone();
            if duplicate_nested_component(&mut current_components, &component_id) {
                components.set(current_components);
            }
        })
    };

    let on_content_save = {
        let components = components.clone();
        let editing_component = editing_component.clone();
        Callback::from(move |(component_id, new_content): (String, String)| {
            let mut current_components = (*components).clone();
            if update_nested_component_content(&mut current_components, &component_id, new_content) {
                components.set(current_components);
                editing_component.set(None);
            }
        })
    };

    // Media picker callbacks
    let open_media_picker = {
        let show_media_picker = show_media_picker.clone();
        let media_picker_target_component = media_picker_target_component.clone();
        let media_picker_images_only = media_picker_images_only.clone();
        Callback::from(move |(component_id, images_only): (String, bool)| {
            web_sys::console::log_1(&format!("🎯 Opening media picker for component: {}", component_id).into());
            media_picker_target_component.set(Some(component_id));
            media_picker_images_only.set(images_only);
            show_media_picker.set(true);
        })
    };

    let close_media_picker = {
        let show_media_picker = show_media_picker.clone();
        let media_picker_target_component = media_picker_target_component.clone();
        Callback::from(move |_| {
            show_media_picker.set(false);
            media_picker_target_component.set(None);
        })
    };

    let on_media_select = {
        let components = components.clone();
        let media_picker_target_component = media_picker_target_component.clone();
        let show_media_picker = show_media_picker.clone();
        Callback::from(move |media_item: MediaItem| {
            web_sys::console::log_1(&format!("🎯 Media selected: {}", media_item.name).into());
            
            if let Some(ref component_id) = *media_picker_target_component {
                web_sys::console::log_1(&format!("🎯 Target component ID: {}", component_id).into());
                
                let mut current_components = (*components).clone();
                if let Some(component) = current_components.iter_mut().find(|c| c.id == *component_id) {
                    web_sys::console::log_1(&format!("🎯 Found component type: {:?}", component.component_type).into());
                    
                    match component.component_type {
                        ComponentType::Image => {
                            component.properties.image_url = format!("http://localhost:8081{}", media_item.url);
                            if component.properties.image_alt.is_empty() {
                                component.properties.image_alt = media_item.name;
                            }
                            web_sys::console::log_1(&"🎯 Updated Image component".into());
                        }
                        ComponentType::Video => {
                            component.properties.video_url = format!("http://localhost:8081{}", media_item.url);
                            web_sys::console::log_1(&"🎯 Updated Video component".into());
                        }
                        ComponentType::Gallery => {
                            web_sys::console::log_1(&"🎯 Processing Gallery component".into());
                            // Add the media item to the gallery images
                            let new_gallery_image = GalleryImage {
                                url: format!("http://localhost:8081{}", media_item.url),
                                alt: media_item.name.clone(),
                                caption: String::new(),
                                title: media_item.name,
                            };
                            component.properties.gallery_images.push(new_gallery_image);
                            web_sys::console::log_1(&format!("🎯 Added image to gallery. Total images: {}", component.properties.gallery_images.len()).into());
                        }
                        ComponentType::Sidebar | _ => {
                            web_sys::console::log_1(&format!("🎯 Unhandled component type: {:?}", component.component_type).into());
                        }
                    }
                } else {
                    web_sys::console::log_1(&format!("🎯 Component not found with ID: {}", component_id).into());
                }
                components.set(current_components);
            } else {
                web_sys::console::log_1(&"🎯 No target component ID set".into());
            }
            show_media_picker.set(false);
            media_picker_target_component.set(None);
        })
    };

    let on_multi_media_select = {
        let components = components.clone();
        let media_picker_target_component = media_picker_target_component.clone();
        let show_media_picker = show_media_picker.clone();
        Callback::from(move |media_items: Vec<MediaItem>| {
            web_sys::console::log_1(&format!("🎯 Multiple media selected: {} items", media_items.len()).into());
            
            if let Some(ref component_id) = *media_picker_target_component {
                web_sys::console::log_1(&format!("🎯 Target component ID: {}", component_id).into());
                
                let mut current_components = (*components).clone();
                if let Some(component) = current_components.iter_mut().find(|c| c.id == *component_id) {
                    web_sys::console::log_1(&format!("🎯 Found component type: {:?}", component.component_type).into());
                    
                    match component.component_type {
                        ComponentType::Gallery => {
                            web_sys::console::log_1(&"🎯 Processing Gallery component for multiple items".into());
                            // Add all media items to the gallery images
                            for media_item in media_items {
                                let new_gallery_image = GalleryImage {
                                    url: format!("http://localhost:8081{}", media_item.url),
                                    alt: media_item.name.clone(),
                                    caption: String::new(),
                                    title: media_item.name,
                                };
                                component.properties.gallery_images.push(new_gallery_image);
                            }
                            web_sys::console::log_1(&format!("🎯 Added images to gallery. Total images: {}", component.properties.gallery_images.len()).into());
                        }
                        _ => {
                            web_sys::console::log_1(&"🎯 Multi-select only supported for Gallery components".into());
                        }
                    }
                    
                    components.set(current_components);
                } else {
                    web_sys::console::log_1(&"🎯 Component not found".into());
                }
            }
            
            show_media_picker.set(false);
            media_picker_target_component.set(None);
        })
    };

    let on_property_update = {
        let components = components.clone();
        Callback::from(move |(component_id, property_name, prop_value): (String, String, String)| {
            let mut current_components = (*components).clone();
            if let Some(component) = current_components.iter_mut().find(|c| c.id == component_id) {
                match property_name.as_str() {
                    "image_url" => component.properties.image_url = prop_value,
                    "image_alt" => component.properties.image_alt = prop_value,
                    "image_title" => component.properties.image_title = prop_value,
                    "button_text" => component.properties.button_text = prop_value,
                    "button_url" => component.properties.button_url = prop_value,
                    "button_target" => component.properties.button_target = prop_value,
                    "button_size" => component.properties.button_size = prop_value,
                    "button_variant" => component.properties.button_variant = prop_value,
                    "button_icon" => component.properties.button_icon = prop_value,
                    
                    // Card properties
                    "card_title" => component.properties.card_title = prop_value,
                    "card_description" => component.properties.card_description = prop_value,
                    "card_image" => component.properties.card_image = prop_value,
                    "card_image_alt" => component.properties.card_image_alt = prop_value,
                    "card_background" => component.properties.card_background = prop_value,
                    "card_border_radius" => component.properties.card_border_radius = prop_value,
                    "card_shadow" => component.properties.card_shadow = prop_value,
                    "card_padding" => component.properties.card_padding = prop_value,
                    "card_meta_text" => component.properties.card_meta_text = prop_value,
                    "card_button_text" => component.properties.card_button_text = prop_value,
                    "card_button_url" => component.properties.card_button_url = prop_value,
                    
                    // Hero properties
                    "hero_badge_text" => component.properties.hero_badge_text = prop_value,
                    "hero_title" => component.properties.hero_title = prop_value,
                    "hero_subtitle" => component.properties.hero_subtitle = prop_value,
                    "hero_description" => component.properties.hero_description = prop_value,
                    "hero_background_type" => component.properties.hero_background_type = prop_value,
                    "hero_background_color" => component.properties.hero_background_color = prop_value,
                    "hero_background_gradient_start" => component.properties.hero_background_gradient_start = prop_value,
                    "hero_background_gradient_end" => component.properties.hero_background_gradient_end = prop_value,
                    "hero_background_image" => component.properties.hero_background_image = prop_value,
                    "hero_text_color" => component.properties.hero_text_color = prop_value,
                    "hero_alignment" => component.properties.hero_alignment = prop_value,
                    "hero_padding" => component.properties.hero_padding = prop_value,
                    "hero_min_height" => component.properties.hero_min_height = prop_value,
                    "hero_primary_button_text" => component.properties.hero_primary_button_text = prop_value,
                    "hero_primary_button_url" => component.properties.hero_primary_button_url = prop_value,
                    "hero_secondary_button_text" => component.properties.hero_secondary_button_text = prop_value,
                    "hero_secondary_button_url" => component.properties.hero_secondary_button_url = prop_value,
                    "hero_stat1_number" => component.properties.hero_stat1_number = prop_value,
                    "hero_stat1_label" => component.properties.hero_stat1_label = prop_value,
                    "hero_stat2_number" => component.properties.hero_stat2_number = prop_value,
                    "hero_stat2_label" => component.properties.hero_stat2_label = prop_value,
                    "hero_stat3_number" => component.properties.hero_stat3_number = prop_value,
                    "hero_stat3_label" => component.properties.hero_stat3_label = prop_value,
                    
                    // List properties
                    "list_background" => component.properties.list_background = prop_value,
                    "list_border_radius" => component.properties.list_border_radius = prop_value,
                    "list_padding" => component.properties.list_padding = prop_value,
                    "list_item_spacing" => component.properties.list_item_spacing = prop_value,
                    "list_text_color" => component.properties.list_text_color = prop_value,
                    
                    "video_url" => component.properties.video_url = prop_value,
                    "gallery_layout" => component.properties.gallery_layout = prop_value,
                    "gallery_columns" => {
                        if let Ok(columns) = prop_value.parse::<i32>() {
                            component.properties.gallery_columns = columns;
                        }
                    },
                    "gallery_gap" => {
                        if let Ok(gap) = prop_value.parse::<i32>() {
                            component.properties.gallery_gap = gap;
                        }
                    },
                    "gallery_border_radius" => {
                        if let Ok(radius) = prop_value.parse::<i32>() {
                            component.properties.gallery_border_radius = radius;
                        }
                    },
                    "gallery_show_captions" => {
                        component.properties.gallery_show_captions = prop_value == "true";
                    },
                    "gallery_enable_lightbox" => {
                        component.properties.gallery_enable_lightbox = prop_value == "true";
                    },
                    "gallery_enable_drag_reorder" => {
                        component.properties.gallery_enable_drag_reorder = prop_value == "true";
                    },
                    "gallery_images" => {
                        if let Ok(images) = serde_json::from_str::<Vec<GalleryImage>>(&prop_value) {
                            component.properties.gallery_images = images;
                        }
                    },
                    "divider_style" => component.properties.divider_style = prop_value,
                    "divider_thickness" => component.properties.divider_thickness = prop_value,
                    "divider_color" => component.properties.divider_color = prop_value,
                    "divider_margin" => component.properties.divider_margin = prop_value,
                    "divider_width" => component.properties.divider_width = prop_value,
                    
                    // PostsList properties
                    "posts_list_card_background" => component.properties.posts_list_card_background = prop_value,
                    "posts_list_grid_gap" => component.properties.posts_list_grid_gap = prop_value,
                    "posts_list_card_radius" => component.properties.posts_list_card_radius = prop_value,
                    "posts_list_card_shadow" => component.properties.posts_list_card_shadow = prop_value,
                    "posts_list_title_color" => component.properties.posts_list_title_color = prop_value,
                    "posts_list_meta_color" => component.properties.posts_list_meta_color = prop_value,
                    "posts_list_link_color" => component.properties.posts_list_link_color = prop_value,
                    "posts_list_columns" => {
                        if let Ok(columns) = prop_value.parse::<i32>() {
                            component.properties.posts_list_columns = columns;
                        }
                    },
                    "posts_list_count" => {
                        if let Ok(count) = prop_value.parse::<i32>() {
                            component.properties.posts_list_count = count;
                        }
                    },
                    "posts_list_excerpt_length" => {
                        if let Ok(length) = prop_value.parse::<i32>() {
                            component.properties.posts_list_excerpt_length = length;
                        }
                    },
                    _ => {}
                }
            }
            components.set(current_components);
        })
    };

    let on_boolean_property_update = {
        let components = components.clone();
        Callback::from(move |(component_id, property_name, prop_value): (String, String, bool)| {
            let mut current_components = (*components).clone();
            if let Some(component) = current_components.iter_mut().find(|c| c.id == component_id) {
                match property_name.as_str() {
                    "image_lazy_load" => component.properties.image_lazy_load = prop_value,
                    "video_autoplay" => component.properties.video_autoplay = prop_value,
                    "video_controls" => component.properties.video_controls = prop_value,
                    "video_muted" => component.properties.video_muted = prop_value,
                    "video_loop" => component.properties.video_loop = prop_value,
                    
                    // Card boolean properties
                    "card_button_show" => component.properties.card_button_show = prop_value,
                    
                    // Hero boolean properties
                    "hero_show_primary_button" => component.properties.hero_show_primary_button = prop_value,
                    "hero_show_secondary_button" => component.properties.hero_show_secondary_button = prop_value,
                    "hero_show_badge" => component.properties.hero_show_badge = prop_value,
                    "hero_show_stats" => component.properties.hero_show_stats = prop_value,
                    
                    // List boolean properties
                    "list_show_icons" => component.properties.list_show_icons = prop_value,
                    
                    // PostsList boolean properties
                    "posts_list_show_author" => component.properties.posts_list_show_author = prop_value,
                    "posts_list_show_date" => component.properties.posts_list_show_date = prop_value,
                    "posts_list_show_excerpt" => component.properties.posts_list_show_excerpt = prop_value,
                    "posts_list_show_view_all" => component.properties.posts_list_show_view_all = prop_value,
                    
                    // Page Title properties - TODO: Fix type inference issue
                    "page_title_tag" | "page_title_prefix" | "page_title_suffix" |
                    "page_title_show_prefix" | "page_title_show_suffix" |
                    "published_date_format" | "published_date_prefix" |
                    "published_date_show_prefix" | "published_date_show_time" | "published_date_relative" => {
                        // Temporarily disabled due to type inference issue
                        // Properties will use default values until this is resolved
                        web_sys::console::log_1(&format!("Property update for {} temporarily disabled", property_name).into());
                    },
                    _ => {}
                }
            }
            components.set(current_components);
        })
    };

    let on_style_update = {
        let components = components.clone();
        Callback::from(move |(component_id, style_name, style_value): (String, String, String)| {
            let mut current_components = (*components).clone();
            if let Some(component) = current_components.iter_mut().find(|c| c.id == component_id) {
                match style_name.as_str() {
                    "padding" => component.styles.padding = style_value,
                    "margin" => component.styles.margin = style_value,
                    "border_radius" => component.styles.border_radius = style_value,
                    "background_color" => component.styles.background_color = style_value,
                    "text_color" => component.styles.text_color = style_value,
                    "font_size" => component.styles.font_size = style_value,
                    "font_weight" => component.styles.font_weight = style_value,
                    "text_align" => component.styles.text_align = style_value,
                    "border_width" => component.styles.border_width = style_value,
                    "border_color" => component.styles.border_color = style_value,
                    "border_style" => component.styles.border_style = style_value,
                    "box_shadow" => component.styles.box_shadow = style_value,
                    "font_family" => component.styles.font_family = style_value,
                    "line_height" => component.styles.line_height = style_value,
                    "letter_spacing" => component.styles.letter_spacing = style_value,
                    "text_decoration" => component.styles.text_decoration = style_value,
                    "text_transform" => component.styles.text_transform = style_value,
                    "background_image" => component.styles.background_image = style_value,
                    "background_size" => component.styles.background_size = style_value,
                    "background_position" => component.styles.background_position = style_value,
                    "background_repeat" => component.styles.background_repeat = style_value,
                    "opacity" => {
                        if let Ok(opacity_value) = style_value.parse::<f32>() {
                            component.styles.opacity = opacity_value;
                        }
                    },
                    "z_index" => {
                        if let Ok(z_index_value) = style_value.parse::<i32>() {
                            component.styles.z_index = z_index_value;
                        }
                    },
                    _ => {}
                }
            }
            components.set(current_components);
        })
    };

    // Component reordering callbacks
    let on_component_drag_start = {
        let dragging_existing_component = dragging_existing_component.clone();
        Callback::from(move |component_id: String| {
            dragging_existing_component.set(Some(component_id));
        })
    };

    let on_component_drag_end = {
        let dragging_existing_component = dragging_existing_component.clone();
        let drop_zone_hover = drop_zone_hover.clone();
        Callback::from(move |_| {
            dragging_existing_component.set(None);
            drop_zone_hover.set(None);
        })
    };

    let on_drop_zone_enter = {
        let drop_zone_hover = drop_zone_hover.clone();
        Callback::from(move |drop_zone_id: String| {
            drop_zone_hover.set(Some(drop_zone_id));
        })
    };

    let on_drop_zone_leave = {
        let drop_zone_hover = drop_zone_hover.clone();
        Callback::from(move |_| {
            drop_zone_hover.set(None);
        })
    };

    let on_component_reorder = {
        let components = components.clone();
        let dragging_existing_component = dragging_existing_component.clone();
        let drop_zone_hover = drop_zone_hover.clone();
        Callback::from(move |target_position: usize| {
            web_sys::console::log_1(&format!("🔄 Reorder callback triggered! Target position: {}", target_position).into());
            
            if let Some(dragged_id) = (*dragging_existing_component).clone() {
                web_sys::console::log_1(&format!("📦 Dragging component: {}", dragged_id).into());
                let mut current_components = (*components).clone();
                
                web_sys::console::log_1(&format!("📋 Current components count: {}", current_components.len()).into());
                
                // Find the dragged component and remove it
                if let Some(dragged_index) = current_components.iter().position(|c| c.id == dragged_id) {
                    web_sys::console::log_1(&format!("📍 Found dragged component at index: {}", dragged_index).into());
                    let dragged_component = current_components.remove(dragged_index);
                    
                    // Insert at the new position
                    let insert_position = if target_position > dragged_index {
                        target_position - 1
                    } else {
                        target_position
                    };
                    
                    web_sys::console::log_1(&format!("🎯 Inserting at position: {} (calculated from target: {})", insert_position, target_position).into());
                    
                    current_components.insert(insert_position.min(current_components.len()), dragged_component);
                    components.set(current_components);
                    
                    web_sys::console::log_1(&"✅ Component reordered successfully!".into());
                } else {
                    web_sys::console::log_1(&format!("❌ Could not find component with id: {}", dragged_id).into());
                }
                
                // Clear drag state
                dragging_existing_component.set(None);
                drop_zone_hover.set(None);
            } else {
                web_sys::console::log_1(&"❌ No component being dragged!".into());
            }
        })
    };

    // Main canvas drop handler (defined after on_component_reorder)
    let on_drop = {
        let components = components.clone();
        let drag_over = drag_over.clone();
        let dragging_component = dragging_component.clone();
        let dragging_existing_component = dragging_existing_component.clone();
        let on_component_reorder = on_component_reorder.clone();
        let container_drag_over = container_drag_over.clone();
        let column_drag_over = column_drag_over.clone();
        let component_templates = component_templates.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_over.set(false);
            
            // Clear all drag-over states
            container_drag_over.set(None);
            column_drag_over.set(None);
            
            // Check if we're reordering an existing component
            if (*dragging_existing_component).is_some() {
                web_sys::console::log_1(&"🔄 Main canvas drop - reordering to end of list".into());
                let current_components_len = (*components).len();
                on_component_reorder.emit(current_components_len);
                return;
            }
            
            // Otherwise, handle new component drop
            if let Some(component_type) = (*dragging_component).clone() {
                let mut new_component = PageComponent {
                    id: uuid::Uuid::new_v4().to_string(),
                    component_type: component_type.clone(),
                    content: component_type.default_content(),
                    styles: ComponentStyles::default(),
                    position: Position::default(),
                    properties: ComponentProperties::default(),
                };
                if let ComponentType::Sidebar = component_type {
                    if let Some(sidebar) = (*component_templates)
                        .iter()
                        .find(|t| t.component_type == "sidebar")
                    {
                        if let Some(obj) = sidebar.template_data.as_object() {
                            if let Some(pos) = obj.get("position").and_then(|v| v.as_str()) {
                                new_component.properties.sidebar_position = pos.to_string();
                            }
                            if let Some(width) = obj.get("width").and_then(|v| v.as_str()) {
                                new_component.properties.sidebar_width = width.to_string();
                            }
                            if let Some(sticky) = obj.get("sticky").and_then(|v| v.as_bool()) {
                                new_component.properties.sidebar_sticky = sticky;
                            }
                        }
                    }
                }
                
                let mut current_components = (*components).clone();
                current_components.push(new_component);
                components.set(current_components);
            } else {
                // Log warning but don't crash if no component is being dragged
                web_sys::console::log_1(&"No component being dragged to main canvas".into());
            }
            
            // Always clear the dragging state
            dragging_component.set(None);
            dragging_existing_component.set(None);
        })
    };

    html! {
        <>
        <div class="drag-drop-page-builder">
            <div class="builder-layout">
                // Component Library Sidebar
                <div class="component-library">
                    <h3>{"Components"}</h3>
                    <div class="component-grid">
                        {component_types.iter().map(|component_type| {
                            let component_type_str = serde_json::to_string(component_type).unwrap().trim_matches('"').to_string();
                            html! {
                                <div 
                                    class="component-item"
                                    draggable="true"
                                    ondragstart={on_drag_start.clone()}
                                    data-component-type={component_type_str}
                                >
                                    <span class="component-icon">{component_type.icon()}</span>
                                    <span class="component-name">{component_type.display_name()}</span>
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Main Canvas Area
                <div class="builder-canvas">
                    <div 
                        class={classes!("drop-zone", 
                            if *drag_over { Some("drag-over") } else { None },
                            if (*dragging_existing_component).is_some() { Some("reordering") } else { None }
                        )}
                        ondragover={on_drag_over}
                        ondragleave={on_drag_leave}
                        ondrop={on_drop}
                        onclick={{
                            let selected_component = selected_component.clone();
                            Callback::from(move |e: web_sys::MouseEvent| {
                                // Only deselect if clicking on the canvas itself (not on a component)
                                if let Some(target) = e.target() {
                                    if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                                        let class_name = element.class_name();
                                        if class_name.contains("drop-zone") || 
                                           class_name.contains("canvas-components") ||
                                           class_name.contains("empty-canvas") {
                                            selected_component.set(None);
                                        }
                                    }
                                }
                            })
                        }}
                    >
                        {if components.is_empty() {
                            html! {
                                <div class="empty-canvas">
                                    <div class="empty-message">
                                        <h3>{"Start Building Your Page"}</h3>
                                        <p>{"Drag components from the left sidebar to start building your page."}</p>
                                    </div>
                                </div>
                            }
                        } else {
                            {
                                let mut elements = Vec::new();
                                        
                                        // Add drop zone at the beginning
                                        let is_drop_zone_active = drop_zone_hover.as_ref() == Some(&format!("drop-zone-0"));
                                        let drop_zone_class = if is_drop_zone_active { "component-drop-zone active" } else { "component-drop-zone" };
                                        
                                        let on_drop_zone_dragover = {
                                            let on_drop_zone_enter = on_drop_zone_enter.clone();
                                            let dragging_existing_component = dragging_existing_component.clone();
                                            Callback::from(move |e: DragEvent| {
                                                // Only allow drop if we have a component being dragged
                                                if (*dragging_existing_component).is_some() {
                                                    e.prevent_default(); // This is crucial for allowing drop
                                                    web_sys::console::log_1(&"🎯 Hovering over top drop zone (position 0)".into());
                                                    on_drop_zone_enter.emit(format!("drop-zone-0"));
                                                }
                                            })
                                        };
                                        
                                        let on_drop_zone_drop = {
                                            let on_component_reorder = on_component_reorder.clone();
                                            let dragging_existing_component = dragging_existing_component.clone();
                                            Callback::from(move |e: DragEvent| {
                                                e.prevent_default();
                                                e.stop_propagation(); // Prevent main canvas from handling this
                                                // Check if we have a component being dragged
                                                if (*dragging_existing_component).is_some() {
                                                    web_sys::console::log_1(&"🎯 Dropping at position 0 (top drop zone)".into());
                                                    on_component_reorder.emit(0);
                                                } else {
                                                    web_sys::console::log_1(&"❌ No component being dragged to top drop zone".into());
                                                }
                                            })
                                        };
                                        
                                        elements.push(html! {
                                            <div 
                                                class={drop_zone_class}
                                                ondragover={on_drop_zone_dragover}
                                                ondrop={on_drop_zone_drop}
                                                ondragleave={on_drop_zone_leave.clone()}
                                            />
                                        });
                                        
                                        // Add components with drop zones between them
                                        for (index, component) in components.iter().enumerate() {
                                            let is_selected = selected_component.as_ref() == Some(&component.id);
                                            let is_editing = editing_component.as_ref() == Some(&component.id);
                                            let is_being_dragged = dragging_existing_component.as_ref() == Some(&component.id);
                                            
                                            // Add the component with drag functionality
                                            // Always render the component, but apply different styles when being dragged
                                                if is_editing {
                                                    let component_id = component.id.clone();
                                                    let content = component.content.clone();
                                                    let on_save = {
                                                        let on_content_save = on_content_save.clone();
                                                        let component_id = component_id.clone();
                                                        Callback::from(move |new_content: String| {
                                                            on_content_save.emit((component_id.clone(), new_content));
                                                        })
                                                    };
                                                    let on_cancel = {
                                                        let editing_component = editing_component.clone();
                                                        Callback::from(move |_| {
                                                            editing_component.set(None);
                                                        })
                                                    };
                                                    let on_save_click = {
                                                        let on_save = on_save.clone();
                                                        let content = content.clone();
                                                        Callback::from(move |_| {
                                                            on_save.emit(content.clone());
                                                        })
                                                    };
                                                    
                                                    elements.push(html! {
                                                        <div class="component-editor">
                                                            <div class="editor-header">
                                                                <h4>{format!("Editing: {}", component.component_type.display_name())}</h4>
                                                                <div class="editor-actions">
                                                                    <button class="btn btn-secondary" onclick={on_cancel}>{"Cancel"}</button>
                                                                    <button class="btn" onclick={on_save_click}>{"Save"}</button>
                                                                </div>
                                                            </div>
                                                            <MarkdownEditor
                                                                value={component.content.clone()}
                                                                on_change={on_save}
                                                                placeholder={Some(format!("Enter content for your {}...", component.component_type.display_name()))}
                                                                rows={Some(15)}
                                                            />
                                                        </div>
                                                    });
                                                } else {
                                                    let component_id = component.id.clone();
                                                    let on_click = {
                                                        let on_component_click = on_component_click.clone();
                                                        let component_id = component_id.clone();
                                                        Callback::from(move |_| {
                                                            on_component_click.emit(component_id.clone());
                                                        })
                                                    };
                                                    let on_edit = {
                                                        let on_component_edit = on_component_edit.clone();
                                                        let component_id = component_id.clone();
                                                        Callback::from(move |_| {
                                                            on_component_edit.emit(component_id.clone());
                                                        })
                                                    };
                                                    let on_delete = {
                                                        let on_component_delete = on_component_delete.clone();
                                                        let component_id = component_id.clone();
                                                        Callback::from(move |_| {
                                                            on_component_delete.emit(component_id.clone());
                                                        })
                                                    };
                                                    let on_duplicate = {
                                                        let on_component_duplicate = on_component_duplicate.clone();
                                                        let component_id = component_id.clone();
                                                        Callback::from(move |_| {
                                                            on_component_duplicate.emit(component_id.clone());
                                                        })
                                                    };
                                                    
                                                    // Drag event handlers for existing components
                                                    let on_drag_start = {
                                                        let on_component_drag_start = on_component_drag_start.clone();
                                                        let component_id = component_id.clone();
                                                        Callback::from(move |_e: DragEvent| {
                                                            // Don't prevent default - we want the drag to work
                                                            // Set drag data for the component using the event target
                                                            web_sys::console::log_1(&format!("Drag started for component: {}", component_id).into());
                                                            on_component_drag_start.emit(component_id.clone());
                                                        })
                                                    };
                                                    
                                                    let on_drag_end = {
                                                        let on_component_drag_end = on_component_drag_end.clone();
                                                        Callback::from(move |_: DragEvent| {
                                                            on_component_drag_end.emit(());
                                                        })
                                                    };
                                                    
                                                    let selection_border = if is_selected {
                                                        "3px solid #007bff"
                                                    } else {
                                                        &format!("{} {} {}", component.styles.border_width, component.styles.border_style, component.styles.border_color)
                                                    };
                                                    
                                                    let selection_box_shadow = if is_selected {
                                                        format!("{}, 0 0 0 2px rgba(0, 123, 255, 0.25)", component.styles.box_shadow)
                                                    } else {
                                                        component.styles.box_shadow.clone()
                                                    };
                                                    
                                                    // Apply ghost effect when being dragged
                                                    let drag_opacity = if is_being_dragged { "0.3" } else { &component.styles.opacity.to_string() };
                                                    let drag_transform = if is_being_dragged { "scale(0.95)" } else { "scale(1)" };
                                                    
                                                    elements.push(html! {
                                                        <div 
                                                            class={classes!("canvas-component", if is_selected { Some("selected") } else { None })}
                                                            draggable="true"
                                                            ondragstart={on_drag_start}
                                                            ondragend={on_drag_end}
                                                            onclick={on_click}
                                                            style={format!(
                                                                "background-color: {}; color: {}; padding: {}; margin: {}; border-radius: {}; font-size: {}; font-weight: {}; text-align: {}; border: {}; box-shadow: {}; opacity: {}; z-index: {}; font-family: {}; line-height: {}; letter-spacing: {}; text-decoration: {}; text-transform: {}; background-image: {}; background-size: {}; background-position: {}; background-repeat: {}; position: relative; cursor: move; transform: {};",
                                                                component.styles.background_color,
                                                                component.styles.text_color,
                                                                component.styles.padding,
                                                                component.styles.margin,
                                                                component.styles.border_radius,
                                                                component.styles.font_size,
                                                                component.styles.font_weight,
                                                                component.styles.text_align,
                                                                selection_border,
                                                                selection_box_shadow,
                                                                drag_opacity,
                                                                if is_selected { "10" } else { &component.styles.z_index.to_string() },
                                                                component.styles.font_family,
                                                                component.styles.line_height,
                                                                component.styles.letter_spacing,
                                                                component.styles.text_decoration,
                                                                component.styles.text_transform,
                                                                component.styles.background_image,
                                                                component.styles.background_size,
                                                                component.styles.background_position,
                                                                component.styles.background_repeat,
                                                                drag_transform
                                                            )}
                                                        >
                                                            // Drag handle
                                                            <div 
                                                                class="drag-handle" 
                                                                style="position: absolute; top: 4px; left: 4px; width: 20px; height: 20px; background: rgba(0,123,255,0.8); color: white; border-radius: 3px; display: flex; align-items: center; justify-content: center; font-size: 12px; cursor: grab; z-index: 12; opacity: 0; transition: opacity 0.2s; pointer-events: none;"
                                                                title="Drag to reorder"
                                                            >
                                                                {"⋮⋮"}
                                                            </div>
                                                            
                                                            {render_component_content_with_drop_zones(
                                                                component, 
                                                                on_nested_drop.clone(), 
                                                                dragging_component.clone(), 
                                                                container_drag_over.clone(), 
                                                                column_drag_over.clone(),
                                                                selected_component.clone(),
                                                                on_component_click.clone(),
                                                                on_component_edit.clone(),
                                                                on_component_duplicate.clone(),
                                                                on_component_delete.clone()
                                                            )}
                                                            
                                                            {if is_selected {
                                                                html! {
                                                                    <>
                                                                        <div class="selection-indicator" style="position: absolute; top: -8px; left: -8px; background: #007bff; color: white; padding: 2px 6px; border-radius: 3px; font-size: 10px; font-weight: 600; z-index: 11; box-shadow: 0 2px 4px rgba(0,0,0,0.2);">
                                                                            {"✓ Selected"}
                                                                        </div>
                                                                    <div class="component-controls">
                                                                        <button class="control-btn" onclick={on_edit} title="Edit Content">{"✏️"}</button>
                                                                        <button class="control-btn" onclick={on_duplicate} title="Duplicate">{"📋"}</button>
                                                                        <button class="control-btn" onclick={on_delete} title="Delete">{"🗑️"}</button>
                                                                    </div>
                                                                    </>
                                                                }
                                                            } else {
                                                                html! {}
                                                            }}
                                                        </div>
                                                    });
                                                }
                                            
                                            // Add drop zone after each component (except the last one)
                                            if index < components.len() - 1 {
                                                let drop_zone_id = format!("drop-zone-{}", index + 1);
                                                let is_drop_zone_active = drop_zone_hover.as_ref() == Some(&drop_zone_id);
                                                let drop_zone_class = if is_drop_zone_active { "component-drop-zone active" } else { "component-drop-zone" };
                                                
                                                let on_drop_zone_dragover = {
                                                    let on_drop_zone_enter = on_drop_zone_enter.clone();
                                                    let dragging_existing_component = dragging_existing_component.clone();
                                                    let drop_zone_id = drop_zone_id.clone();
                                                    Callback::from(move |e: DragEvent| {
                                                        // Only allow drop if we have a component being dragged
                                                        if (*dragging_existing_component).is_some() {
                                                            e.prevent_default(); // This is crucial for allowing drop
                                                            e.stop_propagation(); // Prevent main canvas from handling this
                                                            web_sys::console::log_1(&format!("🎯 Hovering over drop zone: {}", drop_zone_id).into());
                                                            on_drop_zone_enter.emit(drop_zone_id.clone());
                                                        }
                                                    })
                                                };
                                                
                                                let on_drop_zone_drop = {
                                                    let on_component_reorder = on_component_reorder.clone();
                                                    let dragging_existing_component = dragging_existing_component.clone();
                                                    let target_index = index + 1;
                                                    Callback::from(move |e: DragEvent| {
                                                        e.prevent_default();
                                                        e.stop_propagation(); // Prevent main canvas from handling this
                                                        // Check if we have a component being dragged
                                                        if (*dragging_existing_component).is_some() {
                                                            web_sys::console::log_1(&format!("🎯 Dropping at position {} (drop zone)", target_index).into());
                                                            on_component_reorder.emit(target_index);
                                                        } else {
                                                            web_sys::console::log_1(&format!("❌ No component being dragged to drop zone {}", target_index).into());
                                                        }
                                                    })
                                                };
                                                
                                                elements.push(html! {
                                                    <div 
                                                        class={drop_zone_class}
                                                        ondragover={on_drop_zone_dragover}
                                                        ondrop={on_drop_zone_drop}
                                                        ondragleave={on_drop_zone_leave.clone()}
                                                    />
                                                });
                                            }
                                        }
                                        
                                        html! {
                                            <div class="canvas-components">
                                                {Html::from_iter(elements.into_iter())}
                                            </div>
                                        }
                            }
                        }}
                    </div>
                </div>
            </div>
        </div>

        // Properties Modal - Opens when editing_component is set
            {if let Some(editing_id) = editing_component.as_ref() {
                // Find the component from current state each render to ensure reactivity (including nested components)
                if let Some(component) = find_component_by_id(&(*components), editing_id) {
                    let close_modal = {
                        let editing_component = editing_component.clone();
                        Callback::from(move |e: MouseEvent| {
                            e.prevent_default();
                            editing_component.set(None);
                        })
                    };

                    let close_modal_overlay = {
                        let editing_component = editing_component.clone();
                        Callback::from(move |e: MouseEvent| {
                            // Only close if clicking on the overlay itself, not the content
                            if let Some(target) = e.target() {
                                if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                                    if element.class_name().contains("properties-modal") {
                                        e.prevent_default();
                                        editing_component.set(None);
                                    }
                                }
                            }
                        })
                    };

                    html! {
                        <div 
                            class="properties-modal" 
                            key={format!("modal-{}", component.id)}
                            onclick={close_modal_overlay}
                            onkeydown={{
                                let editing_component = editing_component.clone();
                                Callback::from(move |e: KeyboardEvent| {
                                    if e.key() == "Escape" {
                                        e.prevent_default();
                                        editing_component.set(None);
                                    }
                                })
                            }}
                            tabindex="-1"
                        >
                            <div 
                                class="properties-modal-content"
                                onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                            >
                                <div class="modal-header">
                                    <div>
                                        <h3>{"Edit Component"}</h3>
                                        <div class="component-type-badge">
                                            <span class="badge-icon">{component.component_type.icon()}</span>
                                            <span class="badge-text">{component.component_type.display_name()}</span>
                                        </div>
                                    </div>
                                    <button class="modal-close-btn" onclick={{
                                        let editing_component = editing_component.clone();
                                        Callback::from(move |e: MouseEvent| {
                                            e.prevent_default();
                                            editing_component.set(None);
                                        })
                                    }}>{"✕"}</button>
                                </div>

                                <div class="modal-body">
                                    <div class="property-sections">
                                        // Content Section (hide for image, video, and list components as they use component-specific properties)
                                        {if !matches!(component.component_type, ComponentType::Image | ComponentType::Video | ComponentType::List) {
                                            html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Content"}</h4>
                                                    <div class="property-group">
                                                        <label>{"Content Editor"}</label>
                                                        <MarkdownEditor
                                                            value={component.content.clone()}
                                                            on_change={{
                                                                let on_content_save = on_content_save.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |new_content: String| {
                                                                    on_content_save.emit((component_id.clone(), new_content));
                                                                })
                                                            }}
                                                            placeholder={Some(format!("Enter content for your {}...", component.component_type.display_name()))}
                                                            rows={Some(10)}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Preview"}</label>
                                                        <div class="property-preview">
                                                            {render_component_content(component)}
                                                        </div>
                                                    </div>
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }}

                                        // List Properties Section (show at top for List components)
                                        {if matches!(component.component_type, ComponentType::List) {
                                            html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"List Properties"}</h4>
                                                    
                                                    // List Items Management
                                                    <div class="property-group">
                                                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                                                            <label style="font-weight: 600;">{"List Items"}</label>
                                                            <button 
                                                                type="button"
                                                                style="padding: 6px 12px; background: #6b7280; color: white; border: none; border-radius: 4px; cursor: not-allowed; font-size: 12px;"
                                                                disabled=true
                                                                title="Add/Remove items will be implemented in next update"
                                                            >
                                                                {"+ Add Item (Coming Soon)"}
                                                            </button>
                                                        </div>
                                                        
                                                        <div style="max-height: 300px; overflow-y: auto; border: 1px solid #e1e5e9; border-radius: 6px; padding: 8px;">
                                                            {component.properties.list_items.iter().enumerate().map(|(index, item)| {
                                                                html! {
                                                                    <div style="border: 1px solid #e1e5e9; border-radius: 6px; padding: 12px; margin-bottom: 8px; background: #f8f9fa;">
                                                                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                                                                            <span style="font-weight: 600; font-size: 12px; color: #666;">
                                                                                {format!("Item #{}", index + 1)}
                                                                            </span>
                                                                            if component.properties.list_items.len() > 1 {
                                                                                <button 
                                                                                    type="button"
                                                                                    style="padding: 2px 6px; background: #9ca3af; color: white; border: none; border-radius: 3px; cursor: not-allowed; font-size: 10px;"
                                                                                    disabled=true
                                                                                    title="Add/Remove items will be implemented in next update"
                                                                                >
                                                                                    {"Remove"}
                                                                                </button>
                                                                            }
                                                                        </div>
                                                                        
                                                                        if component.properties.list_show_icons {
                                                                            <div style="margin-bottom: 8px;">
                                                                                <label style="font-size: 12px; display: block; margin-bottom: 4px;">{"Icon"}</label>
                                                                                <input 
                                                                                    type="text" 
                                                                                    value={item.icon.clone()}
                                                                                    placeholder="✓"
                                                                                    style="width: 100%; padding: 4px 8px; border: 1px solid #d1d5db; border-radius: 4px; font-size: 12px; background: #f9f9f9;"
                                                                                    readonly=true
                                                                                    title="Item editing will be implemented in next update"
                                                                                />
                                                                            </div>
                                                                        }
                                                                        
                                                                        <div style="margin-bottom: 8px;">
                                                                            <label style="font-size: 12px; display: block; margin-bottom: 4px;">{"Title"}</label>
                                                                            <input 
                                                                                type="text" 
                                                                                value={item.title.clone()}
                                                                                placeholder="List item title"
                                                                                style="width: 100%; padding: 4px 8px; border: 1px solid #d1d5db; border-radius: 4px; font-size: 12px; background: #f9f9f9;"
                                                                                readonly=true
                                                                                title="Item editing will be implemented in next update"
                                                                            />
                                                                        </div>
                                                                        
                                                                        <div>
                                                                            <label style="font-size: 12px; display: block; margin-bottom: 4px;">{"Description"}</label>
                                                                            <textarea 
                                                                                value={item.description.clone()}
                                                                                placeholder="Item description"
                                                                                style="width: 100%; padding: 4px 8px; border: 1px solid #d1d5db; border-radius: 4px; font-size: 12px; min-height: 60px; resize: vertical; background: #f9f9f9;"
                                                                                readonly=true
                                                                                title="Item editing will be implemented in next update"
                                                                            />
                                                                        </div>
                                                                    </div>
                                                                }
                                                            }).collect::<Html>()}
                                                        </div>
                                                    </div>
                                                    
                                                    // Display Settings
                                                    <div class="property-group">
                                                        <label>
                                                            <input type="checkbox" 
                                                                checked={component.properties.list_show_icons}
                                                                onchange={
                                                                    let on_boolean_property_update = on_boolean_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                                        on_boolean_property_update.emit((component_id.clone(), "list_show_icons".to_string(), target.checked()));
                                                                    })
                                                                }
                                                            />
                                                            {" Show Icons"}
                                                        </label>
                                                    </div>
                                                    
                                                    // List-specific styling Settings
                                                    <div class="property-group">
                                                        <label>{"List Background Color"}</label>
                                                        <input type="color"
                                                            value={component.properties.list_background.clone()}
                                                            oninput={
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "list_background".to_string(), target.value()));
                                                                })
                                                            }
                                                        />
                                                    </div>
                                                    
                                                    <div class="property-group">
                                                        <label>{"List Text Color"}</label>
                                                        <input type="color"
                                                            value={component.properties.list_text_color.clone()}
                                                            oninput={
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "list_text_color".to_string(), target.value()));
                                                                })
                                                            }
                                                        />
                                                    </div>
                                                    
                                                    // Layout Settings
                                                    <div class="property-group">
                                                        <label>{"Border Radius"}</label>
                                                        <input type="text" 
                                                            value={component.properties.list_border_radius.clone()}
                                                            placeholder="12px"
                                                            oninput={
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "list_border_radius".to_string(), target.value()));
                                                                })
                                                            }
                                                        />
                                                    </div>
                                                    
                                                    <div class="property-group">
                                                        <label>{"Padding"}</label>
                                                        <input type="text" 
                                                            value={component.properties.list_padding.clone()}
                                                            placeholder="32px"
                                                            oninput={
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "list_padding".to_string(), target.value()));
                                                                })
                                                            }
                                                        />
                                                    </div>
                                                    
                                                    <div class="property-group">
                                                        <label>{"Item Spacing"}</label>
                                                        <input type="text" 
                                                            value={component.properties.list_item_spacing.clone()}
                                                            placeholder="16px"
                                                            oninput={
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "list_item_spacing".to_string(), target.value()));
                                                                })
                                                            }
                                                        />
                                                    </div>
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }}

                                        // Style Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"Colors & Background"}</h4>
                                            <div class="property-group">
                                                <label>{"Background Color"}</label>
                                                <div class="color-input-group">
                                                    <input type="color" value={component.styles.background_color.clone()} />
                                                    <input type="text" value={component.styles.background_color.clone()} placeholder="transparent" />
                                                </div>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Text Color"}</label>
                                                <div class="color-input-group">
                                                    <input type="color" value={component.styles.text_color.clone()} />
                                                    <input type="text" value={component.styles.text_color.clone()} placeholder="var(--public-text-primary)" />
                                                </div>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Background Image URL"}</label>
                                                <input type="text" value={component.styles.background_image.clone()} placeholder="https://..." />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Background Size"}</label>
                                                <select value={component.styles.background_size.clone()}>
                                                    <option value="cover">{"Cover"}</option>
                                                    <option value="contain">{"Contain"}</option>
                                                    <option value="auto">{"Auto"}</option>
                                                    <option value="100% 100%">{"Stretch"}</option>
                                                </select>
                                            </div>
                                        </div>
                                        
                                        // Typography Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"Typography"}</h4>
                                            <div class="property-group">
                                                <label>{"Font Family"}</label>
                                                <select value={component.styles.font_family.clone()}>
                                                    <option value="system-ui, -apple-system, sans-serif">{"System UI"}</option>
                                                    <option value="Georgia, serif">{"Georgia"}</option>
                                                    <option value="Times New Roman, serif">{"Times New Roman"}</option>
                                                    <option value="Arial, sans-serif">{"Arial"}</option>
                                                    <option value="Helvetica, sans-serif">{"Helvetica"}</option>
                                                    <option value="Courier New, monospace">{"Courier New"}</option>
                                                </select>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Font Size"}</label>
                                                <input type="text" value={component.styles.font_size.clone()} placeholder="16px" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Font Weight"}</label>
                                                <select value={component.styles.font_weight.clone()}>
                                                    <option value="100">{"Thin"}</option>
                                                    <option value="300">{"Light"}</option>
                                                    <option value="normal">{"Normal"}</option>
                                                    <option value="500">{"Medium"}</option>
                                                    <option value="600">{"Semi Bold"}</option>
                                                    <option value="bold">{"Bold"}</option>
                                                    <option value="800">{"Extra Bold"}</option>
                                                    <option value="900">{"Black"}</option>
                                                </select>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Line Height"}</label>
                                                <input type="text" value={component.styles.line_height.clone()} placeholder="1.5" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Letter Spacing"}</label>
                                                <input type="text" value={component.styles.letter_spacing.clone()} placeholder="normal" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Text Transform"}</label>
                                                <select value={component.styles.text_transform.clone()}>
                                                    <option value="none">{"None"}</option>
                                                    <option value="uppercase">{"Uppercase"}</option>
                                                    <option value="lowercase">{"Lowercase"}</option>
                                                    <option value="capitalize">{"Capitalize"}</option>
                                                </select>
                                            </div>
                                        </div>
                                        
                                        // Enhanced Spacing & Layout Section  
                                        <div class="property-section">
                                            <h4 class="section-title">{"🎯 Spacing & Layout"}</h4>
                                            
                                            // Padding Controls
                                            <div class="property-group">
                                                <label style="font-weight: 600; margin-bottom: 8px; display: block;">{"Padding"}</label>
                                                <input 
                                                    type="text" 
                                                    value={component.styles.padding.clone()} 
                                                    placeholder="16px or 8px, 16px, 8px, 16px"
                                                    style="font-family: monospace; width: 100%; padding: 8px; border: 1px solid #dee2e6; border-radius: 4px;"
                                                    oninput={{
                                                        let on_style_update = on_style_update.clone();
                                                        let component_id = component.id.clone();
                                                        Callback::from(move |e: InputEvent| {
                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                            on_style_update.emit((component_id.clone(), "padding".to_string(), target.value()));
                                                        })
                                                    }}
                                                />
                                                <div style="font-size: 11px; color: #6c757d; margin-top: 4px;">
                                                    {"Examples: '16px', '8px, 16px', '8px, 16px, 12px, 16px' (top, right, bottom, left)"}
                                                </div>
                                            </div>
                                            
                                            // Margin Controls
                                            <div class="property-group">
                                                <label style="font-weight: 600; margin-bottom: 8px; display: block;">{"Margin"}</label>
                                                <input 
                                                    type="text" 
                                                    value={component.styles.margin.clone()} 
                                                    placeholder="0px or 8px, 0px, 16px, 0px"
                                                    style="font-family: monospace; width: 100%; padding: 8px; border: 1px solid #dee2e6; border-radius: 4px;"
                                                    oninput={{
                                                        let on_style_update = on_style_update.clone();
                                                        let component_id = component.id.clone();
                                                        Callback::from(move |e: InputEvent| {
                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                            on_style_update.emit((component_id.clone(), "margin".to_string(), target.value()));
                                                        })
                                                    }}
                                                />
                                                <div style="font-size: 11px; color: #6c757d; margin-top: 4px;">
                                                    {"Use 'auto' for centering, '0' for no margin. Examples: '16px', 'auto, 0px', '8px, auto, 16px, auto'"}
                                                </div>
                                            </div>
                                            
                                            // Border Radius Controls
                                            <div class="property-group">
                                                <label style="font-weight: 600; margin-bottom: 8px; display: block;">{"Border Radius"}</label>
                                                <input 
                                                    type="text" 
                                                    value={component.styles.border_radius.clone()} 
                                                    placeholder="4px or 8px, 8px, 0px, 0px"
                                                    style="font-family: monospace; width: 100%; padding: 8px; border: 1px solid #dee2e6; border-radius: 4px;"
                                                    oninput={{
                                                        let on_style_update = on_style_update.clone();
                                                        let component_id = component.id.clone();
                                                        Callback::from(move |e: InputEvent| {
                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                            on_style_update.emit((component_id.clone(), "border_radius".to_string(), target.value()));
                                                        })
                                                    }}
                                                />
                                                <div style="font-size: 11px; color: #6c757d; margin-top: 4px;">
                                                    {"Use '50%' for circular, '0' for sharp corners. Examples: '8px', '8px, 4px', '8px, 8px, 0px, 0px'"}
                                                </div>
                                            </div>
                                            
                                            // Width & Height Controls
                                            <div class="property-group">
                                                <label style="font-weight: 600; margin-bottom: 8px; display: block;">{"Dimensions"}</label>
                                                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px;">
                                                    <div>
                                                        <label style="font-size: 12px; color: #6c757d; margin-bottom: 4px; display: block;">{"Width"}</label>
                                                        <select 
                                                            value={component.position.width.clone()}
                                                            onchange={{
                                                                let components = components.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    let mut current_components = (*components).clone();
                                                                    if let Some(comp) = current_components.iter_mut().find(|c| c.id == component_id) {
                                                                        comp.position.width = target.value();
                                                                    }
                                                                    components.set(current_components);
                                                                })
                                                            }}
                                                            style="width: 100%; padding: 6px; border: 1px solid #dee2e6; border-radius: 4px; font-size: 12px;"
                                                        >
                                                            <option value="auto">{"Auto"}</option>
                                                            <option value="100%">{"Full (100%)"}</option>
                                                            <option value="75%">{"3/4 (75%)"}</option>
                                                            <option value="66.67%">{"2/3 (66%)"}</option>
                                                            <option value="50%">{"Half (50%)"}</option>
                                                            <option value="33.33%">{"1/3 (33%)"}</option>
                                                            <option value="25%">{"1/4 (25%)"}</option>
                                                            <option value="300px">{"Fixed (300px)"}</option>
                                                            <option value="500px">{"Fixed (500px)"}</option>
                                                        </select>
                                                    </div>
                                                    <div>
                                                        <label style="font-size: 12px; color: #6c757d; margin-bottom: 4px; display: block;">{"Height"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.position.height.clone()} 
                                                            placeholder="auto"
                                                            style="width: 100%; padding: 6px; border: 1px solid #dee2e6; border-radius: 4px; font-size: 12px; font-family: monospace;"
                                                            oninput={{
                                                                let components = components.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    let mut current_components = (*components).clone();
                                                                    if let Some(comp) = current_components.iter_mut().find(|c| c.id == component_id) {
                                                                        comp.position.height = target.value();
                                                                    }
                                                                    components.set(current_components);
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        // Border & Effects Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"Border & Effects"}</h4>
                                            <div class="property-group">
                                                <label>{"Border Width"}</label>
                                                <input type="text" value={component.styles.border_width.clone()} placeholder="0px" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Border Color"}</label>
                                                <div class="color-input-group">
                                                    <input type="color" value={component.styles.border_color.clone()} />
                                                    <input type="text" value={component.styles.border_color.clone()} placeholder="var(--public-border-light)" />
                                                </div>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Border Style"}</label>
                                                <select value={component.styles.border_style.clone()}>
                                                    <option value="solid">{"Solid"}</option>
                                                    <option value="dashed">{"Dashed"}</option>
                                                    <option value="dotted">{"Dotted"}</option>
                                                    <option value="double">{"Double"}</option>
                                                    <option value="none">{"None"}</option>
                                                </select>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Box Shadow"}</label>
                                                <input type="text" value={component.styles.box_shadow.clone()} placeholder="0 2px 4px rgba(0,0,0,0.1)" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Opacity"}</label>
                                                <input type="range" min="0" max="1" step="0.1" value={component.styles.opacity.to_string()} />
                                                <span class="opacity-value">{format!("{}%", (component.styles.opacity * 100.0) as i32)}</span>
                                            </div>
                                        </div>

                                        // Layout Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"Layout"}</h4>
                                            <div class="property-group">
                                                <label>{"Width"}</label>
                                                <select value={component.position.width.clone()}>
                                                    <option value="auto">{"Auto"}</option>
                                                    <option value="100%">{"Full Width"}</option>
                                                    <option value="75%">{"Three Quarters"}</option>
                                                    <option value="50%">{"Half Width"}</option>
                                                    <option value="25%">{"Quarter Width"}</option>
                                                    <option value="300px">{"Fixed 300px"}</option>
                                                </select>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Text Alignment"}</label>
                                                <div class="alignment-buttons">
                                                    <button class={classes!("align-btn", if component.styles.text_align == "left" { Some("active") } else { None })} title="Left">{"⬅"}</button>
                                                    <button class={classes!("align-btn", if component.styles.text_align == "center" { Some("active") } else { None })} title="Center">{"⬆"}</button>
                                                    <button class={classes!("align-btn", if component.styles.text_align == "right" { Some("active") } else { None })} title="Right">{"➡"}</button>
                                                    <button class={classes!("align-btn", if component.styles.text_align == "justify" { Some("active") } else { None })} title="Justify">{"⬌"}</button>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        // Component-Specific Properties
                                        {match component.component_type {
                                            ComponentType::List => html! {
                                                // List properties are handled at the top of the modal
                                                <></>
                                            },
                                            ComponentType::Image => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Image Properties"}</h4>
                                                    
                                                    <div class="property-group">
                                                        <label>{"Image Source"}</label>
                                                        <div style="display: flex; gap: 8px; align-items: center;">
                                                            <button 
                                                                class="btn btn-primary"
                                                                style="white-space: nowrap;"
                                                                onclick={{
                                                                    let open_media_picker = open_media_picker.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |_: MouseEvent| {
                                                                        open_media_picker.emit((component_id.clone(), true));
                                                                    })
                                                                }}
                                                            >
                                                                {"📁 Browse Media"}
                                                            </button>
                                                            <span style="font-size: 12px; color: var(--public-text-secondary, #666);">
                                                                {"or enter URL manually:"}
                                                            </span>
                                                        </div>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.image_url.clone()} 
                                                            placeholder="https://example.com/image.jpg"
                                                            style="margin-top: 8px;"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "image_url".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                        {if !component.properties.image_url.is_empty() {
                                                            html! {
                                                                <div class="image-preview" style="margin-top: 12px; padding: 12px; background: var(--public-background-secondary, #f8f9fa); border-radius: 6px; text-align: center;">
                                                                    <img 
                                                                        src={component.properties.image_url.clone()}
                                                                        alt="Preview"
                                                                        style="max-width: 100%; max-height: 120px; border-radius: 4px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);"
                                                                        onerror={Callback::from(|_| {
                                                                            web_sys::console::log_1(&"Image failed to load".into());
                                                                        })}
                                                                    />
                                                                </div>
                                                            }
                                                        } else {
                                                            html! {}
                                                        }}
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Alt Text"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.image_alt.clone()} 
                                                            placeholder="Describe this image for accessibility"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "image_alt".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Image Title"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.image_title.clone()} 
                                                            placeholder="Image title (tooltip text)"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "image_title".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label class="checkbox-label">
                                                            <input 
                                                                type="checkbox" 
                                                                checked={component.properties.image_lazy_load}
                                                                onchange={{
                                                                    let on_boolean_property_update = on_boolean_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_boolean_property_update.emit((component_id.clone(), "image_lazy_load".to_string(), target.checked()));
                                                                    })
                                                                }}
                                                            />
                                                            {"Enable lazy loading"}
                                                        </label>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Preview"}</label>
                                                        <div class="property-preview">
                                                            {if !component.properties.image_url.is_empty() {
                                                                html! {
                                                                    <img 
                                                                        src={component.properties.image_url.clone()}
                                                                        alt={component.properties.image_alt.clone()}
                                                                        title={component.properties.image_title.clone()}
                                                                        style="max-width: 100%; max-height: 200px; object-fit: contain; border-radius: 4px;"
                                                                    />
                                                                }
                                                            } else {
                                                                html! {
                                                                    <div class="image-placeholder" style="background: var(--public-background-secondary, #f5f5f5); border: 2px dashed var(--public-border-light, #ccc); padding: 40px; text-align: center; border-radius: 4px;">
                                                                        <span style="color: var(--public-text-muted, #999);">{"📷 Enter an image URL above to see preview"}</span>
                                                                    </div>
                                                                }
                                                            }}
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::Button => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Button Properties"}</h4>
                                                    <div class="property-group">
                                                        <label>{"Button Text"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.button_text.clone()} 
                                                            placeholder="Click Here"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "button_text".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Link URL"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.button_url.clone()} 
                                                            placeholder="https://example.com"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "button_url".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Target"}</label>
                                                        <select 
                                                            value={component.properties.button_target.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "button_target".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="_self">{"Same Window"}</option>
                                                            <option value="_blank">{"New Window"}</option>
                                                            <option value="_parent">{"Parent Frame"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Button Size"}</label>
                                                        <select 
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "button_size".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="small" selected={component.properties.button_size == "small"}>{"Small"}</option>
                                                            <option value="medium" selected={component.properties.button_size == "medium"}>{"Medium"}</option>
                                                            <option value="large" selected={component.properties.button_size == "large"}>{"Large"}</option>
                                                            <option value="xl" selected={component.properties.button_size == "xl"}>{"Extra Large"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Button Style"}</label>
                                                        <select 
                                                            value={component.properties.button_variant.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "button_variant".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="primary">{"Primary"}</option>
                                                            <option value="secondary">{"Secondary"}</option>
                                                            <option value="outline">{"Outline"}</option>
                                                            <option value="ghost">{"Ghost"}</option>
                                                            <option value="danger">{"Danger"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Icon (Emoji)"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.button_icon.clone()} 
                                                            placeholder="🚀"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "button_icon".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Preview"}</label>
                                                        <div class="property-preview">
                                                            {render_component_content(component)}
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::Card => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Card Properties"}</h4>
                                                    <div class="property-group">
                                                        <label>{"Card Title"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.card_title.clone()} 
                                                            placeholder="Enter card title"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "card_title".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Card Description"}</label>
                                                        <textarea 
                                                            rows="3"
                                                            value={component.properties.card_description.clone()} 
                                                            placeholder="Enter card description"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlTextAreaElement>();
                                                                    on_property_update.emit((component_id.clone(), "card_description".to_string(), target.value()));
                                                                })
                                                            }}
                                                        ></textarea>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Card Image URL"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.card_image.clone()} 
                                                            placeholder="https://example.com/image.jpg"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "card_image".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Image Alt Text"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.card_image_alt.clone()} 
                                                            placeholder="Descriptive alt text"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "card_image_alt".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Meta Text"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.card_meta_text.clone()} 
                                                            placeholder="Published on March 1, 2024"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "card_meta_text".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Show Button"}</label>
                                                        <input 
                                                            type="checkbox" 
                                                            checked={component.properties.card_button_show}
                                                            onchange={{
                                                                let on_boolean_property_update = on_boolean_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_boolean_property_update.emit((component_id.clone(), "card_button_show".to_string(), target.checked()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    if component.properties.card_button_show {
                                                        <div class="property-group">
                                                            <label>{"Button Text"}</label>
                                                            <input 
                                                                type="text" 
                                                                value={component.properties.card_button_text.clone()} 
                                                                placeholder="Learn More"
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "card_button_text".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                        <div class="property-group">
                                                            <label>{"Button URL"}</label>
                                                            <input 
                                                                type="text" 
                                                                value={component.properties.card_button_url.clone()} 
                                                                placeholder="https://example.com"
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "card_button_url".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                    }
                                                    <div class="property-group">
                                                        <label>{"Background Color"}</label>
                                                        <input 
                                                            type="color" 
                                                            value={component.properties.card_background.clone()} 
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "card_background".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Border Radius"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.card_border_radius.clone()} 
                                                            placeholder="4px"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "card_border_radius".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Shadow"}</label>
                                                        <select 
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "card_shadow".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="none" selected={component.properties.card_shadow == "none"}>{"None"}</option>
                                                            <option value="small" selected={component.properties.card_shadow == "small"}>{"Small"}</option>
                                                            <option value="medium" selected={component.properties.card_shadow == "medium"}>{"Medium"}</option>
                                                            <option value="large" selected={component.properties.card_shadow == "large"}>{"Large"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Padding"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.card_padding.clone()} 
                                                            placeholder="1.5rem"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "card_padding".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Preview"}</label>
                                                        <div class="property-preview">
                                                            {render_component_content(component)}
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::Hero => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Hero Properties"}</h4>
                                                    <div class="property-group">
                                                        <label>{"Hero Title"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.hero_title.clone()} 
                                                            placeholder="Transform Your Ideas Into Reality"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "hero_title".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Hero Subtitle (Optional)"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.hero_subtitle.clone()} 
                                                            placeholder="Optional subtitle"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "hero_subtitle".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Hero Description"}</label>
                                                        <textarea 
                                                            rows="3"
                                                            value={component.properties.hero_description.clone()} 
                                                            placeholder="Compelling description of your product or service"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlTextAreaElement>();
                                                                    on_property_update.emit((component_id.clone(), "hero_description".to_string(), target.value()));
                                                                })
                                                            }}
                                                        ></textarea>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Background Type"}</label>
                                                        <select 
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "hero_background_type".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="gradient" selected={component.properties.hero_background_type == "gradient"}>{"Gradient"}</option>
                                                            <option value="solid" selected={component.properties.hero_background_type == "solid"}>{"Solid Color"}</option>
                                                            <option value="image" selected={component.properties.hero_background_type == "image"}>{"Background Image"}</option>
                                                        </select>
                                                    </div>
                                                    if component.properties.hero_background_type == "gradient" {
                                                        <div class="property-group">
                                                            <label>{"Gradient Start Color"}</label>
                                                            <input 
                                                                type="color" 
                                                                value={component.properties.hero_background_gradient_start.clone()} 
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "hero_background_gradient_start".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                        <div class="property-group">
                                                            <label>{"Gradient End Color"}</label>
                                                            <input 
                                                                type="color" 
                                                                value={component.properties.hero_background_gradient_end.clone()} 
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "hero_background_gradient_end".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                    } else if component.properties.hero_background_type == "solid" {
                                                        <div class="property-group">
                                                            <label>{"Background Color"}</label>
                                                            <input 
                                                                type="color" 
                                                                value={component.properties.hero_background_color.clone()} 
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "hero_background_color".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                    } else if component.properties.hero_background_type == "image" {
                                                        <div class="property-group">
                                                            <label>{"Background Image URL"}</label>
                                                            <input 
                                                                type="text" 
                                                                value={component.properties.hero_background_image.clone()} 
                                                                placeholder="https://example.com/background.jpg"
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "hero_background_image".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                        <div class="property-group">
                                                            <label>{"Overlay Color"}</label>
                                                            <input 
                                                                type="color" 
                                                                value={component.properties.hero_background_color.clone()} 
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "hero_background_color".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                    }
                                                    <div class="property-group">
                                                        <label>{"Text Color"}</label>
                                                        <input 
                                                            type="color" 
                                                            value={component.properties.hero_text_color.clone()} 
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "hero_text_color".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Text Alignment"}</label>
                                                        <select 
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "hero_alignment".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="left" selected={component.properties.hero_alignment == "left"}>{"Left"}</option>
                                                            <option value="center" selected={component.properties.hero_alignment == "center"}>{"Center"}</option>
                                                            <option value="right" selected={component.properties.hero_alignment == "right"}>{"Right"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Show Badge"}</label>
                                                        <input 
                                                            type="checkbox" 
                                                            checked={component.properties.hero_show_badge}
                                                            onchange={{
                                                                let on_boolean_property_update = on_boolean_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_boolean_property_update.emit((component_id.clone(), "hero_show_badge".to_string(), target.checked()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    if component.properties.hero_show_badge {
                                                        <div class="property-group">
                                                            <label>{"Badge Text"}</label>
                                                            <input 
                                                                type="text" 
                                                                value={component.properties.hero_badge_text.clone()} 
                                                                placeholder="🚀 Welcome to the Future"
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "hero_badge_text".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                    }
                                                    <div class="property-group">
                                                        <label>{"Show Primary Button"}</label>
                                                        <input 
                                                            type="checkbox" 
                                                            checked={component.properties.hero_show_primary_button}
                                                            onchange={{
                                                                let on_boolean_property_update = on_boolean_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_boolean_property_update.emit((component_id.clone(), "hero_show_primary_button".to_string(), target.checked()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    if component.properties.hero_show_primary_button {
                                                        <div class="property-group">
                                                            <label>{"Primary Button Text"}</label>
                                                            <input 
                                                                type="text" 
                                                                value={component.properties.hero_primary_button_text.clone()} 
                                                                placeholder="Get Started"
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "hero_primary_button_text".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                        <div class="property-group">
                                                            <label>{"Primary Button URL"}</label>
                                                            <input 
                                                                type="text" 
                                                                value={component.properties.hero_primary_button_url.clone()} 
                                                                placeholder="#get-started"
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "hero_primary_button_url".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                    }
                                                    <div class="property-group">
                                                        <label>{"Show Secondary Button"}</label>
                                                        <input 
                                                            type="checkbox" 
                                                            checked={component.properties.hero_show_secondary_button}
                                                            onchange={{
                                                                let on_boolean_property_update = on_boolean_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_boolean_property_update.emit((component_id.clone(), "hero_show_secondary_button".to_string(), target.checked()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    if component.properties.hero_show_secondary_button {
                                                        <div class="property-group">
                                                            <label>{"Secondary Button Text"}</label>
                                                            <input 
                                                                type="text" 
                                                                value={component.properties.hero_secondary_button_text.clone()} 
                                                                placeholder="📖 Learn More"
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "hero_secondary_button_text".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                        <div class="property-group">
                                                            <label>{"Secondary Button URL"}</label>
                                                            <input 
                                                                type="text" 
                                                                value={component.properties.hero_secondary_button_url.clone()} 
                                                                placeholder="#learn-more"
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "hero_secondary_button_url".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                    }
                                                    <div class="property-group">
                                                        <label>{"Show Statistics"}</label>
                                                        <input 
                                                            type="checkbox" 
                                                            checked={component.properties.hero_show_stats}
                                                            onchange={{
                                                                let on_boolean_property_update = on_boolean_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_boolean_property_update.emit((component_id.clone(), "hero_show_stats".to_string(), target.checked()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    if component.properties.hero_show_stats {
                                                        <div class="property-group">
                                                            <label>{"Stat 1 - Number"}</label>
                                                            <input 
                                                                type="text" 
                                                                value={component.properties.hero_stat1_number.clone()} 
                                                                placeholder="1000+"
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "hero_stat1_number".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                        <div class="property-group">
                                                            <label>{"Stat 1 - Label"}</label>
                                                            <input 
                                                                type="text" 
                                                                value={component.properties.hero_stat1_label.clone()} 
                                                                placeholder="Happy Users"
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "hero_stat1_label".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                        <div class="property-group">
                                                            <label>{"Stat 2 - Number"}</label>
                                                            <input 
                                                                type="text" 
                                                                value={component.properties.hero_stat2_number.clone()} 
                                                                placeholder="99.9%"
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "hero_stat2_number".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                        <div class="property-group">
                                                            <label>{"Stat 2 - Label"}</label>
                                                            <input 
                                                                type="text" 
                                                                value={component.properties.hero_stat2_label.clone()} 
                                                                placeholder="Uptime"
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "hero_stat2_label".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                        <div class="property-group">
                                                            <label>{"Stat 3 - Number"}</label>
                                                            <input 
                                                                type="text" 
                                                                value={component.properties.hero_stat3_number.clone()} 
                                                                placeholder="24/7"
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "hero_stat3_number".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                        <div class="property-group">
                                                            <label>{"Stat 3 - Label"}</label>
                                                            <input 
                                                                type="text" 
                                                                value={component.properties.hero_stat3_label.clone()} 
                                                                placeholder="Support"
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "hero_stat3_label".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                    }
                                                    <div class="property-group">
                                                        <label>{"Preview"}</label>
                                                        <div class="property-preview">
                                                            {render_component_content(component)}
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::Link => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Link Properties"}</h4>
                                                    <div class="property-group" style="padding: 16px; background: var(--public-background-secondary, #f8f9fa); border-radius: 8px; margin-bottom: 16px;">
                                                        <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 8px;">
                                                            <span style="font-size: 20px;">{"ℹ️"}</span>
                                                            <h5 style="margin: 0; font-size: 14px; font-weight: 600; color: var(--public-text-primary, #333);">{"How Link Components Work"}</h5>
                                                        </div>
                                                        <p style="margin: 0; font-size: 12px; color: var(--public-text-secondary, #666); line-height: 1.4;">
                                                            {"Link components render as editable text with link functionality. Edit the link text and URL directly in the content editor above using markdown syntax: "}
                                                            <code style="background: var(--public-background-primary, #fff); padding: 2px 4px; border-radius: 2px; font-family: monospace;">{"[Link Text](URL)"}</code>
                                                        </p>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Quick URL Update"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.button_url.clone()} 
                                                            placeholder="https://example.com or #anchor"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                let on_content_save = on_content_save.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    let new_url = target.value();
                                                                    on_property_update.emit((component_id.clone(), "button_url".to_string(), new_url.clone()));
                                                                    
                                                                    // Also update the content to reflect the new URL
                                                                    // Extract current link text from content if possible
                                                                    let new_content = if new_url.starts_with('#') || new_url.starts_with("http") {
                                                                        format!("[Link Text]({})", new_url)
                                                                    } else if !new_url.is_empty() {
                                                                        format!("[Link Text](https://{})", new_url)
                                                                    } else {
                                                                        "[Link Text](#)".to_string()
                                                                    };
                                                                    on_content_save.emit((component_id.clone(), new_content));
                                                                })
                                                            }}
                                                        />
                                                        <div style="font-size: 11px; color: var(--public-text-secondary, #666); margin-top: 4px;">
                                                            {"Updates both the URL and content editor automatically"}
                                                        </div>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Link Target"}</label>
                                                        <select 
                                                            value={component.properties.button_target.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "button_target".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="_self">{"Same Window"}</option>
                                                            <option value="_blank">{"New Window"}</option>
                                                            <option value="_parent">{"Parent Frame"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Preview"}</label>
                                                        <div class="property-preview">
                                                            {render_component_content(component)}
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::Video => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Video Properties"}</h4>
                                                    
                                                    <div class="property-group">
                                                        <label>{"Video Source"}</label>
                                                        <div style="display: flex; gap: 8px; align-items: center; margin-bottom: 8px;">
                                                            <button 
                                                                class="btn btn-primary"
                                                                style="white-space: nowrap;"
                                                                onclick={{
                                                                    let open_media_picker = open_media_picker.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |_: MouseEvent| {
                                                                        open_media_picker.emit((component_id.clone(), false));
                                                                    })
                                                                }}
                                                            >
                                                                {"🎥 Browse Videos"}
                                                            </button>
                                                            <span style="font-size: 12px; color: var(--public-text-secondary, #666);">
                                                                {"or enter URL manually:"}
                                                            </span>
                                                        </div>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.video_url.clone()} 
                                                            placeholder="YouTube, Vimeo, or direct video URL"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "video_url".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                        <small style="color: var(--public-text-secondary, #666); font-size: 12px; margin-top: 4px; display: block;">
                                                            {"Supports: Uploaded videos, YouTube, Vimeo, .mp4, .webm, .ogg files"}
                                                        </small>
                                                    </div>
                                                    <div class="property-group">
                                                        <label class="checkbox-label">
                                                            <input 
                                                                type="checkbox" 
                                                                checked={component.properties.video_autoplay}
                                                                onchange={{
                                                                    let on_boolean_property_update = on_boolean_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_boolean_property_update.emit((component_id.clone(), "video_autoplay".to_string(), target.checked()));
                                                                    })
                                                                }}
                                                            />
                                                            {"Autoplay"}
                                                        </label>
                                                    </div>
                                                    <div class="property-group">
                                                        <label class="checkbox-label">
                                                            <input 
                                                                type="checkbox" 
                                                                checked={component.properties.video_controls}
                                                                onchange={{
                                                                    let on_boolean_property_update = on_boolean_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_boolean_property_update.emit((component_id.clone(), "video_controls".to_string(), target.checked()));
                                                                    })
                                                                }}
                                                            />
                                                            {"Show controls"}
                                                        </label>
                                                    </div>
                                                    <div class="property-group">
                                                        <label class="checkbox-label">
                                                            <input 
                                                                type="checkbox" 
                                                                checked={component.properties.video_muted}
                                                                onchange={{
                                                                    let on_boolean_property_update = on_boolean_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_boolean_property_update.emit((component_id.clone(), "video_muted".to_string(), target.checked()));
                                                                    })
                                                                }}
                                                            />
                                                            {"Muted"}
                                                        </label>
                                                    </div>
                                                    <div class="property-group">
                                                        <label class="checkbox-label">
                                                            <input 
                                                                type="checkbox" 
                                                                checked={component.properties.video_loop}
                                                                onchange={{
                                                                    let on_boolean_property_update = on_boolean_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_boolean_property_update.emit((component_id.clone(), "video_loop".to_string(), target.checked()));
                                                                    })
                                                                }}
                                                            />
                                                            {"Loop"}
                                                        </label>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Preview"}</label>
                                                        <div class="property-preview">
                                                            {render_component_content(component)}
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::Gallery => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Gallery"}</h4>
                                                    
                                                    // Add Images Section
                                                    <div class="property-group">
                                                        <button 
                                                            class="media-picker-btn"
                                                            onclick={{
                                                                let open_media_picker = open_media_picker.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |_| {
                                                                    web_sys::console::log_1(&format!("🎯 Opening media picker for gallery component: {}", component_id).into());
                                                                    open_media_picker.emit((component_id.clone(), true));
                                                                })
                                                            }}
                                                            style="
                                                                width: 100%;
                                                                padding: 16px;
                                                                background: var(--admin-primary, #007bff);
                                                                color: white;
                                                                border: none;
                                                                border-radius: 8px;
                                                                cursor: pointer;
                                                                font-size: 16px;
                                                                font-weight: 600;
                                                                margin-bottom: 20px;
                                                                transition: all 0.2s ease;
                                                                display: flex;
                                                                align-items: center;
                                                                justify-content: center;
                                                                gap: 8px;
                                                            "
                                                        >
                                                            <span style="font-size: 18px;">{"📷"}</span>
                                                            {"Add Images"}
                                                        </button>
                                                    </div>
                                                    
                                                    // Gallery Management
                                                    {if !component.properties.gallery_images.is_empty() {
                                                        html! {
                                                            <div class="property-group">
                                                                <label style="font-weight: 600; margin-bottom: 12px; display: block;">
                                                                    {format!("Gallery Images ({})", component.properties.gallery_images.len())}
                                                                </label>
                                                                <div class="enhanced-gallery-wrapper" style="
                                                                    border: 1px solid #e1e5e9; 
                                                                    border-radius: 8px; 
                                                                    overflow: hidden;
                                                                    max-height: 400px;
                                                                ">
                                                                    <EnhancedGallery 
                                                                        images={component.properties.gallery_images.iter().map(|img| EnhancedGalleryImage {
                                                                            id: uuid::Uuid::new_v4().to_string(),
                                                                            url: img.url.clone(),
                                                                            alt: img.alt.clone(),
                                                                            caption: img.caption.clone(),
                                                                            title: img.title.clone(),
                                                                            media_id: None,
                                                                        }).collect::<Vec<_>>()}
                                                                        on_images_change={{
                                                                            let on_property_update = on_property_update.clone();
                                                                            let component_id = component.id.clone();
                                                                            Callback::from(move |images: Vec<EnhancedGalleryImage>| {
                                                                                let gallery_images = images.iter().map(|img| GalleryImage {
                                                                                    url: img.url.clone(),
                                                                                    alt: img.alt.clone(),
                                                                                    caption: img.caption.clone(),
                                                                                    title: img.title.clone(),
                                                                                }).collect::<Vec<_>>();
                                                                                
                                                                                // Serialize the gallery images to JSON string for property update
                                                                                if let Ok(json_str) = serde_json::to_string(&gallery_images) {
                                                                                    on_property_update.emit((component_id.clone(), "gallery_images".to_string(), json_str));
                                                                                }
                                                                            })
                                                                        }}
                                                                        layout={component.properties.gallery_layout.clone()}
                                                                        columns={component.properties.gallery_columns}
                                                                        gap={component.properties.gallery_gap}
                                                                        border_radius={component.properties.gallery_border_radius}
                                                                        show_captions={component.properties.gallery_show_captions}
                                                                        enable_lightbox={component.properties.gallery_enable_lightbox}
                                                                        enable_drag_reorder={component.properties.gallery_enable_drag_reorder}
                                                                        is_admin_mode={true}
                                                                    />
                                                                </div>
                                                            </div>
                                                        }
                                                    } else {
                                                        html! {
                                                            <div class="property-group">
                                                                <div style="
                                                                    text-align: center;
                                                                    padding: 40px 20px;
                                                                    border: 2px dashed #ddd;
                                                                    border-radius: 8px;
                                                                    color: #666;
                                                                    background: #f8f9fa;
                                                                ">
                                                                    <div style="font-size: 48px; margin-bottom: 12px;">{"🖼️"}</div>
                                                                    <p style="margin: 0; font-size: 16px; font-weight: 500;">{"No images added yet"}</p>
                                                                    <p style="margin: 8px 0 0 0; font-size: 14px;">{"Click 'Add Images' to get started"}</p>
                                                                </div>
                                                            </div>
                                                        }
                                                    }}
                                                </div>
                                            },
                                            ComponentType::Container => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Container Properties"}</h4>
                                                    <div class="property-group">
                                                        <label>{"Max Width"}</label>
                                                        <select value={component.properties.container_max_width.clone()}>
                                                            <option value="none">{"No Limit"}</option>
                                                            <option value="800px">{"Small (800px)"}</option>
                                                            <option value="1200px">{"Medium (1200px)"}</option>
                                                            <option value="1600px">{"Large (1600px)"}</option>
                                                            <option value="100%">{"Full Width"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Alignment"}</label>
                                                        <select value={component.properties.container_align.clone()}>
                                                            <option value="left">{"Left"}</option>
                                                            <option value="center">{"Center"}</option>
                                                            <option value="right">{"Right"}</option>
                                                        </select>
                                                    </div>
                                                    
                                                    <div class="property-group">
                                                        <label>{"Visual Effects"}</label>
                                                        <select value={component.properties.effects.clone()}>
                                                            <option value="none">{"None"}</option>
                                                            <option value="glassmorphism">{"Glassmorphism"}</option>
                                                            <option value="neumorphism">{"Neumorphism"}</option>
                                                            <option value="claymorphism">{"Claymorphism"}</option>
                                                            <option value="cybermorphism">{"Cybermorphism"}</option>
                                                        </select>
                                                    </div>
                                                    
                                                    {if component.properties.effects != "none" {
                                                        html! {
                                                            <div class="property-group">
                                                                <label>{"Effect Opacity (%)"}</label>
                                                                <input 
                                                                    type="range" 
                                                                    min="10" 
                                                                    max="100" 
                                                                    value={component.properties.effects_intensity.clone()}
                                                                    class="range-input"
                                                                />
                                                                <span class="range-value">{format!("{}%", component.properties.effects_intensity)}</span>
                                                            </div>
                                                        }
                                                    } else { html! {} }}
                                                    
                                                    // Nested Components Management
                                                    <div class="property-group">
                                                        <label style="font-weight: 600; margin-bottom: 8px; display: block;">{"Nested Components"}</label>
                                                        <div style="border: 1px solid #e1e5e9; border-radius: 6px; padding: 12px; background: #f8f9fa;">
                                                            {if component.properties.nested_components.is_empty() {
                                                                html! {
                                                                    <div style="text-align: center; color: #666; font-style: italic; padding: 20px;">
                                                                        {"No components in this container yet."}
                                                                        <br/>
                                                                        {"Drag components from the sidebar to add them."}
                                                                    </div>
                                                                }
                                                            } else {
                                                                html! {
                                                                    <div>
                                                                        <div style="font-size: 12px; color: #666; margin-bottom: 8px;">
                                                                            {format!("{} component(s) in container:", component.properties.nested_components.len())}
                                                                        </div>
                                                                        {component.properties.nested_components.iter().enumerate().map(|(index, nested_comp)| {
                                                                            html! {
                                                                                <div style="display: flex; justify-content: space-between; align-items: center; padding: 8px; border: 1px solid #ddd; border-radius: 4px; margin-bottom: 4px; background: white;">
                                                                                    <div style="flex: 1;">
                                                                                        <div style="font-size: 14px; font-weight: 500; margin-bottom: 2px;">
                                                                                            {format!("{}. {}", index + 1, nested_comp.component_type.display_name())}
                                                                                        </div>
                                                                                        <div style="font-size: 11px; color: #666;">
                                                                                            {"ID: "}{&nested_comp.id[..8]}{"..."}
                                                                                        </div>
                                                                                    </div>
                                                                                    <div style="display: flex; gap: 4px;">
                                                                                        <button 
                                                                                            type="button"
                                                                                            style="padding: 4px 8px; background: #6c757d; color: white; border: none; border-radius: 3px; cursor: pointer; font-size: 10px;"
                                                                                            title="Edit nested component properties"
                                                                                        >
                                                                                            {"✏️ Edit"}
                                                                                        </button>
                                                                                        <button 
                                                                                            type="button"
                                                                                            style="padding: 4px 8px; background: #17a2b8; color: white; border: none; border-radius: 3px; cursor: pointer; font-size: 10px;"
                                                                                            title="Duplicate nested component"
                                                                                        >
                                                                                            {"📋 Copy"}
                                                                                        </button>
                                                                                        <button 
                                                                                            type="button"
                                                                                            style="padding: 4px 8px; background: #dc3545; color: white; border: none; border-radius: 3px; cursor: pointer; font-size: 10px;"
                                                                                            title="Remove from container"
                                                                                        >
                                                                                            {"🗑️ Remove"}
                                                                                        </button>
                                                                                    </div>
                                                                                </div>
                                                                            }
                                                                        }).collect::<Html>()}
                                                                    </div>
                                                                }
                                                            }}
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::TwoColumn => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Two Column Layout Properties"}</h4>
                                                    
                                                    // Column 1 Management
                                                    <div class="property-group">
                                                        <label style="font-weight: 600; margin-bottom: 8px; display: block;">{"Column 1 Components"}</label>
                                                        <div style="border: 1px solid #e1e5e9; border-radius: 6px; padding: 12px; background: #f8f9fa;">
                                                            {if component.properties.column_1_components.is_empty() {
                                                                html! {
                                                                    <div style="text-align: center; color: #666; font-style: italic; padding: 12px;">
                                                                        {"No components in column 1."}
                                                                        <br/>
                                                                        {"Drag components to the left column."}
                                                                    </div>
                                                                }
                                                            } else {
                                                                html! {
                                                                    <div>
                                                                        <div style="font-size: 12px; color: #666; margin-bottom: 8px;">
                                                                            {format!("{} component(s) in column 1:", component.properties.column_1_components.len())}
                                                                        </div>
                                                                        {component.properties.column_1_components.iter().enumerate().map(|(index, nested_comp)| {
                                                                            html! {
                                                                                <div style="display: flex; justify-content: space-between; align-items: center; padding: 6px; border: 1px solid #ddd; border-radius: 3px; margin-bottom: 3px; background: white; font-size: 12px;">
                                                                                    <span>{format!("C1-{}: {}", index + 1, nested_comp.component_type.display_name())}</span>
                                                                                    <button 
                                                                                        type="button"
                                                                                        style="padding: 2px 6px; background: #dc3545; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 9px;"
                                                                                        title="Remove from column 1"
                                                                                    >
                                                                                        {"🗑️"}
                                                                                    </button>
                                                                                </div>
                                                                            }
                                                                        }).collect::<Html>()}
                                                                    </div>
                                                                }
                                                            }}
                                                        </div>
                                                    </div>
                                                    
                                                    // Column 2 Management
                                                    <div class="property-group">
                                                        <label style="font-weight: 600; margin-bottom: 8px; display: block;">{"Column 2 Components"}</label>
                                                        <div style="border: 1px solid #e1e5e9; border-radius: 6px; padding: 12px; background: #f8f9fa;">
                                                            {if component.properties.column_2_components.is_empty() {
                                                                html! {
                                                                    <div style="text-align: center; color: #666; font-style: italic; padding: 12px;">
                                                                        {"No components in column 2."}
                                                                        <br/>
                                                                        {"Drag components to the right column."}
                                                                    </div>
                                                                }
                                                            } else {
                                                                html! {
                                                                    <div>
                                                                        <div style="font-size: 12px; color: #666; margin-bottom: 8px;">
                                                                            {format!("{} component(s) in column 2:", component.properties.column_2_components.len())}
                                                                        </div>
                                                                        {component.properties.column_2_components.iter().enumerate().map(|(index, nested_comp)| {
                                                                            html! {
                                                                                <div style="display: flex; justify-content: space-between; align-items: center; padding: 6px; border: 1px solid #ddd; border-radius: 3px; margin-bottom: 3px; background: white; font-size: 12px;">
                                                                                    <span>{format!("C2-{}: {}", index + 1, nested_comp.component_type.display_name())}</span>
                                                                                    <button 
                                                                                        type="button"
                                                                                        style="padding: 2px 6px; background: #dc3545; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 9px;"
                                                                                        title="Remove from column 2"
                                                                                    >
                                                                                        {"🗑️"}
                                                                                    </button>
                                                                                </div>
                                                                            }
                                                                        }).collect::<Html>()}
                                                                    </div>
                                                                }
                                                            }}
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::ThreeColumn => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Three Column Layout Properties"}</h4>
                                                    
                                                    <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px;">
                                                        // Column 1
                                                        <div class="property-group">
                                                            <label style="font-weight: 600; margin-bottom: 6px; display: block; font-size: 12px;">{"Column 1"}</label>
                                                            <div style="border: 1px solid #e1e5e9; border-radius: 4px; padding: 8px; background: #f8f9fa; min-height: 60px;">
                                                                {if component.properties.column_1_components.is_empty() {
                                                                    html! {
                                                                        <div style="text-align: center; color: #666; font-style: italic; font-size: 10px; padding: 8px;">
                                                                            {"Empty"}
                                                                        </div>
                                                                    }
                                                                } else {
                                                                    html! {
                                                                        <div>
                                                                            {component.properties.column_1_components.iter().enumerate().map(|(_, nested_comp)| {
                                                                                html! {
                                                                                    <div style="display: flex; justify-content: space-between; align-items: center; padding: 4px; border: 1px solid #ddd; border-radius: 2px; margin-bottom: 2px; background: white; font-size: 10px;">
                                                                                        <span>{format!("{}", nested_comp.component_type.display_name())}</span>
                                                                                        <button 
                                                                                            type="button"
                                                                                            style="padding: 1px 3px; background: #dc3545; color: white; border: none; border-radius: 1px; cursor: pointer; font-size: 8px;"
                                                                                            title="Remove from column 1"
                                                                                        >
                                                                                            {"×"}
                                                                                        </button>
                                                                                    </div>
                                                                                }
                                                                            }).collect::<Html>()}
                                                                        </div>
                                                                    }
                                                                }}
                                                            </div>
                                                        </div>
                                                        
                                                        // Column 2
                                                        <div class="property-group">
                                                            <label style="font-weight: 600; margin-bottom: 6px; display: block; font-size: 12px;">{"Column 2"}</label>
                                                            <div style="border: 1px solid #e1e5e9; border-radius: 4px; padding: 8px; background: #f8f9fa; min-height: 60px;">
                                                                {if component.properties.column_2_components.is_empty() {
                                                                    html! {
                                                                        <div style="text-align: center; color: #666; font-style: italic; font-size: 10px; padding: 8px;">
                                                                            {"Empty"}
                                                                        </div>
                                                                    }
                                                                } else {
                                                                    html! {
                                                                        <div>
                                                                            {component.properties.column_2_components.iter().enumerate().map(|(_, nested_comp)| {
                                                                                html! {
                                                                                    <div style="display: flex; justify-content: space-between; align-items: center; padding: 4px; border: 1px solid #ddd; border-radius: 2px; margin-bottom: 2px; background: white; font-size: 10px;">
                                                                                        <span>{format!("{}", nested_comp.component_type.display_name())}</span>
                                                                                        <button 
                                                                                            type="button"
                                                                                            style="padding: 1px 3px; background: #dc3545; color: white; border: none; border-radius: 1px; cursor: pointer; font-size: 8px;"
                                                                                            title="Remove from column 2"
                                                                                        >
                                                                                            {"×"}
                                                                                        </button>
                                                                                    </div>
                                                                                }
                                                                            }).collect::<Html>()}
                                                                        </div>
                                                                    }
                                                                }}
                                                            </div>
                                                        </div>
                                                        
                                                        // Column 3
                                                        <div class="property-group">
                                                            <label style="font-weight: 600; margin-bottom: 6px; display: block; font-size: 12px;">{"Column 3"}</label>
                                                            <div style="border: 1px solid #e1e5e9; border-radius: 4px; padding: 8px; background: #f8f9fa; min-height: 60px;">
                                                                {if component.properties.column_3_components.is_empty() {
                                                                    html! {
                                                                        <div style="text-align: center; color: #666; font-style: italic; font-size: 10px; padding: 8px;">
                                                                            {"Empty"}
                                                                        </div>
                                                                    }
                                                                } else {
                                                                    html! {
                                                                        <div>
                                                                            {component.properties.column_3_components.iter().enumerate().map(|(_, nested_comp)| {
                                                                                html! {
                                                                                    <div style="display: flex; justify-content: space-between; align-items: center; padding: 4px; border: 1px solid #ddd; border-radius: 2px; margin-bottom: 2px; background: white; font-size: 10px;">
                                                                                        <span>{format!("{}", nested_comp.component_type.display_name())}</span>
                                                                                        <button 
                                                                                            type="button"
                                                                                            style="padding: 1px 3px; background: #dc3545; color: white; border: none; border-radius: 1px; cursor: pointer; font-size: 8px;"
                                                                                            title="Remove from column 3"
                                                                                        >
                                                                                            {"×"}
                                                                                        </button>
                                                                                    </div>
                                                                                }
                                                                            }).collect::<Html>()}
                                                                        </div>
                                                                    }
                                                                }}
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::Divider => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Divider Properties"}</h4>
                                                    <div class="property-group">
                                                        <label>{"Divider Style"}</label>
                                                        <select 
                                                            value={component.properties.divider_style.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "divider_style".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="solid">{"Solid"}</option>
                                                            <option value="dashed">{"Dashed"}</option>
                                                            <option value="dotted">{"Dotted"}</option>
                                                            <option value="double">{"Double"}</option>
                                                            <option value="groove">{"Groove"}</option>
                                                            <option value="ridge">{"Ridge"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Thickness"}</label>
                                                        <select 
                                                            value={component.properties.divider_thickness.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "divider_thickness".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="1px">{"Thin (1px)"}</option>
                                                            <option value="2px">{"Medium (2px)"}</option>
                                                            <option value="3px">{"Thick (3px)"}</option>
                                                            <option value="4px">{"Extra Thick (4px)"}</option>
                                                            <option value="5px">{"Very Thick (5px)"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Color"}</label>
                                                        <div class="color-input-group">
                                                            <input 
                                                                type="color" 
                                                                value={if component.properties.divider_color.starts_with("#") { 
                                                                    component.properties.divider_color.clone() 
                                                                } else { 
                                                                    "#dddddd".to_string() 
                                                                }}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "divider_color".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                            <input 
                                                                type="text" 
                                                                value={component.properties.divider_color.clone()} 
                                                                placeholder="var(--public-border-light, #ddd)"
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "divider_color".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Width"}</label>
                                                        <select 
                                                            value={component.properties.divider_width.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "divider_width".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="25%">{"Quarter (25%)"}</option>
                                                            <option value="50%">{"Half (50%)"}</option>
                                                            <option value="75%">{"Three Quarters (75%)"}</option>
                                                            <option value="100%">{"Full Width (100%)"}</option>
                                                            <option value="200px">{"Fixed Small (200px)"}</option>
                                                            <option value="400px">{"Fixed Medium (400px)"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Spacing (Top/Bottom)"}</label>
                                                        <select 
                                                            value={component.properties.divider_margin.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "divider_margin".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="10px">{"Small (10px)"}</option>
                                                            <option value="20px">{"Medium (20px)"}</option>
                                                            <option value="30px">{"Large (30px)"}</option>
                                                            <option value="40px">{"Extra Large (40px)"}</option>
                                                            <option value="60px">{"Huge (60px)"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Preview"}</label>
                                                        <div class="property-preview">
                                                            {render_component_content(component)}
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::PostsList => html! {
                                                <>
                                                    <div class="property-section">
                                                        <h4 class="section-title">{"Posts List Properties"}</h4>
                                                        
                                                        // Layout & Display Options
                                                        <div class="property-group">
                                                            <label>{"Grid Columns"}</label>
                                                            <select 
                                                                value={component.properties.posts_list_columns.to_string()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "posts_list_columns".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="1">{"1 Column"}</option>
                                                                <option value="2">{"2 Columns"}</option>
                                                                <option value="3">{"3 Columns"}</option>
                                                                <option value="4">{"4 Columns"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Posts to Show"}</label>
                                                            <select 
                                                                value={component.properties.posts_list_count.to_string()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "posts_list_count".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="3">{"3 Posts"}</option>
                                                                <option value="6">{"6 Posts"}</option>
                                                                <option value="9">{"9 Posts"}</option>
                                                                <option value="12">{"12 Posts"}</option>
                                                                <option value="18">{"18 Posts"}</option>
                                                                <option value="24">{"24 Posts"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Excerpt Length"}</label>
                                                            <select 
                                                                value={component.properties.posts_list_excerpt_length.to_string()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "posts_list_excerpt_length".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="100">{"Short (100 chars)"}</option>
                                                                <option value="200">{"Medium (200 chars)"}</option>
                                                                <option value="300">{"Long (300 chars)"}</option>
                                                                <option value="500">{"Very Long (500 chars)"}</option>
                                                            </select>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="property-section">
                                                        <h4 class="section-title">{"Post Card Styling"}</h4>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Card Background"}</label>
                                                            <div class="color-input-group">
                                                                <input 
                                                                    type="color" 
                                                                    value={if component.properties.posts_list_card_background.starts_with("#") { 
                                                                        component.properties.posts_list_card_background.clone() 
                                                                    } else { 
                                                                        "#ffffff".to_string() 
                                                                    }}
                                                                    onchange={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: Event| {
                                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "posts_list_card_background".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                                <input 
                                                                    type="text" 
                                                                    value={component.properties.posts_list_card_background.clone()} 
                                                                    placeholder="var(--public-background-primary, #ffffff)"
                                                                    oninput={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: InputEvent| {
                                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "posts_list_card_background".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                            </div>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Card Border Radius"}</label>
                                                            <select 
                                                                value={component.properties.posts_list_card_radius.clone()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "posts_list_card_radius".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="0px">{"No Radius"}</option>
                                                                <option value="4px">{"Small (4px)"}</option>
                                                                <option value="8px">{"Medium (8px)"}</option>
                                                                <option value="12px">{"Large (12px)"}</option>
                                                                <option value="16px">{"Extra Large (16px)"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Card Shadow"}</label>
                                                            <select 
                                                                value={component.properties.posts_list_card_shadow.clone()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "posts_list_card_shadow".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="none">{"No Shadow"}</option>
                                                                <option value="small">{"Small Shadow"}</option>
                                                                <option value="medium">{"Design System (Medium)"}</option>
                                                                <option value="large">{"Large Shadow"}</option>
                                                                <option value="design-system">{"Design System Default"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Grid Gap"}</label>
                                                            <select 
                                                                value={component.properties.posts_list_grid_gap.clone()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "posts_list_grid_gap".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="12px">{"Small (12px)"}</option>
                                                                <option value="16px">{"Medium (16px)"}</option>
                                                                <option value="24px">{"Large (24px)"}</option>
                                                                <option value="32px">{"Extra Large (32px)"}</option>
                                                            </select>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="property-section">
                                                        <h4 class="section-title">{"Typography & Colors"}</h4>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Title Color"}</label>
                                                            <div class="color-input-group">
                                                                <input 
                                                                    type="color" 
                                                                    value={if component.properties.posts_list_title_color.starts_with("#") { 
                                                                        component.properties.posts_list_title_color.clone() 
                                                                    } else { 
                                                                        "#333333".to_string() 
                                                                    }}
                                                                    onchange={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: Event| {
                                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "posts_list_title_color".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                                <input 
                                                                    type="text" 
                                                                    value={component.properties.posts_list_title_color.clone()} 
                                                                    placeholder="var(--public-text-primary, #333)"
                                                                    oninput={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: InputEvent| {
                                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "posts_list_title_color".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                            </div>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Meta Text Color"}</label>
                                                            <div class="color-input-group">
                                                                <input 
                                                                    type="color" 
                                                                    value={if component.properties.posts_list_meta_color.starts_with("#") { 
                                                                        component.properties.posts_list_meta_color.clone() 
                                                                    } else { 
                                                                        "#666666".to_string() 
                                                                    }}
                                                                    onchange={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: Event| {
                                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "posts_list_meta_color".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                                <input 
                                                                    type="text" 
                                                                    value={component.properties.posts_list_meta_color.clone()} 
                                                                    placeholder="var(--public-text-secondary, #666)"
                                                                    oninput={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: InputEvent| {
                                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "posts_list_meta_color".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                            </div>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Link Color"}</label>
                                                            <div class="color-input-group">
                                                                <input 
                                                                    type="color" 
                                                                    value={if component.properties.posts_list_link_color.starts_with("#") { 
                                                                        component.properties.posts_list_link_color.clone() 
                                                                    } else { 
                                                                        "#2563eb".to_string() 
                                                                    }}
                                                                    onchange={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: Event| {
                                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "posts_list_link_color".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                                <input 
                                                                    type="text" 
                                                                    value={component.properties.posts_list_link_color.clone()} 
                                                                    placeholder="var(--public-link-primary, #2563eb)"
                                                                    oninput={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: InputEvent| {
                                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "posts_list_link_color".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="property-section">
                                                        <h4 class="section-title">{"Content Options"}</h4>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Show Author"}</label>
                                                            <select 
                                                                value={component.properties.video_autoplay.to_string()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "video_autoplay".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="true">{"Show Author"}</option>
                                                                <option value="false">{"Hide Author"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Show Date"}</label>
                                                            <select 
                                                                value={component.properties.video_controls.to_string()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "video_controls".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="true">{"Show Date"}</option>
                                                                <option value="false">{"Hide Date"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Show Excerpt"}</label>
                                                            <select 
                                                                value={component.properties.video_muted.to_string()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "video_muted".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="true">{"Show Excerpt"}</option>
                                                                <option value="false">{"Hide Excerpt"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Show 'View All' Button"}</label>
                                                            <select 
                                                                value={component.properties.video_loop.to_string()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "video_loop".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="true">{"Show Button"}</option>
                                                                <option value="false">{"Hide Button"}</option>
                                                            </select>
                                                        </div>
                                                    </div>
                                                </>
                                            },
                                            ComponentType::Comments => html! {
                                                <>
                                                    <div class="property-section">
                                                        <h4 class="section-title">{"Comments Properties"}</h4>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Allow Comments"}</label>
                                                            <select 
                                                                value={component.properties.comments_enabled.to_string()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "comments_enabled".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="true">{"Enabled"}</option>
                                                                <option value="false">{"Disabled"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Comments Per Page"}</label>
                                                            <select 
                                                                value={component.properties.comments_per_page.to_string()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "comments_per_page".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="10">{"10 Comments"}</option>
                                                                <option value="20">{"20 Comments"}</option>
                                                                <option value="50">{"50 Comments"}</option>
                                                                <option value="100">{"100 Comments"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Show Avatar Size"}</label>
                                                            <select 
                                                                value={component.properties.comments_avatar_size.to_string()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "comments_avatar_size".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="32">{"Small (32px)"}</option>
                                                                <option value="48">{"Medium (48px)"}</option>
                                                                <option value="64">{"Large (64px)"}</option>
                                                                <option value="80">{"Extra Large (80px)"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Show Login Prompt"}</label>
                                                            <select 
                                                                value={component.properties.comments_show_auth_prompt.to_string()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "comments_show_auth_prompt".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="true">{"Show Auth Prompt"}</option>
                                                                <option value="false">{"Hide Auth Prompt"}</option>
                                                            </select>
                                                        </div>
                                                    </div>
                                                </>
                                            },
                                            ComponentType::PageTitle => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Page Title"}</h4>
                                                    
                                                    <div class="property-group">
                                                        <label>{"HTML Tag"}</label>
                                                        <select 
                                                            value={component.properties.page_title_tag.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "page_title_tag".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="h1">{"H1 (Main Title)"}</option>
                                                            <option value="h2">{"H2 (Section Title)"}</option>
                                                            <option value="h3">{"H3 (Subsection)"}</option>
                                                            <option value="h4">{"H4 (Minor Heading)"}</option>
                                                            <option value="h5">{"H5 (Small Heading)"}</option>
                                                            <option value="h6">{"H6 (Smallest Heading)"}</option>
                                                        </select>
                                                    </div>
                                                    
                                                    <div class="property-group">
                                                        <label>
                                                            <input 
                                                                type="checkbox" 
                                                                checked={component.properties.page_title_show_prefix}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "page_title_show_prefix".to_string(), target.checked().to_string()));
                                                                    })
                                                                }}
                                                            />
                                                            {" Show Prefix"}
                                                        </label>
                                                    </div>
                                                    
                                                    {if component.properties.page_title_show_prefix {
                                                        html! {
                                                            <div class="property-group">
                                                                <label>{"Prefix Text"}</label>
                                                                <input 
                                                                    type="text" 
                                                                    value={component.properties.page_title_prefix.clone()}
                                                                    placeholder="e.g., 'Page:', 'Article:'"
                                                                    oninput={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: InputEvent| {
                                                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "page_title_prefix".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                            </div>
                                                        }
                                                    } else { html! {} }}
                                                    
                                                    <div class="property-group">
                                                        <label>
                                                            <input 
                                                                type="checkbox" 
                                                                checked={component.properties.page_title_show_suffix}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "page_title_show_suffix".to_string(), target.checked().to_string()));
                                                                    })
                                                                }}
                                                            />
                                                            {" Show Suffix"}
                                                        </label>
                                                    </div>
                                                    
                                                    {if component.properties.page_title_show_suffix {
                                                        html! {
                                                            <div class="property-group">
                                                                <label>{"Suffix Text"}</label>
                                                                <input 
                                                                    type="text" 
                                                                    value={component.properties.page_title_suffix.clone()}
                                                                    placeholder="e.g., '- My Site', '| Blog'"
                                                                    oninput={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: InputEvent| {
                                                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "page_title_suffix".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                            </div>
                                                        }
                                                    } else { html! {} }}
                                                </div>
                                            },
                                            ComponentType::PublishedDate => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Published Date"}</h4>
                                                    
                                                    <div class="property-group">
                                                        <label>{"Date Format"}</label>
                                                        <select 
                                                            value={component.properties.published_date_format.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "published_date_format".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="Month DD, YYYY">{"August 25, 2024"}</option>
                                                            <option value="YYYY-MM-DD">{"2024-08-25"}</option>
                                                            <option value="DD/MM/YYYY">{"25/08/2024"}</option>
                                                            <option value="MM/DD/YYYY">{"08/25/2024"}</option>
                                                        </select>
                                                    </div>
                                                    
                                                    <div class="property-group">
                                                        <label>
                                                            <input 
                                                                type="checkbox" 
                                                                checked={component.properties.published_date_show_prefix}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "published_date_show_prefix".to_string(), target.checked().to_string()));
                                                                    })
                                                                }}
                                                            />
                                                            {" Show Prefix"}
                                                        </label>
                                                    </div>
                                                    
                                                    {if component.properties.published_date_show_prefix {
                                                        html! {
                                                            <div class="property-group">
                                                                <label>{"Prefix Text"}</label>
                                                                <input 
                                                                    type="text" 
                                                                    value={component.properties.published_date_prefix.clone()}
                                                                    placeholder="e.g., 'Published on', 'Date:'"
                                                                    oninput={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: InputEvent| {
                                                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "published_date_prefix".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                            </div>
                                                        }
                                                    } else { html! {} }}
                                                    
                                                    <div class="property-group">
                                                        <label>
                                                            <input 
                                                                type="checkbox" 
                                                                checked={component.properties.published_date_show_time}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "published_date_show_time".to_string(), target.checked().to_string()));
                                                                    })
                                                                }}
                                                            />
                                                            {" Show Time"}
                                                        </label>
                                                    </div>
                                                    
                                                    <div class="property-group">
                                                        <label>
                                                            <input 
                                                                type="checkbox" 
                                                                checked={component.properties.published_date_relative}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "published_date_relative".to_string(), target.checked().to_string()));
                                                                    })
                                                                }}
                                                            />
                                                            {" Use Relative Dates (e.g., '2 days ago')"}
                                                        </label>
                                                    </div>
                                                </div>
                                            },
                                            _ => html! {}
                                        }}
                                        
                                        // Animation Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"Animation & Effects"}</h4>
                                            <div class="property-group">
                                                <label>{"Animation Type"}</label>
                                                <select value={component.properties.animation_type.clone()}>
                                                    <option value="none">{"None"}</option>
                                                    <option value="fadeIn">{"Fade In"}</option>
                                                    <option value="slideUp">{"Slide Up"}</option>
                                                    <option value="slideDown">{"Slide Down"}</option>
                                                    <option value="slideLeft">{"Slide Left"}</option>
                                                    <option value="slideRight">{"Slide Right"}</option>
                                                    <option value="zoomIn">{"Zoom In"}</option>
                                                    <option value="bounce">{"Bounce"}</option>
                                                </select>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Animation Duration"}</label>
                                                <select value={component.properties.animation_duration.clone()}>
                                                    <option value="0.1s">{"Very Fast (0.1s)"}</option>
                                                    <option value="0.3s">{"Fast (0.3s)"}</option>
                                                    <option value="0.5s">{"Medium (0.5s)"}</option>
                                                    <option value="1s">{"Slow (1s)"}</option>
                                                    <option value="2s">{"Very Slow (2s)"}</option>
                                                </select>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Animation Delay"}</label>
                                                <select value={component.properties.animation_delay.clone()}>
                                                    <option value="0s">{"No Delay"}</option>
                                                    <option value="0.1s">{"0.1s"}</option>
                                                    <option value="0.3s">{"0.3s"}</option>
                                                    <option value="0.5s">{"0.5s"}</option>
                                                    <option value="1s">{"1s"}</option>
                                                </select>
                                            </div>
                                        </div>
                                        
                                        // SEO & Accessibility Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"SEO & Accessibility"}</h4>
                                            <div class="property-group">
                                                <label>{"SEO Title"}</label>
                                                <input type="text" value={component.properties.seo_title.clone()} placeholder="SEO-friendly title" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"SEO Description"}</label>
                                                <textarea value={component.properties.seo_description.clone()} placeholder="Brief description for search engines" rows="3"></textarea>
                                            </div>
                                            <div class="property-group">
                                                <label>{"ARIA Label"}</label>
                                                <input type="text" value={component.properties.aria_label.clone()} placeholder="Accessibility label" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"ARIA Description"}</label>
                                                <input type="text" value={component.properties.aria_description.clone()} placeholder="Accessibility description" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Tab Index"}</label>
                                                <input type="number" value={component.properties.tab_index.to_string()} min="-1" max="999" />
                                            </div>
                                        </div>

                                        // Actions Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"Actions"}</h4>
                                            <div class="action-buttons">
                                                <button 
                                                    class="action-btn secondary"
                                                    onclick={{
                                                        let on_component_duplicate = on_component_duplicate.clone();
                                                        let component_id = component.id.clone();
                                                        Callback::from(move |_: MouseEvent| {
                                                            on_component_duplicate.emit(component_id.clone());
                                                        })
                                                    }}
                                                >
                                                    {"Duplicate"}
                                                </button>
                                                <button 
                                                    class="action-btn danger"
                                                    onclick={{
                                                        let on_component_delete = on_component_delete.clone();
                                                        let component_id = component.id.clone();
                                                        let selected_component = selected_component.clone();
                                                        Callback::from(move |_: MouseEvent| {
                                                            on_component_delete.emit(component_id.clone());
                                                            selected_component.set(None);
                                                        })
                                                    }}
                                                >
                                                    {"Delete"}
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    // Save Button Section
                                    <div class="modal-footer" style="
                                        padding: 20px;
                                        border-top: 1px solid #e9ecef;
                                        background: #f8f9fa;
                                        display: flex;
                                        justify-content: space-between;
                                        align-items: center;
                                        gap: 12px;
                                    ">
                                        <div style="color: #6c757d; font-size: 14px;">
                                            {"Changes are applied instantly"}
                                        </div>
                                        <div style="display: flex; gap: 8px;">
                                            <button 
                                                class="btn btn-secondary"
                                                onclick={{
                                                    let editing_component = editing_component.clone();
                                                    Callback::from(move |e: MouseEvent| {
                                                        e.prevent_default();
                                                        editing_component.set(None);
                                                    })
                                                }}
                                                style="
                                                    padding: 8px 16px;
                                                    background: #6c757d;
                                                    color: white;
                                                    border: none;
                                                    border-radius: 4px;
                                                    cursor: pointer;
                                                    font-size: 14px;
                                                "
                                            >
                                                {"Close"}
                                            </button>
                                            <button 
                                                class="btn btn-primary"
                                                onclick={{
                                                    let editing_component = editing_component.clone();
                                                    let components = components.clone();
                                                    let on_save = props.on_save.clone();
                                                    Callback::from(move |_: MouseEvent| {
                                                        // Save the current components state
                                                        on_save.emit((*components).clone());
                                                        editing_component.set(None);
                                                    })
                                                }}
                                                style="
                                                    padding: 8px 16px;
                                                    background: #007bff;
                                                    color: white;
                                                    border: none;
                                                    border-radius: 4px;
                                                    cursor: pointer;
                                                    font-size: 14px;
                                                    font-weight: 500;
                                                "
                                            >
                                                {"💾 Save Changes"}
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            }}
            
            // Media Picker Modal
            <MediaPicker 
                show={*show_media_picker}
                filter_images_only={*media_picker_images_only}
                on_select={on_media_select}
                on_multi_select={Some(on_multi_media_select)}
                on_close={close_media_picker}
                allow_multi_select={
                    // Enable multi-select for gallery components
                    if let Some(ref component_id) = *media_picker_target_component {
                        if let Some(component) = components.iter().find(|c| c.id == *component_id) {
                            matches!(component.component_type, ComponentType::Gallery)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            />
        </>
    }
}

fn render_component_content(component: &PageComponent) -> Html {
    match component.component_type {
        ComponentType::Sidebar => {
            let is_left = component.properties.sidebar_position == "left";
            let sticky_style = if component.properties.sidebar_sticky { "position: sticky; top: 16px;" } else { "" };
            let style = if is_left {
                format!("display: grid; grid-template-columns: {w} 1fr; gap: 16px;", w = component.properties.sidebar_width)
            } else {
                format!("display: grid; grid-template-columns: 1fr {w}; gap: 16px;", w = component.properties.sidebar_width)
            };
            html! {
                <div class="component-sidebar" style={style}>
                    if is_left {
                        <aside class="sidebar-rail" style={format!("border: 2px dashed #ddd; border-radius: 8px; padding: 12px; background: #f9f9f9; {}", sticky_style)}>
                            <div style="font-size: 12px; color: #666; margin-bottom: 8px; text-align: center;">{"Sidebar (Left)"}</div>
                            <div class="sidebar-preview"><div class="sidebar-item" style="font-size: 12px; color: #777;">{"Navigation, Recent Posts, etc. (Template)"}</div></div>
                        </aside>
                        <div class="sidebar-body" style="min-height: 120px; border: 1px dashed #ccc; border-radius: 8px; padding: 16px; background: white;">
                            {"Page Content Area"}
                        </div>
                    } else {
                        <div class="sidebar-body" style="min-height: 120px; border: 1px dashed #ccc; border-radius: 8px; padding: 16px; background: white;">
                            {"Page Content Area"}
                        </div>
                        <aside class="sidebar-rail" style={format!("border: 2px dashed #ddd; border-radius: 8px; padding: 12px; background: #f9f9f9; {}", sticky_style)}>
                            <div style="font-size: 12px; color: #666; margin-bottom: 8px; text-align: center;">{"Sidebar (Right)"}</div>
                            <div class="sidebar-preview"><div class="sidebar-item" style="font-size: 12px; color: #777;">{"Navigation, Recent Posts, etc. (Template)"}</div></div>
                        </aside>
                    }
                </div>
            }
        }
        ComponentType::Text | ComponentType::Heading | ComponentType::Subheading => {
            // Render as markdown
            let parser = pulldown_cmark::Parser::new(&component.content);
            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser);
            Html::from_html_unchecked(html_output.into())
        }
        ComponentType::Hero => {
            // Dynamic hero using component properties
            let background_style = match component.properties.hero_background_type.as_str() {
                "gradient" => format!(
                    "background: linear-gradient(135deg, {} 0%, {} 100%);",
                    component.properties.hero_background_gradient_start,
                    component.properties.hero_background_gradient_end
                ),
                "solid" => format!("background: {};", component.properties.hero_background_color),
                "image" => if !component.properties.hero_background_image.is_empty() {
                    format!(
                        "background: linear-gradient(135deg, {}66 0%, {}66 100%), url({}) center/cover;",
                        component.properties.hero_background_color,
                        component.properties.hero_background_color,
                        component.properties.hero_background_image
                    )
                } else {
                    format!("background: {};", component.properties.hero_background_color)
                },
                _ => format!(
                    "background: linear-gradient(135deg, {} 0%, {} 100%);",
                    component.properties.hero_background_gradient_start,
                    component.properties.hero_background_gradient_end
                ),
            };
            
            let hero_style = format!(
                "{} color: {}; padding: {}; text-align: {}; border-radius: 12px; position: relative; overflow: hidden; min-height: {};",
                background_style,
                component.properties.hero_text_color,
                component.properties.hero_padding,
                component.properties.hero_alignment,
                component.properties.hero_min_height
            );
            
            html! {
                <section class="hero-section" style={hero_style}>
                    // Background pattern
                    <div style="position: absolute; top: 0; left: 0; right: 0; bottom: 0; opacity: 0.1; background-image: radial-gradient(circle at 25% 25%, white 2px, transparent 2px), radial-gradient(circle at 75% 75%, white 2px, transparent 2px); background-size: 50px 50px;"></div>
                    
                    <div class="hero-content" style="position: relative; z-index: 1; max-width: 800px; margin: 0 auto;">
                        // Hero badge (conditional)
                        if component.properties.hero_show_badge && !component.properties.hero_badge_text.is_empty() {
                        <div class="hero-badge" style="display: inline-block; background: rgba(255,255,255,0.2); padding: 8px 16px; border-radius: 20px; font-size: 14px; margin-bottom: 24px; border: 1px solid rgba(255,255,255,0.3);">
                                {&component.properties.hero_badge_text}
                        </div>
                        }
                        
                        // Hero title
                        <h1 style="font-size: 48px; font-weight: 700; margin: 0 0 24px 0; line-height: 1.2;">
                            {&component.properties.hero_title}
                        </h1>
                        
                        // Hero subtitle (conditional)
                        if !component.properties.hero_subtitle.is_empty() {
                            <h2 style="font-size: 28px; font-weight: 400; margin: 0 0 24px 0; opacity: 0.9;">
                                {&component.properties.hero_subtitle}
                            </h2>
                        }
                        
                        // Hero description
                        <p style="font-size: 20px; margin: 0 0 32px 0; opacity: 0.9; line-height: 1.6;">
                            {&component.properties.hero_description}
                        </p>
                        
                        // Hero action buttons (conditional)
                        if component.properties.hero_show_primary_button || component.properties.hero_show_secondary_button {
                        <div class="hero-actions" style="display: flex; gap: 16px; justify-content: center; flex-wrap: wrap;">
                                if component.properties.hero_show_primary_button && !component.properties.hero_primary_button_text.is_empty() {
                                    <a 
                                        href={component.properties.hero_primary_button_url.clone()}
                                        style="display: inline-flex; align-items: center; gap: 8px; padding: 16px 32px; background: white; color: var(--public-primary, #3b82f6); text-decoration: none; border-radius: 8px; font-weight: 600; font-size: 16px; transition: all 0.2s; box-shadow: 0 4px 12px rgba(0,0,0,0.1);"
                                    >
                                        {&component.properties.hero_primary_button_text} <span>{"→"}</span>
                                    </a>
                                }
                                if component.properties.hero_show_secondary_button && !component.properties.hero_secondary_button_text.is_empty() {
                                    <a 
                                        href={component.properties.hero_secondary_button_url.clone()}
                                        style="display: inline-flex; align-items: center; gap: 8px; padding: 16px 32px; background: transparent; color: white; text-decoration: none; border: 2px solid rgba(255,255,255,0.3); border-radius: 8px; font-weight: 600; font-size: 16px; transition: all 0.2s;"
                                    >
                                        {&component.properties.hero_secondary_button_text}
                                    </a>
                                }
                        </div>
                        }
                        
                        // Hero stats (conditional)
                        if component.properties.hero_show_stats && (!component.properties.hero_stat1_number.is_empty() || !component.properties.hero_stat2_number.is_empty() || !component.properties.hero_stat3_number.is_empty()) {
                        <div class="hero-stats" style="margin-top: 48px; display: grid; grid-template-columns: repeat(auto-fit, minmax(120px, 1fr)); gap: 24px; opacity: 0.9;">
                                if !component.properties.hero_stat1_number.is_empty() {
                            <div class="stat-item" style="text-align: center;">
                                        <div style="font-size: 24px; font-weight: 700; margin-bottom: 4px;">{&component.properties.hero_stat1_number}</div>
                                        <div style="font-size: 14px; opacity: 0.8;">{&component.properties.hero_stat1_label}</div>
                            </div>
                                }
                                if !component.properties.hero_stat2_number.is_empty() {
                            <div class="stat-item" style="text-align: center;">
                                        <div style="font-size: 24px; font-weight: 700; margin-bottom: 4px;">{&component.properties.hero_stat2_number}</div>
                                        <div style="font-size: 14px; opacity: 0.8;">{&component.properties.hero_stat2_label}</div>
                            </div>
                                }
                                if !component.properties.hero_stat3_number.is_empty() {
                            <div class="stat-item" style="text-align: center;">
                                        <div style="font-size: 24px; font-weight: 700; margin-bottom: 4px;">{&component.properties.hero_stat3_number}</div>
                                        <div style="font-size: 14px; opacity: 0.8;">{&component.properties.hero_stat3_label}</div>
                            </div>
                                }
                        </div>
                        }
                    </div>
                </section>
            }
        }
        ComponentType::Card => {
            // Use component properties for dynamic content
            let card_style = format!(
                "background: {}; border-radius: {}; padding: {}; box-shadow: {}; border: 1px solid #eee; transition: transform 0.2s ease, box-shadow 0.2s ease;",
                component.properties.card_background,
                component.properties.card_border_radius,
                component.properties.card_padding,
                match component.properties.card_shadow.as_str() {
                    "none" => "none",
                    "small" => "0 2px 4px rgba(0, 0, 0, 0.05)",
                    "medium" => "0 4px 12px rgba(0, 0, 0, 0.1)",
                    "large" => "0 8px 32px rgba(0, 0, 0, 0.15)",
                    _ => "0 4px 12px rgba(0, 0, 0, 0.1)",
                }
            );
            
            html! {
                <div class="post-card" style={card_style}>
                    // Card image if provided
                    if !component.properties.card_image.is_empty() {
                        <div class="card-image" style="margin-bottom: 1rem;">
                            <img 
                                src={component.properties.card_image.clone()} 
                                alt={component.properties.card_image_alt.clone()}
                                style="width: 100%; height: 200px; object-fit: cover; border-radius: 4px;"
                            />
                        </div>
                    }
                    
                    // Card title
                    <h4 style="font-size: 1.3rem; margin-bottom: 0.5rem; color: var(--public-heading-h3, #000);">
                        {&component.properties.card_title}
                    </h4>
                    
                    // Card meta text if provided
                    if !component.properties.card_meta_text.is_empty() {
                        <div class="post-meta" style="color: var(--public-text-meta, #666); font-size: 0.9rem; margin-bottom: 1rem;">
                            {&component.properties.card_meta_text}
                        </div>
                    }
                    
                    // Card description
                    <div class="post-excerpt" style="color: var(--public-text-secondary, #555); margin-bottom: 1rem;">
                        {&component.properties.card_description}
                    </div>
                    
                    // Card button if enabled
                    if component.properties.card_button_show && !component.properties.card_button_text.is_empty() {
                        <a 
                            href={component.properties.card_button_url.clone()}
                            class="read-more"
                            style="color: var(--public-link-primary, #000); text-decoration: none; font-weight: 600; border-bottom: 2px solid var(--public-link-primary, #000); transition: border-color 0.2s ease;"
                        >
                            {&component.properties.card_button_text}
                        </a>
                    }
                </div>
            }
        }
        ComponentType::List => {
            let list_style = format!(
                "background: {}; border-radius: {}; padding: {}; border: 1px solid #e1e5e9;",
                component.properties.list_background,
                component.properties.list_border_radius,
                component.properties.list_padding
            );
            
            html! {
                <div class="enhanced-list" style={list_style}>
                    <div class="list-items" style={format!("display: grid; gap: {};", component.properties.list_item_spacing)}>
                        {component.properties.list_items.iter().enumerate().map(|(index, item)| {
                            // Icon colors for visual variety
                            let icon_colors = [
                                "linear-gradient(135deg, #10b981, #059669)", // Green
                                "linear-gradient(135deg, #3b82f6, #1d4ed8)", // Blue  
                                "linear-gradient(135deg, #8b5cf6, #7c3aed)", // Purple
                                "linear-gradient(135deg, #f59e0b, #d97706)", // Orange
                                "linear-gradient(135deg, #ef4444, #dc2626)", // Red
                                "linear-gradient(135deg, #06b6d4, #0891b2)", // Cyan
                            ];
                            let icon_gradient = icon_colors[index % icon_colors.len()];
                            
                            html! {
                                <div class="list-item" style="display: flex; align-items: flex-start; gap: 16px; padding: 16px; background: #f8f9fa; border-radius: 8px; transition: all 0.2s;">
                                    if component.properties.list_show_icons {
                                        <div class="item-icon" style={format!("flex-shrink: 0; width: 40px; height: 40px; background: {}; border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; font-size: 18px;", icon_gradient)}>
                                            {&item.icon}
                            </div>
                                    }
                            <div class="item-content" style="flex: 1;">
                                        <h4 style={format!("margin: 0 0 4px 0; font-size: 16px; font-weight: 600; color: #333333;")}>
                                            {&item.title}
                                </h4>
                                        if !item.description.is_empty() {
                                            <p style={format!("margin: 0; color: {}; font-size: 14px; line-height: 1.5;", component.properties.list_text_color)}>
                                                {&item.description}
                                </p>
                                        }
                            </div>
                        </div>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            }
        }
        ComponentType::Quote => {
            html! {
                <blockquote class="enhanced-quote" style="background: var(--public-background-primary, #ffffff); border-radius: 16px; padding: 40px; margin: 32px 0; position: relative; border: 1px solid var(--public-border-light, #e1e5e9); box-shadow: 0 4px 16px rgba(0,0,0,0.05);">
                    // Quote accent
                    <div style="position: absolute; top: 0; left: 0; width: 6px; height: 100%; background: linear-gradient(180deg, var(--public-primary, #3b82f6), var(--public-secondary, #1d4ed8)); border-radius: 3px 0 0 3px;"></div>
                    
                    // Quote icon
                    <div class="quote-icon" style="position: absolute; top: -12px; left: 32px; width: 48px; height: 48px; background: linear-gradient(135deg, var(--public-primary, #3b82f6), var(--public-secondary, #1d4ed8)); border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-size: 24px; font-weight: bold; box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);">
                        {"\u{201C}"}
                    </div>
                    
                    <div class="quote-content" style="margin-top: 20px;">
                        <p style="font-size: 20px; line-height: 1.6; color: var(--public-text-primary, #333); margin: 0 0 24px 0; font-style: italic; font-weight: 400;">
                            {"This Rust CMS has completely transformed our content workflow. The performance is incredible, and the page builder makes it so easy for our team to create beautiful, professional pages without any technical knowledge. It's exactly what we needed!"}
                        </p>
                        
                        <div class="quote-author" style="display: flex; align-items: center; gap: 16px;">
                            <div class="author-avatar" style="width: 56px; height: 56px; background: linear-gradient(135deg, #f093fb, #f5576c); border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-size: 20px; font-weight: 600;">
                                {"SJ"}
                            </div>
                            <div class="author-info">
                                <div class="author-name" style="font-size: 16px; font-weight: 600; color: var(--public-text-primary, #333); margin-bottom: 2px;">
                                    {"Sarah Johnson"}
                                </div>
                                <div class="author-title" style="font-size: 14px; color: var(--public-text-secondary, #666);">
                                    {"Content Manager, TechCorp"}
                                </div>
                                <div class="author-company" style="font-size: 12px; color: var(--public-text-muted, #999); margin-top: 2px;">
                                    {"⭐⭐⭐⭐⭐ 5/5 stars"}
                                </div>
                            </div>
                        </div>
                    </div>
                </blockquote>
            }
        }
        ComponentType::Image => {
            if component.properties.image_url.is_empty() {
                html! { 
                    <div class="placeholder-image" style="background: var(--public-background-secondary, #f5f5f5); border: 2px dashed var(--public-border-light, #ccc); padding: 40px; text-align: center; border-radius: 8px; color: var(--public-text-secondary, #666);">
                        {"📷 Configure image properties to display"} 
                    </div> 
                }
            } else {
                html! { 
                    <img 
                        src={component.properties.image_url.clone()} 
                        alt={component.properties.image_alt.clone()} 
                        title={component.properties.image_title.clone()}
                        style="max-width: 100%; height: auto; border-radius: 4px;"
                    /> 
                }
            }
        }
        ComponentType::Button => {
            let button_classes = format!("btn btn-{} btn-{}", 
                component.properties.button_variant,
                component.properties.button_size
            );
            
            let button_style = format!(
                "display: inline-flex; align-items: center; gap: 8px; text-decoration: none; border: none; cursor: pointer; transition: all 0.2s ease; pointer-events: none; {}", 
                if component.styles.background_color != "transparent" {
                    format!("background-color: {} !important;", component.styles.background_color)
                } else { String::new() }
            );

            if component.properties.button_url.is_empty() || component.properties.button_url == "#" {
                html! {
                    <button 
                        class={button_classes}
                        style={button_style}
                        disabled={true}
                        title="Configure button URL in properties"
                    >
                        {if !component.properties.button_icon.is_empty() {
                            html! { <span class="button-icon">{&component.properties.button_icon}</span> }
                        } else { html! {} }}
                        <span class="button-text">
                            {if component.properties.button_text.is_empty() {
                                "Button Text"
                            } else {
                                &component.properties.button_text
                            }}
                        </span>
                    </button>
                }
            } else {
                html! {
                    <a 
                        href={component.properties.button_url.clone()}
                        target={component.properties.button_target.clone()}
                        class={button_classes}
                        style={button_style}
                        role="button"
                    >
                        {if !component.properties.button_icon.is_empty() {
                            html! { <span class="button-icon">{&component.properties.button_icon}</span> }
                        } else { html! {} }}
                        <span class="button-text">
                            {if component.properties.button_text.is_empty() {
                                "Button Text"
                            } else {
                                &component.properties.button_text
                            }}
                        </span>
                    </a>
                }
            }
        }
        ComponentType::Link => {
            // Render as markdown content (editable link text)
            let parser = pulldown_cmark::Parser::new(&component.content);
            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser);
            Html::from_html_unchecked(html_output.into())
        }
        ComponentType::Video => {
            if component.properties.video_url.is_empty() {
                html! { 
                    <div class="placeholder-video" style="background: var(--public-background-secondary, #f5f5f5); border: 2px dashed var(--public-border-light, #ccc); padding: 60px 40px; text-align: center; border-radius: 8px; color: var(--public-text-secondary, #666);">
                        <div style="font-size: 48px; margin-bottom: 16px;">{"🎥"}</div>
                        <div style="font-size: 18px; font-weight: 500; margin-bottom: 8px;">{"Video Component"}</div>
                        <div style="font-size: 14px;">{"Configure video URL in properties to display video"}</div>
                    </div> 
                }
            } else {
                let video_url = &component.properties.video_url;
                let embed_url = convert_to_embed_url(video_url);
                let controls = if component.properties.video_controls { "1" } else { "0" };
                let autoplay = if component.properties.video_autoplay { "1" } else { "0" };
                let muted = if component.properties.video_muted { "1" } else { "0" };
                let loop_param = if component.properties.video_loop { "1" } else { "0" };
                
                if embed_url.contains("youtube.com") || embed_url.contains("youtu.be") {
                    let youtube_params = format!("?controls={}&autoplay={}&mute={}&loop={}", controls, autoplay, muted, loop_param);
                    html! {
                        <div class="video-container" style="position: relative; width: 100%; height: 0; padding-bottom: 56.25%; border-radius: 8px; overflow: hidden;">
                            <iframe 
                                src={format!("{}{}", embed_url, youtube_params)}
                                style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; border: none;"
                                title="YouTube video player"
                            />
                        </div>
                    }
                } else if embed_url.contains("vimeo.com") {
                    let vimeo_params = format!("?autoplay={}&muted={}&loop={}", autoplay, muted, loop_param);
                    html! {
                        <div class="video-container" style="position: relative; width: 100%; height: 0; padding-bottom: 56.25%; border-radius: 8px; overflow: hidden;">
                            <iframe 
                                src={format!("{}{}", embed_url, vimeo_params)}
                                style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; border: none;"
                                title="Vimeo video player"
                            />
                        </div>
                    }
                } else if video_url.ends_with(".mp4") || video_url.ends_with(".webm") || video_url.ends_with(".ogg") {
                    html! {
                        <div class="video-container" style="width: 100%; border-radius: 8px; overflow: hidden;">
                            <video 
                                style="width: 100%; height: auto; border-radius: 8px;"
                            >
                                <source src={video_url.clone()} type={format!("video/{}", get_video_type(video_url))} />
                                {"Your browser does not support the video tag."}
                            </video>
                        </div>
                    }
                } else {
                    html! { 
                        <div class="video-error" style="background: #fff3cd; border: 1px solid #ffeaa7; padding: 20px; border-radius: 8px; color: #856404; text-align: center;">
                            <div style="font-size: 24px; margin-bottom: 8px;">{"⚠️"}</div>
                            <div style="font-weight: 500; margin-bottom: 4px;">{"Unsupported Video URL"}</div>
                            <div style="font-size: 14px;">{"Please use YouTube, Vimeo, or direct video file URLs (.mp4, .webm, .ogg)"}</div>
                        </div> 
                    }
                }
            }
        }
        ComponentType::Gallery => {
            // Use the enhanced gallery component for rendering
            html! {
                <div class="gallery-preview-container" style="border-radius: 8px; overflow: hidden;">
                    <EnhancedGallery 
                        images={component.properties.gallery_images.iter().map(|img| EnhancedGalleryImage {
                            id: uuid::Uuid::new_v4().to_string(),
                            url: img.url.clone(),
                            alt: img.alt.clone(),
                            caption: img.caption.clone(),
                            title: img.title.clone(),
                            media_id: None,
                        }).collect::<Vec<_>>()}
                        on_images_change={Callback::noop()} // Read-only for preview
                        layout={component.properties.gallery_layout.clone()}
                        columns={component.properties.gallery_columns}
                        gap={component.properties.gallery_gap}
                        border_radius={component.properties.gallery_border_radius}
                        show_captions={component.properties.gallery_show_captions}
                        enable_lightbox={component.properties.gallery_enable_lightbox}
                        enable_drag_reorder={false} // Disable drag reorder in preview
                        is_admin_mode={false} // Preview should behave like public view
                    />
                </div>
            }
        }
        ComponentType::ContactForm => {
            html! {
                <div class="contact-form-container" style="max-width: 600px; margin: 0 auto; padding: 24px; background: var(--public-background-primary, #ffffff); border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                    <div class="form-header" style="text-align: center; margin-bottom: 24px;">
                        <div style="font-size: 48px; margin-bottom: 16px;">{"📧"}</div>
                        <h3 style="margin: 0 0 8px 0; color: var(--public-text-primary, #333);">{"Get in Touch"}</h3>
                        <p style="margin: 0; color: var(--public-text-secondary, #666); font-size: 16px;">{"We'd love to hear from you. Send us a message!"}</p>
                    </div>
                    
                    <form class="contact-form" style="display: grid; gap: 20px;">
                        <div class="form-row" style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px;">
                            <div class="form-group">
                                <label style="display: block; margin-bottom: 6px; font-weight: 500; color: var(--public-text-primary, #333);">{"First Name"}</label>
                                <input 
                                    type="text" 
                                    name="firstName"
                                    required={true}
                                    style="width: 100%; padding: 12px 16px; border: 2px solid var(--public-border-light, #e1e5e9); border-radius: 6px; font-size: 16px; transition: border-color 0.2s; box-sizing: border-box;"
                                    placeholder="Your first name"
                                />
                            </div>
                            <div class="form-group">
                                <label style="display: block; margin-bottom: 6px; font-weight: 500; color: var(--public-text-primary, #333);">{"Last Name"}</label>
                                <input 
                                    type="text" 
                                    name="lastName"
                                    required={true}
                                    style="width: 100%; padding: 12px 16px; border: 2px solid var(--public-border-light, #e1e5e9); border-radius: 6px; font-size: 16px; transition: border-color 0.2s; box-sizing: border-box;"
                                    placeholder="Your last name"
                                />
                            </div>
                        </div>
                        
                        <div class="form-group">
                            <label style="display: block; margin-bottom: 6px; font-weight: 500; color: var(--public-text-primary, #333);">{"Email Address"}</label>
                            <input 
                                type="email" 
                                name="email"
                                required={true}
                                style="width: 100%; padding: 12px 16px; border: 2px solid var(--public-border-light, #e1e5e9); border-radius: 6px; font-size: 16px; transition: border-color 0.2s; box-sizing: border-box;"
                                placeholder="your.email@example.com"
                            />
                        </div>
                        
                        <div class="form-group">
                            <label style="display: block; margin-bottom: 6px; font-weight: 500; color: var(--public-text-primary, #333);">{"Subject"}</label>
                            <input 
                                type="text" 
                                name="subject"
                                required={true}
                                style="width: 100%; padding: 12px 16px; border: 2px solid var(--public-border-light, #e1e5e9); border-radius: 6px; font-size: 16px; transition: border-color 0.2s; box-sizing: border-box;"
                                placeholder="What's this about?"
                            />
                        </div>
                        
                        <div class="form-group">
                            <label style="display: block; margin-bottom: 6px; font-weight: 500; color: var(--public-text-primary, #333);">{"Message"}</label>
                            <textarea 
                                name="message"
                                required={true}
                                rows="5"
                                style="width: 100%; padding: 12px 16px; border: 2px solid var(--public-border-light, #e1e5e9); border-radius: 6px; font-size: 16px; transition: border-color 0.2s; resize: vertical; box-sizing: border-box; font-family: inherit;"
                                placeholder="Your message here..."
                            />
                        </div>
                        
                        <div class="form-actions" style="display: flex; gap: 12px; justify-content: flex-end; margin-top: 8px;">
                            <button 
                                type="reset" 
                                style="padding: 12px 24px; border: 2px solid var(--public-border-light, #e1e5e9); background: transparent; color: var(--public-text-secondary, #666); border-radius: 6px; font-size: 16px; font-weight: 500; cursor: pointer; transition: all 0.2s;"
                            >
                                {"Clear"}
                            </button>
                            <button 
                                type="submit" 
                                style="padding: 12px 32px; border: none; background: var(--public-primary, #3b82f6); color: white; border-radius: 6px; font-size: 16px; font-weight: 500; cursor: pointer; transition: all 0.2s;"
                            >
                                {"Send Message"}
                            </button>
                        </div>
                    </form>
                    
                    <div class="form-note" style="margin-top: 16px; padding: 12px; background: var(--public-background-secondary, #f8f9fa); border-radius: 6px; text-align: center;">
                        <small style="color: var(--public-text-secondary, #666); font-size: 14px;">
                            {"We'll get back to you within 24 hours!"}
                        </small>
                    </div>
                </div>
            }
        }
        ComponentType::Newsletter => {
            html! {
                <div class="newsletter-container" style="max-width: 500px; margin: 0 auto; padding: 32px 24px; background: linear-gradient(135deg, var(--public-primary, #3b82f6) 0%, var(--public-secondary, #1d4ed8) 100%); border-radius: 12px; text-align: center; color: white;">
                    <div class="newsletter-header" style="margin-bottom: 24px;">
                        <div style="font-size: 48px; margin-bottom: 16px;">{"📮"}</div>
                        <h3 style="margin: 0 0 12px 0; font-size: 24px; font-weight: 600;">{"Stay Updated"}</h3>
                        <p style="margin: 0; font-size: 16px; opacity: 0.9; line-height: 1.5;">
                            {"Get the latest updates, articles, and exclusive content delivered to your inbox."}
                        </p>
                    </div>
                    
                    <form class="newsletter-form" style="display: flex; flex-direction: column; gap: 16px;">
                        <div class="email-input-group" style="display: flex; gap: 12px; flex-wrap: wrap;">
                            <input 
                                type="email" 
                                name="email"
                                required={true}
                                style="flex: 1; min-width: 250px; padding: 14px 18px; border: none; border-radius: 8px; font-size: 16px; box-sizing: border-box; outline: none;"
                                placeholder="Enter your email address"
                            />
                            <button 
                                type="submit" 
                                style="padding: 14px 24px; border: none; background: white; color: var(--public-primary, #3b82f6); border-radius: 8px; font-size: 16px; font-weight: 600; cursor: pointer; transition: all 0.2s; white-space: nowrap;"
                            >
                                {"Subscribe"}
                            </button>
                        </div>
                        
                        <div class="newsletter-benefits" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap: 16px; margin-top: 8px; font-size: 14px; opacity: 0.9;">
                            <div class="benefit-item" style="display: flex; align-items: center; gap: 8px; justify-content: center;">
                                <span>{"✨"}</span>
                                <span>{"Weekly insights"}</span>
                            </div>
                            <div class="benefit-item" style="display: flex; align-items: center; gap: 8px; justify-content: center;">
                                <span>{"🎯"}</span>
                                <span>{"Exclusive content"}</span>
                            </div>
                            <div class="benefit-item" style="display: flex; align-items: center; gap: 8px; justify-content: center;">
                                <span>{"🚫"}</span>
                                <span>{"No spam ever"}</span>
                            </div>
                        </div>
                    </form>
                    
                    <div class="newsletter-note" style="margin-top: 20px; padding-top: 16px; border-top: 1px solid rgba(255,255,255,0.2);">
                        <small style="font-size: 13px; opacity: 0.8;">
                            {"Join 1,000+ subscribers. Unsubscribe anytime."}
                        </small>
                    </div>
                </div>
            }
        }
        ComponentType::Map => {
            html! {
                <div class="map-container" style="width: 100%; background: var(--public-background-primary, #ffffff); border-radius: 8px; overflow: hidden; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                    <div class="map-header" style="padding: 20px; background: var(--public-background-secondary, #f8f9fa); border-bottom: 1px solid var(--public-border-light, #e1e5e9);">
                        <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 12px;">
                            <span style="font-size: 24px;">{"🗺️"}</span>
                            <h3 style="margin: 0; color: var(--public-text-primary, #333);">{"Visit Our Location"}</h3>
                        </div>
                        <div class="location-details" style="color: var(--public-text-secondary, #666); line-height: 1.6;">
                            <div style="margin-bottom: 8px; font-weight: 500;">{"Our Office"}</div>
                            <div style="margin-bottom: 4px;">{"123 Business District"}</div>
                            <div style="margin-bottom: 4px;">{"Tech City, TC 12345"}</div>
                            <div style="margin-bottom: 8px;">{"United States"}</div>
                            <div style="font-size: 14px;">
                                <span style="margin-right: 16px;">{"📞 (555) 123-4567"}</span>
                                <span>{"📧 hello@example.com"}</span>
                            </div>
                        </div>
                    </div>
                    
                    <div class="map-placeholder" style="height: 300px; background: linear-gradient(135deg, #e3f2fd 0%, #bbdefb 50%, #90caf9 100%); display: flex; align-items: center; justify-content: center; position: relative;">
                        <div class="map-content" style="text-align: center; color: var(--public-text-primary, #1565c0);">
                            <div style="font-size: 48px; margin-bottom: 16px;">{"📍"}</div>
                            <div style="font-size: 18px; font-weight: 500; margin-bottom: 8px;">{"Interactive Map"}</div>
                            <div style="font-size: 14px; opacity: 0.8;">{"Click to open in Google Maps"}</div>
                        </div>
                        
                        // Simulate map with some location markers
                        <div style="position: absolute; top: 20%; left: 30%; width: 8px; height: 8px; background: #e53e3e; border-radius: 50%; box-shadow: 0 0 0 3px rgba(229, 62, 62, 0.3);"></div>
                        <div style="position: absolute; top: 40%; left: 60%; width: 6px; height: 6px; background: #38a169; border-radius: 50%; box-shadow: 0 0 0 2px rgba(56, 161, 105, 0.3);"></div>
                        <div style="position: absolute; top: 70%; left: 20%; width: 6px; height: 6px; background: #3182ce; border-radius: 50%; box-shadow: 0 0 0 2px rgba(49, 130, 206, 0.3);"></div>
                        
                        // Main location marker (larger)
                        <div style="position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); width: 16px; height: 16px; background: #d53f8c; border: 3px solid white; border-radius: 50%; box-shadow: 0 2px 8px rgba(0,0,0,0.2), 0 0 0 4px rgba(213, 63, 140, 0.3);"></div>
                    </div>
                    
                    <div class="map-footer" style="padding: 16px 20px; background: var(--public-background-secondary, #f8f9fa); display: flex; justify-content: space-between; align-items: center; gap: 16px; flex-wrap: wrap;">
                        <div class="business-hours" style="color: var(--public-text-secondary, #666); font-size: 14px;">
                            <span style="font-weight: 500;">{"Hours: "}</span>
                            <span>{"Mon-Fri 9AM-6PM"}</span>
                        </div>
                        <div class="map-actions" style="display: flex; gap: 12px;">
                            <button style="padding: 8px 16px; border: 1px solid var(--public-border-light, #e1e5e9); background: white; color: var(--public-text-primary, #333); border-radius: 4px; font-size: 14px; cursor: pointer; transition: all 0.2s;">
                                {"📍 Get Directions"}
                            </button>
                            <button style="padding: 8px 16px; border: 1px solid var(--public-primary, #3b82f6); background: var(--public-primary, #3b82f6); color: white; border-radius: 4px; font-size: 14px; cursor: pointer; transition: all 0.2s;">
                                {"🗺️ View in Maps"}
                            </button>
                        </div>
                    </div>
                </div>
            }
        }
        ComponentType::Spacer => {
            html! { <div class="component-spacer" style="height: 40px; background: transparent;"></div> }
        }
        ComponentType::Divider => {
            let divider_style = format!(
                "border: none; height: {}; background: {}; border-top: {} {} {}; margin: {} 0; width: {}; margin-left: auto; margin-right: auto;",
                component.properties.divider_thickness,
                "transparent",
                component.properties.divider_thickness,
                component.properties.divider_style,
                component.properties.divider_color,
                component.properties.divider_margin,
                component.properties.divider_width
            );
            
            html! { 
                <div class="component-divider" style="display: flex; justify-content: center; width: 100%;">
                    <hr style={divider_style} />
                </div> 
            }
        }
        ComponentType::PostsList => {
            html! {
                <div class="posts-list-component" style="width: 100%; padding: 20px; background: var(--public-background-primary, #ffffff); border: 2px dashed var(--public-border-light, #ddd); border-radius: 8px; text-align: center;">
                    <div style="color: var(--public-text-secondary, #666); margin-bottom: 16px;">
                        <span style="font-size: 48px; display: block; margin-bottom: 12px;">{"📄"}</span>
                        <h3 style="margin: 0 0 8px 0; font-size: 18px; color: var(--public-text-primary, #333);">{"Posts List Component"}</h3>
                        <p style="margin: 0; font-size: 14px; line-height: 1.5;">
                            {"This will display your latest blog posts in a grid layout. "}
                            {"The actual posts from your CMS will appear here when published on your site."}
                        </p>
                    </div>
                    <div style="display: flex; justify-content: center; gap: 12px; flex-wrap: wrap; font-size: 12px; color: var(--public-text-secondary, #888);">
                        <span>{"📋 Dynamic Content"}</span>
                        <span>{"🎨 Customizable Layout"}</span>
                        <span>{"📱 Responsive Design"}</span>
                    </div>
                </div>
            }
        }
        ComponentType::Comments => {
            html! {
                <div class="comments-component" style="width: 100%; padding: 20px; background: var(--public-background-primary, #ffffff); border: 2px dashed var(--public-border-light, #ddd); border-radius: 8px; text-align: center;">
                    <div style="color: var(--public-text-secondary, #666); margin-bottom: 16px;">
                        <span style="font-size: 48px; display: block; margin-bottom: 12px;">{"💬"}</span>
                        <h3 style="margin: 0 0 8px 0; font-size: 18px; color: var(--public-text-primary, #333);">{"Comments Component"}</h3>
                        <p style="margin: 0; font-size: 14px; line-height: 1.5;">
                            {"Interactive comments section with text message-style bubbles and Gravatar avatars. "}
                            {"Users can sign in and leave comments that appear here in real-time."}
                        </p>
                    </div>
                    
                    // Preview of comment bubbles
                    <div style="display: flex; flex-direction: column; gap: 12px; margin-top: 20px; text-align: left; max-width: 500px; margin-left: auto; margin-right: auto;">
                        // Sample comment 1
                        <div style="display: flex; gap: 12px; align-items: flex-start;">
                            <div style="width: 32px; height: 32px; border-radius: 50%; background: linear-gradient(45deg, #3b82f6, #1d4ed8); flex-shrink: 0;"></div>
                            <div style="flex: 1;">
                                <div style="background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%); padding: 8px 12px; border-radius: 16px 16px 16px 4px; font-size: 13px; position: relative;">
                                    <div style="font-weight: 600; margin-bottom: 2px; color: #2c3e50;">{"Alex Johnson"}</div>
                                    <div style="color: #374151;">{"Great article! The new features look amazing."}</div>
                                </div>
                                <div style="font-size: 11px; color: #6b7280; margin-top: 4px; margin-left: 12px;">{"2 hours ago"}</div>
                            </div>
                        </div>
                        
                        // Sample comment 2  
                        <div style="display: flex; gap: 12px; align-items: flex-start;">
                            <div style="width: 32px; height: 32px; border-radius: 50%; background: linear-gradient(45deg, #10b981, #047857); flex-shrink: 0;"></div>
                            <div style="flex: 1;">
                                <div style="background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%); padding: 8px 12px; border-radius: 16px 16px 16px 4px; font-size: 13px; position: relative;">
                                    <div style="font-weight: 600; margin-bottom: 2px; color: #2c3e50;">{"Sarah Chen"}</div>
                                    <div style="color: #374151;">{"Can't wait to try this out on my project!"}</div>
                                </div>
                                <div style="font-size: 11px; color: #6b7280; margin-top: 4px; margin-left: 12px;">{"5 hours ago"}</div>
                            </div>
                        </div>
                    </div>
                    
                    <div style="margin-top: 20px; padding-top: 16px; border-top: 1px solid var(--public-border-light, #e1e5e9);">
                        <small style="color: var(--public-text-muted, #888); font-size: 12px;">
                            {"💡 Comments will be interactive when published. Users can sign in/up to participate."}
                        </small>
                    </div>
                </div>
            }
        }
        ComponentType::Container => {
            html! { 
                <div class="component-container" style="min-height: 100px; border: 2px dashed #ddd; border-radius: 8px; padding: 16px; background: #f9f9f9;">
                    <div class="container-drop-zone" 
                         style="min-height: 60px; border: 1px dashed #ccc; border-radius: 4px; padding: 8px; background: white; display: flex; align-items: center; justify-content: center; color: #666; font-style: italic;">
                        {if component.properties.nested_components.is_empty() {
                            html! { 
                                <div class="drop-placeholder">
                                    {"📦 Drop components here to add them to this container"}
                                </div>
                            }
                        } else {
                            html! {
                                <div class="nested-components">
                                    {component.properties.nested_components.iter().map(|nested_comp| {
                                        html! {
                                            <div class="nested-component" style="margin: 8px 0; padding: 8px; border: 1px solid #eee; border-radius: 4px;">
                                                <div class="nested-header" style="font-size: 12px; color: #666; margin-bottom: 4px;">
                                                    {format!("{} Component", nested_comp.component_type.display_name())}
                                                </div>
                                                <div style="pointer-events: none;">
                                                    {render_component_content(nested_comp)}
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Html>()}
                                </div>
                            }
                        }}
                    </div>
                </div> 
            }
        }
        ComponentType::TwoColumn => {
            html! { 
                <div class="component-two-column" style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px; min-height: 120px; border: 2px dashed #ddd; border-radius: 8px; padding: 16px; background: #f9f9f9;">
                    <div class="column column-1" style="min-height: 80px; border: 1px dashed #ccc; border-radius: 4px; padding: 8px; background: white;">
                        <div class="column-header" style="font-size: 12px; color: #666; margin-bottom: 8px; text-align: center;">{"Column 1"}</div>
                        <div class="column-drop-zone" style="min-height: 40px; display: flex; align-items: center; justify-content: center; color: #666; font-style: italic;">
                            {if component.properties.column_1_components.is_empty() {
                                html! { <div class="drop-placeholder">{"Drop components here"}</div> }
                            } else {
                                html! {
                                    <div class="nested-components">
                                        {component.properties.column_1_components.iter().map(|nested_comp| {
                                            html! {
                                                <div class="nested-component" style="margin: 4px 0; padding: 4px; border: 1px solid #eee; border-radius: 4px;">
                                                    <div style="pointer-events: none;">
                                                    {render_component_content(nested_comp)}
                                                </div>
                                                </div>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                    <div class="column column-2" style="min-height: 80px; border: 1px dashed #ccc; border-radius: 4px; padding: 8px; background: white;">
                        <div class="column-header" style="font-size: 12px; color: #666; margin-bottom: 8px; text-align: center;">{"Column 2"}</div>
                        <div class="column-drop-zone" style="min-height: 40px; display: flex; align-items: center; justify-content: center; color: #666; font-style: italic;">
                            {if component.properties.column_2_components.is_empty() {
                                html! { <div class="drop-placeholder">{"Drop components here"}</div> }
                            } else {
                                html! {
                                    <div class="nested-components">
                                        {component.properties.column_2_components.iter().map(|nested_comp| {
                                            html! {
                                                <div class="nested-component" style="margin: 4px 0; padding: 4px; border: 1px solid #eee; border-radius: 4px;">
                                                    <div style="pointer-events: none;">
                                                    {render_component_content(nested_comp)}
                                                </div>
                                                </div>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                </div> 
            }
        }
        ComponentType::ThreeColumn => {
            html! { 
                <div class="component-three-column" style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px; min-height: 120px; border: 2px dashed #ddd; border-radius: 8px; padding: 16px; background: #f9f9f9;">
                    <div class="column column-1" style="min-height: 80px; border: 1px dashed #ccc; border-radius: 4px; padding: 8px; background: white;">
                        <div class="column-header" style="font-size: 12px; color: #666; margin-bottom: 8px; text-align: center;">{"Column 1"}</div>
                        <div class="column-drop-zone" style="min-height: 40px; display: flex; align-items: center; justify-content: center; color: #666; font-style: italic;">
                            {if component.properties.column_1_components.is_empty() {
                                html! { <div class="drop-placeholder">{"Drop components here"}</div> }
                            } else {
                                html! {
                                    <div class="nested-components">
                                        {component.properties.column_1_components.iter().map(|nested_comp| {
                                            html! {
                                                <div class="nested-component" style="margin: 4px 0; padding: 4px; border: 1px solid #eee; border-radius: 4px;">
                                                    <div style="pointer-events: none;">
                                                    {render_component_content(nested_comp)}
                                                </div>
                </div> 
            }
                                        }).collect::<Html>()}
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                    <div class="column column-2" style="min-height: 80px; border: 1px dashed #ccc; border-radius: 4px; padding: 8px; background: white;">
                        <div class="column-header" style="font-size: 12px; color: #666; margin-bottom: 8px; text-align: center;">{"Column 2"}</div>
                        <div class="column-drop-zone" style="min-height: 40px; display: flex; align-items: center; justify-content: center; color: #666; font-style: italic;">
                            {if component.properties.column_2_components.is_empty() {
                                html! { <div class="drop-placeholder">{"Drop components here"}</div> }
                            } else {
                                html! {
                                    <div class="nested-components">
                                        {component.properties.column_2_components.iter().map(|nested_comp| {
                                            html! {
                                                <div class="nested-component" style="margin: 4px 0; padding: 4px; border: 1px solid #eee; border-radius: 4px;">
                                                    <div style="pointer-events: none;">
                                                    {render_component_content(nested_comp)}
                                                </div>
                                                </div>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                    <div class="column column-3" style="min-height: 80px; border: 1px dashed #ccc; border-radius: 4px; padding: 8px; background: white;">
                        <div class="column-header" style="font-size: 12px; color: #666; margin-bottom: 8px; text-align: center;">{"Column 3"}</div>
                        <div class="column-drop-zone" style="min-height: 40px; display: flex; align-items: center; justify-content: center; color: #666; font-style: italic;">
                            {if component.properties.column_3_components.is_empty() {
                                html! { <div class="drop-placeholder">{"Drop components here"}</div> }
                            } else {
                                html! {
                                    <div class="nested-components">
                                        {component.properties.column_3_components.iter().map(|nested_comp| {
                                            html! {
                                                <div class="nested-component" style="margin: 4px 0; padding: 4px; border: 1px solid #eee; border-radius: 4px;">
                                                    <div style="pointer-events: none;">
                                                    {render_component_content(nested_comp)}
                                                </div>
                                                </div>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                </div> 
            }
        }
        ComponentType::PageTitle => {
            let tag = &component.properties.page_title_tag;
            let prefix = if component.properties.page_title_show_prefix && !component.properties.page_title_prefix.is_empty() {
                format!("{} ", component.properties.page_title_prefix)
            } else {
                String::new()
            };
            let suffix = if component.properties.page_title_show_suffix && !component.properties.page_title_suffix.is_empty() {
                format!(" {}", component.properties.page_title_suffix)
            } else {
                String::new()
            };
            
            let title_content = format!("{}[Page Title]{}", prefix, suffix);
            
            match tag.as_str() {
                "h1" => html! { <h1 style="margin: 0;">{title_content}</h1> },
                "h2" => html! { <h2 style="margin: 0;">{title_content}</h2> },
                "h3" => html! { <h3 style="margin: 0;">{title_content}</h3> },
                "h4" => html! { <h4 style="margin: 0;">{title_content}</h4> },
                "h5" => html! { <h5 style="margin: 0;">{title_content}</h5> },
                "h6" => html! { <h6 style="margin: 0;">{title_content}</h6> },
                _ => html! { <h1 style="margin: 0;">{title_content}</h1> },
            }
        }
        ComponentType::PublishedDate => {
            let prefix = if component.properties.published_date_show_prefix && !component.properties.published_date_prefix.is_empty() {
                format!("{} ", component.properties.published_date_prefix)
            } else {
                String::new()
            };
            
            let date_content = if component.properties.published_date_relative {
                format!("{}[Published 2 days ago]", prefix)
            } else {
                match component.properties.published_date_format.as_str() {
                    "YYYY-MM-DD" => format!("{}[2024-08-25]", prefix),
                    "DD/MM/YYYY" => format!("{}[25/08/2024]", prefix),
                    "MM/DD/YYYY" => format!("{}[08/25/2024]", prefix),
                    "Month DD, YYYY" => format!("{}[August 25, 2024]", prefix),
                    _ => format!("{}[August 25, 2024]", prefix),
                }
            };
            
            let time_suffix = if component.properties.published_date_show_time {
                " at 2:30 PM"
            } else {
                ""
            };
            
            html! {
                <div class="published-date" style="color: var(--public-text-secondary, #666); font-size: 0.9em;">
                    {format!("{}{}", date_content, time_suffix)}
                </div>
            }
        }
    }
}

// Flatten component hierarchy - create a list of all components (main + nested) for unified rendering
#[allow(dead_code)]
fn flatten_components_for_rendering(components: &[PageComponent]) -> Vec<ComponentWithContext> {
    let mut flattened = Vec::new();
    
    for component in components {
        // Add the main component
        flattened.push(ComponentWithContext {
            component: component.clone(),
            context: ComponentContext::Main,
            parent_id: None,
        });
        
        // Add nested components based on component type
        match component.component_type {
            ComponentType::Container => {
                for nested_comp in &component.properties.nested_components {
                    flattened.push(ComponentWithContext {
                        component: nested_comp.clone(),
                        context: ComponentContext::NestedInContainer,
                        parent_id: Some(component.id.clone()),
                    });
                }
            },
            ComponentType::TwoColumn => {
                for nested_comp in &component.properties.column_1_components {
                    flattened.push(ComponentWithContext {
                        component: nested_comp.clone(),
                        context: ComponentContext::NestedInColumn { column: 1 },
                        parent_id: Some(component.id.clone()),
                    });
                }
                for nested_comp in &component.properties.column_2_components {
                    flattened.push(ComponentWithContext {
                        component: nested_comp.clone(),
                        context: ComponentContext::NestedInColumn { column: 2 },
                        parent_id: Some(component.id.clone()),
                    });
                }
            },
            ComponentType::ThreeColumn => {
                for nested_comp in &component.properties.column_1_components {
                    flattened.push(ComponentWithContext {
                        component: nested_comp.clone(),
                        context: ComponentContext::NestedInColumn { column: 1 },
                        parent_id: Some(component.id.clone()),
                    });
                }
                for nested_comp in &component.properties.column_2_components {
                    flattened.push(ComponentWithContext {
                        component: nested_comp.clone(),
                        context: ComponentContext::NestedInColumn { column: 2 },
                        parent_id: Some(component.id.clone()),
                    });
                }
                for nested_comp in &component.properties.column_3_components {
                    flattened.push(ComponentWithContext {
                        component: nested_comp.clone(),
                        context: ComponentContext::NestedInColumn { column: 3 },
                        parent_id: Some(component.id.clone()),
                    });
                }
            },
            _ => {
                // Other component types don't have nested components
            }
        }
    }
    
    flattened
}

#[derive(Clone)]
#[allow(dead_code)]
struct ComponentWithContext {
    component: PageComponent,
    context: ComponentContext,
    parent_id: Option<String>,
}

#[derive(Clone)]
#[allow(dead_code)]
enum ComponentContext {
    Main,
    NestedInContainer,
    NestedInColumn { column: i32 },
}

// Render a selectable nested component with full selection capabilities
fn render_selectable_nested_component(
    component: &PageComponent,
    selected_component: &UseStateHandle<Option<String>>,
    on_component_click: &Callback<String>,
    on_component_edit: &Callback<String>,
    on_component_duplicate: &Callback<String>,
    on_component_delete: &Callback<String>
) -> Html {
    let is_selected = selected_component.as_ref() == Some(&component.id);
    
    let on_click = {
        let on_component_click = on_component_click.clone();
        let component_id = component.id.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation(); // Prevent parent component from being selected
            on_component_click.emit(component_id.clone());
        })
    };
    
    let on_edit = {
        let on_component_edit = on_component_edit.clone();
        let component_id = component.id.clone();
        Callback::from(move |_| {
            on_component_edit.emit(component_id.clone());
        })
    };
    
    let on_duplicate = {
        let on_component_duplicate = on_component_duplicate.clone();
        let component_id = component.id.clone();
        Callback::from(move |_| {
            on_component_duplicate.emit(component_id.clone());
        })
    };
    
    let on_delete = {
        let on_component_delete = on_component_delete.clone();
        let component_id = component.id.clone();
        Callback::from(move |_| {
            on_component_delete.emit(component_id.clone());
        })
    };
    
    let selection_border = if is_selected {
        "2px solid #007bff"
    } else {
        &format!("{} {} {}", component.styles.border_width, component.styles.border_style, component.styles.border_color)
    };
    
    let selection_box_shadow = if is_selected {
        format!("{}, 0 0 0 1px rgba(0, 123, 255, 0.25)", component.styles.box_shadow)
    } else {
        component.styles.box_shadow.clone()
    };
    
    html! {
        <div 
            class={classes!("nested-component", if is_selected { Some("selected") } else { None })}
            onclick={on_click}
            style={format!(
                "background-color: {}; color: {}; padding: {}; margin: 4px; border-radius: {}; font-size: {}; font-weight: {}; text-align: {}; border: {}; box-shadow: {}; opacity: {}; z-index: {}; font-family: {}; line-height: {}; letter-spacing: {}; text-decoration: {}; text-transform: {}; background-image: {}; background-size: {}; background-position: {}; background-repeat: {}; position: relative; cursor: pointer;",
                component.styles.background_color,
                component.styles.text_color,
                component.styles.padding,
                component.styles.border_radius,
                component.styles.font_size,
                component.styles.font_weight,
                component.styles.text_align,
                selection_border,
                selection_box_shadow,
                component.styles.opacity,
                if is_selected { "10" } else { &component.styles.z_index.to_string() },
                component.styles.font_family,
                component.styles.line_height,
                component.styles.letter_spacing,
                component.styles.text_decoration,
                component.styles.text_transform,
                component.styles.background_image,
                component.styles.background_size,
                component.styles.background_position,
                component.styles.background_repeat
            )}
        >
            {render_component_content(component)}
            
            {if is_selected {
                html! {
                    <>
                        <div class="nested-selection-indicator" style="position: absolute; top: -6px; left: -6px; background: #007bff; color: white; padding: 1px 4px; border-radius: 2px; font-size: 8px; font-weight: 600; z-index: 11; box-shadow: 0 1px 3px rgba(0,0,0,0.2);">
                            {"✓ Nested"}
                        </div>
                        <div class="nested-component-controls" style="position: absolute; top: -30px; right: -6px; display: flex; gap: 2px; z-index: 11;">
                            <button class="nested-control-btn" onclick={on_edit} title="Edit Nested Component" style="padding: 2px 4px; background: #6c757d; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"✏️"}</button>
                            <button class="nested-control-btn" onclick={on_duplicate} title="Duplicate Nested Component" style="padding: 2px 4px; background: #17a2b8; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"📋"}</button>
                            <button class="nested-control-btn" onclick={on_delete} title="Delete Nested Component" style="padding: 2px 4px; background: #dc3545; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"🗑️"}</button>
                        </div>
                    </>
                }
            } else { html! {} }}
        </div>
    }
}

// Enhanced component content renderer that supports drop zones for nested components
fn render_component_content_with_drop_zones(
    component: &PageComponent, 
    on_nested_drop: Callback<(String, String)>,
    _dragging_component: UseStateHandle<Option<ComponentType>>,
    container_drag_over: UseStateHandle<Option<String>>,
    column_drag_over: UseStateHandle<Option<(String, String)>>,
    selected_component: UseStateHandle<Option<String>>,
    on_component_click: Callback<String>,
    on_component_edit: Callback<String>,
    on_component_duplicate: Callback<String>,
    on_component_delete: Callback<String>
) -> Html {
    match component.component_type {
        ComponentType::Sidebar => {
            // Both columns are droppable. Map columns based on sidebar position.
            let is_left = component.properties.sidebar_position == "left";
            let container_id = component.id.clone();

            // Determine which logical columns map to column-1 / column-2
            let (sidebar_col, body_col) = if is_left { ("column-1", "column-2") } else { ("column-2", "column-1") };

            // Drag-over checks
            let is_drag_over_sidebar = match (*column_drag_over).as_ref() {
                Some((id, col)) if id == &container_id && col == sidebar_col => true,
                _ => false,
            };
            let is_drag_over_body = match (*column_drag_over).as_ref() {
                Some((id, col)) if id == &container_id && col == body_col => true,
                _ => false,
            };

            // Handlers: sidebar column
            let on_drop_sidebar = {
                let on_nested_drop = on_nested_drop.clone();
                let container_id = container_id.clone();
                let sidebar_col = sidebar_col.to_string();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    on_nested_drop.emit((container_id.clone(), sidebar_col.clone()));
                })
            };
            let on_drag_over_sidebar = {
                let column_drag_over = column_drag_over.clone();
                let container_id = container_id.clone();
                let sidebar_col = sidebar_col.to_string();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    column_drag_over.set(Some((container_id.clone(), sidebar_col.clone())));
                })
            };
            let on_drag_leave_sidebar = {
                let column_drag_over = column_drag_over.clone();
                Callback::from(move |e: DragEvent| {
                    e.stop_propagation();
                    column_drag_over.set(None);
                })
            };

            // Handlers: body column
            let on_drop_body = {
                let on_nested_drop = on_nested_drop.clone();
                let container_id = container_id.clone();
                let body_col = body_col.to_string();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    on_nested_drop.emit((container_id.clone(), body_col.clone()));
                })
            };
            let on_drag_over_body = {
                let column_drag_over = column_drag_over.clone();
                let container_id = container_id.clone();
                let body_col = body_col.to_string();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    column_drag_over.set(Some((container_id.clone(), body_col.clone())));
                })
            };
            let on_drag_leave_body = {
                let column_drag_over = column_drag_over.clone();
                Callback::from(move |e: DragEvent| {
                    e.stop_propagation();
                    column_drag_over.set(None);
                })
            };

            // Styles
            let drop_zone_style_sidebar = if is_drag_over_sidebar {
                "min-height: 60px; border: 2px dashed #007bff; border-radius: 4px; padding: 8px; background: #e3f2fd; display: flex; align-items: center; justify-content: center; color: #007bff; font-style: italic; font-weight: 500; box-shadow: 0 0 8px rgba(0,123,255,0.3);"
            } else {
                "min-height: 60px; border: 1px dashed #ccc; border-radius: 4px; padding: 8px; background: white; display: flex; align-items: center; justify-content: center; color: #666; font-style: italic;"
            };
            let drop_zone_style_body = if is_drag_over_body {
                "min-height: 60px; border: 2px dashed #007bff; border-radius: 4px; padding: 8px; background: #e3f2fd; display: flex; align-items: center; justify-content: center; color: #007bff; font-style: italic; font-weight: 500; box-shadow: 0 0 8px rgba(0,123,255,0.3);"
            } else {
                "min-height: 60px; border: 1px dashed #ccc; border-radius: 4px; padding: 8px; background: white; display: flex; align-items: center; justify-content: center; color: #666; font-style: italic;"
            };
            let grid_style = if is_left {
                format!("display: grid; grid-template-columns: {} 1fr; gap: 16px;", component.properties.sidebar_width)
            } else {
                format!("display: grid; grid-template-columns: 1fr {}; gap: 16px;", component.properties.sidebar_width)
            };
            let sticky_style = if component.properties.sidebar_sticky { "position: sticky; top: 16px;" } else { "" };

            // Column references for rendering children
            let (col1_children, col2_children) = (&component.properties.column_1_components, &component.properties.column_2_components);

            html! {
                <div class="component-sidebar" style={grid_style}>
                    // Left column
                    <div class="sidebar-col-1">
                        <div class="column-drop-zone"
                             style={if sidebar_col == "column-1" { drop_zone_style_sidebar } else { drop_zone_style_body }}
                             ondragover={if sidebar_col == "column-1" { on_drag_over_sidebar.clone() } else { on_drag_over_body.clone() }}
                             ondragleave={if sidebar_col == "column-1" { on_drag_leave_sidebar.clone() } else { on_drag_leave_body.clone() }}
                             ondrop={if sidebar_col == "column-1" { on_drop_sidebar.clone() } else { on_drop_body.clone() }}
                        >
                            {if col1_children.is_empty() { html! { <div class="drop-placeholder">{"Drop here"}</div> } } else {
                                html! { <div class="nested-components">{col1_children.iter().map(|nested_comp| {
                                    render_selectable_nested_component(
                                        nested_comp,
                                        &selected_component,
                                        &on_component_click,
                                        &on_component_edit,
                                        &on_component_duplicate,
                                        &on_component_delete,
                                    )
                                }).collect::<Html>()}</div> }
                            }}
                        </div>
                        if is_left {
                            <div class="sidebar-rail-meta" style={format!("margin-top: 8px; {}", sticky_style)}>
                                <div style="font-size: 12px; color: #666; text-align: center;">{"Sidebar (Left)"}</div>
                            </div>
                        }
                    </div>

                    // Right column
                    <div class="sidebar-col-2">
                        <div class="column-drop-zone"
                             style={if sidebar_col == "column-2" { drop_zone_style_sidebar } else { drop_zone_style_body }}
                             ondragover={if sidebar_col == "column-2" { on_drag_over_sidebar.clone() } else { on_drag_over_body.clone() }}
                             ondragleave={if sidebar_col == "column-2" { on_drag_leave_sidebar.clone() } else { on_drag_leave_body.clone() }}
                             ondrop={if sidebar_col == "column-2" { on_drop_sidebar.clone() } else { on_drop_body.clone() }}
                        >
                            {if col2_children.is_empty() { html! { <div class="drop-placeholder">{"Drop here"}</div> } } else {
                                html! { <div class="nested-components">{col2_children.iter().map(|nested_comp| {
                                    render_selectable_nested_component(
                                        nested_comp,
                                        &selected_component,
                                        &on_component_click,
                                        &on_component_edit,
                                        &on_component_duplicate,
                                        &on_component_delete,
                                    )
                                }).collect::<Html>()}</div> }
                            }}
                        </div>
                        if !is_left {
                            <div class="sidebar-rail-meta" style={format!("margin-top: 8px; {}", sticky_style)}>
                                <div style="font-size: 12px; color: #666; text-align: center;">{"Sidebar (Right)"}</div>
                            </div>
                        }
                    </div>
                </div>
            }
        }
        ComponentType::Container => {
            let container_id = component.id.clone();
            let is_drag_over = (*container_drag_over).as_ref() == Some(&container_id);
            
            let on_drop = {
                let on_nested_drop = on_nested_drop.clone();
                let container_id = container_id.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    on_nested_drop.emit((container_id.clone(), "container".to_string()));
                })
            };
            
            let on_drag_over = {
                let container_drag_over = container_drag_over.clone();
                let container_id = container_id.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    container_drag_over.set(Some(container_id.clone()));
                })
            };
            
            let on_drag_leave = {
                let container_drag_over = container_drag_over.clone();
                Callback::from(move |e: DragEvent| {
                    e.stop_propagation();
                    container_drag_over.set(None);
                })
            };
            
            let drop_zone_style = if is_drag_over {
                "min-height: 60px; border: 2px dashed #007bff; border-radius: 4px; padding: 8px; background: #e3f2fd; display: flex; align-items: center; justify-content: center; color: #007bff; font-style: italic; font-weight: 500; box-shadow: 0 0 8px rgba(0,123,255,0.3);"
            } else {
                "min-height: 60px; border: 1px dashed #ccc; border-radius: 4px; padding: 8px; background: white; display: flex; align-items: center; justify-content: center; color: #666; font-style: italic;"
            };
            
            html! { 
                <div class="component-container" style="min-height: 100px; border: 2px dashed #ddd; border-radius: 8px; padding: 16px; background: #f9f9f9;">
                    <div class="container-drop-zone" 
                         style={drop_zone_style}
                         ondragover={on_drag_over}
                         ondragleave={on_drag_leave}
                         ondrop={on_drop}
                    >
                        {if component.properties.nested_components.is_empty() {
                            html! { 
                                <div class="drop-placeholder">
                                    {"📦 Drop components here to add them to this container"}
                                </div>
                            }
                        } else {
                            html! {
                                <div class="nested-components">
                                    {component.properties.nested_components.iter().map(|nested_comp| {
                                        let is_selected = selected_component.as_ref() == Some(&nested_comp.id);
                                        let nested_on_click = {
                                            let on_component_click = on_component_click.clone();
                                            let component_id = nested_comp.id.clone();
                                            Callback::from(move |e: MouseEvent| {
                                                e.stop_propagation();
                                                on_component_click.emit(component_id.clone());
                                            })
                                        };
                                        
                                        let border_style = if is_selected {
                                            "2px solid #007bff"
                                        } else {
                                            "1px solid #eee"
                                        };
                                        
                                        let box_shadow_style = if is_selected {
                                            "0 0 0 1px rgba(0, 123, 255, 0.25)"
                                        } else {
                                            "none"
                                        };
                                        
                                        html! {
                                            <div class="nested-component-wrapper" 
                                                 style={format!("margin: 8px 0; padding: 8px; border: {}; border-radius: 4px; cursor: pointer; position: relative; box-shadow: {}; background: white; pointer-events: auto;", border_style, box_shadow_style)}
                                                 onclick={nested_on_click}>
                                                {if is_selected {
                                                    let component_id = nested_comp.id.clone();
                                                    let on_edit = {
                                                        let on_component_edit = on_component_edit.clone();
                                                        let component_id = component_id.clone();
                                                        Callback::from(move |_| {
                                                            on_component_edit.emit(component_id.clone());
                                                        })
                                                    };
                                                    let on_duplicate = {
                                                        let on_component_duplicate = on_component_duplicate.clone();
                                                        let component_id = component_id.clone();
                                                        Callback::from(move |_| {
                                                            on_component_duplicate.emit(component_id.clone());
                                                        })
                                                    };
                                                    let on_delete = {
                                                        let on_component_delete = on_component_delete.clone();
                                                        let component_id = component_id.clone();
                                                        Callback::from(move |_| {
                                                            on_component_delete.emit(component_id.clone());
                                                        })
                                                    };
                                                    
                                                    html! {
                                                        <>
                                                            <div class="nested-selection-indicator" style="position: absolute; top: -6px; left: -6px; background: #007bff; color: white; padding: 1px 4px; border-radius: 2px; font-size: 8px; font-weight: 600; z-index: 11;">
                                                                {"✓ Nested"}
                                                            </div>
                                                            <div class="nested-component-controls" style="position: absolute; top: -6px; right: -6px; display: flex; gap: 2px; z-index: 11;">
                                                                <button class="control-btn" onclick={on_edit} title="Edit Nested Component" style="padding: 2px 4px; background: #6c757d; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"✏️"}</button>
                                                                <button class="control-btn" onclick={on_duplicate} title="Duplicate Nested Component" style="padding: 2px 4px; background: #17a2b8; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"📋"}</button>
                                                                <button class="control-btn" onclick={on_delete} title="Delete Nested Component" style="padding: 2px 4px; background: #dc3545; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"🗑️"}</button>
                                                            </div>
                                                        </>
                                                    }
                                                } else { html! {} }}
                                                <div class="nested-header" style="font-size: 12px; color: #666; margin-bottom: 4px;">
                                                    {format!("{} Component", nested_comp.component_type.display_name())}
                                                </div>
                                                <div style="pointer-events: none;">
                                                    {render_component_content(nested_comp)}
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Html>()}
                                </div>
                            }
                        }}
                    </div>
                </div> 
            }
        }
        ComponentType::TwoColumn => {
            let container_id = component.id.clone();
            let is_col1_drag_over = (*column_drag_over).as_ref() == Some(&(container_id.clone(), "column-1".to_string()));
            let is_col2_drag_over = (*column_drag_over).as_ref() == Some(&(container_id.clone(), "column-2".to_string()));
            
            let on_drop_col1 = {
                let on_nested_drop = on_nested_drop.clone();
                let container_id = container_id.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    on_nested_drop.emit((container_id.clone(), "column-1".to_string()));
                })
            };
            let on_drop_col2 = {
                let on_nested_drop = on_nested_drop.clone();
                let container_id = container_id.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    on_nested_drop.emit((container_id.clone(), "column-2".to_string()));
                })
            };
            
            let on_drag_over_col1 = {
                let column_drag_over = column_drag_over.clone();
                let container_id = container_id.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    column_drag_over.set(Some((container_id.clone(), "column-1".to_string())));
                })
            };
            
            let on_drag_over_col2 = {
                let column_drag_over = column_drag_over.clone();
                let container_id = container_id.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    column_drag_over.set(Some((container_id.clone(), "column-2".to_string())));
                })
            };
            
            let on_drag_leave_col = {
                let column_drag_over = column_drag_over.clone();
                Callback::from(move |e: DragEvent| {
                    e.stop_propagation();
                    column_drag_over.set(None);
                })
            };
            
            let col1_style = if is_col1_drag_over {
                "min-height: 40px; display: flex; align-items: center; justify-content: center; color: #007bff; font-style: italic; font-weight: 500; border: 2px dashed #007bff; border-radius: 4px; padding: 6px; background: #e3f2fd; box-shadow: 0 0 6px rgba(0,123,255,0.3);"
            } else {
                "min-height: 40px; display: flex; align-items: center; justify-content: center; color: #666; font-style: italic;"
            };
            
            let col2_style = if is_col2_drag_over {
                "min-height: 40px; display: flex; align-items: center; justify-content: center; color: #007bff; font-style: italic; font-weight: 500; border: 2px dashed #007bff; border-radius: 4px; padding: 6px; background: #e3f2fd; box-shadow: 0 0 6px rgba(0,123,255,0.3);"
            } else {
                "min-height: 40px; display: flex; align-items: center; justify-content: center; color: #666; font-style: italic;"
            };
            
            html! { 
                <div class="component-two-column" style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px; min-height: 120px; border: 2px dashed #ddd; border-radius: 8px; padding: 16px; background: #f9f9f9;">
                    <div class="column column-1" style="min-height: 80px; border: 1px dashed #ccc; border-radius: 4px; padding: 8px; background: white;">
                        <div class="column-header" style="font-size: 12px; color: #666; margin-bottom: 8px; text-align: center;">{"Column 1"}</div>
                        <div class="column-drop-zone" 
                             style={col1_style}
                             ondragover={on_drag_over_col1}
                             ondragleave={on_drag_leave_col.clone()}
                             ondrop={on_drop_col1}
                        >
                            {if component.properties.column_1_components.is_empty() {
                                html! { <div class="drop-placeholder">{"Drop components here"}</div> }
                            } else {
                                html! {
                                    <div class="nested-components">
                                        {component.properties.column_1_components.iter().map(|nested_comp| {
                                            let is_selected = selected_component.as_ref() == Some(&nested_comp.id);
                                            let nested_on_click = {
                                                let on_component_click = on_component_click.clone();
                                                let component_id = nested_comp.id.clone();
                                                Callback::from(move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    on_component_click.emit(component_id.clone());
                                                })
                                            };
                                            
                                            let border_style = if is_selected {
                                                "2px solid #007bff"
                                            } else {
                                                "1px solid #eee"
                                            };
                                            
                                            let box_shadow_style = if is_selected {
                                                "0 0 0 1px rgba(0, 123, 255, 0.25)"
                                            } else {
                                                "none"
                                            };
                                            
                                            html! {
                                                <div class="nested-component" 
                                                     style={format!("margin: 4px 0; padding: 4px; border: {}; border-radius: 4px; cursor: pointer; position: relative; box-shadow: {}; background: white; pointer-events: auto;", border_style, box_shadow_style)}
                                                     onclick={nested_on_click}>
                                                    {if is_selected {
                                                        let component_id = nested_comp.id.clone();
                                                        let on_edit = {
                                                            let on_component_edit = on_component_edit.clone();
                                                            let component_id = component_id.clone();
                                                            Callback::from(move |_| {
                                                                on_component_edit.emit(component_id.clone());
                                                            })
                                                        };
                                                        let on_duplicate = {
                                                            let on_component_duplicate = on_component_duplicate.clone();
                                                            let component_id = component_id.clone();
                                                            Callback::from(move |_| {
                                                                on_component_duplicate.emit(component_id.clone());
                                                            })
                                                        };
                                                        let on_delete = {
                                                            let on_component_delete = on_component_delete.clone();
                                                            let component_id = component_id.clone();
                                                            Callback::from(move |_| {
                                                                on_component_delete.emit(component_id.clone());
                                                            })
                                                        };
                                                        
                                                        html! {
                                                            <>
                                                                <div class="nested-selection-indicator" style="position: absolute; top: -4px; left: -4px; background: #007bff; color: white; padding: 1px 3px; border-radius: 2px; font-size: 7px; font-weight: 600; z-index: 11;">
                                                                    {"✓"}
                                                                </div>
                                                                <div class="nested-component-controls" style="position: absolute; top: -4px; right: -4px; display: flex; gap: 2px; z-index: 11;">
                                                                    <button class="control-btn" onclick={on_edit} title="Edit Nested Component" style="padding: 2px 4px; background: #6c757d; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"✏️"}</button>
                                                                    <button class="control-btn" onclick={on_duplicate} title="Duplicate Nested Component" style="padding: 2px 4px; background: #17a2b8; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"📋"}</button>
                                                                    <button class="control-btn" onclick={on_delete} title="Delete Nested Component" style="padding: 2px 4px; background: #dc3545; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"🗑️"}</button>
                                                                </div>
                                                            </>
                                                        }
                                                    } else { html! {} }}
                                                    <div style="pointer-events: none;">
                                                    {render_component_content(nested_comp)}
                                                </div>
                                                </div>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                    <div class="column column-2" style="min-height: 80px; border: 1px dashed #ccc; border-radius: 4px; padding: 8px; background: white;">
                        <div class="column-header" style="font-size: 12px; color: #666; margin-bottom: 8px; text-align: center;">{"Column 2"}</div>
                        <div class="column-drop-zone" 
                             style={col2_style}
                             ondragover={on_drag_over_col2}
                             ondragleave={on_drag_leave_col}
                             ondrop={on_drop_col2}
                        >
                            {if component.properties.column_2_components.is_empty() {
                                html! { <div class="drop-placeholder">{"Drop components here"}</div> }
                            } else {
                                html! {
                                    <div class="nested-components">
                                        {component.properties.column_2_components.iter().map(|nested_comp| {
                                            let is_selected = selected_component.as_ref() == Some(&nested_comp.id);
                                            let nested_on_click = {
                                                let on_component_click = on_component_click.clone();
                                                let component_id = nested_comp.id.clone();
                                                Callback::from(move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    on_component_click.emit(component_id.clone());
                                                })
                                            };
                                            
                                            let border_style = if is_selected {
                                                "2px solid #007bff"
                                            } else {
                                                "1px solid #eee"
                                            };
                                            
                                            let box_shadow_style = if is_selected {
                                                "0 0 0 1px rgba(0, 123, 255, 0.25)"
                                            } else {
                                                "none"
                                            };
                                            
                                            html! {
                                                <div class="nested-component" 
                                                     style={format!("margin: 4px 0; padding: 4px; border: {}; border-radius: 4px; cursor: pointer; position: relative; box-shadow: {}; background: white; pointer-events: auto;", border_style, box_shadow_style)}
                                                     onclick={nested_on_click}>
                                                    {if is_selected {
                                                        let component_id = nested_comp.id.clone();
                                                        let on_edit = {
                                                            let on_component_edit = on_component_edit.clone();
                                                            let component_id = component_id.clone();
                                                            Callback::from(move |_| {
                                                                on_component_edit.emit(component_id.clone());
                                                            })
                                                        };
                                                        let on_duplicate = {
                                                            let on_component_duplicate = on_component_duplicate.clone();
                                                            let component_id = component_id.clone();
                                                            Callback::from(move |_| {
                                                                on_component_duplicate.emit(component_id.clone());
                                                            })
                                                        };
                                                        let on_delete = {
                                                            let on_component_delete = on_component_delete.clone();
                                                            let component_id = component_id.clone();
                                                            Callback::from(move |_| {
                                                                on_component_delete.emit(component_id.clone());
                                                            })
                                                        };
                                                        
                                                        html! {
                                                            <>
                                                                <div class="nested-selection-indicator" style="position: absolute; top: -4px; left: -4px; background: #007bff; color: white; padding: 1px 3px; border-radius: 2px; font-size: 7px; font-weight: 600; z-index: 11;">
                                                                    {"✓"}
                                                                </div>
                                                                <div class="nested-component-controls" style="position: absolute; top: -4px; right: -4px; display: flex; gap: 2px; z-index: 11;">
                                                                    <button class="control-btn" onclick={on_edit} title="Edit Nested Component" style="padding: 2px 4px; background: #6c757d; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"✏️"}</button>
                                                                    <button class="control-btn" onclick={on_duplicate} title="Duplicate Nested Component" style="padding: 2px 4px; background: #17a2b8; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"📋"}</button>
                                                                    <button class="control-btn" onclick={on_delete} title="Delete Nested Component" style="padding: 2px 4px; background: #dc3545; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"🗑️"}</button>
                                                                </div>
                                                            </>
                                                        }
                                                    } else { html! {} }}
                                                    <div style="pointer-events: none;">
                                                    {render_component_content(nested_comp)}
                                                </div>
                                                </div>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                </div> 
            }
        }
        ComponentType::ThreeColumn => {
            let container_id = component.id.clone();
            let is_col1_drag_over = (*column_drag_over).as_ref() == Some(&(container_id.clone(), "column-1".to_string()));
            let is_col2_drag_over = (*column_drag_over).as_ref() == Some(&(container_id.clone(), "column-2".to_string()));
            let is_col3_drag_over = (*column_drag_over).as_ref() == Some(&(container_id.clone(), "column-3".to_string()));
            
            let on_drop_col1 = {
                let on_nested_drop = on_nested_drop.clone();
                let container_id = container_id.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    on_nested_drop.emit((container_id.clone(), "column-1".to_string()));
                })
            };
            let on_drop_col2 = {
                let on_nested_drop = on_nested_drop.clone();
                let container_id = container_id.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    on_nested_drop.emit((container_id.clone(), "column-2".to_string()));
                })
            };
            let on_drop_col3 = {
                let on_nested_drop = on_nested_drop.clone();
                let container_id = container_id.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    on_nested_drop.emit((container_id.clone(), "column-3".to_string()));
                })
            };
            let on_drag_over_col1 = {
                let column_drag_over = column_drag_over.clone();
                let container_id = container_id.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    column_drag_over.set(Some((container_id.clone(), "column-1".to_string())));
                })
            };
            
            let on_drag_over_col2 = {
                let column_drag_over = column_drag_over.clone();
                let container_id = container_id.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    column_drag_over.set(Some((container_id.clone(), "column-2".to_string())));
                })
            };
            
            let on_drag_over_col3 = {
                let column_drag_over = column_drag_over.clone();
                let container_id = container_id.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    column_drag_over.set(Some((container_id.clone(), "column-3".to_string())));
                })
            };
            
            let on_drag_leave_col = {
                let column_drag_over = column_drag_over.clone();
                Callback::from(move |e: DragEvent| {
                    e.stop_propagation();
                    column_drag_over.set(None);
                })
            };
            
            let col1_style = if is_col1_drag_over {
                "min-height: 40px; display: flex; align-items: center; justify-content: center; color: #007bff; font-style: italic; font-weight: 500; border: 2px dashed #007bff; border-radius: 4px; padding: 6px; background: #e3f2fd; box-shadow: 0 0 6px rgba(0,123,255,0.3);"
            } else {
                "min-height: 40px; display: flex; align-items: center; justify-content: center; color: #666; font-style: italic;"
            };
            
            let col2_style = if is_col2_drag_over {
                "min-height: 40px; display: flex; align-items: center; justify-content: center; color: #007bff; font-style: italic; font-weight: 500; border: 2px dashed #007bff; border-radius: 4px; padding: 6px; background: #e3f2fd; box-shadow: 0 0 6px rgba(0,123,255,0.3);"
            } else {
                "min-height: 40px; display: flex; align-items: center; justify-content: center; color: #666; font-style: italic;"
            };
            
            let col3_style = if is_col3_drag_over {
                "min-height: 40px; display: flex; align-items: center; justify-content: center; color: #007bff; font-style: italic; font-weight: 500; border: 2px dashed #007bff; border-radius: 4px; padding: 6px; background: #e3f2fd; box-shadow: 0 0 6px rgba(0,123,255,0.3);"
            } else {
                "min-height: 40px; display: flex; align-items: center; justify-content: center; color: #666; font-style: italic;"
            };
            
            html! { 
                <div class="component-three-column" style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px; min-height: 120px; border: 2px dashed #ddd; border-radius: 8px; padding: 16px; background: #f9f9f9;">
                    <div class="column column-1" style="min-height: 80px; border: 1px dashed #ccc; border-radius: 4px; padding: 8px; background: white;">
                        <div class="column-header" style="font-size: 12px; color: #666; margin-bottom: 8px; text-align: center;">{"Column 1"}</div>
                        <div class="column-drop-zone" 
                             style={col1_style}
                             ondragover={on_drag_over_col1}
                             ondragleave={on_drag_leave_col.clone()}
                             ondrop={on_drop_col1}
                        >
                            {if component.properties.column_1_components.is_empty() {
                                html! { <div class="drop-placeholder">{"Drop components here"}</div> }
                            } else {
                                html! {
                                    <div class="nested-components">
                                        {component.properties.column_1_components.iter().map(|nested_comp| {
                                            let is_selected = selected_component.as_ref() == Some(&nested_comp.id);
                                            let nested_on_click = {
                                                let on_component_click = on_component_click.clone();
                                                let component_id = nested_comp.id.clone();
                                                Callback::from(move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    on_component_click.emit(component_id.clone());
                                                })
                                            };
                                            
                                            let border_style = if is_selected {
                                                "2px solid #007bff"
                                            } else {
                                                "1px solid #eee"
                                            };
                                            
                                            let box_shadow_style = if is_selected {
                                                "0 0 0 1px rgba(0, 123, 255, 0.25)"
                                            } else {
                                                "none"
                                            };
                                            
                                            html! {
                                                <div class="nested-component" 
                                                     style={format!("margin: 4px 0; padding: 4px; border: {}; border-radius: 4px; cursor: pointer; position: relative; box-shadow: {}; background: white; pointer-events: auto;", border_style, box_shadow_style)}
                                                     onclick={nested_on_click}>
                                                    {if is_selected {
                                                        let component_id = nested_comp.id.clone();
                                                        let on_edit = {
                                                            let on_component_edit = on_component_edit.clone();
                                                            let component_id = component_id.clone();
                                                            Callback::from(move |_| {
                                                                on_component_edit.emit(component_id.clone());
                                                            })
                                                        };
                                                        let on_duplicate = {
                                                            let on_component_duplicate = on_component_duplicate.clone();
                                                            let component_id = component_id.clone();
                                                            Callback::from(move |_| {
                                                                on_component_duplicate.emit(component_id.clone());
                                                            })
                                                        };
                                                        let on_delete = {
                                                            let on_component_delete = on_component_delete.clone();
                                                            let component_id = component_id.clone();
                                                            Callback::from(move |_| {
                                                                on_component_delete.emit(component_id.clone());
                                                            })
                                                        };
                                                        
                                                        html! {
                                                            <>
                                                                <div class="nested-selection-indicator" style="position: absolute; top: -4px; left: -4px; background: #007bff; color: white; padding: 1px 3px; border-radius: 2px; font-size: 7px; font-weight: 600; z-index: 11;">
                                                                    {"✓"}
                                                                </div>
                                                                <div class="nested-component-controls" style="position: absolute; top: -4px; right: -4px; display: flex; gap: 2px; z-index: 11;">
                                                                    <button class="control-btn" onclick={on_edit} title="Edit Nested Component" style="padding: 2px 4px; background: #6c757d; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"✏️"}</button>
                                                                    <button class="control-btn" onclick={on_duplicate} title="Duplicate Nested Component" style="padding: 2px 4px; background: #17a2b8; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"📋"}</button>
                                                                    <button class="control-btn" onclick={on_delete} title="Delete Nested Component" style="padding: 2px 4px; background: #dc3545; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"🗑️"}</button>
                                                                </div>
                                                            </>
                                                        }
                                                    } else { html! {} }}
                                                    <div style="pointer-events: none;">
                                                    {render_component_content(nested_comp)}
                                                </div>
                                                </div>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                    <div class="column column-2" style="min-height: 80px; border: 1px dashed #ccc; border-radius: 4px; padding: 8px; background: white;">
                        <div class="column-header" style="font-size: 12px; color: #666; margin-bottom: 8px; text-align: center;">{"Column 2"}</div>
                        <div class="column-drop-zone" 
                             style={col2_style}
                             ondragover={on_drag_over_col2}
                             ondragleave={on_drag_leave_col.clone()}
                             ondrop={on_drop_col2}
                        >
                            {if component.properties.column_2_components.is_empty() {
                                html! { <div class="drop-placeholder">{"Drop components here"}</div> }
                            } else {
                                html! {
                                    <div class="nested-components">
                                        {component.properties.column_2_components.iter().map(|nested_comp| {
                                            let is_selected = selected_component.as_ref() == Some(&nested_comp.id);
                                            let nested_on_click = {
                                                let on_component_click = on_component_click.clone();
                                                let component_id = nested_comp.id.clone();
                                                Callback::from(move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    on_component_click.emit(component_id.clone());
                                                })
                                            };
                                            
                                            let border_style = if is_selected {
                                                "2px solid #007bff"
                                            } else {
                                                "1px solid #eee"
                                            };
                                            
                                            let box_shadow_style = if is_selected {
                                                "0 0 0 1px rgba(0, 123, 255, 0.25)"
                                            } else {
                                                "none"
                                            };
                                            
                                            html! {
                                                <div class="nested-component" 
                                                     style={format!("margin: 4px 0; padding: 4px; border: {}; border-radius: 4px; cursor: pointer; position: relative; box-shadow: {}; background: white; pointer-events: auto;", border_style, box_shadow_style)}
                                                     onclick={nested_on_click}>
                                                    {if is_selected {
                                                        let component_id = nested_comp.id.clone();
                                                        let on_edit = {
                                                            let on_component_edit = on_component_edit.clone();
                                                            let component_id = component_id.clone();
                                                            Callback::from(move |_| {
                                                                on_component_edit.emit(component_id.clone());
                                                            })
                                                        };
                                                        let on_duplicate = {
                                                            let on_component_duplicate = on_component_duplicate.clone();
                                                            let component_id = component_id.clone();
                                                            Callback::from(move |_| {
                                                                on_component_duplicate.emit(component_id.clone());
                                                            })
                                                        };
                                                        let on_delete = {
                                                            let on_component_delete = on_component_delete.clone();
                                                            let component_id = component_id.clone();
                                                            Callback::from(move |_| {
                                                                on_component_delete.emit(component_id.clone());
                                                            })
                                                        };
                                                        
                                                        html! {
                                                            <>
                                                                <div class="nested-selection-indicator" style="position: absolute; top: -4px; left: -4px; background: #007bff; color: white; padding: 1px 3px; border-radius: 2px; font-size: 7px; font-weight: 600; z-index: 11;">
                                                                    {"✓"}
                                                                </div>
                                                                <div class="nested-component-controls" style="position: absolute; top: -4px; right: -4px; display: flex; gap: 2px; z-index: 11;">
                                                                    <button class="control-btn" onclick={on_edit} title="Edit Nested Component" style="padding: 2px 4px; background: #6c757d; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"✏️"}</button>
                                                                    <button class="control-btn" onclick={on_duplicate} title="Duplicate Nested Component" style="padding: 2px 4px; background: #17a2b8; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"📋"}</button>
                                                                    <button class="control-btn" onclick={on_delete} title="Delete Nested Component" style="padding: 2px 4px; background: #dc3545; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"🗑️"}</button>
                                                                </div>
                                                            </>
                                                        }
                                                    } else { html! {} }}
                                                    <div style="pointer-events: none;">
                                                    {render_component_content(nested_comp)}
                                                </div>
                                                </div>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                    <div class="column column-3" style="min-height: 80px; border: 1px dashed #ccc; border-radius: 4px; padding: 8px; background: white;">
                        <div class="column-header" style="font-size: 12px; color: #666; margin-bottom: 8px; text-align: center;">{"Column 3"}</div>
                        <div class="column-drop-zone" 
                             style={col3_style}
                             ondragover={on_drag_over_col3}
                             ondragleave={on_drag_leave_col}
                             ondrop={on_drop_col3}
                        >
                            {if component.properties.column_3_components.is_empty() {
                                html! { <div class="drop-placeholder">{"Drop components here"}</div> }
                            } else {
                                html! {
                                    <div class="nested-components">
                                        {component.properties.column_3_components.iter().map(|nested_comp| {
                                            let is_selected = selected_component.as_ref() == Some(&nested_comp.id);
                                            let nested_on_click = {
                                                let on_component_click = on_component_click.clone();
                                                let component_id = nested_comp.id.clone();
                                                Callback::from(move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    on_component_click.emit(component_id.clone());
                                                })
                                            };
                                            
                                            let border_style = if is_selected {
                                                "2px solid #007bff"
                                            } else {
                                                "1px solid #eee"
                                            };
                                            
                                            let box_shadow_style = if is_selected {
                                                "0 0 0 1px rgba(0, 123, 255, 0.25)"
                                            } else {
                                                "none"
                                            };
                                            
                                            html! {
                                                <div class="nested-component" 
                                                     style={format!("margin: 4px 0; padding: 4px; border: {}; border-radius: 4px; cursor: pointer; position: relative; box-shadow: {}; background: white; pointer-events: auto;", border_style, box_shadow_style)}
                                                     onclick={nested_on_click}>
                                                    {if is_selected {
                                                        let component_id = nested_comp.id.clone();
                                                        let on_edit = {
                                                            let on_component_edit = on_component_edit.clone();
                                                            let component_id = component_id.clone();
                                                            Callback::from(move |_| {
                                                                on_component_edit.emit(component_id.clone());
                                                            })
                                                        };
                                                        let on_duplicate = {
                                                            let on_component_duplicate = on_component_duplicate.clone();
                                                            let component_id = component_id.clone();
                                                            Callback::from(move |_| {
                                                                on_component_duplicate.emit(component_id.clone());
                                                            })
                                                        };
                                                        let on_delete = {
                                                            let on_component_delete = on_component_delete.clone();
                                                            let component_id = component_id.clone();
                                                            Callback::from(move |_| {
                                                                on_component_delete.emit(component_id.clone());
                                                            })
                                                        };
                                                        
                                                        html! {
                                                            <>
                                                                <div class="nested-selection-indicator" style="position: absolute; top: -4px; left: -4px; background: #007bff; color: white; padding: 1px 3px; border-radius: 2px; font-size: 7px; font-weight: 600; z-index: 11;">
                                                                    {"✓"}
                                                                </div>
                                                                <div class="nested-component-controls" style="position: absolute; top: -4px; right: -4px; display: flex; gap: 2px; z-index: 11;">
                                                                    <button class="control-btn" onclick={on_edit} title="Edit Nested Component" style="padding: 2px 4px; background: #6c757d; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"✏️"}</button>
                                                                    <button class="control-btn" onclick={on_duplicate} title="Duplicate Nested Component" style="padding: 2px 4px; background: #17a2b8; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"📋"}</button>
                                                                    <button class="control-btn" onclick={on_delete} title="Delete Nested Component" style="padding: 2px 4px; background: #dc3545; color: white; border: none; border-radius: 2px; cursor: pointer; font-size: 8px;">{"🗑️"}</button>
                                                                </div>
                                                            </>
                                                        }
                                                    } else { html! {} }}
                                                    <div style="pointer-events: none;">
                                                    {render_component_content(nested_comp)}
                                                </div>
                                                </div>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                </div> 
            }
        }
        // For all other component types, use the original rendering function
        _ => render_component_content(component)
    }
}

// Utility functions for video component
fn convert_to_embed_url(url: &str) -> String {
    if url.contains("youtube.com/watch?v=") {
        // Convert YouTube watch URL to embed URL
        if let Some(video_id) = url.split("v=").nth(1) {
            let video_id = video_id.split('&').next().unwrap_or(video_id);
            return format!("https://www.youtube.com/embed/{}", video_id);
        }
    } else if url.contains("youtu.be/") {
        // Convert YouTube short URL to embed URL
        if let Some(video_id) = url.split("youtu.be/").nth(1) {
            let video_id = video_id.split('?').next().unwrap_or(video_id);
            return format!("https://www.youtube.com/embed/{}", video_id);
        }
    } else if url.contains("vimeo.com/") {
        // Convert Vimeo URL to embed URL
        if let Some(video_id) = url.split("vimeo.com/").nth(1) {
            let video_id = video_id.split('?').next().unwrap_or(video_id);
            return format!("https://player.vimeo.com/video/{}", video_id);
        }
    }
    url.to_string()
}

fn get_video_type(url: &str) -> &str {
    if url.ends_with(".mp4") {
        "mp4"
    } else if url.ends_with(".webm") {
        "webm"
    } else if url.ends_with(".ogg") {
        "ogg"
    } else {
        "mp4" // default
    }
}