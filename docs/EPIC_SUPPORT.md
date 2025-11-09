# Epic Support in Jility

**Epic support** in Jility provides JIRA-like hierarchical organization for tickets, allowing you to group related work into themed packages with automatic progress tracking.

---

## What are Epics?

**Epics** are high-level work containers that group related tickets together. Think of them as:
- Large features that need multiple tickets to complete
- Themed work packages (e.g., "User Authentication", "Payment Integration")
- Organizational units for complex projects
- Progress tracking containers

**Key characteristics:**
- Epics are special tickets with `is_epic = true`
- Epics can have child tickets (regular tasks)
- Regular tickets can belong to one epic (via `parent_epic_id`)
- Epics **cannot be nested** (no epic-of-epic for simplicity)
- Each epic can have a custom color for visual organization

---

## Creating Epics

### Via Web UI

1. **Navigate to Epics page** (`/epics`)
2. **Click "Create Epic"** button
3. **Fill in the form:**
   - Title: Name of the epic (e.g., "User Authentication System")
   - Description: Detailed explanation of the epic's scope
   - Epic Color: Choose a color for visual identification (optional, defaults to blue)
4. **Click "Create"**

The epic appears in the grid view with a progress bar showing 0/0 tasks completed.

### Via MCP (for AI Agents)

```typescript
// Create an epic
const epic = await mcp__jility__create_epic({
  title: "User Authentication System",
  description: "Complete authentication with login, registration, password reset, and OAuth",
  epic_color: "#3b82f6"  // Optional: hex color code
})

// Returns: { id: "uuid", number: "JIL-42", title: "...", ... }
```

### Via API

```bash
curl -X POST http://localhost:3000/api/tickets \
  -H "Content-Type: application/json" \
  -d '{
    "title": "User Authentication System",
    "description": "Complete authentication system",
    "is_epic": true,
    "epic_color": "#3b82f6"
  }'
```

---

## Organizing Tickets into Epics

### Assigning Tickets to Epics

**During ticket creation (UI):**
1. Create or edit a ticket
2. In the "Epic" dropdown, select the epic this ticket belongs to
3. Save the ticket

**During ticket creation (MCP):**
```typescript
await mcp__jility__create_ticket({
  title: "Build login UI",
  description: "Create login form with email/password",
  story_points: 3,
  parent_epic_id: epic.id,  // Link to epic
  labels: ["frontend", "ui"]
})
```

**During ticket creation (API):**
```bash
curl -X POST http://localhost:3000/api/tickets \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Build login UI",
    "parent_epic_id": "uuid-of-epic",
    "story_points": 3
  }'
```

### Batch Creating Epic Tickets (MCP)

```typescript
// Create multiple tickets for an epic at once
await mcp__jility__create_tickets_batch({
  parent_id: epic.id,
  tickets: [
    {
      title: "Design database schema for users",
      story_points: 2,
      labels: ["backend", "database"]
    },
    {
      title: "Implement JWT token generation",
      story_points: 3,
      labels: ["backend"]
    },
    {
      title: "Build login UI component",
      story_points: 3,
      labels: ["frontend", "ui"]
    },
    {
      title: "Add password reset flow",
      story_points: 5,
      labels: ["backend", "frontend"]
    }
  ]
})
```

---

## Epic Progress Tracking

### How Progress is Calculated

Epic progress is **automatically calculated** based on the status of child tickets:

- **Total:** Count of all tickets in the epic
- **Done:** Count of tickets with status = `done`
- **In Progress:** Count of tickets with status = `in_progress`
- **Todo:** Count of tickets with status = `todo` or `backlog`
- **Blocked:** Count of tickets with status = `blocked`
- **Review:** Count of tickets with status = `review`
- **Completion Percentage:** `(done / total) * 100`

**Example:**
```json
{
  "epic": "User Authentication",
  "progress": {
    "total": 10,
    "done": 3,
    "in_progress": 2,
    "todo": 4,
    "blocked": 1,
    "completion_percentage": 30
  }
}
```

### Viewing Progress

**Epic Board (`/epics`):**
- Each epic card shows a progress bar
- Text displays: "X/Y tasks completed"
- Status breakdown: "3 done, 2 in progress, 5 todo"

**Epic Detail Page (`/epics/:id`):**
- Large progress bar with percentage
- Detailed stats breakdown
- Visual status distribution

**MCP/API:**
```typescript
// List all epics with progress
const epics = await mcp__jility__list_epics()

// Each epic includes progress object
epics.forEach(epic => {
  console.log(`${epic.title}: ${epic.progress.done}/${epic.progress.total} (${epic.progress.completion_percentage}%)`)
})
```

---

## Epic Visualization

### Color Coding

**Purpose:** Visually distinguish epics at a glance.

**How it works:**
- Each epic can have a custom color (hex code like `#3b82f6`)
- Colors appear as:
  - Left border on epic cards
  - Colored badge on ticket cards that belong to the epic
  - Background accent in epic detail view

**Color picker:**
- Available when creating/editing epics
- Default color: Theme primary blue
- Recommended: Use distinct colors for different epics

### Epic Badges on Tickets

When viewing tickets on the board or in lists:
- Tickets belonging to an epic show a **colored badge** with the epic name
- Badge uses the epic's custom color
- Click the badge to filter the board to that epic
- Helps identify which tickets are part of which epic

---

## Epic Filtering and Navigation

### Board View Filtering

**Filter by Epic on Board (`/board`):**
1. Look for the "Epic" filter dropdown at the top
2. Select an epic from the list
3. Board shows only tickets belonging to that epic
4. URL updates: `/board?epic=uuid`
5. Filter persists across page refreshes

**Filter Options:**
- "All tickets" - Show everything (default)
- "No epic" - Show only tickets without an epic
- [Epic names] - Show tickets for specific epic

### Epic Detail View

**Navigate to Epic Detail (`/epics/:id`):**
- Click an epic card on the epics board
- View epic-specific kanban board (filtered to this epic only)
- See all tickets organized by status
- Add new tickets directly to the epic

**Epic Detail Sections:**
1. **Header:** Title, description, color indicator
2. **Progress:** Visual progress bar and stats
3. **Kanban Board:** Filtered to show only this epic's tickets
4. **"Add Task" button:** Creates ticket with `parent_epic_id` pre-filled

### MCP/API Filtering

```typescript
// List all tickets for a specific epic
const epicTickets = await mcp__jility__list_tickets({
  epic_id: "uuid-of-epic"
})

// Or use the dedicated endpoint
const result = await fetch(`/api/epics/${epicId}/tickets`)
// Returns: { epic, tickets, progress }
```

---

## Epic Hierarchy Rules

### What You CAN Do
✅ Create epics (special tickets with `is_epic = true`)
✅ Assign tickets to epics (via `parent_epic_id`)
✅ Have multiple epics in one project
✅ Have tickets that don't belong to any epic
✅ Change a ticket's epic assignment
✅ Delete epics (see below)

### What You CANNOT Do
❌ Nest epics (no epic-of-epic)
❌ Assign a ticket to multiple epics
❌ Set `parent_epic_id` on an epic itself
❌ Create circular dependencies with epics

### Validation Rules

**When creating an epic:**
- Must have `is_epic = true`
- Cannot have `parent_epic_id` set
- `epic_color` is optional (defaults to theme primary)

**When assigning a ticket to an epic:**
- The epic must exist and have `is_epic = true`
- The ticket must not be an epic itself
- Only one epic per ticket

---

## Epic Deletion

### What Happens When You Delete an Epic?

**Current behavior (soft delete):**
1. Epic is marked as deleted (`deleted_at` timestamp set)
2. Epic no longer appears in lists or boards
3. **Child tickets are orphaned** - their `parent_epic_id` is set to `null`
4. Epic is preserved in database for audit trail

**To delete an epic:**

**Via UI:**
1. Navigate to epic detail page
2. Click "Delete Epic" button
3. Confirm deletion in dialog

**Via MCP:**
```typescript
await mcp__jility__delete_ticket({
  ticket_id: epic.id
})
```

**Via API:**
```bash
curl -X DELETE http://localhost:3000/api/tickets/{epic-id}
```

**Important:** Deleting an epic does NOT delete its child tickets. They simply lose their epic assignment.

---

## Common Workflows

### Workflow 1: Planning a New Feature

```typescript
// 1. Create the epic
const epic = await mcp__jility__create_epic({
  title: "Payment Integration",
  description: "Integrate Stripe for payments",
  epic_color: "#10b981"  // Green
})

// 2. Break down into tasks
await mcp__jility__create_tickets_batch({
  parent_id: epic.id,
  tickets: [
    { title: "Research Stripe API", story_points: 2 },
    { title: "Set up Stripe account", story_points: 1 },
    { title: "Implement payment flow", story_points: 5 },
    { title: "Add webhook handlers", story_points: 3 },
    { title: "Test payment scenarios", story_points: 3 }
  ]
})

// 3. Track progress
const epics = await mcp__jility__list_epics()
console.log(`Payment Integration: ${epics[0].progress.completion_percentage}% complete`)
```

### Workflow 2: Viewing Epic Status

```typescript
// List all epics with progress
const epics = await mcp__jility__list_epics()

// Find incomplete epics
const incomplete = epics.filter(e => e.progress.completion_percentage < 100)

// Report on progress
incomplete.forEach(epic => {
  console.log(`${epic.title}:`)
  console.log(`  Progress: ${epic.progress.done}/${epic.progress.total} (${epic.progress.completion_percentage}%)`)
  console.log(`  In Progress: ${epic.progress.in_progress}`)
  console.log(`  Blocked: ${epic.progress.blocked}`)
})
```

### Workflow 3: Agent Working on Epic Ticket

```typescript
// 1. Agent finds unassigned tickets in an epic
const tickets = await mcp__jility__list_tickets({
  epic_id: "uuid-of-epic",
  status: ["todo", "backlog"]
})

// 2. Agent claims a ticket
await mcp__jility__claim_ticket({
  ticket_id: tickets[0].id
})

// 3. Agent reads comments for context
const comments = await mcp__jility__get_comments({
  ticket_id: tickets[0].id
})

// 4. Agent works on the ticket...

// 5. Agent marks complete
await mcp__jility__update_status({
  ticket_id: tickets[0].id,
  status: "done"
})

// 6. Epic progress automatically updates!
```

---

## Tips and Best Practices

### Organization
- **Use epics for large features** - If a feature needs 3+ tickets, make it an epic
- **Choose meaningful colors** - Use colors to represent themes (e.g., green for payments, blue for auth, purple for admin)
- **Write clear epic descriptions** - Explain the scope and acceptance criteria
- **Keep epics focused** - Don't make them too broad (10-15 tickets max recommended)

### Progress Tracking
- **Check epic progress regularly** - Use `list_epics()` to monitor completion
- **Update ticket status promptly** - Epic progress auto-updates when ticket status changes
- **Use blocked status** - Mark tickets as blocked to identify bottlenecks in epic progress

### Navigation
- **Use epic filtering on board** - Focus on one epic at a time when working
- **Leverage epic detail pages** - View all epic work in one place
- **Click epic badges** - Quick way to filter board to specific epic

### AI Agent Workflows
- **Always check epic context** - Use `list_epics()` to understand project organization
- **Filter tickets by epic** - Focus agent work on completing one epic at a time
- **Batch create epic tickets** - Use `create_tickets_batch()` for efficiency
- **Track epic progress** - Report completion percentages to users

---

## Technical Details

### Database Schema

**tickets table additions:**
```sql
ALTER TABLE tickets ADD COLUMN is_epic BOOLEAN DEFAULT false;
ALTER TABLE tickets ADD COLUMN epic_color VARCHAR(50);
ALTER TABLE tickets ADD COLUMN parent_epic_id UUID REFERENCES tickets(id);
```

### API Endpoints

- `GET /api/epics` - List all epics with progress
- `GET /api/epics/:id` - Get epic details
- `GET /api/epics/:id/tickets` - Get tickets for an epic
- `POST /api/tickets` - Create ticket (or epic with `is_epic: true`)
- `DELETE /api/tickets/:id` - Soft delete epic or ticket

### MCP Tools

- `mcp__jility__create_epic(title, description, epic_color)` - Create epic
- `mcp__jility__list_epics()` - List epics with progress
- `mcp__jility__create_ticket(..., parent_epic_id)` - Create ticket in epic
- `mcp__jility__list_tickets(..., epic_id)` - Filter tickets by epic
- `mcp__jility__delete_ticket(ticket_id)` - Delete epic (orphans children)

---

## Troubleshooting

### "Cannot assign ticket to epic"
**Cause:** The epic ID doesn't exist or the target is not an epic.
**Solution:** Verify the epic exists and has `is_epic = true`.

### "Progress not updating"
**Cause:** Ticket status wasn't properly updated, or using old cached data.
**Solution:** Refresh the page or re-fetch epic data via API/MCP.

### "Epic shows in ticket list"
**Cause:** Epics are stored as special tickets in the database.
**Solution:** Filter by `is_epic = false` if you only want regular tickets.

### "Too many tickets in one epic"
**Recommendation:** If an epic has 20+ tickets, consider breaking it into multiple smaller epics.

---

## Future Enhancements

Potential improvements for epic support (not currently implemented):

- **Epic templates** - Pre-defined epic structures for common patterns
- **Epic nesting** - Allow sub-epics for large projects (currently not supported)
- **Epic milestones** - Track key checkpoints within an epic
- **Epic velocity** - Historical completion rate tracking
- **Epic burndown charts** - Visual progress tracking over time
- **Epic dependencies** - Mark epics that depend on other epics

---

## Related Documentation

- [API Reference](./api/API.md) - Complete API documentation including epic endpoints
- [MCP Tools](./.claude/CLAUDE.md#epic-management) - Full MCP tool reference
- [Foundation & Epic Sprint Design](./plans/2025-11-09-foundation-and-epic-sprint-design.md) - Original design document

---

**Last Updated:** November 2025
**Feature Status:** ✅ Complete and Production-Ready
