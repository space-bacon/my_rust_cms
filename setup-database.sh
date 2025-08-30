#!/bin/bash

# =============================================================================
# RUST CMS DATABASE SETUP SCRIPT
# =============================================================================
# This script ensures the correct Docker database setup is always used.
# Run this script before starting development to prevent database mishaps.
# =============================================================================

set -e  # Exit on any error

echo "🚀 Setting up Rust CMS Database Environment..."
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    print_error "Docker is not running. Please start Docker Desktop first."
    exit 1
fi

print_status "Docker is running"

# Check if PostgreSQL container exists and is running
if ! docker ps | grep -q "rustcms_dev_postgres"; then
    print_warning "PostgreSQL container is not running"
    
    # Check if container exists but is stopped
    if docker ps -a | grep -q "rustcms_dev_postgres"; then
        print_info "Starting existing PostgreSQL container..."
        docker start rustcms_dev_postgres
    else
        print_error "PostgreSQL container does not exist. Please run 'docker-compose up -d' first."
        exit 1
    fi
else
    print_status "PostgreSQL container is running"
fi

# Stop any local PostgreSQL services that might conflict
if brew services list | grep -q "postgresql.*started"; then
    print_warning "Stopping local PostgreSQL services to prevent conflicts..."
    brew services stop postgresql@14 2>/dev/null || true
    brew services stop postgresql@15 2>/dev/null || true
    print_status "Local PostgreSQL services stopped"
fi

# Copy the correct environment configuration
print_info "Setting up environment configuration..."
cp .env.docker .env
print_status "Environment configuration updated (.env.docker → .env)"

# Verify database connection
print_info "Testing database connection..."
if docker exec rustcms_dev_postgres psql -U rustcms -d my_rust_cms -c "SELECT COUNT(*) FROM settings;" > /dev/null 2>&1; then
    SETTINGS_COUNT=$(docker exec rustcms_dev_postgres psql -U rustcms -d my_rust_cms -t -c "SELECT COUNT(*) FROM settings;" | xargs)
    TYPOGRAPHY_COUNT=$(docker exec rustcms_dev_postgres psql -U rustcms -d my_rust_cms -t -c "SELECT COUNT(*) FROM settings WHERE setting_type = 'typography';" | xargs)
    
    print_status "Database connection successful"
    print_info "Total settings: $SETTINGS_COUNT"
    print_info "Typography settings: $TYPOGRAPHY_COUNT"
else
    print_error "Failed to connect to database"
    exit 1
fi

echo ""
echo "🎉 Database setup complete!"
echo ""
print_info "Next steps:"
echo "  1. cd backend && cargo run    # Start the backend"
echo "  2. cd frontend && trunk serve # Start the frontend"
echo ""
print_warning "Always run this script before development to ensure correct database setup!"
