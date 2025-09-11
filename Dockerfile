# Multi-stage Dockerfile for My Rust CMS

# Stage 1: Build the Rust backend
FROM rustlang/rust:nightly-slim as backend-builder

# Install system dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy backend source code and necessary files
COPY backend/ ./backend/
COPY src/ ./src/
COPY Cargo.toml ./
COPY Cargo.lock ./
COPY Diesel.toml ./
COPY migrations/ ./migrations/
COPY docs/ ./docs/

# Create a dummy frontend directory to satisfy workspace requirements
RUN mkdir -p frontend/src && \
    echo '[package]\nname = "frontend"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\nname = "frontend"' > frontend/Cargo.toml && \
    echo 'pub fn dummy() {}' > frontend/src/lib.rs

# Build the backend application from the workspace root
RUN cargo build --release -p backend

# Stage 2: Use pre-built frontend files
# Since we already have built frontend files in dist/, we'll use them directly
# This avoids the complexity of building WASM in Docker

# Stage 3: Runtime image
FROM rustlang/rust:nightly-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create application user
RUN groupadd -r myrustcms && useradd -r -g myrustcms myrustcms

# Set working directory
WORKDIR /app

# Copy the built backend binary
COPY --from=backend-builder /app/target/release/backend ./backend

# Copy the pre-built frontend files from host
COPY dist/ ./dist/

# Copy necessary runtime files
COPY docs/ ./docs/
COPY migrations/ ./migrations/
COPY scripts/ ./scripts/

# Create uploads directory and set permissions
RUN mkdir -p uploads && chown -R myrustcms:myrustcms /app

# Switch to non-root user
USER myrustcms

# Expose port
EXPOSE 8081

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8081/health || exit 1

# Start the application
CMD ["./backend"]
