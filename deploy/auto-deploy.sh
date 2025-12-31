#!/bin/bash
# Auto-deployment script for bunnylol.rs
# Checks for upstream changes and redeploys if necessary
#
# This script is designed to be run by cron every 5 minutes.
# It will only rebuild and redeploy if there are new commits on the remote branch, or if a docker container for this service isn't already running.

set -euo pipefail

# Derive repository directory from script location
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"

# Configuration (can be overridden by environment variables)
LOG_FILE="${LOG_FILE:-/var/log/bunnylol-deploy.log}"
BRANCH="${BRANCH:-main}"

# Logging function
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

# Change to repository directory
cd "$REPO_DIR" || {
    log "ERROR: Could not change to directory $REPO_DIR"
    exit 1
}

# Fetch latest changes from remote
log "Checking for updates from origin/$BRANCH..."
git fetch origin "$BRANCH" 2>&1 | tee -a "$LOG_FILE" || {
    log "ERROR: Failed to fetch from remote"
    exit 1
}

# Check if there are updates
LOCAL=$(git rev-parse HEAD)
REMOTE=$(git rev-parse origin/$BRANCH)

# Check if container is running
CONTAINER_RUNNING=false
if docker ps | grep -q bunnylol; then
    CONTAINER_RUNNING=true
fi

if [ "$LOCAL" = "$REMOTE" ]; then
    if [ "$CONTAINER_RUNNING" = true ]; then
        log "No changes detected and container is running. Current commit: $LOCAL"
        exit 0
    else
        log "No changes detected, but container is not running. Redeploying..."
    fi
fi

log "Changes detected! Deploying..."
log "  Local:  $LOCAL"
log "  Remote: $REMOTE"

# Pull the changes
log "Pulling changes..."
git pull origin "$BRANCH" 2>&1 | tee -a "$LOG_FILE" || {
    log "ERROR: Failed to pull changes"
    exit 1
}

# Rebuild and redeploy
log "Rebuilding and redeploying with zero-downtime..."
docker compose up --build -d 2>&1 | tee -a "$LOG_FILE" || {
    log "ERROR: Deployment failed"
    exit 1
}

# Wait for container to be ready
log "Waiting for container to be ready..."
sleep 5

# Health check
log "Running health check..."
if curl -f http://localhost:8000/health > /dev/null 2>&1; then
    log "✓ Health check passed"
else
    log "ERROR: Health check failed!"
    docker compose logs --tail=50 bunnylol | tee -a "$LOG_FILE"
    exit 1
fi

# Verify deployment
if docker ps | grep -q bunnylol; then
    CONTAINER_ID=$(docker ps --filter "name=bunnylol" --format "{{.ID}}")
    CONTAINER_CREATED=$(docker inspect "$CONTAINER_ID" --format='{{.Created}}')
    NEW_COMMIT=$(git rev-parse HEAD)
    log "SUCCESS: Deployment completed"
    log "  New commit: $NEW_COMMIT"
    log "  Container ID: $CONTAINER_ID"
    log "  Created at: $CONTAINER_CREATED"
else
    log "ERROR: Container not running after deployment!"
    docker compose logs --tail=50 bunnylol | tee -a "$LOG_FILE"
    exit 1
fi

log "Auto-deployment completed successfully"
