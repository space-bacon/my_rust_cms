#!/bin/bash

# Rust CMS Development Environment Manager
# Handles complete startup and shutdown of frontend, backend, database, and Docker services

set -e

# Ensure PostgreSQL 15 is in PATH for backup compatibility
export PATH="/opt/homebrew/opt/postgresql@15/bin:$PATH"

# Colors for better output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Print functions with consistent formatting
print_header() {
    echo -e "\n${PURPLE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${PURPLE}  🦀 Rust CMS Development Environment${NC}"
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════════${NC}\n"
}

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
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

print_step() {
    echo -e "${CYAN}▶${NC} $1"
}

# Check if required tools are installed
check_dependencies() {
    print_step "Checking dependencies..."
    
    local missing_deps=()
    
    if ! command -v docker &> /dev/null; then
        missing_deps+=("docker")
    fi
    
    if ! command -v docker-compose &> /dev/null; then
        missing_deps+=("docker-compose")
    fi
    
    if ! command -v cargo &> /dev/null; then
        missing_deps+=("cargo (Rust)")
    fi
    
    if ! command -v trunk &> /dev/null; then
        missing_deps+=("trunk")
    fi
    
    if ! command -v cargo-watch &> /dev/null; then
        missing_deps+=("cargo-watch")
    fi
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        print_error "Missing dependencies: ${missing_deps[*]}"
        echo "Please install the missing dependencies:"
        echo "  - Docker: https://docs.docker.com/get-docker/"
        echo "  - Rust: https://rustup.rs/"
        echo "  - Trunk: cargo install trunk"
        echo "  - Cargo-watch: cargo install cargo-watch"
        exit 1
    fi
    
    print_success "All dependencies are installed"
}

# Check if Docker is running
check_docker() {
    print_step "Checking Docker status..."
    
    if ! docker info >/dev/null 2>&1; then
        print_error "Docker is not running. Please start Docker Desktop."
        exit 1
    fi
    
    print_success "Docker is running"
}

# Load environment variables and setup database
load_environment() {
    print_step "Setting up database environment..."
    
    # Run the database setup script to ensure correct configuration
    if [ -f "./setup-database.sh" ]; then
        ./setup-database.sh
        print_success "Database environment configured"
    else
        print_warning "setup-database.sh not found, using manual setup"
        
        # Fallback: copy Docker environment configuration
        if [ -f ".env.docker" ]; then
            cp .env.docker .env
            print_success "Loaded Docker environment configuration"
        else
            print_error "No Docker environment configuration found"
            exit 1
        fi
    fi
    
    # Load the environment variables
    if [ -f ".env" ]; then
        set -a  # automatically export all variables
        source .env
        set +a
        print_success "Environment variables loaded"
    fi
}

# Comprehensive cleanup function
cleanup_all() {
    print_header
    print_step "🛑 Stopping all Rust CMS services..."
    
    # Stop all related processes
    print_step "Stopping Rust processes..."
    pkill -f "cargo-watch" 2>/dev/null || true
    pkill -f "cargo run" 2>/dev/null || true
    pkill -f "target/debug/backend" 2>/dev/null || true
    pkill -f "my_rust_cms" 2>/dev/null || true
    
    print_step "Stopping frontend processes..."
    pkill -f "trunk serve" 2>/dev/null || true
    pkill -f "trunk build" 2>/dev/null || true
    
    # Stop Docker services
    print_step "Stopping Docker services..."
    docker-compose -f docker-compose.dev.yml down --remove-orphans 2>/dev/null || true
    docker-compose down --remove-orphans 2>/dev/null || true
    
    # Clean up any hanging containers
    print_step "Cleaning up containers..."
    docker ps -q --filter "name=rustcms" | xargs -r docker stop 2>/dev/null || true
    docker ps -aq --filter "name=rustcms" | xargs -r docker rm 2>/dev/null || true
    
    print_success "All services stopped successfully"
    echo ""
}

# Setup database and wait for it to be ready
setup_database() {
    print_step "Setting up database..."
    
    # Start database service
    docker-compose -f docker-compose.dev.yml up -d postgres
    
    # Wait for database to be ready
    print_step "Waiting for database to be ready..."
    ./scripts/wait-for-db.sh localhost 5432 rustcms my_rust_cms
    
    print_success "Database is ready"
}

# Run database migrations
run_migrations() {
    print_step "Running database migrations..."
    
    if [ -d "backend" ]; then
        cd backend
        
        # Set DATABASE_URL if not already set
        export DATABASE_URL="${DATABASE_URL:-postgres://rustcms:password@localhost:5432/my_rust_cms}"
        
        # Run migrations with better error handling
        if diesel migration run; then
            print_success "Database migrations completed"
        else
            print_warning "Migration issues detected, but continuing..."
        fi
        
        cd ..
    else
        print_error "Backend directory not found"
        exit 1
    fi
}

# Start backend service
start_backend() {
    print_step "Starting backend with hot reload..."
    
    if [ ! -d "backend" ]; then
        print_error "Backend directory not found"
        exit 1
    fi
    
    cd backend
    
    # Start backend with cargo-watch for hot reloading
    cargo-watch -x 'run' &
    BACKEND_PID=$!
    
    cd ..
    
    # Wait a moment for backend to start
    sleep 3
    
    print_success "Backend started (PID: $BACKEND_PID)"
}

# Start frontend service  
start_frontend() {
    print_step "Starting frontend with hot reload..."
    
    if [ ! -d "frontend" ]; then
        print_error "Frontend directory not found"
        exit 1
    fi
    
    cd frontend
    
    # Start frontend with trunk serve (using release flag to avoid "too many locals" error)
    trunk serve --release --address 127.0.0.1 --port 8080 --open &
    FRONTEND_PID=$!
    
    cd ..
    
    # Wait a moment for frontend to start
    sleep 3
    
    print_success "Frontend started (PID: $FRONTEND_PID)"
}

# Display service status and URLs
show_status() {
    print_header
    print_success "🚀 Rust CMS Development Environment is Ready!"
    echo ""
    echo -e "${CYAN}📍 Service URLs:${NC}"
    echo -e "   ${GREEN}Frontend:${NC}  http://localhost:8080"
    echo -e "   ${GREEN}Backend:${NC}   http://localhost:8081"
    echo -e "   ${GREEN}Database:${NC}  postgresql://rustcms:password@localhost:5432/my_rust_cms"
    echo ""
    echo -e "${CYAN}🔧 Development Tools:${NC}"
    echo -e "   ${GREEN}Hot Reload:${NC} Both frontend and backend will auto-reload on changes"
    echo -e "   ${GREEN}Logs:${NC}      Check terminal output for real-time logs"
    echo ""
    echo -e "${YELLOW}💡 Tips:${NC}"
    echo -e "   • Press ${CYAN}Ctrl+C${NC} to stop all services"
    echo -e "   • Run ${CYAN}./dev-stop.sh${NC} to stop services manually"
    echo -e "   • Check ${CYAN}docker-compose logs${NC} for database logs"
    echo ""
    print_step "Press Ctrl+C to stop all services..."
}

# Trap signals for cleanup
trap cleanup_all SIGINT SIGTERM EXIT

# Main execution flow
main() {
    print_header
    
    # Always cleanup first to ensure clean state
    cleanup_all
    
    # Check system requirements
    check_dependencies
    check_docker
    
    # Load configuration
    load_environment
    
    # Setup and start services in order
    setup_database
    run_migrations
    start_backend
    start_frontend
    
    # Show status and wait
    show_status
    
    # Keep the script running and wait for processes
    wait
}

# Run main function
main "$@"