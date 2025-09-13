-- Update the Fixed Header template with the current logo settings
-- This ensures new installations get the image logo by default

UPDATE component_templates 
SET template_data = '{
  "animation_type": "fade-in",
  "background": "inherit", 
  "bg_gradient_direction": "to-right",
  "bg_type": "gradient",
  "height": "447px",
  "logo_effect": "none",
  "logo_height": "425px",
  "logo_pulsate_anim_frequency": "1",
  "logo_pulsate_decay": "0.6",
  "logo_pulsate_duration": "9", 
  "logo_pulsate_frequency": "1.2",
  "logo_pulsate_opacity": "11",
  "logo_type": "image",
  "logo_url": "http://localhost:8081/uploads/default-logo.svg",
  "nav_underline_animation": "none",
  "padding": "1rem 0",
  "position": "sticky",
  "scroll_effect": "shrink",
  "shape_mask_lower": "tilt",
  "shape_mask_lower_degrees": "22",
  "shape_mask_upper": "none", 
  "shrink_height": "205",
  "shrink_logo_scale": "72",
  "site_title_color": "#ffffff"
}'::jsonb
WHERE name = 'Fixed Header' AND component_type = 'header';