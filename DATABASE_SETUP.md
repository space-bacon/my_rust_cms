# 🗄️ Database Setup Guide

## Overview

This Rust CMS project uses **Docker PostgreSQL** as the primary database. This guide ensures you always connect to the correct database and prevents data loss.

## ⚠️ IMPORTANT: Database Configuration

**ALWAYS USE THE DOCKER DATABASE** - This contains all your data, settings, and typography configurations.

### Database Details
- **Container**: `rustcms_dev_postgres`
- **Host**: `localhost:5432`
- **Database**: `my_rust_cms`
- **User**: `myrustcms`
- **Password**: `password`

## 🚀 Quick Setup

### Option 1: Automated Setup (Recommended)
```bash
./setup-database.sh
```

### Option 2: Manual Setup
```bash
# 1. Ensure Docker is running
docker info

# 2. Start PostgreSQL container
docker-compose up -d postgres

# 3. Copy correct environment
cp .env.docker .env

# 4. Stop any conflicting local PostgreSQL
brew services stop postgresql@15
brew services stop postgresql@14
```

## 🔧 Development Workflow

### Starting Development
```bash
# Use the development script (includes database setup)
./dev-start.sh
```

### Manual Backend Start
```bash
# 1. Setup database first
./setup-database.sh

# 2. Start backend
cd backend
cargo run
```

## 🚨 Troubleshooting

### Problem: "Connection refused" or "Role does not exist"
**Solution**: You're connecting to the wrong database
```bash
# Stop local PostgreSQL
brew services stop postgresql@15

# Run database setup
./setup-database.sh

# Restart backend
cd backend && cargo run
```

### Problem: "No typography settings found"
**Solution**: Backend is connected to empty database
```bash
# Check Docker container is running
docker ps | grep rustcms_dev_postgres

# Verify data exists in Docker database
docker exec rustcms_dev_postgres psql -U myrustcms -d my_rust_cms -c "SELECT COUNT(*) FROM settings WHERE setting_type = 'typography';"

# Should return 29 typography settings
```

### Problem: Multiple PostgreSQL instances
**Solution**: Stop local PostgreSQL services
```bash
# Stop all local PostgreSQL
brew services stop postgresql@14
brew services stop postgresql@15

# Check what's running on port 5432
lsof -i :5432

# Should only show Docker PostgreSQL
```

## 📁 Environment Files

### `.env.docker` (Master Configuration)
- **Purpose**: Master configuration for Docker database
- **Usage**: Always copy this to `.env` before development
- **Contains**: Correct database credentials and settings

### `.env` (Active Configuration)
- **Purpose**: Active environment file used by backend
- **Usage**: Automatically created by setup scripts
- **Note**: Ignored by git, safe to modify locally

### `dev.env` (Legacy - DO NOT USE)
- **Purpose**: Old development configuration
- **Problem**: Points to local PostgreSQL (wrong database)
- **Status**: Deprecated, use `.env.docker` instead

## 🔒 Data Safety

### Your Data Location
- **Primary**: Docker PostgreSQL container (`myrustcms_dev_postgres`)
- **Contains**: All settings, typography configurations, posts, users
- **Backup**: Regular Docker volume backups recommended

### What NOT to Do
- ❌ Don't use `dev.env` (points to wrong database)
- ❌ Don't start local PostgreSQL services
- ❌ Don't create new databases manually
- ❌ Don't modify database credentials without updating all configs

### What TO Do
- ✅ Always run `./setup-database.sh` before development
- ✅ Use `./dev-start.sh` for automated setup
- ✅ Verify database connection before making changes
- ✅ Keep Docker container running during development

## 🔍 Verification Commands

### Check Database Connection
```bash
# Test connection to Docker database
docker exec myrustcms_dev_postgres psql -U myrustcms -d my_rust_cms -c "SELECT version();"
```

### Check Your Data
```bash
# Count total settings
docker exec myrustcms_dev_postgres psql -U myrustcms -d my_rust_cms -c "SELECT setting_type, COUNT(*) FROM settings GROUP BY setting_type;"

# Should show:
# typography | 29
# theme      | 5
# container  | 30
# etc.
```

### Check Backend Connection
```bash
# Test API endpoint
curl -s "http://localhost:8081/api/public/system/settings" | head -c 100

# Should return JSON with settings data
```

## 📞 Support

If you encounter database issues:

1. **First**: Run `./setup-database.sh`
2. **Second**: Check this guide's troubleshooting section
3. **Third**: Verify Docker container is running and accessible
4. **Last Resort**: Check Docker logs: `docker-compose logs postgres`

Remember: **Always use the Docker database** - it contains all your work!
