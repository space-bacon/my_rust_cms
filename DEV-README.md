# 🦀 Rust CMS Development Setup

Simple development environment for fast iteration with hot reload.

## Quick Start

```bash
# Start development environment
./dev-start.sh

# Stop development environment  
./dev-stop.sh
```

## What it does

**dev-start.sh:**
- 📊 Starts PostgreSQL database in Docker
- 🗄️ Runs database migrations
- 🦀 Starts backend with hot reload (`cargo-watch`)
- 🎨 Starts frontend with hot reload (`trunk serve`)
- 🌐 Opens browser to http://localhost:8080

**dev-stop.sh:**
- 🛑 Stops all running processes
- 📊 Stops Docker database

## URLs

- **Frontend:** http://localhost:8080
- **Backend:** http://localhost:8081  
- **Database:** postgresql://rustcms:password@localhost:5432/my_rust_cms

## Prerequisites

Install these tools if you don't have them:

```bash
# Install cargo-watch for backend hot reload
cargo install cargo-watch

# Install trunk for frontend hot reload
cargo install trunk
```

## Environment

The development environment uses `dev.env` for configuration. You can modify database credentials and other settings there.

## Hot Reload Features

- 🔄 **Backend:** Changes to Rust files automatically restart the server
- 🔄 **Frontend:** Changes to frontend files automatically reload the browser
- 🔄 **Database:** Persistent data across restarts

## Production vs Development

- **Development:** Local Rust processes + Docker database (this setup)
- **Production:** Full Docker setup with `docker-compose up`

The development setup gives you faster iteration while maintaining production parity for the database.
