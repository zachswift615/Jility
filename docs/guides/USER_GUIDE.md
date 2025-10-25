# Jility Web UI User Guide

**Welcome to Jility!** This guide will help you get started with Jility's web interface and make the most of its features.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Dashboard Overview](#dashboard-overview)
3. [Creating & Managing Tickets](#creating--managing-tickets)
4. [Kanban Board](#kanban-board)
5. [Ticket Details](#ticket-details)
6. [Search & Filters](#search--filters)
7. [Sprint Management](#sprint-management)
8. [Profile & Settings](#profile--settings)
9. [Keyboard Shortcuts](#keyboard-shortcuts)
10. [Tips & Tricks](#tips--tricks)

---

## Getting Started

### First Time Setup

1. **Open Jility** in your web browser:
   ```
   http://localhost:3001
   ```
   (Or your production URL)

2. **Create an Account:**
   - Click **"Sign Up"** or navigate to `/register`
   - Enter your email address
   - Choose a username (this will be displayed on tickets)
   - Create a password (minimum 8 characters, must include at least one number)
   - Click **"Create Account"**

3. **You're In!**
   - You'll be automatically logged in
   - You'll see the main Kanban board

### Logging In

**If you already have an account:**

1. Navigate to `/login`
2. Enter your email and password
3. Click **"Sign In"**

**Forgot your password?**
- Contact your admin for a password reset (email-based reset coming soon!)

---

## Dashboard Overview

### Navigation Bar

Located at the top of every page:

```
┌─────────────────────────────────────────────────────┐
│ Jility  Board  Agents  Search  Planning  [⌘K] [👤] │
└─────────────────────────────────────────────────────┘
```

**Navigation Items:**
- **Jility** (logo) - Returns to home/board
- **Board** - Main kanban board view
- **Agents** - Agent activity dashboard
- **Search** (magnifying glass) - Advanced search
- **Sprint Planning** (calendar icon) - Plan sprints
- **Active Sprint** (activity icon) - Current sprint
- **Sprint History** (clock icon) - Past sprints
- **⌘K** - Command palette (click or press Cmd+K)
- **Theme Toggle** - Sun/moon/system icons
- **Profile** (avatar) - Your profile and settings

### Main Views

**Board View (`/board`):**
- Default view when you log in
- See all tickets organized by status
- Drag and drop to change status

**Search View (`/search`):**
- Advanced search with filters
- Save frequently-used searches
- View search results

**Sprint Views (`/sprint/...`):**
- **Planning** - Add tickets to sprint
- **Active** - Track current sprint progress
- **History** - Review past sprints

**Agents View (`/agents`):**
- See AI agent activity
- View agent statistics
- Recent agent actions

---

## Creating & Managing Tickets

### Creating a Ticket

**Method 1: Command Palette (⌘K)**

1. Press `Cmd+K` (Mac) or `Ctrl+K` (Windows/Linux)
2. Type "create" or "new ticket"
3. Click "Create Ticket"
4. Fill in the form (see below)

**Method 2: Plus Button**

1. Look for a **"+"** or **"New Ticket"** button on the board
2. Click it
3. Fill in the form

**Method 3: API/CLI**

- Use the REST API or CLI to create tickets programmatically

### Ticket Form Fields

When creating or editing a ticket:

**Required:**
- **Title** - Short, descriptive name (e.g., "Add user authentication")

**Optional:**
- **Description** - Full details in Markdown format
  - Use headers (`## Acceptance Criteria`)
  - Use checklists (`- [ ] Task item`)
  - Include code blocks with ` ```language `
  - Add links, images, etc.

- **Status** - Current state:
  - **Backlog** - Not yet planned
  - **Todo** - Ready to work on
  - **In Progress** - Currently being worked on
  - **Review** - Ready for review
  - **Done** - Completed
  - **Blocked** - Cannot proceed

- **Story Points** - Effort estimate (1-13, Fibonacci sequence)

- **Assignees** - Who's working on it
  - Can assign to humans or AI agents
  - Multiple assignees supported (pairing)

- **Labels** - Tags for categorization
  - Examples: "backend", "frontend", "bug", "feature"
  - Use for filtering and organization

### Quick Actions

On any ticket card, you can:
- **Click** to view full details
- **Drag** to change status (on Kanban board)
- **Click assignee avatar** to filter by that person

---

## Kanban Board

The Kanban board is your main workspace.

### Board Layout

```
┌────────────┬────────────┬────────────┬────────────┬────────────┬────────────┐
│  Backlog   │    Todo    │ In Progress│   Review   │    Done    │  Blocked   │
│            │            │            │            │            │            │
│ [TASK-123] │ [TASK-156] │ [TASK-167] │ [TASK-145] │ [TASK-134] │ [TASK-189] │
│ Auth fix   │ New API    │ Dashboard  │ Bug fix    │ Login page │ Deploy fix │
│ 5 pts      │ 8 pts      │ 13 pts     │ 3 pts      │ 5 pts      │ 2 pts      │
│ alice      │ agent-1    │ bob        │ alice      │ agent-2    │ unassigned │
│            │            │            │            │            │            │
│ [TASK-124] │ [TASK-157] │            │            │            │            │
│ ...        │ ...        │            │            │            │            │
└────────────┴────────────┴────────────┴────────────┴────────────┴────────────┘
```

### Using the Board

**Move Tickets:**
1. Click and hold a ticket card
2. Drag it to a different column
3. Release to drop
4. Status updates automatically
5. All team members see the change in real-time

**Filter the Board:**
- Use the filters at the top (if available)
- Filter by assignee, label, story points, etc.
- Clear filters to see all tickets

**Real-Time Updates:**
- When someone (human or agent) moves a ticket, you'll see it update live
- No need to refresh the page
- WebSocket magic! ✨

### Ticket Card Information

Each card shows:
```
┌──────────────────────────────┐
│ TASK-123                     │  ← Ticket number
│ Implement JWT auth           │  ← Title
│ ─────────────────────────── │
│ [backend] [feature]          │  ← Labels
│ [alice] [agent-1]            │  ← Assignees
│ 5 pts                        │  ← Story points
└──────────────────────────────┘
```

### Mobile View

On smaller screens:
- Columns scroll horizontally (swipe left/right)
- Tap a ticket to view details
- Use the menu to access filters

---

## Ticket Details

Click any ticket to see full details.

### Ticket Detail Page Layout

```
┌─────────────────────────────────────────┬───────────────────┐
│ TASK-123                                │ Status: Todo      │
│ Implement JWT authentication            │                   │
│                                         │ Story Points: 5   │
│ [Edit Title]                            │                   │
│                                         │ Assignees:        │
│ ──────────────────────────────────────  │ • alice           │
│                                         │ • agent-1         │
│ ## Description                          │                   │
│                                         │ Labels:           │
│ Add JWT-based authentication to the API.│ • backend         │
│                                         │ • feature         │
│ ## Acceptance Criteria                  │                   │
│                                         │ Created:          │
│ - [x] JWT generation                    │ 2 hours ago       │
│ - [ ] Token validation                  │ by alice          │
│ - [ ] Refresh tokens                    │                   │
│                                         │ Linked Commits:   │
│ ## Technical Notes                      │ • abc123f         │
│                                         │   Add JWT service │
│ Use `jsonwebtoken` crate...             │                   │
│                                         │ Dependencies:     │
│ ──────────────────────────────────────  │ • TASK-120 (done) │
│                                         │                   │
│ 💬 Comments (3)                         │ Blocks:           │
│                                         │ • TASK-125        │
│ alice • 1 hour ago                      │ • TASK-126        │
│ Started implementing this. JWT works!   │                   │
│                                         │                   │
│ agent-1 • 30 min ago                    │                   │
│ Added unit tests for token generation.  │                   │
│ @alice please review.                   │                   │
│                                         │                   │
│ [Add comment...]                        │                   │
│                                         │                   │
│ ──────────────────────────────────────  │                   │
│                                         │                   │
│ 📋 Activity                             │                   │
│                                         │                   │
│ • agent-1 added comment (30 min ago)    │                   │
│ • alice changed status todo→in_progress │                   │
│ • alice edited description (2 hrs ago)  │                   │
│ • alice created ticket (2 hrs ago)      │                   │
└─────────────────────────────────────────┴───────────────────┘
```

### Actions on Ticket Detail Page

**Edit Title:**
- Click the title to edit inline
- Press Enter to save, Escape to cancel

**Edit Description:**
- Click the "Edit" button (if available)
- Or use markdown editor
- Full markdown support with preview

**Change Status:**
- Click the status dropdown in the sidebar
- Select new status
- Saves automatically

**Add/Remove Assignees:**
- Click "+" next to Assignees
- Search for team member or agent
- Click to add/remove

**Add/Remove Labels:**
- Click "+" next to Labels
- Type to create new label or select existing
- Click to add/remove

**Add Comment:**
1. Scroll to Comments section
2. Type your comment (markdown supported)
3. Use `@username` to mention someone
4. Click "Comment" to post

**Link Git Commit:**
- Click "Link Commit"
- Enter commit hash
- Optional: paste commit message
- Commit appears in sidebar

**View History:**
- Click "History" or "Show versions"
- See all changes to the description
- View diffs between versions
- Revert to previous version if needed

---

## Search & Filters

### Quick Search

**Search Bar (Navigation):**
1. Click the search icon in navbar
2. Start typing
3. See suggestions appear (top 5 results)
4. Click a result to open that ticket
5. Click "View all results" for full search

**Debounced:** Waits 300ms after you stop typing before searching (prevents too many requests)

### Advanced Search

Navigate to `/search` for powerful filtering.

**Search Interface:**

```
┌─────────────────────────────────────────────────────┐
│ Search: "authentication"                      [🔍]  │
│                                                     │
│ Filters:                                            │
│ ┌─────────┬──────────┬─────────┬──────────┐        │
│ │ Status  │ Assignee │ Labels  │ Points   │        │
│ └─────────┴──────────┴─────────┴──────────┘        │
│                                                     │
│ Applied Filters:                                    │
│ [× todo] [× alice] [× backend]                     │
│                                                     │
│ [Save this search as a view]                        │
└─────────────────────────────────────────────────────┘

Results (42 found):

┌─────────────────────────────────────────────────────┐
│ TASK-123: Implement JWT authentication              │
│ ...add JWT-based <mark>authentication</mark> for    │
│ API endpoints with token validation...              │
│ 5 pts • alice • 2 hours ago                         │
└─────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────┐
│ TASK-156: Update auth middleware                    │
│ ...fix <mark>authentication</mark> issues in         │
│ protected routes...                                 │
│ 3 pts • agent-1 • 1 day ago                         │
└─────────────────────────────────────────────────────┘

[Load more...]
```

### Available Filters

**Status:**
- Select one or multiple statuses
- Example: Show only "todo" and "in_progress" tickets

**Assignees:**
- Filter by who's assigned
- Select multiple people
- Include agents

**Labels:**
- Filter by tags
- Multiple selection supported

**Story Points:**
- Min and max range
- Example: 3-8 points

**Dates:**
- Created after/before date
- Updated after/before date
- Use date picker

**Relations:**
- Has comments (yes/no)
- Has commits (yes/no)
- Has dependencies (yes/no)

**Hierarchy:**
- Filter by epic
- Filter by parent ticket
- See only root-level tickets

### Saved Views

**Save frequently-used searches:**

1. Apply filters you want
2. Click "Save as view"
3. Give it a name (e.g., "My Open Tasks")
4. Optionally set as default view
5. Click "Save"

**Use saved views:**

1. Click "Saved Views" in sidebar
2. Click a view name
3. Filters apply automatically

**Manage saved views:**
- Click the pencil icon to edit
- Click the star to set as default
- Click the trash to delete

---

## Sprint Management

Jility includes complete Agile sprint management.

### Sprint Planning

Navigate to `/sprint/planning`

**Layout:**

```
┌─────────────────────────────────────────────────────┐
│ Sprint 5                            [Start Sprint]  │
│ Goal: Complete authentication system                │
│ Jan 15 - Jan 29 (14 days)                          │
│                                                     │
│ Capacity: 45 pts    Planned: 38 pts    ✅ 84%     │
└─────────────────────────────────────────────────────┘

┌───────────────────────┬─────────────────────────────┐
│ Backlog               │ Sprint 5                    │
│                       │                             │
│ Click to add →        │ ← Click to remove           │
│                       │                             │
│ [TASK-123] 5 pts      │ [TASK-156] 8 pts           │
│ OAuth support         │ JWT authentication          │
│                       │                             │
│ [TASK-124] 3 pts      │ [TASK-157] 5 pts           │
│ Email templates       │ Password reset              │
└───────────────────────┴─────────────────────────────┘
```

**How to Plan a Sprint:**

1. **Create a Sprint:**
   - Click "New Sprint"
   - Enter name (e.g., "Sprint 5")
   - Set goal (what you want to achieve)
   - Set start and end dates
   - Click "Create"

2. **Add Tickets:**
   - Click tickets in the Backlog column
   - They move to the Sprint column
   - Watch the capacity indicator update

3. **Review Capacity:**
   - **Green (< 90%):** Good sprint size
   - **Yellow (90-100%):** At capacity
   - **Red (> 100%):** Overcommitted!

4. **Start the Sprint:**
   - Click "Start Sprint"
   - Sprint becomes active
   - Team can start working

### Active Sprint

Navigate to `/sprint/active`

**Features:**

```
┌─────────────────────────────────────────────────────┐
│ Sprint 5 - In Progress               [Complete]     │
│                                                     │
│ Goal: Complete authentication system                │
│ 7 days remaining (Jan 15 - Jan 29)                 │
│                                                     │
│ [████████████░░░░░] 68% complete (26/38 pts)       │
└─────────────────────────────────────────────────────┘

Burndown Chart:
 40│╲
 30│ ╲
 20│  ╲___
 10│      ╲___●●●
  0└───────────────
   Day 1  5   9  13

[Kanban board filtered to sprint tickets...]
```

**Tracking Progress:**

- **Progress Bar:** Visual completion percentage
- **Days Remaining:** Countdown to sprint end
- **Burndown Chart:**
  - Gray dashed line = ideal burndown
  - Blue solid line = actual progress
  - Should roughly follow the ideal line

**Complete the Sprint:**

1. When sprint ends, click "Complete Sprint"
2. Review what was completed
3. Incomplete tickets move to backlog (or next sprint)
4. Sprint moves to history

### Sprint History

Navigate to `/sprint/history`

**View past sprints:**

```
┌─────────────────────────────────────────────────────┐
│ Velocity Trend                                      │
│ 50│          ●                                      │
│ 40│       ●     ●                                   │
│ 30│    ●                                            │
│ 20│ ●                                               │
│  0└────────────────────                             │
│   S1  S2  S3  S4  S5                               │
│                                                     │
│ Average Velocity: 38 points/sprint                 │
└─────────────────────────────────────────────────────┘

Past Sprints:

Sprint 5 - Completed Jan 29, 2024
  ✅ 38/40 pts completed (95%)
  📊 12 tickets completed, 1 moved to backlog
  🎯 Goal: Authentication system

Sprint 4 - Completed Jan 15, 2024
  ✅ 42/45 pts completed (93%)
  📊 15 tickets completed, 2 moved to next sprint
  🎯 Goal: Database optimization
```

**Use velocity for planning:**
- Average velocity = typical sprint capacity
- Use this to plan future sprints
- Adjust team size or sprint length based on trends

---

## Profile & Settings

Click your avatar in the top-right to access profile.

### Profile Page

Navigate to `/profile`

**Sections:**

1. **Account Information:**
   - Email address
   - Username
   - Full name
   - Avatar URL

2. **API Keys:**
   - Create keys for programmatic access
   - Format: `jil_live_xxxxxxxxxxxx`
   - **Important:** Copy immediately! Only shown once.
   - Revoke keys you no longer need

3. **Active Sessions:**
   - See where you're logged in
   - IP addresses and user agents
   - Logout from other devices

4. **Change Password:**
   - Enter current password
   - Enter new password (8+ chars, one number)
   - Confirm new password
   - Click "Update Password"

### Managing API Keys

**Create an API Key:**

1. Go to Profile page
2. Scroll to "API Keys" section
3. Click "Create New API Key"
4. Enter a name (e.g., "CI/CD Pipeline")
5. Copy the key immediately (shown only once!)
6. Store securely (password manager recommended)

**Use an API Key:**

```bash
curl -X POST http://localhost:3000/api/tickets \
  -H "Authorization: ApiKey jil_live_xxxxxxxxxxxx" \
  -H "Content-Type: application/json" \
  -d '{"title":"New ticket from API"}'
```

**Revoke an API Key:**

1. Find the key in your list
2. Click "Revoke"
3. Confirm deletion
4. Key is immediately invalidated

---

## Keyboard Shortcuts

Jility is designed for keyboard-driven workflows.

### Global Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd+K` / `Ctrl+K` | Open command palette |
| `Esc` | Close modal/palette |
| `/` | Focus search |
| `G` then `B` | Go to board |
| `G` then `S` | Go to search |
| `G` then `P` | Go to sprint planning |
| `G` then `A` | Go to agents |
| `C` | Create new ticket (from board) |

### Command Palette (⌘K)

The command palette is your power tool:

1. Press `Cmd+K` or `Ctrl+K`
2. Start typing:
   - **"create"** - Create new ticket
   - **"search"** - Go to search
   - **"TASK-123"** - Jump to ticket by number
   - **"board"** - Go to board
   - **Keywords** - Find tickets by title

3. Use arrow keys to navigate
4. Press Enter to select
5. Press Esc to close

### Board Shortcuts

| Shortcut | Action |
|----------|--------|
| `Tab` | Navigate between columns |
| `Arrow keys` | Navigate tickets |
| `Enter` | Open selected ticket |
| `D` | Toggle filters |

### Ticket Detail Shortcuts

| Shortcut | Action |
|----------|--------|
| `E` | Edit description |
| `C` | Add comment |
| `Cmd+Enter` | Save comment |
| `Esc` | Cancel edit |

---

## Tips & Tricks

### Theme Switching

**Change the color theme:**

1. Look for sun/moon/monitor icons in navbar (top-right)
2. Click:
   - **Sun ☀️** - Light mode
   - **Moon 🌙** - Dark mode
   - **Monitor 🖥️** - System preference (follows your OS)

3. Your choice is saved and persists across sessions

**All components automatically update** when you switch themes!

### Markdown in Descriptions & Comments

Jility supports full Markdown:

**Headers:**
```markdown
# H1 Header
## H2 Header
### H3 Header
```

**Lists:**
```markdown
- Bullet point
- Another point
  - Nested point

1. Numbered list
2. Second item
```

**Checklists:**
```markdown
- [ ] Todo item
- [x] Completed item
```

**Code:**
```markdown
Inline `code` with backticks

```javascript
// Code block with syntax highlighting
function hello() {
  console.log("Hello!");
}
```
```

**Links & Images:**
```markdown
[Link text](https://example.com)
![Image alt](https://example.com/image.png)
```

**Emphasis:**
```markdown
*italic* or _italic_
**bold** or __bold__
~~strikethrough~~
```

### @Mentions in Comments

Mention team members in comments:

```markdown
@alice Can you review this?
@agent-1 Please implement the tests
```

(Currently visual only; notifications coming soon!)

### Working with AI Agents

**Agents appear just like human team members:**

- Assign tickets to agents (e.g., `agent-1`, `agent-2`)
- Agents appear in assignee lists
- See agent activity in the Agents dashboard
- Agent actions appear in activity timeline
- Pair human + agent on the same ticket

**Agent Dashboard shows:**
- Which agents are active
- What each agent is working on
- Recent agent actions
- Agent productivity metrics

### Real-Time Collaboration

Jility updates in real-time:

- **Someone moves a ticket:** You see it move instantly
- **Someone adds a comment:** It appears immediately
- **Agent completes a task:** Status updates live
- **No refresh needed:** WebSocket magic!

**"Presence" indicators coming soon:**
- See who's viewing a ticket
- See who's typing a comment
- Real-time cursors (future)

### Filtering Best Practices

**Create saved views for common workflows:**

- **"My Open Tasks":** `assignee:you status:todo,in_progress`
- **"Ready for Review":** `status:review`
- **"Blocked Items":** `status:blocked`
- **"This Sprint":** `sprint:current`
- **"Backend Work":** `label:backend`

Save these as views for one-click access!

### Mobile Usage

**Jility works on mobile devices:**

- Board scrolls horizontally (swipe)
- Tap tickets to view details
- Use hamburger menu for navigation
- Pull to refresh (coming soon)
- Touch-friendly drag and drop

**Best experience:** Use the command palette (`⌘K`) on mobile!

### Sprint Planning Tips

**Effective sprint planning:**

1. **Review velocity:** Look at past 3-5 sprints
2. **Set capacity:** Team size × days × 3 points/day (default)
3. **Prioritize:** Add highest-priority tickets first
4. **Leave buffer:** Don't plan to 100% capacity (aim for 80-90%)
5. **Include variety:** Mix high and low point tickets
6. **Consider dependencies:** Add prerequisite tickets first

**During sprint:**
- Update tickets daily
- Move to "Done" as you complete
- Watch the burndown chart
- Adjust if needed (add/remove tickets)

### Productivity Hacks

**Keyboard-driven workflow:**

1. Press `⌘K` to open palette
2. Type ticket number or keyword
3. Press Enter to open
4. Use `E` to edit, `C` to comment
5. Press `Esc` to close
6. Back to board with `G` + `B`

**Batch operations (coming soon):**
- Select multiple tickets
- Bulk assign
- Bulk status change
- Bulk label add/remove

### Searching Pro Tips

**Query syntax (basic):**
- `auth` - Match "auth" anywhere
- `"exact phrase"` - Match exact phrase
- `auth AND jwt` - Both terms required
- `auth OR oauth` - Either term
- `auth NOT password` - Exclude term

**Filters in search:**
- `status:todo` - Specific status
- `assignee:alice` - Specific person
- `label:backend` - Has label
- `points:5` - Exact story points
- `points:>5` - Greater than 5 points
- `created:>2024-01-01` - After date

**Combine search + filters for powerful queries!**

---

## Getting Help

### Common Issues

**Can't find a ticket:**
- Use search (⌘K or /search)
- Check filters aren't hiding it
- Try "All Tickets" view

**Changes not saving:**
- Check your internet connection
- Look for error messages
- Try refreshing the page

**Agent not working:**
- Agents need API keys
- Check MCP server is running
- See agent documentation

**Lost your API key:**
- Can't recover! (security feature)
- Revoke the old key
- Create a new one

### Best Practices

**Ticket Writing:**
- Clear, descriptive titles
- Use markdown for structure
- Add acceptance criteria
- Include technical details
- Link related tickets

**Sprint Management:**
- Plan realistically
- Update tickets daily
- Review retrospectives
- Adjust velocity over time

**Team Collaboration:**
- Comment on tickets
- Use @mentions
- Pair on complex tasks
- Review each other's work

### Support & Feedback

**Need help?**
- Check this user guide
- See technical documentation
- Ask your team admin
- Submit GitHub issues

**Found a bug?**
- Note what you were doing
- Check browser console (F12)
- Report to your admin
- Include screenshots if possible

---

## Appendix: Features by Role

### For Developers

- Markdown support with code blocks
- Git commit linking
- API keys for automation
- CLI for terminal lovers
- WebSocket for real-time updates
- REST API for integrations

### For Product Managers

- Sprint planning and tracking
- Burndown charts
- Velocity metrics
- Roadmap view (via epics)
- Search and filters
- Saved views

### For Team Leads

- Assign work to team members
- Track progress in real-time
- Review team velocity
- Manage sprints
- Monitor bottlenecks
- Agent productivity dashboard

### For AI Agents

- MCP server integration
- API key authentication
- Precise description editing
- Context bundling
- Template system
- Automated workflows

---

## What's Next?

**Upcoming features:**
- Email notifications
- Batch operations
- Advanced git integration
- Custom workflows
- Time tracking
- Reports and analytics
- Mobile apps
- More themes

**Stay tuned!** 🚀

---

**Happy project managing with Jility!**

Built with ❤️ for human-agent collaboration.

---

*Last updated: October 24, 2024*
*Version: 1.0*
