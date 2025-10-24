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
        echo "Start command - to be implemented"
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
        echo "Restart command - to be implemented"
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
