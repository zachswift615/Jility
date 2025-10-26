# Dev Script Usage

The `./dev.sh` script manages both backend and frontend development servers with support for multiple environment configurations.

## Basic Commands

```bash
# Start both servers (uses .env.local by default)
./dev.sh start

# Stop both servers
./dev.sh stop

# Restart both servers
./dev.sh restart

# Check server status
./dev.sh status
```

## Multiple Environment Configs

The script supports different environment configurations via the `--config` flag.

### Usage

```bash
# Start with custom config
./dev.sh start --config=.env.ngrok

# Restart with custom config (preserves config on restart)
./dev.sh restart --config=.env.ngrok

# Switch back to default config
./dev.sh restart
```

### How It Works

1. When you specify `--config=FILE`, the script:
   - Sources the specified env file using `set -a && . FILE && set +a`
   - Exports all variables from that file to the environment
   - Starts the frontend with those environment variables
   - **Does NOT modify `.env.local`** - both files stay intact!

2. Next.js picks up the environment variables from the shell

3. When you restart without `--config`, it uses the default `.env.local`

### Available Configs

- **`.env.local`** (default) - Local development configuration
- **`.env.ngrok`** - Configuration for ngrok/external testing

### Creating Custom Configs

Create any `.env.*` file in `jility-web/`:

```bash
# jility-web/.env.production
NEXT_PUBLIC_API_URL=/api
BACKEND_URL=http://localhost:3900
# Add your custom vars
```

Then use it:

```bash
./dev.sh start --config=.env.production
```

## Examples

### Local Development (Default)

```bash
./dev.sh start
# Frontend: http://localhost:3901
# Backend: http://localhost:3900
# API calls: /api/* → proxied to localhost:3900/api/*
```

### Ngrok Testing

```bash
# Terminal 1: Start with ngrok config
./dev.sh start --config=.env.ngrok

# Terminal 2: Start ngrok
ngrok http 3901

# Access via ngrok URL from any device
```

### Switching Configs

```bash
# Start with ngrok config
./dev.sh start --config=.env.ngrok

# Switch back to local (restart required)
./dev.sh restart

# Or switch to different config
./dev.sh restart --config=.env.production
```

## Port Configuration

The script uses these ports (configurable in the script):

- **Backend**: 3900
- **Frontend**: 3901

PID files are stored in `/tmp/jility-*.pid` for process tracking.

## Troubleshooting

### Config not applying?

Make sure to **restart** after changing configs:

```bash
./dev.sh restart --config=.env.ngrok
```

### Want to see which config is loaded?

The script prints which config file it's using when starting:

```bash
./dev.sh start --config=.env.ngrok
# Output: ℹ Using config file: .env.ngrok
```

### Config files stay intact

Both `.env.local` and `.env.ngrok` remain unchanged. The script only reads from the specified file and exports variables to the environment.
