pub mod header;
pub mod sidebar;
pub mod notification;
pub mod simple_notification;
pub mod simple_component_editor;
pub mod unified_properties_panel;
pub mod enhanced_live_edit_system;
mod public_layout;
mod media_picker;
mod enhanced_gallery;
mod modern_menu_designer;
mod live_edit_mode;
// mod hamburger_menu; // Temporarily removed
pub mod admin;
pub mod auth_guard;
pub mod posts_list_widget;
pub mod markdown_editor;
pub mod page_builder;
pub mod performance_monitor;
pub mod comment_item;
pub mod comments_section;
pub mod plugin_modal;

// Export essential components that are used across the app
pub use sidebar::ActiveTab;
pub use public_layout::PublicLayout;
pub use enhanced_live_edit_system::EnhancedLiveEditSystem;
pub use posts_list_widget::PostsListWidget;
pub use auth_guard::AdminGuard;
// pub use hamburger_menu::HamburgerMenu; // Will be used when integrated
pub use performance_monitor::PerformanceMonitor;
pub use media_picker::MediaPicker;
pub use enhanced_gallery::{EnhancedGallery, EnhancedGalleryImage};
pub use modern_menu_designer::{ModernMenuDesigner, MenuStyle, MenuProperties};
// pub use comment_item::CommentItem; // Used internally by CommentsSection
pub use comments_section::CommentsSection;
pub use plugin_modal::PluginModal;
