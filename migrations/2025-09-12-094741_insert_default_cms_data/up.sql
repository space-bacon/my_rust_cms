-- Insert default CMS data based on current state
-- This migration sets up the default data for new installations

-- Note: This migration is designed to be idempotent
-- It will only insert data if it doesn't already exist

-- Default categories (only if none exist)
INSERT INTO categories (name) 
SELECT 'General' 
WHERE NOT EXISTS (SELECT 1 FROM categories);

-- Default site settings (only if they don't exist)
INSERT INTO settings (setting_key, setting_value, setting_type, description)
SELECT * FROM (VALUES
    ('admin_button_visible', 'true', 'site', 'Show admin button in navigation'),
    ('login_button_visible', 'true', 'site', 'Show login button in navigation'),
    ('site_title', 'My Rust CMS', 'site', 'Site title displayed in header'),
    ('site_description', 'Modern Content Management System built with Rust', 'site', 'Site description for SEO'),
    ('typography_font_family', 'pixelify-sans', 'typography', 'Primary font family'),
    ('typography_font_size', '16', 'typography', 'Base font size in pixels'),
    ('typography_line_height', '1.6', 'typography', 'Base line height'),
    ('typography_font_weight', '400', 'typography', 'Base font weight'),
    ('container_background_type', 'none', 'container', 'Container background type'),
    ('container_background_color', '#ffffff', 'container', 'Container background color'),
    ('container_border_color', '#000000', 'container', 'Container border color'),
    ('container_animation', 'none', 'container', 'Container animation type')
) AS new_settings(setting_key, setting_value, setting_type, description)
WHERE NOT EXISTS (
    SELECT 1 FROM settings s 
    WHERE s.setting_key = new_settings.setting_key
);

-- Default navigation items (only if none exist)
INSERT INTO navigation (title, url, order_position, is_active, menu_area, icon, target, mobile_visible, description)
SELECT * FROM (VALUES
    ('Home', '/', 1, true, 'header', 'home', '_self', true, 'Homepage link'),
    ('Posts', '/posts', 2, true, 'header', 'article', '_self', true, 'View all posts')
) AS new_nav(title, url, order_position, is_active, menu_area, icon, target, mobile_visible, description)
WHERE NOT EXISTS (SELECT 1 FROM navigation);

-- Default component templates (only if none exist)
INSERT INTO component_templates (name, component_type, template_data, breakpoints, is_default, is_active)
SELECT * FROM (VALUES
    ('Fixed Header', 'header', 
     '{"bg_color": "#66f0a2", "bg_gradient_direction": "to right", "bg_gradient_end": "#764ba2", "bg_gradient_start": "#667eea", "bg_type": "gradient", "background": "inherit", "border": "none", "effects": "none", "height": "429px", "logo_font_size": "1.5rem", "logo_text": "My Site", "logo_type": "text", "nav_underline_animation": "slide", "padding": "1rem 0", "position": "sticky", "scroll_effect": "shrink", "scroll_trigger": "100", "shape_mask_lower": "tilt", "shape_mask_lower_amplitude": "20", "shape_mask_lower_degrees": "2", "shape_mask_lower_direction": "left", "shape_mask_lower_frequency": "1.5", "shape_mask_lower_scale": "100", "shape_mask_upper": "none", "shape_mask_upper_amplitude": "20", "shape_mask_upper_degrees": "0", "shape_mask_upper_direction": "right", "shape_mask_upper_frequency": "1", "shape_mask_upper_scale": "100", "shrink_height": "135", "shrink_logo_scale": "70", "site_title_color": "#ffffff", "text_color": "#db3d3d"}'::jsonb,
     '{}'::jsonb, true, true),
    ('Default Footer', 'footer',
     '{"background": "inherit", "padding": "2rem 0", "border": "none"}'::jsonb,
     '{}'::jsonb, true, true)
) AS new_templates(name, component_type, template_data, breakpoints, is_default, is_active)
WHERE NOT EXISTS (SELECT 1 FROM component_templates);

-- Default posts (only if none exist)
INSERT INTO posts (title, content, category_id, user_id)
SELECT * FROM (VALUES
    ('Welcome to My Rust CMS', 'Welcome to your new Rust-powered content management system! This CMS is built for performance, security, and scalability.', 1, 1),
    ('Getting Started Guide', 'Learn how to customize your CMS, create content, and manage your website with this comprehensive getting started guide.', 1, 1),
    ('Modern Content Management', 'Experience the power of modern web technologies with Rust on the backend and cutting-edge frontend frameworks.', 1, 1),
    ('Thank you for your attention on this matter', 'Thank you for your attention on this matter', 1, 1)
) AS new_posts(title, content, category_id, user_id)
WHERE NOT EXISTS (SELECT 1 FROM posts);

-- Default pages (only if none exist)
INSERT INTO pages (title, content, slug, status, user_id)
SELECT * FROM (VALUES
    ('Home', '[{"id": "hero-section", "component_type": "hero", "content": "# Modern Content Management\n\nBuilt for Performance and Scalability", "properties": {"background_type": "gradient", "background_gradient_start": "#667eea", "background_gradient_end": "#764ba2", "text_align": "center", "padding": "4rem 2rem"}}, {"id": "features-section", "component_type": "text", "content": "## Built for Performance and Scalability\n\nOur Rust-powered CMS delivers exceptional speed and reliability.", "properties": {"text_align": "center", "padding": "2rem"}}, {"id": "cta-section", "component_type": "text", "content": "Thank you for your attention on this matter", "properties": {"text_align": "center", "padding": "2rem"}}]', 'home', 'published', 1),
    ('Posts', '', 'posts', 'published', 1)
) AS new_pages(title, content, slug, status, user_id)
WHERE NOT EXISTS (SELECT 1 FROM pages WHERE slug IN ('home', 'posts'));
