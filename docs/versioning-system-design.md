# Jility Versioning System Design
## Complete Change History & Time-Travel for Tickets

**Version:** 1.0
**Last Updated:** 2024-10-23
**Status:** Design Specification

---

## Overview

Jility tracks **every change** to every ticket, creating a complete audit trail. Unlike traditional issue trackers that only version descriptions, Jility versions **everything**:

- Title changes
- Description edits
- Status transitions
- Assignee changes
- Label additions/removals
- Dependencies
- Story point updates
- Comments (as activity)
- Commit links

This enables:
- ✅ **Full transparency** - See exactly what agents and humans did
- ✅ **Time-travel** - View ticket state at any point in history
- ✅ **Rollback** - Revert to previous states
- ✅ **Diffs** - Compare changes between versions
- ✅ **Attribution** - Know who changed what, when, and why
- ✅ **Debugging** - Trace how a ticket evolved

---

## Core Concepts

### 1. Everything is an Event

Every modification to a ticket creates a change event stored in the `ticket_changes` table:

```rust
struct TicketChange {
    id: Uuid,
    ticket_id: Uuid,
    change_type: String,      // "title_changed", "status_changed", etc.
    field_name: Option<String>, // Which field changed
    old_value: Option<String>, // Previous value (JSON)
    new_value: Option<String>, // New value (JSON)
    changed_by: String,        // Who made the change
    changed_at: DateTime,      // When it happened
    message: Option<String>,   // Optional context (e.g., handoff notes)
}
```

### 2. Timeline is the Source of Truth

The timeline of changes **is** the history. You can reconstruct any ticket state by replaying events from creation to any point in time.

### 3. Human-Readable Display

While events are stored as structured data, they're displayed as a human-readable activity timeline:

```
📝 agent-1 created ticket                           3 hours ago

⚡ alice moved from Backlog to In Progress           2 hours ago

👥 alice assigned to agent-1                         2 hours ago
   Message: "I've set up the structure, please implement the JWT logic"

📝 agent-1 updated description                       30 mins ago
   Changed lines 8-10: Updated checklist progress

🔗 agent-1 linked commit abc123f                     15 mins ago
   "Add JWT generation service with tests"

💬 alice commented                                   5 mins ago
   "Make sure to handle token expiration properly"
```

---

## Change Types

### Lifecycle Events

**`created`**
- Triggered: Ticket is created
- Captures: Full initial state
- Example:
  ```json
  {
    "change_type": "created",
    "new_value": "{\"title\":\"Add JWT auth\",\"status\":\"backlog\",\"story_points\":5}",
    "changed_by": "agent-1",
    "changed_at": "2024-10-23T10:00:00Z"
  }
  ```

### Field Changes

**`title_changed`**
- Old value: Previous title
- New value: New title

**`description_changed`**
- Old value: Full previous description
- New value: Full new description
- Message: Optional edit metadata (line ranges, operation type)

**`status_changed`**
- Old value: Previous status (e.g., "todo")
- New value: New status (e.g., "in_progress")

**`story_points_changed`**
- Old value: Previous points
- New value: New points

### Relationship Changes

**`assignee_added`**
- New value: Assignee name
- Message: Optional handoff context

**`assignee_removed`**
- Old value: Removed assignee name

**`label_added`**
- New value: Label name

**`label_removed`**
- Old value: Label name

**`dependency_added`**
- New value: Dependency ticket ID + title (JSON)

**`dependency_removed`**
- Old value: Dependency ticket ID

**`parent_changed`**
- Old value: Previous parent ticket
- New value: New parent ticket

**`epic_changed`**
- Old value: Previous epic
- New value: New epic

### Collaboration Events

**`comment_added`**
- New value: Comment content
- Captures: Author, timestamp (via changed_by/changed_at)

**`commit_linked`**
- New value: Commit hash + message (JSON)

**`added_to_sprint`**
- New value: Sprint ID + name

**`removed_from_sprint`**
- Old value: Sprint ID + name

---

## Description Editing: The Killer Feature

Since descriptions can be long, Jility supports **precise editing** to save tokens and make changes clear.

### Edit Operations

**1. Replace All**
```rust
EditOperation::ReplaceAll {
    content: String
}
```
- Replaces entire description
- Stores full old/new content

**2. Append**
```rust
EditOperation::Append {
    content: String
}
```
- Adds content to end
- Old value: previous description
- New value: previous + "\n" + content

**3. Prepend**
```rust
EditOperation::Prepend {
    content: String
}
```
- Adds content to beginning

**4. Replace Lines**
```rust
EditOperation::ReplaceLines {
    start_line: usize,
    end_line: usize,
    content: String
}
```
- Replaces specific line range
- Message field stores: `{"operation":"replace_lines","start_line":8,"end_line":10}`
- Most token-efficient for agents!

**5. Replace Section**
```rust
EditOperation::ReplaceSection {
    section_header: String,  // e.g., "## Acceptance Criteria"
    content: String
}
```
- Finds markdown section by header
- Replaces content between header and next header
- Message field stores: `{"operation":"replace_section","section":"Acceptance Criteria"}`

### Example: Agent Updates Checklist

**Original description:**
```markdown
## Context
Implement JWT token generation for API auth.

## Acceptance Criteria
- [ ] Generate JWT with configurable expiration
- [ ] Include user claims (id, email, role)
- [ ] Sign with RS256 algorithm
- [ ] Unit tests cover happy path and errors
```

**Agent makes progress, uses replace_lines:**

```rust
jility_update_description(
    ticket_id: "TASK-42",
    operation: "replace_lines",
    start_line: 6,
    end_line: 6,
    content: "- [x] Generate JWT with configurable expiration",
    message: "Completed first acceptance criterion"
)
```

**Change recorded:**
```json
{
  "change_type": "description_changed",
  "field_name": "description",
  "old_value": "- [ ] Generate JWT with configurable expiration",
  "new_value": "- [x] Generate JWT with configurable expiration",
  "changed_by": "agent-1",
  "message": "{\"operation\":\"replace_lines\",\"start_line\":6,\"end_line\":6}"
}
```

**Timeline shows:**
```
📝 agent-1 updated description                       2 mins ago
   Changed line 6: Marked checklist item complete
```

---

## Viewing History

### 1. Timeline View (Default)

**CLI:**
```bash
jility ticket history TASK-42

# Output:
📋 TASK-42: Implement JWT token generation

📝 agent-1 created ticket                           3 hours ago

⚡ alice moved from Backlog to In Progress           2 hours ago

👥 alice assigned to agent-1                         2 hours ago
   "I've set up the basic structure, please implement the signing logic"

📝 agent-1 updated description                       30 mins ago
   Changed lines 6-9: Updated progress on checklist

🔗 agent-1 linked commit abc123f                     15 mins ago
   "Add JWT generation service with tests"

💬 alice commented                                   5 mins ago
   "Make sure to handle token expiration properly"
```

**MCP Tool:**
```rust
jility_get_ticket_context(ticket_id: "TASK-42")
// Returns ticket with recent_changes array (last 20 changes)
```

**Web UI:**
- Activity timeline panel on ticket detail page
- Real-time updates via WebSocket
- Infinite scroll for full history

### 2. Diff View (Compare Versions)

**CLI:**
```bash
jility ticket diff TASK-42 --from="2024-10-23T10:00" --to="2024-10-23T14:00"

# Output:
📋 Changes to TASK-42 from 10:00 to 14:00

Title: (unchanged)
  Implement JWT token generation

Status:
- backlog
+ in_progress

Assignees:
+ agent-1 (added by alice at 12:00)

Description:
  ## Acceptance Criteria
- - [ ] Generate JWT with configurable expiration
+ - [x] Generate JWT with configurable expiration
  - [ ] Include user claims (id, email, role)
  - [ ] Sign with RS256 algorithm

Labels:
+ backend (added at 10:30)
+ security (added at 10:30)

Commits:
+ abc123f - "Add JWT generation service" (linked at 13:45)
```

**Web UI:**
- Side-by-side diff view
- Color-coded additions/removals
- Click any timeline event to "jump to this version"

### 3. Point-in-Time Snapshot

**CLI:**
```bash
jility ticket show TASK-42 --at="2024-10-23T12:00"

# Shows ticket state at noon, before recent changes
```

**Implementation:**
```rust
async fn get_ticket_at_time(
    db: &DatabaseConnection,
    ticket_id: Uuid,
    timestamp: DateTime<Utc>,
) -> Result<TicketSnapshot> {
    // 1. Get creation event
    let created = TicketChange::find()
        .filter(ticket_change::Column::TicketId.eq(ticket_id))
        .filter(ticket_change::Column::ChangeType.eq("created"))
        .one(db)
        .await?
        .ok_or(anyhow!("Ticket not found"))?;

    let mut snapshot: TicketSnapshot = serde_json::from_str(&created.new_value.unwrap())?;

    // 2. Get all changes after creation, up to timestamp
    let changes = TicketChange::find()
        .filter(ticket_change::Column::TicketId.eq(ticket_id))
        .filter(ticket_change::Column::ChangedAt.gt(created.changed_at))
        .filter(ticket_change::Column::ChangedAt.lte(timestamp))
        .order_by_asc(ticket_change::Column::ChangedAt)
        .all(db)
        .await?;

    // 3. Apply changes in chronological order
    for change in changes {
        apply_change_to_snapshot(&mut snapshot, &change)?;
    }

    Ok(snapshot)
}

fn apply_change_to_snapshot(snapshot: &mut TicketSnapshot, change: &TicketChange) -> Result<()> {
    match change.change_type.as_str() {
        "title_changed" => {
            snapshot.title = change.new_value.clone().unwrap();
        }
        "description_changed" => {
            snapshot.description = change.new_value.clone().unwrap();
        }
        "status_changed" => {
            snapshot.status = change.new_value.clone().unwrap();
        }
        "story_points_changed" => {
            snapshot.story_points = change.new_value.as_ref().and_then(|v| v.parse().ok());
        }
        "assignee_added" => {
            snapshot.assignees.push(change.new_value.clone().unwrap());
        }
        "assignee_removed" => {
            snapshot.assignees.retain(|a| a != change.new_value.as_ref().unwrap());
        }
        "label_added" => {
            snapshot.labels.push(change.new_value.clone().unwrap());
        }
        "label_removed" => {
            snapshot.labels.retain(|l| l != change.new_value.as_ref().unwrap());
        }
        // ... handle all change types
        _ => {}
    }
    Ok(())
}
```

### 4. Filtered History

**CLI:**
```bash
# Only show changes by specific person
jility ticket history TASK-42 --by=agent-1

# Only show specific change types
jility ticket history TASK-42 --types=description_changed,comment_added

# Date range
jility ticket history TASK-42 --since="2024-10-20" --until="2024-10-23"
```

---

## Reverting Changes

### Revert to Previous State

**CLI:**
```bash
# Revert description to state from 1 hour ago
jility ticket revert TASK-42 --field=description --to="2024-10-23T13:00"

# Revert entire ticket to previous state
jility ticket revert TASK-42 --to="2024-10-23T13:00" --all

# Revert last change
jility ticket undo TASK-42
```

**Implementation:**
```rust
async fn revert_ticket_field(
    db: &DatabaseConnection,
    ticket_id: Uuid,
    field: &str,
    timestamp: DateTime<Utc>,
    reverted_by: String,
) -> Result<()> {
    // 1. Get ticket state at target time
    let snapshot = get_ticket_at_time(db, ticket_id, timestamp).await?;

    // 2. Get current ticket
    let current = Ticket::find_by_id(ticket_id)
        .one(db)
        .await?
        .ok_or(anyhow!("Ticket not found"))?;

    // 3. Extract field value from snapshot
    let old_value = get_field_value(&current, field)?;
    let new_value = get_field_value_from_snapshot(&snapshot, field)?;

    if old_value == new_value {
        return Err(anyhow!("Field value unchanged at that time"));
    }

    // 4. Update ticket
    update_ticket_field(db, ticket_id, field, &new_value).await?;

    // 5. Record revert as a change
    let change = ticket_change::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(ticket_id),
        change_type: Set(format!("{}_changed", field)),
        field_name: Set(Some(field.to_string())),
        old_value: Set(Some(old_value)),
        new_value: Set(Some(new_value)),
        changed_by: Set(reverted_by),
        changed_at: Set(Utc::now()),
        message: Set(Some(format!(
            "Reverted to state from {}",
            timestamp.to_rfc3339()
        ))),
    };

    change.insert(db).await?;

    Ok(())
}
```

**Timeline shows:**
```
📝 alice reverted description                        just now
   Reverted to state from 2024-10-23T13:00 (1 hour ago)
```

---

## Agent Transparency Features

### 1. Agent Activity Summary

**CLI:**
```bash
jility agent-activity agent-1 --since="2024-10-20"

# Output:
🤖 agent-1 Activity Summary (last 3 days)

Tickets Created: 5
Tickets Completed: 3
Description Updates: 12
Commits Linked: 8
Comments Posted: 4

Recent Activity:
- TASK-42: Updated description (2 mins ago)
- TASK-41: Linked commit def456 (15 mins ago)
- TASK-40: Moved to Done (1 hour ago)
```

**MCP Tool:**
```typescript
jility_get_agent_activity(agent_id: "agent-1", since: "2024-10-20")
```

### 2. Who Did What Report

**CLI:**
```bash
jility stats --by-person --project=current

# Output:
📊 Project Activity by Person

👤 alice
  - 8 tickets created
  - 12 tickets completed
  - 45 comments posted
  - 32 story points delivered

🤖 agent-1
  - 15 tickets created
  - 18 tickets completed
  - 3 comments posted
  - 52 story points delivered

🤖 agent-2
  - 7 tickets created
  - 9 tickets completed
  - 1 comment posted
  - 28 story points delivered
```

### 3. Handoff Tracking

When tickets are reassigned, the change includes optional context:

```rust
assign_ticket(
    ticket_id: "TASK-42",
    new_assignee: "agent-1",
    assigned_by: "alice",
    message: "I've set up the basic structure. Please implement the JWT signing logic using RS256. Check TASK-98 for error handling patterns we're using."
)
```

**Timeline shows:**
```
👥 alice assigned to agent-1                         2 hours ago
   "I've set up the basic structure. Please implement the JWT signing
    logic using RS256. Check TASK-98 for error handling patterns."
```

This context is **visible in the MCP response** when the agent gets the ticket:

```json
{
  "ticket": { ... },
  "recent_changes": [
    {
      "type": "assignee_added",
      "changed_by": "alice",
      "message": "I've set up the basic structure. Please implement...",
      "changed_at": "2024-10-23T12:00:00Z"
    }
  ]
}
```

---

## Web UI: Timeline Components

### Activity Item Component

```jsx
function ActivityItem({ change }) {
  const icon = getIconForChangeType(change.change_type);
  const action = formatAction(change);
  const actorType = change.changed_by.startsWith('agent-') ? 'agent' : 'human';

  return (
    <div className="activity-item">
      <div className="activity-icon">{icon}</div>
      <div className="activity-content">
        <div className="activity-header">
          <ActorBadge name={change.changed_by} type={actorType} />
          <span className="activity-action">{action}</span>
          <TimeAgo timestamp={change.changed_at} />
        </div>
        {change.message && (
          <div className="activity-message">{change.message}</div>
        )}
        {change.change_type === 'description_changed' && (
          <button onClick={() => showDiff(change)}>View Changes</button>
        )}
      </div>
    </div>
  );
}
```

### Diff Modal

```jsx
function DescriptionDiff({ oldValue, newValue }) {
  const diff = computeLineDiff(oldValue, newValue);

  return (
    <div className="diff-view">
      <div className="diff-header">
        <span>Before</span>
        <span>After</span>
      </div>
      <div className="diff-content">
        {diff.map((line, idx) => (
          <DiffLine key={idx} line={line} />
        ))}
      </div>
    </div>
  );
}

function DiffLine({ line }) {
  if (line.type === 'added') {
    return <div className="diff-line added">+ {line.content}</div>;
  } else if (line.type === 'removed') {
    return <div className="diff-line removed">- {line.content}</div>;
  } else {
    return <div className="diff-line unchanged">{line.content}</div>;
  }
}
```

### Version History Dropdown

```jsx
function VersionSelector({ ticketId, currentVersion }) {
  const [changes, setChanges] = useState([]);

  useEffect(() => {
    fetchTicketChanges(ticketId).then(setChanges);
  }, [ticketId]);

  return (
    <select onChange={(e) => loadVersionAtTime(ticketId, e.target.value)}>
      <option value="current">Current (now)</option>
      {changes.map(change => (
        <option key={change.id} value={change.changed_at}>
          {formatTimestamp(change.changed_at)} - {change.changed_by}: {formatAction(change)}
        </option>
      ))}
    </select>
  );
}
```

---

## Performance Optimization

### 1. Limit Timeline Depth

For tickets with hundreds of changes, paginate:

```rust
async fn get_ticket_timeline(
    db: &DatabaseConnection,
    ticket_id: Uuid,
    limit: u64,
    offset: u64,
) -> Result<TimelinePage> {
    let total = TicketChange::find()
        .filter(ticket_change::Column::TicketId.eq(ticket_id))
        .count(db)
        .await?;

    let changes = TicketChange::find()
        .filter(ticket_change::Column::TicketId.eq(ticket_id))
        .order_by_desc(ticket_change::Column::ChangedAt)
        .limit(limit)
        .offset(offset)
        .all(db)
        .await?;

    Ok(TimelinePage {
        changes,
        total,
        has_more: offset + limit < total,
    })
}
```

**Web UI:** Infinite scroll or "Load more" button

**CLI:** `jility ticket history TASK-42 --limit=20 --offset=0`

### 2. Compress Old Descriptions

For description changes older than 30 days, optionally compress:

```rust
// Periodic cleanup job
async fn compress_old_description_changes(db: &DatabaseConnection) -> Result<()> {
    let cutoff = Utc::now() - Duration::days(30);

    let changes = TicketChange::find()
        .filter(ticket_change::Column::ChangeType.eq("description_changed"))
        .filter(ticket_change::Column::ChangedAt.lt(cutoff))
        .all(db)
        .await?;

    for change in changes {
        if let Some(old_val) = &change.old_value {
            let compressed = compress_string(old_val)?;
            // Update with compressed value or move to archive table
        }
    }

    Ok(())
}
```

### 3. Index Strategy

```sql
-- Most common query: Get recent changes for a ticket
CREATE INDEX idx_ticket_changes_ticket_recent
ON ticket_changes(ticket_id, changed_at DESC);

-- Agent activity queries
CREATE INDEX idx_ticket_changes_actor_recent
ON ticket_changes(changed_by, changed_at DESC);

-- Change type filtering
CREATE INDEX idx_ticket_changes_type
ON ticket_changes(change_type, changed_at DESC);
```

---

## Data Retention Policy (Optional - Phase 4)

For very large projects, consider archiving old changes:

```rust
pub struct RetentionPolicy {
    // Keep full history for this long
    retain_full_history_days: u32,  // e.g., 90 days

    // After that, compress to snapshots
    compress_to_daily_snapshots: bool,

    // After this long, only keep creation + final state
    archive_after_days: Option<u32>,  // e.g., 365 days
}
```

**Example:**
- Days 0-90: Full change history
- Days 90-365: Daily snapshots only
- After 365 days: Creation event + final state only

---

## CLI Commands Summary

### Viewing History

```bash
# Full timeline
jility ticket history TASK-42

# Filtered timeline
jility ticket history TASK-42 --by=agent-1
jility ticket history TASK-42 --types=description_changed,comment_added
jility ticket history TASK-42 --since="2024-10-20"

# Compare versions
jility ticket diff TASK-42 --from="10:00" --to="14:00"

# Point-in-time snapshot
jility ticket show TASK-42 --at="2024-10-23T12:00"

# Agent activity
jility agent-activity agent-1 --since="2024-10-20"
```

### Reverting Changes

```bash
# Revert specific field
jility ticket revert TASK-42 --field=description --to="2024-10-23T13:00"

# Undo last change
jility ticket undo TASK-42

# Undo last N changes
jility ticket undo TASK-42 --count=3
```

### Export History

```bash
# Export full ticket history as JSON
jility ticket export TASK-42 --output=task-42-history.json

# Export as Markdown report
jility ticket export TASK-42 --format=markdown --output=task-42-history.md
```

---

## MCP Tools for History

### `jility_get_ticket_context`

Already includes recent changes (last 20) in response.

### `jility_get_ticket_history`

```typescript
{
  "name": "jility_get_ticket_history",
  "description": "Get full change history for a ticket",
  "inputSchema": {
    "type": "object",
    "properties": {
      "ticket_id": { "type": "string" },
      "limit": { "type": "integer", "default": 50 },
      "change_types": {
        "type": "array",
        "items": { "type": "string" },
        "description": "Filter by change types"
      }
    },
    "required": ["ticket_id"]
  }
}
```

### `jility_revert_ticket`

```typescript
{
  "name": "jility_revert_ticket",
  "description": "Revert ticket to previous state",
  "inputSchema": {
    "type": "object",
    "properties": {
      "ticket_id": { "type": "string" },
      "field": { "type": "string", "enum": ["description", "title", "status", "all"] },
      "to_timestamp": { "type": "string", "description": "ISO 8601 timestamp" }
    },
    "required": ["ticket_id", "field", "to_timestamp"]
  }
}
```

---

## Summary

**Every change is tracked** - title, description, status, assignees, labels, dependencies, comments, commits.

**Timeline view** - Human-readable activity feed showing who did what, when.

**Time-travel** - Reconstruct ticket state at any point in history.

**Diffs** - Compare changes between versions.

**Revert** - Roll back to previous states.

**Agent transparency** - Complete audit trail of AI actions.

**Performance** - Indexed queries, pagination, optional compression for old changes.

**User experience:**
- CLI: Rich history commands with filtering
- Web UI: Interactive timeline with diffs and version selector
- MCP: History included in context for agents

This makes Jility the **most transparent project management tool** for human-agent collaboration.
