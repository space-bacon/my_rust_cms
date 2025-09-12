# Default CMS Setup

This document describes the default data and configuration that comes with My Rust CMS.

## 🎯 **What's Included**

Your Rust CMS now comes pre-configured with a complete, production-ready setup based on your current customizations:

### 🎨 **Visual Design**
- **Header Template**: Fixed header with gradient background (#667eea to #764ba2)
- **Shape Masks**: Lower tilt mask for modern design
- **Scroll Effects**: Shrink effect with smooth animations
- **Typography**: Pixelify Sans font family
- **Live Edit Mode**: Fully functional with all current improvements

### 📝 **Content**
- **4 Sample Posts**: Including welcome content and guides
- **3 Pages**: Home page with hero section, Posts page, and additional content
- **Navigation**: Header navigation with Home and Posts links
- **Categories**: General category for organizing content

### ⚙️ **Settings**
- **47 Preconfigured Settings**: Including typography, container, and site settings
- **Admin Interface**: Ready-to-use admin dashboard
- **User Management**: Admin user (username: admin, password: admin)

## 🚀 **How It Works**

### **New Installation Process**

When someone installs your CMS:

1. **Database Migration**: The `2025-09-12-094741_insert_default_cms_data` migration runs automatically
2. **Default Data Seeder**: `backend/src/services/default_data_seeder.rs` populates the database
3. **Ready to Use**: The CMS starts with all your current customizations

### **Key Files**

- `backend/src/services/default_data_seeder.rs` - Main seeder with all default data
- `migrations/2025-09-12-094741_insert_default_cms_data/` - Database migration
- `default_data_export/` - Raw exported data (for reference)
- `scripts/export_via_api.py` - Export script for future updates

## 🔧 **Technical Details**

### **Component Templates**
- **Fixed Header**: ID 2, includes gradient, tilt mask, shrink effects
- **Default Footer**: Simple footer template
- **All Current Customizations**: Height settings, colors, animations

### **Database Structure**
```sql
-- Key default data includes:
- Users: Admin user with proper authentication
- Categories: General category for posts
- Navigation: Header navigation items
- Settings: 47 settings including typography and design
- Posts: 4 sample posts with content
- Pages: Home and Posts pages with components
- Component Templates: Header and footer templates with your styling
```

### **Settings Included**
- Site title and description
- Typography settings (Pixelify Sans, 16px, 1.6 line height)
- Container settings (white background, black border)
- Admin and login button visibility
- All live edit customizations

## 🎯 **For New Users**

When someone installs your CMS, they get:

1. **Complete Working Site**: Ready-to-use with modern design
2. **Sample Content**: Posts and pages to demonstrate functionality
3. **Admin Access**: Can immediately login and start customizing
4. **Live Edit Mode**: Fully functional with all improvements
5. **Professional Styling**: Gradient headers, animations, responsive design

## 🔄 **Updating Default Data**

To update the default data with new customizations:

1. Make your changes in the CMS
2. Run `python3 scripts/export_via_api.py` to export current state
3. Update `backend/src/services/default_data_seeder.rs` with new data
4. Create a new migration if needed
5. Test on a fresh database

## 📊 **Current Statistics**

- **Posts**: 4 sample posts
- **Pages**: 3 pages (Home, Posts, Why My Rust CMS)
- **Settings**: 47 configured settings
- **Templates**: 8 component templates
- **Navigation**: 2 header navigation items
- **Categories**: 1 general category

Your CMS is now packaged as a complete, professional content management system with all your customizations as the default experience!
