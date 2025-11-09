# Foundation & Epic Sprint - Design Document

**Date:** 2025-11-09
**Sprint Goal:** Make Jility solid and usable - foundation cleanup, epic support, and UI polish
**Implementation:** AI agents (primarily), ship as ready (quality over speed)
**Target Audience:** Solo developers using Claude Code to manage their product backlog

---

## ✅ SPRINT COMPLETED - November 2025

**Completion Date:** November 9, 2025
**Total Commits:** 20 commits
**Status:** All planned features delivered and tested

### Summary of Delivered Features

**Slice 1: Foundation Cleanup ✅**
- MCP `get_comments` tool for reading ticket comment threads
- Soft delete functionality (database, API, MCP, UI)
- Fixed activity attribution to display usernames instead of "system"
- Removed unused Agents tab from navigation

**Slice 2: Epic Support - Backend ✅**
- Database schema: Added `is_epic`, `epic_color`, `parent_epic_id` columns to tickets table
- Epic API endpoints: `GET /api/epics`, `GET /api/epics/:id`, `GET /api/epics/:id/tickets`
- Epic progress calculation (total/done/in_progress/todo/blocked counts, completion percentage)
- MCP tools: `create_epic`, `list_epics`
- Updated MCP tools to support epic filtering

**Slice 3: Epic Support - Frontend ✅**
- Epic board view (`/epics`) with card-based grid layout
- Epic detail page with progress visualization and filtered kanban board
- Epic color badges on ticket cards
- Epic filtering on main board view
- "Epics" tab added to main navigation
- Create epic functionality with color picker

**Slice 4: UI/UX Polish ✅**
- Consolidated settings into single `/settings` page with tabs (Profile, Workspace, Projects, API Keys)
- Removed duplicate `/project` route (redirects to `/settings?tab=projects`)
- Fixed Quick Add in backlog view - now creates tickets successfully
- Fixed Quick Add button at top of backlog - opens working inline form

### Integration Testing Results
All features tested end-to-end:
- ✅ Epic creation (UI, MCP, API)
- ✅ Ticket assignment to epics
- ✅ Epic progress calculation accuracy
- ✅ Epic filtering on board
- ✅ Soft delete functionality
- ✅ Comment retrieval via MCP
- ✅ Settings consolidation
- ✅ Quick Add functionality in backlog

### Known Issues
None - all discovered issues were fixed during implementation.

### Commit Log
```
f428730 fix: Quick Add button at top of backlog opens working form
d31b6f7 fix: Quick Add in backlog view creates tickets successfully
4b3488c feat: unify settings pages with tabbed interface
423e056 feat: add epic filtering to board view
858f8de feat: add epic detail page with progress visualization and kanban board
af33611 feat: add epic-card component and epics list page
31733e7 feat: add epic management MCP tools
dd55916 feat(backend): Add epic support API endpoints and validation
fdda784 feat: add database schema support for epics
7ad8a6e feat: remove unused Agents page and navigation link
1b8905c fix: display usernames instead of user IDs in activity log
16002f0 fix(api): add auth headers to deleteTicket endpoint
c288ec4 feat: add delete button with confirmation to ticket detail view
461158f feat: add delete_ticket MCP tool for soft-deleting tickets
7355f76 fix(core): add ChangeType::Deleted enum variant
1b7fbc5 feat(JIL-28): Add soft delete for tickets
5861adc fix(api): support ticket numbers in list_comments endpoint
d880882 feat(mcp): add get_comments tool for reading ticket comments
0530f0b chore: remove unused jility-cli crate
758404e Add Foundation & Epic Sprint design document
```

### Documentation Updated
- ✅ `.claude/CLAUDE.md` - Added new MCP tools documentation
- ✅ `README.md` - Updated roadmap and key features
- ✅ `docs/api/API.md` - Added epic endpoints documentation
- ✅ `docs/EPIC_SUPPORT.md` - Created comprehensive epic feature guide

---

## Executive Summary

This sprint focuses on **foundation and polish** rather than AI magic features. The goal is to prove the solo dev UX by making Jility rock-solid, organized with epics, and fixing UI paper cuts.

**Key Deliverables:**
1. Foundation cleanup (MCP comments, delete tickets, attribution, remove unused Agents tab)
2. Epic support with Standard JIRA-like features (board, progress, filtering)
3. UI/UX polish (consolidate settings, fix Quick Add buttons)

**Approach:** Vertical slices - complete features end-to-end before moving to the next.

---

## Sprint Structure

### Vertical Slices (In Order)

1. **Slice 1: Foundation Cleanup** (~2-3 days)
2. **Slice 2: Epic Support - Backend** (~3-4 days)
3. **Slice 3: Epic Support - Frontend** (~3-4 days)
4. **Slice 4: UI/UX Polish** (~2-3 days)

**Total:** ~10-14 days (ship as ready)

---

## Slice 1: Foundation Cleanup

### Overview
Polish the basics and remove unused features. Make MCP more powerful, enable ticket deletion, fix attribution bugs, and declutter navigation.

### Features

#### 1. JIL-29: MCP Tool to Read Comments
**Why:** Agents need context from human discussions before working on tickets.

**Implementation:**
- Add new MCP tool: `mcp__jility__get_comments(ticket_id)`
- Returns: Array of comments with `{ author, timestamp, content }`
- Backend: API endpoint already exists (`GET /api/tickets/:id/comments`)
- Just expose via MCP server

**Estimated Effort:** ~1 hour

#### 2. JIL-28: Delete Ticket Functionality
**Why:** Clean up test tickets, mistakes, duplicates.

**Implementation:**
- **Database:** Add `deleted_at` timestamp column (soft delete for audit trail)
- **API:** `DELETE /api/tickets/:id` endpoint with soft delete
- **MCP:** `mcp__jility__delete_ticket(ticket_id)` tool
- **UI:** Delete button in ticket detail view with confirmation dialog

**Soft Delete Behavior:**
- Deleted tickets don't appear in lists/boards
- Can optionally show in "Deleted" view for recovery
- Preserves audit trail and relationships

**Estimated Effort:** ~4-6 hours

#### 3. JIL-30: Fix Activity Attribution
**Problem:** Activity log shows "system" instead of actual username.

**Implementation:**
- Update activity log queries to join with users table
- Fix locations: Ticket detail view, activity feed
- Expected output: "Jane Doe added a comment" instead of "system added a comment"

**Estimated Effort:** ~2 hours

#### 4. Remove Agents Tab
**Why:** Not hooked up, takes up navigation space, no current use case.

**Implementation:**
- Delete: `jility-web/app/agents/page.tsx` (if exists)
- Update: Navigation component to remove Agents link
- Clean up: Any agent-specific routes or components

**Estimated Effort:** ~1 hour

### Success Criteria for Slice 1
- ✅ Agent can read comments via MCP before working on ticket
- ✅ Can delete tickets from UI, MCP, and API
- ✅ Activity log shows correct user attribution
- ✅ Agents tab no longer appears in navigation

---

## Slice 2: Epic Support - Backend

### Overview
Add database schema, API endpoints, and MCP tools to support epic hierarchy and progress tracking.

### Database Schema Changes

**Add to `tickets` table:**
```sql
ALTER TABLE tickets ADD COLUMN is_epic BOOLEAN DEFAULT false;
ALTER TABLE tickets ADD COLUMN epic_color VARCHAR(50);
ALTER TABLE tickets ADD COLUMN parent_epic_id UUID REFERENCES tickets(id);
```

**Migration considerations:**
- Existing tickets default to `is_epic = false`
- `epic_color` is optional (nullable)
- `parent_epic_id` nullable (not all tickets belong to epics)

### Epic Hierarchy Rules

1. **Epics can have child tickets** (regular tasks)
2. **Regular tickets can belong to one epic** (via `parent_epic_id`)
3. **Epics cannot be nested** (no epic-of-epic - keep it simple)
4. **Validation:** Prevent setting `parent_epic_id` to a non-epic ticket

### API Enhancements

**Existing Endpoints:**
- `POST /api/tickets` - Accept new fields: `is_epic`, `epic_color`
- `GET /api/tickets` - Add `epic_id` filter parameter

**New Endpoints:**
- `GET /api/epics` - List only epics with progress stats
- `GET /api/epics/:id/tickets` - Get all tickets for an epic with status breakdown

**Epic Response Format:**
```json
{
  "id": "uuid",
  "title": "User Authentication",
  "description": "...",
  "is_epic": true,
  "epic_color": "#3b82f6",
  "progress": {
    "total": 10,
    "done": 3,
    "in_progress": 2,
    "todo": 5,
    "blocked": 0,
    "completion_percentage": 30
  },
  "created_at": "...",
  "updated_at": "..."
}
```

### Epic Progress Calculation

**Logic:**
1. Count child tickets by status: `todo`, `in_progress`, `done`, etc.
2. Calculate completion percentage: `done / total * 100`
3. Return with epic data

**Implementation:**
- SQL query joins tickets to count by status
- Calculate in backend (not frontend) for consistency
- Cache if performance becomes an issue

### MCP Tools

**New Tools:**
- `mcp__jility__create_epic(title, description, color)` - Create epic and return ID
- `mcp__jility__list_epics()` - List all epics with progress

**Updated Tools:**
- `mcp__jility__create_ticket()` - Accept `parent_epic_id` parameter
- `mcp__jility__list_tickets()` - Accept `epic_id` filter parameter

### Success Criteria for Slice 2
- ✅ Can create epics via API and MCP
- ✅ Can assign tickets to epics
- ✅ Epic progress calculates correctly
- ✅ Can filter tickets by epic
- ✅ Migrations run without breaking existing data

---

## Slice 3: Epic Support - Frontend

### Overview
Build UI for viewing, creating, and managing epics with Standard JIRA-like features.

### New Epic Board View (`/epics`)

**Layout:** Card-based grid showing all epics

**Epic Card Contents:**
- Epic title and description (truncated if long)
- Epic color indicator (left border or colored badge)
- Progress bar: "X/Y tasks completed" (visual progress bar)
- Status breakdown: "3 done, 2 in progress, 5 todo"
- Click card → navigate to epic detail view

**Empty State:**
"No epics yet. Create your first epic to organize work."

**UI Components:**
- Use CSS Grid or Flexbox for responsive layout
- Cards should be clickable and hoverable
- Progress bar uses theme colors

### Epic Detail View (`/epics/:id`)

**Sections:**
1. **Epic Header**
   - Title (editable inline)
   - Description (editable, markdown support)
   - Epic color badge/indicator

2. **Progress Visualization**
   - Large progress bar with percentage
   - Stats breakdown: "3 of 10 tasks completed"
   - Status distribution (todo, in progress, done, blocked)

3. **Epic Kanban Board**
   - Filtered to show only this epic's tickets
   - Same board component as main board, just filtered
   - "Add Task" button → creates ticket with `parent_epic_id` pre-filled

4. **Edit Epic**
   - Inline editing for title/description
   - Color picker for epic color
   - Delete epic button (with confirmation)

### Enhanced Ticket Creation

**Form Updates:**
- Add "Epic" dropdown field (list of available epics)
- Checkbox: "This is an epic" → shows/hides epic-specific fields
- Epic-specific fields:
  - Color picker for epic color (default: blue)
- When creating from epic detail view: auto-populate `parent_epic_id`

**Validation:**
- Can't create epic with parent epic (no nesting)
- Epic color is optional (default to theme primary color)

### Board View Enhancements

**Epic Filtering:**
- Add epic filter dropdown at top of board
- Options: "All tickets" | "No epic" | [list of epic names]
- Filter persists in URL query params

**Visual Indicators:**
- Show epic badge/color on ticket cards that belong to an epic
- Badge shows epic name (truncated if long)
- Click badge → quick filter to that epic

### Navigation Updates

**Add "Epics" to Main Nav:**
- Position: Between "Board" and "Backlog"
- Icon: `Layers` or `FolderKanban` from Lucide React
- Mobile: Show in bottom nav or drawer

### Success Criteria for Slice 3
- ✅ Can view all epics in grid/card layout
- ✅ Epic cards show accurate progress
- ✅ Can create epics from UI with color selection
- ✅ Can assign tickets to epics during creation
- ✅ Can filter board by epic
- ✅ Visual indicators show which tickets belong to which epic
- ✅ Epic detail view shows filtered kanban board

---

## Slice 4: UI/UX Polish

### Overview
Fix UI paper cuts, consolidate settings pages, and make Quick Add buttons work.

### 1. Combine Duplicate Settings Pages

**Problem:** Multiple settings pages exist that should be unified.

**Solution:** Single `/settings` page with tabbed interface

**Tab Structure:**
- **Profile** tab - User settings (name, email, password)
- **Workspace** tab - Workspace settings (sprint capacity, defaults)
- **Projects** tab - Project management (moved from `/project` page)
- **API Keys** tab - API key management (if currently separate)

**Implementation:**
- Use shadcn/ui `Tabs` component for consistent UI
- Preserve all existing functionality, just reorganize layout
- Tab selection persists in URL: `/settings?tab=projects`
- Default to first tab if no query param

**UI Components:**
```tsx
<Tabs defaultValue="profile">
  <TabsList>
    <TabsTrigger value="profile">Profile</TabsTrigger>
    <TabsTrigger value="workspace">Workspace</TabsTrigger>
    <TabsTrigger value="projects">Projects</TabsTrigger>
    <TabsTrigger value="api-keys">API Keys</TabsTrigger>
  </TabsList>
  <TabsContent value="profile">{/* Profile settings */}</TabsContent>
  {/* ... */}
</Tabs>
```

### 2. Move /project Page to Settings

**Changes:**
- Delete standalone `/project` route
- Create "Projects" tab in unified `/settings` page
- Move project list, create project, manage project UI
- Update navigation to remove separate "Projects" link
- Redirect `/project` → `/settings?tab=projects` (for bookmarks)

**Considerations:**
- Projects tab shows: Project list, create new project, switch project
- Same functionality as current `/project` page, just relocated

### 3. Fix Quick Add Ticket in Backlog View

**Problem:** Quick Add ticket form in backlog doesn't work.

**Investigation Required:**
- Identify the bug: Form submission? API call? State update?
- Check: Is the form rendering? Does submit trigger? Does API respond?

**Expected Fix:**
- Ensure ticket creation works from backlog inline form
- New ticket appears in backlog immediately (optimistic UI update)
- Form clears after successful creation
- Error handling shows validation errors

**Testing:**
- Create ticket with Quick Add in backlog
- Verify ticket appears without page refresh
- Verify ticket has correct default status ("backlog")

### 4. Fix Quick Add Button (Top of Backlog)

**Problem:** "Quick Add" button at top of backlog page doesn't work.

**Expected Behavior:**
- Click button → Opens inline ticket creation form (or modal)
- Fill title/description → Submit → Ticket created

**UX Decision:** Inline form vs modal?
- **Recommendation:** Inline form (better for "quick" add, less disruptive)
- Form appears at top of backlog list
- Focus moves to title input automatically

**Implementation:**
- Wire up button click handler
- Toggle form visibility state
- Use same ticket creation logic as working forms elsewhere
- Auto-focus title input when form opens

### Success Criteria for Slice 4
- ✅ Single `/settings` page with all settings organized in tabs
- ✅ No duplicate settings pages exist
- ✅ `/project` route redirects to `/settings?tab=projects`
- ✅ Navigation updated (no separate Projects link)
- ✅ Quick Add in backlog view creates tickets successfully
- ✅ Quick Add button at top of backlog opens working form
- ✅ All existing settings functionality preserved
- ✅ Tab navigation works with browser back/forward

---

## Overall Success Metrics

**After this sprint, Jility should:**
1. ✅ Have solid foundation (MCP complete, delete works, attribution correct)
2. ✅ Support epic-based organization (JIRA-like experience)
3. ✅ Have clean, consolidated UI (settings unified, Quick Add works)
4. ✅ Feel professional and polished (no broken features, clear navigation)
5. ✅ Be ready for solo dev dogfooding (prove the UX hypothesis)

**What this sprint is NOT:**
- ❌ AI epic breakdown (removed - not valuable for solo devs chatting with Claude)
- ❌ Advanced AI features (Phase 5 from original roadmap)
- ❌ Team collaboration features (focus is solo dev UX)

---

## Technical Notes

### Tech Stack
- **Backend:** Rust (Axum), SQLite/Postgres
- **Frontend:** Next.js 14, Tailwind CSS, shadcn/ui
- **MCP:** Anthropic Model Context Protocol
- **Icons:** Lucide React

### Testing Approach
- Manual testing by implementing AI agent
- Each slice should be fully tested before moving to next
- Use Playwright for UI testing if time permits
- MCP tools should be tested via Claude Code

### Deployment Strategy
- Ship as ready (no hard deadline)
- Each slice can be deployed independently
- Database migrations must be backwards compatible
- Feature flags not needed (no rollout risk)

---

## Open Questions / Future Considerations

1. **Epic deletion:** What happens to child tickets? (Orphan them? Block deletion? Future feature)
2. **Epic templates:** Should we add common epic templates (Auth, CRUD, etc.)? Not in this sprint, but consider for later.
3. **Epic nesting:** Might be needed for large projects, but explicitly excluding for MVP
4. **Quick Add UX:** Should we standardize Quick Add across all views (Board, Backlog, Epic detail)?

---

## Related Tickets

- JIL-29: Add MCP tool to read ticket comments
- JIL-28: Delete ticket functionality
- JIL-30: Fix activity log attribution
- JIL-27: Add Epic Support (JIRA-like)
- (New tickets will be created for settings consolidation and Quick Add fixes)

---

## Appendix: Decisions Made During Brainstorming

**Decision 1: Removed AI Epic Breakdown**
- **Reasoning:** If we're using MCP, developers can just chat with Claude directly to create epics. No need for separate AI breakdown feature.
- **Alternative considered:** Backend service with Claude API calls - Too complex, doesn't add value over MCP workflow

**Decision 2: Vertical Slices Over Category Batches**
- **Reasoning:** Better for AI agent implementation - complete features end-to-end, ship incrementally
- **Alternative considered:** Category batches (all foundation, then all epics) - Less flexible, delayed feedback

**Decision 3: Remove Agents Tab Entirely**
- **Reasoning:** Not hooked up, no current use case, clutters navigation
- **Alternative considered:** Make it functional - Too much scope for this sprint, not core to solo dev UX

**Decision 4: Standard JIRA-like Epic Features**
- **Reasoning:** Users understand this pattern, proven UX, balances simplicity with power
- **Alternative considered:** Minimal (just hierarchy) - Too basic, wouldn't feel complete
- **Alternative considered:** Enhanced (AI-first) - Too much scope, conflicts with removing AI breakdown

---

**End of Design Document**
