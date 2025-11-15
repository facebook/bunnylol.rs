# Deployment Guide for Bunnylol

This guide covers multiple deployment options for bunnylol.rs, from containerized deployments to bare-metal Linux installations.

## Table of Contents

- [Docker Deployment](#docker-deployment)
- [Bare-Metal Linux Deployment](#bare-metal-linux-deployment)
- [Deployment on Hetzner](#deployment-on-hetzner)
- [Deployment on AWS](#deployment-on-aws)
- [Heroku Deployment](#heroku-deployment)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)

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

### Environment Variables for Docker

You can customize the deployment by setting environment variables in a `.env` file:

```bash
# .env
PORT=8000
ROCKET_LOG_LEVEL=normal
```

## Bare-Metal Linux Deployment

For deploying directly on a Linux server (Ubuntu/Debian), use the automated deployment script:

### Automated Deployment

1. **Clone the repository**:
   ```bash
   git clone https://github.com/facebook/bunnylol.rs.git
   cd bunnylol.rs
   ```

2. **Run the deployment script**:
   ```bash
   sudo ./deploy/deploy.sh
   ```

This script will:
- Install Rust and dependencies
- Build the application in release mode
- Create a dedicated service user
- Install the systemd service
- Start bunnylol automatically

3. **Manage the service**:
   ```bash
   # Check status
   sudo systemctl status bunnylol

   # View logs
   sudo journalctl -u bunnylol -f

   # Restart service
   sudo systemctl restart bunnylol

   # Stop service
   sudo systemctl stop bunnylol
   ```

### Manual Deployment

If you prefer to deploy manually:

1. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Install dependencies**:
   ```bash
   sudo apt-get update
   sudo apt-get install -y pkg-config libssl-dev
   ```

3. **Build the application**:
   ```bash
   cargo build --release
   ```

4. **Create installation directory**:
   ```bash
   sudo mkdir -p /opt/bunnylol
   sudo cp target/release/bunnylol /opt/bunnylol/
   ```

5. **Create service user**:
   ```bash
   sudo useradd -r -s /bin/false -m -d /opt/bunnylol bunnylol
   sudo chown -R bunnylol:bunnylol /opt/bunnylol
   ```

6. **Install systemd service**:
   ```bash
   sudo cp deploy/bunnylol.service /etc/systemd/system/
   sudo systemctl daemon-reload
   sudo systemctl enable bunnylol
   sudo systemctl start bunnylol
   ```

## Deployment on Hetzner

Hetzner Cloud provides affordable VPS options. Here's how to deploy:

### Using Hetzner Cloud Console

1. **Create a new server**:
   - Choose Ubuntu 22.04 or 24.04
   - Select your preferred instance size (CX11 is sufficient for light usage)
   - Add your SSH key

2. **Connect to your server**:
   ```bash
   ssh root@<your-server-ip>
   ```

3. **Deploy using Docker** (Recommended):
   ```bash
   # Install Docker
   curl -fsSL https://get.docker.com -o get-docker.sh
   sh get-docker.sh

   # Install Docker Compose
   apt-get install -y docker-compose

   # Clone and deploy
   git clone https://github.com/facebook/bunnylol.rs.git
   cd bunnylol.rs
   docker-compose up -d
   ```

4. **Or deploy bare-metal**:
   ```bash
   git clone https://github.com/facebook/bunnylol.rs.git
   cd bunnylol.rs
   ./deploy/deploy.sh
   ```

5. **Set up a reverse proxy** (Optional but recommended):

   Install and configure nginx or Caddy for HTTPS:

   ```bash
   # Using Caddy (easiest for HTTPS)
   apt install -y debian-keyring debian-archive-keyring apt-transport-https
   curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
   curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | tee /etc/apt/sources.list.d/caddy-stable.list
   apt update
   apt install caddy

   # Create Caddyfile
   cat > /etc/caddy/Caddyfile << 'EOF'
   your-domain.com {
       reverse_proxy localhost:8000
   }
   EOF

   systemctl restart caddy
   ```

## Deployment on AWS

### Using EC2

1. **Launch an EC2 instance**:
   - Choose Ubuntu Server 22.04 or 24.04 LTS
   - Select instance type (t2.micro is eligible for free tier)
   - Configure security group to allow inbound traffic on port 8000 (or 80/443 if using a reverse proxy)
   - Add your SSH key pair

2. **Connect to your instance**:
   ```bash
   ssh -i your-key.pem ubuntu@<ec2-public-ip>
   ```

3. **Deploy using Docker**:
   ```bash
   # Install Docker
   sudo apt-get update
   sudo apt-get install -y docker.io docker-compose
   sudo systemctl enable docker
   sudo systemctl start docker

   # Clone and deploy
   git clone https://github.com/facebook/bunnylol.rs.git
   cd bunnylol.rs
   sudo docker-compose up -d
   ```

4. **Or deploy bare-metal**:
   ```bash
   git clone https://github.com/facebook/bunnylol.rs.git
   cd bunnylol.rs
   sudo ./deploy/deploy.sh
   ```

### Using AWS Elastic Container Service (ECS)

1. **Build and push Docker image**:
   ```bash
   # Build image
   docker build -t bunnylol .

   # Tag for ECR
   docker tag bunnylol:latest <aws-account-id>.dkr.ecr.<region>.amazonaws.com/bunnylol:latest

   # Login to ECR
   aws ecr get-login-password --region <region> | docker login --username AWS --password-stdin <aws-account-id>.dkr.ecr.<region>.amazonaws.com

   # Push image
   docker push <aws-account-id>.dkr.ecr.<region>.amazonaws.com/bunnylol:latest
   ```

2. **Create ECS task definition and service** through the AWS Console or CLI

## Heroku Deployment

The `Procfile` is located in `deploy/Procfile` for Heroku deployments.

1. **Create a Heroku app**:
   ```bash
   heroku create your-app-name
   ```

2. **Add Rust buildpack**:
   ```bash
   heroku buildpacks:set emk/rust
   ```

3. **Copy Procfile to root** (Heroku requires it in the root):
   ```bash
   cp deploy/Procfile .
   ```

4. **Deploy**:
   ```bash
   git add Procfile
   git commit -m "Add Procfile for Heroku"
   git push heroku main
   ```

## Configuration

### Environment Variables

Bunnylol can be configured using these environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `ROCKET_ADDRESS` | IP address to bind to | `0.0.0.0` |
| `ROCKET_PORT` | Port to listen on | `8000` |
| `ROCKET_LOG_LEVEL` | Logging level (normal, debug, critical) | `normal` |
| `PORT` | Alternative port specification (for Heroku) | `8000` |

### Systemd Service Configuration

To modify the systemd service configuration:

1. Edit the service file:
   ```bash
   sudo systemctl edit bunnylol
   ```

2. Add your overrides:
   ```ini
   [Service]
   Environment="ROCKET_PORT=9000"
   ```

3. Reload and restart:
   ```bash
   sudo systemctl daemon-reload
   sudo systemctl restart bunnylol
   ```

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

### Systemd Service Issues

**Service fails to start:**
```bash
# Check service status
sudo systemctl status bunnylol

# View detailed logs
sudo journalctl -u bunnylol -n 100 --no-pager

# Check if binary exists and is executable
ls -la /opt/bunnylol/bunnylol
```

**Port already in use:**
```bash
# Find what's using the port
sudo netstat -tlnp | grep 8000

# Or use lsof
sudo lsof -i :8000
```

### Build Issues

**Rust compilation errors:**
```bash
# Update Rust
rustup update

# Clean build cache
cargo clean
cargo build --release
```

**Missing dependencies:**
```bash
# Install required packages
sudo apt-get install -y pkg-config libssl-dev
```

### Network/Firewall Issues

**Can't access the service:**
```bash
# Check if service is listening
sudo netstat -tlnp | grep 8000

# Check firewall (UFW)
sudo ufw status
sudo ufw allow 8000/tcp

# Check firewall (iptables)
sudo iptables -L -n
```

## Security Recommendations

1. **Use a reverse proxy** (nginx/Caddy) for HTTPS
2. **Set up a firewall** to restrict access
3. **Keep the system updated**:
   ```bash
   sudo apt-get update && sudo apt-get upgrade -y
   ```
4. **Use non-root user** (the deployment script does this automatically)
5. **Monitor logs** regularly for suspicious activity

## Performance Tuning

For high-traffic deployments:

1. **Increase worker threads** in Rocket configuration
2. **Use a CDN** if serving static assets
3. **Set up monitoring** with tools like Prometheus/Grafana
4. **Consider horizontal scaling** with a load balancer

## Support

For issues or questions:
- GitHub Issues: https://github.com/facebook/bunnylol.rs/issues
- Documentation: https://github.com/facebook/bunnylol.rs
