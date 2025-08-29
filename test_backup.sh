#!/bin/bash

# Test backup functionality directly
echo "🧪 Testing backup functionality..."

# Set environment variables
export DATABASE_URL="postgres://rustcms:password@localhost:5432/my_rust_cms"
export BACKUP_DIR="./backups"

# Create backup directory if it doesn't exist
mkdir -p ./backups

echo "📁 Backup directory: $BACKUP_DIR"
echo "🗄️  Database URL: $DATABASE_URL"

# Test database backup
echo ""
echo "🔄 Testing database backup..."
cd backend
cargo run --bin test_backup_tool database "Test database backup"

echo ""
echo "🔄 Testing media backup..."
cargo run --bin test_backup_tool media "Test media backup"

echo ""
echo "🔄 Testing full backup..."
cargo run --bin test_backup_tool full "Test full backup"

echo ""
echo "📋 Listing backups..."
ls -la ../backups/

echo ""
echo "✅ Backup tests completed!"
