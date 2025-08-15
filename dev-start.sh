#!/bin/bash

# Simple Rust CMS Development Setup

set -e

echo "🚀 Starting Rust CMS development environment..."

# Load environment variables
if [ -f dev.env ]; then
    export $(cat dev.env | grep -v '^#' | xargs)
fi

# Function to cleanup on exit
cleanup() {
    echo "🛑 Stopping development environment..."
    pkill -f "cargo-watch" 2>/dev/null || true
    pkill -f "trunk serve" 2>/dev/null || true
    docker-compose -f docker-compose.dev.yml down
    exit 0
}

trap cleanup SIGINT SIGTERM

# Start database
echo "📊 Starting database..."
docker-compose -f docker-compose.dev.yml up -d

# Wait for database
echo "⏳ Waiting for database..."
sleep 5

# Run migrations
if [ -d "backend" ]; then
    cd backend
    echo "🗄️  Running migrations..."
    diesel migration run 2>/dev/null || true
    cd ..
fi

# Start backend with hot reload
echo "🦀 Starting backend with hot reload..."
cd backend
cargo-watch -x 'run' &
cd ..

# Start frontend with hot reload  
echo "🎨 Starting frontend with hot reload..."
cd frontend
trunk serve --release --address 127.0.0.1 --port 8080 --open &
cd ..

echo ""
echo "✅ Development environment ready!"
echo "   Frontend: http://localhost:8080"
echo "   Backend:  http://localhost:8081"
echo "   Database: postgresql://rustcms:password@localhost:5432/my_rust_cms"
echo ""
echo "Press Ctrl+C to stop"

# Keep running
wait
