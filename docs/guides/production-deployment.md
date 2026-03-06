# Production Deployment Guide

This guide covers deploying BullShift's API server to production environments.

## Prerequisites

- Docker 24+ (or Rust 1.82+ for bare-metal)
- Alpaca broker API credentials
- One of: AWS account, GCP project, or Azure subscription (for cloud deployments)

## Quick Start (Docker)

```bash
# 1. Configure environment
cp .env.example .env
# Edit .env with your Alpaca credentials

# 2. Build and run
docker compose up -d

# 3. Verify
curl http://localhost:8787/health
```

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `ALPACA_API_KEY` | Yes | — | Alpaca API key ID |
| `ALPACA_API_SECRET` | Yes | — | Alpaca API secret |
| `ALPACA_SANDBOX` | No | `true` | `false` for live trading |
| `BULLSHIFT_PORT` | No | `8787` | HTTP listen port |
| `RUST_LOG` | No | `info` | Log level (`debug`, `info`, `warn`, `error`) |

## Docker Build

The `Dockerfile` uses a multi-stage build:

1. **Builder stage** — Rust toolchain compiles `api_server` in release mode
2. **Runtime stage** — Minimal Debian slim image with only the binary and CA certs

```bash
# Build image
docker build -t bullshift-api:latest .

# Run container
docker run -d \
  --name bullshift-api \
  -p 8787:8787 \
  -e ALPACA_API_KEY=your-key \
  -e ALPACA_API_SECRET=your-secret \
  bullshift-api:latest
```

The container runs as a non-root `bullshift` user with a built-in health check.

## Cloud Deployment

### AWS (ECS Fargate)

Template: `deploy/aws/task-definition.json`

```bash
# 1. Create ECR repository
aws ecr create-repository --repository-name bullshift-api

# 2. Push image
aws ecr get-login-password | docker login --username AWS --password-stdin ACCOUNT.dkr.ecr.REGION.amazonaws.com
docker tag bullshift-api:latest ACCOUNT.dkr.ecr.REGION.amazonaws.com/bullshift-api:latest
docker push ACCOUNT.dkr.ecr.REGION.amazonaws.com/bullshift-api:latest

# 3. Store secrets
aws secretsmanager create-secret --name bullshift/alpaca \
  --secret-string '{"api_key":"...","api_secret":"..."}'

# 4. Update task-definition.json with your ACCOUNT_ID and REGION, then register
aws ecs register-task-definition --cli-input-json file://deploy/aws/task-definition.json

# 5. Create service
aws ecs create-service \
  --cluster default \
  --service-name bullshift-api \
  --task-definition bullshift-api \
  --desired-count 1 \
  --launch-type FARGATE \
  --network-configuration "awsvpcConfiguration={subnets=[subnet-xxx],securityGroups=[sg-xxx],assignPublicIp=ENABLED}"
```

### Google Cloud Run

Template: `deploy/gcp/cloudrun.yml`

```bash
# 1. Push image to Artifact Registry
gcloud artifacts repositories create bullshift --repository-format=docker --location=REGION
docker tag bullshift-api:latest REGION-docker.pkg.dev/PROJECT/bullshift/api:latest
docker push REGION-docker.pkg.dev/PROJECT/bullshift/api:latest

# 2. Create secrets
echo -n "your-key" | gcloud secrets create alpaca-api-key --data-file=-
echo -n "your-secret" | gcloud secrets create alpaca-api-secret --data-file=-

# 3. Deploy
gcloud run services replace deploy/gcp/cloudrun.yml --region=REGION
```

### Azure Container Apps

Template: `deploy/azure/container-app.json`

```bash
# 1. Create resource group and container registry
az group create --name bullshift-rg --location eastus
az acr create --resource-group bullshift-rg --name bullshiftacr --sku Basic
az acr login --name bullshiftacr

# 2. Push image
docker tag bullshift-api:latest bullshiftacr.azurecr.io/bullshift-api:latest
docker push bullshiftacr.azurecr.io/bullshift-api:latest

# 3. Deploy
az deployment group create \
  --resource-group bullshift-rg \
  --template-file deploy/azure/container-app.json \
  --parameters containerImage=bullshiftacr.azurecr.io/bullshift-api:latest \
               alpacaApiKey=your-key \
               alpacaApiSecret=your-secret
```

## CI/CD Pipeline

GitHub Actions workflows are in `.github/workflows/`:

- **`ci.yml`** — Runs on every push and PR to `main`:
  - `cargo fmt --check` — formatting
  - `cargo clippy` — linting (warnings treated as errors)
  - `cargo test` — full test suite
  - `cargo build --release` — release binary (uploaded as artifact)
  - Docker build (main branch only)

- **`release.yml`** — Runs on version tags (`v*`):
  - Cross-compiles for Linux x86_64, macOS x86_64, macOS ARM64
  - Creates GitHub Release with binary archives

### Triggering a release

```bash
git tag v2026.3.6
git push origin v2026.3.6
```

## Monitoring

BullShift includes a built-in monitoring module (`src/monitoring/`) with:

- **Health checks** — Component-level health with latency tracking. Access via `GET /health`.
- **Metrics** — Counters, gauges, and histograms with Prometheus text export.
- **Alerting** — Threshold-based alert rules with severity levels and cooldown periods.

### Prometheus Integration

Expose the `/metrics` endpoint and configure Prometheus to scrape it:

```yaml
scrape_configs:
  - job_name: bullshift
    static_configs:
      - targets: ['bullshift-api:8787']
    metrics_path: /metrics
    scrape_interval: 15s
```

### Recommended Alert Rules

| Metric | Condition | Severity |
|--------|-----------|----------|
| `order_errors_total` | > 10 per minute | Critical |
| `api_latency_ms_mean` | > 500 | Warning |
| `active_connections` | > 1000 | Warning |
| `account_balance` | < configured minimum | Critical |

## Security Checklist

- [ ] Set `ALPACA_SANDBOX=false` only when ready for live trading
- [ ] Use secrets managers (not env vars) for credentials in production
- [ ] Run behind a reverse proxy (nginx, ALB, Cloud Load Balancer) with TLS
- [ ] Enable rate limiting on the reverse proxy
- [ ] Restrict network access to the API port
- [ ] Enable audit logging (`RUST_LOG=info`)
- [ ] Set up monitoring alerts for error rates and latency
- [ ] Regularly rotate API credentials

## Bare-Metal Deployment

If Docker is not an option:

```bash
# Build
cd rust
cargo build --release

# Install
sudo cp target/release/api_server /usr/local/bin/bullshift-api

# Create systemd service
sudo tee /etc/systemd/system/bullshift-api.service << 'EOF'
[Unit]
Description=BullShift API Server
After=network.target

[Service]
Type=simple
User=bullshift
Group=bullshift
EnvironmentFile=/etc/bullshift/env
ExecStart=/usr/local/bin/bullshift-api
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable --now bullshift-api
```
