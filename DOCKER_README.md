# Docker Setup for Rust CMS

This document explains how to use Docker with your Rust CMS application.

## Overview

The Docker setup includes:
- **PostgreSQL 15** database with persistent storage
- **Adminer** for database management (development only)
- **Rust CMS Application** (backend + frontend)
- **Helper scripts** for easy Docker management

## Quick Start

### 1. Prerequisites
- Docker Desktop for macOS is installed and running
- Git repository is cloned

### 2. Environment Configuration
```bash
# Copy the example environment file
cp docker.env.example .env

# Edit the .env file with your settings
nano .env
```

**Important**: Change the default passwords in `.env` before running in production!

### 3. Development Workflow

#### Using Helper Scripts (Recommended)
```bash
# Build the application
./docker-dev.sh build

# Start all services with development tools
./docker-dev.sh up-dev

# View logs
./docker-dev.sh logs

# Stop services
./docker-dev.sh down

# Get help
./docker-dev.sh help
```

#### Direct Docker Compose Commands
```bash
# Start services in background
docker-compose up -d

# Start with development profile (includes Adminer)
docker-compose --profile dev up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

## Services and Ports

| Service | Port | Description |
|---------|------|-------------|
| Rust CMS App | 8080 | Main application |
| PostgreSQL | 5432 | Database |
| Adminer | 8081 | Database admin (dev only) |

## Access Points

- **Application**: http://localhost:8080
- **Database Admin**: http://localhost:8081 (development only)
  - Server: `postgres`
  - Username: `myrustcms` (or your configured value)
  - Password: (as set in .env)
  - Database: `my_rust_cms`

## File Structure

```
├── Dockerfile                  # Main application Dockerfile
├── Dockerfile.simple          # Simple testing Dockerfile
├── docker-compose.yml         # Main compose configuration
├── docker-compose.prod.yml    # Production overrides
├── docker-compose.test.yml    # Testing configuration
├── .dockerignore              # Files excluded from build
├── docker.env.example        # Environment template
├── docker-dev.sh             # Development helper script
└── docker-prod.sh            # Production helper script
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `POSTGRES_USER` | myrustcms | Database username |
| `POSTGRES_PASSWORD` | (required) | Database password |
| `POSTGRES_DB` | my_rust_cms | Database name |
| `RUST_LOG` | info | Logging level |
| `SERVER_HOST` | 0.0.0.0 | Server bind address |
| `SERVER_PORT` | 8080 | Server port |

## Development Commands

### Build and Start
```bash
# Full rebuild and start
./docker-dev.sh build && ./docker-dev.sh up-dev
```

### Database Operations
```bash
# Run database migrations
./docker-dev.sh migrate

# Access database shell
docker exec -it myrustcms_postgres psql -U myrustcms -d my_rust_cms

# Backup database
./docker-prod.sh backup
```

### Debugging
```bash
# View application logs
./docker-dev.sh logs web

# View database logs
./docker-dev.sh logs postgres

# Access application shell
./docker-dev.sh shell

# Check service status
./docker-dev.sh status
```

## Production Deployment

### 1. Create Production Environment
```bash
# Generate secure production configuration
./docker-prod.sh create-env

# Edit production settings
nano .env.production
```

### 2. Deploy
```bash
# Build and deploy to production
./docker-prod.sh build
./docker-prod.sh deploy

# Monitor production
./docker-prod.sh status
./docker-prod.sh logs
```

### 3. Maintenance
```bash
# Backup database
./docker-prod.sh backup

# Update application
./docker-prod.sh update

# Scale web service
./docker-prod.sh scale 3
```

## Troubleshooting

### Common Issues

**Port already in use**:
```bash
# Find what's using the port
lsof -i :8080

# Stop existing containers
docker-compose down
```

**Build fails**:
```bash
# Clean rebuild
./docker-dev.sh clean
./docker-dev.sh build
```

**Database connection issues**:
```bash
# Check database health
docker-compose exec postgres pg_isready -U myrustcms

# Reset database
docker-compose down -v
docker-compose up -d
```

**Permission issues**:
```bash
# Make scripts executable
chmod +x docker-dev.sh docker-prod.sh
```

### Logs and Debugging
```bash
# View all logs
docker-compose logs

# Follow specific service logs
docker-compose logs -f web

# Check container status
docker-compose ps

# Inspect container
docker inspect myrustcms_web
```

## Security Notes

### Development
- Default passwords are used for convenience
- Adminer is exposed for database management
- Debug logging is enabled

### Production
- **Always** change default passwords
- Adminer is disabled by default
- Use environment-specific configurations
- Enable SSL/TLS in production reverse proxy

## Performance Optimization

### Resource Limits
The production compose file includes resource limits:
- CPU: 1.0 core limit, 0.5 core reservation
- Memory: 512MB limit, 256MB reservation

### Database Tuning
Production PostgreSQL includes optimized settings:
- Connection pooling (200 max connections)
- Memory optimization (256MB shared buffers)
- Performance tuning for SSDs

## Backup and Recovery

### Automated Backups
```bash
# Create backup
./docker-prod.sh backup

# Restore from backup
./docker-prod.sh restore backup_20240103_120000.sql
```

### Manual Backup
```bash
# Database dump
docker exec myrustcms_postgres pg_dump -U myrustcms my_rust_cms > backup.sql

# File backup
docker cp myrustcms_web:/app/uploads ./uploads_backup
```

## Advanced Configuration

### Custom Networks
The setup creates isolated Docker networks for security and organization.

### Volume Management
- `postgres_data`: Database storage
- `uploads`: Application file uploads

### Health Checks
All services include health checks:
- PostgreSQL: `pg_isready`
- Web application: HTTP endpoint check

## Next Steps

1. **Development**: Use `./docker-dev.sh up-dev` for daily development
2. **Testing**: Run tests in isolated containers
3. **Production**: Follow the production deployment guide
4. **Monitoring**: Set up log aggregation and monitoring
5. **CI/CD**: Integrate with your deployment pipeline

For more help, run `./docker-dev.sh help` or `./docker-prod.sh help`.
