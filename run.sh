#!/usr/bin/env bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default mode
MODE="local"

# Show usage
usage() {
    echo "Jility Task Runner"
    echo ""
    echo "Usage: $0 <command> [options]"
    echo ""
    echo "Commands:"
    echo "  start       - Start all services"
    echo "  stop        - Stop all services"
    echo "  restart     - Restart all services"
    echo "  logs        - View service logs (use -f to follow)"
    echo "  status      - Show service status"
    echo "  build       - Build Docker images"
    echo "  clean       - Clean up containers, volumes, and images"
    echo "  shell       - Open shell in service (backend/frontend)"
    echo "  ngrok-url   - Get ngrok tunnel URL (ngrok mode only)"
    echo ""
    echo "Modes:"
    echo "  --local     - Run locally (default, production build)"
    echo "  --dev       - Run in development mode (hot reload)"
    echo "  --ngrok     - Run with ngrok tunnel for external access"
    echo ""
    echo "Examples:"
    echo "  $0 start              # Start in local mode"
    echo "  $0 start --dev        # Start in dev mode with hot reload"
    echo "  $0 start --ngrok      # Start with ngrok"
    echo "  $0 logs -f            # Follow logs"
    echo "  $0 logs backend       # Show backend logs only"
    echo "  $0 shell backend      # Open shell in backend container"
    echo "  $0 restart --ngrok    # Restart in ngrok mode"
    exit 1
}

# Parse mode flags
parse_mode() {
    for arg in "$@"; do
        case $arg in
            --local)
                MODE="local"
                ;;
            --dev)
                MODE="dev"
                ;;
            --ngrok)
                MODE="ngrok"
                ;;
        esac
    done
}

# Get compose files based on mode
get_compose_files() {
    case $MODE in
        local)
            echo "-f docker-compose.yml"
            ;;
        dev)
            echo "-f docker-compose.yml -f docker-compose.dev.yml"
            ;;
        ngrok)
            echo "-f docker-compose.yml -f docker-compose.ngrok.yml"
            ;;
    esac
}

# Start services
start() {
    parse_mode "$@"
    COMPOSE_FILES=$(get_compose_files)

    echo -e "${BLUE}🚀 Starting Jility in ${YELLOW}${MODE}${BLUE} mode...${NC}"

    if [ "$MODE" = "ngrok" ]; then
        if [ -z "$NGROK_AUTHTOKEN" ]; then
            echo -e "${YELLOW}⚠️  NGROK_AUTHTOKEN not set${NC}"
            echo -e "${YELLOW}   Get your token from: https://dashboard.ngrok.com${NC}"
            echo -e "${YELLOW}   Set it with: export NGROK_AUTHTOKEN=your_token${NC}"
            echo ""
            read -p "Continue anyway? (y/N) " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                exit 1
            fi
        fi
    fi

    docker-compose $COMPOSE_FILES up -d

    echo ""
    echo -e "${GREEN}✅ Services started!${NC}"
    echo ""

    case $MODE in
        local)
            echo -e "   Frontend: ${BLUE}http://localhost:3901${NC}"
            echo -e "   Backend:  ${BLUE}http://localhost:3900${NC}"
            ;;
        dev)
            echo -e "   Frontend: ${BLUE}http://localhost:3901${NC} (with hot reload)"
            echo -e "   Backend:  ${BLUE}http://localhost:3900${NC} (with debug logs)"
            ;;
        ngrok)
            echo -e "   Frontend: ${BLUE}http://localhost:3901${NC}"
            echo -e "   Backend:  ${BLUE}http://localhost:3900${NC}"
            echo -e "   Ngrok UI: ${BLUE}http://localhost:4040${NC}"
            echo ""
            echo -e "${YELLOW}⏳ Waiting for ngrok to start...${NC}"
            sleep 3
            NGROK_URL=$(curl -s http://localhost:4040/api/tunnels 2>/dev/null | grep -o '"public_url":"https://[^"]*' | grep -o 'https://[^"]*' | head -1 || echo "")
            if [ -n "$NGROK_URL" ]; then
                echo -e "   ${GREEN}Ngrok URL: ${BLUE}${NGROK_URL}${NC}"
            else
                echo -e "   ${YELLOW}Ngrok URL not ready yet. Run: $0 ngrok-url${NC}"
            fi
            ;;
    esac

    echo ""
    echo -e "${YELLOW}💡 Tip: Use '$0 logs -f' to view logs${NC}"
}

# Stop services
stop() {
    parse_mode "$@"
    COMPOSE_FILES=$(get_compose_files)

    echo -e "${BLUE}🛑 Stopping services...${NC}"
    docker-compose $COMPOSE_FILES down
    echo -e "${GREEN}✅ Services stopped${NC}"
}

# Restart services
restart() {
    echo -e "${BLUE}🔄 Restarting services...${NC}"
    stop "$@"
    sleep 2
    start "$@"
}

# View logs
logs() {
    parse_mode "$@"
    COMPOSE_FILES=$(get_compose_files)

    # Remove mode flags from arguments
    ARGS=()
    for arg in "$@"; do
        case $arg in
            --local|--dev|--ngrok)
                ;;
            *)
                ARGS+=("$arg")
                ;;
        esac
    done

    docker-compose $COMPOSE_FILES logs "${ARGS[@]}"
}

# Show status
status() {
    parse_mode "$@"
    COMPOSE_FILES=$(get_compose_files)

    echo -e "${BLUE}📊 Service Status (${YELLOW}${MODE}${BLUE} mode)${NC}"
    echo ""
    docker-compose $COMPOSE_FILES ps
}

# Build images
build() {
    parse_mode "$@"
    COMPOSE_FILES=$(get_compose_files)

    echo -e "${BLUE}🔨 Building Docker images...${NC}"
    docker-compose $COMPOSE_FILES build
    echo -e "${GREEN}✅ Build complete${NC}"
}

# Clean up
clean() {
    echo -e "${YELLOW}⚠️  This will remove:${NC}"
    echo -e "   - All Jility containers"
    echo -e "   - All Jility volumes (including database!)"
    echo -e "   - All Jility images"
    echo ""
    read -p "Are you sure? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${BLUE}Cancelled${NC}"
        exit 0
    fi

    echo -e "${BLUE}🧹 Cleaning up...${NC}"

    # Stop all compose files
    docker-compose -f docker-compose.yml down -v 2>/dev/null || true
    docker-compose -f docker-compose.yml -f docker-compose.dev.yml down -v 2>/dev/null || true
    docker-compose -f docker-compose.yml -f docker-compose.ngrok.yml down -v 2>/dev/null || true

    # Remove images
    docker images | grep jility | awk '{print $3}' | xargs docker rmi -f 2>/dev/null || true

    echo -e "${GREEN}✅ Cleanup complete${NC}"
}

# Open shell in service
shell() {
    parse_mode "$@"
    COMPOSE_FILES=$(get_compose_files)

    SERVICE="${2:-backend}"

    echo -e "${BLUE}🐚 Opening shell in ${YELLOW}${SERVICE}${BLUE}...${NC}"
    docker-compose $COMPOSE_FILES exec "$SERVICE" sh
}

# Get ngrok URL
ngrok_url() {
    echo -e "${BLUE}🔗 Fetching ngrok URL...${NC}"

    # Check if ngrok container is running
    if ! docker-compose -f docker-compose.yml -f docker-compose.ngrok.yml ps | grep -q "ngrok.*Up"; then
        echo -e "${RED}❌ Ngrok service not running${NC}"
        echo -e "${YELLOW}Start with: $0 start --ngrok${NC}"
        exit 1
    fi

    # Get URL
    NGROK_URL=$(curl -s http://localhost:4040/api/tunnels 2>/dev/null | grep -o '"public_url":"https://[^"]*' | grep -o 'https://[^"]*' | head -1 || echo "")

    if [ -n "$NGROK_URL" ]; then
        echo -e "${GREEN}✅ Ngrok URL: ${BLUE}${NGROK_URL}${NC}"
        echo ""
        echo -e "   Web Interface: ${BLUE}http://localhost:4040${NC}"
    else
        echo -e "${RED}❌ Could not fetch ngrok URL${NC}"
        echo -e "${YELLOW}Check logs with: $0 logs ngrok${NC}"
        exit 1
    fi
}

# Check if command provided
if [ $# -eq 0 ]; then
    usage
fi

# Parse command
COMMAND="$1"
shift

case "$COMMAND" in
    start)
        start "$@"
        ;;
    stop)
        stop "$@"
        ;;
    restart)
        restart "$@"
        ;;
    logs)
        logs "$@"
        ;;
    status)
        status "$@"
        ;;
    build)
        build "$@"
        ;;
    clean)
        clean "$@"
        ;;
    shell)
        shell "$@"
        ;;
    ngrok-url)
        ngrok_url
        ;;
    *)
        echo -e "${RED}Unknown command: $COMMAND${NC}"
        echo ""
        usage
        ;;
esac
