# 🦀 Rust CMS Development Guide

A comprehensive guide for setting up and working with the Rust CMS development environment.

## 🚀 Quick Start

### First Time Setup

1. **Clone and setup the environment:**
   ```bash
   git clone <repository-url>
   cd my_rust_cms
   ./setup-dev.sh
   ```

2. **Start development:**
   ```bash
   ./dev-start.sh
   ```

3. **Access the application:**
   - Frontend: http://localhost:8080
   - Backend API: http://localhost:8081
   - Database: postgresql://rustcms:password@localhost:5432/my_rust_cms

### Daily Development

- **Start everything:** `./dev-start.sh`
- **Stop everything:** `./dev-stop.sh` or `Ctrl+C`

## 🎯 What the Scripts Do

**dev-start.sh:**
- 📊 Starts PostgreSQL database in Docker
- 🗄️ Runs database migrations
- 🦀 Starts backend with hot reload (`cargo-watch`)
- 🎨 Starts frontend with hot reload (`trunk serve`)
- 🌐 Opens browser to http://localhost:8080

**dev-stop.sh:**
- 🛑 Stops all running processes
- 📊 Stops Docker database

## 📋 Development Scripts

| Script | Purpose | Description |
|--------|---------|-------------|
| `./setup-dev.sh` | One-time setup | Installs tools, creates configs, tests setup |
| `./dev-start.sh` | Start development | Starts database, backend, frontend with hot reload |
| `./dev-stop.sh` | Stop development | Stops all services cleanly |
| `./docker-dev.sh` | Docker development | Helper for Docker-based development |
| `./docker-prod.sh` | Docker production | Production deployment helper |

## 🛠️ Prerequisites

### Required Tools

- **Rust** (latest stable)
- **Docker & Docker Compose**
- **Node.js** (for some build tools)

### Rust Tools (auto-installed by setup-dev.sh)

- `trunk` - Frontend build tool
- `cargo-watch` - Auto-rebuild on file changes
- `diesel_cli` - Database migrations

## 🏗️ Architecture Overview

```
my_rust_cms/
├── backend/          # Rust backend (Actix Web)
├── frontend/         # Rust frontend (Yew + Trunk)
├── migrations/       # Database migrations
├── scripts/          # Helper scripts
├── docker-compose.*  # Docker configurations
└── dev-*.sh         # Development scripts
```

## 🔧 Configuration Files

### Environment Files

- **`dev.env`** - Development environment variables
- **`docker.env`** - Docker environment variables
- **`.env.production`** - Production environment (create manually)

### Key Settings

```bash
# Database
DATABASE_URL=postgres://rustcms:password@localhost:5432/my_rust_cms
POSTGRES_USER=rustcms
POSTGRES_PASSWORD=password
POSTGRES_DB=my_rust_cms

# Backend
BACKEND_HOST=localhost
BACKEND_PORT=8081
RUST_LOG=debug

# Frontend
TRUNK_SERVE_HOST=127.0.0.1
TRUNK_SERVE_PORT=8080
```

## 🗄️ Database Management

### Development Database

The development database runs in Docker and is automatically managed by the dev scripts.

### Manual Database Operations

```bash
# Connect to database
psql postgres://rustcms:password@localhost:5432/my_rust_cms

# Run migrations manually
cd backend && diesel migration run

# Reset database
docker-compose -f docker-compose.dev.yml down -v
./dev-start.sh
```

### Database Troubleshooting

**"database does not exist" error:**
1. Stop all services: `./dev-stop.sh`
2. Remove Docker volumes: `docker-compose -f docker-compose.dev.yml down -v`
3. Restart: `./dev-start.sh`

## 🔥 Hot Reload Development

Both frontend and backend support hot reloading:

- **Backend**: Uses `cargo-watch` to rebuild on Rust file changes
- **Frontend**: Uses `trunk serve` to rebuild on frontend changes
- **Database**: Automatically applies migrations on startup

## 🐳 Docker Development

### Development Mode

```bash
# Start with Docker
./docker-dev.sh up-dev

# View logs
./docker-dev.sh logs

# Stop services
./docker-dev.sh down
```

### Production Mode

```bash
# Create production environment
./docker-prod.sh create-env

# Build and deploy
./docker-prod.sh build
./docker-prod.sh deploy
```

## 🚨 Troubleshooting

### Common Issues

**Port already in use:**
```bash
./dev-stop.sh  # Stop all services
lsof -ti:8080 | xargs kill -9  # Force kill port 8080
lsof -ti:8081 | xargs kill -9  # Force kill port 8081
```

**Database connection issues:**
```bash
# Check if database is running
docker ps | grep postgres

# Restart database
docker-compose -f docker-compose.dev.yml restart postgres

# Check database logs
docker-compose -f docker-compose.dev.yml logs postgres
```

**Frontend build issues:**
```bash
# Clear trunk cache
cd frontend && trunk clean

# Reinstall trunk
cargo install trunk --force

# Check wasm target
rustup target add wasm32-unknown-unknown
```

**Backend compilation issues:**
```bash
# Clean build cache
cd backend && cargo clean

# Update dependencies
cargo update

# Check for missing system dependencies (Linux)
sudo apt-get install pkg-config libssl-dev libpq-dev
```

### Performance Tips

1. **Use release builds for frontend in development:**
   ```bash
   # In frontend/
   trunk serve --release
   ```

2. **Optimize Docker builds:**
   ```bash
   # Use BuildKit for faster builds
   export DOCKER_BUILDKIT=1
   ```

3. **Database performance:**
   - Use local PostgreSQL for faster development
   - Increase shared_buffers in PostgreSQL config

## 📁 Project Structure

### Backend (`backend/`)

```
backend/
├── src/
│   ├── controllers/     # API endpoints
│   ├── models/         # Database models
│   ├── services/       # Business logic
│   ├── middleware/     # Request middleware
│   └── main.rs        # Application entry point
├── migrations/         # Database migrations
└── Cargo.toml         # Backend dependencies
```

### Frontend (`frontend/`)

```
frontend/
├── src/
│   ├── components/     # Reusable UI components
│   ├── pages/         # Page components
│   ├── services/      # API clients
│   └── main.rs       # Frontend entry point
├── styles/           # CSS stylesheets
└── Cargo.toml       # Frontend dependencies
```

## 🔐 Security Notes

### Development Security

- Default passwords are used for development
- HTTPS is not enabled by default
- Debug logging is enabled

### Production Security

- Change all default passwords
- Enable HTTPS with proper certificates
- Set appropriate log levels
- Use environment-specific secrets

## 📚 Additional Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Yew Framework](https://yew.rs/)
- [Actix Web](https://actix.rs/)
- [Diesel ORM](https://diesel.rs/)
- [Docker Documentation](https://docs.docker.com/)

## 🤝 Contributing

1. Follow the existing code style
2. Write tests for new features
3. Update documentation as needed
4. Use the provided development scripts

## 📞 Support

If you encounter issues:

1. Check this documentation
2. Review the troubleshooting section
3. Check Docker and database logs
4. Ensure all prerequisites are installed
