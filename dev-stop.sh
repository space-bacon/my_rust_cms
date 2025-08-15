#!/bin/bash

# Stop Rust CMS Development Environment

echo "🛑 Stopping Rust CMS development environment..."

# Stop backend processes
echo "🦀 Stopping backend..."
pkill -f "cargo-watch" 2>/dev/null || true
pkill -f "cargo run" 2>/dev/null || true
pkill -f "target/debug/backend" 2>/dev/null || true

# Stop frontend processes
echo "🎨 Stopping frontend..."
pkill -f "trunk serve" 2>/dev/null || true

# Stop Docker services
echo "📊 Stopping database..."
docker-compose -f docker-compose.dev.yml down

echo "✅ Development environment stopped!"
echo "Run ./dev-start.sh to start again."
