# Development Startup Script Design

**Date:** 2025-10-24
**Purpose:** Simplify starting the Jility application with a single command
**Primary User:** Solo developer (you), with future consideration for deployment

## Problem Statement

Currently starting the Jility application requires:
1. Opening two terminal windows
2. Remembering exact commands with flags (`RUST_LOG=info cargo run`, etc.)
3. Manually killing old processes with `lsof` when ports are in use
4. Starting backend before frontend to avoid connection errors
5. Monitoring two separate log streams

This is tedious and error-prone for daily development.

## Solution: Shell Script with Prefixed Logs

Create a `dev.sh` script at project root that handles all startup concerns with a single command.

## Design

### Commands

```bash
./dev.sh start   # Kill old processes, start both servers with prefixed logs
./dev.sh stop    # Gracefully stop both servers
./dev.sh restart # Stop then start
./dev.sh status  # Check if servers are running
```

### Port Configuration

- **Backend (Rust):** Port 3900
- **Frontend (Next.js):** Port 3901

**Rationale:** Using 3900/3901 instead of common 3000/3001 reduces conflicts with other dev servers while staying in a familiar range.

### Script Behavior

#### Cleanup Phase (start command)
1. Use `lsof -ti:3900` and `lsof -ti:3901` to find running processes
2. Kill processes gracefully (SIGTERM first, SIGKILL after 2s if needed)
3. Verify ports are free before starting
4. Create `.jility/` directory if it doesn't exist (for database)

#### Startup Phase
1. Check prerequisites: `cargo` and `npm` installed
2. Start backend: `RUST_LOG=info cargo run --manifest-path jility-server/Cargo.toml`
3. Wait 2 seconds for backend compilation/startup
4. Start frontend: `cd jility-web && npm run dev -- -p 3901`
5. Pipe both outputs through prefix function

#### Log Prefixing
- All output goes to same terminal with color-coded prefixes
- Backend: Blue `[backend]` prefix
- Frontend: Green `[frontend]` prefix
- Implementation: Bash function that reads lines and prepends prefix

```bash
prefix_output() {
    local prefix=$1
    local color=$2
    while IFS= read -r line; do
        echo -e "${color}[${prefix}]${NC} ${line}"
    done
}
```

#### Signal Handling
- Trap SIGINT (Ctrl+C) and EXIT
- On interrupt: Kill both backend and frontend processes
- Store PIDs in temp files for tracking: `/tmp/jility-backend.pid` and `/tmp/jility-frontend.pid`

#### Status Command
- Check if processes are running on ports 3900/3901
- Display: "✓ Backend running on port 3900" or "✗ Backend not running"
- Exit code: 0 if both running, 1 if either stopped

### Error Handling

1. **Missing dependencies:**
   - Check for `cargo` before starting backend
   - Check for `npm` before starting frontend
   - Clear error message: "Error: cargo not found. Install Rust toolchain."

2. **Port conflicts:**
   - If ports still occupied after cleanup, show error with PID details
   - Suggest manual `kill -9 <PID>` if needed

3. **Backend failure:**
   - If backend fails to start, don't start frontend
   - Prevents cascade of "cannot connect to API" errors
   - Show backend error output prominently

4. **Frontend failure:**
   - If frontend fails after backend started, keep backend running
   - User can debug frontend issues without restarting backend

### Example Output

```
$ ./dev.sh start
🧹 Cleaning up old processes...
✓ Killed process on port 3900
✓ Killed process on port 3901
🚀 Starting backend on port 3900...
🚀 Starting frontend on port 3901...
[backend]  Starting Jility server...
[backend]  Connecting to database: sqlite://.jility/data.db
[frontend] ▲ Next.js 14.2.0
[frontend] - Local:        http://localhost:3901
[backend]  Listening on 0.0.0.0:3900
[frontend] ✓ Ready in 1654ms

✅ Both servers running!
   Backend:  http://localhost:3900
   Frontend: http://localhost:3901

Press Ctrl+C to stop both servers.
```

## Configuration Changes Required

### Backend (jility-server)

Update port binding in `jility-server/src/main.rs`:

```rust
// Change from:
let addr = "0.0.0.0:3000";

// To:
let addr = "0.0.0.0:3900";
```

### Frontend (jility-web)

1. **package.json:** Update dev script to use port 3901
   ```json
   "scripts": {
     "dev": "next dev -p 3901"
   }
   ```

2. **.env.local:** Update API base URL
   ```
   NEXT_PUBLIC_API_URL=http://localhost:3900/api
   ```

3. **Script validation:** Check `.env.local` exists and warn if API URL incorrect

## Future Considerations

### For Team Collaboration
- Add `.env.example` with correct ports documented
- Include `dev.sh` instructions in README.md
- Consider adding `dev.sh setup` command to check prerequisites

### For Production Deployment
- Ports 3900/3901 are for local dev only
- Production will use standard HTTP/HTTPS ports (80/443)
- Consider separate `docker-compose.yml` for production deployment
- MCP server always runs locally on developer machines (not deployed)

## Trade-offs

**Chosen: Shell script with interleaved logs**

✅ Pros:
- Zero dependencies (just bash)
- Simple, readable code
- Works on macOS and Linux
- Easy to customize
- Logs viewable in one place

❌ Cons:
- Less sophisticated than process managers (overmind, foreman)
- No built-in auto-restart on crash
- Windows users need WSL or Git Bash

**Not chosen: Docker Compose**

- Reason: Rust binary is simpler than Docker setup for local dev
- Docker adds overhead and complexity for solo developer
- Better suited for production deployment

**Not chosen: Process managers (overmind/foreman)**

- Reason: Requires installing additional tools
- Shell script accomplishes same goal with no dependencies
- Can reconsider if team grows or needs become more complex

## Implementation Checklist

- [ ] Create `dev.sh` script with all commands (start, stop, restart, status)
- [ ] Update backend port to 3900 in `jility-server/src/main.rs`
- [ ] Update frontend port to 3901 in `jility-web/package.json`
- [ ] Create/update `jility-web/.env.local` with new API URL
- [ ] Test script on macOS
- [ ] Make script executable: `chmod +x dev.sh`
- [ ] Update README.md with new startup instructions
- [ ] Commit changes
