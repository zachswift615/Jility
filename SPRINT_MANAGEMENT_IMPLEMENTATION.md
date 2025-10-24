# Sprint Management Implementation

## Overview

This document describes the complete sprint management system implemented for Jility, including backend APIs, frontend pages, and all necessary components for planning, tracking, and analyzing sprints.

## Backend Implementation

### 1. Database Schema

The sprint system uses the existing database entities:

**Sprint Table** (`jility-core/src/entities/sprint.rs`):
- `id`: UUID (primary key)
- `project_id`: UUID (foreign key to projects)
- `name`: String
- `goal`: Optional text
- `start_date`: Optional DateTime
- `end_date`: Optional DateTime
- `status`: String ("planning", "active", "completed")
- `created_at`: DateTime
- `updated_at`: DateTime

**SprintTicket Junction Table** (`jility-core/src/entities/sprint_ticket.rs`):
- `id`: UUID (primary key)
- `sprint_id`: UUID (foreign key to sprints)
- `ticket_id`: UUID (foreign key to tickets)
- `added_at`: DateTime
- `added_by`: String

### 2. API Endpoints

All endpoints are implemented in `/home/user/Jility/jility-server/src/api/sprints.rs`:

#### Sprint CRUD Operations

- **`GET /api/projects/:project_id/sprints`** - List all sprints for a project
  - Query params: `status` (optional filter)
  - Returns: Array of sprint summaries

- **`POST /api/projects/:project_id/sprints`** - Create new sprint
  - Body: `{ name, goal?, start_date?, end_date? }`
  - Returns: Created sprint

- **`GET /api/sprints/:id`** - Get sprint details with tickets and stats
  - Returns: Sprint, tickets, and calculated statistics

- **`PUT /api/sprints/:id`** - Update sprint
  - Body: `{ name?, goal?, start_date?, end_date? }`
  - Returns: Updated sprint

- **`DELETE /api/sprints/:id`** - Delete sprint
  - Cascades to remove sprint_ticket entries

#### Sprint Lifecycle

- **`POST /api/sprints/:id/start`** - Start a sprint
  - Body: `{ start_date, end_date }`
  - Changes status from "planning" to "active"

- **`POST /api/sprints/:id/complete`** - Complete a sprint
  - Changes status from "active" to "completed"

#### Ticket Management

- **`POST /api/sprints/:id/tickets/:ticket_id`** - Add ticket to sprint
  - Body: `{ added_by }`
  - Creates sprint_ticket record
  - Records change event in ticket_changes

- **`DELETE /api/sprints/:id/tickets/:ticket_id`** - Remove ticket from sprint
  - Deletes sprint_ticket record
  - Records change event

#### Analytics

- **`GET /api/sprints/:id/stats`** - Get sprint statistics
  - Returns:
    - Total tickets and points
    - Completed tickets and points
    - In-progress tickets and points
    - To-do tickets and points
    - Completion percentage

- **`GET /api/sprints/:id/burndown`** - Get burndown chart data
  - Returns: Array of daily data points with ideal vs actual remaining points
  - Calculates ideal burndown (linear)
  - Tracks actual progress based on ticket completion

- **`GET /api/projects/:project_id/sprint-history`** - Get historical data
  - Returns:
    - All completed sprints
    - Velocity data (points completed per sprint)
    - Average velocity across all sprints

### 3. Request/Response Models

Located in `/home/user/Jility/jility-server/src/models/`:

**Request Types**:
```rust
CreateSprintRequest { name, goal?, start_date?, end_date? }
UpdateSprintRequest { name?, goal?, start_date?, end_date? }
StartSprintRequest { start_date, end_date }
AddTicketToSprintRequest { added_by }
```

**Response Types**:
```rust
SprintResponse {
    id, project_id, name, goal?,
    status, start_date?, end_date?,
    created_at, updated_at
}

SprintDetailsResponse {
    sprint: SprintResponse,
    tickets: Vec<TicketResponse>,
    stats: SprintStats
}

SprintStats {
    total_tickets, total_points,
    completed_tickets, completed_points,
    in_progress_tickets, in_progress_points,
    todo_tickets, todo_points,
    completion_percentage
}

BurndownData {
    sprint_id,
    data_points: Vec<BurndownDataPoint>
}

BurndownDataPoint { date, ideal, actual }

SprintHistoryResponse {
    sprints: Vec<SprintResponse>,
    velocity_data: Vec<VelocityData>,
    average_velocity
}

VelocityData { sprint_name, completed_points }
```

## Frontend Implementation

### 1. Utility Functions

**`/home/user/Jility/jility-web/lib/sprint-utils.ts`**:

- `calculateCapacity(teamMembers, sprintDays, pointsPerDay)` - Calculate sprint capacity
- `calculateVelocity(completedSprints)` - Calculate average velocity
- `calculateProgress(stats)` - Calculate completion percentage
- `calculateDaysRemaining(endDate)` - Days left in sprint
- `formatSprintDateRange(start, end)` - Format date range
- `getSprintStatusColor(status)` - Get Tailwind classes for status badge

### 2. Components

#### Burndown Chart

**`/home/user/Jility/jility-web/components/sprint/burndown-chart.tsx`**:

- Pure SVG implementation (no external chart library)
- Shows ideal burndown (dashed line) vs actual (solid line)
- Responsive width, auto-scaling Y-axis
- Includes legend and grid lines
- Dark mode support

#### Sprint Selector

**`/home/user/Jility/jility-web/components/sprint/sprint-selector.tsx`**:

- Dropdown to select active sprint
- Auto-selects active sprint by default
- Filters tickets by selected sprint
- Reusable component with callback

### 3. Pages

#### Sprint Planning

**`/home/user/Jility/jility-web/app/sprint/planning/page.tsx`**:

**Features**:
- View backlog tickets
- Add/remove tickets to/from sprint
- Visual capacity indicator
  - Shows planned points vs capacity
  - Color-coded: green (<80%), yellow (80-100%), red (>100%)
- Sprint goal display
- Start sprint button

**Layout**:
```
┌─────────────────────────────────────────┐
│ Sprint Name                [Start Sprint] │
│ Goal: Complete authentication            │
│                                          │
│ Capacity: 210 pts | Planned: 176 pts | 84% │
│ [████████████████░░] Progress Bar         │
└─────────────────────────────────────────┘

┌──────────────────┬──────────────────────┐
│ Backlog          │ Sprint 5             │
│ (Drag to add →)  │ (← Drag to remove)   │
│                  │                      │
│ TASK-123 (5 pts) │ TASK-156 (8 pts)    │
│ TASK-124 (3 pts) │ TASK-157 (5 pts)    │
└──────────────────┴──────────────────────┘
```

#### Active Sprint View

**`/home/user/Jility/jility-web/app/sprint/active/page.tsx`**:

**Features**:
- Sprint progress bar with percentage
- Days remaining counter
- Sprint statistics dashboard:
  - Total tickets
  - Completed tickets
  - In-progress tickets
  - To-do tickets
- Burndown chart
- Kanban board filtered to sprint tickets
  - Three columns: To Do, In Progress, Done
  - Click ticket to view details
- Complete sprint button

**Layout**:
```
┌─────────────────────────────────────────┐
│ Sprint 5                  [Complete Sprint] │
│ Jan 15 - Jan 29 | 7 days remaining      │
│                                          │
│ 26/38 points | 68% complete             │
│ [████████████████░░░░░░░░]               │
│                                          │
│ 15 Total | 10 Done | 3 In Progress | 2 To Do │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│ Burndown Chart                          │
│                                          │
│ [SVG chart showing ideal vs actual]     │
└─────────────────────────────────────────┘

┌──────────┬──────────┬──────────┐
│ To Do    │ In Prog  │ Done     │
│ TASK-123 │ TASK-156 │ TASK-157 │
│ TASK-124 │ TASK-158 │ TASK-159 │
└──────────┴──────────┴──────────┘
```

#### Sprint History

**`/home/user/Jility/jility-web/app/sprint/history/page.tsx`**:

**Features**:
- Velocity trend chart (bar chart)
- Average velocity calculation
- List of completed sprints with:
  - Sprint name and goal
  - Date range
  - Points completed
  - Click to view details

**Layout**:
```
┌─────────────────────────────────────────┐
│ Velocity Trend                          │
│ Average Velocity: 42 points/sprint      │
│                                          │
│ Sprint 5  [████████████] 45 pts         │
│ Sprint 4  [███████████░] 42 pts         │
│ Sprint 3  [██████████░░] 38 pts         │
│ Sprint 2  [█████████░░░] 35 pts         │
│ Sprint 1  [███████░░░░░] 28 pts         │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│ Sprint 5 - completed                    │
│ Goal: Complete authentication           │
│ Jan 15 - Jan 29 | 45 points             │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│ Sprint 4 - completed                    │
│ Goal: Database optimization             │
│ Jan 1 - Jan 14 | 42 points              │
└─────────────────────────────────────────┘
```

### 4. Navigation

Updated `/home/user/Jility/jility-web/components/layout/navbar.tsx` to include:
- Sprint Planning link (Calendar icon)
- Active Sprint link (Activity icon)
- Sprint History link (Clock icon)

## Usage Guide

### 1. Creating a Sprint

```bash
# Create a sprint via API
curl -X POST http://localhost:3001/api/projects/{project_id}/sprints \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Sprint 1",
    "goal": "Build authentication system"
  }'
```

Or use the frontend Sprint Planning page.

### 2. Planning a Sprint

1. Navigate to **Sprint Planning** page
2. View backlog tickets on the left
3. Click tickets to add them to the sprint (right side)
4. Monitor capacity indicator (green = good, yellow = near capacity, red = over)
5. Click **Start Sprint** when ready

### 3. Running a Sprint

1. Navigate to **Active Sprint** page
2. View sprint progress and burndown chart
3. Click tickets to update their status
4. Track daily progress on burndown chart
5. Click **Complete Sprint** when done

### 4. Reviewing Sprint History

1. Navigate to **Sprint History** page
2. View velocity trend across sprints
3. Click individual sprints to see details
4. Use average velocity for capacity planning

## Key Features

### Sprint Statistics

The system automatically calculates:
- Total tickets and story points in sprint
- Completed, in-progress, and to-do breakdowns
- Completion percentage
- Burndown data (ideal vs actual)

### Burndown Chart

- **Ideal line**: Linear burndown from total points to zero
- **Actual line**: Real remaining points based on ticket completion
- Updates as tickets are marked done
- Helps identify if sprint is on track

### Velocity Tracking

- Tracks completed points per sprint
- Calculates rolling average velocity
- Helps with capacity planning for future sprints

### Capacity Planning

Default formula: `teamMembers × sprintDays × pointsPerDay`
- Default: 5 team members × 14 days × 3 points/day = 210 points
- Customizable in sprint-utils.ts

## Configuration

### Team Capacity

Edit `/home/user/Jility/jility-web/lib/sprint-utils.ts`:

```typescript
// Change default points per person per day
export function calculateCapacity(
  teamMembers: number,
  sprintDays: number,
  pointsPerDay: number = 3  // Change this value
): number {
  return teamMembers * sprintDays * pointsPerDay
}
```

### Project ID

Currently hardcoded in pages. To make dynamic:

1. Add project context provider
2. Pass project ID from URL params
3. Update all API calls to use context

Example:
```typescript
// In layout or page
const projectId = params.projectId

// Pass to child components
<SprintSelector projectId={projectId} />
```

## Testing

### Manual Testing

1. **Create Sprint**:
   ```bash
   curl -X POST http://localhost:3001/api/projects/{project_id}/sprints \
     -H "Content-Type: application/json" \
     -d '{"name": "Test Sprint", "goal": "Test goal"}'
   ```

2. **Add Ticket to Sprint**:
   ```bash
   curl -X POST http://localhost:3001/api/sprints/{sprint_id}/tickets/{ticket_id} \
     -H "Content-Type: application/json" \
     -d '{"added_by": "user"}'
   ```

3. **Start Sprint**:
   ```bash
   curl -X POST http://localhost:3001/api/sprints/{sprint_id}/start \
     -H "Content-Type: application/json" \
     -d '{
       "start_date": "2024-01-01T00:00:00Z",
       "end_date": "2024-01-14T23:59:59Z"
     }'
   ```

4. **Get Sprint Stats**:
   ```bash
   curl http://localhost:3001/api/sprints/{sprint_id}/stats
   ```

5. **Get Burndown Data**:
   ```bash
   curl http://localhost:3001/api/sprints/{sprint_id}/burndown
   ```

6. **Complete Sprint**:
   ```bash
   curl -X POST http://localhost:3001/api/sprints/{sprint_id}/complete
   ```

### Expected Behaviors

- Creating a sprint sets status to "planning"
- Starting a sprint requires start_date and end_date
- Only "planning" sprints can be started
- Only "active" sprints can be completed
- Adding ticket to sprint records change in ticket_changes table
- Burndown chart shows linear ideal line
- Stats calculate correctly based on ticket statuses
- Velocity averages completed points across all sprints

## Architecture Decisions

### 1. Sprint Ticket Association

Used junction table (sprint_tickets) rather than foreign key on tickets table:
- ✅ Allows tickets to be in multiple historical sprints
- ✅ Preserves sprint history when tickets are removed
- ✅ Enables "moved from sprint" tracking

### 2. Burndown Calculation

Simplified implementation:
- Uses ticket completion timestamps
- Does not track intra-day changes
- Assumes tickets marked "done" stay done

For production:
- Consider tracking state changes throughout day
- Handle tickets moving back to "in progress"
- Account for scope changes mid-sprint

### 3. Frontend Charts

Used pure SVG rather than chart library:
- ✅ No additional dependencies
- ✅ Full control over styling
- ✅ Dark mode support
- ⚠️ Less features than libraries like Recharts

Alternative: Install recharts for more advanced charts:
```bash
npm install recharts
```

### 4. Status Management

Three-state sprint lifecycle:
1. **planning**: Sprint being planned, tickets can be added/removed
2. **active**: Sprint in progress, burndown tracked
3. **completed**: Historical sprint, included in velocity

## Files Created/Modified

### Backend

**Created**:
- `/home/user/Jility/jility-server/src/api/sprints.rs` - Sprint API endpoints (698 lines)

**Modified**:
- `/home/user/Jility/jility-server/src/api/mod.rs` - Added sprint routes
- `/home/user/Jility/jility-server/src/models/request.rs` - Added sprint request types
- `/home/user/Jility/jility-server/src/models/response.rs` - Added sprint response types

### Frontend

**Created**:
- `/home/user/Jility/jility-web/lib/sprint-utils.ts` - Sprint utility functions
- `/home/user/Jility/jility-web/components/sprint/burndown-chart.tsx` - Burndown visualization
- `/home/user/Jility/jility-web/components/sprint/sprint-selector.tsx` - Sprint dropdown
- `/home/user/Jility/jility-web/app/sprint/planning/page.tsx` - Sprint planning page
- `/home/user/Jility/jility-web/app/sprint/active/page.tsx` - Active sprint page
- `/home/user/Jility/jility-web/app/sprint/history/page.tsx` - Sprint history page

**Modified**:
- `/home/user/Jility/jility-web/components/layout/navbar.tsx` - Added sprint navigation

## Future Enhancements

### Backend

1. **Advanced Burndown**:
   - Track hourly/daily snapshots of remaining work
   - Store burndown snapshots in dedicated table
   - Support scope changes mid-sprint

2. **Sprint Reports**:
   - Generate PDF/HTML sprint report
   - Include charts, statistics, and retrospective notes
   - Export velocity trends

3. **Sprint Templates**:
   - Save sprint configurations as templates
   - Auto-populate sprints based on templates
   - Include default ticket allocation

4. **Sprint Metrics**:
   - Track sprint goal completion
   - Measure predictability (planned vs completed)
   - Calculate sprint health score

### Frontend

1. **Drag-and-Drop**:
   - Implement actual drag-and-drop with @dnd-kit
   - Visual feedback during drag
   - Multi-select for bulk operations

2. **Advanced Charts**:
   - Install Recharts or similar library
   - Add cumulative flow diagram
   - Sprint comparison charts

3. **Sprint Calendar**:
   - Calendar view of all sprints
   - Drag to adjust dates
   - Visualize overlapping sprints

4. **Retrospective**:
   - Add retrospective notes to completed sprints
   - Track action items from retros
   - Link action items to next sprint

5. **Forecasting**:
   - Use velocity to forecast completion dates
   - Suggest sprint capacity based on history
   - Warn about over-commitment

## Summary

The sprint management system is now fully implemented with:

✅ **11 Backend API Endpoints** - Full CRUD + lifecycle + analytics
✅ **3 Frontend Pages** - Planning, active sprint, history
✅ **3 Reusable Components** - Burndown chart, sprint selector, utilities
✅ **Complete Sprint Lifecycle** - Create → Plan → Start → Track → Complete
✅ **Analytics & Reporting** - Burndown charts, velocity tracking, statistics
✅ **Navigation Integration** - Sprint links in main navbar

The system is ready for use and can be extended with the future enhancements listed above.
