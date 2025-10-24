# Jility MCP Protocol Design
## Model Context Protocol Server for Claude Code Integration

**Version:** 1.0
**Last Updated:** 2024-10-23
**Status:** Design Specification

---

## Overview

Jility's MCP server enables AI agents (primarily Claude Code) to create and manage tickets through the Model Context Protocol. The server supports two deployment modes:

1. **Local Mode** - Direct SQLite access for single-user, local-first workflows
2. **Cloud Mode** - Bridge to remote Jility API with token authentication

---

## Reference Implementation

**Jility can use the proven MCP architecture from agent-power-tools** - a working Rust MCP server that's Claude Code compatible and already deployed.

### Existing Implementation

**Location:** `/Users/zachswift/projects/agent-power-tools`
**GitHub:** `https://github.com/zachswift615/agent-power-tools`
**Status:** Production-ready, Claude Code compatible

**Key Files:**
- `powertools-cli/src/mcp/server.rs` - MCP server runner using `rmcp` crate
- `powertools-cli/src/mcp/tools.rs` - Tool definitions and handlers
- `powertools-cli/src/main.rs` - CLI entry point with `--mcp-server` flag

### Technology Stack

**Uses the `rmcp` crate** (Rust MCP library):
- `rmcp::ServiceExt` - Server lifecycle management
- `#[rmcp::tool_router]` macro - Automatic tool registration
- `#[tool(description = "...")]` - Tool definitions with JSON schemas
- `Parameters<T>` - Type-safe parameter handling
- `CallToolResult` - Standardized response format

### Code Structure Overview

```rust
use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Content},
    tool, ErrorData as McpError, ServerHandler,
};
use schemars::JsonSchema;
use serde::Deserialize;

// Service struct with state
#[derive(Clone)]
pub struct JilityService {
    tool_router: ToolRouter<Self>,
    db: Arc<DatabaseConnection>,  // SeaORM connection
    project_root: PathBuf,
}

// Parameter types with JSON Schema
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateTicketParams {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub story_points: Option<i32>,
    // ... more fields
}

// Tool implementations using macro
#[rmcp::tool_router]
impl JilityService {
    /// Create a new ticket
    #[tool(description = "Create a new ticket in Jility with title, description, and metadata.")]
    async fn create_ticket(
        &self,
        Parameters(params): Parameters<CreateTicketParams>,
    ) -> Result<CallToolResult, McpError> {
        // Business logic here
        match create_ticket_in_db(&self.db, params).await {
            Ok(ticket) => Ok(CallToolResult::success(vec![Content::text(
                format!("✅ Created ticket {}", ticket.number)
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(
                format!("Failed to create ticket: {}", e)
            )])),
        }
    }

    // ... more tools
}

// Server handler for metadata
#[rmcp::tool_handler]
impl ServerHandler for JilityService {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        rmcp::model::ServerInfo {
            protocol_version: ProtocolVersion::default(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: None,
                }),
                ..Default::default()
            },
            server_info: Implementation {
                name: "jility-mcp".to_string(),
                title: Some("Jility MCP Server".to_string()),
                version: env!("CARGO_PKG_VERSION").to_string(),
                icons: None,
                website_url: Some("https://github.com/yourusername/jility".to_string()),
            },
            instructions: Some(
                "Jility provides AI-native project management. \
                 Use create_ticket to add tasks, update_description for precise edits, \
                 and get_ticket_context to see full ticket details.".to_string()
            ),
        }
    }
}
```

### Server Runner

```rust
// From powertools-cli/src/mcp/server.rs
pub async fn run_mcp_server() -> Result<()> {
    // Get current directory (or .jility/ in project root)
    let current_dir = std::env::current_dir()?;
    let db_path = current_dir.join(".jility/data.db");

    // Connect to database
    let db = connect_to_database(&db_path).await?;

    // Create the service
    let service = JilityService::new(db, current_dir)?;

    // Start the server with stdio transport
    let peer = service
        .serve((tokio::io::stdin(), tokio::io::stdout()))
        .await?;

    // Wait for the service to complete
    peer.waiting().await?;

    Ok(())
}
```

### CLI Integration

```rust
// From powertools-cli/src/main.rs
#[derive(Parser)]
struct Cli {
    /// Run as MCP (Model Context Protocol) server
    #[arg(long)]
    mcp_server: bool,

    // ... other flags
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Check if running as MCP server
    if cli.mcp_server {
        return mcp::run_mcp_server().await;
    }

    // Otherwise run as normal CLI
    // ...
}
```

### Key Differences for Jility

**agent-power-tools:**
- Manages code indexes and navigation
- Single project root
- No authentication needed (local only)

**Jility adaptations:**
- Manages tickets, comments, dependencies
- Multi-mode: local (direct DB) vs cloud (HTTP bridge)
- Cloud mode needs authentication
- Database operations via SeaORM instead of file I/O

### Dependencies for Jility

```toml
[dependencies]
# MCP server
rmcp = "0.1"  # Rust MCP library
schemars = "0.8"  # JSON Schema generation
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Database (already specified in database design doc)
sea-orm = { version = "0.12", features = ["sqlx-sqlite", "runtime-tokio-native-tls", "macros"] }

# Async runtime
tokio = { version = "1", features = ["full"] }

# Error handling
anyhow = "1.0"
```

### Proven Patterns to Reuse

1. **Parameter validation** - Use `schemars::JsonSchema` derive
2. **Error handling** - Return `CallToolResult::error()` with user-friendly messages
3. **State management** - Store DB connection in service struct
4. **Tool registration** - Use `#[rmcp::tool_router]` macro for automatic discovery
5. **Stdio transport** - Use `service.serve((stdin, stdout))` pattern

### Testing with Claude Code

**From agent-power-tools `.mcp.json`:**
```json
{
  "mcpServers": {
    "powertools": {
      "command": "powertools",
      "args": ["--mcp-server"]
    }
  }
}
```

**For Jility:**
```json
{
  "mcpServers": {
    "jility": {
      "command": "jility",
      "args": ["--mcp-server"],
      "env": {
        "JILITY_MODE": "local"
      }
    }
  }
}
```

---

## Architecture

### Mode 1: Local (Phase 1-3)

```
┌─────────────────────────────────────────────────────┐
│ User's Development Machine                          │
│                                                     │
│  ┌──────────────┐                                  │
│  │ Claude Code  │                                  │
│  │   (Agent)    │                                  │
│  └──────┬───────┘                                  │
│         │ stdio (JSON-RPC)                         │
│         │                                           │
│  ┌──────▼───────────────────┐                      │
│  │   Jility MCP Server      │                      │
│  │  (jility --mcp-server)   │                      │
│  └──────┬───────────────────┘                      │
│         │                                           │
│         │ Direct DB access                         │
│         │                                           │
│  ┌──────▼───────────────────┐                      │
│  │  SQLite Database         │                      │
│  │  .jility/data.db         │                      │
│  └──────────────────────────┘                      │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Use Case:** Solo developer or small team working locally, all data in `.jility/` directory.

### Mode 2: Cloud Bridge (Phase 4)

```
┌─────────────────────────────────────────────────────┐
│ User's Development Machine                          │
│                                                     │
│  ┌──────────────┐                                  │
│  │ Claude Code  │                                  │
│  │   (Agent)    │                                  │
│  └──────┬───────┘                                  │
│         │ stdio (JSON-RPC)                         │
│         │                                           │
│  ┌──────▼───────────────────┐                      │
│  │  Jility MCP Bridge       │                      │
│  │  (jility --mcp-server)   │                      │
│  │                          │                      │
│  │  - Stdio ↔ HTTP adapter  │                      │
│  │  - Token auth            │                      │
│  │  - Response caching      │                      │
│  └──────┬───────────────────┘                      │
│         │                                           │
│         │ HTTPS + Bearer Token                     │
└─────────┼─────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────┐
│ Cloud (AWS / Fly.io / Self-hosted)                  │
│                                                     │
│  ┌──────────────────────────┐                      │
│  │   Jility API Server      │                      │
│  │   (Axum REST + WS)       │                      │
│  │                          │                      │
│  │  - JWT auth              │                      │
│  │  - Multi-tenant          │                      │
│  │  - Rate limiting         │                      │
│  └──────┬───────────────────┘                      │
│         │                                           │
│         ▼                                           │
│  ┌──────────────────────────┐                      │
│  │   PostgreSQL             │                      │
│  │   (Multi-tenant)         │                      │
│  └──────────────────────────┘                      │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Use Case:** Team using hosted Jility, agents on different machines all connecting to shared cloud instance.

---

## Configuration

### Claude Code `.mcp.json` Setup

Place at project root (committed to git for team):

**Local Mode:**
```json
{
  "mcpServers": {
    "jility": {
      "command": "jility",
      "args": ["--mcp-server"],
      "env": {
        "JILITY_MODE": "local"
      }
    }
  }
}
```

**Cloud Mode:**
```json
{
  "mcpServers": {
    "jility": {
      "command": "jility",
      "args": ["--mcp-server"],
      "env": {
        "JILITY_MODE": "cloud",
        "JILITY_API_URL": "https://api.jility.app",
        "JILITY_API_TOKEN": "${JILITY_TOKEN}"
      }
    }
  }
}
```

Users set token via:
```bash
export JILITY_TOKEN="jil_live_abc123..."
# or
jility auth login --token=jil_live_abc123...
```

---

## Authentication

### Local Mode
- No authentication required
- MCP server accesses local `.jility/data.db` in current directory
- Uses file system permissions for security

### Cloud Mode
- **API Token required** - Long-lived personal access token
- Token format: `jil_{env}_{random}` (e.g., `jil_live_a1b2c3d4e5f6`)
- Passed via environment variable `JILITY_API_TOKEN`
- MCP bridge includes token in `Authorization: Bearer` header

**Token Management:**
```bash
# Generate new token (via web UI or CLI)
jility auth create-token --name="claude-code-laptop"

# List tokens
jility auth list-tokens

# Revoke token
jility auth revoke-token jil_live_abc123
```

**Token Scopes (Phase 4):**
- `tickets:read` - Read tickets
- `tickets:write` - Create/update tickets
- `tickets:delete` - Delete tickets
- `comments:write` - Add comments
- `full_access` - All operations (default for MVP)

---

## MCP Protocol Implementation

### Lifecycle Methods

**1. Initialize**

Request:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "tools": {}
    },
    "clientInfo": {
      "name": "claude-code",
      "version": "1.0.0"
    }
  }
}
```

Response:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "tools": {}
    },
    "serverInfo": {
      "name": "jility-mcp",
      "version": "0.1.0"
    }
  }
}
```

**2. List Tools**

Request:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list"
}
```

Response:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "tools": [
      {
        "name": "jility_create_ticket",
        "description": "Create a new ticket in Jility",
        "inputSchema": {
          "type": "object",
          "properties": {
            "title": { "type": "string", "description": "Ticket title" },
            "description": { "type": "string", "description": "Markdown description" },
            "story_points": { "type": "integer", "description": "Effort estimate (1-13)" },
            "status": {
              "type": "string",
              "enum": ["backlog", "todo", "in_progress", "review", "done", "blocked"],
              "default": "backlog"
            },
            "assignees": {
              "type": "array",
              "items": { "type": "string" },
              "description": "List of assignees (e.g., ['agent-1', 'alice'])"
            },
            "labels": {
              "type": "array",
              "items": { "type": "string" },
              "description": "Labels like 'backend', 'frontend', 'bug'"
            },
            "parent_id": { "type": "string", "description": "Parent ticket ID for sub-tasks" }
          },
          "required": ["title"]
        }
      }
      // ... more tools (see below)
    ]
  }
}
```

**3. Call Tool**

Request:
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "jility_create_ticket",
    "arguments": {
      "title": "Implement user authentication",
      "description": "## Context\n\nAdd JWT-based auth to API.\n\n## Acceptance Criteria\n\n- [ ] Users can register\n- [ ] Users can login\n- [ ] Tokens expire after 7 days",
      "story_points": 5,
      "assignees": ["agent-1"],
      "labels": ["backend", "security"]
    }
  }
}
```

Response (success):
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "✅ Created ticket TASK-123\n\n**Title:** Implement user authentication\n**Status:** Backlog\n**Story Points:** 5\n**Assignees:** agent-1\n**Labels:** backend, security\n\nView: `jility ticket show TASK-123`"
      }
    ],
    "isError": false
  }
}
```

Response (error):
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "❌ Error: Invalid status 'in-progress'. Valid statuses: backlog, todo, in_progress, review, done, blocked"
      }
    ],
    "isError": true
  }
}
```

---

## Core MCP Tools

### 1. Ticket Management

#### `jility_create_ticket`

**Description:** Create a new ticket with title, description, and metadata.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "title": { "type": "string" },
    "description": { "type": "string" },
    "story_points": { "type": "integer", "minimum": 1, "maximum": 13 },
    "status": {
      "type": "string",
      "enum": ["backlog", "todo", "in_progress", "review", "done", "blocked"],
      "default": "backlog"
    },
    "assignees": { "type": "array", "items": { "type": "string" } },
    "labels": { "type": "array", "items": { "type": "string" } },
    "parent_id": { "type": "string" },
    "epic_id": { "type": "string" }
  },
  "required": ["title"]
}
```

**Output Example:**
```
✅ Created ticket TASK-42

**Title:** Add password validation
**Status:** Backlog
**Story Points:** 3
**Assignees:** agent-1
**Labels:** backend, validation

View: `jility ticket show TASK-42`
```

---

#### `jility_create_tickets_batch`

**Description:** Create multiple tickets at once (useful when agent breaks down epic).

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "tickets": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "title": { "type": "string" },
          "description": { "type": "string" },
          "story_points": { "type": "integer" },
          "assignees": { "type": "array", "items": { "type": "string" } },
          "labels": { "type": "array", "items": { "type": "string" } },
          "depends_on": { "type": "array", "items": { "type": "string" } }
        },
        "required": ["title"]
      }
    },
    "parent_id": { "type": "string", "description": "Epic or parent ticket" }
  },
  "required": ["tickets"]
}
```

**Output Example:**
```
✅ Created 5 tickets under EPIC-10

- TASK-50: Setup database schema (3 pts) → agent-1
- TASK-51: Create user model (2 pts) → agent-1
- TASK-52: Build registration endpoint (5 pts) → agent-2
- TASK-53: Add password hashing (3 pts) → agent-2
- TASK-54: Write integration tests (5 pts) → unassigned

Total: 18 story points
```

---

#### `jility_get_ticket`

**Description:** Get full ticket context including comments, dependencies, history.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "ticket_id": { "type": "string" }
  },
  "required": ["ticket_id"]
}
```

**Output Example:**
```json
{
  "ticket": {
    "id": "uuid-123",
    "number": "TASK-42",
    "title": "Implement JWT token generation",
    "description": "## Context\n\n...",
    "status": "in_progress",
    "story_points": 3,
    "assignees": ["agent-1"],
    "labels": ["backend", "security"],
    "created_at": "2024-10-20T10:00:00Z",
    "created_by": "alice"
  },
  "comments": [
    {
      "author": "alice",
      "content": "Make sure to handle token expiration properly",
      "created_at": "2024-10-20T14:30:00Z"
    }
  ],
  "dependencies": [
    { "number": "TASK-40", "title": "Create User model", "status": "done" }
  ],
  "dependents": [
    { "number": "TASK-45", "title": "Add login endpoint", "status": "todo" }
  ],
  "linked_commits": [
    {
      "hash": "abc123f",
      "message": "Add JWT service",
      "linked_at": "2024-10-20T15:00:00Z"
    }
  ],
  "recent_changes": [
    {
      "type": "status_changed",
      "old_value": "todo",
      "new_value": "in_progress",
      "changed_by": "agent-1",
      "changed_at": "2024-10-20T13:00:00Z"
    }
  ]
}
```

---

#### `jility_list_tickets`

**Description:** Query tickets with filters.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "status": {
      "type": "array",
      "items": { "type": "string" },
      "description": "Filter by status (can select multiple)"
    },
    "assignee": { "type": "string", "description": "Filter by assignee" },
    "labels": { "type": "array", "items": { "type": "string" } },
    "parent_id": { "type": "string", "description": "Get sub-tasks of parent" },
    "epic_id": { "type": "string", "description": "Get all tickets in epic" },
    "unassigned": { "type": "boolean", "description": "Show only unassigned tickets" },
    "limit": { "type": "integer", "default": 50, "maximum": 200 }
  }
}
```

**Output Example:**
```
📋 Found 12 tickets

## In Progress (3)
- TASK-42: Implement JWT token generation (agent-1) • 3 pts
- TASK-43: Add password hashing (agent-2) • 2 pts
- TASK-44: Build login UI (alice) • 5 pts

## Todo (5)
- TASK-45: Add login endpoint (unassigned) • 3 pts
- TASK-46: Create user settings page (unassigned) • 5 pts
...

## Backlog (4)
- TASK-50: Add OAuth support (unassigned) • 8 pts
...
```

---

#### `jility_claim_ticket`

**Description:** Agent claims an unassigned ticket (auto-assigns to self).

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "ticket_id": { "type": "string" },
    "message": {
      "type": "string",
      "description": "Optional message when claiming (e.g., 'Starting work on this')"
    }
  },
  "required": ["ticket_id"]
}
```

**Output Example:**
```
✅ Claimed TASK-45 and assigned to agent-1

**Title:** Add login endpoint
**Status:** In Progress (moved from Todo)
**Story Points:** 3

Get context: `jility ticket show TASK-45`
```

---

### 2. Description Editing (Your Killer Feature!)

#### `jility_update_description`

**Description:** Precisely edit ticket description with line-based operations (token-efficient!).

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "ticket_id": { "type": "string" },
    "operation": {
      "type": "string",
      "enum": ["replace_all", "append", "prepend", "replace_lines", "replace_section"],
      "description": "Type of edit operation"
    },
    "content": {
      "type": "string",
      "description": "New content to insert"
    },
    "start_line": {
      "type": "integer",
      "description": "For replace_lines: starting line number (1-indexed)"
    },
    "end_line": {
      "type": "integer",
      "description": "For replace_lines: ending line number (inclusive)"
    },
    "section_header": {
      "type": "string",
      "description": "For replace_section: markdown header (e.g., '## Acceptance Criteria')"
    },
    "message": {
      "type": "string",
      "description": "Optional change message for history"
    }
  },
  "required": ["ticket_id", "operation", "content"]
}
```

**Examples:**

**Append to description:**
```json
{
  "ticket_id": "TASK-42",
  "operation": "append",
  "content": "\n## Update\n\n- Added error handling for expired tokens\n- Tests passing",
  "message": "Added progress update"
}
```

**Replace specific lines:**
```json
{
  "ticket_id": "TASK-42",
  "operation": "replace_lines",
  "start_line": 8,
  "end_line": 10,
  "content": "- [x] Generate JWT with configurable expiration\n- [x] Include user claims (id, email, role)\n- [ ] Sign with RS256 algorithm",
  "message": "Updated checklist progress"
}
```

**Replace entire section:**
```json
{
  "ticket_id": "TASK-42",
  "operation": "replace_section",
  "section_header": "## Acceptance Criteria",
  "content": "- [x] Generate JWT with expiration\n- [x] Include user claims\n- [x] Sign with RS256\n- [x] Unit tests complete",
  "message": "Marked all criteria complete"
}
```

**Output Example:**
```
✅ Updated description for TASK-42

**Operation:** replace_lines (lines 8-10)
**Changed by:** agent-1
**Message:** Updated checklist progress

Version history: `jility ticket history TASK-42`
```

---

### 3. Workflow & Status

#### `jility_update_status`

**Description:** Move ticket through workflow states.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "ticket_id": { "type": "string" },
    "status": {
      "type": "string",
      "enum": ["backlog", "todo", "in_progress", "review", "done", "blocked"]
    },
    "message": { "type": "string", "description": "Optional context message" }
  },
  "required": ["ticket_id", "status"]
}
```

**Output Example:**
```
✅ Moved TASK-42 to Review

**Previous:** In Progress
**Current:** Review
**Changed by:** agent-1

Ready for human review! 🎉
```

---

#### `jility_add_comment`

**Description:** Add comment to ticket (supports @mentions).

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "ticket_id": { "type": "string" },
    "content": { "type": "string", "description": "Markdown comment content" }
  },
  "required": ["ticket_id", "content"]
}
```

**Output Example:**
```
✅ Added comment to TASK-42

**From:** agent-1
**Content:**
> Implementation complete! @alice Please review the error handling logic.

View: `jility ticket show TASK-42`
```

---

#### `jility_assign_ticket`

**Description:** Assign or reassign ticket to humans or agents (supports pairing).

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "ticket_id": { "type": "string" },
    "assignees": {
      "type": "array",
      "items": { "type": "string" },
      "description": "List of assignees. Empty array = unassign."
    },
    "message": {
      "type": "string",
      "description": "Handoff message (e.g., 'I've set up the structure, please finish the implementation')"
    }
  },
  "required": ["ticket_id", "assignees"]
}
```

**Output Example:**
```
✅ Reassigned TASK-42

**Previous:** agent-1
**Current:** alice, agent-1 (pairing)
**Message:** "I've implemented the JWT service, but need help with edge cases"

Handoff message added as comment.
```

---

### 4. Git Integration

#### `jility_link_commit`

**Description:** Link git commit to ticket for traceability.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "ticket_id": { "type": "string" },
    "commit_hash": { "type": "string" },
    "commit_message": { "type": "string", "description": "Optional commit message" }
  },
  "required": ["ticket_id", "commit_hash"]
}
```

**Output Example:**
```
✅ Linked commit to TASK-42

**Commit:** abc123f
**Message:** Add JWT token generation service

View commits: `jility ticket show TASK-42`
```

---

### 5. Dependencies

#### `jility_add_dependency`

**Description:** Mark that a ticket depends on another (blocks/blocked-by).

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "ticket_id": { "type": "string", "description": "Ticket that has dependency" },
    "depends_on": { "type": "string", "description": "Ticket that must be completed first" }
  },
  "required": ["ticket_id", "depends_on"]
}
```

**Output Example:**
```
✅ Added dependency

TASK-45 now depends on TASK-42

**Dependency chain:**
TASK-40 (done) → TASK-42 (review) → TASK-45 (todo)

TASK-45 is blocked until TASK-42 is complete.
```

---

#### `jility_remove_dependency`

**Description:** Remove dependency relationship.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "ticket_id": { "type": "string" },
    "depends_on": { "type": "string" }
  },
  "required": ["ticket_id", "depends_on"]
}
```

---

#### `jility_get_dependency_graph`

**Description:** Get full dependency tree for a ticket (what blocks it, what it blocks).

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "ticket_id": { "type": "string" }
  },
  "required": ["ticket_id"]
}
```

**Output Example:**
```json
{
  "ticket": "TASK-42",
  "dependencies": [
    { "ticket": "TASK-40", "title": "Create User model", "status": "done" }
  ],
  "dependents": [
    { "ticket": "TASK-45", "title": "Add login endpoint", "status": "blocked" },
    { "ticket": "TASK-46", "title": "Add refresh token logic", "status": "todo" }
  ],
  "is_blocked": false,
  "blocking_count": 2
}
```

---

### 6. Templates & Planning

#### `jility_list_templates`

**Description:** List available ticket templates.

**Output Example:**
```
📋 Available Templates

1. **api-endpoint** - REST API endpoint with validation
2. **database-migration** - Schema migration with rollback
3. **react-component** - React component with tests
4. **bug-fix** - Bug investigation and fix
5. **refactor** - Code refactoring task
```

---

#### `jility_create_from_template`

**Description:** Create ticket from template with variable substitution.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "template": { "type": "string", "description": "Template name" },
    "variables": {
      "type": "object",
      "description": "Key-value pairs for template variables"
    },
    "assignee": { "type": "string" }
  },
  "required": ["template", "variables"]
}
```

**Example:**
```json
{
  "template": "api-endpoint",
  "variables": {
    "resource": "users",
    "method": "POST",
    "endpoint": "/api/users/register"
  },
  "assignee": "agent-1"
}
```

**Output:**
```
✅ Created TASK-55 from template 'api-endpoint'

**Title:** POST /api/users/register endpoint
**Description:**
## Context
Implement POST endpoint for users resource.

## Files to Create
- `src/routes/users.rs` - Route handler
- `src/schemas/users.rs` - Request/response schemas
- `tests/api/users.rs` - Integration tests

## Acceptance Criteria
- [ ] Request validation (email, password)
- [ ] Success response (201 Created)
- [ ] Error handling (400, 409)
- [ ] Integration tests passing
```

---

### 7. Search & Discovery

#### `jility_search_tickets`

**Description:** Full-text search across title, description, and comments.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "query": { "type": "string" },
    "limit": { "type": "integer", "default": 20 }
  },
  "required": ["query"]
}
```

**Output Example:**
```
🔍 Found 3 tickets matching "JWT token"

1. **TASK-42** • Implement JWT token generation (in_progress)
   > ...add JWT-based auth to API. Token should expire after 7 days...

2. **TASK-45** • Add login endpoint (todo)
   > ...endpoint should validate credentials and return JWT token...

3. **TASK-50** • Add token refresh endpoint (backlog)
   > ...allow users to refresh their JWT token without re-authenticating...
```

---

## Error Handling

### Standard Error Response

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "❌ Error: Ticket not found\n\nTicket ID: TASK-999\n\nUse `jility ticket list` to see available tickets."
      }
    ],
    "isError": true
  }
}
```

### Error Codes

| Code | Meaning | Example |
|------|---------|---------|
| `not_found` | Resource doesn't exist | Ticket TASK-999 not found |
| `invalid_status` | Invalid status transition | Cannot move from Done to Backlog |
| `already_assigned` | Ticket already assigned | TASK-42 is already assigned to alice |
| `invalid_input` | Bad parameters | Title cannot be empty |
| `permission_denied` | Not authorized (cloud mode) | User cannot delete tickets in this project |
| `network_error` | Cloud API unreachable (cloud mode) | Failed to connect to https://api.jility.app |
| `dependency_cycle` | Would create circular dependency | TASK-42 cannot depend on TASK-45 (would create cycle) |
| `rate_limit` | Too many requests (cloud mode) | Rate limit exceeded, retry in 60s |

---

## Cloud Mode: Bridge Implementation

### Request Flow

1. Claude Code sends JSON-RPC to MCP bridge (stdio)
2. Bridge parses request, extracts tool name and arguments
3. Bridge translates to HTTP request:
   - URL: `{JILITY_API_URL}/api/v1/{endpoint}`
   - Headers: `Authorization: Bearer {JILITY_API_TOKEN}`
   - Body: JSON arguments
4. Bridge receives HTTP response
5. Bridge formats as MCP response and sends to Claude Code (stdio)

### Example Translation

**MCP Request (stdio):**
```json
{
  "method": "tools/call",
  "params": {
    "name": "jility_create_ticket",
    "arguments": {
      "title": "Add authentication",
      "story_points": 5
    }
  }
}
```

**HTTP Request (to cloud):**
```http
POST https://api.jility.app/api/v1/tickets
Authorization: Bearer jil_live_abc123...
Content-Type: application/json

{
  "title": "Add authentication",
  "story_points": 5
}
```

**HTTP Response:**
```json
{
  "id": "uuid-123",
  "number": "TASK-42",
  "title": "Add authentication",
  "story_points": 5,
  "status": "backlog",
  "created_at": "2024-10-23T10:00:00Z"
}
```

**MCP Response (stdio):**
```json
{
  "result": {
    "content": [
      {
        "type": "text",
        "text": "✅ Created ticket TASK-42\n\n**Title:** Add authentication\n**Status:** Backlog\n**Story Points:** 5"
      }
    ],
    "isError": false
  }
}
```

### Caching Strategy (Cloud Mode)

To reduce API calls and improve performance:

```rust
// Cache responses for read operations
Cache {
  get_ticket: TTL 30s,
  list_tickets: TTL 10s,
  search_tickets: TTL 60s
}

// Invalidate on writes
on_create_ticket() -> invalidate(list_tickets)
on_update_ticket(id) -> invalidate(get_ticket(id), list_tickets)
```

---

## Implementation Notes

### Stdio Communication (Both Modes)

```rust
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

fn run_mcp_server() -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let request: Value = serde_json::from_str(&line?)?;

        let response = match request["method"].as_str() {
            Some("initialize") => handle_initialize(request),
            Some("tools/list") => handle_list_tools(),
            Some("tools/call") => handle_call_tool(request),
            _ => error_response("unknown_method"),
        };

        writeln!(stdout, "{}", serde_json::to_string(&response)?)?;
        stdout.flush()?;
    }

    Ok(())
}
```

### Mode Detection

```rust
pub enum ServerMode {
    Local { db_path: PathBuf },
    Cloud { api_url: String, api_token: String },
}

impl ServerMode {
    pub fn from_env() -> Result<Self> {
        match env::var("JILITY_MODE")?.as_str() {
            "local" => Ok(Self::Local {
                db_path: PathBuf::from(".jility/data.db")
            }),
            "cloud" => Ok(Self::Cloud {
                api_url: env::var("JILITY_API_URL")?,
                api_token: env::var("JILITY_API_TOKEN")?,
            }),
            mode => Err(anyhow!("Invalid mode: {}", mode))
        }
    }
}
```

---

## Security Considerations

### Local Mode
- **File system access:** MCP server can only access `.jility/` in current directory
- **No network access:** All operations are local
- **User isolation:** Protected by OS file permissions

### Cloud Mode
- **Token security:** Tokens stored in environment variables (not in code/git)
- **Token rotation:** Users can revoke and regenerate tokens anytime
- **Rate limiting:** Prevent abuse (100 requests/minute per token)
- **HTTPS only:** All cloud API communication over TLS
- **Token expiration:** Tokens can have expiration dates (optional)
- **Audit log:** All API operations logged with token ID and timestamp

---

## Testing Strategy

### MCP Protocol Tests
```bash
# Test MCP server with mock Claude Code
cargo test --package jility-mcp

# Integration tests for both modes
JILITY_MODE=local cargo test test_local_mode
JILITY_MODE=cloud JILITY_API_URL=http://localhost:3000 cargo test test_cloud_mode

# Test stdio communication
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | jility --mcp-server
```

### Cloud Bridge Tests
```bash
# Start local API server
jility serve --port=3000

# Start MCP bridge in cloud mode
JILITY_MODE=cloud JILITY_API_URL=http://localhost:3000 JILITY_API_TOKEN=test jility --mcp-server

# Run Claude Code integration tests
npm run test:mcp
```

---

## Future Enhancements (Post-MVP)

1. **Batch Operations**
   - `jility_bulk_update` - Update multiple tickets at once
   - `jility_bulk_assign` - Assign multiple tickets

2. **Smart Context**
   - `jility_suggest_related` - AI suggests related tickets
   - `jility_estimate_effort` - AI estimates story points

3. **Sprint Management**
   - `jility_create_sprint`
   - `jility_add_to_sprint`
   - `jility_sprint_stats`

4. **Webhooks** (Cloud mode)
   - Notify external services when tickets change
   - Integrate with Slack, Discord, etc.

5. **Advanced Search**
   - Semantic search using embeddings
   - Query language (e.g., `status:in_progress assignee:agent-1 created:>2024-10-20`)

---

## Summary

**MCP Server Modes:**
- Local: Direct SQLite access, perfect for solo/small teams
- Cloud: HTTP bridge with token auth, scales to large teams

**Core Tools:** 15+ tools covering ticket CRUD, description editing, workflow, dependencies, templates

**Key Differentiator:** Precise description editing (line-based, section-based) saves tokens and enables efficient agent collaboration

**Next Steps:**
1. Implement Phase 1 with local mode only
2. Add cloud bridge in Phase 4
3. Test extensively with Claude Code
