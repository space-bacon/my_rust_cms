#!/bin/bash

# Rust CMS Development Environment Setup
# One-time setup script for new development environments

set -e

# Colors for better output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

print_header() {
    echo -e "\n${PURPLE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${PURPLE}  🛠️  Rust CMS Development Environment Setup${NC}"
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════════${NC}\n"
}

print_step() {
    echo -e "${CYAN}▶${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Install Rust if not present
setup_rust() {
    print_step "Checking Rust installation..."
    
    if command_exists rustc && command_exists cargo; then
        local rust_version=$(rustc --version)
        print_success "Rust is already installed: $rust_version"
    else
        print_step "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
        print_success "Rust installed successfully"
    fi
    
    # Ensure we have the wasm target
    print_step "Adding WebAssembly target..."
    rustup target add wasm32-unknown-unknown
    print_success "WebAssembly target added"
}

# Install required Rust tools
setup_rust_tools() {
    print_step "Installing required Rust tools..."
    
    local tools=("trunk" "cargo-watch" "diesel_cli")
    
    for tool in "${tools[@]}"; do
        if command_exists "$tool"; then
            print_success "$tool is already installed"
        else
            print_step "Installing $tool..."
            case $tool in
                "diesel_cli")
                    cargo install diesel_cli --no-default-features --features postgres
                    ;;
                *)
                    cargo install "$tool"
                    ;;
            esac
            print_success "$tool installed"
        fi
    done
}

# Check Docker installation
setup_docker() {
    print_step "Checking Docker installation..."
    
    if command_exists docker; then
        print_success "Docker is installed"
        
        if docker info >/dev/null 2>&1; then
            print_success "Docker is running"
        else
            print_warning "Docker is installed but not running. Please start Docker Desktop."
        fi
    else
        print_error "Docker is not installed. Please install Docker Desktop:"
        print_info "  macOS: https://docs.docker.com/desktop/mac/install/"
        print_info "  Linux: https://docs.docker.com/engine/install/"
        print_info "  Windows: https://docs.docker.com/desktop/windows/install/"
        exit 1
    fi
    
    if command_exists docker-compose; then
        print_success "Docker Compose is available"
    else
        print_error "Docker Compose is not available. Please ensure Docker Desktop is properly installed."
        exit 1
    fi
}

# Setup environment files
setup_environment() {
    print_step "Setting up environment files..."
    
    # Create dev.env if it doesn't exist
    if [ ! -f dev.env ]; then
        print_step "Creating dev.env..."
        cat > dev.env << 'EOF'
# Development Environment Variables
# Database Configuration
DATABASE_URL=postgres://rustcms:password@localhost:5432/my_rust_cms
POSTGRES_USER=rustcms
POSTGRES_PASSWORD=password
POSTGRES_DB=my_rust_cms

# Backend Configuration
BACKEND_HOST=localhost
BACKEND_PORT=8081
RUST_LOG=debug

# Frontend Configuration (for trunk)
TRUNK_SERVE_HOST=127.0.0.1
TRUNK_SERVE_PORT=8080

# Development flags
DEVELOPMENT_MODE=true
HOT_RELOAD=true

# Session and Security (development values)
SESSION_SECRET=dev_session_secret_key_please_change_in_production
JWT_SECRET=dev_jwt_secret_key_please_change_in_production
EOF
        print_success "Created dev.env"
    else
        print_success "dev.env already exists"
    fi
    
    # Create docker.env if it doesn't exist
    if [ ! -f docker.env ]; then
        if [ -f docker.env.example ]; then
            cp docker.env.example docker.env
            print_success "Created docker.env from example"
        else
            print_warning "docker.env.example not found, creating basic docker.env"
            cat > docker.env << 'EOF'
# Docker Environment Configuration
POSTGRES_USER=rustcms
POSTGRES_PASSWORD=password
POSTGRES_DB=my_rust_cms
DATABASE_URL=postgres://rustcms:password@postgres:5432/my_rust_cms
RUST_LOG=info
SERVER_HOST=0.0.0.0
SERVER_PORT=8081
SESSION_SECRET=dev_session_secret_change_in_production
EOF
        fi
    else
        print_success "docker.env already exists"
    fi
}

# Create necessary directories
setup_directories() {
    print_step "Creating necessary directories..."
    
    local dirs=("scripts" "backend/uploads" "uploads" "backups" "logs")
    
    for dir in "${dirs[@]}"; do
        if [ ! -d "$dir" ]; then
            mkdir -p "$dir"
            print_success "Created directory: $dir"
        else
            print_success "Directory exists: $dir"
        fi
    done
}

# Test the setup
test_setup() {
    print_step "Testing the setup..."
    
    # Test Rust compilation
    print_step "Testing Rust compilation..."
    if cd backend && cargo check --quiet; then
        print_success "Backend compiles successfully"
        cd ..
    else
        print_error "Backend compilation failed"
        cd ..
        return 1
    fi
    
    # Test frontend compilation
    print_step "Testing frontend compilation..."
    if cd frontend && cargo check --quiet; then
        print_success "Frontend compiles successfully"
        cd ..
    else
        print_error "Frontend compilation failed"
        cd ..
        return 1
    fi
    
    print_success "All tests passed!"
}

# Display final instructions
show_final_instructions() {
    print_header
    print_success "🎉 Development environment setup completed!"
    echo ""
    echo -e "${CYAN}📋 Next Steps:${NC}"
    echo -e "   1. ${GREEN}Start development:${NC} ./dev-start.sh"
    echo -e "   2. ${GREEN}Stop development:${NC}  ./dev-stop.sh"
    echo -e "   3. ${GREEN}Access frontend:${NC}   http://localhost:8080"
    echo -e "   4. ${GREEN}Access backend:${NC}    http://localhost:8081"
    echo ""
    echo -e "${CYAN}🔧 Available Scripts:${NC}"
    echo -e "   • ${GREEN}./dev-start.sh${NC}     - Start full development environment"
    echo -e "   • ${GREEN}./dev-stop.sh${NC}      - Stop all development services"
    echo -e "   • ${GREEN}./docker-dev.sh${NC}    - Docker development helper"
    echo -e "   • ${GREEN}./docker-prod.sh${NC}   - Docker production helper"
    echo ""
    echo -e "${YELLOW}💡 Tips:${NC}"
    echo -e "   • Edit ${CYAN}dev.env${NC} to customize development settings"
    echo -e "   • Check ${CYAN}docker-compose logs${NC} for database logs"
    echo -e "   • Use ${CYAN}cargo watch${NC} for automatic rebuilds"
    echo ""
}

# Main execution
main() {
    print_header
    
    print_info "This script will set up your Rust CMS development environment."
    print_info "It will install required tools and configure the environment."
    echo ""
    
    setup_rust
    setup_rust_tools
    setup_docker
    setup_directories
    setup_environment
    
    print_step "Running setup tests..."
    if test_setup; then
        show_final_instructions
    else
        print_error "Setup tests failed. Please check the errors above."
        exit 1
    fi
}

# Run main function
main "$@"
