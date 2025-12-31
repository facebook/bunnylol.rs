# Multi-stage build for bunnylol.rs
# Stage 1: Build the application
FROM rust:1.91.0-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to cache dependencies
RUN mkdir -p src

# Copy source code
COPY src ./src

# Build the application (server only)
RUN cargo build --release --no-default-features --features server

# Stage 2: Runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 curl && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from builder
COPY --from=builder /app/target/release/bunnylol /app/bunnylol

# Create bunnylol user and system config directory
RUN useradd -m -u 1000 bunnylol && \
    mkdir -p /etc/bunnylol

# Copy Docker-specific config template
COPY config.toml.docker /etc/bunnylol/config.toml

# Set ownership
RUN chown -R bunnylol:bunnylol /app /etc/bunnylol

USER bunnylol

# Run the application
# Config file at /etc/bunnylol/config.toml sets address to 0.0.0.0 for Docker
CMD ["/app/bunnylol", "serve"]
