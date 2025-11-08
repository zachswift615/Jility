# Configurable Sprint Capacity Implementation Plan

> **Status:** ✅ COMPLETE (JIL-23, Implemented 2025-01-08)
> **Note:** Currently uses localStorage. See JIL-26 for backend persistence.

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Make sprint capacity configurable per workspace, with intelligent defaults based on team velocity history. Replace the hardcoded 70-point capacity with a user-editable setting.

**Architecture:** Add a workspace settings page with sprint capacity configuration. Store the capacity value in workspace settings. Default to team's average velocity from completed sprints, or 40 points if no history exists. Display capacity in sprint planning with an edit button.

**Tech Stack:** Next.js 14 App Router, React hooks, backend workspace API, existing sprint history calculations

---

## Task 1: Add Workspace Settings Backend Support

**Files:**
- Check: `jility-server/src/models/workspace.rs`
- Check: `jility-server/src/handlers/workspace.rs`

**Step 1: Check if workspace model has settings field**

Read the workspace model:

```bash
cat jility-server/src/models/workspace.rs | grep -A 5 "struct Workspace"
```

Expected: Check if there's a `settings` or `sprint_capacity` field.

**Step 2: If no settings field exists, plan backend changes**

Note: The backend likely needs a migration to add workspace settings. Since this plan is frontend-focused, we'll document the backend requirement:

**Backend Requirements (separate task):**
- Add `sprint_capacity: Option<i32>` to Workspace model
- Add migration to add `sprint_capacity` column to workspaces table
- Add `PATCH /api/workspaces/:id/settings` endpoint to update capacity
- Add `GET /api/workspaces/:id/settings` endpoint to fetch settings

**For this plan, we'll mock the backend and use localStorage for now.**

**Step 3: Document backend dependency**

Create a note file:

```bash
echo "Sprint Capacity backend changes needed:
1. Add sprint_capacity column to workspaces table (nullable integer)
2. Add PATCH /api/workspaces/:slug/settings endpoint
3. Add GET /api/workspaces/:slug/settings endpoint
4. Add WorkspaceSettings type to API responses

Once backend is ready, replace localStorage implementation in frontend." > docs/plans/2025-11-08-sprint-capacity-backend-todo.md
```

---

## Task 2: Add Workspace Settings Types

**Files:**
- Modify: `jility-web/lib/types.ts`

**Step 1: Add WorkspaceSettings type**

Append to the end of the types file:

```typescript
export interface WorkspaceSettings {
  sprint_capacity?: number
}

export interface WorkspaceWithSettings extends Workspace {
  settings: WorkspaceSettings
}
```

**Step 2: Commit the types**

```bash
git add jility-web/lib/types.ts
git commit -m "feat: add workspace settings types for sprint capacity"
```

---

## Task 3: Add Capacity Storage Hook (Temporary localStorage)

**Files:**
- Create: `jility-web/lib/use-sprint-capacity.ts`

**Step 1: Create a hook for managing sprint capacity**

```typescript
'use client'

import { useState, useEffect, useCallback } from 'react'
import { useWorkspace } from './workspace-context'
import { api } from './api'

/**
 * Hook for managing sprint capacity setting.
 *
 * Currently uses localStorage as a temporary solution.
 * TODO: Replace with backend API calls when workspace settings endpoint is ready.
 */
export function useSprintCapacity() {
  const { currentWorkspace } = useWorkspace()
  const [capacity, setCapacity] = useState<number | null>(null)
  const [loading, setLoading] = useState(true)

  const slug = currentWorkspace?.slug || ''

  // Calculate default capacity from team velocity
  const calculateDefaultCapacity = useCallback(async (): Promise<number> => {
    if (!slug) return 40

    try {
      const history = await api.getSprintHistory(slug)
      if (history.average_velocity > 0) {
        // Use average velocity as default capacity
        return Math.round(history.average_velocity)
      }
    } catch (error) {
      console.error('Failed to fetch sprint history:', error)
    }

    // Default to 40 points if no history
    return 40
  }, [slug])

  // Load capacity from localStorage or calculate default
  useEffect(() => {
    const loadCapacity = async () => {
      if (!slug) {
        setLoading(false)
        return
      }

      // Try to get from localStorage first
      const storageKey = `sprint-capacity-${slug}`
      const stored = localStorage.getItem(storageKey)

      if (stored) {
        setCapacity(parseInt(stored, 10))
      } else {
        // Calculate default from team velocity
        const defaultCapacity = await calculateDefaultCapacity()
        setCapacity(defaultCapacity)
      }

      setLoading(false)
    }

    loadCapacity()
  }, [slug, calculateDefaultCapacity])

  // Update capacity
  const updateCapacity = useCallback(
    async (newCapacity: number) => {
      if (!slug) return

      // TODO: Replace with API call when backend is ready
      // await api.updateWorkspaceSettings(slug, { sprint_capacity: newCapacity })

      // For now, use localStorage
      const storageKey = `sprint-capacity-${slug}`
      localStorage.setItem(storageKey, newCapacity.toString())
      setCapacity(newCapacity)
    },
    [slug]
  )

  return {
    capacity,
    updateCapacity,
    loading,
  }
}
```

**Step 2: Commit the hook**

```bash
git add jility-web/lib/use-sprint-capacity.ts
git commit -m "feat: add sprint capacity hook with localStorage (temporary)"
```

---

## Task 4: Create Capacity Editor Component

**Files:**
- Create: `jility-web/components/sprint/capacity-editor.tsx`

**Step 1: Create the inline editor component**

```typescript
'use client'

import { useState } from 'react'
import { Pencil, Check, X } from 'lucide-react'

interface CapacityEditorProps {
  capacity: number
  onSave: (newCapacity: number) => Promise<void>
}

export function CapacityEditor({ capacity, onSave }: CapacityEditorProps) {
  const [editing, setEditing] = useState(false)
  const [value, setValue] = useState(capacity.toString())
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState('')

  const handleSave = async () => {
    const parsed = parseInt(value, 10)

    if (isNaN(parsed) || parsed < 1) {
      setError('Capacity must be a positive number')
      return
    }

    setSaving(true)
    setError('')

    try {
      await onSave(parsed)
      setEditing(false)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to save')
    } finally {
      setSaving(false)
    }
  }

  const handleCancel = () => {
    setValue(capacity.toString())
    setError('')
    setEditing(false)
  }

  if (!editing) {
    return (
      <button
        onClick={() => setEditing(true)}
        className="inline-flex items-center gap-1 text-sm font-medium hover:text-primary transition-colors"
        title="Edit capacity"
      >
        <span>{capacity} pts</span>
        <Pencil className="h-3 w-3" />
      </button>
    )
  }

  return (
    <div className="inline-flex items-center gap-2">
      <div className="flex flex-col">
        <div className="flex items-center gap-1">
          <input
            type="number"
            value={value}
            onChange={(e) => setValue(e.target.value)}
            className="w-20 px-2 py-1 text-sm bg-background border-input border rounded"
            min="1"
            autoFocus
            onKeyDown={(e) => {
              if (e.key === 'Enter') handleSave()
              if (e.key === 'Escape') handleCancel()
            }}
          />
          <span className="text-sm text-muted-foreground">pts</span>
          <button
            onClick={handleSave}
            disabled={saving}
            className="p-1 text-green-600 hover:bg-green-100 dark:hover:bg-green-900 rounded disabled:opacity-50"
            title="Save"
          >
            <Check className="h-4 w-4" />
          </button>
          <button
            onClick={handleCancel}
            disabled={saving}
            className="p-1 text-destructive hover:bg-destructive/10 rounded disabled:opacity-50"
            title="Cancel"
          >
            <X className="h-4 w-4" />
          </button>
        </div>
        {error && (
          <span className="text-xs text-destructive mt-1">{error}</span>
        )}
      </div>
    </div>
  )
}
```

**Step 2: Commit the component**

```bash
git add jility-web/components/sprint/capacity-editor.tsx
git commit -m "feat: add inline capacity editor component"
```

---

## Task 5: Update Sprint Planning Page

**Files:**
- Modify: `jility-web/app/w/[slug]/sprint/planning/page.tsx`

**Step 1: Replace hardcoded capacity with hook**

Add import at the top:

```typescript
import { useSprintCapacity } from '@/lib/use-sprint-capacity'
import { CapacityEditor } from '@/components/sprint/capacity-editor'
```

Replace the hardcoded capacity line:

```typescript
  // OLD (remove this):
  // const capacity = 70 // TODO: Make configurable

  // NEW:
  const { capacity: workspaceCapacity, updateCapacity, loading: capacityLoading } = useSprintCapacity()
  const capacity = workspaceCapacity || 40
```

**Step 2: Update capacity display to use editor**

Find the capacity indicator section and update it:

```typescript
              {/* Capacity Indicator */}
              <div className="bg-card rounded-lg p-4 md:p-6 border-border border">
                <div className="flex items-center justify-between mb-2 text-sm md:text-base">
                  <span className="font-medium">
                    Capacity:{' '}
                    {capacityLoading ? (
                      <span className="text-muted-foreground">Loading...</span>
                    ) : (
                      <CapacityEditor capacity={capacity} onSave={updateCapacity} />
                    )}
                  </span>
                  <span className="font-medium">Planned: {plannedPoints} pts</span>
                  <span className={`font-medium ${
                    capacityPercentage > 100 ? 'text-destructive' :
                    capacityPercentage > 80 ? 'text-yellow-600' :
                    'text-green-600'
                  }`}>
                    {capacityPercentage}%
                  </span>
                </div>
                <div className="w-full bg-secondary rounded-full h-2">
                  <div
                    className={`h-2 rounded-full transition-all ${
                      capacityPercentage > 100 ? 'bg-destructive' :
                      capacityPercentage > 80 ? 'bg-yellow-600' :
                      'bg-green-600'
                    }`}
                    style={{ width: `${Math.min(capacityPercentage, 100)}%` }}
                  />
                </div>
              </div>
```

**Step 3: Verify changes compile**

Run: `cd jility-web && npm run build`
Expected: Build succeeds with no TypeScript errors

**Step 4: Commit the changes**

```bash
git add jility-web/app/w/[slug]/sprint/planning/page.tsx
git commit -m "feat: replace hardcoded capacity with editable setting"
```

---

## Task 6: Add Capacity Info Tooltip

**Files:**
- Modify: `jility-web/app/w/[slug]/sprint/planning/page.tsx`

**Step 1: Add helpful tooltip explaining capacity**

Add an info icon next to the capacity label:

```typescript
import { Plus, Info } from 'lucide-react'
```

Update the capacity indicator:

```typescript
              <div className="bg-card rounded-lg p-4 md:p-6 border-border border">
                <div className="flex items-center justify-between mb-2 text-sm md:text-base">
                  <div className="flex items-center gap-2">
                    <span className="font-medium">
                      Capacity:{' '}
                      {capacityLoading ? (
                        <span className="text-muted-foreground">Loading...</span>
                      ) : (
                        <CapacityEditor capacity={capacity} onSave={updateCapacity} />
                      )}
                    </span>
                    <div className="group relative">
                      <Info className="h-4 w-4 text-muted-foreground cursor-help" />
                      <div className="absolute left-0 top-6 w-64 p-2 bg-popover text-popover-foreground border border-border rounded shadow-lg opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-10">
                        <p className="text-xs">
                          Sprint capacity is the target amount of work (in story points) your team can complete in one sprint.
                          {workspaceCapacity && workspaceCapacity !== 40 && (
                            <> Defaults to your team's average velocity from past sprints.</>
                          )}
                        </p>
                      </div>
                    </div>
                  </div>
                  <span className="font-medium">Planned: {plannedPoints} pts</span>
                  <span className={`font-medium ${
                    capacityPercentage > 100 ? 'text-destructive' :
                    capacityPercentage > 80 ? 'text-yellow-600' :
                    'text-green-600'
                  }`}>
                    {capacityPercentage}%
                  </span>
                </div>
                <div className="w-full bg-secondary rounded-full h-2">
                  <div
                    className={`h-2 rounded-full transition-all ${
                      capacityPercentage > 100 ? 'bg-destructive' :
                      capacityPercentage > 80 ? 'bg-yellow-600' :
                      'bg-green-600'
                    }`}
                    style={{ width: `${Math.min(capacityPercentage, 100)}%` }}
                  />
                </div>
              </div>
```

**Step 2: Commit the tooltip**

```bash
git add jility-web/app/w/[slug]/sprint/planning/page.tsx
git commit -m "feat: add info tooltip explaining sprint capacity"
```

---

## Task 7: Manual Testing

**Step 1: Start the development server**

Run: `./dev.sh start-frontend` or `cd jility-web && npm run dev`

**Step 2: Test default capacity (no sprint history)**

1. Create a new workspace with no completed sprints
2. Navigate to Sprint Planning
3. Verify: Capacity defaults to 40 points
4. Verify: Info icon appears next to capacity

**Step 3: Test default capacity (with sprint history)**

1. In a workspace with completed sprints
2. Navigate to Sprint Planning
3. Verify: Capacity defaults to team's average velocity
4. Example: If average velocity is 65, capacity should be 65

**Step 4: Test capacity editor**

1. Hover over the capacity value
2. Verify: Pencil icon appears
3. Click the capacity value
4. Verify: Input field appears with current value
5. Type a new value (e.g., 80)
6. Press Enter or click checkmark
7. Verify: Capacity updates immediately
8. Verify: Capacity percentage recalculates
9. Refresh the page
10. Verify: New capacity persists

**Step 5: Test capacity validation**

1. Click to edit capacity
2. Enter an invalid value (e.g., 0, -5, or "abc")
3. Try to save
4. Verify: Error message appears
5. Verify: Capacity is not updated
6. Click X to cancel
7. Verify: Editor closes without saving

**Step 6: Test capacity indicator colors**

1. Set capacity to 100
2. Add tickets totaling 50 points
3. Verify: Progress bar is green (50%)
4. Add tickets totaling 85 points
5. Verify: Progress bar is yellow (85%)
6. Add tickets totaling 120 points
7. Verify: Progress bar is red (120%)

**Step 7: Test info tooltip**

1. Hover over the info icon next to capacity
2. Verify: Tooltip appears explaining sprint capacity
3. Verify: Tooltip mentions average velocity if applicable

**Step 8: Test across workspaces**

1. Set capacity to 80 in workspace A
2. Navigate to workspace B
3. Verify: Capacity is different (not 80)
4. Set capacity to 60 in workspace B
5. Navigate back to workspace A
6. Verify: Capacity is still 80

---

## Task 8: Documentation Update

**Files:**
- Modify: `docs/features/sprint-planning.md`

**Step 1: Document configurable capacity**

Add to the sprint planning documentation:

```markdown
## Sprint Capacity

Sprint capacity represents the target amount of work (in story points) your team can realistically complete in one sprint.

### Default Capacity

When you first use sprint planning, Jility automatically sets capacity based on your team's historical performance:

- **With sprint history**: Defaults to your team's average velocity from completed sprints
- **No sprint history**: Defaults to 40 points

### Editing Capacity

You can customize sprint capacity at any time:

1. Navigate to Sprint Planning
2. Click the capacity value (displayed as "X pts")
3. Enter your desired capacity
4. Press Enter or click the checkmark to save

The capacity setting is workspace-specific and persists across sessions.

### Capacity Indicator

The capacity indicator shows how much work is planned for the sprint:

- **Green (0-80%)**: Comfortable sprint load
- **Yellow (80-100%)**: Approaching capacity
- **Red (>100%)**: Over capacity - consider removing tickets

### Best Practices

- Review capacity at the start of each sprint
- Adjust based on team availability (holidays, vacation, etc.)
- Use historical velocity as a guide, not a hard rule
- Leave some buffer capacity for unexpected work
```

**Step 2: Commit documentation**

```bash
git add docs/features/sprint-planning.md
git commit -m "docs: document configurable sprint capacity feature"
```

---

## Task 9: Add Backend Migration Note

**Files:**
- Create: `docs/plans/backend-migration-sprint-capacity.md`

**Step 1: Document the backend migration needed**

```markdown
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
```

**Step 2: Commit the migration plan**

```bash
git add docs/plans/backend-migration-sprint-capacity.md
git commit -m "docs: add backend migration plan for sprint capacity"
```

---

## Verification Checklist

- [ ] WorkspaceSettings types added
- [ ] useSprintCapacity hook created with localStorage fallback
- [ ] CapacityEditor component created with inline editing
- [ ] Sprint planning page uses hook instead of hardcoded value
- [ ] Capacity defaults to team velocity or 40 points
- [ ] Capacity can be edited inline
- [ ] Capacity validation prevents invalid values
- [ ] Capacity persists across page refreshes
- [ ] Capacity is workspace-specific
- [ ] Info tooltip explains capacity concept
- [ ] Capacity indicator colors update correctly
- [ ] No TypeScript errors in build
- [ ] Feature works on mobile and desktop layouts
- [ ] Documentation updated
- [ ] Backend migration plan documented

## Notes

- Current implementation uses localStorage as a temporary solution
- Once backend workspace settings API is available, replace localStorage with API calls
- Default capacity calculation from team velocity helps new teams start with realistic estimates
- Inline editing provides quick access without needing a separate settings page
- Consider adding capacity to workspace settings page when that feature is built
