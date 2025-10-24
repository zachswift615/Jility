# Dev Startup Script Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Create a single-command startup script that starts both backend and frontend servers with interleaved logs.

**Architecture:** Bash script that handles process cleanup, starts servers with port configuration, and pipes output through prefix functions for colored, interleaved logs.

**Tech Stack:** Bash, Rust (backend), Next.js (frontend)

---

## Task 1: Create dev.sh Script Structure

**Files:**
- Create: `dev.sh`

**Step 1: Create script with basic structure**

Create `dev.sh` in project root:

```bash
#!/usr/bin/env bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Ports
BACKEND_PORT=3900
FRONTEND_PORT=3901

# PID files
BACKEND_PID="/tmp/jility-backend.pid"
FRONTEND_PID="/tmp/jility-frontend.pid"

# Show usage
usage() {
    echo "Usage: $0 {start|stop|restart|status}"
    echo ""
    echo "Commands:"
    echo "  start   - Start both backend and frontend servers"
    echo "  stop    - Stop both servers gracefully"
    echo "  restart - Restart both servers"
    echo "  status  - Check if servers are running"
    exit 1
}

# Check if command provided
if [ $# -eq 0 ]; then
    usage
fi

case "$1" in
    start)
        echo "Start command - to be implemented"
        ;;
    stop)
        echo "Stop command - to be implemented"
        ;;
    restart)
        echo "Restart command - to be implemented"
        ;;
    status)
        echo "Status command - to be implemented"
        ;;
    *)
        usage
        ;;
esac
```

**Step 2: Make script executable**

Run:
```bash
chmod +x dev.sh
```

**Step 3: Test basic script works**

Run:
```bash
./dev.sh
```

Expected output:
```
Usage: ./dev.sh {start|stop|restart|status}
...
```

**Step 4: Commit**

```bash
git add dev.sh
git commit -m "feat: add dev.sh script structure with basic commands"
```

---

## Task 2: Implement Cleanup and Status Functions

**Files:**
- Modify: `dev.sh`

**Step 1: Add helper functions**

Add these functions after the variable declarations in `dev.sh`:

```bash
# Kill process on port
kill_port() {
    local port=$1
    local pid=$(lsof -ti:$port 2>/dev/null)

    if [ -n "$pid" ]; then
        echo -e "${YELLOW}🧹 Killing process on port $port (PID: $pid)${NC}"
        kill -TERM $pid 2>/dev/null || true
        sleep 1
        # Force kill if still running
        if kill -0 $pid 2>/dev/null; then
            kill -KILL $pid 2>/dev/null || true
        fi
        echo -e "${GREEN}✓ Process on port $port stopped${NC}"
        return 0
    else
        echo -e "${BLUE}ℹ No process running on port $port${NC}"
        return 1
    fi
}

# Check if port is in use
is_port_in_use() {
    local port=$1
    lsof -ti:$port >/dev/null 2>&1
}

# Check prerequisites
check_prerequisites() {
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Error: cargo not found. Install Rust toolchain.${NC}"
        exit 1
    fi

    if ! command -v npm &> /dev/null; then
        echo -e "${RED}Error: npm not found. Install Node.js.${NC}"
        exit 1
    fi
}

# Prefix output with color
prefix_output() {
    local prefix=$1
    local color=$2
    while IFS= read -r line; do
        echo -e "${color}[${prefix}]${NC} ${line}"
    done
}
```

**Step 2: Implement status command**

Replace the `status)` case with:

```bash
    status)
        echo "🔍 Checking server status..."

        backend_running=0
        frontend_running=0

        if is_port_in_use $BACKEND_PORT; then
            echo -e "${GREEN}✓ Backend running on port $BACKEND_PORT${NC}"
            backend_running=1
        else
            echo -e "${RED}✗ Backend not running${NC}"
        fi

        if is_port_in_use $FRONTEND_PORT; then
            echo -e "${GREEN}✓ Frontend running on port $FRONTEND_PORT${NC}"
            frontend_running=1
        else
            echo -e "${RED}✗ Frontend not running${NC}"
        fi

        if [ $backend_running -eq 1 ] && [ $frontend_running -eq 1 ]; then
            exit 0
        else
            exit 1
        fi
        ;;
```

**Step 3: Test status command**

Run:
```bash
./dev.sh status
```

Expected output (servers not running):
```
🔍 Checking server status...
✗ Backend not running
✗ Frontend not running
```

Exit code should be 1: `echo $?` returns `1`

**Step 4: Commit**

```bash
git add dev.sh
git commit -m "feat: add status command and helper functions for port management"
```

---

## Task 3: Implement Stop Command

**Files:**
- Modify: `dev.sh`

**Step 1: Implement stop command**

Replace the `stop)` case with:

```bash
    stop)
        echo "🛑 Stopping servers..."

        # Stop backend
        if [ -f "$BACKEND_PID" ]; then
            pid=$(cat "$BACKEND_PID")
            if kill -0 $pid 2>/dev/null; then
                kill -TERM $pid 2>/dev/null || true
                rm -f "$BACKEND_PID"
                echo -e "${GREEN}✓ Backend stopped${NC}"
            fi
        fi

        # Stop frontend
        if [ -f "$FRONTEND_PID" ]; then
            pid=$(cat "$FRONTEND_PID")
            if kill -0 $pid 2>/dev/null; then
                kill -TERM $pid 2>/dev/null || true
                rm -f "$FRONTEND_PID"
                echo -e "${GREEN}✓ Frontend stopped${NC}"
            fi
        fi

        # Also try to kill by port (fallback)
        kill_port $BACKEND_PORT
        kill_port $FRONTEND_PORT

        echo -e "${GREEN}✅ All servers stopped${NC}"
        ;;
```

**Step 2: Test stop command (when nothing is running)**

Run:
```bash
./dev.sh stop
```

Expected output:
```
🛑 Stopping servers...
ℹ No process running on port 3900
ℹ No process running on port 3901
✅ All servers stopped
```

**Step 3: Commit**

```bash
git add dev.sh
git commit -m "feat: implement stop command with cleanup"
```

---

## Task 4: Implement Start Command

**Files:**
- Modify: `dev.sh`

**Step 1: Add trap handler and start command**

Add trap handler after variable declarations:

```bash
# Trap handler for clean shutdown
cleanup() {
    echo -e "\n${YELLOW}Caught interrupt signal, stopping servers...${NC}"
    if [ -f "$BACKEND_PID" ]; then
        kill $(cat "$BACKEND_PID") 2>/dev/null || true
        rm -f "$BACKEND_PID"
    fi
    if [ -f "$FRONTEND_PID" ]; then
        kill $(cat "$FRONTEND_PID") 2>/dev/null || true
        rm -f "$FRONTEND_PID"
    fi
    exit 0
}

trap cleanup INT TERM
```

Replace the `start)` case with:

```bash
    start)
        check_prerequisites

        echo "🧹 Cleaning up old processes..."
        kill_port $BACKEND_PORT
        kill_port $FRONTEND_PORT

        # Ensure .jility directory exists
        mkdir -p .jility

        echo "🚀 Starting backend on port $BACKEND_PORT..."

        # Start backend
        RUST_LOG=info cargo run --manifest-path jility-server/Cargo.toml 2>&1 | prefix_output "backend" "$BLUE" &
        BACKEND_PID_VAL=$!
        echo $BACKEND_PID_VAL > "$BACKEND_PID"

        # Wait for backend to start
        echo "⏳ Waiting for backend to compile and start..."
        sleep 5

        if ! is_port_in_use $BACKEND_PORT; then
            echo -e "${RED}Error: Backend failed to start${NC}"
            exit 1
        fi

        echo "🚀 Starting frontend on port $FRONTEND_PORT..."

        # Start frontend
        (cd jility-web && npm run dev -- -p $FRONTEND_PORT) 2>&1 | prefix_output "frontend" "$GREEN" &
        FRONTEND_PID_VAL=$!
        echo $FRONTEND_PID_VAL > "$FRONTEND_PID"

        # Wait a bit for frontend
        sleep 3

        echo ""
        echo -e "${GREEN}✅ Both servers running!${NC}"
        echo -e "   Backend:  ${BLUE}http://localhost:$BACKEND_PORT${NC}"
        echo -e "   Frontend: ${GREEN}http://localhost:$FRONTEND_PORT${NC}"
        echo ""
        echo -e "${YELLOW}Press Ctrl+C to stop both servers${NC}"
        echo ""

        # Wait for both processes
        wait
        ;;
```

**Step 2: Implement restart command**

Replace the `restart)` case with:

```bash
    restart)
        $0 stop
        sleep 2
        $0 start
        ;;
```

**Step 3: Commit**

```bash
git add dev.sh
git commit -m "feat: implement start and restart commands with log prefixing"
```

---

## Task 5: Update Backend Port Configuration

**Files:**
- Modify: `jility-server/src/main.rs`

**Step 1: Find current port binding**

Run:
```bash
grep -n "0.0.0.0:3000" jility-server/src/main.rs
```

**Step 2: Update port to 3900**

Find the line that binds to port (likely around line 50-100):

```rust
let addr = "0.0.0.0:3000";
```

Change to:

```rust
let addr = "0.0.0.0:3900";
```

Also update any logging that mentions the port:

```rust
info!("Listening on {}", addr);
```

**Step 3: Verify no other hardcoded 3000 references**

Run:
```bash
grep -r "3000" jility-server/src/ --include="*.rs"
```

Update any other references to port 3000 to use 3900.

**Step 4: Test compilation**

Run:
```bash
cargo check --manifest-path jility-server/Cargo.toml
```

Expected: No compilation errors

**Step 5: Commit**

```bash
git add jility-server/src/main.rs
git commit -m "feat: update backend to use port 3900"
```

---

## Task 6: Update Frontend Port Configuration

**Files:**
- Modify: `jility-web/package.json`
- Create/Modify: `jility-web/.env.local`

**Step 1: Update package.json dev script**

Open `jility-web/package.json` and find the `scripts` section.

Change:
```json
"scripts": {
  "dev": "next dev -p 3001",
  ...
}
```

To:
```json
"scripts": {
  "dev": "next dev -p 3901",
  ...
}
```

**Step 2: Create/update .env.local**

Create or update `jility-web/.env.local`:

```
NEXT_PUBLIC_API_URL=http://localhost:3900/api
```

**Step 3: Check for hardcoded port references**

Run:
```bash
grep -r "localhost:3000\|localhost:3001" jility-web/lib/ --include="*.ts" --include="*.tsx"
```

Update any hardcoded URLs to use environment variables.

**Step 4: Verify .env.local is in .gitignore**

Run:
```bash
grep -q "^\.env\.local$" jility-web/.gitignore && echo "Found" || echo ".env.local not in .gitignore"
```

If not found, add it:
```bash
echo ".env.local" >> jility-web/.gitignore
```

**Step 5: Create .env.local.example for documentation**

Create `jility-web/.env.local.example`:

```
# API Base URL
NEXT_PUBLIC_API_URL=http://localhost:3900/api
```

**Step 6: Commit**

```bash
git add jility-web/package.json jility-web/.env.local.example jility-web/.gitignore
git commit -m "feat: update frontend to use port 3901 and connect to backend on 3900"
```

Note: `.env.local` should not be committed (gitignored)

---

## Task 7: Update Documentation

**Files:**
- Modify: `README.md` or create `DEVELOPMENT.md`

**Step 1: Add development section to README**

Add this section to `README.md` (or create `DEVELOPMENT.md`):

```markdown
## Development

### Quick Start

Start both backend and frontend servers with one command:

```bash
./dev.sh start
```

This will:
- Clean up any processes on ports 3900/3901
- Start the Rust backend on port 3900
- Start the Next.js frontend on port 3901
- Show logs from both servers with colored prefixes

Access the application at: http://localhost:3901
Backend API at: http://localhost:3900

### Other Commands

```bash
./dev.sh stop     # Stop both servers
./dev.sh restart  # Restart both servers
./dev.sh status   # Check if servers are running
```

### Ports

- Backend: 3900
- Frontend: 3901

### Manual Startup

If you prefer to run servers separately:

**Backend:**
```bash
RUST_LOG=info cargo run --manifest-path jility-server/Cargo.toml
```

**Frontend:**
```bash
cd jility-web
npm run dev
```
```

**Step 2: Commit documentation**

```bash
git add README.md
git commit -m "docs: add development setup instructions for new dev.sh script"
```

---

## Task 8: Integration Testing

**Files:**
- None (testing existing code)

**Step 1: Stop any running servers**

Run:
```bash
./dev.sh stop
```

**Step 2: Verify status shows nothing running**

Run:
```bash
./dev.sh status
```

Expected output:
```
✗ Backend not running
✗ Frontend not running
```

Exit code: 1

**Step 3: Start servers**

Run:
```bash
./dev.sh start
```

Expected:
- See "🧹 Cleaning up old processes..."
- See "🚀 Starting backend on port 3900..."
- See blue `[backend]` prefixed logs
- See "🚀 Starting frontend on port 3901..."
- See green `[frontend]` prefixed logs
- See "✅ Both servers running!"
- See URLs for both servers

**Step 4: Verify status shows both running**

In another terminal:
```bash
./dev.sh status
```

Expected output:
```
✓ Backend running on port 3900
✓ Frontend running on port 3901
```

Exit code: 0

**Step 5: Test frontend connects to backend**

Open browser to http://localhost:3901

Check browser console for any connection errors to API.

Navigate to backlog page: http://localhost:3901/backlog

Verify page loads and can fetch tickets.

**Step 6: Test stop command**

Press Ctrl+C in the terminal running `./dev.sh start`

Expected:
- See "Caught interrupt signal, stopping servers..."
- Both servers should stop cleanly
- Script should exit

**Step 7: Verify cleanup**

Run:
```bash
./dev.sh status
```

Expected: Both servers stopped (exit code 1)

**Step 8: Test restart command**

Run:
```bash
./dev.sh restart
```

Expected:
- Servers stop
- Short pause
- Servers start again with same output as start command

**Step 9: Final cleanup**

Press Ctrl+C to stop, then run:
```bash
./dev.sh stop
```

---

## Task 9: Final Commit and Merge Preparation

**Files:**
- None

**Step 1: Verify all changes committed**

Run:
```bash
git status
```

Expected: "working tree clean"

**Step 2: View commit log**

Run:
```bash
git log --oneline main..HEAD
```

Expected: See all 7 commits from this implementation

**Step 3: Push feature branch**

Run:
```bash
git push -u origin feature/dev-startup-script
```

**Step 4: Record completion in workshop**

Run:
```bash
workshop decision "Implemented dev.sh startup script" -r "Created single-command startup with interleaved logs on ports 3900/3901. Handles cleanup, signal trapping, and provides start/stop/restart/status commands."
```

---

## Verification Checklist

After completing all tasks, verify:

- [ ] `./dev.sh start` starts both servers with prefixed logs
- [ ] `./dev.sh stop` stops both servers cleanly
- [ ] `./dev.sh restart` restarts both servers
- [ ] `./dev.sh status` shows correct status
- [ ] Ctrl+C stops both servers gracefully
- [ ] Backend runs on port 3900
- [ ] Frontend runs on port 3901
- [ ] Frontend can connect to backend API
- [ ] Backlog page loads data from backend
- [ ] Documentation updated with new workflow
- [ ] All changes committed to feature branch

## Success Criteria

The implementation is complete when:
1. User can type `./dev.sh start` and have both servers running with visible logs
2. Logs are clearly prefixed and color-coded
3. Servers can be stopped with Ctrl+C or `./dev.sh stop`
4. Status command works correctly
5. Documentation explains the new workflow
6. Feature branch is ready for merge to main
