# ⚡ Jility

**AI-native project management for humans and agents working together**

Jility is a lightweight, beautiful, and blazingly fast alternative to JIRA, designed from the ground up for teams that use AI coding assistants. It treats humans and AI agents as equal collaborators, enabling seamless handoffs, real-time updates, and intelligent workflow automation.

---

## Why Jility?

### The Problem
- **JIRA is bloated and slow** - Takes forever to load, cluttered with features nobody uses
- **Expensive** - Per-seat pricing adds up quickly for small teams
- **Not built for AI workflows** - No native support for AI agents creating/managing tickets
- **Poor UX** - Design-conscious teams deserve better

### The Solution
Jility combines:
- ⚡ **Speed** - Built in Rust, near-instant interactions
- 🎨 **Beautiful UI** - Linear-inspired design that developers actually enjoy using
- 🤖 **AI-native** - MCP server built-in, agents are first-class team members
- 💰 **Affordable** - Self-hosted or per-project pricing (not per-seat)
- 🔒 **Local-first** - Your data stays with your code in `.jility/` directory

---

## Key Features

### Human ↔ Agent Collaboration
- **MCP-native** - Claude Code can create, update, and manage tickets via MCP tools
- **Seamless handoffs** - Assign work between humans and agents with full context
- **Pairing** - Multiple assignees (human + agent) can work together on tickets
- **Comment threads** - Natural communication between team members and agents
- **Activity transparency** - Full audit log showing who (human or agent) did what, when

### Intelligent Workflows
- **AI-powered sprint planning** - Claude Code can create sprints, estimate capacity, add tickets, and track progress autonomously
- **Epic support** - Organize tickets into epics with JIRA-like hierarchy and progress tracking
- **Context bundling** - Agents get full ticket history, comments, dependencies, and guidance
- **Smart decomposition** - Agents can split complex tickets into sub-tasks
- **Template system** - Quick ticket creation for common patterns
- **Dependency tracking** - Automatic detection and management

### Developer Experience
- **MCP tools** - 26+ tools for creating, updating, and managing tickets via Claude Code
- **Precise editing** - Token-efficient line-based description updates via MCP
- **Full activity log** - Complete history of ticket changes with timestamps and attribution
- **Git integration** - Link commits to tickets for traceability
- **Markdown support** - Comments support markdown for rich formatting
- **Soft delete** - Deleted tickets preserved for audit trail, don't clutter views

### Performance
- **Fast** - Rust backend, no JVM bloat
- **Lightweight** - Single binary + SQLite file
- **Offline-capable** - Works without internet
- **Mobile-friendly** - Actually usable on phones

---

## Architecture

```
jility/
├── jility-core      # Shared models, business logic
├── jility-server    # Axum web server + REST API
├── jility-mcp       # MCP server for AI agents
└── jility-web       # Next.js frontend
```

**Tech Stack:**
- Backend: Rust (Axum, SeaORM, SQLite/Postgres)
- Frontend: Next.js 14 + Tailwind CSS + shadcn/ui
- MCP: Anthropic's Model Context Protocol
- AI Integration: Claude Code via MCP server

---

## Getting Started

### Quick Start with Task Runner

Jility uses [Task](https://taskfile.dev) for streamlined development:

```bash
# Install dependencies and build
task build

# Start all services (Docker)
task start

# Or run natively with hot reload
task start-dev

# Access the app
open http://localhost:3901
```

**Access Points:**
- Frontend UI: `http://localhost:3901`
- Backend API: `http://localhost:3900`

### Using with Claude Code

Jility is designed to be managed via Claude Code's MCP integration:

```bash
# Create .mcp.json in project root
cat > .mcp.json <<EOF
{
  "mcpServers": {
    "jility": {
      "command": "/path/to/jility/target/release/jility-mcp",
      "env": {
        "JILITY_API_URL": "http://localhost:3900/api",
        "JILITY_PROJECT_ID": "your-project-id",
        "JILITY_API_TOKEN": "your-api-token"
      }
    }
  }
}
EOF

# Restart Claude Code
# Now you can use MCP tools:
# - mcp__jility__create_ticket
# - mcp__jility__create_epic
# - mcp__jility__list_tickets
# And many more!
```

---

## Development

### Prerequisites
- Rust (latest stable)
- Node.js 18+ and npm
- Git

### Quick Start with dev.sh ⚡

The **recommended** way to run Jility in development mode is using the `dev.sh` script. It handles both backend and frontend servers with a single command, displays color-coded interleaved logs, and manages process cleanup automatically.

```bash
# Start both backend and frontend servers
./dev.sh start

# Check status of running services
./dev.sh status

# Restart all services
./dev.sh restart

# Stop all services
./dev.sh stop
```

**What `dev.sh start` does:**
- ✅ Automatically cleans up any processes on ports 3900/3901
- ✅ Starts the Rust backend on port 3900
- ✅ Starts the Next.js frontend on port 3901
- ✅ Shows color-coded logs: 🔵 `[backend]` and 🟢 `[frontend]`
- ✅ Handles graceful shutdown with Ctrl+C

**Access points:**
- Backend API: `http://localhost:3900`
- Frontend UI: `http://localhost:3901` ← **Open this in your browser**

### Manual Development Setup

If you prefer to run services manually or need more control:

#### 1. Backend (Rust/Axum)

```bash
# Navigate to backend directory
cd jility-server

# Build and run in development mode
cargo run

# Or use cargo watch for auto-reload
cargo install cargo-watch
cargo watch -x run
```

The backend will start on `http://localhost:3900`.

#### 2. Frontend (Next.js)

```bash
# Navigate to frontend directory
cd jility-web

# Install dependencies (first time only)
npm install

# Create local environment file
cp .env.local.example .env.local

# Start development server
npm run dev
```

The frontend will start on `http://localhost:3901`.

### Environment Configuration

#### Backend (.env in project root)
```bash
DATABASE_URL=sqlite://.jility/data.db?mode=rwc
JWT_SECRET=your_secret_key_here
BIND_ADDRESS=0.0.0.0:3900
```

#### Frontend (.env.local in jility-web/)
```bash
NEXT_PUBLIC_API_URL=http://localhost:3900/api
```

### Database Migrations

Migrations run automatically on backend startup. The database is stored in `.jility/data.db`.

To reset the database:
```bash
rm -rf .jility/data.db*
cargo run  # Will recreate and run migrations
```

### Project Structure

```
jility/
├── crates/
│   ├── jility-core/    # Shared Rust models/logic
│   ├── jility-server/  # Axum backend + REST API
│   └── jility-mcp/     # MCP server for AI agents
├── jility-web/         # Next.js frontend
├── dev.sh              # Development helper script
├── Taskfile.yml        # Task runner configuration
└── .mcp.json           # MCP server configuration
```

---

## Documentation

### Getting Started Guides
- **[Quick Start](docs/guides/QUICK_START.md)** - Get up and running in 5 minutes
- **[User Guide](docs/guides/USER_GUIDE.md)** - Comprehensive user documentation
- **[Build Guide](docs/guides/BUILD_GUIDE.md)** - Building Jility from source
- **[Quick Reference](docs/guides/QUICK_REFERENCE.md)** - Command reference cheat sheet

### API Documentation
- **[API Reference](docs/api/API.md)** - Complete REST API documentation
- **[Search API](docs/api/SEARCH_DOCUMENTATION.md)** - Advanced search functionality

### Architecture & Design
- **[Wireframes](docs/jility-wireframes.html)** - Interactive UI mockups showing all key screens
- **[Project Plan](docs/jility-project-plan.md)** - Complete development roadmap with 55 tickets across 4 phases
- **[CLI Collaboration Guide](docs/jility-cli-collaboration-guide.md)** - Human/agent collaboration examples
- **[Database Schema](docs/database-schema-design.md)** - Database design and entity relationships
- **[MCP Protocol Design](docs/mcp-protocol-design.md)** - Model Context Protocol implementation
- **[Versioning System](docs/versioning-system-design.md)** - Ticket description versioning design

### Implementation Documentation
- **[Phase 3 Complete](docs/implementation/PHASE_3_COMPLETE.md)** - Web UI implementation summary
- **[Phase 4 Complete](docs/implementation/PHASE_4_COMPLETE.md)** - Search and sprint features
- **[Authentication](docs/implementation/AUTHENTICATION_IMPLEMENTATION.md)** - JWT authentication system
- **[Database](docs/implementation/DATABASE_IMPLEMENTATION.md)** - Database migrations and schema
- **[Sprint Management](docs/implementation/SPRINT_MANAGEMENT_IMPLEMENTATION.md)** - Sprint planning features
- **[Search Implementation](docs/implementation/SEARCH_IMPLEMENTATION_SUMMARY.md)** - Full-text search with FTS5

### Testing
- **[Testing Guide](docs/testing/TESTING.md)** - How to run and write tests
- **[MCP Server Testing](docs/testing/MCP_SERVER_TESTING.md)** - Testing the MCP server integration

---

## Use Cases

### Solo Developer with Claude Code
```typescript
// In Claude Code, use MCP tools to manage your backlog
// Create an epic for a major feature
await mcp__jility__create_epic("User Authentication System", {
  description: "Complete auth with login, registration, password reset",
  epic_color: "#3b82f6"
})

// Break it down into tickets
await mcp__jility__create_ticket({
  title: "Build login UI",
  parent_epic_id: epic_id,
  story_points: 3,
  labels: ["frontend", "ui"]
})

// Claude implements, you review in the UI
// Add feedback via comments
await mcp__jility__add_comment(ticket_id,
  "Looks good! Let's add error handling for invalid credentials"
)
```

### Small Agency Team
- Replace expensive JIRA/Linear subscription ($10-15/user/month)
- Self-host on your infrastructure (one-time setup)
- AI agents handle routine tasks (tests, migrations, docs)
- Humans focus on creative/strategic work
- Full transparency - see exactly what agents did and why

### AI-First Development Workflow
1. **Plan in UI** - Create epics and high-level tickets in the web interface
2. **Assign to Claude** - Let Claude Code break down features and implement
3. **Review via MCP** - Claude can self-review, update statuses, add comments
4. **Track Progress** - Real-time updates in the UI as Claude works
5. **Iterate** - Claude responds to your feedback and adjusts implementation

---

## Roadmap

### Phase 1: MVP Core ✅ Complete
- ✅ CLI with ticket CRUD
- ✅ SQLite storage with migrations
- ✅ Description versioning and precise editing
- ✅ Basic local workflow

### Phase 2: MCP Server ✅ Complete
- ✅ Full MCP protocol implementation
- ✅ Agent ticket creation and management
- ✅ Context bundling for LLMs
- ✅ Template system

### Phase 3: Web UI ✅ Complete
- ✅ Beautiful Kanban board
- ✅ Ticket detail view with markdown
- ✅ Real-time updates via WebSocket
- ✅ Command palette (⌘K)
- ✅ Agent activity dashboard

### Phase 4: Polish & Cloud ✅ Complete
- ✅ Advanced search and filtering
- ✅ Sprint management
- ✅ Git integration
- ✅ PostgreSQL support
- ✅ Optional cloud deployment

### Foundation & Epic Sprint ✅ Complete (November 2025)
- ✅ Foundation cleanup (MCP comments, soft delete, attribution fixes)
- ✅ Epic support with JIRA-like hierarchy
- ✅ Epic progress tracking and visualization
- ✅ Epic filtering on boards
- ✅ Color-coded epic badges
- ✅ Consolidated settings UI
- ✅ UI polish (Quick Add fixes, navigation cleanup)

### Current Phase: Production Ready
Jility is feature-complete for solo developer workflows with AI assistants. Focus is now on:
- Performance optimization
- Mobile experience improvements
- Team collaboration features (future)

---

## Philosophy

1. **Agent-first, human-friendly** - AI agents are teammates, not tools
2. **Speed above all** - Every interaction should feel instant
3. **Beautiful by default** - Good design shouldn't be optional
4. **Local-first** - Your data lives with your code
5. **Progressive disclosure** - Simple by default, powerful when needed

---

## Comparison

| Feature | Jility | JIRA | Linear |
|---------|--------|------|--------|
| Speed | ⚡ Instant | 🐌 Slow | ⚡ Fast |
| AI Agents | 🤖 Native | ❌ None | ❌ None |
| Self-hosted | ✅ Yes | 💰 Expensive | ❌ No |
| Price | 💰 Per-project | 💰💰 Per-seat | 💰💰 Per-seat |
| Local-first | ✅ Yes | ❌ Cloud-only | ❌ Cloud-only |
| Open Source | 🎯 Coming | ❌ No | ❌ No |

---

## Contributing

Jility is under active development. We'll open source once Phase 1 MVP is stable.

Interested in:
- Early access?
- Beta testing?
- Contributing to the project?

Reach out! We'd love to hear your feedback.

---

## Name Origin

**Jility** = **Ji**RA + Agi**lity** + Uti**lity**

A nimble, practical tool that takes the best parts of JIRA's structure while ditching the bloat, built for the age of AI-assisted development.

---

## License

TBD (likely MIT or Apache 2.0)

---

## Contact

Built with ⚡ by developers tired of slow, expensive project management tools.
