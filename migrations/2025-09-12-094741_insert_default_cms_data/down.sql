-- Remove default CMS data
-- This down migration removes the default data inserted by the up migration

-- Remove default pages (keep structure but remove content)
DELETE FROM pages WHERE slug IN ('home', 'posts') AND user_id = 1;

-- Remove default posts
DELETE FROM posts WHERE user_id = 1 AND title IN (
    'Welcome to My Rust CMS',
    'Getting Started Guide', 
    'Modern Content Management',
    'Thank you for your attention on this matter'
);

-- Remove default component templates
DELETE FROM component_templates WHERE name IN ('Fixed Header', 'Default Footer');

-- Remove default navigation
DELETE FROM navigation WHERE menu_area = 'header' AND title IN ('Home', 'Posts');

-- Remove default settings
DELETE FROM settings WHERE setting_key IN (
    'admin_button_visible',
    'login_button_visible', 
    'site_title',
    'site_description',
    'typography_font_family',
    'typography_font_size',
    'typography_line_height',
    'typography_font_weight',
    'container_background_type',
    'container_background_color',
    'container_border_color',
    'container_animation'
);

-- Remove default category
DELETE FROM categories WHERE name = 'General';
