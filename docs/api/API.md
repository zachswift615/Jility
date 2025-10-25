# Jility REST API Documentation

## Overview

The Jility server provides a REST API for managing tickets, projects, comments, and dependencies. It also includes WebSocket support for real-time updates.

**Base URL:** `http://localhost:3000`

**WebSocket:** `ws://localhost:3000/ws`

---

## Authentication

Currently, the API does not require authentication (marked as TODO in the code). All endpoints are accessible without tokens.

---

## Projects

### List Projects

```
GET /api/projects
```

**Response:**
```json
[
  {
    "id": "uuid",
    "name": "Project Name",
    "description": "Optional description",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
]
```

### Create Project

```
POST /api/projects
Content-Type: application/json

{
  "name": "My Project",
  "description": "Optional description"
}
```

**Response:**
```json
{
  "id": "uuid",
  "name": "My Project",
  "description": "Optional description",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

### Get Project

```
GET /api/projects/:id
```

**Response:** Same as create project response.

---

## Tickets

### List Tickets

```
GET /api/tickets?project_id={uuid}&status={status}&assignee={name}
```

**Query Parameters:**
- `project_id` (optional): Filter by project UUID
- `status` (optional): Filter by status (backlog, todo, in_progress, review, done, blocked)
- `assignee` (optional): Filter by assignee name

**Response:**
```json
[
  {
    "id": "uuid",
    "number": "TASK-1",
    "title": "Ticket Title",
    "description": "Ticket description",
    "status": "backlog",
    "story_points": 5,
    "assignees": ["alice", "agent-1"],
    "labels": ["backend", "feature"],
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z",
    "created_by": "alice",
    "parent_id": null,
    "epic_id": null
  }
]
```

### Create Ticket

```
POST /api/tickets
Content-Type: application/json

{
  "title": "Add user authentication",
  "description": "Implement JWT-based auth",
  "story_points": 5,
  "status": "todo",
  "assignees": ["alice", "agent-1"],
  "labels": ["backend", "feature"],
  "parent_id": null,
  "epic_id": null
}
```

**Response:** Same format as ticket in list response.

**WebSocket Broadcast:** Sends `TicketCreated` message to all connected clients.

### Get Ticket Details

```
GET /api/tickets/:id
```

**Response:**
```json
{
  "ticket": {
    "id": "uuid",
    "number": "TASK-1",
    "title": "Ticket Title",
    ...
  },
  "comments": [
    {
      "id": "uuid",
      "ticket_id": "uuid",
      "author": "alice",
      "content": "Comment text",
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": null
    }
  ],
  "dependencies": [
    {
      "id": "uuid",
      "number": "TASK-2",
      "title": "Dependency ticket",
      "status": "done"
    }
  ],
  "dependents": [...],
  "linked_commits": [
    {
      "id": "uuid",
      "commit_hash": "abc123",
      "commit_message": "Fix bug",
      "linked_at": "2024-01-01T00:00:00Z",
      "linked_by": "alice"
    }
  ],
  "recent_changes": [
    {
      "id": "uuid",
      "change_type": "status_changed",
      "field_name": "status",
      "old_value": "todo",
      "new_value": "in_progress",
      "changed_by": "alice",
      "changed_at": "2024-01-01T00:00:00Z",
      "message": null
    }
  ]
}
```

### Update Ticket

```
PUT /api/tickets/:id
Content-Type: application/json

{
  "title": "Updated title",
  "story_points": 8,
  "parent_id": "uuid",
  "epic_id": "uuid"
}
```

**Response:** Ticket response.

**WebSocket Broadcast:** Sends `TicketUpdated` message.

### Update Description

```
PATCH /api/tickets/:id/description
Content-Type: application/json

{
  "description": "New description text",
  "operation": "replace_all"
}
```

**Response:** Ticket response.

### Update Status

```
PATCH /api/tickets/:id/status
Content-Type: application/json

{
  "status": "in_progress"
}
```

**Valid statuses:** `backlog`, `todo`, `in_progress`, `review`, `done`, `blocked`

**Response:** Ticket response.

**WebSocket Broadcast:** Sends `StatusChanged` message.

### Assign Ticket

```
POST /api/tickets/:id/assign
Content-Type: application/json

{
  "assignee": "alice"
}
```

**Response:** Ticket response with updated assignees.

### Unassign Ticket

```
POST /api/tickets/:id/unassign
Content-Type: application/json

{
  "assignee": "alice"
}
```

**Response:** Ticket response with updated assignees.

### Delete Ticket

```
DELETE /api/tickets/:id
```

**Response:**
```json
{
  "success": true
}
```

---

## Comments

### List Comments

```
GET /api/tickets/:id/comments
```

**Response:**
```json
[
  {
    "id": "uuid",
    "ticket_id": "uuid",
    "author": "alice",
    "content": "Comment text",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": null
  }
]
```

### Create Comment

```
POST /api/tickets/:id/comments
Content-Type: application/json

{
  "content": "This is a comment"
}
```

**Response:** Comment object.

**WebSocket Broadcast:** Sends `CommentAdded` message.

### Update Comment

```
PUT /api/comments/:id
Content-Type: application/json

{
  "content": "Updated comment text"
}
```

**Response:** Comment object.

### Delete Comment

```
DELETE /api/comments/:id
```

**Response:**
```json
{
  "success": true
}
```

---

## Dependencies

### Add Dependency

```
POST /api/tickets/:id/dependencies
Content-Type: application/json

{
  "depends_on_id": "uuid"
}
```

**Response:**
```json
{
  "success": true
}
```

### Remove Dependency

```
DELETE /api/tickets/:id/dependencies/:dep_id
```

**Response:**
```json
{
  "success": true
}
```

### Get Dependency Graph

```
GET /api/tickets/:id/dependency-graph
```

**Response:**
```json
{
  "ticket": {
    "id": "uuid",
    "number": "TASK-1",
    "title": "Ticket Title",
    "status": "in_progress"
  },
  "dependencies": [...],
  "dependents": [...]
}
```

---

## Activity & History

### Get Activity Timeline

```
GET /api/tickets/:id/activity
```

Returns all changes to a ticket in chronological order.

**Response:**
```json
[
  {
    "id": "uuid",
    "change_type": "status_changed",
    "field_name": "status",
    "old_value": "todo",
    "new_value": "in_progress",
    "changed_by": "alice",
    "changed_at": "2024-01-01T00:00:00Z",
    "message": null
  }
]
```

### Get Description History

```
GET /api/tickets/:id/history
```

Returns all versions of the ticket description.

**Response:**
```json
[
  {
    "version": 1,
    "description": "Original description",
    "changed_by": "alice",
    "changed_at": "2024-01-01T00:00:00Z"
  }
]
```

### Get Specific Version

```
GET /api/tickets/:id/history/:version
```

**Response:**
```json
{
  "version": 1,
  "description": "Description at this version",
  "changed_by": "alice",
  "changed_at": "2024-01-01T00:00:00Z"
}
```

### Revert to Version

```
POST /api/tickets/:id/revert/:version
```

**Response:**
```json
{
  "success": true,
  "message": "Not implemented yet"
}
```

---

## Search

### Search Tickets

```
GET /api/search?q=authentication&limit=10
```

**Query Parameters:**
- `q` (required): Search query
- `limit` (optional): Maximum results

**Response:** Array of ticket responses.

**Note:** Currently only searches in ticket titles. Full-text search with FTS5 is marked as TODO.

---

## Git Integration

### Link Commit

```
POST /api/tickets/:id/commits
Content-Type: application/json

{
  "commit_hash": "abc123def456",
  "commit_message": "Fix authentication bug"
}
```

**Response:**
```json
{
  "id": "uuid",
  "commit_hash": "abc123def456",
  "commit_message": "Fix authentication bug",
  "linked_at": "2024-01-01T00:00:00Z",
  "linked_by": "alice"
}
```

### List Commits

```
GET /api/tickets/:id/commits
```

**Response:**
```json
[
  {
    "id": "uuid",
    "commit_hash": "abc123def456",
    "commit_message": "Fix authentication bug",
    "linked_at": "2024-01-01T00:00:00Z",
    "linked_by": "alice"
  }
]
```

---

## WebSocket

### Connection

Connect to `ws://localhost:3000/ws`

### Server Messages

The server broadcasts the following message types:

**TicketCreated:**
```json
{
  "type": "ticket_created",
  "ticket": { ... }
}
```

**TicketUpdated:**
```json
{
  "type": "ticket_updated",
  "ticket": { ... }
}
```

**StatusChanged:**
```json
{
  "type": "status_changed",
  "ticket_id": "uuid",
  "old_status": "todo",
  "new_status": "in_progress"
}
```

**CommentAdded:**
```json
{
  "type": "comment_added",
  "ticket_id": "uuid",
  "comment": { ... }
}
```

**DescriptionEdited:**
```json
{
  "type": "description_edited",
  "ticket_id": "uuid",
  "version": 2
}
```

---

## Error Responses

All endpoints return errors in the following format:

```json
{
  "error": "error_type",
  "message": "Human readable error message",
  "details": null
}
```

**Status Codes:**
- `400 Bad Request` - Invalid input or validation error
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Database or server error

---

## Change Types

The following change types are tracked in `ticket_changes`:

- `created` - Ticket created
- `title_changed` - Title updated
- `description_changed` - Description updated
- `status_changed` - Status changed
- `story_points_changed` - Story points changed
- `assignee_added` - Assignee added
- `assignee_removed` - Assignee removed
- `label_added` - Label added
- `label_removed` - Label removed
- `dependency_added` - Dependency added
- `dependency_removed` - Dependency removed
- `parent_changed` - Parent changed
- `epic_changed` - Epic changed
- `comment_added` - Comment added
- `commit_linked` - Commit linked
- `added_to_sprint` - Added to sprint
- `removed_from_sprint` - Removed from sprint
