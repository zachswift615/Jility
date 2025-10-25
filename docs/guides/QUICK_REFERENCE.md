# Jility Database Layer - Quick Reference

## File Locations

### Core Database Files

```
/home/user/Jility/crates/jility-core/src/
├── lib.rs                              # Main exports
├── entities/                           # Database models
│   ├── mod.rs                          # Entity exports
│   ├── project.rs                      # Projects table
│   ├── ticket.rs                       # Tickets table + TicketStatus enum
│   ├── ticket_assignee.rs              # Many-to-many assignees
│   ├── ticket_label.rs                 # Many-to-many labels
│   ├── ticket_dependency.rs            # Ticket dependencies
│   ├── comment.rs                      # Ticket comments
│   ├── commit_link.rs                  # Git commit links
│   ├── sprint.rs                       # Sprints + SprintStatus enum
│   ├── sprint_ticket.rs                # Many-to-many sprint tickets
│   └── ticket_change.rs                # Event sourcing + ChangeType enum
├── migration/                          # Database migrations
│   ├── mod.rs                          # Migrator setup
│   └── m20241024_000001_create_initial_schema.rs  # Initial schema
└── db/                                 # Database connection
    ├── mod.rs                          # DB module exports
    └── connection.rs                   # Connection + migrations
```

### Configuration

```
/home/user/Jility/
├── Cargo.toml                          # Workspace config
└── crates/
    ├── jility-core/Cargo.toml          # Core dependencies
    ├── jility-cli/Cargo.toml           # CLI dependencies
    ├── jility-server/Cargo.toml        # Server dependencies
    └── jility-mcp/Cargo.toml           # MCP dependencies
```

## Key Code Patterns

### Database Connection

```rust
use jility_core::{connect, run_migrations, DatabaseConfig};

// SQLite (local development)
let config = DatabaseConfig::sqlite(".jility/data.db");
let db = connect(&config).await?;
run_migrations(&db).await?;

// PostgreSQL (production)
let config = DatabaseConfig::postgres("postgresql://localhost/jility");
let db = connect(&config).await?;
run_migrations(&db).await?;
```

### Creating Entities

```rust
use jility_core::entities::*;
use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;

// Create project
let project = project::ActiveModel {
    id: Set(Uuid::new_v4()),
    name: Set("My Project".to_string()),
    description: Set(Some("Description".to_string())),
    created_at: Set(Utc::now()),
    updated_at: Set(Utc::now()),
};
let project = project.insert(&db).await?;

// Create ticket
let ticket = ticket::ActiveModel {
    id: Set(Uuid::new_v4()),
    project_id: Set(project.id),
    ticket_number: Set(1),  // Get from get_next_ticket_number()
    title: Set("Implement feature".to_string()),
    description: Set("Details here".to_string()),
    status: Set("backlog".to_string()),
    story_points: Set(Some(5)),
    created_at: Set(Utc::now()),
    updated_at: Set(Utc::now()),
    created_by: Set("alice".to_string()),
    ..Default::default()
};
let ticket = ticket.insert(&db).await?;
```

### Using Status Enums

```rust
use jility_core::entities::ticket::TicketStatus;

// Convert enum to string for DB
let status_str = TicketStatus::InProgress.as_str();  // "in_progress"

// Convert string from DB to enum
let status = TicketStatus::from_str("in_progress")?;

// Display enum
println!("{}", TicketStatus::Done);  // "done"
```

### Recording Changes (Event Sourcing)

```rust
use jility_core::entities::ticket_change;

// Record a status change
let change = ticket_change::ActiveModel {
    id: Set(Uuid::new_v4()),
    ticket_id: Set(ticket.id),
    change_type: Set("status_changed".to_string()),
    field_name: Set(Some("status".to_string())),
    old_value: Set(Some("backlog".to_string())),
    new_value: Set(Some("in_progress".to_string())),
    changed_by: Set("alice".to_string()),
    changed_at: Set(Utc::now()),
    message: Set(None),
};
change.insert(&db).await?;
```

### Querying with Relations

```rust
use sea_orm::*;
use jility_core::entities::*;

// Find all tickets in a project
let tickets = Ticket::find()
    .filter(ticket::Column::ProjectId.eq(project_id))
    .all(&db)
    .await?;

// Find ticket with assignees
let ticket_with_assignees = Ticket::find()
    .find_with_related(TicketAssignee)
    .filter(ticket::Column::Id.eq(ticket_id))
    .all(&db)
    .await?;

// Find all changes for a ticket
let changes = TicketChange::find()
    .filter(ticket_change::Column::TicketId.eq(ticket_id))
    .order_by_desc(ticket_change::Column::ChangedAt)
    .all(&db)
    .await?;
```

## Entity Relationships

### One-to-Many
- `Project` → `Ticket` (project.id → ticket.project_id)
- `Project` → `Sprint` (project.id → sprint.project_id)
- `Ticket` → `Comment` (ticket.id → comment.ticket_id)
- `Ticket` → `TicketChange` (ticket.id → ticket_change.ticket_id)
- `Ticket` → `CommitLink` (ticket.id → commit_link.ticket_id)

### Many-to-Many
- `Ticket` ↔ `TicketAssignee` (via ticket_id)
- `Ticket` ↔ `TicketLabel` (via ticket_id)
- `Sprint` ↔ `SprintTicket` (via sprint_id + ticket_id)

### Self-Referential
- `Ticket` → `Ticket` (epic_id, parent_id)
- `Ticket` ↔ `Ticket` (via TicketDependency)

## Status Enums

### TicketStatus
```rust
enum TicketStatus {
    Backlog,      // "backlog"
    Todo,         // "todo"
    InProgress,   // "in_progress"
    Review,       // "review"
    Done,         // "done"
    Blocked,      // "blocked"
}
```

### SprintStatus
```rust
enum SprintStatus {
    Planning,     // "planning"
    Active,       // "active"
    Completed,    // "completed"
}
```

### ChangeType
```rust
enum ChangeType {
    Created, TitleChanged, DescriptionChanged, StatusChanged,
    StoryPointsChanged, AssigneeAdded, AssigneeRemoved,
    LabelAdded, LabelRemoved, DependencyAdded, DependencyRemoved,
    ParentChanged, EpicChanged, CommentAdded, CommitLinked,
    AddedToSprint, RemovedFromSprint,
}
```

## Database Schema Highlights

### Unique Constraints
- `projects`: None (UUID primary key)
- `tickets`: (project_id, ticket_number) - ensures unique numbering per project
- `ticket_assignees`: (ticket_id, assignee)
- `ticket_labels`: (ticket_id, label)
- `ticket_dependencies`: (ticket_id, depends_on_id)
- `commit_links`: (ticket_id, commit_hash)
- `sprint_tickets`: (sprint_id, ticket_id)

### CHECK Constraints
- `tickets.status` IN ('backlog', 'todo', 'in_progress', 'review', 'done', 'blocked')
- `sprints.status` IN ('planning', 'active', 'completed')
- `ticket_dependencies`: ticket_id ≠ depends_on_id

### Foreign Key Actions
- **CASCADE**: ticket_assignees, ticket_labels, ticket_dependencies, comments, commit_links, sprint_tickets
- **SET NULL**: tickets.epic_id

## Migration Commands

```bash
# Run migrations (will be in jility CLI)
jility init

# Or programmatically
let db = connect(&config).await?;
run_migrations(&db).await?;
```

## Build Commands

```bash
# Build entire workspace
cargo build

# Build specific crate
cargo build -p jility-core
cargo build -p jility-cli
cargo build -p jility-server
cargo build -p jility-mcp

# Run tests
cargo test

# Check without building
cargo check
```

## Next Implementation Steps

1. **Service Layer**: Business logic on top of entities
2. **Query Helpers**: Common query patterns (get_ticket_by_number, etc.)
3. **CLI Commands**: Implement ticket CRUD via jility-cli
4. **REST API**: Implement Axum routes in jility-server
5. **MCP Server**: AI agent integration in jility-mcp
6. **Tests**: Integration tests with in-memory SQLite

## Key Files to Remember

| Purpose | File |
|---------|------|
| Add entity field | `crates/jility-core/src/entities/{entity}.rs` |
| Add migration | `crates/jility-core/src/migration/m*.rs` |
| Change DB config | `crates/jility-core/src/db/connection.rs` |
| Add workspace dep | `/home/user/Jility/Cargo.toml` |
| Add CLI command | `crates/jility-cli/src/commands/*.rs` |
| Add API route | `crates/jility-server/src/*.rs` |

## Documentation

- `/home/user/Jility/docs/database-schema-design.md` - Original schema design
- `/home/user/Jility/DATABASE_IMPLEMENTATION.md` - This implementation
- `/home/user/Jility/IMPLEMENTATION_SUMMARY.md` - Detailed summary

---

**Quick Start:**
```rust
use jility_core::{connect, run_migrations, DatabaseConfig, entities::*};

let db = connect(&DatabaseConfig::sqlite(".jility/data.db")).await?;
run_migrations(&db).await?;
// Start using entities!
```
