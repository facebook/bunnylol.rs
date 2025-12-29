#!/bin/bash
# Setup script for automatic deployment of bunnylol.rs
# This script configures auto-deployment to check for updates every 5 minutes

set -e

# Derive repository directory from script location
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
LOG_FILE="${LOG_FILE:-/var/log/bunnylol-deploy.log}"
BRANCH="${BRANCH:-main}"
CRON_SCHEDULE="${CRON_SCHEDULE:-*/5 * * * *}" # Every 5 minutes by default

echo -e "${GREEN}=== Bunnylol Auto-Deployment Setup ===${NC}\n"

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo -e "${RED}Error: This script must be run as root (use sudo)${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} Repository detected at: $REPO_DIR"

# Check if docker-compose.yml exists
if [ ! -f "$REPO_DIR/docker-compose.yml" ]; then
    echo -e "${RED}Error: docker-compose.yml not found in $REPO_DIR${NC}"
    exit 1
fi
echo -e "${GREEN}✓${NC} Found docker-compose.yml"

# Check if auto-deploy.sh exists
if [ ! -f "$REPO_DIR/deploy/auto-deploy.sh" ]; then
    echo -e "${RED}Error: auto-deploy.sh not found in $REPO_DIR/deploy/${NC}"
    exit 1
fi
echo -e "${GREEN}✓${NC} Found auto-deploy.sh"

# Make auto-deploy.sh executable
chmod +x "$REPO_DIR/deploy/auto-deploy.sh"
echo -e "${GREEN}✓${NC} Made auto-deploy.sh executable"

# Create log directory if it doesn't exist
LOG_DIR=$(dirname "$LOG_FILE")
if [ ! -d "$LOG_DIR" ]; then
    mkdir -p "$LOG_DIR"
    echo -e "${GREEN}✓${NC} Created log directory: $LOG_DIR"
fi

# Create log file with proper permissions
touch "$LOG_FILE"
chmod 644 "$LOG_FILE"
echo -e "${GREEN}✓${NC} Created log file: $LOG_FILE"

# Configure git to avoid asking for credentials (useful for cron)
cd "$REPO_DIR"
git config pull.rebase false
echo -e "${GREEN}✓${NC} Configured git settings"

# Set up cron job
AUTO_DEPLOY_SCRIPT="$REPO_DIR/deploy/auto-deploy.sh"
CRON_COMMAND="LOG_FILE=$LOG_FILE BRANCH=$BRANCH $AUTO_DEPLOY_SCRIPT >> $LOG_FILE 2>&1"
CRON_JOB="$CRON_SCHEDULE $CRON_COMMAND"

# Check if cron job already exists
if crontab -l 2>/dev/null | grep -q "auto-deploy.sh"; then
    echo -e "${YELLOW}⚠${NC}  Cron job already exists. Updating..."
    # Remove existing cron job
    crontab -l 2>/dev/null | grep -v "auto-deploy.sh" | crontab -
fi

# Add new cron job
(crontab -l 2>/dev/null; echo "$CRON_JOB") | crontab -
echo -e "${GREEN}✓${NC} Added cron job: $CRON_SCHEDULE"

echo -e "\n${GREEN}=== Setup Complete! ===${NC}\n"
echo "Configuration:"
echo "  Repository: $REPO_DIR"
echo "  Branch: $BRANCH"
echo "  Log file: $LOG_FILE"
echo "  Check interval: $CRON_SCHEDULE (every 5 minutes)"
echo ""
echo "The auto-deployment script will now:"
echo "  1. Check for updates every 5 minutes"
echo "  2. Pull and redeploy if changes are detected"
echo "  3. Log all activity to $LOG_FILE"
echo ""
echo "Useful commands:"
echo "  View logs:        tail -f $LOG_FILE"
echo "  List cron jobs:   crontab -l"
echo "  Remove cron job:  crontab -e (then delete the auto-deploy line)"
echo "  Manual run:       $AUTO_DEPLOY_SCRIPT"
echo ""
echo -e "${YELLOW}Running initial deployment...${NC}\n"
LOG_FILE="$LOG_FILE" BRANCH="$BRANCH" "$AUTO_DEPLOY_SCRIPT"
