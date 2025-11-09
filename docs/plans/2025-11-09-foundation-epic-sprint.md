# Foundation & Epic Sprint Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Make Jility solid and usable with foundation cleanup, JIRA-like epic support, and UI polish for solo developers.

**Architecture:** Four vertical slices implemented sequentially - Foundation cleanup first (MCP tools, deletion, attribution, remove unused code), then Epic backend (schema + API), then Epic frontend (board + detail views), finally UI polish (settings consolidation + Quick Add fixes). Each slice is fully tested before moving to next.

**Tech Stack:** Rust (Axum, SeaORM, SQLite), Next.js 14, TypeScript, Tailwind CSS, shadcn/ui, MCP Protocol

---

## Slice 1: Foundation Cleanup

### Task 1: Remove Unused CLI Crate

**Context:** `jility-cli` is unused (MCP uses API directly) and has compilation errors blocking builds.

**Files:**
- Modify: `Cargo.toml:2-7` (workspace members)
- Delete: `crates/jility-cli/` (entire directory)

**Step 1: Remove CLI from workspace**

Edit `Cargo.toml`:
```toml
[workspace]
members = [
    "crates/jility-core",
    "crates/jility-server",
    "crates/jility-mcp",
]
```

**Step 2: Verify build works**

Run: `cargo build --release`
Expected: Compilation succeeds without jility-cli errors

**Step 3: Delete CLI directory**

Run: `rm -rf crates/jility-cli`

**Step 4: Verify tests pass**

Run: `cargo test`
Expected: All tests pass (no CLI tests to fail)

**Step 5: Commit**

```bash
git add Cargo.toml
git add -A  # Captures deleted directory
git commit -m "chore: remove unused jility-cli crate

CLI is unused (MCP uses API directly) and had compilation errors.
MCP server and web UI are the primary interfaces.
"
```

---

### Task 2: Add MCP Tool to Read Comments (JIL-29)

**Context:** Agents need to read ticket comments for context before working. API endpoint exists (`GET /api/tickets/:id/comments`), just needs MCP exposure.

**Files:**
- Modify: `crates/jility-mcp/src/main.rs` (add new tool)
- Test: Manual testing via Claude Code MCP

**Step 1: Add get_comments tool to MCP server**

In `crates/jility-mcp/src/main.rs`, find the tools array and add new tool (around line 200-300):

```rust
ToolInfo {
    name: "get_comments".to_string(),
    description: Some("Get all comments for a ticket".to_string()),
    input_schema: json!({
        "type": "object",
        "properties": {
            "ticket_id": {
                "type": "string",
                "description": "Ticket ID or number (e.g., 'JIL-42' or UUID)"
            }
        },
        "required": ["ticket_id"]
    }),
}
```

**Step 2: Implement get_comments handler**

In the same file, find the tool call handler (around line 400-600) and add:

```rust
"get_comments" => {
    let ticket_id = params
        .get("ticket_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("ticket_id is required"))?;

    // Resolve ticket ID (supports both JIL-N and UUID)
    let resolved_id = if ticket_id.starts_with("JIL-") {
        let number: i32 = ticket_id[4..].parse()
            .map_err(|_| anyhow::anyhow!("Invalid ticket number"))?;

        // Query to get UUID from ticket number
        let url = format!("{}/tickets?number={}", base_url, number);
        let response = client.get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Ticket not found: {}", ticket_id));
        }

        let tickets: Vec<serde_json::Value> = response.json().await?;
        tickets.first()
            .and_then(|t| t.get("id"))
            .and_then(|id| id.as_str())
            .ok_or_else(|| anyhow::anyhow!("Ticket not found"))?
            .to_string()
    } else {
        ticket_id.to_string()
    };

    // Fetch comments
    let url = format!("{}/tickets/{}/comments", base_url, resolved_id);
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to fetch comments"));
    }

    let comments: Vec<serde_json::Value> = response.json().await?;

    // Format output
    let mut output = format!("💬 {} comments on ticket {}\n\n", comments.len(), ticket_id);

    for comment in comments {
        let author = comment.get("author_name")
            .and_then(|a| a.as_str())
            .unwrap_or("Unknown");
        let created_at = comment.get("created_at")
            .and_then(|t| t.as_str())
            .unwrap_or("");
        let content = comment.get("content")
            .and_then(|c| c.as_str())
            .unwrap_or("");

        output.push_str(&format!("**{}** ({})\n{}\n\n---\n\n", author, created_at, content));
    }

    Ok(vec![TextContent {
        type_: "text".to_string(),
        text: output,
    }])
}
```

**Step 3: Build and test**

Run: `cargo build --release`
Expected: Compilation succeeds

**Step 4: Manual MCP test**

1. Restart MCP server (if running)
2. In Claude Code, try: "Use the get_comments tool on ticket JIL-1"
3. Expected: Returns formatted list of comments

**Step 5: Commit**

```bash
git add crates/jility-mcp/src/main.rs
git commit -m "feat(mcp): add get_comments tool for reading ticket comments

Agents can now read comment threads before working on tickets.
Supports both ticket numbers (JIL-42) and UUIDs.
"
```

---

### Task 3: Add Delete Ticket API Endpoint (JIL-28 - Backend)

**Context:** Soft delete tickets to preserve audit trail. Add `deleted_at` column, API endpoint, and business logic.

**Files:**
- Create: `crates/jility-server/src/migrations/m20250109_000000_add_deleted_at.rs`
- Modify: `crates/jility-server/src/migrations/mod.rs`
- Modify: `crates/jility-server/src/api/tickets.rs` (add DELETE endpoint)

**Step 1: Create migration for deleted_at column**

Create file `crates/jility-server/src/migrations/m20250109_000000_add_deleted_at.rs`:

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Tickets::Table)
                    .add_column(
                        ColumnDef::new(Tickets::DeletedAt)
                            .timestamp_with_time_zone()
                            .null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Tickets::Table)
                    .drop_column(Tickets::DeletedAt)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Tickets {
    Table,
    DeletedAt,
}
```

**Step 2: Register migration**

In `crates/jility-server/src/migrations/mod.rs`, add to migrations vector:

```rust
pub use m20250109_000000_add_deleted_at::Migration as M20250109AddDeletedAt;

// In migrations() function:
vec![
    // ... existing migrations ...
    Box::new(M20250109AddDeletedAt),
]
```

**Step 3: Update Ticket entity model**

In `crates/jility-core/src/lib.rs`, add field to Ticket struct:

```rust
pub struct Ticket {
    // ... existing fields ...
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}
```

**Step 4: Add DELETE endpoint**

In `crates/jility-server/src/api/tickets.rs`, add handler:

```rust
pub async fn delete_ticket(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<AuthUser>,
    Path(ticket_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    use sea_orm::*;

    // Find ticket
    let ticket = entity::ticket::Entity::find_by_id(ticket_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;

    // Soft delete: update deleted_at timestamp
    let mut ticket: entity::ticket::ActiveModel = ticket.into();
    ticket.deleted_at = Set(Some(chrono::Utc::now()));
    ticket.update(&state.db).await?;

    Ok(StatusCode::NO_CONTENT)
}
```

**Step 5: Register route**

In `crates/jility-server/src/api/mod.rs`, add route:

```rust
.route("/tickets/:id", delete(tickets::delete_ticket))
```

**Step 6: Update list queries to exclude deleted tickets**

In `crates/jility-server/src/api/tickets.rs`, find `list_tickets` handler and add filter:

```rust
let mut query = entity::ticket::Entity::find()
    .filter(entity::ticket::Column::DeletedAt.is_null()); // Add this line
```

**Step 7: Run migration**

Run: `cargo run --bin jility-server`
Expected: Migration runs, adds `deleted_at` column

**Step 8: Test DELETE endpoint**

```bash
# Create test ticket
curl -X POST http://localhost:3900/api/tickets \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"title": "Test Delete", "status": "backlog"}'

# Delete it (use ID from response)
curl -X DELETE http://localhost:3900/api/tickets/{ID} \
  -H "Authorization: Bearer $TOKEN"

# Verify it doesn't appear in list
curl http://localhost:3900/api/tickets \
  -H "Authorization: Bearer $TOKEN"
```

Expected: 204 No Content, ticket not in list

**Step 9: Commit**

```bash
git add crates/jility-server/src/migrations/
git add crates/jility-core/src/lib.rs
git add crates/jility-server/src/api/tickets.rs
git add crates/jility-server/src/api/mod.rs
git commit -m "feat(api): add soft delete for tickets

- Add deleted_at timestamp column
- DELETE /api/tickets/:id endpoint
- Filter deleted tickets from all list queries
- Preserves audit trail and relationships
"
```

---

### Task 4: Add Delete Ticket MCP Tool (JIL-28 - MCP)

**Files:**
- Modify: `crates/jility-mcp/src/main.rs`

**Step 1: Add delete_ticket tool**

In `crates/jility-mcp/src/main.rs`, add to tools array:

```rust
ToolInfo {
    name: "delete_ticket".to_string(),
    description: Some("Delete a ticket (soft delete)".to_string()),
    input_schema: json!({
        "type": "object",
        "properties": {
            "ticket_id": {
                "type": "string",
                "description": "Ticket ID or number to delete"
            }
        },
        "required": ["ticket_id"]
    }),
}
```

**Step 2: Implement handler**

In tool call handler:

```rust
"delete_ticket" => {
    let ticket_id = params
        .get("ticket_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("ticket_id is required"))?;

    // Resolve ticket ID
    let resolved_id = resolve_ticket_id(&client, &base_url, &api_key, ticket_id).await?;

    // Delete ticket
    let url = format!("{}/tickets/{}", base_url, resolved_id);
    let response = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to delete ticket"));
    }

    Ok(vec![TextContent {
        type_: "text".to_string(),
        text: format!("✅ Deleted ticket {}", ticket_id),
    }])
}
```

**Step 3: Test via Claude Code**

Manual test: "Delete ticket JIL-2 using the MCP tool"
Expected: Ticket deleted, confirmation message

**Step 4: Commit**

```bash
git add crates/jility-mcp/src/main.rs
git commit -m "feat(mcp): add delete_ticket tool

Agents can now clean up test tickets, duplicates, and mistakes.
"
```

---

### Task 5: Add Delete Button to UI (JIL-28 - Frontend)

**Files:**
- Modify: `jility-web/app/tickets/[id]/page.tsx`

**Step 1: Add delete button to ticket detail view**

In `jility-web/app/tickets/[id]/page.tsx`, find the ticket header section and add delete button:

```tsx
import { Trash2 } from 'lucide-react';
import { Button } from '@/components/ui/button';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";

// In the component, add delete handler:
const handleDelete = async () => {
  try {
    const res = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/tickets/${ticket.id}`, {
      method: 'DELETE',
      headers: {
        'Authorization': `Bearer ${token}`,
      },
    });

    if (res.ok) {
      router.push('/backlog');
    }
  } catch (error) {
    console.error('Failed to delete ticket:', error);
  }
};

// Add button to header (after edit button):
<AlertDialog>
  <AlertDialogTrigger asChild>
    <Button variant="destructive" size="sm">
      <Trash2 className="h-4 w-4 mr-2" />
      Delete
    </Button>
  </AlertDialogTrigger>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>Delete Ticket</AlertDialogTitle>
      <AlertDialogDescription>
        Are you sure you want to delete this ticket? This action cannot be undone.
      </AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel>Cancel</AlertDialogCancel>
      <AlertDialogAction onClick={handleDelete}>Delete</AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
```

**Step 2: Test in UI**

1. Run: `npm run dev` (in jility-web/)
2. Navigate to any ticket detail page
3. Click Delete button
4. Confirm deletion
5. Expected: Redirects to backlog, ticket no longer visible

**Step 3: Commit**

```bash
git add jility-web/app/tickets/[id]/page.tsx
git commit -m "feat(ui): add delete button to ticket detail view

Users can now delete tickets with confirmation dialog.
Uses shadcn/ui AlertDialog for safety.
"
```

---

### Task 6: Fix Activity Log Attribution (JIL-30)

**Context:** Activity log shows "system" instead of actual username. Need to join users table in queries.

**Files:**
- Modify: `crates/jility-server/src/api/tickets.rs` (get_ticket endpoint)

**Step 1: Identify activity log query**

Find the `get_ticket` handler that returns ticket with activity history.

**Step 2: Update query to join users table**

Modify the activity log query to include user information:

```rust
// In get_ticket handler, update activity query:
let activities = entity::activity::Entity::find()
    .filter(entity::activity::Column::TicketId.eq(ticket.id))
    .find_also_related(entity::user::Entity) // Join users table
    .order_by_desc(entity::activity::Column::CreatedAt)
    .all(&state.db)
    .await?;

// Transform to include username
let activities: Vec<ActivityResponse> = activities
    .into_iter()
    .map(|(activity, user)| ActivityResponse {
        id: activity.id,
        ticket_id: activity.ticket_id,
        user_name: user.map(|u| u.username).unwrap_or_else(|| "system".to_string()),
        action: activity.action,
        field_name: activity.field_name,
        old_value: activity.old_value,
        new_value: activity.new_value,
        created_at: activity.created_at,
    })
    .collect();
```

**Step 3: Update ActivityResponse type**

In response types, change `user_id` to `user_name`:

```rust
#[derive(Serialize)]
pub struct ActivityResponse {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub user_name: String, // Changed from user_id
    pub action: String,
    pub field_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub created_at: DateTime<Utc>,
}
```

**Step 4: Update frontend to display username**

In `jility-web/app/tickets/[id]/page.tsx`, find activity rendering:

```tsx
<div className="text-sm text-muted-foreground">
  <span className="font-medium">{activity.user_name}</span> {activity.action}
  {activity.field_name && <> changed {activity.field_name}</>}
</div>
```

**Step 5: Test**

1. Make a change to a ticket (update status, add comment)
2. View ticket detail page
3. Expected: Activity log shows your username, not "system"

**Step 6: Commit**

```bash
git add crates/jility-server/src/api/tickets.rs
git add jility-web/app/tickets/[id]/page.tsx
git commit -m "fix: show actual username in activity log instead of 'system'

Join users table in activity queries to display proper attribution.
"
```

---

### Task 7: Remove Agents Tab from Navigation (Part 1 of JIL-30)

**Files:**
- Delete: `jility-web/app/agents/` (if exists)
- Modify: `jility-web/components/navigation.tsx` or wherever nav is defined

**Step 1: Check if agents page exists**

Run: `ls jility-web/app/agents/`

If exists, delete: `rm -rf jility-web/app/agents/`

**Step 2: Find navigation component**

Run: `find jility-web -name "*nav*" -type f`

**Step 3: Remove Agents link from navigation**

Find the navigation component and remove the Agents link:

```tsx
// Before:
<Link href="/agents">Agents</Link>

// After: (remove entire link)
```

**Step 4: Verify UI**

1. Run: `npm run dev`
2. Expected: Agents tab no longer visible in navigation

**Step 5: Commit**

```bash
git add jility-web/app/agents/ # Captures deletion
git add jility-web/components/ # Navigation changes
git commit -m "chore: remove unused Agents tab from navigation

Not hooked up and clutters interface. Can add back when needed.
"
```

---

## Slice 2: Epic Support - Backend

### Task 8: Add Epic Columns Migration

**Files:**
- Create: `crates/jility-server/src/migrations/m20250109_000001_add_epic_support.rs`
- Modify: `crates/jility-server/src/migrations/mod.rs`

**Step 1: Create migration**

Create `crates/jility-server/src/migrations/m20250109_000001_add_epic_support.rs`:

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Tickets::Table)
                    .add_column(
                        ColumnDef::new(Tickets::IsEpic)
                            .boolean()
                            .not_null()
                            .default(false)
                    )
                    .add_column(
                        ColumnDef::new(Tickets::EpicColor)
                            .string()
                            .null()
                    )
                    .add_column(
                        ColumnDef::new(Tickets::ParentEpicId)
                            .uuid()
                            .null()
                    )
                    .to_owned(),
            )
            .await?;

        // Add foreign key constraint
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_ticket_parent_epic")
                    .from(Tickets::Table, Tickets::ParentEpicId)
                    .to(Tickets::Table, Tickets::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Tickets::Table)
                    .name("fk_ticket_parent_epic")
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Tickets::Table)
                    .drop_column(Tickets::ParentEpicId)
                    .drop_column(Tickets::EpicColor)
                    .drop_column(Tickets::IsEpic)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Tickets {
    Table,
    Id,
    IsEpic,
    EpicColor,
    ParentEpicId,
}
```

**Step 2: Register migration**

In `crates/jility-server/src/migrations/mod.rs`:

```rust
pub use m20250109_000001_add_epic_support::Migration as M20250109AddEpicSupport;

// Add to migrations vec:
Box::new(M20250109AddEpicSupport),
```

**Step 3: Update Ticket entity**

In `crates/jility-core/src/lib.rs`:

```rust
pub struct Ticket {
    // ... existing fields ...
    pub is_epic: bool,
    pub epic_color: Option<String>,
    pub parent_epic_id: Option<Uuid>,
}
```

**Step 4: Run migration**

Run: `cargo run --bin jility-server`
Expected: Migration adds columns successfully

**Step 5: Commit**

```bash
git add crates/jility-server/src/migrations/
git add crates/jility-core/src/lib.rs
git commit -m "feat(db): add epic support columns to tickets table

- is_epic: marks ticket as an epic
- epic_color: visual identifier for epics
- parent_epic_id: links child tickets to parent epic
"
```

---

### Task 9: Add Epic Progress API Endpoint

**Files:**
- Create: `crates/jility-server/src/api/epics.rs`
- Modify: `crates/jility-server/src/api/mod.rs`

**Step 1: Create epics API module**

Create `crates/jility-server/src/api/epics.rs`:

```rust
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{auth::AuthUser, error::AppError, state::AppState};

#[derive(Serialize)]
pub struct EpicProgress {
    pub total: i64,
    pub todo: i64,
    pub in_progress: i64,
    pub review: i64,
    pub done: i64,
    pub blocked: i64,
    pub completion_percentage: f64,
}

#[derive(Serialize)]
pub struct EpicResponse {
    pub id: Uuid,
    pub number: i32,
    pub title: String,
    pub description: Option<String>,
    pub is_epic: bool,
    pub epic_color: Option<String>,
    pub progress: EpicProgress,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn list_epics(
    State(state): State<Arc<AppState>>,
    Extension(_user): Extension<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    use entity::ticket;

    // Get all epics
    let epics = ticket::Entity::find()
        .filter(ticket::Column::IsEpic.eq(true))
        .filter(ticket::Column::DeletedAt.is_null())
        .order_by_desc(ticket::Column::CreatedAt)
        .all(&state.db)
        .await?;

    let mut epic_responses = Vec::new();

    for epic in epics {
        let progress = calculate_epic_progress(&state.db, epic.id).await?;

        epic_responses.push(EpicResponse {
            id: epic.id,
            number: epic.number,
            title: epic.title,
            description: epic.description,
            is_epic: epic.is_epic,
            epic_color: epic.epic_color,
            progress,
            created_at: epic.created_at,
            updated_at: epic.updated_at,
        });
    }

    Ok(Json(epic_responses))
}

pub async fn get_epic(
    State(state): State<Arc<AppState>>,
    Extension(_user): Extension<AuthUser>,
    Path(epic_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    use entity::ticket;

    let epic = ticket::Entity::find_by_id(epic_id)
        .filter(ticket::Column::IsEpic.eq(true))
        .filter(ticket::Column::DeletedAt.is_null())
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Epic not found".into()))?;

    let progress = calculate_epic_progress(&state.db, epic.id).await?;

    Ok(Json(EpicResponse {
        id: epic.id,
        number: epic.number,
        title: epic.title,
        description: epic.description,
        is_epic: epic.is_epic,
        epic_color: epic.epic_color,
        progress,
        created_at: epic.created_at,
        updated_at: epic.updated_at,
    }))
}

pub async fn get_epic_tickets(
    State(state): State<Arc<AppState>>,
    Extension(_user): Extension<AuthUser>,
    Path(epic_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    use entity::ticket;

    let tickets = ticket::Entity::find()
        .filter(ticket::Column::ParentEpicId.eq(epic_id))
        .filter(ticket::Column::DeletedAt.is_null())
        .order_by_asc(ticket::Column::Number)
        .all(&state.db)
        .await?;

    Ok(Json(tickets))
}

async fn calculate_epic_progress(
    db: &DatabaseConnection,
    epic_id: Uuid,
) -> Result<EpicProgress, DbErr> {
    use entity::ticket;

    let tickets = ticket::Entity::find()
        .filter(ticket::Column::ParentEpicId.eq(epic_id))
        .filter(ticket::Column::DeletedAt.is_null())
        .all(db)
        .await?;

    let total = tickets.len() as i64;
    let mut todo = 0i64;
    let mut in_progress = 0i64;
    let mut review = 0i64;
    let mut done = 0i64;
    let mut blocked = 0i64;

    for ticket in tickets {
        match ticket.status.as_str() {
            "todo" | "backlog" => todo += 1,
            "in_progress" => in_progress += 1,
            "review" => review += 1,
            "done" => done += 1,
            "blocked" => blocked += 1,
            _ => todo += 1,
        }
    }

    let completion_percentage = if total > 0 {
        (done as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    Ok(EpicProgress {
        total,
        todo,
        in_progress,
        review,
        done,
        blocked,
        completion_percentage,
    })
}
```

**Step 2: Register epic routes**

In `crates/jility-server/src/api/mod.rs`:

```rust
mod epics;

// In routes function:
.route("/epics", get(epics::list_epics))
.route("/epics/:id", get(epics::get_epic))
.route("/epics/:id/tickets", get(epics::get_epic_tickets))
```

**Step 3: Update create_ticket to accept epic fields**

In `crates/jility-server/src/api/tickets.rs`:

```rust
#[derive(Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub story_points: Option<i32>,
    pub labels: Option<Vec<String>>,
    pub assignees: Option<Vec<String>>,
    pub is_epic: Option<bool>, // Add this
    pub epic_color: Option<String>, // Add this
    pub parent_epic_id: Option<Uuid>, // Add this
}

// In create_ticket handler, add to ticket creation:
ticket.is_epic = Set(req.is_epic.unwrap_or(false));
ticket.epic_color = Set(req.epic_color);
ticket.parent_epic_id = Set(req.parent_epic_id);
```

**Step 4: Add validation for epic hierarchy**

In `create_ticket` handler, before saving:

```rust
// Validate: can't nest epics
if let Some(parent_id) = req.parent_epic_id {
    let parent = ticket::Entity::find_by_id(parent_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::BadRequest("Parent epic not found".into()))?;

    if !parent.is_epic {
        return Err(AppError::BadRequest("Parent must be an epic".into()));
    }

    if req.is_epic.unwrap_or(false) {
        return Err(AppError::BadRequest("Epics cannot be nested".into()));
    }
}
```

**Step 5: Test epic creation**

```bash
# Create epic
curl -X POST http://localhost:3900/api/tickets \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "User Authentication Epic",
    "description": "Implement full authentication system",
    "is_epic": true,
    "epic_color": "#3b82f6"
  }'

# Create child ticket
curl -X POST http://localhost:3900/api/tickets \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Setup JWT middleware",
    "parent_epic_id": "EPIC_UUID_FROM_ABOVE"
  }'

# List epics
curl http://localhost:3900/api/epics \
  -H "Authorization: Bearer $TOKEN"
```

Expected: Epic created, child ticket linked, progress calculated

**Step 6: Commit**

```bash
git add crates/jility-server/src/api/epics.rs
git add crates/jility-server/src/api/mod.rs
git add crates/jility-server/src/api/tickets.rs
git commit -m "feat(api): add epic support with progress tracking

- GET /api/epics - list all epics with progress
- GET /api/epics/:id - get epic with progress
- GET /api/epics/:id/tickets - get epic's child tickets
- Progress calculation: status breakdown and completion %
- Validation: prevent epic nesting
"
```

---

### Task 10: Add Epic MCP Tools

**Files:**
- Modify: `crates/jility-mcp/src/main.rs`

**Step 1: Add create_epic tool**

```rust
ToolInfo {
    name: "create_epic".to_string(),
    description: Some("Create an epic to group related tickets".to_string()),
    input_schema: json!({
        "type": "object",
        "properties": {
            "title": {"type": "string"},
            "description": {"type": "string"},
            "color": {
                "type": "string",
                "description": "Hex color (e.g., #3b82f6)",
                "default": "#3b82f6"
            }
        },
        "required": ["title"]
    }),
}
```

**Step 2: Add list_epics tool**

```rust
ToolInfo {
    name: "list_epics".to_string(),
    description: Some("List all epics with progress".to_string()),
    input_schema: json!({
        "type": "object",
        "properties": {}
    }),
}
```

**Step 3: Implement create_epic handler**

```rust
"create_epic" => {
    let title = params.get("title").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("title is required"))?;
    let description = params.get("description").and_then(|v| v.as_str());
    let color = params.get("color").and_then(|v| v.as_str()).unwrap_or("#3b82f6");

    let body = json!({
        "title": title,
        "description": description,
        "is_epic": true,
        "epic_color": color,
        "status": "backlog"
    });

    let response = client.post(&format!("{}/tickets", base_url))
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to create epic"));
    }

    let epic: serde_json::Value = response.json().await?;
    let epic_number = epic.get("number").and_then(|n| n.as_i64()).unwrap_or(0);

    Ok(vec![TextContent {
        type_: "text".to_string(),
        text: format!("✅ Created epic JIL-{}: {}", epic_number, title),
    }])
}
```

**Step 4: Implement list_epics handler**

```rust
"list_epics" => {
    let response = client.get(&format!("{}/epics", base_url))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to fetch epics"));
    }

    let epics: Vec<serde_json::Value> = response.json().await?;

    let mut output = format!("📚 {} epics\n\n", epics.len());

    for epic in epics {
        let number = epic.get("number").and_then(|n| n.as_i64()).unwrap_or(0);
        let title = epic.get("title").and_then(|t| t.as_str()).unwrap_or("Untitled");
        let progress = epic.get("progress").unwrap();
        let total = progress.get("total").and_then(|t| t.as_i64()).unwrap_or(0);
        let done = progress.get("done").and_then(|d| d.as_i64()).unwrap_or(0);
        let pct = progress.get("completion_percentage").and_then(|p| p.as_f64()).unwrap_or(0.0);

        output.push_str(&format!("- JIL-{}: {} ({}/{} tasks, {:.0}% complete)\n",
            number, title, done, total, pct));
    }

    Ok(vec![TextContent {
        type_: "text".to_string(),
        text: output,
    }])
}
```

**Step 5: Update create_ticket to accept parent_epic_id**

In existing `create_ticket` handler, add:

```rust
let parent_epic_id = params.get("parent_epic_id")
    .and_then(|v| v.as_str())
    .map(|id| id.to_string());

// Add to request body:
if let Some(epic_id) = parent_epic_id {
    body["parent_epic_id"] = json!(epic_id);
}
```

**Step 6: Test via Claude Code**

1. "Create an epic called 'User Auth' with description 'Authentication system'"
2. "List all epics"
3. "Create a ticket 'Add JWT middleware' in epic JIL-X"

Expected: Epic created, listed with progress, child ticket linked

**Step 7: Commit**

```bash
git add crates/jility-mcp/src/main.rs
git commit -m "feat(mcp): add epic management tools

- create_epic: Create epics with title, description, color
- list_epics: View all epics with progress stats
- create_ticket: Accept parent_epic_id to link tickets to epics
"
```

---

## Slice 3: Epic Support - Frontend

### Task 11: Create Epic Board View

**Files:**
- Create: `jility-web/app/epics/page.tsx`
- Create: `jility-web/components/epic-card.tsx`

**Step 1: Create epic card component**

Create `jility-web/components/epic-card.tsx`:

```tsx
import Link from 'next/link';
import { Layers } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from './ui/card';
import { Progress } from './ui/progress';

interface EpicProgress {
  total: number;
  done: number;
  in_progress: number;
  todo: number;
  blocked: number;
  completion_percentage: number;
}

interface Epic {
  id: string;
  number: number;
  title: string;
  description?: string;
  epic_color?: string;
  progress: EpicProgress;
}

export function EpicCard({ epic }: { epic: Epic }) {
  const color = epic.epic_color || '#3b82f6';

  return (
    <Link href={`/epics/${epic.id}`}>
      <Card className="hover:shadow-lg transition-shadow cursor-pointer">
        <div className="absolute left-0 top-0 bottom-0 w-1" style={{ backgroundColor: color }} />
        <CardHeader className="pl-6">
          <div className="flex items-center gap-2">
            <Layers className="h-5 w-5 text-muted-foreground" />
            <CardTitle className="text-lg">
              JIL-{epic.number}: {epic.title}
            </CardTitle>
          </div>
        </CardHeader>
        <CardContent className="pl-6">
          {epic.description && (
            <p className="text-sm text-muted-foreground mb-4 line-clamp-2">
              {epic.description}
            </p>
          )}

          <div className="space-y-2">
            <div className="flex justify-between text-sm">
              <span className="text-muted-foreground">
                {epic.progress.done} of {epic.progress.total} tasks completed
              </span>
              <span className="font-medium">
                {Math.round(epic.progress.completion_percentage)}%
              </span>
            </div>
            <Progress value={epic.progress.completion_percentage} />

            <div className="flex gap-4 text-xs text-muted-foreground mt-2">
              <span>{epic.progress.todo} todo</span>
              <span>{epic.progress.in_progress} in progress</span>
              <span>{epic.progress.done} done</span>
              {epic.progress.blocked > 0 && (
                <span className="text-destructive">{epic.progress.blocked} blocked</span>
              )}
            </div>
          </div>
        </CardContent>
      </Card>
    </Link>
  );
}
```

**Step 2: Create epics list page**

Create `jility-web/app/epics/page.tsx`:

```tsx
'use client';

import { useEffect, useState } from 'react';
import { Plus } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { EpicCard } from '@/components/epic-card';

interface Epic {
  id: string;
  number: number;
  title: string;
  description?: string;
  epic_color?: string;
  progress: {
    total: number;
    done: number;
    in_progress: number;
    todo: number;
    blocked: number;
    completion_percentage: number;
  };
}

export default function EpicsPage() {
  const [epics, setEpics] = useState<Epic[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchEpics();
  }, []);

  const fetchEpics = async () => {
    try {
      const token = localStorage.getItem('token');
      const res = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/epics`, {
        headers: {
          'Authorization': `Bearer ${token}`,
        },
      });

      if (res.ok) {
        const data = await res.json();
        setEpics(data);
      }
    } catch (error) {
      console.error('Failed to fetch epics:', error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return <div className="p-6">Loading epics...</div>;
  }

  return (
    <div className="p-3 md:p-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">Epics</h1>
        <Button>
          <Plus className="h-4 w-4 mr-2" />
          New Epic
        </Button>
      </div>

      {epics.length === 0 ? (
        <div className="text-center py-12 text-muted-foreground">
          <p className="mb-4">No epics yet.</p>
          <p>Create your first epic to organize work.</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {epics.map((epic) => (
            <EpicCard key={epic.id} epic={epic} />
          ))}
        </div>
      )}
    </div>
  );
}
```

**Step 3: Add Progress component if missing**

If `jility-web/components/ui/progress.tsx` doesn't exist:

```bash
npx shadcn-ui@latest add progress
```

**Step 4: Add Epics to navigation**

In navigation component (find via `grep -r "Board" jility-web/components/`), add:

```tsx
import { Layers } from 'lucide-react';

// In nav items:
<Link href="/epics">
  <Layers className="h-5 w-5" />
  <span>Epics</span>
</Link>
```

**Step 5: Test in browser**

1. Run: `npm run dev`
2. Navigate to `/epics`
3. Expected: Grid of epic cards showing progress

**Step 6: Commit**

```bash
git add jility-web/app/epics/
git add jility-web/components/epic-card.tsx
git add jility-web/components/ui/progress.tsx
git add jility-web/components/ # Navigation
git commit -m "feat(ui): add epic board view with progress cards

Card layout shows epic title, description, progress bar, and status breakdown.
"
```

---

### Task 12: Create Epic Detail View

**Files:**
- Create: `jility-web/app/epics/[id]/page.tsx`

**Step 1: Create epic detail page**

Create `jility-web/app/epics/[id]/page.tsx`:

```tsx
'use client';

import { useEffect, useState } from 'react';
import { useParams } from 'next/navigation';
import { Layers, Plus } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import { Badge } from '@/components/ui/badge';

interface Epic {
  id: string;
  number: number;
  title: string;
  description?: string;
  epic_color?: string;
  progress: {
    total: number;
    done: number;
    in_progress: number;
    todo: number;
    blocked: number;
    completion_percentage: number;
  };
}

interface Ticket {
  id: string;
  number: number;
  title: string;
  status: string;
  story_points?: number;
}

export default function EpicDetailPage() {
  const params = useParams();
  const [epic, setEpic] = useState<Epic | null>(null);
  const [tickets, setTickets] = useState<Ticket[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchEpic();
    fetchTickets();
  }, [params.id]);

  const fetchEpic = async () => {
    try {
      const token = localStorage.getItem('token');
      const res = await fetch(
        `${process.env.NEXT_PUBLIC_API_URL}/epics/${params.id}`,
        {
          headers: { 'Authorization': `Bearer ${token}` },
        }
      );

      if (res.ok) {
        const data = await res.json();
        setEpic(data);
      }
    } catch (error) {
      console.error('Failed to fetch epic:', error);
    }
  };

  const fetchTickets = async () => {
    try {
      const token = localStorage.getItem('token');
      const res = await fetch(
        `${process.env.NEXT_PUBLIC_API_URL}/epics/${params.id}/tickets`,
        {
          headers: { 'Authorization': `Bearer ${token}` },
        }
      );

      if (res.ok) {
        const data = await res.json();
        setTickets(data);
      }
    } catch (error) {
      console.error('Failed to fetch tickets:', error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return <div className="p-6">Loading...</div>;
  }

  if (!epic) {
    return <div className="p-6">Epic not found</div>;
  }

  const statusColumns = {
    backlog: tickets.filter(t => t.status === 'backlog'),
    todo: tickets.filter(t => t.status === 'todo'),
    in_progress: tickets.filter(t => t.status === 'in_progress'),
    review: tickets.filter(t => t.status === 'review'),
    done: tickets.filter(t => t.status === 'done'),
  };

  return (
    <div className="p-3 md:p-6">
      {/* Epic Header */}
      <div className="mb-6">
        <div className="flex items-center gap-3 mb-2">
          <div
            className="h-8 w-8 rounded flex items-center justify-center"
            style={{ backgroundColor: epic.epic_color || '#3b82f6' }}
          >
            <Layers className="h-5 w-5 text-white" />
          </div>
          <h1 className="text-2xl font-bold">
            JIL-{epic.number}: {epic.title}
          </h1>
        </div>

        {epic.description && (
          <p className="text-muted-foreground mb-4">{epic.description}</p>
        )}

        {/* Progress */}
        <div className="bg-card border border-border rounded-lg p-4 mb-6">
          <div className="flex justify-between items-center mb-2">
            <span className="text-sm font-medium">Progress</span>
            <span className="text-2xl font-bold">
              {Math.round(epic.progress.completion_percentage)}%
            </span>
          </div>
          <Progress value={epic.progress.completion_percentage} className="mb-2" />
          <div className="flex gap-4 text-sm text-muted-foreground">
            <span>{epic.progress.done} / {epic.progress.total} tasks completed</span>
            <span>{epic.progress.in_progress} in progress</span>
            <span>{epic.progress.todo} remaining</span>
          </div>
        </div>
      </div>

      {/* Kanban Board */}
      <div className="mb-4">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-lg font-semibold">Tasks</h2>
          <Button size="sm">
            <Plus className="h-4 w-4 mr-2" />
            Add Task
          </Button>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-5 gap-4">
          {Object.entries(statusColumns).map(([status, tickets]) => (
            <div key={status} className="bg-muted rounded-lg p-3">
              <h3 className="font-medium mb-2 capitalize">
                {status.replace('_', ' ')} ({tickets.length})
              </h3>
              <div className="space-y-2">
                {tickets.map((ticket) => (
                  <div
                    key={ticket.id}
                    className="bg-card border border-border rounded p-3 hover:shadow cursor-pointer"
                  >
                    <div className="text-sm font-medium mb-1">
                      JIL-{ticket.number}
                    </div>
                    <div className="text-sm">{ticket.title}</div>
                    {ticket.story_points && (
                      <Badge variant="secondary" className="mt-2">
                        {ticket.story_points} pts
                      </Badge>
                    )}
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
```

**Step 2: Add Badge component if missing**

```bash
npx shadcn-ui@latest add badge
```

**Step 3: Test epic detail view**

1. Navigate to `/epics/[epic-id]`
2. Expected: Epic header, progress, kanban board filtered to epic's tickets

**Step 4: Commit**

```bash
git add jility-web/app/epics/[id]/
git add jility-web/components/ui/badge.tsx
git commit -m "feat(ui): add epic detail view with kanban board

Shows epic header, progress visualization, and filtered kanban board.
"
```

---

### Task 13: Add Epic Filter to Main Board

**Files:**
- Modify: `jility-web/app/board/page.tsx` (or wherever main board is)

**Step 1: Add epic filter dropdown**

In board page, add state and fetch epics:

```tsx
const [epics, setEpics] = useState<Epic[]>([]);
const [selectedEpic, setSelectedEpic] = useState<string | null>(null);

useEffect(() => {
  fetchEpics();
}, []);

const fetchEpics = async () => {
  const token = localStorage.getItem('token');
  const res = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/epics`, {
    headers: { 'Authorization': `Bearer ${token}` },
  });
  if (res.ok) {
    setEpics(await res.json());
  }
};
```

**Step 2: Add filter UI**

```tsx
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

// In board header:
<Select value={selectedEpic || 'all'} onValueChange={(v) => setSelectedEpic(v === 'all' ? null : v)}>
  <SelectTrigger className="w-[200px]">
    <SelectValue placeholder="Filter by epic" />
  </SelectTrigger>
  <SelectContent>
    <SelectItem value="all">All tickets</SelectItem>
    <SelectItem value="no-epic">No epic</SelectItem>
    {epics.map((epic) => (
      <SelectItem key={epic.id} value={epic.id}>
        JIL-{epic.number}: {epic.title}
      </SelectItem>
    ))}
  </SelectContent>
</Select>
```

**Step 3: Filter tickets by epic**

```tsx
const filteredTickets = useMemo(() => {
  if (!selectedEpic) return tickets;

  if (selectedEpic === 'no-epic') {
    return tickets.filter(t => !t.parent_epic_id);
  }

  return tickets.filter(t => t.parent_epic_id === selectedEpic);
}, [tickets, selectedEpic]);
```

**Step 4: Add epic badge to ticket cards**

In ticket card component:

```tsx
{ticket.parent_epic_id && ticket.epic_info && (
  <Badge
    variant="outline"
    className="text-xs"
    style={{ borderColor: ticket.epic_info.color }}
  >
    {ticket.epic_info.title}
  </Badge>
)}
```

**Step 5: Update API to include epic info in tickets**

In `crates/jility-server/src/api/tickets.rs`, join epic when listing tickets:

```rust
let tickets = ticket::Entity::find()
    .find_also_related(ticket::Entity) // Self-join for parent epic
    .filter(ticket::Column::DeletedAt.is_null())
    .all(&state.db)
    .await?;
```

**Step 6: Test filtering**

1. Create epic with tickets
2. Go to board
3. Select epic in filter dropdown
4. Expected: Only tickets belonging to that epic visible

**Step 7: Commit**

```bash
git add jility-web/app/board/page.tsx
git add jility-web/components/ui/select.tsx
git commit -m "feat(ui): add epic filter to board view

Filter tickets by epic or view unassigned tickets.
Epic badges on ticket cards show which epic they belong to.
"
```

---

## Slice 4: UI/UX Polish

### Task 14: Combine Settings Pages into Tabs

**Files:**
- Create: `jility-web/app/settings/page.tsx`
- Delete: Old settings pages (identify first)
- Modify: Navigation

**Step 1: Find existing settings pages**

Run: `find jility-web/app -name "*setting*" -o -name "*profile*" -o -name "*workspace*"`

(Note the paths for deletion later)

**Step 2: Create unified settings page**

Create `jility-web/app/settings/page.tsx`:

```tsx
'use client';

import { useState, useEffect } from 'react';
import { useSearchParams, useRouter } from 'next/navigation';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

export default function SettingsPage() {
  const searchParams = useSearchParams();
  const router = useRouter();
  const [activeTab, setActiveTab] = useState(searchParams.get('tab') || 'profile');

  const handleTabChange = (tab: string) => {
    setActiveTab(tab);
    router.push(`/settings?tab=${tab}`);
  };

  return (
    <div className="p-3 md:p-6">
      <h1 className="text-2xl font-bold mb-6">Settings</h1>

      <Tabs value={activeTab} onValueChange={handleTabChange}>
        <TabsList>
          <TabsTrigger value="profile">Profile</TabsTrigger>
          <TabsTrigger value="workspace">Workspace</TabsTrigger>
          <TabsTrigger value="projects">Projects</TabsTrigger>
          <TabsTrigger value="api-keys">API Keys</TabsTrigger>
        </TabsList>

        <TabsContent value="profile" className="mt-6">
          {/* Move content from old profile settings page here */}
          <ProfileSettings />
        </TabsContent>

        <TabsContent value="workspace" className="mt-6">
          {/* Move content from old workspace settings page here */}
          <WorkspaceSettings />
        </TabsContent>

        <TabsContent value="projects" className="mt-6">
          {/* Move content from /project page here */}
          <ProjectsSettings />
        </TabsContent>

        <TabsContent value="api-keys" className="mt-6">
          {/* Move content from API keys page here */}
          <APIKeysSettings />
        </TabsContent>
      </Tabs>
    </div>
  );
}

// Components for each tab (extract from existing pages)
function ProfileSettings() {
  // TODO: Copy from old profile settings page
  return <div>Profile settings content</div>;
}

function WorkspaceSettings() {
  // TODO: Copy from old workspace settings page
  return <div>Workspace settings content</div>;
}

function ProjectsSettings() {
  // TODO: Copy from /project page
  return <div>Projects content</div>;
}

function APIKeysSettings() {
  // TODO: Copy from API keys page
  return <div>API keys content</div>;
}
```

**Step 3: Copy content from old pages into tab components**

For each tab component above, find the corresponding old page and copy its JSX content.

**Step 4: Add redirect from /project**

Create `jility-web/app/project/page.tsx`:

```tsx
'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function ProjectRedirect() {
  const router = useRouter();

  useEffect(() => {
    router.replace('/settings?tab=projects');
  }, []);

  return <div>Redirecting...</div>;
}
```

**Step 5: Update navigation**

Remove separate "Projects" link, ensure "Settings" link exists:

```tsx
<Link href="/settings">
  <Settings className="h-5 w-5" />
  <span>Settings</span>
</Link>
```

**Step 6: Delete old settings pages**

```bash
# Based on what you found in Step 1
rm -rf jility-web/app/old-settings-page/
rm -rf jility-web/app/old-profile-page/
```

**Step 7: Test**

1. Navigate to `/settings`
2. Click through all tabs
3. Navigate to `/project` - should redirect to `/settings?tab=projects`
4. Browser back/forward buttons should work with tabs

**Step 8: Commit**

```bash
git add jility-web/app/settings/
git add jility-web/app/project/
git add jility-web/components/
git add -A  # Captures deleted pages
git commit -m "feat(ui): consolidate settings into tabbed interface

- Single /settings page with Profile, Workspace, Projects, API Keys tabs
- /project redirects to /settings?tab=projects
- Tab state persists in URL query params
"
```

---

### Task 15: Fix Quick Add in Backlog View

**Files:**
- Modify: `jility-web/app/backlog/page.tsx` (or wherever backlog view is)

**Step 1: Find Quick Add form in backlog**

Search for the inline ticket creation form in backlog view.

**Step 2: Debug the issue**

Add console logs to identify the problem:

```tsx
const handleQuickAdd = async (title: string) => {
  console.log('Quick Add triggered:', title);

  try {
    const token = localStorage.getItem('token');
    const res = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/tickets`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        title,
        status: 'backlog',
      }),
    });

    console.log('Response status:', res.status);

    if (res.ok) {
      const newTicket = await res.json();
      console.log('Created ticket:', newTicket);

      // Optimistic UI update
      setTickets([newTicket, ...tickets]);

      // Clear form
      setQuickAddValue('');
    }
  } catch (error) {
    console.error('Quick Add error:', error);
  }
};
```

**Step 3: Fix form submission**

Ensure form has proper submit handler:

```tsx
<form onSubmit={(e) => {
  e.preventDefault();
  handleQuickAdd(quickAddValue);
}}>
  <Input
    value={quickAddValue}
    onChange={(e) => setQuickAddValue(e.target.value)}
    placeholder="Add a ticket..."
    className="mb-2"
  />
  <Button type="submit">Add</Button>
</form>
```

**Step 4: Test**

1. Type title in Quick Add form
2. Press Enter or click Add
3. Expected: Ticket appears at top of backlog
4. Form clears

**Step 5: Remove debug logs and commit**

```bash
git add jility-web/app/backlog/page.tsx
git commit -m "fix: Quick Add ticket form in backlog view

- Wire up form submission handler
- Add optimistic UI update
- Clear form after successful creation
"
```

---

### Task 16: Fix Quick Add Button at Top of Backlog

**Files:**
- Modify: `jility-web/app/backlog/page.tsx`

**Step 1: Find Quick Add button**

Locate the button in the backlog page header (likely near title).

**Step 2: Add state for inline form visibility**

```tsx
const [showQuickAdd, setShowQuickAdd] = useState(false);
const [quickAddTitle, setQuickAddTitle] = useState('');
const quickAddInputRef = useRef<HTMLInputElement>(null);
```

**Step 3: Wire up button click**

```tsx
<Button onClick={() => {
  setShowQuickAdd(true);
  setTimeout(() => quickAddInputRef.current?.focus(), 0);
}}>
  <Plus className="h-4 w-4 mr-2" />
  Quick Add
</Button>
```

**Step 4: Add inline form**

Below the button (or wherever appropriate):

```tsx
{showQuickAdd && (
  <div className="mb-4 bg-card border border-border rounded-lg p-3">
    <form onSubmit={(e) => {
      e.preventDefault();
      handleQuickAdd(quickAddTitle);
      setShowQuickAdd(false);
    }}>
      <Input
        ref={quickAddInputRef}
        value={quickAddTitle}
        onChange={(e) => setQuickAddTitle(e.target.value)}
        placeholder="Ticket title..."
        className="mb-2"
      />
      <div className="flex gap-2">
        <Button type="submit">Create</Button>
        <Button
          type="button"
          variant="outline"
          onClick={() => setShowQuickAdd(false)}
        >
          Cancel
        </Button>
      </div>
    </form>
  </div>
)}
```

**Step 5: Test**

1. Click "Quick Add" button
2. Form appears, input auto-focuses
3. Type title, press Enter
4. Ticket created, form closes
5. Click Cancel - form closes without creating

**Step 6: Commit**

```bash
git add jility-web/app/backlog/page.tsx
git commit -m "fix: Quick Add button at top of backlog page

Opens inline form with auto-focus.
Create or cancel to close form.
"
```

---

## Final Tasks

### Task 17: Integration Testing

**Step 1: Test complete epic workflow**

1. Create epic via MCP: `create_epic("Test Epic", "Testing epic workflow")`
2. Create 3 child tickets via MCP
3. Move tickets through statuses in UI
4. View epic progress on `/epics` page
5. View epic detail page
6. Filter main board by epic

**Step 2: Test foundation features**

1. Read comments via MCP: `get_comments("JIL-1")`
2. Delete a ticket via UI
3. Verify activity log shows correct username
4. Verify Agents tab is gone from navigation

**Step 3: Test UI polish**

1. Navigate to `/settings` - all tabs work
2. Navigate to `/project` - redirects to settings
3. Quick Add works in backlog
4. Quick Add button works at top of backlog

**Step 4: Document any issues**

Create tickets for any bugs found during testing.

---

### Task 18: Update Documentation

**Files:**
- Modify: `README.md`
- Modify: `docs/plans/2025-11-09-foundation-and-epic-sprint-design.md`

**Step 1: Update README with epic features**

Add to features section:

```markdown
### Epic Support
- Organize tickets into epics (JIRA-like)
- Visual epic board with progress tracking
- Filter board by epic
- Create epics via MCP or UI
```

**Step 2: Update design doc with completion notes**

Add "Implementation Complete" section to design doc with:
- What was completed
- Any deviations from plan
- Known issues or follow-ups

**Step 3: Commit**

```bash
git add README.md docs/
git commit -m "docs: update README and design doc for epic sprint completion"
```

---

## Execution Strategy

**Estimated Total Time:** 10-14 days (varies with testing and polish)

**Slice Completion Order:**
1. Slice 1 (2-3 days) - Foundation must be solid first
2. Slice 2 (3-4 days) - Backend enables frontend
3. Slice 3 (3-4 days) - UI brings features to users
4. Slice 4 (2-3 days) - Polish creates great UX

**Testing Checkpoints:**
- After each task: Unit test or manual verification
- After each slice: Full slice integration test
- After sprint: Complete end-to-end test

**Commit Frequency:**
- Commit after every task (frequent commits)
- Use conventional commit messages (`feat:`, `fix:`, `chore:`, `docs:`)

---

**End of Implementation Plan**
