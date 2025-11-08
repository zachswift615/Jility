#!/usr/bin/env tsx

/**
 * Create Jility tickets from the feature roadmap
 *
 * Usage:
 *   export JILITY_API_URL=http://localhost:3000/api
 *   export JILITY_TOKEN=your-api-token
 *   export JILITY_PROJECT_ID=your-project-id
 *   npx tsx create-roadmap-tickets.ts
 */

const API_BASE = process.env.JILITY_API_URL || 'http://localhost:3900/api'
const API_TOKEN = process.env.JILITY_TOKEN
const PROJECT_ID = process.env.JILITY_PROJECT_ID

if (!API_TOKEN || !PROJECT_ID) {
  console.error('Error: Missing required environment variables')
  console.error('Please set: JILITY_TOKEN and JILITY_PROJECT_ID')
  process.exit(1)
}

interface CreateTicketRequest {
  project_id: string
  title: string
  description: string
  story_points?: number
  status?: string
  labels?: string[]
  parent_id?: string
  epic_id?: string
}

async function createTicket(data: CreateTicketRequest): Promise<any> {
  const response = await fetch(`${API_BASE}/tickets`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `ApiKey ${API_TOKEN}`,  // Use "ApiKey" prefix, not "Bearer"
    },
    body: JSON.stringify(data),
  })

  if (!response.ok) {
    const error = await response.text()
    throw new Error(`Failed to create ticket: ${response.status} ${error}`)
  }

  return response.json()
}

// Define roadmap tickets
const roadmapTickets = [
  // Phase 1: Core Collaboration
  {
    title: 'Phase 1: Core Collaboration',
    description: `## Overview
Enable team collaboration through comments and sprint planning.

## Goals
- Quick wins with backend-ready features
- Enable async collaboration
- Foundation for agile workflow

## Features
- Comments System (1.1)
- Sprint Planning (via sub-tickets)

## Timeline
Weeks 1-2

## Success Metrics
- 90%+ of tickets have comments
- Sprints created for next 3 iterations
- Average 5+ comments per ticket`,
    labels: ['epic', 'phase-1', 'collaboration'],
    status: 'todo',
  },

  {
    title: '1.1 Comments System',
    description: `## Status
🟢 Backend Complete - Quick Win!

## Why This First?
- Backend table & API ready
- Enables async collaboration
- Table stakes for team adoption
- 2-3 day effort

## Backend (Already Complete ✅)
- Table: \`comments\` (id, ticket_id, author, content, created_at, updated_at)
- API: POST/GET comments
- WebSocket: Real-time updates

## Frontend Work
### Components
1. **CommentItem** - Individual comment display with edit/delete
2. **CommentsSection** - Comment list + new comment form
3. API integration - Wire up to backend

### Features
- Create/edit/delete comments
- Real-time updates via WebSocket
- Markdown support
- Author validation
- Optimistic UI updates

## Testing Checklist
- [ ] Create comment on ticket
- [ ] Edit own comment
- [ ] Delete own comment
- [ ] Cannot edit/delete other users' comments
- [ ] Markdown renders correctly
- [ ] Mobile responsive

## Nice-to-Have
- Markdown preview
- @mentions autocomplete
- Reactions (👍, ❤️)
- Comment notifications`,
    labels: ['feature', 'frontend', 'phase-1'],
    status: 'todo',
    story_points: 3,
  },

  {
    title: '2.1 Fix Sprint Planning Page',
    description: `## Status
🟡 Partial - Page exists but broken

## Why This Matters?
- Managers need sprint planning
- Unlocks burndown charts
- Backend infrastructure ready
- Enables agile workflow

## Backend (Ready ✅)
- Tables: \`sprints\`, \`sprint_tickets\`
- Entities: Sprint, SprintTicket
- API: Needs audit

## Tasks
1. **Audit Backend API** (1 hour)
   - Verify all sprint CRUD endpoints
   - Check sprint-ticket association endpoints
   - Test start/complete sprint actions

2. **Create Frontend Models** (30 min)
   - Sprint interface
   - SprintDetails interface
   - API client functions

3. **Build Sprint Dialog** (2 hours)
   - Create/edit sprint form
   - Date picker for start/end
   - Sprint goal field
   - Validation

4. **Sprint List Page** (3 hours)
   - Active/Planning/Completed sections
   - Sprint cards with actions
   - Create sprint button
   - Navigate to detail view

5. **Sprint Detail Page** (4 hours)
   - Display sprint info
   - List tickets in sprint
   - Add/remove tickets
   - Show capacity (story points)
   - Progress bar
   - Start/Complete actions

6. **Ticket Integration** (2 hours)
   - Add "Add to Sprint" dropdown
   - Sprint indicator on board
   - Remove from sprint action

## Testing Checklist
- [ ] Create new sprint
- [ ] Edit sprint details
- [ ] Add tickets to sprint
- [ ] Remove tickets from sprint
- [ ] Start sprint
- [ ] Complete sprint
- [ ] View sprint history`,
    labels: ['feature', 'backend', 'frontend', 'phase-2'],
    status: 'backlog',
    story_points: 8,
  },

  // Phase 3: Visual Workflows
  {
    title: 'Phase 3: Visual Workflows',
    description: `## Overview
Improve visualization with swimlanes and burndown charts.

## Features
- Swimlanes for Board (3.1)
- Burndown Chart (3.2)

## Timeline
Week 4

## Dependencies
- Sprint Planning must be complete for burndown charts

## Success Metrics
- 60%+ users use swimlanes feature
- Burndown charts viewed daily by managers
- Sprint velocity stabilizes after 3 sprints`,
    labels: ['epic', 'phase-3', 'visualization'],
    status: 'backlog',
  },

  {
    title: '3.1 Swimlanes for Board',
    description: `## Status
🔴 New Build

## What Are Swimlanes?
Group tickets by assignee, epic, or label on the board view.

## Features
- Group by: Assignee, Epic, Label, Priority
- Collapsible swimlanes
- Ticket counts per swimlane
- Drag-and-drop between swimlanes
- Persist user preference

## Implementation
1. **Grouping Logic** (2 hours)
   - Group tickets by selected field
   - Sort groups intelligently
   - Handle "Unassigned" group

2. **UI Components** (3 hours)
   - Swimlane header with toggle
   - Collapsible sections
   - Ticket count badges
   - Smooth animations

3. **Drag & Drop** (2 hours)
   - Update existing DnD to work with groups
   - Visual feedback
   - Persist changes

4. **Settings** (1 hour)
   - Group by dropdown
   - Save preference to local storage
   - Apply on page load

## Testing
- [ ] Group by assignee
- [ ] Group by epic
- [ ] Group by label
- [ ] Drag ticket within swimlane
- [ ] Drag ticket between swimlanes
- [ ] Collapse/expand swimlanes
- [ ] Mobile responsive`,
    labels: ['feature', 'frontend', 'phase-3', 'board'],
    status: 'backlog',
    story_points: 5,
  },

  {
    title: '3.2 Burndown Chart',
    description: `## Status
🔴 New Build

## Dependencies
⚠️ Requires Sprint Planning (2.1) to be complete

## What Is It?
Visual chart showing work remaining vs. time in a sprint.

## Data Required
- Sprint start/end dates
- Daily snapshots of remaining story points
- Completed vs. remaining work

## Implementation
1. **Data Collection** (2 hours)
   - Create \`sprint_burndown\` table
   - Daily cron job to snapshot progress
   - Calculate remaining points

2. **Chart Component** (3 hours)
   - Use Chart.js or Recharts
   - Ideal line (straight diagonal)
   - Actual line (daily progress)
   - Scope changes indicator
   - Responsive design

3. **Sprint Detail Integration** (1 hour)
   - Add chart to sprint detail page
   - Show key metrics (velocity, completion %)
   - Export chart as image

## Testing
- [ ] Chart shows on active sprint
- [ ] Ideal line calculates correctly
- [ ] Actual progress tracks daily
- [ ] Handles scope changes
- [ ] Mobile responsive`,
    labels: ['feature', 'backend', 'frontend', 'phase-3', 'charts'],
    status: 'backlog',
    story_points: 5,
  },

  // Phase 4: Search & Discovery
  {
    title: 'Phase 4: Search & Discovery',
    description: `## Overview
Enable users to find tickets quickly with global search and advanced filters.

## Features
- Global Search (4.1)
- Board Filters (4.2)

## Timeline
Week 5

## Success Metrics
- Average search time < 5 seconds
- 70%+ of searches find target ticket
- Custom filters saved by power users`,
    labels: ['epic', 'phase-4', 'search'],
    status: 'backlog',
  },

  {
    title: '4.1 Global Search',
    description: `## Status
🟡 Partial - Basic search exists, needs enhancement

## Current State
- Simple text search on title
- No full-text indexing
- Limited to 50 results

## Improvements Needed
1. **Full-Text Search** (3 hours)
   - SQLite FTS5 virtual table
   - Index: title, description, comments
   - Relevance ranking
   - Highlight matches

2. **Advanced Filters** (2 hours)
   - Status multi-select
   - Assignee filter
   - Label filter
   - Date range (created/updated)
   - Story points range

3. **Search UI** (2 hours)
   - Keyboard shortcut (Cmd+K)
   - Search dialog with filters
   - Recent searches
   - Search history
   - Clear filters button

4. **Performance** (2 hours)
   - Debounce search input
   - Pagination (25 results per page)
   - Virtual scrolling for long lists
   - Loading states

## Testing
- [ ] Search by title
- [ ] Search in description
- [ ] Search in comments
- [ ] Filter by status
- [ ] Filter by assignee
- [ ] Combine multiple filters
- [ ] Performance with 1000+ tickets`,
    labels: ['feature', 'backend', 'frontend', 'phase-4'],
    status: 'backlog',
    story_points: 8,
  },

  {
    title: '4.2 Board Filters',
    description: `## Status
🔴 New Build

## What Are Board Filters?
Quick filters on the board view to focus on specific tickets.

## Features
- Filter by assignee (me, specific user, unassigned)
- Filter by label (multi-select)
- Filter by epic
- Filter by story points range
- "My tickets" quick filter
- Save custom filter presets

## Implementation
1. **Filter UI** (2 hours)
   - Filter toolbar above board
   - Multi-select dropdowns
   - Active filters badges
   - Clear all button

2. **Filter Logic** (2 hours)
   - Client-side filtering for speed
   - Combine multiple filters (AND logic)
   - Update ticket counts
   - Maintain across page refresh

3. **Saved Filters** (2 hours)
   - Save filter preset
   - Name custom filters
   - Quick access dropdown
   - Delete saved filters

## Testing
- [ ] Filter by assignee
- [ ] Filter by label
- [ ] Combine filters
- [ ] Save custom filter
- [ ] Load saved filter
- [ ] Clear filters
- [ ] Performance with large boards`,
    labels: ['feature', 'frontend', 'phase-4', 'board'],
    status: 'backlog',
    story_points: 5,
  },

  // Phase 5: AI/Agent Features
  {
    title: 'Phase 5: AI/Agent Features',
    description: `## Overview
Leverage AI for enhanced MCP server, epic breakdown, and Git integration.

## Features
- Enhanced MCP Server (5.1)
- AI Epic Breakdown (5.2)
- Smart Git Integration (5.3)

## Timeline
Weeks 6-7

## Success Metrics
- 40%+ of epics use AI breakdown
- AI suggestions accepted 80%+ of time
- Git auto-linking catches 90%+ of commits`,
    labels: ['epic', 'phase-5', 'ai', 'agent'],
    status: 'backlog',
  },

  {
    title: '5.1 Enhanced MCP Server',
    description: `## Status
🟡 Partial - Basic MCP exists, needs expansion

## Current MCP Features
- Create/update/list tickets
- Add comments
- Update status

## New Features Needed
1. **Bulk Operations**
   - Create multiple tickets at once
   - Batch update status
   - Bulk assign tickets

2. **Smart Queries**
   - "Find tickets blocking X"
   - "What's ready for review?"
   - "Show my tickets updated this week"

3. **Context Awareness**
   - Remember recent tickets
   - Suggest related tickets
   - Auto-link dependencies

4. **Workflow Automation**
   - Auto-transition on PR merge
   - Auto-assign based on labels
   - Auto-add to sprint

## Implementation
See detailed steps in roadmap doc

## Testing
- [ ] Bulk create tickets
- [ ] Smart query examples
- [ ] Context across sessions
- [ ] Workflow triggers work`,
    labels: ['feature', 'backend', 'mcp', 'phase-5'],
    status: 'backlog',
    story_points: 8,
  },

  {
    title: '5.2 AI Epic Breakdown',
    description: `## Status
🔴 New Build

## What Is It?
AI assistant breaks down epics into implementable sub-tickets.

## Features
- Input: Epic description
- AI generates: Sub-tickets with titles, descriptions, story points
- User reviews/edits before creating
- Learns from project patterns

## Implementation
1. **AI Integration** (2 hours)
   - OpenAI/Claude API
   - Prompt engineering for ticket breakdown
   - Parse structured output (JSON)

2. **UI Flow** (3 hours)
   - "Generate Sub-tickets" button on epic
   - AI loading state
   - Review generated tickets
   - Edit before creating
   - Create all or selectively

3. **Learning System** (2 hours)
   - Analyze existing tickets for patterns
   - Include project context in prompt
   - Feedback loop (accept/reject)

## Testing
- [ ] Generate tickets from epic
- [ ] Review suggestions
- [ ] Edit generated tickets
- [ ] Create all tickets
- [ ] API rate limiting works`,
    labels: ['feature', 'backend', 'frontend', 'ai', 'phase-5'],
    status: 'backlog',
    story_points: 5,
  },

  {
    title: '5.3 Smart Git Integration',
    description: `## Status
🟡 Partial - Manual commit linking exists

## Current State
- Manual commit linking via MCP
- No webhook integration
- No PR tracking

## New Features
1. **GitHub Webhooks** (3 hours)
   - Receive push events
   - Parse commit messages for ticket IDs
   - Auto-link commits to tickets

2. **PR Integration** (2 hours)
   - Link PRs to tickets
   - Show PR status on ticket
   - Auto-update ticket on PR merge

3. **Smart Parsing** (2 hours)
   - Detect ticket references (PROJ-123, #123)
   - Extract multiple tickets from commit
   - Support conventional commits

4. **UI Components** (2 hours)
   - Show linked commits on ticket
   - Show linked PRs
   - GitHub/GitLab icons
   - Click to view on GitHub

## Implementation
See detailed steps in roadmap doc

## Testing
- [ ] Webhook receives events
- [ ] Commit parsed for ticket ID
- [ ] PR linked to ticket
- [ ] Ticket status updates on merge
- [ ] Webhook security verified`,
    labels: ['feature', 'backend', 'frontend', 'git', 'phase-5'],
    status: 'backlog',
    story_points: 8,
  },
]

async function main() {
  console.log('🚀 Creating roadmap tickets in Jility...\n')
  console.log(`API: ${API_BASE}`)
  console.log(`Project: ${PROJECT_ID}\n`)

  const createdTickets: any[] = []
  let epicParentId: string | undefined

  for (const ticketData of roadmapTickets) {
    try {
      console.log(`Creating: ${ticketData.title}...`)

      // If this is an epic/phase, save its ID as parent for next tickets
      const isEpic = ticketData.labels?.includes('epic')

      const ticket = await createTicket({
        ...ticketData,
        project_id: PROJECT_ID!,
        epic_id: isEpic ? undefined : epicParentId,
      })

      createdTickets.push(ticket)
      console.log(`✅ Created ${ticket.number}: ${ticket.title}`)

      // If this was an epic, save it as parent for following tickets
      if (isEpic) {
        epicParentId = ticket.id
        console.log(`   (Set as parent epic for following tickets)`)
      }

    } catch (error) {
      console.error(`❌ Failed to create ticket: ${ticketData.title}`)
      console.error(error)
    }
  }

  console.log(`\n✨ Done! Created ${createdTickets.length}/${roadmapTickets.length} tickets`)
  console.log('\nCreated tickets:')
  createdTickets.forEach(t => {
    console.log(`  - ${t.number}: ${t.title}`)
  })
}

main().catch(console.error)
