# Backend Migration: Sprint Capacity in Workspace Settings

## Overview

The frontend currently stores sprint capacity in localStorage. This is a temporary solution.
To make capacity sync across devices and be truly workspace-specific, we need backend support.

## Required Changes

### Database Migration

Add `sprint_capacity` column to `workspaces` table:

```sql
ALTER TABLE workspaces
ADD COLUMN sprint_capacity INTEGER;
```

### Model Update

Update `jility-server/src/models/workspace.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub sprint_capacity: Option<i32>,  // NEW
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### API Endpoints

Add two new endpoints:

**GET /api/workspaces/:slug/settings**
Returns workspace settings including sprint capacity.

**PATCH /api/workspaces/:slug/settings**
Updates workspace settings.

Request body:
```json
{
  "sprint_capacity": 80
}
```

### Frontend Update

Once backend is ready, update `jility-web/lib/use-sprint-capacity.ts`:

1. Remove localStorage logic
2. Call `api.getWorkspaceSettings(slug)` in useEffect
3. Call `api.updateWorkspaceSettings(slug, { sprint_capacity })` in updateCapacity
4. Add these methods to `jility-web/lib/api.ts`

## Testing

After backend migration:

1. Set capacity in workspace A
2. Log in from different device
3. Verify capacity is the same
4. Update capacity from device 2
5. Refresh device 1
6. Verify capacity updated
