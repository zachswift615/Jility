# Jility Server

Fast, event-sourced REST API server for the Jility project management system.

## Features

- **Full REST API** - Complete CRUD for projects, tickets, comments, dependencies
- **Event Sourcing** - All changes tracked in `ticket_changes` table for full auditability
- **WebSocket Support** - Real-time updates for collaborative editing
- **Fast** - Built with Axum and async Rust
- **Database Agnostic** - Works with SQLite (local) or PostgreSQL (production)

## Tech Stack

- **Framework**: Axum 0.7
- **ORM**: SeaORM 0.12
- **Database**: SQLite / PostgreSQL
- **Async Runtime**: Tokio
- **Serialization**: Serde
- **WebSocket**: Axum built-in

## Project Structure

```
jility-server/
├── src/
│   ├── main.rs              # Server entry point
│   ├── state.rs             # AppState and database connection
│   ├── error.rs             # Error types and handling
│   ├── models/              # Request/response types
│   │   ├── mod.rs
│   │   ├── request.rs
│   │   └── response.rs
│   ├── api/                 # REST API endpoints
│   │   ├── mod.rs           # Router setup
│   │   ├── projects.rs      # Project endpoints
│   │   ├── tickets.rs       # Ticket CRUD
│   │   ├── comments.rs      # Comment management
│   │   ├── dependencies.rs  # Dependency graph
│   │   ├── activity.rs      # History and timeline
│   │   ├── search.rs        # Full-text search
│   │   └── git.rs           # Git integration
│   └── websocket/           # WebSocket handlers
│       └── mod.rs
└── Cargo.toml
```

## API Endpoints

### Projects
- `GET /api/projects` - List all projects
- `POST /api/projects` - Create new project
- `GET /api/projects/:id` - Get project details

### Tickets
- `GET /api/tickets` - List tickets (with filters)
- `POST /api/tickets` - Create ticket
- `GET /api/tickets/:id` - Get ticket details (with full context)
- `PUT /api/tickets/:id` - Update ticket metadata
- `DELETE /api/tickets/:id` - Delete ticket
- `PATCH /api/tickets/:id/description` - Update description
- `PATCH /api/tickets/:id/status` - Change status
- `POST /api/tickets/:id/assign` - Assign/unassign user
- `POST /api/tickets/:id/unassign` - Remove assignee

### Comments
- `GET /api/tickets/:id/comments` - List comments
- `POST /api/tickets/:id/comments` - Add comment
- `PUT /api/comments/:id` - Update comment
- `DELETE /api/comments/:id` - Delete comment

### Dependencies
- `POST /api/tickets/:id/dependencies` - Add dependency
- `DELETE /api/tickets/:id/dependencies/:dep_id` - Remove dependency
- `GET /api/tickets/:id/dependency-graph` - Get dependency graph

### Activity & History
- `GET /api/tickets/:id/activity` - Get activity timeline
- `GET /api/tickets/:id/history` - Get description version history
- `GET /api/tickets/:id/history/:version` - Get specific version
- `POST /api/tickets/:id/revert/:version` - Revert to version

### Search
- `GET /api/search?q=<query>` - Full-text search (currently title only)

### Git Integration
- `POST /api/tickets/:id/commits` - Link commit to ticket
- `GET /api/tickets/:id/commits` - List linked commits

### WebSocket
- `WS /ws` - Real-time updates

## WebSocket Messages

The server broadcasts these message types to all connected clients:

```typescript
type ServerMessage =
  | { type: "ticket_created", ticket: Ticket }
  | { type: "ticket_updated", ticket: Ticket }
  | { type: "status_changed", ticket_id: string, old_status: string, new_status: string }
  | { type: "comment_added", ticket_id: string, comment: Comment }
  | { type: "description_edited", ticket_id: string, version: number }
```

## Event Sourcing

All ticket changes are recorded in the `ticket_changes` table with:
- Change type (created, status_changed, description_changed, etc.)
- Old value and new value
- Who made the change and when
- Optional context message

This enables:
- Full audit trail
- Time-travel debugging
- Version history for descriptions
- Activity timelines

## Running the Server

### Development

```bash
# Set database URL (optional, defaults to SQLite)
export DATABASE_URL="sqlite://.jility/data.db?mode=rwc"

# Start server
cargo run

# Server starts on http://localhost:3000
```

### Production

```bash
# Use PostgreSQL
export DATABASE_URL="postgres://user:pass@localhost/jility"
export BIND_ADDRESS="0.0.0.0:8080"

# Run with release optimizations
cargo run --release
```

## Configuration

Environment variables:

- `DATABASE_URL` - Database connection string (default: `sqlite://.jility/data.db?mode=rwc`)
- `BIND_ADDRESS` - Server bind address (default: `0.0.0.0:3000`)
- `RUST_LOG` - Logging level (default: `jility_server=debug,tower_http=debug`)

## Middleware

The server includes:

1. **CORS** - Permissive CORS for development (configure for production)
2. **Tracing** - HTTP request/response logging
3. **Error Handling** - Converts errors to proper HTTP responses

## Error Handling

All API errors return this format:

```json
{
  "error": "error_type",
  "message": "Human readable error message",
  "details": null
}
```

Status codes:
- `400 Bad Request` - Invalid input
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Database or server error

## Database Schema

See `../docs/database-schema-design.md` for full schema details.

Key entities:
- `projects` - Project metadata
- `tickets` - Ticket data (current state)
- `ticket_changes` - Event log (all changes)
- `ticket_assignees` - Many-to-many assignees
- `ticket_labels` - Many-to-many labels
- `ticket_dependencies` - Dependency graph
- `comments` - Ticket comments
- `commit_links` - Git commit links
- `sprints` - Sprint management (Phase 4)

## TODO

- [ ] Add authentication (JWT)
- [ ] Create database migration tool
- [ ] Implement full-text search (FTS5/PostgreSQL)
- [ ] Add input validation
- [ ] Add pagination for large result sets
- [ ] Optimize queries (N+1 prevention)
- [ ] Add rate limiting
- [ ] Add request caching
- [ ] Add metrics/monitoring
- [ ] Add health check endpoint
- [ ] Implement revert functionality
- [ ] Add batch operations
- [ ] Add filtering/sorting options

## Testing

See `../TESTING.md` for comprehensive testing guide.

Quick test:

```bash
# Create a ticket
curl -X POST http://localhost:3000/api/tickets \
  -H "Content-Type: application/json" \
  -d '{"title": "Test ticket", "status": "todo"}'

# List tickets
curl http://localhost:3000/api/tickets
```

## Contributing

This is part of the Jility project. See main README for contribution guidelines.

## License

TBD (MIT or Apache 2.0)
