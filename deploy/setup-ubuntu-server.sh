#!/bin/bash

#############################################################################
# Ubuntu Server Setup Script for bunnylol.rs
#############################################################################
# This script installs Rust, bunnylol, and sets it up as a systemd service
#
# Usage:
#   sudo ./setup-ubuntu-server.sh
#
# Or run directly:
#   curl -fsSL <raw-script-url> | sudo bash
#############################################################################

set -e  # Exit on error
set -u  # Exit on undefined variable

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
RUSTUP_INIT_URL="https://sh.rustup.rs"

#############################################################################
# Helper Functions
#############################################################################

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_root() {
    if [ "$EUID" -ne 0 ]; then
        log_error "This script must be run as root. Please use sudo."
        exit 1
    fi
}

check_ubuntu_version() {
    log_info "Checking Ubuntu version..."

    # Check if running on Ubuntu
    if [ ! -f /etc/os-release ]; then
        log_error "Cannot determine OS version (/etc/os-release not found)"
        exit 1
    fi

    . /etc/os-release

    if [ "$ID" != "ubuntu" ]; then
        log_error "This script is designed for Ubuntu only. Detected OS: $ID"
        exit 1
    fi

    # Extract version number (e.g., "24.04" from "24.04.1 LTS")
    VERSION_NUMBER=$(echo "$VERSION_ID" | cut -d. -f1,2)
    VERSION_MAJOR=$(echo "$VERSION_NUMBER" | cut -d. -f1)
    VERSION_MINOR=$(echo "$VERSION_NUMBER" | cut -d. -f2)

    # Minimum supported version: 22.04
    MIN_MAJOR=22
    MIN_MINOR=4

    # Compare versions
    if [ "$VERSION_MAJOR" -lt "$MIN_MAJOR" ] ||
       ([ "$VERSION_MAJOR" -eq "$MIN_MAJOR" ] && [ "$VERSION_MINOR" -lt "$MIN_MINOR" ]); then
        log_error "Unsupported Ubuntu version: $VERSION_ID"
        echo ""
        echo "This script requires Ubuntu 22.04 (Jammy) or higher:"
        echo "  - Ubuntu 22.04 LTS (Jammy)"
        echo "  - Ubuntu 24.04 LTS (Noble)"
        echo "  - Ubuntu 25.04 (Plucky)"
        echo "  - Ubuntu 25.10 (Questing)"
        echo ""
        echo "Your version: Ubuntu $VERSION_ID ($VERSION_CODENAME)"
        exit 1
    fi

    log_success "Ubuntu $VERSION_ID ($VERSION_CODENAME) - supported ✓"
}

#############################################################################
# Installation Functions
#############################################################################

update_system() {
    log_info "Updating system packages..."
    apt update
    apt upgrade -y
    log_success "System packages updated"
}

install_prerequisites() {
    log_info "Installing prerequisites..."
    apt install -y \
        curl \
        build-essential \
        pkg-config \
        libssl-dev \
        ca-certificates
    log_success "Prerequisites installed"
}

install_rust() {
    # Check if Rust is already installed
    if command -v rustc &> /dev/null; then
        log_warning "Rust is already installed ($(rustc --version))"
        log_info "Updating Rust..."
        sudo -u "${SUDO_USER:-$USER}" rustup update
        return 0
    fi

    log_info "Installing Rust..."

    # Install rustup as the non-root user if run with sudo
    if [ -n "${SUDO_USER:-}" ]; then
        log_info "Installing Rust for user: $SUDO_USER"
        sudo -u "$SUDO_USER" sh -c "curl --proto '=https' --tlsv1.2 -sSf $RUSTUP_INIT_URL | sh -s -- -y"

        # Source cargo env for this script
        export PATH="/home/$SUDO_USER/.cargo/bin:$PATH"
    else
        log_info "Installing Rust for root user"
        curl --proto '=https' --tlsv1.2 -sSf $RUSTUP_INIT_URL | sh -s -- -y

        # Source cargo env for this script
        export PATH="$HOME/.cargo/bin:$PATH"
    fi

    log_success "Rust installed successfully"
}

install_bunnylol() {
    log_info "Installing bunnylol from crates.io..."

    # Install as the non-root user if run with sudo
    if [ -n "${SUDO_USER:-}" ]; then
        sudo -u "$SUDO_USER" bash -c "source ~/.cargo/env && cargo install bunnylol"
    else
        source "$HOME/.cargo/env"
        cargo install bunnylol
    fi

    # Verify installation
    if command -v bunnylol &> /dev/null; then
        log_success "bunnylol installed successfully ($(bunnylol --version))"
    else
        log_error "bunnylol installation failed - binary not found in PATH"
        exit 1
    fi
}

install_service() {
    log_info "Installing bunnylol as systemd service..."

    # The bunnylol binary should be in the user's PATH
    # We need to install the service as root
    bunnylol service install

    log_success "Bunnylol service installed and started"
}

verify_installation() {
    log_info "Verifying installation..."

    # Check if service is running
    if systemctl is-active --quiet bunnylol; then
        log_success "Bunnylol service is running"
    else
        log_error "Bunnylol service is not running"
        systemctl status bunnylol
        exit 1
    fi

    # Wait a moment for the service to be ready
    sleep 2

    # Test the endpoint
    log_info "Testing HTTP endpoint..."
    if curl -f http://localhost:8000/health &> /dev/null; then
        log_success "HTTP endpoint is responding"
    else
        log_warning "HTTP endpoint may not be ready yet"
        log_info "Check status with: sudo bunnylol service status"
    fi
}

show_completion_message() {
    echo ""
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}  Setup Complete!${NC}"
    echo -e "${GREEN}========================================${NC}"
    echo ""
    echo -e "Application is running at: ${BLUE}http://$(hostname -I | awk '{print $1}'):8000${NC}"
    echo ""
    echo "Useful commands:"
    echo "  - View status:     sudo bunnylol service status"
    echo "  - View logs:       sudo bunnylol service logs"
    echo "  - Follow logs:     sudo bunnylol service logs -f"
    echo "  - Restart service: sudo bunnylol service restart"
    echo "  - Stop service:    sudo bunnylol service stop"
    echo "  - Start service:   sudo bunnylol service start"
    echo "  - Uninstall:       sudo bunnylol service uninstall"
    echo ""
    echo "Service details:"
    echo "  - Service file:    /etc/systemd/system/bunnylol.service"
    echo "  - Autostart:       Enabled (starts on boot)"
    echo "  - Running as:      root"
    echo ""
}

#############################################################################
# Main Execution
#############################################################################

main() {
    log_info "Starting Ubuntu Server setup for bunnylol.rs..."
    echo ""

    check_root
    check_ubuntu_version
    update_system
    install_prerequisites
    install_rust
    install_bunnylol
    install_service
    verify_installation
    show_completion_message
}

# Run main function
main
