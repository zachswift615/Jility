# Building the Jility Rust Server Locally

This guide will walk you through building and running the Jility server on your local machine.

## Prerequisites

### 1. Install Rust

If you don't have Rust installed:

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts, then restart your terminal or run:
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

You need Rust 1.70 or later.

### 2. Install System Dependencies

Depending on your OS, you may need some build tools:

**macOS:**
```bash
xcode-select --install
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

**Linux (Fedora/RHEL):**
```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install openssl-devel
```

**Windows:**
- Install [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/downloads/)
- Or install [Visual Studio Community](https://visualstudio.microsoft.com/vs/community/) with C++ support

## Building the Server

### Option 1: Build for Development (Fast, with Debug Info)

```bash
# Navigate to the project root
cd /home/user/Jility

# Build the server (this will take a few minutes the first time)
cargo build -p jility-server

# The binary will be at: target/debug/jility-server
```

### Option 2: Build for Production (Optimized, Smaller Binary)

```bash
# Navigate to the project root
cd /home/user/Jility

# Build with release optimizations (takes longer but produces faster binary)
cargo build -p jility-server --release

# The binary will be at: target/release/jility-server
```

## Running the Server

### 1. Set Environment Variables

Create a `.env` file in the project root or export these variables:

```bash
# Create .env file
cat > .env << 'EOF'
# Database connection
DATABASE_URL=sqlite://.jility/data.db?mode=rwc

# JWT secret for authentication (change this!)
JWT_SECRET=change-this-to-a-secure-random-string-in-production

# Server configuration (optional)
HOST=0.0.0.0
PORT=3000

# Rust log level (optional)
RUST_LOG=info
EOF
```

**Important:** Generate a secure JWT_SECRET:
```bash
# Generate a random secret
openssl rand -base64 32
# Or
python3 -c "import secrets; print(secrets.token_urlsafe(32))"
```

### 2. Initialize Database

The database will be created automatically on first run, but you can initialize it manually:

```bash
# Make sure .jility directory exists
mkdir -p .jility

# The migrations will run automatically when you start the server
```

### 3. Run the Server

**Development mode (with hot reload):**
```bash
# Run directly with cargo (rebuilds on code changes with cargo-watch)
cargo install cargo-watch  # Install if you don't have it
cargo watch -x 'run -p jility-server'
```

**Development mode (normal):**
```bash
cargo run -p jility-server

# Or run the binary directly
./target/debug/jility-server
```

**Production mode:**
```bash
./target/release/jility-server
```

### 4. Verify It's Running

The server should start and you'll see output like:

```
[2024-10-24T12:00:00Z INFO  jility_server] Starting Jility server...
[2024-10-24T12:00:00Z INFO  jility_server] Database connected
[2024-10-24T12:00:00Z INFO  jility_server] Running migrations...
[2024-10-24T12:00:00Z INFO  jility_server] Server listening on http://0.0.0.0:3000
```

Test it with curl:
```bash
# Health check (if implemented)
curl http://localhost:3000/health

# List projects
curl http://localhost:3000/api/projects

# Or open in browser
open http://localhost:3000/api/projects
```

## Building All Crates

If you want to build everything (CLI, server, MCP, core):

```bash
# Build all workspace members
cargo build --workspace

# Or build release versions
cargo build --workspace --release
```

## Common Build Issues

### Issue: "linker `cc` not found"
**Solution:** Install C compiler
```bash
# macOS
xcode-select --install

# Linux
sudo apt install build-essential
```

### Issue: "failed to run custom build command for `openssl-sys`"
**Solution:** Install OpenSSL development libraries
```bash
# Ubuntu/Debian
sudo apt install libssl-dev pkg-config

# macOS
brew install openssl
export OPENSSL_DIR=$(brew --prefix openssl)

# Fedora
sudo dnf install openssl-devel
```

### Issue: "error: could not compile `sea-orm`"
**Solution:** Make sure you have the latest Rust version
```bash
rustup update stable
```

### Issue: Build is very slow
**Solution:** Enable parallel compilation
```bash
# Add to ~/.cargo/config.toml
[build]
jobs = 8  # Or number of CPU cores you want to use
```

## Development Tips

### 1. Faster Builds with `sccache`

Install sccache to cache compiled dependencies:
```bash
cargo install sccache

# Add to ~/.cargo/config.toml
[build]
rustc-wrapper = "sccache"
```

### 2. Check for Errors Without Building

```bash
# Fast error checking
cargo check -p jility-server

# With warnings
cargo clippy -p jility-server
```

### 3. Run Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for server only
cargo test -p jility-server

# Run with output
cargo test -- --nocapture
```

### 4. Clean Build

If you're having issues, try a clean build:
```bash
cargo clean
cargo build -p jility-server
```

## Building the CLI

If you also want to build the CLI:

```bash
# Build CLI
cargo build -p jility-cli --release

# Install globally
cargo install --path crates/jility-cli

# Now you can use it from anywhere
jility --help
```

## Building for Different Platforms

### Cross-Compilation

To build for a different platform (e.g., build Linux binary on Mac):

```bash
# Install cross
cargo install cross

# Build for Linux
cross build --target x86_64-unknown-linux-gnu --release -p jility-server

# Build for Windows
cross build --target x86_64-pc-windows-gnu --release -p jility-server
```

## Directory Structure After Build

```
Jility/
├── target/
│   ├── debug/
│   │   ├── jility-server      # Development binary
│   │   ├── jility-cli         # CLI binary
│   │   └── jility-mcp         # MCP server binary
│   └── release/
│       ├── jility-server      # Optimized binary
│       ├── jility-cli
│       └── jility-mcp
├── .jility/
│   └── data.db               # SQLite database (created on first run)
└── .env                      # Environment variables
```

## Next Steps

After building the server:

1. **Start the frontend:**
   ```bash
   cd jility-web
   npm install
   npm run dev
   ```

2. **Open the app:** http://localhost:3001

3. **Register a user:** Create your first account

4. **Explore the features!**

## Production Deployment

For production deployment:

1. Build with release profile: `cargo build --release -p jility-server`
2. Use PostgreSQL instead of SQLite
3. Set secure JWT_SECRET
4. Enable HTTPS
5. Set up reverse proxy (nginx/Caddy)
6. Configure firewall
7. Set up monitoring

See `DEPLOYMENT.md` for detailed production deployment instructions (if it exists).

## Need Help?

- **Build errors:** Check the "Common Build Issues" section above
- **Runtime errors:** Check the logs with `RUST_LOG=debug`
- **Database issues:** Delete `.jility/data.db` to start fresh
- **Port conflicts:** Change `PORT` in `.env` file

## Performance Tips

**Development builds are slow but compile fast.**
**Release builds are fast but compile slow.**

Use development builds while coding, release builds for deployment.

Build times:
- First build: 5-10 minutes (downloading and compiling dependencies)
- Incremental builds: 10-30 seconds
- Release builds: 10-15 minutes (with optimizations)

Happy building! 🚀
