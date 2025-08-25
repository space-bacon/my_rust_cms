#!/bin/bash

# Rust CMS Development Environment Stopper
# Comprehensive shutdown of all development services

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
    echo -e "${PURPLE}  🛑 Stopping Rust CMS Development Environment${NC}"
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

# Function to stop processes by pattern with better feedback
stop_processes() {
    local pattern=$1
    local description=$2
    
    print_step "Stopping $description..."
    
    local pids=$(pgrep -f "$pattern" 2>/dev/null || true)
    
    if [ -n "$pids" ]; then
        echo "$pids" | xargs kill -TERM 2>/dev/null || true
        sleep 2
        
        # Force kill if still running
        local remaining_pids=$(pgrep -f "$pattern" 2>/dev/null || true)
        if [ -n "$remaining_pids" ]; then
            print_warning "Force killing remaining $description processes..."
            echo "$remaining_pids" | xargs kill -KILL 2>/dev/null || true
        fi
        
        print_success "$description stopped"
    else
        print_step "No $description processes found"
    fi
}

# Function to stop Docker services
stop_docker_services() {
    print_step "Stopping Docker services..."
    
    # Stop development compose services
    if [ -f docker-compose.dev.yml ]; then
        docker-compose -f docker-compose.dev.yml down --remove-orphans 2>/dev/null || true
        print_success "Development Docker services stopped"
    fi
    
    # Stop main compose services
    if [ -f docker-compose.yml ]; then
        docker-compose down --remove-orphans 2>/dev/null || true
        print_success "Main Docker services stopped"
    fi
    
    # Clean up any orphaned rustcms containers
    print_step "Cleaning up rustcms containers..."
    local containers=$(docker ps -q --filter "name=rustcms" 2>/dev/null || true)
    if [ -n "$containers" ]; then
        echo "$containers" | xargs docker stop 2>/dev/null || true
        echo "$containers" | xargs docker rm 2>/dev/null || true
        print_success "Cleaned up rustcms containers"
    fi
}

# Function to check and report final status
check_final_status() {
    print_step "Checking final status..."
    
    local issues=0
    
    # Check for remaining Rust processes
    if pgrep -f "cargo-watch\|cargo run\|target/debug/backend\|my_rust_cms" >/dev/null 2>&1; then
        print_warning "Some Rust processes may still be running"
        issues=$((issues + 1))
    fi
    
    # Check for remaining frontend processes
    if pgrep -f "trunk serve\|trunk build" >/dev/null 2>&1; then
        print_warning "Some frontend processes may still be running"
        issues=$((issues + 1))
    fi
    
    # Check for remaining Docker containers
    if docker ps --filter "name=rustcms" --format "table {{.Names}}" | grep -q rustcms 2>/dev/null; then
        print_warning "Some rustcms Docker containers may still be running"
        issues=$((issues + 1))
    fi
    
    if [ $issues -eq 0 ]; then
        print_success "All services stopped cleanly"
    else
        print_warning "Some processes may still be running. Check manually if needed."
    fi
}

# Main execution
main() {
    print_header
    
    # Stop all process types
    stop_processes "cargo-watch" "cargo-watch processes"
    stop_processes "cargo run" "cargo run processes"
    stop_processes "target/debug/backend" "backend processes"
    stop_processes "my_rust_cms" "CMS processes"
    stop_processes "trunk serve" "trunk serve processes"
    stop_processes "trunk build" "trunk build processes"
    
    # Stop Docker services
    stop_docker_services
    
    # Final status check
    check_final_status
    
    echo ""
    print_success "🎉 Development environment stopped successfully!"
    echo -e "${CYAN}💡 Run ${GREEN}./dev-start.sh${CYAN} to start the development environment again${NC}"
    echo ""
}

# Run main function
main "$@"