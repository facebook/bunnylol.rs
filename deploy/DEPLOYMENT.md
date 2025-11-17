# Deployment Guide for Bunnylol

This guide covers deploying bunnylol.rs using Docker.

## Table of Contents

- [Quick Start: Automated Setup](#quick-start-automated-setup)
- [Docker Deployment](#docker-deployment)
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

## Docker Deployment

### Using Docker Compose (Recommended)

The easiest way to deploy bunnylol with Docker is using Docker Compose:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/facebook/bunnylol.rs.git
   cd bunnylol.rs
   ```

2. **Start the service**:
   ```bash
   docker-compose up -d
   ```

3. **Access the application**:
   Open your browser to `http://localhost:8000`

4. **View logs**:
   ```bash
   docker-compose logs -f
   ```

5. **Stop the service**:
   ```bash
   docker-compose down
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
docker-compose logs bunnylol

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
