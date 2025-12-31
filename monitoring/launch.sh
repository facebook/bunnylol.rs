#!/usr/bin/env bash
# Helper script to launch bunnylol monitoring stack with password from config

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get bunnylol config path
CONFIG_PATH="${XDG_CONFIG_HOME:-$HOME/.config}/bunnylol/config.toml"

echo -e "${GREEN}Bunnylol Monitoring Stack Launcher${NC}"
echo ""

# Check if config file exists
if [ ! -f "$CONFIG_PATH" ]; then
    echo -e "${RED}Error: Config file not found at $CONFIG_PATH${NC}"
    echo ""
    echo "Create a config file with:"
    echo ""
    echo "[monitoring]"
    echo "grafana_password = \"your_password_here\""
    echo ""
    exit 1
fi

# Extract grafana_password from TOML
# This uses grep and sed to parse the TOML file
PASSWORD=$(grep -A 5 '^\[monitoring\]' "$CONFIG_PATH" 2>/dev/null | \
           grep 'grafana_password' | \
           sed 's/.*grafana_password.*=.*"\([^"]*\)".*/\1/' || true)

# Check if password was found
if [ -z "$PASSWORD" ]; then
    echo -e "${YELLOW}Warning: No grafana_password found in config${NC}"
    echo ""
    echo "Add to $CONFIG_PATH:"
    echo ""
    echo "[monitoring]"
    echo "grafana_password = \"your_password_here\""
    echo ""
    echo -e "${YELLOW}Using default password 'admin'${NC}"
    PASSWORD="admin"
fi

# Export for docker-compose
export GRAFANA_PASSWORD="$PASSWORD"

# Parse command line arguments
COMMAND="${1:-up}"
shift || true

echo -e "${GREEN}Launching monitoring stack...${NC}"
echo ""

# Launch docker compose with monitoring profile
docker compose --profile monitoring "$COMMAND" "$@"
