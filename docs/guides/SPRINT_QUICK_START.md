# Sprint Management Quick Start Guide

## Getting Started

### 1. Start the Backend Server

```bash
cd /home/user/Jility/jility-server
cargo run
```

Server runs on `http://localhost:3001`

### 2. Start the Frontend

```bash
cd /home/user/Jility/jility-web
npm run dev
```

Frontend runs on `http://localhost:3000`

### 3. Access Sprint Features

Navigate to:
- **Sprint Planning**: http://localhost:3000/sprint/planning
- **Active Sprint**: http://localhost:3000/sprint/active
- **Sprint History**: http://localhost:3000/sprint/history

## Quick Workflow

### Create Your First Sprint

**Option A: Via API**

```bash
# 1. Create a sprint
curl -X POST http://localhost:3001/api/projects/550e8400-e29b-41d4-a716-446655440000/sprints \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Sprint 1",
    "goal": "Build authentication system"
  }'

# Response will include sprint ID
```

**Option B: Via Frontend**

1. Go to Sprint Planning page
2. Create sprint via UI (if available)
3. Or use the API as shown above

### Add Tickets to Sprint

**Via API**:

```bash
# Get sprint ID and ticket ID from previous responses
SPRINT_ID="your-sprint-id"
TICKET_ID="your-ticket-id"

# Add ticket to sprint
curl -X POST http://localhost:3001/api/sprints/$SPRINT_ID/tickets/$TICKET_ID \
  -H "Content-Type: application/json" \
  -d '{"added_by": "user"}'
```

**Via Frontend**:

1. Go to Sprint Planning page
2. Click tickets in backlog (left) to add to sprint (right)
3. Click tickets in sprint to remove back to backlog

### Start the Sprint

**Via API**:

```bash
SPRINT_ID="your-sprint-id"

curl -X POST http://localhost:3001/api/sprints/$SPRINT_ID/start \
  -H "Content-Type: application/json" \
  -d '{
    "start_date": "2024-01-15T00:00:00Z",
    "end_date": "2024-01-29T23:59:59Z"
  }'
```

**Via Frontend**:

1. Go to Sprint Planning page
2. Review planned tickets and capacity
3. Click **Start Sprint** button
4. Sprint will start with a 2-week duration (default)

### Track Sprint Progress

1. Go to **Active Sprint** page
2. View:
   - Progress bar and completion percentage
   - Days remaining
   - Sprint statistics (total, completed, in-progress, to-do)
   - Burndown chart
   - Kanban board with sprint tickets

### Complete the Sprint

**Via API**:

```bash
SPRINT_ID="your-sprint-id"

curl -X POST http://localhost:3001/api/sprints/$SPRINT_ID/complete
```

**Via Frontend**:

1. Go to Active Sprint page
2. Click **Complete Sprint** button
3. Confirm completion

### View Sprint History

1. Go to **Sprint History** page
2. View:
   - Velocity trend chart
   - Average velocity
   - List of completed sprints
   - Click individual sprints for details

## API Endpoints Reference

### Sprint Management

```bash
# List sprints
GET /api/projects/:project_id/sprints?status=planning

# Create sprint
POST /api/projects/:project_id/sprints
Body: { name, goal?, start_date?, end_date? }

# Get sprint details
GET /api/sprints/:id

# Update sprint
PUT /api/sprints/:id
Body: { name?, goal?, start_date?, end_date? }

# Delete sprint
DELETE /api/sprints/:id

# Start sprint
POST /api/sprints/:id/start
Body: { start_date, end_date }

# Complete sprint
POST /api/sprints/:id/complete
```

### Ticket Management

```bash
# Add ticket to sprint
POST /api/sprints/:id/tickets/:ticket_id
Body: { added_by }

# Remove ticket from sprint
DELETE /api/sprints/:id/tickets/:ticket_id
```

### Analytics

```bash
# Get sprint statistics
GET /api/sprints/:id/stats

# Get burndown data
GET /api/sprints/:id/burndown

# Get sprint history
GET /api/projects/:project_id/sprint-history
```

## Common Tasks

### Check Sprint Status

```bash
SPRINT_ID="your-sprint-id"

# Get full sprint details with stats
curl http://localhost:3001/api/sprints/$SPRINT_ID | jq
```

### Monitor Burndown

```bash
SPRINT_ID="your-sprint-id"

# Get burndown chart data
curl http://localhost:3001/api/sprints/$SPRINT_ID/burndown | jq
```

### Calculate Velocity

```bash
PROJECT_ID="550e8400-e29b-41d4-a716-446655440000"

# Get all completed sprints with velocity
curl http://localhost:3001/api/projects/$PROJECT_ID/sprint-history | jq
```

## Configuration

### Adjust Team Capacity

Edit `/home/user/Jility/jility-web/lib/sprint-utils.ts`:

```typescript
// Default: 5 members × 14 days × 3 pts/day = 210 pts
export function calculateCapacity(
  teamMembers: number,
  sprintDays: number,
  pointsPerDay: number = 3  // Adjust this
): number {
  return teamMembers * sprintDays * pointsPerDay
}
```

Then update Sprint Planning page to use your values:

```typescript
const capacity = calculateCapacity(8, 10, 4)  // 8 members, 10 days, 4 pts/day
```

### Change Sprint Duration

In Sprint Planning page, update the `startSprint` function:

```typescript
async function startSprint() {
  const startDate = new Date().toISOString()
  const endDate = new Date(Date.now() + 10 * 24 * 60 * 60 * 1000).toISOString() // 10 days
  // ... rest of code
}
```

## Troubleshooting

### No Sprints Showing

**Problem**: Sprint planning page shows "No Planning Sprint"

**Solution**:
```bash
# Create a sprint via API
curl -X POST http://localhost:3001/api/projects/550e8400-e29b-41d4-a716-446655440000/sprints \
  -H "Content-Type: application/json" \
  -d '{"name": "Sprint 1"}'
```

### No Active Sprint

**Problem**: Active sprint page shows "No Active Sprint"

**Solution**: Start a sprint from the planning page or via API.

### Tickets Not Loading

**Problem**: Backlog is empty in sprint planning

**Solution**: Create tickets first:
```bash
curl -X POST http://localhost:3001/api/tickets \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Test Ticket",
    "description": "Test",
    "project_id": "550e8400-e29b-41d4-a716-446655440000",
    "status": "backlog",
    "story_points": 5,
    "created_by": "user"
  }'
```

### Burndown Chart Empty

**Problem**: Burndown chart shows no data

**Solution**:
1. Ensure sprint has start_date and end_date
2. Add tickets to the sprint
3. Mark some tickets as "done"

## Sample Data Setup

### Create Complete Sprint with Data

```bash
# 1. Create sprint
SPRINT_RESPONSE=$(curl -s -X POST http://localhost:3001/api/projects/550e8400-e29b-41d4-a716-446655440000/sprints \
  -H "Content-Type: application/json" \
  -d '{"name": "Demo Sprint", "goal": "Demonstrate sprint features"}')

SPRINT_ID=$(echo $SPRINT_RESPONSE | jq -r '.id')

# 2. Create tickets
for i in {1..5}; do
  TICKET_RESPONSE=$(curl -s -X POST http://localhost:3001/api/tickets \
    -H "Content-Type: application/json" \
    -d "{
      \"title\": \"Task $i\",
      \"description\": \"Demo task\",
      \"project_id\": \"550e8400-e29b-41d4-a716-446655440000\",
      \"status\": \"backlog\",
      \"story_points\": 5,
      \"created_by\": \"user\"
    }")

  TICKET_ID=$(echo $TICKET_RESPONSE | jq -r '.id')

  # Add to sprint
  curl -s -X POST http://localhost:3001/api/sprints/$SPRINT_ID/tickets/$TICKET_ID \
    -H "Content-Type: application/json" \
    -d '{"added_by": "user"}'
done

# 3. Start sprint
curl -X POST http://localhost:3001/api/sprints/$SPRINT_ID/start \
  -H "Content-Type: application/json" \
  -d "{
    \"start_date\": \"$(date -u +%Y-%m-%dT00:00:00Z)\",
    \"end_date\": \"$(date -u -d '+14 days' +%Y-%m-%dT23:59:59Z)\"
  }"

echo "Demo sprint created with ID: $SPRINT_ID"
```

## Next Steps

1. **Customize capacity calculation** for your team size
2. **Add more tickets** to build realistic sprint planning
3. **Complete a few sprints** to see velocity trends
4. **Review burndown charts** to identify bottlenecks
5. **Use sprint history** to improve future planning

## Additional Resources

- **Full Documentation**: `/home/user/Jility/SPRINT_MANAGEMENT_IMPLEMENTATION.md`
- **API Documentation**: `/home/user/Jility/API.md`
- **Database Schema**: `/home/user/Jility/jility-core/src/entities/sprint.rs`

## Support

For issues or questions:
1. Check the full implementation docs
2. Review API endpoint responses for errors
3. Check browser console for frontend errors
4. Verify backend server is running on port 3001
