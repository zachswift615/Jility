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
- **Unified interface** - Humans and agents use the same CLI, API, and UI
- **Seamless handoffs** - Assign work between humans and agents with context
- **Pairing** - Multiple assignees (human + agent) can work together on tickets
- **@ mentions** - Natural communication between team members
- **Activity transparency** - See who (human or agent) did what, when

### Intelligent Workflows
- **Agent planning** - AI agents can break down epics into tickets
- **Context bundling** - Agents get full ticket history, dependencies, and guidance
- **Smart decomposition** - Agents can split complex tickets into sub-tasks
- **Template system** - Quick ticket creation for common patterns
- **Dependency tracking** - Automatic detection and management

### Developer Experience
- **Command palette (⌘K)** - Keyboard-driven for power users
- **Precise editing** - Token-efficient line-based description updates
- **Version control** - Full history of ticket changes with diffs
- **Git integration** - Auto-link commits, branch name suggestions
- **Real-time updates** - WebSocket-powered live collaboration
- **Markdown everywhere** - Native support with syntax highlighting

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
├── jility-cli       # Command-line interface
├── jility-server    # Axum web server + REST API
├── jility-mcp       # MCP server for AI agents
└── jility-web       # Next.js frontend
```

**Tech Stack:**
- Backend: Rust (Axum, SQLite/Postgres)
- Frontend: Next.js 14 + Tailwind CSS
- MCP: Anthropic's Model Context Protocol
- Real-time: WebSockets

---

## Getting Started

### Installation (coming soon)
```bash
# Install CLI
cargo install jility

# Initialize project
cd your-project
jility init

# Start server (includes web UI)
jility serve
```

### Quick Start
```bash
# Create a ticket
jility ticket create --title "Add user auth" --points 5

# Assign to agent
jility ticket assign TASK-1 --to=agent-1

# Check team status
jility team status

# View in web UI
open http://localhost:3000
```

---

## Development

### Prerequisites
- Rust (latest stable)
- Node.js 18+ and npm
- Git

### Quick Start with dev.sh

The easiest way to run Jility in development mode is using the `dev.sh` script:

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

**Ports:**
- Backend API: `http://localhost:3900`
- Frontend UI: `http://localhost:3901`

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
├── jility-server/      # Rust backend (Axum + SQLite)
├── jility-web/         # Next.js frontend
├── jility-core/        # Shared Rust models/logic
├── jility-cli/         # CLI tool (coming soon)
├── jility-mcp/         # MCP server for AI agents
└── dev.sh              # Development helper script
```

---

## Documentation

- **[Wireframes](jility-wireframes.html)** - Interactive UI mockups showing all key screens
- **[Project Plan](jility-project-plan.md)** - Complete development roadmap with 55 tickets across 4 phases
- **[CLI Guide](jility-cli-collaboration-guide.md)** - Comprehensive examples of human/agent collaboration

---

## Use Cases

### Solo Developer with AI Assistants
```bash
# You plan, agent implements
jility ticket create --title "Build checkout flow" --assignee=agent-1
jility ticket assign TASK-10 --to=agent-1 --message "Use Stripe SDK"

# Agent works, you review
jility review-queue
jility ticket comment TASK-10 "Looks good! Let's add error handling"
```

### Small Agency Team
- Replace expensive JIRA/Linear subscription
- Self-host on your infrastructure
- Agents handle routine tasks (tests, migrations, etc.)
- Humans focus on creative/strategic work

### AI-First Development
- Multiple agents work in parallel on different tickets
- Agents break down features into actionable tasks
- Humans provide high-level direction and review
- Full audit trail of agent decisions

---

## Roadmap

### Phase 1: MVP Core (2-3 weeks)
- ✅ CLI with ticket CRUD
- ✅ SQLite storage with migrations
- ✅ Description versioning and precise editing
- ✅ Basic local workflow

### Phase 2: MCP Server (2 weeks)
- ✅ Full MCP protocol implementation
- ✅ Agent ticket creation and management
- ✅ Context bundling for LLMs
- ✅ Template system

### Phase 3: Web UI (3 weeks)
- ✅ Beautiful Kanban board
- ✅ Ticket detail view with markdown
- ✅ Real-time updates via WebSocket
- ✅ Command palette (⌘K)
- ✅ Agent activity dashboard

### Phase 4: Polish & Cloud (2-3 weeks)
- ✅ Advanced search and filtering
- ✅ Sprint management
- ✅ Git integration
- ✅ PostgreSQL support
- ✅ Optional cloud deployment

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

Project maintainer: [Your details here]

Built with ⚡ by developers tired of slow, expensive project management tools.
