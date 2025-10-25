# Jility Server Testing Guide

## Prerequisites

1. Install Rust and Cargo
2. Install SQLite or PostgreSQL
3. Install `curl` or similar HTTP client

## Setup

### 1. Set Environment Variables

```bash
# For SQLite (default, creates .jility/data.db)
export DATABASE_URL="sqlite://.jility/data.db?mode=rwc"

# For PostgreSQL
export DATABASE_URL="postgres://user:password@localhost/jility"

# Optional: Custom bind address
export BIND_ADDRESS="0.0.0.0:3000"
```

### 2. Run Migrations

```bash
# TODO: Create migration tool
# For now, the database schema needs to be created manually
```

### 3. Start Server

```bash
cd jility-server
cargo run
```

The server will start on `http://localhost:3000`

---

## Testing with curl

### Create a Project

```bash
curl -X POST http://localhost:3000/api/projects \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My First Project",
    "description": "Testing the API"
  }'
```

**Expected Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "My First Project",
  "description": "Testing the API",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

### List Projects

```bash
curl http://localhost:3000/api/projects
```

### Create a Ticket

```bash
curl -X POST http://localhost:3000/api/tickets \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Implement user authentication",
    "description": "Add JWT-based authentication to the API",
    "story_points": 5,
    "status": "todo",
    "assignees": ["alice"],
    "labels": ["backend", "feature"]
  }'
```

**Expected Response:**
```json
{
  "id": "650e8400-e29b-41d4-a716-446655440000",
  "number": "TASK-1",
  "title": "Implement user authentication",
  "description": "Add JWT-based authentication to the API",
  "status": "todo",
  "story_points": 5,
  "assignees": ["alice"],
  "labels": ["backend", "feature"],
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "created_by": "system",
  "parent_id": null,
  "epic_id": null
}
```

### Get Ticket Details

```bash
TICKET_ID="650e8400-e29b-41d4-a716-446655440000"
curl http://localhost:3000/api/tickets/$TICKET_ID
```

### Update Ticket Status

```bash
curl -X PATCH http://localhost:3000/api/tickets/$TICKET_ID/status \
  -H "Content-Type: application/json" \
  -d '{
    "status": "in_progress"
  }'
```

### Add a Comment

```bash
curl -X POST http://localhost:3000/api/tickets/$TICKET_ID/comments \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Working on the JWT implementation now"
  }'
```

### Update Description

```bash
curl -X PATCH http://localhost:3000/api/tickets/$TICKET_ID/description \
  -H "Content-Type: application/json" \
  -d '{
    "description": "Add JWT-based authentication to the API\n\n## Implementation Plan\n1. Install jsonwebtoken crate\n2. Create auth middleware\n3. Add login endpoint",
    "operation": "replace_all"
  }'
```

### Assign Ticket

```bash
curl -X POST http://localhost:3000/api/tickets/$TICKET_ID/assign \
  -H "Content-Type: application/json" \
  -d '{
    "assignee": "agent-1"
  }'
```

### Add Dependency

```bash
# First create another ticket
curl -X POST http://localhost:3000/api/tickets \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Setup database schema",
    "status": "done"
  }'

# Get the second ticket ID, then add dependency
DEPENDENCY_ID="750e8400-e29b-41d4-a716-446655440000"

curl -X POST http://localhost:3000/api/tickets/$TICKET_ID/dependencies \
  -H "Content-Type: application/json" \
  -d "{
    \"depends_on_id\": \"$DEPENDENCY_ID\"
  }"
```

### Link a Commit

```bash
curl -X POST http://localhost:3000/api/tickets/$TICKET_ID/commits \
  -H "Content-Type: application/json" \
  -d '{
    "commit_hash": "abc123def456",
    "commit_message": "Add JWT authentication middleware"
  }'
```

### View Activity Timeline

```bash
curl http://localhost:3000/api/tickets/$TICKET_ID/activity
```

### Search Tickets

```bash
curl "http://localhost:3000/api/search?q=authentication&limit=10"
```

---

## Testing WebSocket

### Using `wscat` (Node.js)

```bash
npm install -g wscat
wscat -c ws://localhost:3000/ws
```

Then perform actions (create tickets, update status, add comments) in another terminal to see real-time broadcasts.

### Using Python

```python
import asyncio
import websockets
import json

async def test_websocket():
    async with websockets.connect('ws://localhost:3000/ws') as websocket:
        while True:
            message = await websocket.recv()
            data = json.loads(message)
            print(f"Received: {data['type']}")
            print(json.dumps(data, indent=2))

asyncio.run(test_websocket())
```

---

## Testing Workflow

### Complete Ticket Lifecycle

```bash
# 1. Create project
PROJECT_RESPONSE=$(curl -s -X POST http://localhost:3000/api/projects \
  -H "Content-Type: application/json" \
  -d '{"name": "Test Project"}')

# 2. Create ticket
TICKET_RESPONSE=$(curl -s -X POST http://localhost:3000/api/tickets \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Build feature X",
    "description": "Initial description",
    "status": "backlog"
  }')

TICKET_ID=$(echo $TICKET_RESPONSE | jq -r '.id')

# 3. Move to todo
curl -X PATCH http://localhost:3000/api/tickets/$TICKET_ID/status \
  -H "Content-Type: application/json" \
  -d '{"status": "todo"}'

# 4. Assign to developer
curl -X POST http://localhost:3000/api/tickets/$TICKET_ID/assign \
  -H "Content-Type: application/json" \
  -d '{"assignee": "alice"}'

# 5. Start work
curl -X PATCH http://localhost:3000/api/tickets/$TICKET_ID/status \
  -H "Content-Type: application/json" \
  -d '{"status": "in_progress"}'

# 6. Add comment
curl -X POST http://localhost:3000/api/tickets/$TICKET_ID/comments \
  -H "Content-Type: application/json" \
  -d '{"content": "Started implementation"}'

# 7. Update description with details
curl -X PATCH http://localhost:3000/api/tickets/$TICKET_ID/description \
  -H "Content-Type: application/json" \
  -d '{
    "description": "Initial description\n\n## Progress\n- [x] Setup\n- [ ] Implementation\n- [ ] Testing"
  }'

# 8. Link commit
curl -X POST http://localhost:3000/api/tickets/$TICKET_ID/commits \
  -H "Content-Type: application/json" \
  -d '{
    "commit_hash": "abc123",
    "commit_message": "Implement feature X"
  }'

# 9. Move to review
curl -X PATCH http://localhost:3000/api/tickets/$TICKET_ID/status \
  -H "Content-Type: application/json" \
  -d '{"status": "review"}'

# 10. Complete
curl -X PATCH http://localhost:3000/api/tickets/$TICKET_ID/status \
  -H "Content-Type: application/json" \
  -d '{"status": "done"}'

# 11. View full timeline
curl http://localhost:3000/api/tickets/$TICKET_ID/activity

# 12. View description history
curl http://localhost:3000/api/tickets/$TICKET_ID/history
```

---

## Expected Behavior

### Event Sourcing

Every change should be recorded in the `ticket_changes` table:
- Creating a ticket records a "created" event
- Changing status records a "status_changed" event
- Adding assignee records an "assignee_added" event
- Updating description records a "description_changed" event

### WebSocket Broadcasting

When you:
- Create a ticket → All connected clients receive `TicketCreated`
- Update status → All clients receive `StatusChanged`
- Add comment → All clients receive `CommentAdded`
- Update ticket → All clients receive `TicketUpdated`

### History Tracking

- Description changes are versioned
- Can view all previous versions
- Can see who changed what and when
- Can view activity timeline

---

## Database Verification

### Check Ticket Changes

```bash
# Connect to SQLite
sqlite3 .jility/data.db

# View all changes
SELECT * FROM ticket_changes ORDER BY changed_at DESC LIMIT 10;

# View changes for specific ticket
SELECT change_type, changed_by, changed_at 
FROM ticket_changes 
WHERE ticket_id = 'your-ticket-id' 
ORDER BY changed_at ASC;
```

### Check Relationships

```bash
# View ticket assignees
SELECT t.ticket_number, t.title, ta.assignee
FROM tickets t
JOIN ticket_assignees ta ON t.id = ta.ticket_id;

# View dependencies
SELECT 
  t1.ticket_number as ticket,
  t2.ticket_number as depends_on
FROM ticket_dependencies td
JOIN tickets t1 ON td.ticket_id = t1.id
JOIN tickets t2 ON td.depends_on_id = t2.id;
```

---

## Performance Testing

### Load Test with `ab` (Apache Bench)

```bash
# Test list tickets endpoint
ab -n 1000 -c 10 http://localhost:3000/api/tickets

# Test create ticket
ab -n 100 -c 10 -p ticket.json -T application/json \
  http://localhost:3000/api/tickets
```

### Monitor Database

```bash
# Check database size
du -h .jility/data.db

# Check number of tickets
sqlite3 .jility/data.db "SELECT COUNT(*) FROM tickets;"

# Check number of changes
sqlite3 .jility/data.db "SELECT COUNT(*) FROM ticket_changes;"
```

---

## Troubleshooting

### Server won't start

1. Check DATABASE_URL is set correctly
2. Ensure database file/server is accessible
3. Check port 3000 is not in use: `lsof -i :3000`

### WebSocket not connecting

1. Ensure server is running
2. Check firewall settings
3. Verify WebSocket URL (ws:// not http://)

### Changes not being recorded

1. Check ticket_changes table exists
2. Verify database migrations ran
3. Check server logs for errors

---

## Next Steps

1. **Add Authentication**: Implement JWT-based auth
2. **Add Migrations**: Create database migration tool
3. **Add Tests**: Unit and integration tests
4. **Add Validation**: Input validation and constraints
5. **Optimize Queries**: Add indexes, use joins
6. **Add Full-Text Search**: Implement FTS5 for SQLite
7. **Add Rate Limiting**: Prevent API abuse
8. **Add Pagination**: For large result sets
