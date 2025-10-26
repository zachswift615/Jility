# Docker Setup

Jility uses Docker for consistent development and production environments.

## Quick Start

### Production Build

```bash
# Build and start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

Access:
- Frontend: http://localhost:3901
- Backend API: http://localhost:3900

### Ngrok Testing

```bash
# Set your ngrok auth token (get from https://dashboard.ngrok.com)
export NGROK_AUTHTOKEN=your_token_here

# Start with ngrok service
docker-compose -f docker-compose.yml -f docker-compose.ngrok.yml up -d

# Get your ngrok URL
curl http://localhost:4040/api/tunnels | jq '.tunnels[0].public_url'

# Or visit: http://localhost:4040 (ngrok web interface)
```

Access from anywhere: https://your-ngrok-url.ngrok-free.dev

## Architecture

```
┌─────────────┐
│   Ngrok     │ (optional)
│   Service   │
└──────┬──────┘
       │
┌──────▼──────┐         ┌─────────────┐
│  Frontend   │────────▶│   Backend   │
│  (Next.js)  │  proxy  │   (Rust)    │
│  Port 3901  │         │  Port 3900  │
└─────────────┘         └──────┬──────┘
                               │
                        ┌──────▼──────┐
                        │   SQLite    │
                        │   Volume    │
                        └─────────────┘
```

## Services

### Backend
- **Image**: Custom Rust build
- **Port**: 3900
- **Database**: SQLite (persisted in `jility-data` volume)
- **Environment**: See `docker-compose.yml`

### Frontend
- **Image**: Custom Next.js build (standalone output)
- **Port**: 3901
- **API Proxy**: Proxies `/api/*` → `http://backend:3900/api/*`
- **Build args**: `NEXT_PUBLIC_API_URL`, `BACKEND_URL`

### Ngrok (optional)
- **Image**: ngrok/ngrok:latest
- **Port**: 4040 (web interface)
- **Tunnels**: frontend:3901
- **Requires**: `NGROK_AUTHTOKEN` environment variable

## Environment Variables

### Backend

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Log level |
| `DATABASE_URL` | `sqlite:///app/.jility/data.db?mode=rwc` | Database path |
| `BIND_ADDRESS` | `0.0.0.0:3900` | Server bind address |
| `JWT_SECRET` | `insecure_default...` | JWT signing secret (⚠️ change in production!) |

### Frontend

| Variable | Default | Description |
|----------|---------|-------------|
| `NEXT_PUBLIC_API_URL` | `/api` | API URL for browser (relative) |
| `BACKEND_URL` | `http://backend:3900` | Backend URL for server-side proxy |
| `NODE_ENV` | `production` | Node environment |

### Ngrok

| Variable | Required | Description |
|----------|----------|-------------|
| `NGROK_AUTHTOKEN` | ✅ Yes | Get from https://dashboard.ngrok.com |

## Docker Compose Files

### `docker-compose.yml` (base)
Production configuration with optimized builds.

### `docker-compose.dev.yml` (development)
Development override with:
- Volume mounts for hot reload
- Debug logging
- Source code mounted

Usage:
```bash
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up
```

### `docker-compose.ngrok.yml` (ngrok)
Adds ngrok service for external testing.

Usage:
```bash
docker-compose -f docker-compose.yml -f docker-compose.ngrok.yml up
```

## Volumes

- **`jility-data`**: SQLite database persistence
- **`jility-cargo-cache`** (dev only): Cargo dependency cache

## Common Commands

```bash
# Build without cache
docker-compose build --no-cache

# Rebuild specific service
docker-compose build backend

# View service logs
docker-compose logs -f backend
docker-compose logs -f frontend

# Execute command in container
docker-compose exec backend sh
docker-compose exec frontend sh

# Clean up everything
docker-compose down -v  # includes volumes
docker system prune -a  # full cleanup

# Check service health
docker-compose ps

# Restart a service
docker-compose restart backend
```

## Troubleshooting

### Build fails with "no space left on device"

```bash
# Clean up Docker resources
docker system prune -a --volumes
```

### Frontend can't reach backend

Check that services are on the same network:
```bash
docker-compose exec frontend ping backend
```

### Ngrok tunnel not working

1. Check auth token is set:
   ```bash
   echo $NGROK_AUTHTOKEN
   ```

2. View ngrok logs:
   ```bash
   docker-compose logs ngrok
   ```

3. Check ngrok web interface: http://localhost:4040

### Database changes not persisting

Make sure the volume is mounted:
```bash
docker volume ls | grep jility
docker volume inspect jility_jility-data
```

## Production Deployment

### Security Checklist

- [ ] Change `JWT_SECRET` to a strong random value
- [ ] Use proper secrets management (Docker secrets, vault, etc.)
- [ ] Configure CORS properly in production
- [ ] Enable HTTPS/TLS
- [ ] Set up proper logging and monitoring
- [ ] Regular database backups
- [ ] Resource limits in compose file

### Example Production Override

```yaml
# docker-compose.prod.yml
version: '3.8'

services:
  backend:
    environment:
      - JWT_SECRET=${JWT_SECRET}  # From environment
    deploy:
      resources:
        limits:
          cpus: '1'
          memory: 512M
        reservations:
          cpus: '0.5'
          memory: 256M

  frontend:
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 256M
```

Usage:
```bash
JWT_SECRET=$(openssl rand -base64 32) \
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

## Development Workflow

For local development with hot reload, you have two options:

**Option 1: Docker (recommended for consistency)**
```bash
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up
```

**Option 2: Native (faster hot reload)**
```bash
./dev.sh start
```

Use Docker for final testing before deployment, use native for fast iteration.
