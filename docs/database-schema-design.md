# Jility Database Schema Design
## Event-Sourced Change Tracking with SeaORM

**Version:** 1.0
**Last Updated:** 2024-10-23
**ORM:** SeaORM (async, SQLite + PostgreSQL compatible)

---

## Overview

Jility uses an **event-sourcing lite** approach for full auditability:
- Current ticket state stored in `tickets` table
- All changes tracked in `ticket_changes` table
- Can reconstruct ticket state at any point in time
- Complete transparency for human/agent collaboration

**Benefits:**
- ✅ Full audit trail (who changed what, when)
- ✅ Time-travel debugging (see ticket at any point)
- ✅ Agent accountability (track all AI actions)
- ✅ Easy rollback (revert to previous state)
- ✅ Diffs between versions (show what changed)

---

## Technology Stack

### ORM: SeaORM

**Why SeaORM:**
- ✅ **Async-first** - Works perfectly with Axum
- ✅ **Database agnostic** - Same code for SQLite and PostgreSQL
- ✅ **Migrations built-in** - Schema versioning out of the box
- ✅ **Type-safe** - Compile-time query validation
- ✅ **Active development** - Well-maintained, growing ecosystem
- ✅ **Good DX** - Derive macros reduce boilerplate

**Dependencies (Cargo.toml):**
```toml
[dependencies]
sea-orm = { version = "0.12", features = [
    "sqlx-sqlite",
    "sqlx-postgres",
    "runtime-tokio-native-tls",
    "macros"
] }
sea-orm-migration = "0.12"
uuid = { version = "1.5", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## Core Entities

### Projects

```rust
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub name: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ticket::Entity")]
    Tickets,

    #[sea_orm(has_many = "super::sprint::Entity")]
    Sprints,
}

impl Related<super::ticket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tickets.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
```

**SQL Schema:**
```sql
CREATE TABLE projects (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_projects_name ON projects(name);
```

---

### Tickets

```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tickets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub project_id: Uuid,

    /// Auto-incrementing ticket number within project (e.g., 1, 2, 3...)
    /// Display as "TASK-1", "TASK-2", etc.
    pub ticket_number: i32,

    pub title: String,

    #[sea_orm(column_type = "Text")]
    pub description: String,

    /// Current status (stored as string, converted to enum in code)
    pub status: String, // "backlog", "todo", "in_progress", "review", "done", "blocked"

    #[sea_orm(nullable)]
    pub story_points: Option<i32>,

    /// Reference to epic (large feature)
    #[sea_orm(nullable)]
    pub epic_id: Option<Uuid>,

    /// Reference to parent ticket (for sub-tasks)
    #[sea_orm(nullable)]
    pub parent_id: Option<Uuid>,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,

    /// Who created this ticket ("agent-1", "alice", etc.)
    pub created_by: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::Id"
    )]
    Project,

    #[sea_orm(has_many = "super::ticket_assignee::Entity")]
    Assignees,

    #[sea_orm(has_many = "super::ticket_label::Entity")]
    Labels,

    #[sea_orm(has_many = "super::comment::Entity")]
    Comments,

    #[sea_orm(has_many = "super::ticket_change::Entity")]
    Changes,
}

impl ActiveModelBehavior for ActiveModel {}

/// Rust enum for status (converted to/from string in DB)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TicketStatus {
    Backlog,
    Todo,
    InProgress,
    Review,
    Done,
    Blocked,
}

impl TicketStatus {
    pub fn to_string(&self) -> String {
        match self {
            Self::Backlog => "backlog".to_string(),
            Self::Todo => "todo".to_string(),
            Self::InProgress => "in_progress".to_string(),
            Self::Review => "review".to_string(),
            Self::Done => "done".to_string(),
            Self::Blocked => "blocked".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        match s {
            "backlog" => Ok(Self::Backlog),
            "todo" => Ok(Self::Todo),
            "in_progress" => Ok(Self::InProgress),
            "review" => Ok(Self::Review),
            "done" => Ok(Self::Done),
            "blocked" => Ok(Self::Blocked),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}
```

**SQL Schema:**
```sql
CREATE TABLE tickets (
    id UUID PRIMARY KEY,
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    ticket_number INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    status TEXT NOT NULL DEFAULT 'backlog',
    story_points INTEGER,
    epic_id UUID REFERENCES tickets(id) ON DELETE SET NULL,
    parent_id UUID REFERENCES tickets(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT NOT NULL,

    UNIQUE(project_id, ticket_number),
    CHECK(status IN ('backlog', 'todo', 'in_progress', 'review', 'done', 'blocked'))
);

CREATE INDEX idx_tickets_project_id ON tickets(project_id);
CREATE INDEX idx_tickets_status ON tickets(status);
CREATE INDEX idx_tickets_created_by ON tickets(created_by);
CREATE INDEX idx_tickets_epic_id ON tickets(epic_id);
CREATE INDEX idx_tickets_parent_id ON tickets(parent_id);
CREATE INDEX idx_tickets_number ON tickets(project_id, ticket_number);
```

---

### Ticket Assignees (Many-to-Many for Pairing)

```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ticket_assignees")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub ticket_id: Uuid,

    /// Assignee identifier: "agent-1", "alice", "bob", etc.
    pub assignee: String,

    pub assigned_at: DateTimeUtc,

    /// Who assigned this person/agent
    pub assigned_by: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ticket::Entity",
        from = "Column::TicketId",
        to = "super::ticket::Column::Id"
    )]
    Ticket,
}

impl ActiveModelBehavior for ActiveModel {}
```

**SQL Schema:**
```sql
CREATE TABLE ticket_assignees (
    id UUID PRIMARY KEY,
    ticket_id UUID NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    assignee TEXT NOT NULL,
    assigned_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    assigned_by TEXT NOT NULL,

    UNIQUE(ticket_id, assignee)
);

CREATE INDEX idx_ticket_assignees_ticket_id ON ticket_assignees(ticket_id);
CREATE INDEX idx_ticket_assignees_assignee ON ticket_assignees(assignee);
```

---

### Ticket Labels (Many-to-Many)

```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ticket_labels")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub ticket_id: Uuid,

    pub label: String, // "backend", "frontend", "bug", "feature", etc.

    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ticket::Entity",
        from = "Column::TicketId",
        to = "super::ticket::Column::Id"
    )]
    Ticket,
}

impl ActiveModelBehavior for ActiveModel {}
```

**SQL Schema:**
```sql
CREATE TABLE ticket_labels (
    id UUID PRIMARY KEY,
    ticket_id UUID NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    label TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    UNIQUE(ticket_id, label)
);

CREATE INDEX idx_ticket_labels_ticket_id ON ticket_labels(ticket_id);
CREATE INDEX idx_ticket_labels_label ON ticket_labels(label);
```

---

### Ticket Dependencies

```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ticket_dependencies")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// Ticket that has the dependency
    pub ticket_id: Uuid,

    /// Ticket that must be completed first
    pub depends_on_id: Uuid,

    pub created_at: DateTimeUtc,
    pub created_by: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ticket::Entity",
        from = "Column::TicketId",
        to = "super::ticket::Column::Id"
    )]
    Ticket,
}

impl ActiveModelBehavior for ActiveModel {}
```

**SQL Schema:**
```sql
CREATE TABLE ticket_dependencies (
    id UUID PRIMARY KEY,
    ticket_id UUID NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    depends_on_id UUID NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT NOT NULL,

    UNIQUE(ticket_id, depends_on_id),
    CHECK(ticket_id != depends_on_id)
);

CREATE INDEX idx_ticket_dependencies_ticket_id ON ticket_dependencies(ticket_id);
CREATE INDEX idx_ticket_dependencies_depends_on_id ON ticket_dependencies(depends_on_id);
```

---

### Comments

```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "comments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub ticket_id: Uuid,

    /// Who wrote the comment ("agent-1", "alice", etc.)
    pub author: String,

    /// Markdown content (can include @mentions)
    #[sea_orm(column_type = "Text")]
    pub content: String,

    pub created_at: DateTimeUtc,

    #[sea_orm(nullable)]
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ticket::Entity",
        from = "Column::TicketId",
        to = "super::ticket::Column::Id"
    )]
    Ticket,
}

impl ActiveModelBehavior for ActiveModel {}
```

**SQL Schema:**
```sql
CREATE TABLE comments (
    id UUID PRIMARY KEY,
    ticket_id UUID NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    author TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE INDEX idx_comments_ticket_id ON comments(ticket_id);
CREATE INDEX idx_comments_author ON comments(author);
CREATE INDEX idx_comments_created_at ON comments(created_at DESC);
```

---

### Commit Links

```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "commit_links")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub ticket_id: Uuid,

    pub commit_hash: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub commit_message: Option<String>,

    pub linked_at: DateTimeUtc,

    pub linked_by: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ticket::Entity",
        from = "Column::TicketId",
        to = "super::ticket::Column::Id"
    )]
    Ticket,
}

impl ActiveModelBehavior for ActiveModel {}
```

**SQL Schema:**
```sql
CREATE TABLE commit_links (
    id UUID PRIMARY KEY,
    ticket_id UUID NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    commit_hash TEXT NOT NULL,
    commit_message TEXT,
    linked_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    linked_by TEXT NOT NULL,

    UNIQUE(ticket_id, commit_hash)
);

CREATE INDEX idx_commit_links_ticket_id ON commit_links(ticket_id);
CREATE INDEX idx_commit_links_commit_hash ON commit_links(commit_hash);
```

---

### Sprints

```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sprints")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub project_id: Uuid,

    pub name: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub goal: Option<String>,

    #[sea_orm(nullable)]
    pub start_date: Option<DateTimeUtc>,

    #[sea_orm(nullable)]
    pub end_date: Option<DateTimeUtc>,

    /// "planning", "active", "completed"
    pub status: String,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::Id"
    )]
    Project,

    #[sea_orm(has_many = "super::sprint_ticket::Entity")]
    SprintTickets,
}

impl ActiveModelBehavior for ActiveModel {}
```

**SQL Schema:**
```sql
CREATE TABLE sprints (
    id UUID PRIMARY KEY,
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    goal TEXT,
    start_date TIMESTAMP,
    end_date TIMESTAMP,
    status TEXT NOT NULL DEFAULT 'planning',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CHECK(status IN ('planning', 'active', 'completed'))
);

CREATE INDEX idx_sprints_project_id ON sprints(project_id);
CREATE INDEX idx_sprints_status ON sprints(status);
```

---

### Sprint Tickets (Many-to-Many)

```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sprint_tickets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub sprint_id: Uuid,
    pub ticket_id: Uuid,

    pub added_at: DateTimeUtc,
    pub added_by: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sprint::Entity",
        from = "Column::SprintId",
        to = "super::sprint::Column::Id"
    )]
    Sprint,

    #[sea_orm(
        belongs_to = "super::ticket::Entity",
        from = "Column::TicketId",
        to = "super::ticket::Column::Id"
    )]
    Ticket,
}

impl ActiveModelBehavior for ActiveModel {}
```

**SQL Schema:**
```sql
CREATE TABLE sprint_tickets (
    id UUID PRIMARY KEY,
    sprint_id UUID NOT NULL REFERENCES sprints(id) ON DELETE CASCADE,
    ticket_id UUID NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    added_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    added_by TEXT NOT NULL,

    UNIQUE(sprint_id, ticket_id)
);

CREATE INDEX idx_sprint_tickets_sprint_id ON sprint_tickets(sprint_id);
CREATE INDEX idx_sprint_tickets_ticket_id ON sprint_tickets(ticket_id);
```

---

## Event Sourcing: Ticket Changes

### The Core of Version History

```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ticket_changes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub ticket_id: Uuid,

    /// Type of change (see ChangeType enum below)
    pub change_type: String,

    /// Field that changed (for field updates)
    #[sea_orm(nullable)]
    pub field_name: Option<String>,

    /// Previous value (JSON-encoded)
    #[sea_orm(column_type = "Text", nullable)]
    pub old_value: Option<String>,

    /// New value (JSON-encoded)
    #[sea_orm(column_type = "Text", nullable)]
    pub new_value: Option<String>,

    /// Who made the change
    pub changed_by: String,

    pub changed_at: DateTimeUtc,

    /// Optional context message (e.g., handoff notes)
    #[sea_orm(column_type = "Text", nullable)]
    pub message: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ticket::Entity",
        from = "Column::TicketId",
        to = "super::ticket::Column::Id"
    )]
    Ticket,
}

impl ActiveModelBehavior for ActiveModel {}

/// Change types tracked
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangeType {
    // Lifecycle
    Created,

    // Field updates
    TitleChanged,
    DescriptionChanged,
    StatusChanged,
    StoryPointsChanged,

    // Relationships
    AssigneeAdded,
    AssigneeRemoved,
    LabelAdded,
    LabelRemoved,
    DependencyAdded,
    DependencyRemoved,
    ParentChanged,
    EpicChanged,

    // Collaboration
    CommentAdded,
    CommitLinked,

    // Sprint
    AddedToSprint,
    RemovedFromSprint,
}

impl ChangeType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Created => "created".to_string(),
            Self::TitleChanged => "title_changed".to_string(),
            Self::DescriptionChanged => "description_changed".to_string(),
            Self::StatusChanged => "status_changed".to_string(),
            Self::StoryPointsChanged => "story_points_changed".to_string(),
            Self::AssigneeAdded => "assignee_added".to_string(),
            Self::AssigneeRemoved => "assignee_removed".to_string(),
            Self::LabelAdded => "label_added".to_string(),
            Self::LabelRemoved => "label_removed".to_string(),
            Self::DependencyAdded => "dependency_added".to_string(),
            Self::DependencyRemoved => "dependency_removed".to_string(),
            Self::ParentChanged => "parent_changed".to_string(),
            Self::EpicChanged => "epic_changed".to_string(),
            Self::CommentAdded => "comment_added".to_string(),
            Self::CommitLinked => "commit_linked".to_string(),
            Self::AddedToSprint => "added_to_sprint".to_string(),
            Self::RemovedFromSprint => "removed_from_sprint".to_string(),
        }
    }
}
```

**SQL Schema:**
```sql
CREATE TABLE ticket_changes (
    id UUID PRIMARY KEY,
    ticket_id UUID NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    change_type TEXT NOT NULL,
    field_name TEXT,
    old_value TEXT,
    new_value TEXT,
    changed_by TEXT NOT NULL,
    changed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    message TEXT
);

CREATE INDEX idx_ticket_changes_ticket_id ON ticket_changes(ticket_id);
CREATE INDEX idx_ticket_changes_changed_at ON ticket_changes(changed_at DESC);
CREATE INDEX idx_ticket_changes_changed_by ON ticket_changes(changed_by);
CREATE INDEX idx_ticket_changes_change_type ON ticket_changes(change_type);
```

---

## Example: Recording Changes

### When Ticket is Created

```rust
async fn create_ticket(db: &DatabaseConnection, params: CreateTicketParams) -> Result<Ticket> {
    let ticket_id = Uuid::new_v4();
    let now = Utc::now();

    // 1. Insert ticket
    let ticket = ticket::ActiveModel {
        id: Set(ticket_id),
        project_id: Set(params.project_id),
        ticket_number: Set(get_next_ticket_number(&db, params.project_id).await?),
        title: Set(params.title.clone()),
        description: Set(params.description.unwrap_or_default()),
        status: Set("backlog".to_string()),
        story_points: Set(params.story_points),
        created_at: Set(now),
        updated_at: Set(now),
        created_by: Set(params.created_by.clone()),
        ..Default::default()
    };

    let result = ticket.insert(db).await?;

    // 2. Record creation event
    let change = ticket_change::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(ticket_id),
        change_type: Set("created".to_string()),
        field_name: NotSet,
        old_value: NotSet,
        new_value: Set(Some(serde_json::to_string(&result)?)),
        changed_by: Set(params.created_by.clone()),
        changed_at: Set(now),
        message: Set(params.message),
    };

    change.insert(db).await?;

    // 3. If assignees provided, add them and record
    if let Some(assignees) = params.assignees {
        for assignee in assignees {
            add_assignee_with_change(db, ticket_id, &assignee, &params.created_by).await?;
        }
    }

    Ok(result)
}
```

### When Status Changes

```rust
async fn update_ticket_status(
    db: &DatabaseConnection,
    ticket_id: Uuid,
    new_status: TicketStatus,
    changed_by: String,
) -> Result<()> {
    // 1. Get current ticket
    let ticket = Ticket::find_by_id(ticket_id)
        .one(db)
        .await?
        .ok_or(anyhow!("Ticket not found"))?;

    let old_status = ticket.status.clone();

    // 2. Update ticket
    let mut ticket: ticket::ActiveModel = ticket.into();
    ticket.status = Set(new_status.to_string());
    ticket.updated_at = Set(Utc::now());
    ticket.update(db).await?;

    // 3. Record change
    let change = ticket_change::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(ticket_id),
        change_type: Set("status_changed".to_string()),
        field_name: Set(Some("status".to_string())),
        old_value: Set(Some(old_status)),
        new_value: Set(Some(new_status.to_string())),
        changed_by: Set(changed_by),
        changed_at: Set(Utc::now()),
        message: NotSet,
    };

    change.insert(db).await?;

    Ok(())
}
```

### When Description is Edited

```rust
async fn update_description(
    db: &DatabaseConnection,
    ticket_id: Uuid,
    operation: EditOperation,
    changed_by: String,
) -> Result<()> {
    // 1. Get current ticket
    let ticket = Ticket::find_by_id(ticket_id)
        .one(db)
        .await?
        .ok_or(anyhow!("Ticket not found"))?;

    let old_description = ticket.description.clone();

    // 2. Apply edit operation
    let new_description = match operation {
        EditOperation::ReplaceAll { content } => content,
        EditOperation::Append { content } => format!("{}\n{}", old_description, content),
        EditOperation::ReplaceLines { start_line, end_line, content } => {
            replace_lines(&old_description, start_line, end_line, &content)?
        }
        // ... other operations
    };

    // 3. Update ticket
    let mut ticket: ticket::ActiveModel = ticket.into();
    ticket.description = Set(new_description.clone());
    ticket.updated_at = Set(Utc::now());
    ticket.update(db).await?;

    // 4. Record change with operation metadata
    let metadata = serde_json::json!({
        "operation": operation.operation_type(),
        "start_line": operation.start_line(),
        "end_line": operation.end_line(),
    });

    let change = ticket_change::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(ticket_id),
        change_type: Set("description_changed".to_string()),
        field_name: Set(Some("description".to_string())),
        old_value: Set(Some(old_description)),
        new_value: Set(Some(new_description)),
        changed_by: Set(changed_by),
        changed_at: Set(Utc::now()),
        message: Set(Some(metadata.to_string())),
    };

    change.insert(db).await?;

    Ok(())
}
```

---

## Time-Travel Queries

### Get Ticket at Specific Time

```rust
async fn get_ticket_at_time(
    db: &DatabaseConnection,
    ticket_id: Uuid,
    timestamp: DateTime<Utc>,
) -> Result<TicketSnapshot> {
    // 1. Get all changes up to timestamp
    let changes = TicketChange::find()
        .filter(ticket_change::Column::TicketId.eq(ticket_id))
        .filter(ticket_change::Column::ChangedAt.lte(timestamp))
        .order_by_asc(ticket_change::Column::ChangedAt)
        .all(db)
        .await?;

    // 2. Reconstruct state by applying changes in order
    let mut snapshot = TicketSnapshot::default();

    for change in changes {
        match change.change_type.as_str() {
            "created" => {
                snapshot = serde_json::from_str(&change.new_value.unwrap())?;
            }
            "title_changed" => {
                snapshot.title = change.new_value.unwrap();
            }
            "description_changed" => {
                snapshot.description = change.new_value.unwrap();
            }
            "status_changed" => {
                snapshot.status = change.new_value.unwrap();
            }
            "assignee_added" => {
                snapshot.assignees.push(change.new_value.unwrap());
            }
            "assignee_removed" => {
                snapshot.assignees.retain(|a| a != &change.new_value.unwrap());
            }
            // ... handle all change types
            _ => {}
        }
    }

    Ok(snapshot)
}
```

### Get Change History (Timeline View)

```rust
async fn get_ticket_timeline(
    db: &DatabaseConnection,
    ticket_id: Uuid,
    limit: Option<u64>,
) -> Result<Vec<TimelineEvent>> {
    let changes = TicketChange::find()
        .filter(ticket_change::Column::TicketId.eq(ticket_id))
        .order_by_desc(ticket_change::Column::ChangedAt)
        .limit(limit)
        .all(db)
        .await?;

    // Convert to human-readable timeline
    changes.into_iter().map(|change| {
        TimelineEvent {
            icon: get_icon_for_change(&change.change_type),
            actor: change.changed_by,
            action: format_action(&change),
            timestamp: change.changed_at,
            details: change.message,
        }
    }).collect()
}

fn format_action(change: &TicketChange) -> String {
    match change.change_type.as_str() {
        "created" => "created ticket".to_string(),
        "status_changed" => format!(
            "moved from {} to {}",
            change.old_value.as_ref().unwrap(),
            change.new_value.as_ref().unwrap()
        ),
        "assignee_added" => format!("assigned to {}", change.new_value.as_ref().unwrap()),
        "description_changed" => "updated description".to_string(),
        "comment_added" => "commented".to_string(),
        _ => change.change_type.clone(),
    }
}
```

---

## Database Initialization

### Migration System

```rust
// migration/src/lib.rs

use sea_orm_migration::prelude::*;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create projects table
        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Project::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Project::Name).string().not_null())
                    .col(ColumnDef::new(Project::Description).text())
                    .col(ColumnDef::new(Project::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Project::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Create tickets table
        manager
            .create_table(
                Table::create()
                    .table(Ticket::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Ticket::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Ticket::ProjectId).uuid().not_null())
                    .col(ColumnDef::new(Ticket::TicketNumber).integer().not_null())
                    .col(ColumnDef::new(Ticket::Title).string().not_null())
                    .col(ColumnDef::new(Ticket::Description).text().not_null())
                    .col(ColumnDef::new(Ticket::Status).string().not_null())
                    .col(ColumnDef::new(Ticket::StoryPoints).integer())
                    .col(ColumnDef::new(Ticket::EpicId).uuid())
                    .col(ColumnDef::new(Ticket::ParentId).uuid())
                    .col(ColumnDef::new(Ticket::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Ticket::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Ticket::CreatedBy).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ticket_project")
                            .from(Ticket::Table, Ticket::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // ... create all other tables

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Ticket::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Project::Table).to_owned()).await?;
        // ... drop all tables in reverse order
        Ok(())
    }
}
```

### Running Migrations

```bash
# Generate new migration
sea-orm-cli migrate generate create_initial_schema

# Run migrations (SQLite)
sea-orm-cli migrate up --database-url sqlite://.jility/data.db

# Run migrations (PostgreSQL)
sea-orm-cli migrate up --database-url postgres://user:pass@localhost/jility
```

---

## SQLite → PostgreSQL Migration

### Export/Import Tool

```rust
pub async fn export_database(
    source_db: &DatabaseConnection,
    output_path: &Path,
) -> Result<()> {
    // 1. Export all tables to JSON
    let projects = Project::find().all(source_db).await?;
    let tickets = Ticket::find().all(source_db).await?;
    let changes = TicketChange::find().all(source_db).await?;
    // ... export all tables

    // 2. Write to JSON file
    let export_data = ExportData {
        projects,
        tickets,
        ticket_assignees,
        ticket_labels,
        ticket_dependencies,
        comments,
        commit_links,
        sprints,
        sprint_tickets,
        ticket_changes: changes,
    };

    let json = serde_json::to_string_pretty(&export_data)?;
    fs::write(output_path, json)?;

    Ok(())
}

pub async fn import_database(
    target_db: &DatabaseConnection,
    input_path: &Path,
) -> Result<()> {
    let json = fs::read_to_string(input_path)?;
    let data: ExportData = serde_json::from_str(&json)?;

    // Import in order (respecting foreign keys)
    for project in data.projects {
        let model: project::ActiveModel = project.into();
        model.insert(target_db).await?;
    }

    for ticket in data.tickets {
        let model: ticket::ActiveModel = ticket.into();
        model.insert(target_db).await?;
    }

    // ... import all tables

    Ok(())
}
```

### CLI Command

```bash
# Export from SQLite
jility export --output=backup.json

# Import to PostgreSQL
jility import --input=backup.json --database=postgres://localhost/jility
```

---

## Full-Text Search

### SQLite FTS5

```sql
-- Create virtual FTS table for tickets
CREATE VIRTUAL TABLE tickets_fts USING fts5(
    ticket_id UNINDEXED,
    title,
    description,
    content=tickets,
    content_rowid=id
);

-- Populate FTS index
INSERT INTO tickets_fts(ticket_id, title, description)
SELECT id, title, description FROM tickets;

-- Triggers to keep FTS in sync
CREATE TRIGGER tickets_ai AFTER INSERT ON tickets BEGIN
  INSERT INTO tickets_fts(ticket_id, title, description)
  VALUES (new.id, new.title, new.description);
END;

CREATE TRIGGER tickets_ad AFTER DELETE ON tickets BEGIN
  DELETE FROM tickets_fts WHERE ticket_id = old.id;
END;

CREATE TRIGGER tickets_au AFTER UPDATE ON tickets BEGIN
  UPDATE tickets_fts SET title = new.title, description = new.description
  WHERE ticket_id = old.id;
END;
```

### Search Query

```rust
async fn search_tickets(
    db: &DatabaseConnection,
    query: &str,
    limit: u64,
) -> Result<Vec<Ticket>> {
    // Use raw SQL for FTS query (SeaORM doesn't support FTS5 directly)
    let sql = r#"
        SELECT t.* FROM tickets t
        INNER JOIN tickets_fts fts ON fts.ticket_id = t.id
        WHERE tickets_fts MATCH ?
        ORDER BY rank
        LIMIT ?
    "#;

    let tickets = Ticket::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            vec![query.into(), limit.into()],
        ))
        .all(db)
        .await?;

    Ok(tickets)
}
```

### PostgreSQL Full-Text Search

```sql
-- Add tsvector column
ALTER TABLE tickets ADD COLUMN search_vector tsvector;

-- Create index
CREATE INDEX idx_tickets_search ON tickets USING GIN(search_vector);

-- Update search vector
UPDATE tickets SET search_vector =
    to_tsvector('english', title || ' ' || description);

-- Trigger to keep it updated
CREATE FUNCTION tickets_search_trigger() RETURNS trigger AS $$
BEGIN
  NEW.search_vector := to_tsvector('english', NEW.title || ' ' || NEW.description);
  RETURN NEW;
END
$$ LANGUAGE plpgsql;

CREATE TRIGGER tickets_search_update BEFORE INSERT OR UPDATE
ON tickets FOR EACH ROW EXECUTE FUNCTION tickets_search_trigger();
```

---

## Performance Considerations

### Indexes Summary

```sql
-- Tickets (high read volume)
CREATE INDEX idx_tickets_project_id ON tickets(project_id);
CREATE INDEX idx_tickets_status ON tickets(status);
CREATE INDEX idx_tickets_created_by ON tickets(created_by);
CREATE INDEX idx_tickets_number ON tickets(project_id, ticket_number);

-- Ticket Changes (append-only, time-series queries)
CREATE INDEX idx_ticket_changes_ticket_id ON ticket_changes(ticket_id);
CREATE INDEX idx_ticket_changes_changed_at ON ticket_changes(changed_at DESC);
CREATE INDEX idx_ticket_changes_changed_by ON ticket_changes(changed_by);

-- Comments (time-series, filtered by ticket)
CREATE INDEX idx_comments_ticket_id ON comments(ticket_id);
CREATE INDEX idx_comments_created_at ON comments(created_at DESC);

-- Assignees (frequent lookups by person)
CREATE INDEX idx_ticket_assignees_assignee ON ticket_assignees(assignee);
```

### Query Optimization

```rust
// Bad: N+1 queries
async fn get_tickets_with_assignees_slow(db: &DatabaseConnection) -> Result<Vec<TicketWithAssignees>> {
    let tickets = Ticket::find().all(db).await?;

    let mut result = vec![];
    for ticket in tickets {
        let assignees = TicketAssignee::find()
            .filter(ticket_assignee::Column::TicketId.eq(ticket.id))
            .all(db)
            .await?;

        result.push(TicketWithAssignees { ticket, assignees });
    }

    Ok(result)
}

// Good: Single query with join
async fn get_tickets_with_assignees_fast(db: &DatabaseConnection) -> Result<Vec<TicketWithAssignees>> {
    let tickets = Ticket::find()
        .find_with_related(TicketAssignee)
        .all(db)
        .await?;

    Ok(tickets.into_iter().map(|(ticket, assignees)| {
        TicketWithAssignees { ticket, assignees }
    }).collect())
}
```

---

## Summary

**ORM:** SeaORM (async, SQLite + PostgreSQL compatible)

**Versioning:** Event-sourced change tracking in `ticket_changes` table

**Benefits:**
- Complete audit trail of all changes
- Time-travel debugging
- Agent accountability
- Easy rollback
- Full history diffs

**Migration Path:** Export JSON → Import to new DB (preserves all history)

**Search:** FTS5 (SQLite) / tsvector (PostgreSQL) for full-text search

**Ready for:** Both local-first (SQLite) and cloud deployment (PostgreSQL)
