# Jility Server - Quick Start Guide

## TL;DR

```bash
# Start the server
cd jility-server
export DATABASE_URL="sqlite://.jility/data.db?mode=rwc"
cargo run

# Server runs on http://localhost:3000
```

## Quick Test

```bash
# Create a ticket
curl -X POST http://localhost:3000/api/tickets \
  -H "Content-Type: application/json" \
  -d '{"title": "Test ticket", "status": "todo"}'

# List tickets
curl http://localhost:3000/api/tickets

# Get ticket details
curl http://localhost:3000/api/tickets/:id
```

## What's Included

✅ **26 REST API endpoints** - Complete CRUD for tickets, comments, dependencies, etc.
✅ **Event sourcing** - Full audit trail in ticket_changes table
✅ **WebSocket** - Real-time updates at ws://localhost:3000/ws
✅ **Documentation** - See API.md and TESTING.md

## Key Files

| File | Purpose |
|------|---------|
| **API.md** | Complete REST API documentation |
| **TESTING.md** | Testing guide with curl examples |
| **SERVER_IMPLEMENTATION_SUMMARY.md** | Full implementation details |
| **DELIVERABLES.md** | Summary of what was built |
| **jility-server/README.md** | Server architecture |

## API Endpoints Summary

### Projects
- GET /api/projects - List
- POST /api/projects - Create
- GET /api/projects/:id - Get

### Tickets (Core)
- GET /api/tickets - List with filters
- POST /api/tickets - Create
- GET /api/tickets/:id - Get details
- PUT /api/tickets/:id - Update
- DELETE /api/tickets/:id - Delete

### Tickets (Actions)
- PATCH /api/tickets/:id/description - Update description
- PATCH /api/tickets/:id/status - Change status
- POST /api/tickets/:id/assign - Assign user
- POST /api/tickets/:id/unassign - Remove user

### Comments
- GET /api/tickets/:id/comments - List
- POST /api/tickets/:id/comments - Create
- PUT /api/comments/:id - Update
- DELETE /api/comments/:id - Delete

### Dependencies
- POST /api/tickets/:id/dependencies - Add
- DELETE /api/tickets/:id/dependencies/:dep_id - Remove
- GET /api/tickets/:id/dependency-graph - Get graph

### Activity & History
- GET /api/tickets/:id/activity - Timeline
- GET /api/tickets/:id/history - Versions
- GET /api/tickets/:id/history/:version - Get version
- POST /api/tickets/:id/revert/:version - Revert

### Search & Git
- GET /api/search?q=query - Search
- POST /api/tickets/:id/commits - Link commit
- GET /api/tickets/:id/commits - List commits

### WebSocket
- WS /ws - Real-time updates

## Architecture

```
jility/
├── jility-core/          # Shared database entities (11 models)
│   └── src/entities/     # Project, Ticket, Comment, etc.
└── jility-server/        # Axum web server
    └── src/
        ├── api/          # 7 endpoint modules
        ├── models/       # Request/response types
        └── websocket/    # Real-time updates
```

## Event Sourcing

All ticket changes are recorded in `ticket_changes` table:
- Created, status changed, description changed
- Assignees added/removed, labels added/removed
- Comments added, commits linked
- Full audit trail for humans and agents

## WebSocket Messages

Server broadcasts:
- ticket_created
- ticket_updated
- status_changed
- comment_added
- description_edited

## Next Steps

1. **Test locally** - Use curl examples in TESTING.md
2. **Add authentication** - Implement JWT (marked TODO)
3. **Create migrations** - Database schema setup
4. **Add frontend** - Connect Next.js app
5. **Deploy** - Add PostgreSQL and production config

## TODO

- [ ] JWT authentication
- [ ] Database migrations tool
- [ ] Full-text search (FTS5)
- [ ] Pagination
- [ ] Rate limiting
- [ ] Input validation
- [ ] Production hardening

## Documentation

For detailed information:
- **API Reference:** API.md (9.6 KB)
- **Testing Guide:** TESTING.md (9.6 KB)
- **Implementation Details:** SERVER_IMPLEMENTATION_SUMMARY.md (14 KB)
- **Deliverables:** DELIVERABLES.md (13 KB)

## Support

All endpoints return structured JSON errors:
```json
{
  "error": "error_type",
  "message": "Human readable message",
  "details": null
}
```

Status codes: 400 (bad request), 404 (not found), 500 (server error)
