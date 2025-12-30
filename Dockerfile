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

# Build the application
RUN cargo build --release

# Stage 2: Runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 curl && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from builder
COPY --from=builder /app/target/release/bunnylol /app/bunnylol

# Create a non-root user
RUN useradd -m -u 1000 bunnylol && \
    chown -R bunnylol:bunnylol /app

USER bunnylol

# Set environment variables
ENV ROCKET_ADDRESS=0.0.0.0

# Run the application
CMD ["/app/bunnylol", "serve"]
