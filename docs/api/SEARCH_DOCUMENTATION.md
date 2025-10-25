# Jility Full-Text Search System

This document describes the full-text search implementation for Jility, including backend architecture, API usage, and frontend components.

## Overview

Jility implements a powerful full-text search system that works with both SQLite (local development) and PostgreSQL (cloud/production). The system supports:

- **Full-text search** across ticket titles, descriptions, and comments
- **Advanced filtering** by status, assignee, labels, dates, story points, and more
- **Saved views** for frequently used search queries
- **Relevance ranking** with highlighted search results
- **Real-time search suggestions** in the navigation bar

## Architecture

### Database Layer

#### SQLite (FTS5)

For SQLite, we use FTS5 (Full-Text Search version 5) which provides:
- Porter stemming and Unicode tokenization
- BM25 ranking algorithm
- Snippet generation with highlighting
- Automatic sync via triggers

**FTS Tables:**
```sql
CREATE VIRTUAL TABLE tickets_fts USING fts5(
    ticket_id UNINDEXED,
    ticket_number UNINDEXED,
    title,
    description,
    content=tickets,
    tokenize='porter unicode61'
);

CREATE VIRTUAL TABLE comments_fts USING fts5(
    comment_id UNINDEXED,
    ticket_id UNINDEXED,
    author UNINDEXED,
    content,
    tokenize='porter unicode61'
);
```

**Automatic Triggers:**
```sql
-- Keeps FTS tables in sync with main tables
CREATE TRIGGER tickets_ai AFTER INSERT ON tickets ...
CREATE TRIGGER tickets_au AFTER UPDATE ON tickets ...
CREATE TRIGGER tickets_ad AFTER DELETE ON tickets ...
```

#### PostgreSQL (tsvector)

For PostgreSQL, we use built-in full-text search with tsvector:
- GIN indexes for fast searching
- English language stemming
- ts_rank for relevance scoring
- ts_headline for snippet generation

**Schema:**
```sql
ALTER TABLE tickets ADD COLUMN search_vector tsvector;
CREATE INDEX idx_tickets_search ON tickets USING GIN(search_vector);

CREATE FUNCTION tickets_search_trigger() RETURNS trigger AS $$
BEGIN
    NEW.search_vector :=
        setweight(to_tsvector('english', coalesce(NEW.title, '')), 'A') ||
        setweight(to_tsvector('english', coalesce(NEW.description, '')), 'B');
    RETURN NEW;
END
$$ LANGUAGE plpgsql;

CREATE TRIGGER tickets_search_update
    BEFORE INSERT OR UPDATE ON tickets
    FOR EACH ROW
    EXECUTE FUNCTION tickets_search_trigger();
```

### Backend Service Layer

**Location:** `/home/user/Jility/jility-core/src/search/`

The search service provides a unified API that abstracts over SQLite and PostgreSQL:

```rust
pub struct SearchService {
    db: Arc<DatabaseConnection>,
}

impl SearchService {
    pub async fn search_tickets(
        &self,
        filters: SearchFilters,
        limit: u64,
        offset: u64,
    ) -> Result<SearchResponse, JilityError>
}
```

**Features:**
- Database backend detection (SQLite vs PostgreSQL)
- Query escaping and sanitization
- Snippet generation with highlighting
- Relevance ranking
- Pagination support

### API Endpoints

**Base URL:** `http://localhost:3000/api`

#### Search Tickets
```
GET /api/search?q={query}&status={status}&assignees={assignee}&...
```

**Query Parameters:**
- `q` (required): Search query string
- `status`: Filter by status (can be multiple)
- `assignees`: Filter by assignees (can be multiple)
- `labels`: Filter by labels (can be multiple)
- `created_by`: Filter by creator username
- `created_after`: ISO 8601 date
- `created_before`: ISO 8601 date
- `updated_after`: ISO 8601 date
- `updated_before`: ISO 8601 date
- `min_points`: Minimum story points
- `max_points`: Maximum story points
- `has_comments`: true/false
- `has_commits`: true/false
- `has_dependencies`: true/false
- `epic_id`: UUID
- `parent_id`: UUID
- `project_id`: UUID
- `search_in`: Comma-separated list (title, description, comments)
- `limit`: Results per page (default: 20, max: 100)
- `offset`: Pagination offset (default: 0)

**Response:**
```json
{
  "results": [
    {
      "ticket_id": "uuid",
      "ticket_number": "TASK-123",
      "title": "Add authentication",
      "description": "Implement JWT-based authentication",
      "status": "in_progress",
      "story_points": 5,
      "snippet": "...implement JWT-based <mark>auth</mark>entication...",
      "rank": 0.85,
      "matched_in": ["title", "description"],
      "assignees": ["alice"],
      "labels": ["backend"],
      "created_by": "agent-1",
      "created_at": "2024-10-24T12:00:00Z",
      "updated_at": "2024-10-24T14:30:00Z"
    }
  ],
  "total": 42,
  "has_more": true,
  "offset": 0,
  "limit": 20
}
```

#### Saved Views

**List Saved Views**
```
GET /api/search/views
```

**Get Saved View**
```
GET /api/search/views/:id
```

**Create Saved View**
```
POST /api/search/views
Content-Type: application/json

{
  "name": "My Active Tasks",
  "description": "All my in-progress tasks",
  "filters": {
    "q": "",
    "status": ["in_progress"],
    "assignees": ["alice"]
  },
  "is_default": false,
  "is_shared": false
}
```

**Update Saved View**
```
PUT /api/search/views/:id
Content-Type: application/json

{
  "name": "Updated Name",
  "is_default": true
}
```

**Delete Saved View**
```
DELETE /api/search/views/:id
```

## Frontend Components

### SearchBar Component

**Location:** `/home/user/Jility/jility-web/components/search/search-bar.tsx`

A compact search bar for the navigation bar with:
- 300ms debounced search
- Live search suggestions (max 5 results)
- Highlighted snippets
- Click to navigate to ticket
- Link to advanced search

**Usage:**
```tsx
import { SearchBar } from '@/components/search/search-bar'

export function Navbar() {
  return (
    <nav>
      <SearchBar />
    </nav>
  )
}
```

### Advanced Search Page

**Location:** `/home/user/Jility/jility-web/app/search/page.tsx`

Full-featured search page with:
- Filter sidebar (status, assignees, story points, etc.)
- Saved views management
- Search results with pagination
- Applied filters display
- Save current search as view

**URL:** `/search?q={query}`

**Features:**
- URL-based state (shareable search links)
- Filter chips (removable)
- Load more pagination
- Relevance score display
- Click-to-navigate results

## Search Query Syntax

### Basic Search
- `authentication` - Match any field containing "authentication"
- `"user authentication"` - Exact phrase match

### Boolean Operators (PostgreSQL only)
- `auth AND jwt` - Both terms must be present
- `auth OR oauth` - Either term
- `auth NOT password` - Exclude "password"

### Wildcards (SQLite only)
- `auth*` - Starts with "auth"
- `*tion` - Ends with "tion"

### Filters (via query params)
- `?q=auth&status=todo` - Status filter
- `?q=auth&assignee=alice` - Assignee filter
- `?q=auth&created_after=2024-01-01` - Date filter

## Performance Optimization

### Database Indexes

**SQLite:**
- FTS5 automatically indexes all specified columns
- BM25 ranking is computed on-the-fly
- Triggers keep FTS tables in sync (minimal overhead)

**PostgreSQL:**
- GIN index on `search_vector` column
- Weighted ranking (title = 'A', description = 'B')
- Automatic updates via trigger

### Query Optimization

**Relevance Ranking:**
- Title matches rank higher than description
- Description matches rank higher than comments
- Combined ranking when matched in multiple locations

**Snippet Generation:**
- Limited to 50 words by default
- Highlights search terms with `<mark>` tags
- Shows contextual preview

**Pagination:**
- Default limit: 20 results
- Maximum limit: 100 results
- Offset-based pagination (suitable for most use cases)

### Caching (Future Enhancement)

Planned caching strategies:
- Popular search queries (Redis)
- Saved views metadata
- Search suggestions

## Database Compatibility

The search system works seamlessly with both databases:

| Feature | SQLite (FTS5) | PostgreSQL |
|---------|---------------|------------|
| Full-text search | BM25 | ts_rank |
| Stemming | Porter | English |
| Snippet highlighting | ✅ | ✅ |
| Phrase search | ✅ | ✅ |
| Boolean operators | Limited | Full support |
| Wildcards | ✅ | Limited |
| Performance | Excellent (small-medium) | Excellent (all sizes) |

## Migration

The search system is added via migration:

**File:** `/home/user/Jility/crates/jility-core/src/migration/m20241024_000002_add_fts.rs`

To apply:
```bash
# The migration runs automatically on server start
cargo run -p jility-server

# Or run migrations explicitly
sea-orm-cli migrate up
```

The migration:
1. Creates FTS virtual tables (SQLite) or adds tsvector columns (PostgreSQL)
2. Creates indexes for fast searching
3. Sets up automatic triggers to keep search indexes in sync
4. Creates saved_views table
5. Populates initial FTS data from existing tickets

## Usage Examples

### Basic Search (cURL)
```bash
# Simple search
curl "http://localhost:3000/api/search?q=authentication"

# With filters
curl "http://localhost:3000/api/search?q=auth&status=todo&status=in_progress&assignees=alice&limit=10"

# Date range
curl "http://localhost:3000/api/search?q=bug&created_after=2024-10-01&created_before=2024-10-31"

# Story points range
curl "http://localhost:3000/api/search?q=feature&min_points=5&max_points=13"
```

### Create Saved View (cURL)
```bash
curl -X POST "http://localhost:3000/api/search/views" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "High Priority Bugs",
    "description": "All high priority bugs assigned to me",
    "filters": {
      "q": "bug",
      "status": ["todo", "in_progress"],
      "assignees": ["alice"],
      "min_points": 8
    },
    "is_default": false,
    "is_shared": false
  }'
```

### Frontend Usage

**Quick search in navbar:**
```tsx
import { SearchBar } from '@/components/search/search-bar'

<SearchBar />
```

**Advanced search:**
```tsx
import { useRouter } from 'next/navigation'

const router = useRouter()

// Navigate to search page with query
router.push('/search?q=authentication')

// Or use the API directly
import { api } from '@/lib/api'

const results = await api.advancedSearch({
  q: 'authentication',
  status: ['todo', 'in_progress'],
  assignees: ['alice'],
  limit: 20,
})
```

## Testing

### Manual Testing

1. **Create test data:**
```bash
# Create tickets with various content
curl -X POST http://localhost:3000/api/tickets \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Implement JWT authentication",
    "description": "Add JWT-based authentication for API endpoints",
    "status": "todo",
    "story_points": 5
  }'
```

2. **Test search:**
```bash
# Should find the ticket
curl "http://localhost:3000/api/search?q=authentication"

# Should rank by relevance
curl "http://localhost:3000/api/search?q=JWT"
```

3. **Test filters:**
```bash
# Combine search with filters
curl "http://localhost:3000/api/search?q=auth&status=todo&min_points=3"
```

4. **Test saved views:**
```bash
# Create a view
curl -X POST http://localhost:3000/api/search/views \
  -H "Content-Type: application/json" \
  -d '{"name": "Test View", "filters": {"q": "auth"}}'

# List views
curl http://localhost:3000/api/search/views
```

### Automated Testing (TODO)

Backend tests:
- FTS query generation
- Filter application
- Ranking correctness
- Snippet highlighting
- Saved views CRUD

Frontend tests:
- Search bar debouncing
- Filter application
- Saved views loading
- Pagination

## Troubleshooting

### No search results

1. Check if FTS tables/columns exist:
```sql
-- SQLite
SELECT * FROM sqlite_master WHERE name LIKE '%fts%';

-- PostgreSQL
SELECT column_name FROM information_schema.columns
WHERE table_name = 'tickets' AND column_name = 'search_vector';
```

2. Verify migration ran:
```bash
# Check migration status
sea-orm-cli migrate status
```

3. Check if data is indexed:
```sql
-- SQLite
SELECT COUNT(*) FROM tickets_fts;

-- PostgreSQL
SELECT COUNT(*) FROM tickets WHERE search_vector IS NOT NULL;
```

### Slow searches

1. Check indexes exist:
```sql
-- PostgreSQL
SELECT indexname FROM pg_indexes WHERE tablename = 'tickets';
```

2. Analyze query performance:
```sql
-- PostgreSQL
EXPLAIN ANALYZE SELECT * FROM tickets WHERE search_vector @@ plainto_tsquery('english', 'auth');

-- SQLite
EXPLAIN QUERY PLAN SELECT * FROM tickets_fts WHERE tickets_fts MATCH 'auth';
```

3. Consider limiting result set or adding more specific filters

### Triggers not working

1. Verify triggers exist:
```sql
-- SQLite
SELECT name FROM sqlite_master WHERE type = 'trigger';

-- PostgreSQL
SELECT trigger_name FROM information_schema.triggers WHERE event_object_table = 'tickets';
```

2. Manually rebuild FTS data:
```sql
-- SQLite
DELETE FROM tickets_fts;
INSERT INTO tickets_fts(ticket_id, ticket_number, title, description)
SELECT id, ticket_number, title, description FROM tickets;

-- PostgreSQL
UPDATE tickets SET search_vector =
    setweight(to_tsvector('english', coalesce(title, '')), 'A') ||
    setweight(to_tsvector('english', coalesce(description, '')), 'B');
```

## Future Enhancements

### Planned Features
- [ ] Advanced query syntax parser
- [ ] Search within specific projects
- [ ] Fuzzy matching for typos
- [ ] Search in linked commits
- [ ] Export search results to CSV
- [ ] Search analytics and popular queries
- [ ] Multi-language support
- [ ] Voice search

### Performance Improvements
- [ ] Redis caching for popular queries
- [ ] Cursor-based pagination for large result sets
- [ ] Background indexing for bulk operations
- [ ] Search result prefetching

### UI Enhancements
- [ ] Search history
- [ ] Keyboard shortcuts (Cmd+K)
- [ ] Search filters in URL for sharing
- [ ] Advanced query builder UI
- [ ] Faceted search (show filter counts)

## Related Files

### Backend
- `/home/user/Jility/crates/jility-core/src/migration/m20241024_000002_add_fts.rs` - Migration
- `/home/user/Jility/jility-core/src/search/` - Search service
- `/home/user/Jility/jility-core/src/entities/saved_view.rs` - Saved view entity
- `/home/user/Jility/jility-server/src/api/search.rs` - API endpoints
- `/home/user/Jility/jility-server/src/state.rs` - App state with search service

### Frontend
- `/home/user/Jility/jility-web/components/search/search-bar.tsx` - Quick search
- `/home/user/Jility/jility-web/app/search/page.tsx` - Advanced search page
- `/home/user/Jility/jility-web/lib/api.ts` - API client
- `/home/user/Jility/jility-web/lib/types.ts` - TypeScript types
- `/home/user/Jility/jility-web/lib/hooks.ts` - Debounce hook

## Support

For issues or questions:
1. Check this documentation
2. Review the troubleshooting section
3. Check backend logs: `tail -f logs/jility-server.log`
4. Enable debug logging: `RUST_LOG=debug cargo run`
