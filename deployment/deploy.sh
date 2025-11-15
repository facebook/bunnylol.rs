#!/bin/bash
set -euo pipefail

# Bunnylol Deployment Script
# This script automates the deployment of bunnylol on a Linux server

INSTALL_DIR="/var/lib/bunnylol"
SERVICE_USER="bunnylol"
BINARY_NAME="bunnylol"
BUILD_DIR="target/release"

echo "========================================="
echo "Bunnylol Deployment Script"
echo "========================================="
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "Error: This script must be run as root (use sudo)"
    exit 1
fi

# Check for Debian/Ubuntu-based system
if ! command -v apt-get &> /dev/null; then
    echo "Error: This script requires a Debian/Ubuntu-based system (apt-get not found)"
    echo "For other distributions, please install dependencies manually and build with 'cargo build --release'"
    exit 1
fi

# Install Rust if not already installed
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    # Export cargo path for current session
    export PATH="$HOME/.cargo/bin:$PATH"

    # Verify installation
    if ! command -v cargo &> /dev/null; then
        echo "Error: Rust installation failed"
        exit 1
    fi
    echo "Rust installed successfully"
else
    echo "Rust is already installed ($(rustc --version))"
fi

# Install build dependencies
echo "Installing build dependencies..."
apt-get update
apt-get install -y pkg-config libssl-dev ca-certificates curl

# Build the application
echo "Building bunnylol in release mode..."
cargo build --release

# Validate build succeeded
if [ ! -f "$BUILD_DIR/$BINARY_NAME" ]; then
    echo "Error: Build failed - binary not found at $BUILD_DIR/$BINARY_NAME"
    exit 1
fi

echo "Build successful! Binary size: $(du -h "$BUILD_DIR/$BINARY_NAME" | cut -f1)"

# Create service user if it doesn't exist
if ! id "$SERVICE_USER" &>/dev/null; then
    echo "Creating service user: $SERVICE_USER"
    useradd -r -s /bin/false -d "$INSTALL_DIR" "$SERVICE_USER"
else
    echo "Service user $SERVICE_USER already exists"
fi

# Create installation directory
echo "Creating installation directory: $INSTALL_DIR"
mkdir -p "$INSTALL_DIR"

# Stop service if it's running
if systemctl is-active --quiet bunnylol.service; then
    echo "Stopping existing bunnylol service..."
    systemctl stop bunnylol.service
fi

# Copy binary to installation directory
echo "Installing binary to $INSTALL_DIR..."
cp "$BUILD_DIR/$BINARY_NAME" "$INSTALL_DIR/"
chown -R "$SERVICE_USER:$SERVICE_USER" "$INSTALL_DIR"
chmod 755 "$INSTALL_DIR/$BINARY_NAME"

# Install systemd service
echo "Installing systemd service..."
cp deploy/bunnylol.service /etc/systemd/system/
systemctl daemon-reload

# Enable and start the service
echo "Enabling and starting bunnylol service..."
systemctl enable bunnylol.service
systemctl start bunnylol.service

# Wait a moment for service to start
sleep 2

# Validate service started successfully
if ! systemctl is-active --quiet bunnylol.service; then
    echo ""
    echo "========================================="
    echo "ERROR: Service failed to start!"
    echo "========================================="
    echo ""
    echo "Service status:"
    systemctl status bunnylol.service --no-pager || true
    echo ""
    echo "Recent logs:"
    journalctl -u bunnylol -n 50 --no-pager
    exit 1
fi

# Check service status
echo ""
echo "========================================="
echo "Deployment Complete!"
echo "========================================="
echo ""
echo "Service Status:"
systemctl status bunnylol.service --no-pager
echo ""
echo "✓ Bunnylol is running and accessible at http://localhost:8000"
echo ""
echo "Useful commands:"
echo "  View logs:        journalctl -u bunnylol -f"
echo "  Restart service:  systemctl restart bunnylol"
echo "  Stop service:     systemctl stop bunnylol"
echo "  Service status:   systemctl status bunnylol"
echo ""
