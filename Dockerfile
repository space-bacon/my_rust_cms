# Stage 1: Build the WebAssembly (WASM) frontend
FROM rust:1.69 AS builder

# Install necessary tools
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli

# Create a new directory for the app
WORKDIR /app

# Copy the project files to the container
COPY . .

# Build the frontend
RUN cargo build --target wasm32-unknown-unknown --release
RUN wasm-bindgen --out-dir ./static --target web ./target/wasm32-unknown-unknown/release/my_rust_cms.wasm

# Stage 2: Build the backend and combine with frontend assets
FROM rust:1.69

# Create a new directory for the app
WORKDIR /app

# Copy the project files and build artifacts from the previous stage
COPY --from=builder /app/target/release/my_rust_cms .
COPY --from=builder /app/static ./static

# Install necessary system dependencies (if any)
RUN apt-get update && apt-get install -y libssl-dev pkg-config

# Expose the port that your Rust server listens on
EXPOSE 8080

# Set the entrypoint to run your application
CMD ["./my_rust_cms"]
