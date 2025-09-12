#!/bin/bash

# Export current database state as default setup
# This script exports all current data and creates migration files for default data

set -e

echo "🚀 Exporting current CMS state as default setup..."

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Create export directory
EXPORT_DIR="./default_data_export"
mkdir -p "$EXPORT_DIR"

echo "📊 Exporting current database data..."

# Export all tables with data
pg_dump "$DATABASE_URL" \
    --data-only \
    --inserts \
    --no-owner \
    --no-privileges \
    --file="$EXPORT_DIR/current_data.sql"

echo "📋 Exporting individual table data..."

# Export specific tables separately for easier management
tables=("users" "categories" "posts" "pages" "navigation" "settings" "templates" "components" "page_components" "plugins")

for table in "${tables[@]}"; do
    echo "  Exporting $table..."
    pg_dump "$DATABASE_URL" \
        --data-only \
        --inserts \
        --no-owner \
        --no-privileges \
        --table="$table" \
        --file="$EXPORT_DIR/${table}_data.sql"
done

echo "🎨 Exporting component templates..."
pg_dump "$DATABASE_URL" \
    --data-only \
    --inserts \
    --no-owner \
    --no-privileges \
    --table="component_templates" \
    --file="$EXPORT_DIR/component_templates_data.sql"

echo "📝 Creating default data migration..."

# Create timestamp for new migration
TIMESTAMP=$(date +"%Y-%m-%d-%H%M%S")
MIGRATION_DIR="./migrations/${TIMESTAMP}_insert_default_data"
mkdir -p "$MIGRATION_DIR"

# Create up.sql with all the data
cat > "$MIGRATION_DIR/up.sql" << 'EOF'
-- Insert default data for My Rust CMS
-- This includes all current customizations, posts, and settings

-- First, clear any existing data (in case of re-run)
DELETE FROM page_components;
DELETE FROM component_templates;
DELETE FROM pages WHERE slug != 'home';  -- Keep home page structure
DELETE FROM posts;
DELETE FROM navigation;
DELETE FROM settings;
DELETE FROM categories;
DELETE FROM plugins;

-- Insert default categories
EOF

# Append the exported data to the migration
echo "-- Default Categories" >> "$MIGRATION_DIR/up.sql"
cat "$EXPORT_DIR/categories_data.sql" >> "$MIGRATION_DIR/up.sql"

echo "" >> "$MIGRATION_DIR/up.sql"
echo "-- Default Settings" >> "$MIGRATION_DIR/up.sql"
cat "$EXPORT_DIR/settings_data.sql" >> "$MIGRATION_DIR/up.sql"

echo "" >> "$MIGRATION_DIR/up.sql"
echo "-- Default Navigation" >> "$MIGRATION_DIR/up.sql"
cat "$EXPORT_DIR/navigation_data.sql" >> "$MIGRATION_DIR/up.sql"

echo "" >> "$MIGRATION_DIR/up.sql"
echo "-- Default Component Templates" >> "$MIGRATION_DIR/up.sql"
cat "$EXPORT_DIR/component_templates_data.sql" >> "$MIGRATION_DIR/up.sql"

echo "" >> "$MIGRATION_DIR/up.sql"
echo "-- Default Posts" >> "$MIGRATION_DIR/up.sql"
cat "$EXPORT_DIR/posts_data.sql" >> "$MIGRATION_DIR/up.sql"

echo "" >> "$MIGRATION_DIR/up.sql"
echo "-- Default Pages" >> "$MIGRATION_DIR/up.sql"
cat "$EXPORT_DIR/pages_data.sql" >> "$MIGRATION_DIR/up.sql"

echo "" >> "$MIGRATION_DIR/up.sql"
echo "-- Default Page Components" >> "$MIGRATION_DIR/up.sql"
cat "$EXPORT_DIR/page_components_data.sql" >> "$MIGRATION_DIR/up.sql"

echo "" >> "$MIGRATION_DIR/up.sql"
echo "-- Default Plugins" >> "$MIGRATION_DIR/up.sql"
cat "$EXPORT_DIR/plugins_data.sql" >> "$MIGRATION_DIR/up.sql"

# Create down.sql
cat > "$MIGRATION_DIR/down.sql" << 'EOF'
-- Remove default data
DELETE FROM page_components;
DELETE FROM component_templates;
DELETE FROM posts;
DELETE FROM pages WHERE slug != 'home';
DELETE FROM navigation;
DELETE FROM settings;
DELETE FROM categories;
DELETE FROM plugins;
EOF

echo "🔧 Updating main.rs to use new default data..."

# Backup current main.rs
cp backend/src/main.rs backend/src/main.rs.backup

echo "✅ Export completed!"
echo ""
echo "📁 Files created:"
echo "  - $EXPORT_DIR/ (raw SQL exports)"
echo "  - $MIGRATION_DIR/ (new migration)"
echo "  - backend/src/main.rs.backup (backup)"
echo ""
echo "🎯 Next steps:"
echo "  1. Review the generated migration in $MIGRATION_DIR/"
echo "  2. Update backend/src/main.rs to remove old demo data creation"
echo "  3. Test the migration on a fresh database"
echo "  4. Commit the changes to make them the new default"
