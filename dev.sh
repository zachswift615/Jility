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
    stop)
        echo "🛑 Stopping servers..."

        # Stop backend
        if [ -f "$BACKEND_PID" ]; then
            pid=$(cat "$BACKEND_PID")
            if kill -0 $pid 2>/dev/null; then
                kill -TERM $pid 2>/dev/null || true
                rm -f "$BACKEND_PID"
                echo -e "${GREEN}✓ Backend stopped${NC}"
            else
                # Process not running, clean up stale PID file
                rm -f "$BACKEND_PID"
            fi
        fi

        # Stop frontend
        if [ -f "$FRONTEND_PID" ]; then
            pid=$(cat "$FRONTEND_PID")
            if kill -0 $pid 2>/dev/null; then
                kill -TERM $pid 2>/dev/null || true
                rm -f "$FRONTEND_PID"
                echo -e "${GREEN}✓ Frontend stopped${NC}"
            else
                # Process not running, clean up stale PID file
                rm -f "$FRONTEND_PID"
            fi
        fi

        # Also try to kill by port (fallback)
        kill_port $BACKEND_PORT || true
        kill_port $FRONTEND_PORT || true

        echo -e "${GREEN}✅ All servers stopped${NC}"
        ;;
    restart)
        $0 stop
        sleep 2
        $0 start
        ;;
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
    *)
        usage
        ;;
esac
