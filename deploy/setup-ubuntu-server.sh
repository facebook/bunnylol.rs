#!/bin/bash

#############################################################################
# Ubuntu Server Setup Script for bunnylol.rs
#############################################################################
# This script sets up Docker and deploys bunnylol.rs on a fresh Ubuntu server
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
REPO_URL="https://github.com/alichtman/bunnylol.rs.git"
INSTALL_DIR="${INSTALL_DIR:-$HOME/bunnylol.rs}"

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
    log_success "System packages updated"
}

install_prerequisites() {
    log_info "Installing prerequisites..."
    apt install -y ca-certificates curl
    log_success "Prerequisites installed"
}

install_docker() {
    # Check if Docker is already installed
    if command -v docker &> /dev/null; then
        log_warning "Docker is already installed ($(docker --version))"
        return 0
    fi

    log_info "Installing Docker..."

    # Add Docker's official GPG key
    log_info "Adding Docker's official GPG key..."
    install -m 0755 -d /etc/apt/keyrings
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
    chmod a+r /etc/apt/keyrings/docker.asc

    # Add Docker repository to Apt sources
    log_info "Adding Docker repository..."
    cat > /etc/apt/sources.list.d/docker.sources <<EOF
Types: deb
URIs: https://download.docker.com/linux/ubuntu
Suites: $(. /etc/os-release && echo "${UBUNTU_CODENAME:-$VERSION_CODENAME}")
Components: stable
Signed-By: /etc/apt/keyrings/docker.asc
EOF

    # Update package index with Docker packages
    log_info "Updating package index..."
    apt update

    # Install Docker Engine and plugins
    log_info "Installing Docker Engine and plugins..."
    apt install -y \
        docker-ce \
        docker-ce-cli \
        containerd.io \
        docker-buildx-plugin \
        docker-compose-plugin

    log_success "Docker installed successfully"
}

configure_docker() {
    log_info "Configuring Docker service..."

    # Enable Docker service to start on boot
    systemctl enable docker

    # Start Docker service
    systemctl start docker

    # Verify Docker is running
    if systemctl is-active --quiet docker; then
        log_success "Docker service is running"
    else
        log_error "Docker service failed to start"
        exit 1
    fi
}

clone_or_update_repo() {
    log_info "Setting up bunnylol.rs repository..."

    if [ -d "$INSTALL_DIR" ]; then
        log_warning "Directory $INSTALL_DIR already exists"
        log_info "Updating repository..."
        cd "$INSTALL_DIR"
        git pull
    else
        log_info "Cloning repository to $INSTALL_DIR..."
        git clone "$REPO_URL" "$INSTALL_DIR"
        cd "$INSTALL_DIR"
    fi

    log_success "Repository ready at $INSTALL_DIR"
}

deploy_application() {
    log_info "Deploying application with Docker Compose..."

    cd "$INSTALL_DIR"

    # Deploy/update the application
    # docker compose up -d performs rolling updates if containers are already running
    log_info "Starting containers (or updating if already running)..."
    docker compose up -d --build

    # Wait a moment for containers to start
    sleep 3

    # Check container status
    if docker compose ps | grep -q "Up"; then
        log_success "Application deployed successfully!"
        echo ""
        docker compose ps
    else
        log_error "Application deployment may have failed"
        log_info "Container status:"
        docker compose ps
        exit 1
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
    echo "  - View logs:       cd $INSTALL_DIR && docker compose logs -f"
    echo "  - Stop app:        cd $INSTALL_DIR && docker compose down"
    echo "  - Start app:       cd $INSTALL_DIR && docker compose up -d"
    echo "  - Restart app:     cd $INSTALL_DIR && docker compose restart"
    echo "  - View status:     cd $INSTALL_DIR && docker compose ps"
    echo "  - Update & redeploy: cd $INSTALL_DIR && git pull && docker compose up -d --build"
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
    install_docker
    configure_docker
    clone_or_update_repo
    deploy_application
    show_completion_message
}

# Run main function
main
