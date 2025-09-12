# 🦀 My Rust CMS

A full-stack content management system built entirely in Rust, featuring a sophisticated visual page builder with live edit mode and professional-grade customization. Built with the **RAYDT Stack** (Rust • Axum • Yew • Diesel • Tower) for maximum performance, safety, and developer productivity.

[![Built with RAYDT Stack](https://img.shields.io/badge/Built%20with-RAYDT%20Stack-orange.svg)](./RAYDT-STACK.md)
[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/Frontend-WebAssembly-blue.svg)](https://webassembly.org/)
[![PostgreSQL](https://img.shields.io/badge/Database-PostgreSQL-blue.svg)](https://www.postgresql.org/)
[![Docker](https://img.shields.io/badge/Deployment-Docker-blue.svg)](https://www.docker.com/)
[![Live Edit](https://img.shields.io/badge/Feature-Live%20Edit%20Mode-green.svg)](#-live-edit-mode)

## 🎯 Overview

A professional, production-ready content management system built entirely in Rust. Features an advanced visual page builder with live edit mode, gradient backgrounds, shape masks, and comprehensive customization options. Comes pre-configured with professional styling and sample content.

### 🌟 What Makes This Special

**🎨 Live Edit Mode**
- Real-time visual editing with instant preview
- Multi-property change support with persistent state
- Gradient backgrounds, shape masks, and scroll effects
- Professional header templates with shrink animations

**🚀 Performance**
- WebAssembly frontend for near-native browser performance
- Rust backend with zero-copy serialization
- Optimized component rendering with minimal re-renders
- Production-ready Docker deployment

**🛡️ Security & Safety**
- Memory safety through Rust's ownership system
- Type safety across the entire stack
- Secure session-based authentication
- Input sanitization and XSS protection

**🎯 Production Ready**
- Complete default data setup with professional styling
- Comprehensive error handling and logging
- Docker-based deployment with health checks
- Clean codebase with zero compiler warnings

## ✨ Features

### 🎨 Live Edit Mode

Professional visual editing system with real-time preview:

- **🎛️ Multi-Property Editing**: 
  - Change height, colors, gradients, and effects simultaneously
  - Persistent state management prevents data loss
  - Context-aware property changes preserve existing styling

- **🌈 Visual Effects**:
  - Gradient backgrounds with live preview
  - Shape masks (tilt, wave, zigzag) with real-time updates
  - Scroll effects with smooth animations
  - Site title color customization

- **⚙️ Advanced Controls**:
  - Header height adjustment with proper constraints
  - Navigation styling with hover effects
  - Button customization with theme integration
  - Typography system with live preview

### 🏗️ Page Builder

Sophisticated visual page builder with advanced component management:

- **🧩 Nested Architecture**: 
  - Recursive component nesting with unlimited depth
  - Container, TwoColumn, and ThreeColumn layout components
  - Intuitive drag-and-drop interface
  - Visual selection indicators with clear hierarchy

- **⚙️ Component Management**:
  - Hero, Text, Image, Gallery, and custom components
  - Real-time property editing with live preview
  - Component-specific styling and animations
  - Responsive design controls

- **🎨 Design System**:
  - Professional gradient backgrounds
  - Shape mask effects for modern design
  - Typography system with custom fonts
  - Admin dashboard with crab emoji branding

### 🗄️ Database & Content

Production-grade PostgreSQL with comprehensive schema:

- **📝 Content Management**: Posts, pages, categories with rich metadata
- **🧩 Component Templates**: 8 pre-configured templates with professional styling
- **⚙️ Settings System**: 47+ configuration options for complete customization
- **👥 User Management**: Role-based access with secure authentication
- **🔧 Default Data**: Complete setup with sample content and professional styling

## 🚀 Quick Start

### 🐳 Docker Setup (Recommended)

The easiest way to get started is using Docker with our pre-configured setup:

#### Prerequisites

- **Docker & Docker Compose**: [Install Docker](https://docs.docker.com/get-docker/)
- **Git**: For cloning the repository

#### Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/my_rust_cms.git
   cd my_rust_cms
   ```

2. **Start the entire stack**:
   ```bash
   # Start all services (database, backend, frontend)
   docker-compose up -d
   
   # Or use the development script
   ./dev-start.sh
   ```

3. **Access your CMS**:
   - **Frontend**: http://localhost:8080
   - **Backend API**: http://localhost:8081
   - **Admin Panel**: http://localhost:8080/admin

4. **Login credentials**:
   - **Username**: `admin`
   - **Password**: `admin`

#### What You Get Out of the Box

- 🎨 **Professional Header**: Gradient background with tilt mask and shrink effects
- 📝 **Sample Content**: 4 blog posts and structured pages
- ⚙️ **Complete Configuration**: 47 settings including typography and design system
- 🛠️ **Live Edit Mode**: Fully functional with all improvements
- 🦀 **Admin Dashboard**: Modern interface with crab emoji branding

### 🛠️ Local Development Setup

For development with hot reloading and debugging:

#### Prerequisites

- **Rust** (latest stable): [Install from rustup.rs](https://rustup.rs/)
- **Trunk**: `cargo install trunk`
- **PostgreSQL** (13+): [Install PostgreSQL](https://www.postgresql.org/download/)
- **Diesel CLI**: `cargo install diesel_cli --features postgres`

#### Setup

1. **Environment Configuration**:
   ```bash
   # Copy Docker environment as template
   cp docker.env .env
   
   # Update database URL for local development
   # Change: postgres://user:pass@postgres:5432/db
   # To:     postgres://user:pass@localhost:5432/db
   ```

2. **Database Setup**:
   ```bash
   # Start PostgreSQL (via Docker for convenience)
   docker-compose up -d postgres
   
   # Run migrations
   cd backend
   diesel setup
   diesel migration run
   ```

3. **Start Development Servers**:
   ```bash
   # Terminal 1: Backend
   cd backend
   cargo run
   
   # Terminal 2: Frontend  
   cd frontend
   trunk serve --release  # Use --release for complex builds
   ```

## 🎨 Using the CMS

### Live Edit Mode

1. **Navigate to your site**: http://localhost:8080
2. **Enable Live Edit**: Click the "🎨 Live Edit Mode" button (visible when logged in)
3. **Edit Components**: Click any component to open the properties panel
4. **Customize**: Adjust height, colors, gradients, effects in real-time
5. **Save Changes**: Click "Save Changes" to persist your customizations

### Admin Dashboard

1. **Access Admin**: http://localhost:8080/admin
2. **Login**: Use `admin` / `admin` credentials
3. **Manage Content**: Create posts, pages, and manage site settings
4. **Design System**: Customize typography, colors, and layout
5. **Templates**: Manage header, footer, and component templates

### Page Builder

1. **Edit Pages**: Go to Admin > Pages > Edit
2. **Add Components**: Drag components from the sidebar
3. **Nested Layouts**: Use Container, TwoColumn, ThreeColumn components
4. **Style Components**: Click components to edit properties
5. **Preview**: Real-time preview with responsive design

## 🔧 Configuration

### Default Setup

Your CMS comes pre-configured with:

- **Header Template**: Fixed header with gradient background (#667eea to #764ba2)
- **Shape Effects**: Lower tilt mask for modern design
- **Typography**: Pixelify Sans font family with optimized sizing
- **Navigation**: Responsive header navigation with hover effects
- **Content**: Professional sample posts and hero sections

### Customization Options

- **47+ Settings**: Complete control over typography, colors, layout
- **8 Component Templates**: Headers, footers, containers, modals
- **Design System**: Integrated color schemes and typography
- **Live Edit**: Real-time visual customization
- **Responsive Design**: Mobile-first approach with breakpoint controls

## 🚀 Deployment

### Docker Production Deployment

1. **Build for production**:
   ```bash
   # Build optimized images
   docker-compose -f docker-compose.prod.yml build
   
   # Deploy to production
   docker-compose -f docker-compose.prod.yml up -d
   ```

2. **Environment Configuration**:
   ```bash
   # Create production environment
   cp docker.env .env.prod
   
   # Update for production:
   # - Change passwords and secrets
   # - Set RUST_ENV=production
   # - Configure proper database URL
   # - Set up HTTPS/SSL
   ```

### Manual Deployment

1. **Build the application**:
   ```bash
   # Backend
   cd backend && cargo build --release
   
   # Frontend
   cd frontend && trunk build --release
   ```

2. **Deploy artifacts**:
   - Backend binary: `target/release/backend`
   - Frontend files: `dist/` directory
   - Database: Run migrations on production DB

## 🧪 Development

### Development Scripts

- `./dev-start.sh` - Start all development services
- `./dev-stop.sh` - Stop all development services  
- `docker-compose up -d` - Start Docker services
- `docker-compose down` - Stop Docker services

### Code Quality

The codebase maintains zero warnings and follows Rust best practices:

```bash
# Check compilation (should show 0 warnings)
cargo check --workspace

# Format code
cargo fmt --all

# Advanced linting
cargo clippy --all-targets --all-features

# Security audit
cargo audit
```

### Testing

```bash
# Backend tests
cd backend && cargo test

# Frontend WASM tests  
cd frontend && wasm-pack test --headless --firefox

# Integration tests
cargo test --workspace
```

## 📚 Documentation

### Core Documentation

- **[DEFAULT_SETUP.md](./DEFAULT_SETUP.md)** - Default data and configuration
- **[DEVELOPMENT.md](./DEVELOPMENT.md)** - Development environment setup
- **[DATABASE_SETUP.md](./DATABASE_SETUP.md)** - Database configuration and management
- **[SECURITY.md](./SECURITY.md)** - Security features and best practices
- **[RAYDT-STACK.md](./RAYDT-STACK.md)** - Architecture and technology stack

### Plugin System

- **[PLUGIN_DEVELOPMENT.md](./PLUGIN_DEVELOPMENT.md)** - Plugin development guide
- **[PLUGIN_ARCHITECTURE.md](./docs/PLUGIN_ARCHITECTURE.md)** - Plugin system architecture
- **[PLUGIN_MERKLE_VERIFICATION.md](./docs/PLUGIN_MERKLE_VERIFICATION.md)** - Plugin security

## 🎯 Key Improvements

### Recent Major Updates

- **✅ Live Edit Mode**: Complete rewrite with multi-property support
- **✅ Default Data Setup**: Professional configuration out-of-the-box  
- **✅ Docker Integration**: One-command deployment with health checks
- **✅ Code Cleanup**: Zero compiler warnings across entire codebase
- **✅ Template System**: 8 professional component templates
- **✅ Design System**: Integrated typography and color management

### Performance Optimizations

- **WebAssembly**: Frontend compiled to WASM for maximum performance
- **Zero-Copy**: Efficient data serialization and transfer
- **Component Caching**: Smart rendering with minimal updates
- **Database Optimization**: Indexed queries and connection pooling

### Security Features

- **Session-Based Auth**: Secure authentication without JWT vulnerabilities
- **Input Sanitization**: XSS and injection prevention
- **File Upload Security**: Type validation and size limits
- **CORS Configuration**: Proper cross-origin request handling

## 🤝 Contributing

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes** with proper testing
4. **Ensure zero warnings**: `cargo check --workspace`
5. **Submit a pull request**

### Development Guidelines

- Follow Rust best practices and idioms
- Maintain zero compiler warnings
- Add tests for new features
- Update documentation for significant changes
- Use the provided development scripts

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Rust Community** for the amazing ecosystem
- **Yew Framework** for WebAssembly frontend capabilities  
- **Diesel ORM** for type-safe database operations
- **Axum Framework** for high-performance HTTP handling
- **PostgreSQL** for reliable data persistence

---

## 🚀 Ready to Get Started?

```bash
# One command to rule them all
git clone https://github.com/yourusername/my_rust_cms.git
cd my_rust_cms
docker-compose up -d

# Visit http://localhost:8080 and start building!
```

**Default Login**: `admin` / `admin`

Your professional Rust CMS is ready to use with beautiful gradients, live editing, and modern design! 🎉