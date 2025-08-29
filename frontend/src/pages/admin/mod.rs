pub mod admin;
pub mod dashboard;
pub mod post_list;
pub mod post_editor;
pub mod user_management;
pub mod enhanced_user_management;
pub mod comment_moderation;
pub mod media_library;
pub mod page_builder;
pub mod system_settings;
pub mod navigation_manager;
pub mod template_manager;
pub mod analytics;
pub mod design_system;
pub mod plugin_manager;

// Keeping all admin page exports available for future use
#[allow(unused_imports)]
pub use admin::Admin;
#[allow(unused_imports)]
pub use dashboard::AdminDashboard;
#[allow(unused_imports)]
pub use post_list::PostList;
pub use post_editor::PostEditor;
#[allow(unused_imports)]
pub use user_management::UserManagement;
pub use enhanced_user_management::EnhancedUserManagement;
#[allow(unused_imports)]
pub use comment_moderation::CommentModeration;
#[allow(unused_imports)]
pub use media_library::MediaLibrary;
#[allow(unused_imports)]
pub use page_builder::PageBuilder;
#[allow(unused_imports)]
pub use system_settings::SystemSettings;
#[allow(unused_imports)]
pub use navigation_manager::NavigationManager;
#[allow(unused_imports)]
pub use analytics::Analytics;
#[allow(unused_imports)]
pub use design_system::DesignSystemPage;
#[allow(unused_imports)]
pub use plugin_manager::PluginManager; 