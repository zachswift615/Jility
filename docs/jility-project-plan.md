# Jility Project Plan
## AI-Native Project Management System

**Vision:** A lightweight, fast, agent-first project management tool that makes JIRA seem bloated while being beautiful enough for design-conscious teams.

**Target Users:** 
- Solo developers using AI coding assistants
- Small software agencies (2-20 people)
- Teams frustrated with JIRA's complexity and cost
- Agent-driven development workflows

---

## Architecture Overview

### Tech Stack

**Backend:**
- **Language:** Rust
- **Web Framework:** Axum (fast, ergonomic)
- **Database:** SQLite (local) / PostgreSQL (cloud)
- **Real-time:** WebSockets via tokio-tungstenite
- **MCP Server:** Built on Anthropic's MCP protocol
- **API:** RESTful + WebSocket for real-time updates

**Frontend:**
- **Framework:** Next.js 14 (React with App Router)
- **Styling:** Tailwind CSS
- **State Management:** Zustand (lightweight)
- **Real-time:** WebSocket client
- **Markdown:** marked + syntax highlighting (highlight.js)
- **Drag & Drop:** @dnd-kit/core

**CLI:**
- **Language:** Rust (same codebase as backend)
- **CLI Framework:** clap v4
- **Editor Integration:** $EDITOR support for descriptions

**Infrastructure:**
- **Local:** Single binary + SQLite file
- **Cloud (Phase 4):** Docker container + Postgres
- **File Storage:** Git repo (`.hive/` directory)

### Project Structure

```
jility/
├── Cargo.toml
├── README.md
├── crates/
│   ├── jility-core/          # Shared types, models, business logic
│   │   ├── src/
│   │   │   ├── models/     # Ticket, Sprint, Comment, etc.
│   │   │   ├── storage/    # Database traits and implementations
│   │   │   ├── types/      # Common types and enums
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── jility-cli/           # Command-line interface
│   │   ├── src/
│   │   │   ├── commands/   # CLI command implementations
│   │   │   ├── editor.rs   # $EDITOR integration
│   │   │   ├── output.rs   # Formatting and display
│   │   │   └── main.rs
│   │   └── Cargo.toml
│   ├── jility-server/        # Web server and API
│   │   ├── src/
│   │   │   ├── api/        # REST endpoints
│   │   │   ├── websocket/  # Real-time updates
│   │   │   ├── auth.rs     # Authentication (future)
│   │   │   └── main.rs
│   │   └── Cargo.toml
│   ├── jility-mcp/           # MCP server for AI agents
│   │   ├── src/
│   │   │   ├── handlers/   # MCP request handlers
│   │   │   ├── protocol.rs # MCP protocol implementation
│   │   │   └── main.rs
│   │   └── Cargo.toml
│   └── jility-web/           # Next.js frontend
│       ├── app/
│       │   ├── board/
│       │   ├── ticket/[id]/
│       │   ├── agents/
│       │   └── layout.tsx
│       ├── components/
│       │   ├── ui/         # Reusable UI components
│       │   ├── kanban/
│       │   └── ticket/
│       ├── lib/
│       │   ├── api.ts      # API client
│       │   └── websocket.ts
│       └── package.json
└── docs/
    ├── API.md
    ├── MCP_PROTOCOL.md
    └── ARCHITECTURE.md
```

---

## Data Model

```rust
// Core entities

struct Project {
    id: Uuid,
    name: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

struct Ticket {
    id: Uuid,
    ticket_number: i32,           // Auto-incrementing: TASK-1, TASK-2, etc.
    project_id: Uuid,
    title: String,
    description: String,          // Markdown content
    description_version: i32,     // For version control
    status: TicketStatus,
    story_points: Option<i32>,
    assignee: Option<String>,     // "agent-1", "human-alice", etc.
    labels: Vec<String>,
    epic_id: Option<Uuid>,
    parent_id: Option<Uuid>,      // For sub-tasks
    depends_on: Vec<Uuid>,        // Ticket dependencies
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    created_by: String,
}

enum TicketStatus {
    Backlog,
    Todo,
    InProgress,
    Review,
    Done,
    Blocked,
}

struct DescriptionVersion {
    id: Uuid,
    ticket_id: Uuid,
    version: i32,
    content: String,
    changed_by: String,
    created_at: DateTime<Utc>,
    edit_metadata: Option<EditMetadata>,  // Line ranges changed
}

struct EditMetadata {
    start_line: usize,
    end_line: usize,
    operation: EditOperation,  // Replace, Insert, Delete
}

struct Comment {
    id: Uuid,
    ticket_id: Uuid,
    author: String,
    content: String,
    created_at: DateTime<Utc>,
}

struct Activity {
    id: Uuid,
    ticket_id: Option<Uuid>,
    project_id: Uuid,
    actor: String,
    action: ActivityType,
    metadata: serde_json::Value,
    created_at: DateTime<Utc>,
}

enum ActivityType {
    TicketCreated,
    TicketUpdated,
    StatusChanged,
    DescriptionEdited,
    CommentAdded,
    CommitLinked,
    AssigneeChanged,
    // ... more types
}

struct CommitLink {
    id: Uuid,
    ticket_id: Uuid,
    commit_hash: String,
    commit_message: Option<String>,
    linked_at: DateTime<Utc>,
    linked_by: String,
}

struct Sprint {
    id: Uuid,
    project_id: Uuid,
    name: String,
    goal: Option<String>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    status: SprintStatus,
}

enum SprintStatus {
    Planning,
    Active,
    Completed,
}
```

---

## Development Phases

### **Phase 1: MVP Core (2-3 weeks)**
*Goal: Working local CLI + basic storage*

#### Epic 1.1: Project Initialization & Storage
- **TASK-1**: Setup Rust workspace with multiple crates
  - Initialize monorepo structure
  - Configure Cargo workspace
  - Setup basic CI/CD
  - Story Points: 2

- **TASK-2**: Implement SQLite storage layer
  - Create database schema with migrations
  - Implement connection pooling
  - Add CRUD operations for tickets
  - Add transaction support
  - Story Points: 5

- **TASK-3**: Create core domain models
  - Define Ticket, Project, Comment structs
  - Implement serialization/deserialization
  - Add validation logic
  - Story Points: 3

- **TASK-4**: Implement `.jility/` directory initialization
  - Create config.toml for project settings
  - Initialize SQLite database in `.jility/data.db`
  - Store project metadata
  - Story Points: 2

#### Epic 1.2: Basic CLI
- **TASK-5**: Setup CLI with clap
  - Define command structure
  - Implement argument parsing
  - Add shell completions
  - Story Points: 3

- **TASK-6**: Implement `jility init` command
  - Create `.jility/` directory
  - Initialize database
  - Setup default configuration
  - Story Points: 2

- **TASK-7**: Implement ticket CRUD commands
  - `jility ticket create`
  - `jility ticket list`
  - `jility ticket show <id>`
  - `jility ticket update <id>`
  - Story Points: 5

- **TASK-8**: Add $EDITOR integration for descriptions
  - Launch editor with temp file
  - Parse edited content
  - Handle cancellation
  - Story Points: 3

- **TASK-9**: Implement status management
  - `jility ticket move <id> --to=<status>`
  - Add status validation
  - Record status changes in activity log
  - Story Points: 2

#### Epic 1.3: Description Editing System
- **TASK-10**: Implement description versioning
  - Store each version in database
  - Track who made changes and when
  - Generate diffs between versions
  - Story Points: 5

- **TASK-11**: Add precise line-based editing
  - Parse edit commands (line ranges)
  - Apply edits to description content
  - Validate line numbers
  - Store edit metadata
  - Story Points: 5

- **TASK-12**: Implement `jility ticket edit` command
  - Support full replacement (default)
  - Support `--lines X-Y` for precise edits
  - Support `--append` flag
  - Story Points: 3

- **TASK-13**: Add `jility ticket history` command
  - Show version history
  - Display diffs between versions
  - Allow reverting to previous version
  - Story Points: 3

---

### **Phase 2: MCP Server & Agent Integration (2 weeks)**
*Goal: AI agents can create and manage tickets*

#### Epic 2.1: MCP Server Implementation
- **TASK-14**: Setup MCP server scaffold
  - Implement MCP protocol basics
  - Add JSON-RPC handling
  - Create server lifecycle management
  - Story Points: 5

- **TASK-15**: Implement ticket creation methods
  - `create_ticket(title, description, ...)`
  - `create_tickets_batch(tickets[])`
  - `create_epic(title, description, ...)`
  - Story Points: 3

- **TASK-16**: Implement ticket query methods
  - `get_ticket(id)`
  - `list_tickets(filters)`
  - `search_tickets(query)`
  - Return full context for agents
  - Story Points: 3

- **TASK-17**: Implement ticket update methods
  - `update_ticket_description(id, content)`
  - `edit_ticket_description(id, edits)`
  - `append_ticket_description(id, content)`
  - `update_ticket_status(id, status)`
  - Story Points: 5

- **TASK-18**: Add activity and comment methods
  - `add_comment(ticket_id, content)`
  - `link_commit(ticket_id, hash, message)`
  - `get_activity(ticket_id)`
  - Story Points: 3

- **TASK-19**: Implement dependency management
  - `add_dependency(ticket_id, depends_on_id)`
  - `remove_dependency(ticket_id, depends_on_id)`
  - `get_dependency_graph(ticket_id)`
  - Story Points: 3

#### Epic 2.2: Agent Workflows
- **TASK-20**: Implement ticket claiming/assignment
  - Agent can claim unassigned tickets
  - Handle concurrent claims (locking)
  - Track agent activity timestamps
  - Story Points: 5

- **TASK-21**: Add template system for agents
  - Define template format (YAML/JSON)
  - `create_from_template(template_name, params)`
  - Include built-in templates (API endpoint, migration, etc.)
  - Story Points: 5

- **TASK-22**: Implement ticket decomposition
  - `decompose_ticket(id)` - agent analyzes and creates sub-tickets
  - Link sub-tickets to parent
  - Redistribute story points
  - Story Points: 5

- **TASK-23**: Add smart context bundling
  - When agent claims ticket, bundle related info
  - Include dependencies, linked tickets, relevant comments
  - Format for optimal LLM consumption
  - Story Points: 3

---

### **Phase 3: Web UI (3 weeks)**
*Goal: Beautiful, fast web interface*

#### Epic 3.1: Frontend Setup & Core UI
- **TASK-24**: Setup Next.js project with Tailwind
  - Initialize Next.js 14 with App Router
  - Configure Tailwind with custom theme
  - Setup TypeScript and ESLint
  - Story Points: 2

- **TASK-25**: Create design system components
  - Button, Input, Badge, Card components
  - Status indicators
  - Avatar/icon system
  - Typography components
  - Story Points: 5

- **TASK-26**: Build navigation and layout
  - Top navigation bar
  - Sidebar (collapsible)
  - Responsive layout
  - Keyboard shortcut system foundation
  - Story Points: 3

- **TASK-27**: Implement API client
  - REST client with fetch
  - Error handling and retries
  - TypeScript types from backend
  - Story Points: 3

#### Epic 3.2: Kanban Board
- **TASK-28**: Build kanban board layout
  - Column-based layout
  - Horizontal scroll on mobile
  - Column collapse/expand
  - Story Points: 5

- **TASK-29**: Implement ticket cards
  - Card component with metadata
  - Ticket preview
  - Status badges and labels
  - Agent indicator
  - Story Points: 3

- **TASK-30**: Add drag-and-drop functionality
  - Integrate @dnd-kit
  - Handle status changes on drop
  - Optimistic UI updates
  - Add keyboard alternative
  - Story Points: 5

- **TASK-31**: Implement real-time updates via WebSocket
  - WebSocket connection management
  - Listen for ticket updates
  - Update UI when agents make changes
  - Show "Agent-1 is typing..." indicators
  - Story Points: 5

#### Epic 3.3: Ticket Detail View
- **TASK-32**: Build ticket detail page
  - Header with metadata
  - Markdown rendering for description
  - Syntax highlighting for code blocks
  - Story Points: 5

- **TASK-33**: Implement inline editing
  - Click-to-edit for title
  - Description editor (markdown)
  - Auto-save on blur
  - Story Points: 5

- **TASK-34**: Add activity timeline
  - Chronological activity feed
  - Different styles for different activity types
  - Load more pagination
  - Story Points: 3

- **TASK-35**: Implement comment system
  - Comment input with markdown support
  - Comment threading (optional)
  - Real-time comment updates
  - Story Points: 5

- **TASK-36**: Show description history and diffs
  - Version selector
  - Side-by-side or unified diff view
  - "Revert to version X" button
  - Story Points: 5

#### Epic 3.4: Command Palette & Keyboard Navigation
- **TASK-37**: Build command palette UI
  - ⌘K trigger
  - Fuzzy search
  - Keyboard navigation
  - Action categories
  - Story Points: 5

- **TASK-38**: Implement quick actions
  - Create ticket
  - Search tickets
  - Navigate to views
  - Change status
  - Story Points: 3

- **TASK-39**: Add global keyboard shortcuts
  - G+B (Board), G+A (Agents), etc.
  - C (Create), / (Search)
  - Arrow keys for navigation
  - Story Points: 3

#### Epic 3.5: Agent Dashboard
- **TASK-40**: Build agent activity panel
  - List active agents
  - Show current task
  - Last activity timestamp
  - Real-time status updates
  - Story Points: 5

- **TASK-41**: Create sprint statistics
  - Ticket counts by status
  - Completion percentage
  - Burndown chart (simple)
  - Story Points: 5

- **TASK-42**: Add recent activity feed
  - Global activity stream
  - Filter by agent/human
  - Filter by activity type
  - Story Points: 3

---

### **Phase 4: Advanced Features (2-3 weeks)**
*Goal: Polish, performance, and cloud deployment*

#### Epic 4.1: Search & Filtering
- **TASK-43**: Implement full-text search
  - SQLite FTS5 integration
  - Search across title, description, comments
  - Fuzzy matching
  - Story Points: 5

- **TASK-44**: Add advanced filtering
  - Filter by status, assignee, labels
  - Date range filters
  - Story point range
  - Dependency filters
  - Story Points: 5

- **TASK-45**: Create saved views
  - Save filter combinations
  - Name and organize views
  - Share views (future: team)
  - Story Points: 3

#### Epic 4.2: Sprint Management
- **TASK-46**: Implement sprint CRUD
  - Create, list, update sprints
  - Add tickets to sprint
  - Sprint planning view
  - Story Points: 5

- **TASK-47**: Add sprint planning assistance
  - Agent-assisted sprint planning
  - Story point capacity recommendations
  - Dependency conflict detection
  - Story Points: 5

- **TASK-48**: Build sprint dashboard
  - Active sprint view
  - Burndown chart
  - Velocity tracking
  - Sprint retrospective notes
  - Story Points: 5

#### Epic 4.3: Git Integration
- **TASK-49**: Auto-link commits from messages
  - Parse commit messages for TASK-XX
  - Auto-create commit links
  - Git hook integration
  - Story Points: 5

- **TASK-50**: Branch name suggestions
  - Generate branch names from ticket
  - `git hive checkout TASK-XX` command
  - Auto-move ticket to in-progress
  - Story Points: 3

#### Epic 4.4: Cloud Deployment Option
- **TASK-51**: Add PostgreSQL support
  - Abstract database layer
  - Implement Postgres adapter
  - Migration from SQLite
  - Story Points: 5

- **TASK-52**: Implement authentication
  - JWT-based auth
  - User registration/login
  - API key for agents
  - Story Points: 8

- **TASK-53**: Add multi-project support
  - Project isolation
  - Per-project permissions
  - Project switching UI
  - Story Points: 5

- **TASK-54**: Create Docker deployment
  - Dockerfile for server
  - Docker Compose setup
  - Environment configuration
  - Documentation
  - Story Points: 3

- **TASK-55**: Build simple hosted offering
  - Deployment automation
  - Billing integration (Stripe)
  - Usage limits
  - Admin dashboard
  - Story Points: 13

---

## Technical Specifications

### Precise Description Editing Protocol

```rust
// Edit operations supported by MCP server and CLI

enum EditOperation {
    FullReplace {
        content: String,
    },
    LineReplace {
        start_line: usize,
        end_line: usize,
        new_content: String,
    },
    Append {
        content: String,
    },
    Prepend {
        content: String,
    },
    SectionUpdate {
        section_header: String,  // e.g., "## Acceptance Criteria"
        new_content: String,
    },
}

// CLI usage examples:
// jility ticket edit TASK-123 --replace-lines 5-7 "New content"
// jility ticket edit TASK-123 --append "## Update\nAdded feature"
// jility ticket edit TASK-123 --section "Acceptance Criteria" "- [ ] New criterion"

// MCP usage examples:
// edit_ticket_description(TASK-123, {
//   operation: "line_replace",
//   start_line: 5,
//   end_line: 7,
//   content: "New content"
// })
```

### WebSocket Message Format

```typescript
// Server -> Client
type ServerMessage =
  | { type: 'ticket_updated', ticket: Ticket }
  | { type: 'ticket_created', ticket: Ticket }
  | { type: 'status_changed', ticket_id: string, old_status: string, new_status: string }
  | { type: 'comment_added', ticket_id: string, comment: Comment }
  | { type: 'agent_activity', agent: string, action: string, ticket_id?: string }
  | { type: 'description_edited', ticket_id: string, version: number }

// Client -> Server
type ClientMessage =
  | { type: 'subscribe_ticket', ticket_id: string }
  | { type: 'unsubscribe_ticket', ticket_id: string }
  | { type: 'subscribe_project' }
```

### MCP Server Methods

```typescript
// All methods available to AI agents

interface JilityMCP {
  // Ticket CRUD
  create_ticket(params: CreateTicketParams): Ticket
  create_tickets_batch(tickets: CreateTicketParams[]): Ticket[]
  get_ticket(id: string): TicketDetail
  list_tickets(filters: TicketFilters): Ticket[]
  search_tickets(query: string): Ticket[]
  
  // Ticket updates
  update_ticket_description(id: string, content: string): void
  edit_ticket_description(id: string, edit: EditOperation): void
  append_ticket_description(id: string, content: string): void
  update_ticket_status(id: string, status: TicketStatus): void
  update_ticket_metadata(id: string, updates: Partial<Ticket>): void
  
  // Assignment and claiming
  claim_ticket(id: string, assignee: string): void
  release_ticket(id: string): void
  
  // Comments and activity
  add_comment(ticket_id: string, content: string): Comment
  link_commit(ticket_id: string, hash: string, message?: string): void
  get_activity(ticket_id: string, limit?: number): Activity[]
  
  // Dependencies
  add_dependency(ticket_id: string, depends_on: string): void
  remove_dependency(ticket_id: string, depends_on: string): void
  get_dependency_graph(ticket_id: string): DependencyGraph
  
  // Templates and planning
  list_templates(): Template[]
  create_from_template(template: string, params: object): Ticket
  decompose_ticket(id: string): Ticket[]
  suggest_dependencies(ticket_id: string): string[]
  
  // Context bundling
  get_ticket_context(id: string): TicketContext
  // Returns: ticket + related tickets + comments + linked commits + dependencies
  
  // Epics and organization
  create_epic(params: CreateEpicParams): Epic
  add_to_epic(ticket_id: string, epic_id: string): void
  
  // Version history
  get_description_history(ticket_id: string): DescriptionVersion[]
  get_description_diff(ticket_id: string, v1: number, v2: number): Diff
  revert_description(ticket_id: string, version: number): void
}
```

---

## Success Metrics

**Phase 1 MVP:**
- ✅ Can create and manage tickets via CLI
- ✅ Description editing with version history works
- ✅ All data persists in SQLite

**Phase 2 MCP:**
- ✅ Claude Code can create tickets via MCP
- ✅ Agent can update descriptions precisely (token-efficient)
- ✅ Multiple agents can work in parallel without conflicts

**Phase 3 Web UI:**
- ✅ Beautiful, fast kanban board
- ✅ Real-time updates when agents modify tickets
- ✅ Mobile-friendly
- ✅ Keyboard shortcuts for power users

**Phase 4 Polish:**
- ✅ Sub-5-second deploy to cloud
- ✅ 10+ teams using it daily
- ✅ Positive feedback compared to JIRA

---

## Next Steps

1. **Review wireframes** - [View wireframes](computer:///mnt/user-data/outputs/hive-wireframes.html)
2. **Initialize repo** - Create GitHub repo with Rust workspace
3. **Start with TASK-1** - Setup project structure
4. **Parallel track** - Can work on CLI (Phase 1) while designing MCP protocol

Would you like to start building? We could:
- Initialize the Rust workspace and create the first few tickets
- Design the MCP protocol in detail
- Create mockups for specific UI components
- Set up the database schema

What feels most exciting to tackle first?