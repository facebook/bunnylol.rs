# Bunnylol Monitoring Stack

This directory contains the Prometheus + Grafana monitoring setup for Bunnylol.

## Quick Start

1. **Copy environment file:**
   ```bash
   cp .env.example .env
   ```

2. **Set Grafana password in `.env`:**
   ```bash
   GRAFANA_PASSWORD=your_secure_password_here
   ```

3. **Start with monitoring:**
   ```bash
   docker compose --profile monitoring up -d
   ```

4. **Access dashboards:**
   - **Grafana:** http://localhost:3000 (username: `admin`, password: from `.env`)
   - **Prometheus:** http://localhost:9090
   - **Bunnylol:** http://localhost:8000

## What You Get

### Pre-built Dashboard

The Grafana dashboard (`bunnylol-dashboard.json`) includes:

1. **Service Status** - Up/Down indicator
2. **Uptime Graph** - Visual timeline of service availability
3. **Request Rate** - Requests per second by command
4. **Total Requests** - Cumulative request count per command
5. **Command Usage Table** - Sorted list of most-used commands
6. **Request Latency** - P50, P95, P99 percentiles
7. **Active Requests** - Current in-flight requests

### Metrics Exposed

Bunnylol exposes these Prometheus metrics at `/metrics`:

- `bunnylol_requests_total` - Total requests (labeled by command and status)
- `bunnylol_request_duration_milliseconds` - Request latency histogram
- `bunnylol_active_requests` - Current active requests (gauge)
- `bunnylol_command_usage_total` - Command usage counter

## Configuration Files

- **`prometheus.yml`** - Prometheus scrape configuration (15s interval, 30-day retention)
- **`grafana/provisioning/datasources/prometheus.yml`** - Auto-configures Prometheus datasource
- **`grafana/provisioning/dashboards/default.yml`** - Auto-loads dashboards
- **`grafana/dashboards/bunnylol-dashboard.json`** - Pre-built metrics dashboard

## Data Persistence

Metrics data is stored in Docker volumes:
- `prometheus-data` - Time-series database (30-day retention)
- `grafana-data` - Grafana settings and dashboards

To reset all data:
```bash
docker compose --profile monitoring down -v
```

## Customization

### Change Scrape Interval

Edit `monitoring/prometheus.yml`:
```yaml
global:
  scrape_interval: 30s  # Change from 15s to 30s
```

### Change Retention Period

Edit `docker-compose.yml`:
```yaml
command:
  - '--storage.tsdb.retention.time=90d'  # Change from 30d to 90d
```

### Modify Dashboard

1. Log into Grafana at http://localhost:3000
2. Edit the "Bunnylol Metrics" dashboard
3. Save changes
4. Export JSON and replace `grafana/dashboards/bunnylol-dashboard.json`

## Security Notes

- **Never commit `.env` file** - It's gitignored by default
- **Change default password** - Set `GRAFANA_PASSWORD` in `.env`
- **Network access** - By default, Grafana/Prometheus are exposed on localhost only
- **Production deployment** - Consider adding nginx reverse proxy with auth for external access

## Troubleshooting

**Grafana won't start:**
- Check if port 3000 is already in use: `lsof -i :3000`
- Check logs: `docker logs bunnylol-grafana`

**No metrics showing:**
- Verify bunnylol is running: `curl http://localhost:8000/health`
- Check Prometheus targets: http://localhost:9090/targets
- Verify metrics endpoint: `curl http://localhost:8000/metrics`

**Dashboard not loading:**
- Check Grafana logs: `docker logs bunnylol-grafana`
- Verify datasource connection in Grafana UI

## Running Without Monitoring

To run just bunnylol (without Prometheus/Grafana):
```bash
docker compose up -d
```

The monitoring profile is opt-in - you must explicitly request it with `--profile monitoring`.
