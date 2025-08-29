#!/bin/bash

echo "🧪 Testing basic backup functionality..."

# Set up environment
export DATABASE_URL="postgres://rustcms:password@localhost:5432/my_rust_cms"
export BACKUP_DIR="./backups"

# Create backup directory
mkdir -p ./backups

echo "📁 Backup directory: $BACKUP_DIR"
echo "🗄️  Database URL: $DATABASE_URL"

# Test media backup
echo ""
echo "🔄 Testing media backup..."

# Create some test media files if they don't exist
mkdir -p uploads
echo "Test file 1" > uploads/test1.txt
echo "Test file 2" > uploads/test2.txt

# Create media backup
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
MEDIA_BACKUP="media_backup_test_${TIMESTAMP}.tar.gz"

echo "   Creating archive: $MEDIA_BACKUP"

if tar -czf "./backups/$MEDIA_BACKUP" -C . uploads 2>/dev/null; then
    echo "✅ Media backup created successfully!"
    
    # Check file size
    SIZE=$(stat -f%z "./backups/$MEDIA_BACKUP" 2>/dev/null || stat -c%s "./backups/$MEDIA_BACKUP" 2>/dev/null)
    echo "   Size: $SIZE bytes"
    
    # List contents
    echo "   Contents:"
    tar -tzf "./backups/$MEDIA_BACKUP" | head -5
else
    echo "❌ Media backup failed"
fi

# Test database backup (if pg_dump is available)
echo ""
echo "🔄 Testing database backup..."

if command -v pg_dump >/dev/null 2>&1; then
    DB_BACKUP="db_backup_test_${TIMESTAMP}.sql"
    echo "   Creating database dump: $DB_BACKUP"
    
    # Parse database URL
    DB_HOST="localhost"
    DB_PORT="5432"
    DB_USER="rustcms"
    DB_NAME="my_rust_cms"
    DB_PASS="password"
    
    # Try to create database backup
    if PGPASSWORD="$DB_PASS" pg_dump -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" --clean --create -f "./backups/$DB_BACKUP" 2>/dev/null; then
        echo "✅ Database backup created successfully!"
        
        # Check file size
        SIZE=$(stat -f%z "./backups/$DB_BACKUP" 2>/dev/null || stat -c%s "./backups/$DB_BACKUP" 2>/dev/null)
        echo "   Size: $SIZE bytes"
        
        # Show first few lines
        echo "   First few lines:"
        head -5 "./backups/$DB_BACKUP"
    else
        echo "⚠️  Database backup failed (database may not be accessible)"
        echo "   This is expected if the database is not running or credentials are incorrect"
    fi
else
    echo "⚠️  pg_dump not found - skipping database backup test"
    echo "   Install PostgreSQL client tools to test database backups"
fi

# List all backups
echo ""
echo "📋 Current backups:"
ls -la ./backups/ | grep -E "\.(tar\.gz|sql|pgdump)$" || echo "   No backup files found"

echo ""
echo "✅ Basic backup tests completed!"
echo ""
echo "💡 To test the full backup system:"
echo "   1. Start the database: ./dev-start.sh"
echo "   2. Test via API: curl -X POST http://localhost:8081/api/system/backup -H 'Content-Type: application/json' -d '{\"backup_type\": \"media\", \"description\": \"Test backup\"}'"
