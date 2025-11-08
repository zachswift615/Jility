# Board Active Sprint Filter Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Filter the Kanban board to show only tickets from the active sprint (like JIRA), with an option to toggle back to "All Tickets" view.

**Architecture:** Add a sprint filter toggle to the board page toolbar. Fetch the active sprint and filter tickets by sprint_id. Store filter preference in URL query params for deep linking.

**Tech Stack:** Next.js 14 App Router, React hooks, existing API endpoints, URL search params

---

## Task 1: Add Sprint Filter Toggle Component

**Files:**
- Create: `jility-web/components/board/sprint-filter.tsx`

**Step 1: Create the sprint filter toggle component**

```typescript
'use client'

import { useSearchParams, useRouter } from 'next/navigation'
import { useWorkspace } from '@/lib/workspace-context'

interface SprintFilterProps {
  hasActiveSprint: boolean
}

export function SprintFilter({ hasActiveSprint }: SprintFilterProps) {
  const searchParams = useSearchParams()
  const router = useRouter()
  const { currentWorkspace } = useWorkspace()
  const slug = currentWorkspace?.slug || ''

  const showActiveSprint = searchParams.get('sprint') === 'active'

  const toggleFilter = () => {
    const params = new URLSearchParams(searchParams.toString())

    if (showActiveSprint) {
      params.delete('sprint')
    } else {
      params.set('sprint', 'active')
    }

    router.push(`/w/${slug}/board?${params.toString()}`)
  }

  if (!hasActiveSprint) {
    return null
  }

  return (
    <button
      onClick={toggleFilter}
      className={`px-3 py-1.5 rounded-md text-sm font-medium transition-colors ${
        showActiveSprint
          ? 'bg-primary text-primary-foreground'
          : 'bg-secondary text-secondary-foreground hover:bg-secondary/80'
      }`}
    >
      {showActiveSprint ? 'Active Sprint' : 'All Tickets'}
    </button>
  )
}
```

**Step 2: Commit the new component**

```bash
git add jility-web/components/board/sprint-filter.tsx
git commit -m "feat: add sprint filter toggle component for board"
```

---

## Task 2: Update Board Page to Support Sprint Filtering

**Files:**
- Modify: `jility-web/app/w/[slug]/board/page.tsx`

**Step 1: Add state for active sprint**

Modify the `BoardContent` component to fetch and track the active sprint:

```typescript
function BoardContent() {
  const [showCreateDialog, setShowCreateDialog] = useState(false)
  const [members, setMembers] = useState<WorkspaceMember[]>([])
  const [activeSprint, setActiveSprint] = useState<Sprint | null>(null)
  const { user } = useAuth()
  const { currentWorkspace } = useWorkspace()
  const slug = currentWorkspace?.slug || ''
  const searchParams = useSearchParams()

  // Add new function to fetch active sprint
  const loadActiveSprint = useCallback(async () => {
    if (!slug) return
    try {
      const sprints = await api.listSprints(slug, 'active')
      if (sprints.length > 0) {
        setActiveSprint(sprints[0])
      }
    } catch (error) {
      console.error('Failed to load active sprint:', error)
    }
  }, [slug])

  // Fetch workspace members on mount
  const loadMembers = useCallback(async () => {
    if (!slug) return
    try {
      const data = await api.listWorkspaceMembers(slug)
      setMembers(data)
    } catch (error) {
      console.error('Failed to load workspace members:', error)
    }
  }, [slug])

  useEffect(() => {
    loadMembers()
    loadActiveSprint()
  }, [loadMembers, loadActiveSprint])
```

**Step 2: Add sprint filter function**

Add a function to filter tickets by sprint after the `getFilteredTickets` function:

```typescript
  // Filter tickets by assignee
  const getFilteredTickets = (tickets: Ticket[]) => {
    const assigneeParam = searchParams.get('assignee')
    if (!assigneeParam) return tickets

    const filters = assigneeParam.split(',')

    return tickets.filter((ticket) => {
      // Check for "me" filter
      if (filters.includes('me') && user?.email) {
        if (ticket.assignees.includes(user.email)) return true
      }

      // Check for "unassigned" filter
      if (filters.includes('unassigned')) {
        if (ticket.assignees.length === 0) return true
      }

      // Check for specific member emails
      const memberFilters = filters.filter((f) => f !== 'me' && f !== 'unassigned')
      if (memberFilters.length > 0) {
        if (ticket.assignees.some((a) => memberFilters.includes(a))) return true
      }

      return false
    })
  }

  // NEW: Filter tickets by sprint
  const getSprintFilteredTickets = (tickets: Ticket[]) => {
    const showActiveSprint = searchParams.get('sprint') === 'active'

    if (!showActiveSprint || !activeSprint) {
      return tickets
    }

    return tickets.filter((ticket) => ticket.sprint_id === activeSprint.id)
  }

  // NEW: Combine both filters
  const applyAllFilters = (tickets: Ticket[]) => {
    let filtered = tickets

    // First apply assignee filter
    const assigneeParam = searchParams.get('assignee')
    if (assigneeParam) {
      filtered = getFilteredTickets(filtered)
    }

    // Then apply sprint filter
    filtered = getSprintFilteredTickets(filtered)

    return filtered
  }
```

**Step 3: Update toolbar to include sprint filter**

Modify the toolbar section in the JSX:

```typescript
  return (
    <>
      <div className="flex flex-col h-[calc(100vh-3.5rem-4rem)] md:h-[calc(100vh-4rem)]">
        {/* Toolbar with filters */}
        <div className="flex items-center gap-2 px-4 md:px-6 pt-4 pb-2">
          <SprintFilter hasActiveSprint={!!activeSprint} />
          <AssigneeFilter members={members} currentUserEmail={user?.email} />
        </div>

        {/* Board */}
        <div className="flex-1 overflow-hidden">
          <KanbanBoard filterFn={applyAllFilters} />
        </div>
      </div>

      <MobileFAB onClick={() => setShowCreateDialog(true)} />

      <CreateTicketDialog
        open={showCreateDialog}
        onClose={() => setShowCreateDialog(false)}
      />
    </>
  )
```

**Step 4: Add import for SprintFilter and Sprint type**

At the top of the file, add:

```typescript
import { SprintFilter } from '@/components/board/sprint-filter'
import type { WorkspaceMember, Ticket, Sprint } from '@/lib/types'
```

**Step 5: Verify changes compile**

Run: `cd jility-web && npm run build`
Expected: Build succeeds with no TypeScript errors

**Step 6: Commit the changes**

```bash
git add jility-web/app/w/[slug]/board/page.tsx
git commit -m "feat: add sprint filtering to board view"
```

---

## Task 3: Manual Testing

**Step 1: Start the development server**

Run: `./dev.sh start-frontend` or `cd jility-web && npm run dev`

**Step 2: Test with no active sprint**

1. Navigate to `/w/{workspace}/board`
2. Verify: Sprint filter toggle should NOT be visible
3. Verify: All tickets are shown

**Step 3: Create and start a sprint**

1. Navigate to `/w/{workspace}/sprint/planning`
2. Create a new sprint
3. Add some tickets to the sprint
4. Click "Start Sprint"
5. Navigate back to `/w/{workspace}/board`

**Step 4: Test sprint filter toggle**

1. Verify: Sprint filter toggle is now visible, showing "All Tickets"
2. Click the toggle
3. Verify: URL changes to `?sprint=active`
4. Verify: Toggle shows "Active Sprint"
5. Verify: Only tickets in the active sprint are displayed
6. Click toggle again
7. Verify: All tickets are shown again

**Step 5: Test filter persistence**

1. Enable sprint filter (URL should have `?sprint=active`)
2. Refresh the page
3. Verify: Sprint filter is still active
4. Verify: Only active sprint tickets are shown

**Step 6: Test combined filters**

1. Enable sprint filter
2. Also select an assignee filter
3. Verify: Tickets are filtered by BOTH sprint AND assignee

---

## Task 4: Documentation Update

**Files:**
- Modify: `docs/features/sprint-planning.md` (create if doesn't exist)

**Step 1: Document the new sprint filter feature**

Create or update the sprint planning documentation:

```markdown
# Sprint Planning Features

## Board Sprint Filter

The Kanban board can be filtered to show only tickets from the active sprint, similar to JIRA's sprint board.

**How to use:**
1. Start a sprint from the Sprint Planning page
2. Navigate to the Board view
3. Click the "Active Sprint" toggle in the toolbar
4. The board will now show only tickets assigned to the active sprint

**Combining filters:**
- The sprint filter works alongside assignee filters
- You can filter by both sprint AND assignee simultaneously
- Filters are persisted in the URL for deep linking

**When there's no active sprint:**
- The sprint filter toggle is hidden
- All tickets are displayed regardless of sprint assignment
```

**Step 2: Commit documentation**

```bash
git add docs/features/sprint-planning.md
git commit -m "docs: document board sprint filter feature"
```

---

## Verification Checklist

- [ ] Sprint filter component created with proper styling
- [ ] Board page fetches active sprint on mount
- [ ] Sprint filter toggle appears only when active sprint exists
- [ ] Clicking toggle updates URL with `?sprint=active` param
- [ ] Board correctly filters tickets by sprint_id
- [ ] Filter state persists across page refreshes
- [ ] Sprint filter combines correctly with assignee filter
- [ ] No TypeScript errors in build
- [ ] Feature works on mobile and desktop layouts
- [ ] Documentation updated

## Notes

- This implementation uses URL query params for filter state, making the board state shareable via URL
- The filter is client-side only - all tickets are still fetched, then filtered in the browser
- Future optimization: Add sprint_id query param to the API's listTickets endpoint for server-side filtering
