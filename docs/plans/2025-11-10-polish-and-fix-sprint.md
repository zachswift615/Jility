# Polish & Fix Sprint Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix critical UX bugs, add sprint MCP tools for AI-driven sprint management, complete epic UI features, and polish MCP tool consistency.

**Architecture:** This sprint has 4 phases: (1) Fix blocking bugs in backlog navigation and ticket creation, (2) Add comprehensive sprint management to MCP server for AI-first project management, (3) Complete epic UI with creation and assignment forms, (4) Standardize MCP tool input format across all endpoints.

**Tech Stack:**
- Backend: Rust, Axum, SeaORM, PostgreSQL/SQLite
- Frontend: Next.js 14, TypeScript, Tailwind CSS, shadcn/ui
- MCP: Rust with anthropic-sdk-rust

**Total Story Points:** 19-21 pts (7 tickets)

---

## Phase 1: Critical Bug Fixes (5 pts)

### Task 1: Fix Backlog Page 404 Bug (JIL-31)

**Problem:** Clicking tickets from backlog page returns 404, and back navigation goes to wrong project.

**Files:**
- Investigate: `jility-web/app/w/[slug]/backlog/page.tsx`
- Possibly modify: `jility-web/components/backlog/backlog-view.tsx`

**Step 1: Reproduce the bug**

```bash
# Start dev server
task start-dev

# Navigate to backlog page and click a ticket
# Expected: 404 error
# Expected: Back button goes to wrong project
```

**Step 2: Investigate backlog ticket links**

Read `jility-web/app/w/[slug]/backlog/page.tsx` and `jility-web/components/backlog/backlog-view.tsx` to find:
- How ticket links are constructed
- What route they navigate to
- Why 404 occurs

Common causes:
- Link points to `/ticket/[id]` instead of `/w/[slug]/ticket/[id]`
- Missing workspace slug in URL
- Incorrect route parameters

**Step 3: Fix ticket link construction**

Expected fix in `backlog-view.tsx` or similar:

```typescript
// ❌ WRONG - Missing workspace slug
<Link href={`/ticket/${ticket.id}`}>

// ✅ CORRECT - Include workspace slug
<Link href={`/w/${workspace.slug}/ticket/${ticket.id}`}>
```

**Step 4: Fix back navigation**

Ensure workspace context is preserved. May need to:
- Use `router.back()` instead of hardcoded redirect
- Store workspace slug in session/state
- Update navigation component

**Step 5: Test the fix**

```bash
# Navigate to backlog page
# Click ticket → should navigate to ticket detail (no 404)
# Click back → should return to backlog page in same workspace
```

**Step 6: Commit**

```bash
git add jility-web/components/backlog/backlog-view.tsx
git commit -m "fix: backlog ticket links now navigate correctly with workspace slug

- Fixed ticket URLs to include workspace slug
- Back navigation now returns to correct project
- Resolves JIL-31"
```

---

### Task 2: Fix 422 Error on Second Ticket Creation (JIL-36)

**Problem:** Creating first ticket works, but second ticket fails with 422 Unprocessable Entity.

**Files:**
- Investigate: `jility-web/components/ticket/create-ticket-dialog.tsx`
- Possibly modify: `jility-web/lib/api.ts`

**Step 1: Reproduce the bug**

```bash
# Start dev server
task start-dev

# Open create ticket modal
# Create first ticket → SUCCESS
# Open create ticket modal again
# Create second ticket → 422 ERROR
# Check browser console for error details
```

**Step 2: Investigate form state**

Read `create-ticket-dialog.tsx` to check:
- Is form state being reset after successful creation?
- Are old field values persisting?
- Is project_id being sent correctly?

Common causes:
- Form not clearing after submit
- State containing stale/invalid data
- Missing required fields on second submission

**Step 3: Add form reset after successful creation**

In `create-ticket-dialog.tsx`, find the `handleSubmit` function:

```typescript
const handleSubmit = async () => {
  try {
    const ticket = await api.createTicket({
      title,
      description,
      status,
      project_id: currentProject.id,
      // ... other fields
    })

    // ✅ ADD: Reset form state after success
    setTitle('')
    setDescription('')
    setStatus('backlog')
    setAssignees([])
    setLabels([])
    setStoryPoints(undefined)

    // Close modal
    setIsOpen(false)

    // Refresh ticket list
    onTicketCreated?.(ticket)
  } catch (error) {
    console.error('Failed to create ticket:', error)
  }
}
```

**Step 4: Verify form clears on modal close**

Ensure modal close also resets state:

```typescript
const handleClose = () => {
  // Reset all form fields
  setTitle('')
  setDescription('')
  setStatus('backlog')
  // ... reset other fields

  setIsOpen(false)
}
```

**Step 5: Test the fix**

```bash
# Create first ticket → SUCCESS
# Verify form cleared
# Create second ticket → SUCCESS (no 422 error)
# Create third ticket → SUCCESS
```

**Step 6: Commit**

```bash
git add jility-web/components/ticket/create-ticket-dialog.tsx
git commit -m "fix: reset create ticket form after submission

- Clear all form fields after successful ticket creation
- Reset form state when modal closes
- Prevents 422 error on subsequent ticket creation
- Resolves JIL-36"
```

---

## Phase 2: Sprint MCP Tools (5 pts)

### Task 3: Add Sprint Management MCP Tools (JIL-48)

**Goal:** Add 6 MCP tools for managing sprints programmatically

**Files:**
- Modify: `crates/jility-mcp/src/service.rs`
- Reference: `jility-server/src/api/sprints.rs` (existing API endpoints)
- Update: `.claude/CLAUDE.md` (documentation)

**Step 1: Add create_sprint tool**

In `crates/jility-mcp/src/service.rs`, add after other tools:

```rust
#[tool(
    description = "Create a new sprint with name, capacity, and optional dates. Returns sprint ID and details."
)]
pub async fn create_sprint(
    &self,
    #[tool(param, description = "Sprint name (e.g., 'Polish & Fix Sprint')")]
    name: String,
    #[tool(param, description = "Story point capacity for the sprint")]
    capacity: Option<i32>,
    #[tool(param, description = "Start date in YYYY-MM-DD format")]
    start_date: Option<String>,
    #[tool(param, description = "End date in YYYY-MM-DD format")]
    end_date: Option<String>,
) -> Result<String, String> {
    let project_id = &self.project_id;
    let url = format!("{}/projects/{}/sprints", self.api_base_url, project_id);

    let body = serde_json::json!({
        "name": name,
        "capacity": capacity,
        "start_date": start_date,
        "end_date": end_date,
    });

    let response = self.build_request(&url, Method::POST, Some(body)).await?;
    let sprint: serde_json::Value = serde_json::from_str(&response)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let sprint_id = sprint["id"].as_str().unwrap_or("unknown");
    let sprint_name = sprint["name"].as_str().unwrap_or("unknown");

    Ok(format!(
        "✅ Created sprint: {} (ID: {})\n\nCapacity: {} points\nStatus: planned\n\nUse add_ticket_to_sprint to add tickets.",
        sprint_name,
        sprint_id,
        capacity.unwrap_or(0)
    ))
}
```

**Step 2: Add add_ticket_to_sprint tool**

```rust
#[tool(
    description = "Add one or more tickets to a sprint. Accepts ticket IDs or numbers (e.g., 'JIL-31')."
)]
pub async fn add_ticket_to_sprint(
    &self,
    #[tool(param, description = "Sprint ID or name")]
    sprint_id: String,
    #[tool(param, description = "Ticket ID(s) - can be UUID or ticket number like 'JIL-31'")]
    ticket_ids: Vec<String>,
) -> Result<String, String> {
    let url = format!("{}/sprints/{}/tickets", self.api_base_url, sprint_id);

    let mut added = Vec::new();
    let mut failed = Vec::new();

    for ticket_id in ticket_ids {
        let body = serde_json::json!({
            "ticket_id": ticket_id,
        });

        match self.build_request(&url, Method::POST, Some(body)).await {
            Ok(_) => added.push(ticket_id.clone()),
            Err(e) => failed.push(format!("{}: {}", ticket_id, e)),
        }
    }

    let mut result = String::new();
    if !added.is_empty() {
        result.push_str(&format!("✅ Added {} ticket(s) to sprint:\n", added.len()));
        for id in added {
            result.push_str(&format!("  - {}\n", id));
        }
    }
    if !failed.is_empty() {
        result.push_str(&format!("\n❌ Failed to add {} ticket(s):\n", failed.len()));
        for err in failed {
            result.push_str(&format!("  - {}\n", err));
        }
    }

    Ok(result)
}
```

**Step 3: Add start_sprint tool**

```rust
#[tool(
    description = "Start a planned sprint. Moves sprint from 'planned' to 'active' status."
)]
pub async fn start_sprint(
    &self,
    #[tool(param, description = "Sprint ID or name")]
    sprint_id: String,
) -> Result<String, String> {
    let url = format!("{}/sprints/{}/start", self.api_base_url, sprint_id);

    let response = self.build_request(&url, Method::POST, None::<()>).await?;
    let sprint: serde_json::Value = serde_json::from_str(&response)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let sprint_name = sprint["name"].as_str().unwrap_or("unknown");

    Ok(format!("🏃 Started sprint: {}\n\nStatus: active\n\nUse get_sprint_stats to track progress.", sprint_name))
}
```

**Step 4: Add list_sprints tool**

```rust
#[tool(
    description = "List all sprints with optional status filter. Shows sprint name, status, capacity, and ticket count."
)]
pub async fn list_sprints(
    &self,
    #[tool(param, description = "Filter by status: 'active', 'planned', or 'completed'")]
    status: Option<String>,
) -> Result<String, String> {
    let project_id = &self.project_id;
    let mut url = format!("{}/projects/{}/sprints", self.api_base_url, project_id);

    if let Some(status_filter) = status {
        url.push_str(&format!("?status={}", status_filter));
    }

    let response = self.build_request(&url, Method::GET, None::<()>).await?;
    let sprints: Vec<serde_json::Value> = serde_json::from_str(&response)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if sprints.is_empty() {
        return Ok("📋 No sprints found".to_string());
    }

    let mut result = format!("📋 Found {} sprint(s)\n\n", sprints.len());

    for sprint in sprints {
        let name = sprint["name"].as_str().unwrap_or("unknown");
        let status = sprint["status"].as_str().unwrap_or("unknown");
        let capacity = sprint["capacity"].as_i64().unwrap_or(0);
        let ticket_count = sprint["ticket_count"].as_i64().unwrap_or(0);

        let status_emoji = match status {
            "active" => "🏃",
            "planned" => "📋",
            "completed" => "✅",
            _ => "❓",
        };

        result.push_str(&format!(
            "{} {}\n  Status: {} | Capacity: {} pts | Tickets: {}\n\n",
            status_emoji, name, status, capacity, ticket_count
        ));
    }

    Ok(result)
}
```

**Step 5: Add get_sprint_stats tool**

```rust
#[tool(
    description = "Get detailed statistics for a sprint including points breakdown and ticket status."
)]
pub async fn get_sprint_stats(
    &self,
    #[tool(param, description = "Sprint ID or name")]
    sprint_id: String,
) -> Result<String, String> {
    let url = format!("{}/sprints/{}/stats", self.api_base_url, sprint_id);

    let response = self.build_request(&url, Method::GET, None::<()>).await?;
    let stats: serde_json::Value = serde_json::from_str(&response)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let sprint_name = stats["sprint"]["name"].as_str().unwrap_or("unknown");
    let capacity = stats["capacity"].as_i64().unwrap_or(0);
    let total_points = stats["total_points"].as_i64().unwrap_or(0);
    let completed_points = stats["completed_points"].as_i64().unwrap_or(0);
    let total_tickets = stats["total_tickets"].as_i64().unwrap_or(0);
    let completed_tickets = stats["completed_tickets"].as_i64().unwrap_or(0);

    let completion_pct = if total_points > 0 {
        (completed_points * 100) / total_points
    } else {
        0
    };

    Ok(format!(
        "📊 Sprint Stats: {}\n\n\
        Points: {}/{} ({} pts remaining)\n\
        Tickets: {}/{} completed\n\
        Completion: {}%\n\
        Capacity: {} pts",
        sprint_name,
        completed_points, total_points, total_points - completed_points,
        completed_tickets, total_tickets,
        completion_pct,
        capacity
    ))
}
```

**Step 6: Add complete_sprint tool**

```rust
#[tool(
    description = "Complete an active sprint. Moves remaining tickets to backlog and archives the sprint."
)]
pub async fn complete_sprint(
    &self,
    #[tool(param, description = "Sprint ID or name")]
    sprint_id: String,
) -> Result<String, String> {
    let url = format!("{}/sprints/{}/complete", self.api_base_url, sprint_id);

    let response = self.build_request(&url, Method::POST, None::<()>).await?;
    let result: serde_json::Value = serde_json::from_str(&response)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let sprint_name = result["sprint"]["name"].as_str().unwrap_or("unknown");
    let completed_tickets = result["completed_tickets"].as_i64().unwrap_or(0);
    let incomplete_tickets = result["incomplete_tickets"].as_i64().unwrap_or(0);

    Ok(format!(
        "✅ Completed sprint: {}\n\n\
        Completed tickets: {}\n\
        Moved to backlog: {}\n\
        Status: completed",
        sprint_name, completed_tickets, incomplete_tickets
    ))
}
```

**Step 7: Build and test MCP tools**

```bash
# Build MCP server
cargo build --release -p jility-mcp

# Restart Claude Code to pick up new tools
# (User must do this manually)

# Test tools once session restarts:
# 1. create_sprint("Test Sprint", 10)
# 2. add_ticket_to_sprint(sprint_id, ["JIL-31", "JIL-36"])
# 3. start_sprint(sprint_id)
# 4. list_sprints()
# 5. get_sprint_stats(sprint_id)
```

**Step 8: Update documentation**

Add to `.claude/CLAUDE.md` in the "Jility MCP Tools" section:

```markdown
### Sprint Management

- **`mcp__jility__create_sprint`** - Create a new sprint
  - Parameters: `name`, `capacity`, `start_date`, `end_date`
  - Returns: Sprint ID and details

- **`mcp__jility__add_ticket_to_sprint`** - Add tickets to a sprint
  - Parameters: `sprint_id`, `ticket_ids` (array, supports batch)
  - Accepts both UUID and ticket numbers (e.g., "JIL-31")

- **`mcp__jility__start_sprint`** - Start a planned sprint
  - Parameters: `sprint_id`
  - Moves sprint from 'planned' to 'active'

- **`mcp__jility__list_sprints`** - List all sprints
  - Parameters: `status` (optional: 'active', 'planned', 'completed')
  - Shows sprint stats and ticket counts

- **`mcp__jility__get_sprint_stats`** - Get detailed sprint statistics
  - Parameters: `sprint_id`
  - Returns points breakdown, completion %, ticket status

- **`mcp__jility__complete_sprint`** - Complete an active sprint
  - Parameters: `sprint_id`
  - Moves incomplete tickets to backlog

**Example Usage:**
```typescript
// Create sprint
const sprint = await mcp__jility__create_sprint(
  "Polish & Fix Sprint",
  21,  // capacity
  "2025-11-10",
  "2025-11-24"
)

// Add tickets
await mcp__jility__add_ticket_to_sprint(sprint_id, [
  "JIL-31", "JIL-36", "JIL-48", "JIL-47", "JIL-46", "JIL-37", "JIL-42"
])

// Start sprint
await mcp__jility__start_sprint(sprint_id)

// Check progress
await mcp__jility__get_sprint_stats(sprint_id)
// Shows: "14/21 points complete (67%)"
```
```

**Step 9: Commit**

```bash
git add crates/jility-mcp/src/service.rs .claude/CLAUDE.md
git commit -m "feat: add comprehensive sprint management MCP tools

- Added create_sprint for creating sprints with capacity
- Added add_ticket_to_sprint supporting batch operations
- Added start_sprint to activate planned sprints
- Added list_sprints with status filtering
- Added get_sprint_stats for progress tracking
- Added complete_sprint for archiving sprints
- Updated documentation with examples
- Resolves JIL-48"
```

---

## Phase 3: Epic UI Completion (8 pts)

### Task 4: Add Epic Assignment to Ticket Forms (JIL-47)

**Goal:** Add epic dropdown to create/edit ticket forms and display epic in ticket detail view

**Files:**
- Modify: `jility-web/components/ticket/create-ticket-dialog.tsx`
- Modify: `jility-web/app/w/[slug]/ticket/[id]/page.tsx`
- Modify: `jility-web/lib/api.ts` (if needed)
- Modify: `jility-web/lib/types.ts` (if needed)

**Step 1: Update CreateTicketDialog with epic dropdown**

In `create-ticket-dialog.tsx`, add epic state and dropdown:

```typescript
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'

export function CreateTicketDialog() {
  // Existing state...
  const [epicId, setEpicId] = useState<string | undefined>()
  const [epics, setEpics] = useState<Epic[]>([])

  // Fetch epics on mount
  useEffect(() => {
    const fetchEpics = async () => {
      try {
        const epicsList = await api.listEpics(currentProject?.id)
        setEpics(epicsList)
      } catch (error) {
        console.error('Failed to fetch epics:', error)
      }
    }
    fetchEpics()
  }, [currentProject])

  // In the form JSX, add epic dropdown after status:
  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Ticket</DialogTitle>
        </DialogHeader>

        <div className="space-y-4">
          {/* Existing fields: title, description */}

          <div>
            <Label>Status</Label>
            <Select value={status} onValueChange={setStatus}>
              {/* ... status options */}
            </Select>
          </div>

          {/* NEW: Epic dropdown */}
          <div>
            <Label>Epic</Label>
            <Select value={epicId} onValueChange={setEpicId}>
              <SelectTrigger>
                <SelectValue placeholder="None" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="none">None</SelectItem>
                {epics.map(epic => (
                  <SelectItem key={epic.id} value={epic.id}>
                    <div className="flex items-center gap-2">
                      <div
                        className="w-3 h-3 rounded-full"
                        style={{ backgroundColor: epic.epic_color }}
                      />
                      <span>JIL-{epic.number}: {epic.title}</span>
                    </div>
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          {/* Existing fields: assignees, labels, story points */}
        </div>
      </DialogContent>
    </Dialog>
  )
}
```

**Step 2: Update create ticket API call to include epic**

```typescript
const handleSubmit = async () => {
  try {
    const ticket = await api.createTicket({
      title,
      description,
      status,
      parent_epic_id: epicId === 'none' ? undefined : epicId,  // NEW
      assignees,
      labels,
      story_points: storyPoints,
      project_id: currentProject.id,
    })

    // Reset form including epic
    setEpicId(undefined)
    // ... other resets

    onTicketCreated?.(ticket)
    setIsOpen(false)
  } catch (error) {
    console.error('Failed to create ticket:', error)
  }
}
```

**Step 3: Add epic display to ticket detail page**

In `app/w/[slug]/ticket/[id]/page.tsx`:

```typescript
export default function TicketDetailPage() {
  const [ticket, setTicket] = useState<Ticket | null>(null)
  const [epic, setEpic] = useState<Epic | null>(null)

  useEffect(() => {
    const fetchTicket = async () => {
      const data = await api.getTicket(ticketId)
      setTicket(data)

      // Fetch epic if ticket belongs to one
      if (data.parent_epic_id) {
        const epicData = await api.getEpic(data.parent_epic_id)
        setEpic(epicData)
      }
    }
    fetchTicket()
  }, [ticketId])

  return (
    <div className="container mx-auto p-6">
      <div className="flex items-center gap-4 mb-6">
        <h1 className="text-3xl font-bold">
          JIL-{ticket.number}: {ticket.title}
        </h1>

        {/* Epic badge */}
        {epic && (
          <Link href={`/w/${workspace.slug}/epic/${epic.id}`}>
            <Badge
              variant="outline"
              className="cursor-pointer hover:bg-accent"
              style={{ borderColor: epic.epic_color }}
            >
              <Layers className="h-3 w-3 mr-1" />
              JIL-{epic.number}: {epic.title}
            </Badge>
          </Link>
        )}
      </div>

      {/* Rest of ticket detail */}
    </div>
  )
}
```

**Step 4: Add epic editing capability**

Add an "Edit Epic" button that allows changing the epic assignment:

```typescript
const [isEditingEpic, setIsEditingEpic] = useState(false)
const [selectedEpicId, setSelectedEpicId] = useState<string | undefined>(ticket?.parent_epic_id)

const handleUpdateEpic = async () => {
  try {
    await api.updateTicket(ticket.id, {
      parent_epic_id: selectedEpicId === 'none' ? null : selectedEpicId,
    })

    // Refresh ticket
    const updated = await api.getTicket(ticket.id)
    setTicket(updated)
    setIsEditingEpic(false)
  } catch (error) {
    console.error('Failed to update epic:', error)
  }
}
```

**Step 5: Test epic assignment**

```bash
# Start dev server
task start-dev

# Test create ticket:
# 1. Open create ticket modal
# 2. Verify epic dropdown shows all epics with colors
# 3. Select an epic
# 4. Create ticket
# 5. Verify ticket appears with epic badge

# Test ticket detail:
# 1. Click ticket with epic
# 2. Verify epic badge shows in header
# 3. Click epic badge → navigate to epic detail
# 4. Test editing epic assignment
```

**Step 6: Commit**

```bash
git add jility-web/components/ticket/create-ticket-dialog.tsx
git add jility-web/app/w/[slug]/ticket/[id]/page.tsx
git commit -m "feat: add epic assignment to ticket create/edit forms

- Added epic dropdown to create ticket modal
- Shows epics with color indicators
- Display epic badge in ticket detail view
- Epic badge clickable → navigates to epic detail
- Can change epic assignment from ticket detail
- Resolves JIL-47"
```

---

### Task 5: Add Epic Creation UI (JIL-46)

**Goal:** Add "Create Epic" button and modal to epics page

**Files:**
- Create: `jility-web/components/epic/create-epic-dialog.tsx`
- Modify: `jility-web/app/w/[slug]/epics/page.tsx`

**Step 1: Create CreateEpicDialog component**

Create new file `jility-web/components/epic/create-epic-dialog.tsx`:

```typescript
'use client'

import { useState } from 'react'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Textarea } from '@/components/ui/textarea'
import { api } from '@/lib/api'
import { Epic } from '@/lib/types'

interface CreateEpicDialogProps {
  isOpen: boolean
  onClose: () => void
  onEpicCreated: (epic: Epic) => void
  projectId: string
}

const PRESET_COLORS = [
  { name: 'Blue', value: '#3b82f6' },
  { name: 'Green', value: '#10b981' },
  { name: 'Orange', value: '#f59e0b' },
  { name: 'Red', value: '#ef4444' },
  { name: 'Purple', value: '#8b5cf6' },
  { name: 'Pink', value: '#ec4899' },
  { name: 'Teal', value: '#14b8a6' },
  { name: 'Indigo', value: '#6366f1' },
]

export function CreateEpicDialog({ isOpen, onClose, onEpicCreated, projectId }: CreateEpicDialogProps) {
  const [title, setTitle] = useState('')
  const [description, setDescription] = useState('')
  const [selectedColor, setSelectedColor] = useState(PRESET_COLORS[0].value)
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleSubmit = async () => {
    if (!title.trim()) {
      setError('Title is required')
      return
    }

    setIsSubmitting(true)
    setError(null)

    try {
      const epic = await api.createTicket({
        title,
        description,
        is_epic: true,
        epic_color: selectedColor,
        status: 'backlog',
        project_id: projectId,
      })

      // Reset form
      setTitle('')
      setDescription('')
      setSelectedColor(PRESET_COLORS[0].value)

      onEpicCreated(epic)
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create epic')
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="sm:max-w-[500px]">
        <DialogHeader>
          <DialogTitle>Create Epic</DialogTitle>
        </DialogHeader>

        <div className="space-y-4">
          <div>
            <Label htmlFor="title">Title *</Label>
            <Input
              id="title"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              placeholder="e.g., User Authentication System"
            />
          </div>

          <div>
            <Label htmlFor="description">Description</Label>
            <Textarea
              id="description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="Describe the epic's goals and scope..."
              rows={4}
            />
          </div>

          <div>
            <Label>Epic Color</Label>
            <div className="grid grid-cols-4 gap-2 mt-2">
              {PRESET_COLORS.map(color => (
                <button
                  key={color.value}
                  type="button"
                  onClick={() => setSelectedColor(color.value)}
                  className={`
                    h-10 rounded-md border-2 transition-all
                    ${selectedColor === color.value ? 'border-foreground scale-110' : 'border-border'}
                  `}
                  style={{ backgroundColor: color.value }}
                  title={color.name}
                />
              ))}
            </div>
          </div>

          {error && (
            <div className="text-sm text-destructive">
              {error}
            </div>
          )}

          <div className="flex justify-end gap-2">
            <Button variant="outline" onClick={onClose}>
              Cancel
            </Button>
            <Button onClick={handleSubmit} disabled={isSubmitting}>
              {isSubmitting ? 'Creating...' : 'Create Epic'}
            </Button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  )
}
```

**Step 2: Add Create Epic button to epics page**

Modify `jility-web/app/w/[slug]/epics/page.tsx`:

```typescript
'use client'

import { useState, useEffect } from 'react'
import { Button } from '@/components/ui/button'
import { Plus } from 'lucide-react'
import { EpicCard } from '@/components/epic-card'
import { CreateEpicDialog } from '@/components/epic/create-epic-dialog'
import { api } from '@/lib/api'
import { Epic } from '@/lib/types'

export default function EpicsPage() {
  const [epics, setEpics] = useState<Epic[]>([])
  const [isCreateOpen, setIsCreateOpen] = useState(false)
  const [currentProject, setCurrentProject] = useState(null)

  useEffect(() => {
    const fetchEpics = async () => {
      // Fetch current project
      const project = await api.getCurrentProject()
      setCurrentProject(project)

      // Fetch epics
      const data = await api.listEpics(project.id)
      setEpics(data)
    }
    fetchEpics()
  }, [])

  const handleEpicCreated = (newEpic: Epic) => {
    setEpics(prev => [newEpic, ...prev])
  }

  return (
    <div className="container mx-auto p-6">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold">Epics</h1>

        <Button onClick={() => setIsCreateOpen(true)}>
          <Plus className="h-4 w-4 mr-2" />
          Create Epic
        </Button>
      </div>

      {epics.length === 0 ? (
        <div className="text-center py-12 text-muted-foreground">
          <p>No epics yet. Create your first epic to get started!</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {epics.map(epic => (
            <EpicCard key={epic.id} epic={epic} />
          ))}
        </div>
      )}

      <CreateEpicDialog
        isOpen={isCreateOpen}
        onClose={() => setIsCreateOpen(false)}
        onEpicCreated={handleEpicCreated}
        projectId={currentProject?.id}
      />
    </div>
  )
}
```

**Step 3: Test epic creation**

```bash
# Start dev server
task start-dev

# Navigate to /epics page
# Click "Create Epic" button
# Fill in:
#   Title: "Test Epic"
#   Description: "Testing epic creation"
#   Color: Select a color
# Click "Create Epic"
# Verify:
#   - Epic appears immediately in grid
#   - Epic has correct color
#   - Form closes and resets
```

**Step 4: Commit**

```bash
git add jility-web/components/epic/create-epic-dialog.tsx
git add jility-web/app/w/[slug]/epics/page.tsx
git commit -m "feat: add epic creation UI with color picker

- Created CreateEpicDialog component with preset colors
- Added Create Epic button to epics page
- Color picker shows 8 preset colors
- Epic appears immediately after creation
- Form resets and closes on success
- Resolves JIL-46"
```

---

## Phase 4: MCP Tool Consistency (4 pts)

### Task 6: Fix update_status to Accept Ticket Numbers (JIL-37)

**Goal:** Make update_status MCP tool accept both UUID and ticket number format (e.g., "LIS-4")

**Files:**
- Modify: `jility-server/src/api/tickets.rs` (update_status endpoint)
- Reference: `jility-server/src/api/comments.rs` (working example)

**Step 1: Find the update_status endpoint**

```bash
grep -n "pub async fn update_status" jility-server/src/api/tickets.rs
```

**Step 2: Add ticket number parsing**

In `jility-server/src/api/tickets.rs`, find the `update_status` function and replace the UUID parsing:

```rust
pub async fn update_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateStatusRequest>,
) -> ApiResult<Json<TicketResponse>> {
    // ❌ OLD: Only accepts UUID
    // let ticket_id = Uuid::parse_str(&id)
    //     .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket ID: {}", id)))?;

    // ✅ NEW: Accept both UUID and ticket number
    let ticket_uuid = if let Ok(ticket_id) = Uuid::parse_str(&id) {
        ticket_id
    } else {
        // Parse PROJECT-NUMBER format (e.g., "LIS-4")
        let parts: Vec<&str> = id.split('-').collect();
        if parts.len() != 2 {
            return Err(ApiError::InvalidInput(
                format!("Invalid ticket ID format: {}. Use UUID or PROJECT-NUMBER (e.g., JIL-42)", id)
            ));
        }

        let project_key = parts[0];
        let ticket_number: i32 = parts[1]
            .parse()
            .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket number: {}", parts[1])))?;

        // Find project by key
        let project = Project::find()
            .filter(project::Column::Key.eq(project_key))
            .one(state.db.as_ref())
            .await
            .map_err(ApiError::from)?
            .ok_or_else(|| ApiError::NotFound(format!("Project not found: {}", project_key)))?;

        // Find ticket by number
        let ticket = Ticket::find()
            .filter(ticket::Column::ProjectId.eq(project.id))
            .filter(ticket::Column::Number.eq(ticket_number))
            .filter(ticket::Column::DeletedAt.is_null())
            .one(state.db.as_ref())
            .await
            .map_err(ApiError::from)?
            .ok_or_else(|| ApiError::NotFound(
                format!("Ticket not found: {}-{}", project_key, ticket_number)
            ))?;

        ticket.id
    };

    // Rest of function uses ticket_uuid as before
    let ticket = Ticket::find_by_id(ticket_uuid)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Ticket not found: {}", id)))?;

    // ... rest of update logic
}
```

**Step 3: Test with both formats**

```bash
# Build backend
cargo build --release -p jility-server

# Restart backend server
task stop
task start

# Test via MCP (after restarting Claude Code):
# mcp__jility__update_status("JIL-31", "in_progress")
# mcp__jility__update_status(uuid, "done")
```

**Step 4: Commit**

```bash
git add jility-server/src/api/tickets.rs
git commit -m "fix: update_status endpoint now accepts ticket number format

- Added ticket number parsing (e.g., 'JIL-31')
- Matches pattern from comments endpoint
- Supports both UUID and PROJECT-NUMBER format
- Resolves JIL-37"
```

---

### Task 7: Fix delete_ticket to Accept Ticket Numbers (JIL-42)

**Goal:** Make delete_ticket endpoint accept both UUID and ticket number format

**Files:**
- Modify: `jility-server/src/api/tickets.rs` (delete_ticket endpoint)

**Step 1: Find the delete_ticket endpoint**

```bash
grep -n "pub async fn delete_ticket" jility-server/src/api/tickets.rs
```

**Step 2: Add ticket number parsing (same pattern as Task 6)**

In `jility-server/src/api/tickets.rs`, update the `delete_ticket` function:

```rust
pub async fn delete_ticket(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    // ✅ NEW: Accept both UUID and ticket number
    let ticket_uuid = if let Ok(ticket_id) = Uuid::parse_str(&id) {
        ticket_id
    } else {
        // Parse PROJECT-NUMBER format (e.g., "JIL-41")
        let parts: Vec<&str> = id.split('-').collect();
        if parts.len() != 2 {
            return Err(ApiError::InvalidInput(
                format!("Invalid ticket ID format: {}. Use UUID or PROJECT-NUMBER (e.g., JIL-42)", id)
            ));
        }

        let project_key = parts[0];
        let ticket_number: i32 = parts[1]
            .parse()
            .map_err(|_| ApiError::InvalidInput(format!("Invalid ticket number: {}", parts[1])))?;

        // Find project by key
        let project = Project::find()
            .filter(project::Column::Key.eq(project_key))
            .one(state.db.as_ref())
            .await
            .map_err(ApiError::from)?
            .ok_or_else(|| ApiError::NotFound(format!("Project not found: {}", project_key)))?;

        // Find ticket by number
        let ticket = Ticket::find()
            .filter(ticket::Column::ProjectId.eq(project.id))
            .filter(ticket::Column::Number.eq(ticket_number))
            .filter(ticket::Column::DeletedAt.is_null())
            .one(state.db.as_ref())
            .await
            .map_err(ApiError::from)?
            .ok_or_else(|| ApiError::NotFound(
                format!("Ticket not found: {}-{}", project_key, ticket_number)
            ))?;

        ticket.id
    };

    // Rest of function uses ticket_uuid
    let ticket = Ticket::find_by_id(ticket_uuid)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Ticket not found: {}", id)))?;

    // Soft delete: set deleted_at timestamp
    let mut ticket: ticket::ActiveModel = ticket.into();
    let now = Utc::now();
    ticket.deleted_at = Set(Some(now));
    ticket.updated_at = Set(now);

    ticket.update(state.db.as_ref()).await.map_err(ApiError::from)?;

    Ok(Json(serde_json::json!({ "success": true })))
}
```

**Step 3: Test with both formats**

```bash
# Build backend
cargo build --release -p jility-server

# Restart backend
task stop
task start

# Test via MCP:
# mcp__jility__delete_ticket("JIL-41")  # Test ticket we created earlier
# Should succeed instead of 422 error
```

**Step 4: Commit**

```bash
git add jility-server/src/api/tickets.rs
git commit -m "fix: delete_ticket endpoint now accepts ticket number format

- Added ticket number parsing (e.g., 'JIL-41')
- Consistent with update_status and comments endpoints
- Supports both UUID and PROJECT-NUMBER format
- Resolves JIL-42"
```

---

## Testing & Verification

### End-to-End Sprint Test

After completing all tasks, verify the entire sprint:

**Step 1: Test critical bug fixes**
```bash
# JIL-31: Backlog navigation
# - Navigate to backlog
# - Click ticket → should work (no 404)
# - Back button → returns to backlog in same workspace

# JIL-36: Multiple ticket creation
# - Create ticket 1 → SUCCESS
# - Create ticket 2 → SUCCESS (no 422)
# - Create ticket 3 → SUCCESS
```

**Step 2: Test sprint MCP tools**
```bash
# Create sprint via MCP
mcp__jility__create_sprint("Polish & Fix Sprint", 21)

# Add all sprint tickets
mcp__jility__add_ticket_to_sprint(sprint_id, [
  "JIL-31", "JIL-36", "JIL-48", "JIL-47", "JIL-46", "JIL-37", "JIL-42"
])

# Start sprint
mcp__jility__start_sprint(sprint_id)

# Check stats
mcp__jility__get_sprint_stats(sprint_id)
```

**Step 3: Test epic UI**
```bash
# Create epic from UI
# - Navigate to /epics
# - Click "Create Epic"
# - Fill form, select color
# - Verify epic appears

# Assign ticket to epic
# - Open create ticket modal
# - Select epic from dropdown
# - Create ticket
# - Verify epic badge appears

# View epic from ticket
# - Click ticket with epic
# - Verify epic badge in header
# - Click epic badge → navigate to epic detail
```

**Step 4: Test MCP consistency**
```bash
# Test update_status with ticket number
mcp__jility__update_status("JIL-31", "in_progress")

# Test delete_ticket with ticket number
mcp__jility__create_ticket(title: "Test Delete")
mcp__jility__delete_ticket("JIL-XX")  # Use actual ticket number
```

### Final Verification Checklist

- [ ] All 7 tickets resolved (JIL-31, 36, 48, 47, 46, 37, 42)
- [ ] Backlog navigation works without 404
- [ ] Can create multiple tickets without 422 error
- [ ] Sprint MCP tools working (all 6 tools)
- [ ] Epic creation UI working with color picker
- [ ] Epic assignment working in ticket forms
- [ ] Epic badges clickable in ticket detail
- [ ] MCP tools accept ticket numbers consistently
- [ ] All tests passing: `cargo test --workspace`
- [ ] Frontend builds: `cd jility-web && npm run build`
- [ ] Documentation updated in `.claude/CLAUDE.md`

---

## Commit Summary

Expected commits (7 total):

1. `fix: backlog ticket links now navigate correctly with workspace slug`
2. `fix: reset create ticket form after submission`
3. `feat: add comprehensive sprint management MCP tools`
4. `feat: add epic assignment to ticket create/edit forms`
5. `feat: add epic creation UI with color picker`
6. `fix: update_status endpoint now accepts ticket number format`
7. `fix: delete_ticket endpoint now accepts ticket number format`

---

## Success Metrics

**After this sprint:**
- ✅ Critical UX bugs fixed (backlog navigation, ticket creation)
- ✅ Jility can manage its own sprints via MCP (AI-first!)
- ✅ Epic feature complete with full UI/UX
- ✅ MCP tools have consistent, professional API
- ✅ 19-21 story points delivered
- ✅ Zero broken features, all tests passing

**This makes Jility production-ready for solo developers and small teams!** 🚀
