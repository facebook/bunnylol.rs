# Deployment Guide for Bunnylol

This guide covers deploying `bunnylol.rs` using either native service installation or Docker.

## Table of Contents

- [Quick Start: Automated Setup](#quick-start-automated-setup)
- [Native Service Installation](#native-service-installation)
- [Docker Deployment](#docker-deployment)
- [Rebuilding and Redeploying](#rebuilding-and-redeploying)
- [Auto-Deployment](#auto-deployment)
- [Configuration](#configuration)
- [Running on Boot](#running-on-boot)
- [Reverse Proxy Setup](#reverse-proxy-setup)
- [Troubleshooting](#troubleshooting)

## Quick Start: Automated Setup

For new Ubuntu cloud machines (Ubuntu 22.04+), we provide an automated setup script that installs Docker and deploys bunnylol.rs in one command.

### Prerequisites

- Fresh Ubuntu server (22.04 LTS, 24.04 LTS, 25.04, or 25.10+)
- Root or sudo access
- Internet connection

### Usage

Download and run the setup script:

```bash
curl -fsSL https://raw.githubusercontent.com/alichtman/bunnylol.rs/main/deploy/setup-ubuntu-server.sh -o setup-ubuntu-server.sh
chmod +x setup-ubuntu-server.sh
sudo ./setup-ubuntu-server.sh
```

Or clone the repository first and run locally:

```bash
git clone https://github.com/alichtman/bunnylol.rs.git
cd bunnylol.rs
sudo deploy/setup-ubuntu-server.sh
```

### What the Script Does

The automated setup script will:

1. ✓ Verify you're running a supported Ubuntu version
2. ✓ Update system packages
3. ✓ Install Docker CE and Docker Compose
4. ✓ Configure Docker to start on boot
5. ✓ Clone the bunnylol.rs repository
6. ✓ Deploy the application with `docker compose up -d --build`

After completion, bunnylol will be running on port 8000. The script is safe to run multiple times - it will skip already-installed components and update the repository if it already exists.

---

## Native Service Installation

The recommended deployment method for Linux and macOS is to install bunnylol as a native system service. This provides better integration with your OS and doesn't require Docker.

### Prerequisites

- Rust (only needed if binary not in PATH)
- Linux (systemd) or macOS (launchd) or Windows (Service Manager)
- sudo/root access (for system-level installation)

### Installation

#### System-Level Service (Recommended)

System-level installation runs the server as a dedicated service accessible across the network:

```bash
# Install bunnylol first
$ cargo install bunnylol

# Install as system service (requires sudo)
$ sudo bunnylol install-server --system

# The installer will:
# - Build the binary if needed
# - Create service files (systemd/launchd/Windows Service)
# - Configure autostart on boot
# - Start the service immediately
```

Default configuration:
- **Port**: 8000
- **Address**: 0.0.0.0 (accessible from network)
- **Autostart**: Enabled
- **Service user**: root (system-level) or current user (user-level)

#### User-Level Service

User-level installation runs as your current user (localhost only):

```bash
$ bunnylol install-server

# Default configuration for user-level:
# - Port: 8000
# - Address: 127.0.0.1 (localhost only)
# - Runs as current user
# - Autostart: Enabled
```

### Managing the Service

```bash
# Check service status
$ sudo bunnylol server status --system

# View logs (follow mode)
$ sudo bunnylol server logs --system -f

# Restart the service
$ sudo bunnylol server restart --system

# Stop the service
$ sudo bunnylol server stop --system

# Start the service
$ sudo bunnylol server start --system
```

For user-level services, omit `--system` and `sudo`:
```bash
$ bunnylol server status
$ bunnylol server logs -f
```

### Custom Configuration

Customize port, address, and other settings during installation:

```bash
# Custom port
$ sudo bunnylol install-server --system --port 9000

# Custom address (e.g., localhost only for system service)
$ sudo bunnylol install-server --system --address 127.0.0.1

# Don't autostart on boot
$ sudo bunnylol install-server --system --no-autostart

# Install but don't start immediately
$ sudo bunnylol install-server --system --no-start
```

### Uninstalling

```bash
# Uninstall system service
$ sudo bunnylol uninstall-server --system

# Uninstall user service
$ bunnylol uninstall-server
```

### Platform-Specific Details

#### Linux (systemd)

- Service file: `/etc/systemd/system/bunnylol.service` (system) or `~/.config/systemd/user/bunnylol.service` (user)
- Logs: `journalctl -u bunnylol -f` or `sudo bunnylol server logs --system -f`
- Binary location: `/usr/local/bin/bunnylol` (system) or `~/.local/bin/bunnylol` (user)

#### macOS (launchd)

- Service file: `/Library/LaunchDaemons/com.facebook.bunnylol.plist` (system) or `~/Library/LaunchAgents/com.facebook.bunnylol.plist` (user)
- Logs: Use Console.app or `sudo bunnylol server logs --system`
- Binary location: `/usr/local/bin/bunnylol` (system) or `~/.local/bin/bunnylol` (user)

#### Windows (Service Manager)

- Managed through Windows Service Manager
- Binary location: `C:\Program Files\bunnylol\` (system) or `%USERPROFILE%\.local\bin\` (user)

---

## Docker Deployment

Docker provides an alternative deployment method that's useful for containerized environments.

### Using Docker Compose

The easiest way to deploy bunnylol with Docker is using Docker Compose:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/facebook/bunnylol.rs.git
   cd bunnylol.rs
   ```

2. **Start the service**:
   ```bash
   docker compose up -d
   BUNNYLOL_PORT=9000 docker compose up -d
   ```

3. **Access the application**:
   Open your browser to `http://localhost:8000`

4. **View logs**:
   ```bash
   docker compose logs -f
   ```

5. **Stop the service**:
   ```bash
   docker compose down
   ```

### Using Docker directly

1. **Build the image**:
   ```bash
   docker build -t bunnylol .
   ```

2. **Run the container**:
   ```bash
   docker run -d \
     --name bunnylol \
     -p 8000:8000 \
     --restart unless-stopped \
     bunnylol
   ```

## Rebuilding and Redeploying

When you've made code changes and need to deploy them to your running server:

### Quick Rebuild (Recommended)

The simplest way to rebuild and redeploy:

```bash
docker compose up --build -d
```

This command will:
- Build a new image with your latest changes
- Stop the old container
- Start a new container with the updated image

### Full Rebuild (Clean Build)

If you need to rebuild without using cached layers:

```bash
docker compose down
docker compose build --no-cache
docker compose up -d
```

### Remote Server Rebuild

If your server is running on a remote machine (e.g., Hetzner, AWS, etc.):

1. **SSH into your server and navigate to the project directory**:
   ```bash
   ssh your-server
   cd bunnylol.rs
   ```

2. **Pull the latest changes** (if using Git):
   ```bash
   git pull
   ```

3. **Rebuild and redeploy**:
   ```bash
   docker compose up --build -d
   ```

4. **Verify the deployment**:
   ```bash
   docker ps
   docker logs --tail=20 bunnylol
   ```

### One-Liner for Remote Rebuild

If you have SSH configured with a host alias (e.g., `hetzner`), you can rebuild from your local machine:

```bash
ssh your-server "cd bunnylol.rs && git pull && docker compose up --build -d"
```

### Verifying the Deployment

After rebuilding, check that:
- The container was created recently: `docker ps` (check CREATED column)
- The application is running: `curl http://localhost:8000/health`
- Logs look healthy: `docker logs --tail=50 bunnylol`

## Auto-Deployment

For production servers, you can set up automatic deployment that checks for upstream changes and redeploys automatically.

### How It Works

The auto-deployment system:
1. Checks for new commits on the remote repository every 5 minutes
2. If changes are detected, pulls them and rebuilds the Docker container
3. Logs all activity to `/var/log/bunnylol-deploy.log`
4. Only rebuilds when there are actual changes (no unnecessary rebuilds)

### Setup

On your server, run the setup script:

```bash
sudo /path/to/bunnylol.rs/deploy/setup-auto-deploy.sh
```

Or if using an SSH alias:

```bash
ssh your-server "sudo /root/bunnylol.rs/deploy/setup-auto-deploy.sh"
```

This will:
- Make the auto-deploy script executable
- Create the log file and directory
- Configure git settings for automated pulling
- Set up a cron job to run every 5 minutes
- Test the deployment script

### Customization

You can customize the behavior with environment variables:

```bash
# Check every 10 minutes instead of 5
CRON_SCHEDULE="*/10 * * * *" sudo deploy/setup-auto-deploy.sh

# Use a different branch
BRANCH="production" sudo deploy/setup-auto-deploy.sh

# Custom log location
LOG_FILE="/var/log/custom-deploy.log" sudo deploy/setup-auto-deploy.sh
```

### Monitoring

**View deployment logs:**
```bash
tail -f /var/log/bunnylol-deploy.log
```

**Check cron job status:**
```bash
crontab -l
```

**Manually trigger deployment:**
```bash
sudo /path/to/bunnylol.rs/deploy/auto-deploy.sh
```

### Removing Auto-Deployment

To disable auto-deployment:

```bash
# Remove the cron job
crontab -e
# Delete the line containing "auto-deploy.sh"
```

## Configuration

### Environment Variables

You can customize the deployment by setting environment variables in a `.env` file:

```bash
# .env
BUNNYLOL_PORT=8000
ROCKET_LOG_LEVEL=normal
```

Bunnylol supports these environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `BUNNYLOL_PORT` | Port to listen on | `8000` |
| `ROCKET_LOG_LEVEL` | Logging level (normal, debug, critical) | `normal` |

## Running on Boot

Docker containers can automatically start on system boot using restart policies.

### Enable Docker Service

First, ensure the Docker daemon starts on boot:
```bash
sudo systemctl enable docker
```

### Restart Policies

When running containers, use a restart policy:

**With Docker Compose** (add to your `docker-compose.yml`):
```yaml
services:
  bunnylol:
    restart: unless-stopped
```

**With Docker run**:
```bash
docker run -d --restart unless-stopped ...
```

Available restart policies:
- `always`: Always restart, even if manually stopped and system reboots
- `unless-stopped`: Restart unless explicitly stopped by user
- `on-failure`: Only restart on crashes

## Reverse Proxy Setup

For production deployments with HTTPS, use a reverse proxy like Caddy or nginx.

> TODO: Finish this section

<!-- ### Using Caddy (Easiest for HTTPS) -->
<!---->
<!-- 1. **Install Caddy**: -->
<!--    ```bash -->
<!--    sudo apt install -y debian-keyring debian-archive-keyring apt-transport-https curl -->
<!--    curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg -->
<!--    curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/caddy-stable.list -->
<!--    sudo apt update -->
<!--    sudo apt install caddy -->
<!--    ``` -->
<!---->
<!-- 2. **Configure Caddy**: -->
<!--    ```bash -->
<!--    sudo nano /etc/caddy/Caddyfile -->
<!--    ``` -->
<!---->
<!--    Add: -->
<!--    ``` -->
<!--    your-domain.com { -->
<!--        reverse_proxy localhost:8000 -->
<!--    } -->
<!--    ``` -->
<!---->
<!-- 3. **Restart Caddy**: -->
<!--    ```bash -->
<!--    sudo systemctl restart caddy -->
<!--    ``` -->
<!---->
<!-- Caddy automatically handles SSL certificates via Let's Encrypt. -->
<!---->
<!-- ### Using nginx -->
<!---->
<!-- 1. **Install nginx**: -->
<!--    ```bash -->
<!--    sudo apt install nginx -->
<!--    ``` -->
<!---->
<!-- 2. **Configure nginx**: -->
<!--    ```bash -->
<!--    sudo nano /etc/nginx/sites-available/bunnylol -->
<!--    ``` -->
<!---->
<!--    Add: -->
<!--    ```nginx -->
<!--    server { -->
<!--        listen 80; -->
<!--        server_name your-domain.com; -->
<!---->
<!--        location / { -->
<!--            proxy_pass http://localhost:8000; -->
<!--            proxy_set_header Host $host; -->
<!--            proxy_set_header X-Real-IP $remote_addr; -->
<!--        } -->
<!--    } -->
<!--    ``` -->
<!---->
<!-- 3. **Enable and restart**: -->
<!--    ```bash -->
<!--    sudo ln -s /etc/nginx/sites-available/bunnylol /etc/nginx/sites-enabled/ -->
<!--    sudo systemctl restart nginx -->
<!--    ``` -->

## Troubleshooting

### Docker Issues

**Container won't start:**
```bash
# Check logs
docker compose logs bunnylol

# Check if port is already in use
sudo netstat -tlnp | grep 8000
```

**Permission denied errors:**
```bash
# Add your user to docker group
sudo usermod -aG docker $USER
# Log out and back in
```

**Container not starting on boot:**
```bash
# Verify Docker service is enabled
sudo systemctl status docker

# Check restart policy
docker inspect bunnylol | grep -A 3 RestartPolicy
```

### Build Issues

**Docker build fails:**
```bash
# Clean build cache
docker system prune -a

# Rebuild without cache
docker build --no-cache -t bunnylol .
```

## Support

For issues or questions:
- GitHub Issues: https://github.com/facebook/bunnylol.rs/issues
