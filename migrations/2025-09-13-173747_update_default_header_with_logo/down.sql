-- Revert the Fixed Header template to the previous text logo settings

UPDATE component_templates 
SET template_data = '{
  "animation_type": "fade-in",
  "background": "inherit",
  "bg_gradient_direction": "to-right", 
  "bg_type": "gradient",
  "height": "429px",
  "padding": "1rem 0",
  "position": "sticky",
  "scroll_effect": "shrink",
  "shape_mask_lower": "tilt",
  "shape_mask_lower_degrees": "22",
  "shape_mask_upper": "none",
  "shrink_height": "135", 
  "shrink_logo_scale": "70",
  "site_title_color": "#ffffff"
}'::jsonb
WHERE name = 'Fixed Header' AND component_type = 'header';