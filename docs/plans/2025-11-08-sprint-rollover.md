# Sprint Rollover Implementation Plan

> **Status:** ✅ COMPLETE (JIL-22, Implemented 2025-01-08)

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** When completing a sprint with incomplete tickets, show a dialog asking what to do with them (roll to next sprint, return to backlog, or keep in completed sprint).

**Architecture:** Intercept the "Complete Sprint" action to check for incomplete tickets. Show a modal dialog with rollover options. Create a new sprint in "planning" status if needed, then bulk-move tickets based on user choice.

**Tech Stack:** Next.js 14 App Router, React hooks, existing sprint API endpoints, dialog components

---

## Task 1: Create Sprint Completion Dialog Component

**Files:**
- Create: `jility-web/components/sprint/complete-sprint-dialog.tsx`

**Step 1: Create the dialog component with rollover options**

```typescript
'use client'

import { useState } from 'react'
import { X } from 'lucide-react'
import type { Ticket } from '@/lib/types'

interface CompleteSprintDialogProps {
  isOpen: boolean
  onClose: () => void
  onConfirm: (action: 'rollover' | 'backlog' | 'keep') => Promise<void>
  incompleteTickets: Ticket[]
  sprintName: string
}

export function CompleteSprintDialog({
  isOpen,
  onClose,
  onConfirm,
  incompleteTickets,
  sprintName,
}: CompleteSprintDialogProps) {
  const [selectedAction, setSelectedAction] = useState<'rollover' | 'backlog' | 'keep'>('rollover')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')

  if (!isOpen) return null

  const handleConfirm = async () => {
    setLoading(true)
    setError('')

    try {
      await onConfirm(selectedAction)
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to complete sprint')
    } finally {
      setLoading(false)
    }
  }

  const incompleteCount = incompleteTickets.length
  const incompletePoints = incompleteTickets.reduce((sum, t) => sum + (t.story_points || 0), 0)

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4">
      <div className="bg-card border-border rounded-lg border p-6 w-full max-w-lg">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-bold">Complete Sprint: {sprintName}</h2>
          <button
            onClick={onClose}
            className="text-muted-foreground hover:text-foreground"
            disabled={loading}
          >
            <X className="h-5 w-5" />
          </button>
        </div>

        {incompleteCount > 0 ? (
          <>
            <div className="mb-6">
              <p className="text-sm text-muted-foreground mb-4">
                This sprint has <span className="font-semibold text-foreground">{incompleteCount} incomplete tickets</span> ({incompletePoints} points).
                What would you like to do with them?
              </p>

              <div className="space-y-3">
                {/* Option 1: Roll to next sprint */}
                <label className="flex items-start gap-3 p-3 rounded-lg border border-border cursor-pointer hover:bg-accent/50 transition-colors">
                  <input
                    type="radio"
                    name="action"
                    value="rollover"
                    checked={selectedAction === 'rollover'}
                    onChange={(e) => setSelectedAction(e.target.value as 'rollover')}
                    className="mt-1"
                  />
                  <div className="flex-1">
                    <div className="font-medium">Roll over to next sprint</div>
                    <div className="text-sm text-muted-foreground">
                      Create a new sprint and move incomplete tickets to it
                    </div>
                  </div>
                </label>

                {/* Option 2: Return to backlog */}
                <label className="flex items-start gap-3 p-3 rounded-lg border border-border cursor-pointer hover:bg-accent/50 transition-colors">
                  <input
                    type="radio"
                    name="action"
                    value="backlog"
                    checked={selectedAction === 'backlog'}
                    onChange={(e) => setSelectedAction(e.target.value as 'backlog')}
                    className="mt-1"
                  />
                  <div className="flex-1">
                    <div className="font-medium">Return to backlog</div>
                    <div className="text-sm text-muted-foreground">
                      Remove tickets from sprint and move to backlog status
                    </div>
                  </div>
                </label>

                {/* Option 3: Keep in sprint */}
                <label className="flex items-start gap-3 p-3 rounded-lg border border-border cursor-pointer hover:bg-accent/50 transition-colors">
                  <input
                    type="radio"
                    name="action"
                    value="keep"
                    checked={selectedAction === 'keep'}
                    onChange={(e) => setSelectedAction(e.target.value as 'keep')}
                    className="mt-1"
                  />
                  <div className="flex-1">
                    <div className="font-medium">Keep in this sprint</div>
                    <div className="text-sm text-muted-foreground">
                      Mark sprint as complete but leave incomplete tickets as-is
                    </div>
                  </div>
                </label>
              </div>
            </div>

            {error && (
              <div className="mb-4 text-sm text-destructive">{error}</div>
            )}

            <div className="flex gap-2 justify-end">
              <button
                type="button"
                onClick={onClose}
                disabled={loading}
                className="px-4 py-2 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 disabled:opacity-50"
              >
                Cancel
              </button>
              <button
                onClick={handleConfirm}
                disabled={loading}
                className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:opacity-90 disabled:opacity-50"
              >
                {loading ? 'Completing...' : 'Complete Sprint'}
              </button>
            </div>
          </>
        ) : (
          <>
            <p className="text-sm text-muted-foreground mb-6">
              All tickets in this sprint are complete. Ready to finish?
            </p>

            {error && (
              <div className="mb-4 text-sm text-destructive">{error}</div>
            )}

            <div className="flex gap-2 justify-end">
              <button
                type="button"
                onClick={onClose}
                disabled={loading}
                className="px-4 py-2 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 disabled:opacity-50"
              >
                Cancel
              </button>
              <button
                onClick={handleConfirm}
                disabled={loading}
                className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:opacity-90 disabled:opacity-50"
              >
                {loading ? 'Completing...' : 'Complete Sprint'}
              </button>
            </div>
          </>
        )}
      </div>
    </div>
  )
}
```

**Step 2: Commit the new component**

```bash
git add jility-web/components/sprint/complete-sprint-dialog.tsx
git commit -m "feat: add sprint completion dialog with rollover options"
```

---

## Task 2: Update Active Sprint Page to Use Dialog

**Files:**
- Modify: `jility-web/app/w/[slug]/sprint/active/page.tsx`

**Step 1: Add dialog state and imports**

Add to the imports:

```typescript
import { CompleteSprintDialog } from '@/components/sprint/complete-sprint-dialog'
```

Add state for the dialog in `ActiveSprintContent`:

```typescript
function ActiveSprintContent() {
  const [sprint, setSprint] = useState<SprintDetails | null>(null)
  const [loading, setLoading] = useState(true)
  const [showCompleteDialog, setShowCompleteDialog] = useState(false)
  const { currentWorkspace } = useWorkspace()
  const { user } = useAuth()
  const slug = currentWorkspace?.slug || ''
```

**Step 2: Replace the completeSprint function**

Replace the existing `completeSprint` function with this new implementation:

```typescript
  async function handleCompleteClick() {
    setShowCompleteDialog(true)
  }

  async function handleCompleteConfirm(action: 'rollover' | 'backlog' | 'keep') {
    if (!sprint || !slug || !user) return

    const incompleteTickets = sprint.tickets.filter(t => t.status !== 'done')

    try {
      if (action === 'rollover' && incompleteTickets.length > 0) {
        // Create next sprint
        const nextSprintNumber = parseInt(sprint.sprint.name.match(/\d+/)?.[0] || '1') + 1
        const nextSprint = await api.createSprint(slug, {
          name: `Sprint ${nextSprintNumber}`,
          goal: undefined,
        })

        // Move incomplete tickets to next sprint
        for (const ticket of incompleteTickets) {
          await api.removeTicketFromSprint(sprint.sprint.id, ticket.id)
          await api.addTicketToSprint(nextSprint.id, ticket.id, user.email)
        }
      } else if (action === 'backlog' && incompleteTickets.length > 0) {
        // Remove from sprint and set to backlog status
        for (const ticket of incompleteTickets) {
          await api.removeTicketFromSprint(sprint.sprint.id, ticket.id)
          await api.updateTicketStatus(ticket.id, 'backlog')
        }
      }
      // If 'keep', we don't do anything with the incomplete tickets

      // Complete the sprint
      await api.completeSprint(sprint.sprint.id)

      // Redirect to history
      window.location.href = `/w/${slug}/sprint/history`
    } catch (error) {
      console.error('Failed to complete sprint:', error)
      throw error
    }
  }
```

**Step 3: Update the Complete Sprint button**

Replace the button's onClick handler:

```typescript
          <button
            onClick={handleCompleteClick}
            className="px-6 py-2 bg-primary text-primary-foreground rounded-lg hover:opacity-90 self-start md:self-auto"
          >
            Complete Sprint
          </button>
```

**Step 4: Add the dialog to JSX**

Add the dialog component before the closing tag of the main div:

```typescript
      {/* ... existing content ... */}
      </div>

      <CompleteSprintDialog
        isOpen={showCompleteDialog}
        onClose={() => setShowCompleteDialog(false)}
        onConfirm={handleCompleteConfirm}
        incompleteTickets={sprint?.tickets.filter(t => t.status !== 'done') || []}
        sprintName={sprint?.sprint.name || ''}
      />
    </div>
  )
}
```

**Step 5: Verify changes compile**

Run: `cd jility-web && npm run build`
Expected: Build succeeds with no TypeScript errors

**Step 6: Commit the changes**

```bash
git add jility-web/app/w/[slug]/sprint/active/page.tsx
git commit -m "feat: integrate sprint completion dialog with rollover logic"
```

---

## Task 3: Manual Testing

**Step 1: Start the development server**

Run: `./dev.sh start-frontend` or `cd jility-web && npm run dev`

**Step 2: Test with all tickets complete**

1. Create a sprint and add tickets
2. Start the sprint
3. Move all tickets to "done" status
4. Click "Complete Sprint"
5. Verify: Dialog shows message about all tickets complete
6. Verify: No rollover options are shown
7. Click "Complete Sprint" in dialog
8. Verify: Sprint is completed and redirects to history

**Step 3: Test rollover to next sprint**

1. Create another sprint and add tickets
2. Start the sprint
3. Leave some tickets in "in_progress" or "todo" status
4. Click "Complete Sprint"
5. Verify: Dialog shows incomplete ticket count and points
6. Select "Roll over to next sprint" (default selection)
7. Click "Complete Sprint"
8. Verify: Sprint is completed
9. Navigate to Sprint Planning
10. Verify: A new sprint was created (e.g., "Sprint 2" if previous was "Sprint 1")
11. Verify: Incomplete tickets are in the new sprint

**Step 4: Test return to backlog**

1. Create another sprint, add tickets, start it
2. Leave some tickets incomplete
3. Click "Complete Sprint"
4. Select "Return to backlog"
5. Click "Complete Sprint"
6. Navigate to Sprint Planning
7. Verify: Incomplete tickets are back in the backlog
8. Verify: Their status is "backlog"

**Step 5: Test keep in sprint**

1. Create another sprint, add tickets, start it
2. Leave some tickets incomplete
3. Click "Complete Sprint"
4. Select "Keep in this sprint"
5. Click "Complete Sprint"
6. Navigate to Sprint History
7. View the completed sprint
8. Verify: Incomplete tickets are still associated with the completed sprint

**Step 6: Test cancellation**

1. Start a sprint with incomplete tickets
2. Click "Complete Sprint"
3. Click "Cancel" in dialog
4. Verify: Dialog closes and sprint remains active

---

## Task 4: Add Sprint Name Auto-Increment Logic

**Files:**
- Modify: `jility-web/app/w/[slug]/sprint/active/page.tsx`

**Step 1: Improve sprint name generation**

Update the rollover logic to handle various sprint naming patterns:

```typescript
  async function handleCompleteConfirm(action: 'rollover' | 'backlog' | 'keep') {
    if (!sprint || !slug || !user) return

    const incompleteTickets = sprint.tickets.filter(t => t.status !== 'done')

    try {
      if (action === 'rollover' && incompleteTickets.length > 0) {
        // Generate next sprint name
        const currentName = sprint.sprint.name
        let nextName = 'Sprint 1'

        // Try to extract number from current sprint name
        const match = currentName.match(/(\d+)/)
        if (match) {
          const num = parseInt(match[1])
          const prefix = currentName.substring(0, match.index)
          const suffix = currentName.substring(match.index! + match[1].length)
          nextName = `${prefix}${num + 1}${suffix}`
        } else {
          // No number found, append " 2" to current name
          nextName = `${currentName} 2`
        }

        // Create next sprint
        const nextSprint = await api.createSprint(slug, {
          name: nextName,
          goal: sprint.sprint.goal, // Copy goal from current sprint
        })

        // Move incomplete tickets to next sprint
        for (const ticket of incompleteTickets) {
          await api.removeTicketFromSprint(sprint.sprint.id, ticket.id)
          await api.addTicketToSprint(nextSprint.id, ticket.id, user.email)
        }
      } else if (action === 'backlog' && incompleteTickets.length > 0) {
        // Remove from sprint and set to backlog status
        for (const ticket of incompleteTickets) {
          await api.removeTicketFromSprint(sprint.sprint.id, ticket.id)
          await api.updateTicketStatus(ticket.id, 'backlog')
        }
      }
      // If 'keep', we don't do anything with the incomplete tickets

      // Complete the sprint
      await api.completeSprint(sprint.sprint.id)

      // Redirect to history
      window.location.href = `/w/${slug}/sprint/history`
    } catch (error) {
      console.error('Failed to complete sprint:', error)
      throw error
    }
  }
```

**Step 2: Verify name generation with different patterns**

Test cases:
- "Sprint 1" → "Sprint 2"
- "Sprint 42" → "Sprint 43"
- "Q1 Sprint 5" → "Q1 Sprint 6"
- "Release 2.0" → "Release 3.0"
- "My Sprint" → "My Sprint 2"

**Step 3: Commit the improvement**

```bash
git add jility-web/app/w/[slug]/sprint/active/page.tsx
git commit -m "feat: improve sprint name auto-increment logic"
```

---

## Task 5: Documentation Update

**Files:**
- Modify: `docs/features/sprint-planning.md`

**Step 1: Document the rollover feature**

Add to the sprint planning documentation:

```markdown
## Sprint Rollover

When completing a sprint that has incomplete tickets, Jility provides three options:

### Roll over to next sprint (default)
- Creates a new sprint with an auto-incremented name
- Moves all incomplete tickets to the new sprint
- Copies the sprint goal from the current sprint
- New sprint starts in "planning" status

**Sprint name examples:**
- "Sprint 1" → "Sprint 2"
- "Q2 Sprint 5" → "Q2 Sprint 6"
- "Release 2.0" → "Release 3.0"

### Return to backlog
- Removes incomplete tickets from the sprint
- Changes ticket status to "backlog"
- Tickets become available for future sprint planning

### Keep in this sprint
- Leaves incomplete tickets associated with the completed sprint
- Useful for retrospectives and sprint analysis
- Incomplete tickets remain in their current status

### When all tickets are complete
If all tickets in a sprint are marked as "done", the completion dialog simply confirms the action without showing rollover options.
```

**Step 2: Commit documentation**

```bash
git add docs/features/sprint-planning.md
git commit -m "docs: document sprint rollover feature"
```

---

## Verification Checklist

- [ ] Complete sprint dialog component created with three rollover options
- [ ] Dialog shows incomplete ticket count and story points
- [ ] "Rollover" option creates new sprint with auto-incremented name
- [ ] "Rollover" option copies sprint goal to new sprint
- [ ] Incomplete tickets are moved to new sprint when rolling over
- [ ] "Backlog" option removes tickets from sprint and sets status to backlog
- [ ] "Keep" option leaves tickets in the completed sprint
- [ ] Dialog handles sprints with all tickets complete (no rollover UI)
- [ ] Sprint name auto-increment works for various naming patterns
- [ ] No TypeScript errors in build
- [ ] Feature works on mobile and desktop layouts
- [ ] Documentation updated

## Notes

- This implementation handles rollover synchronously (one ticket at a time)
- Future optimization: Add a bulk rollover API endpoint to handle all tickets in one request
- Sprint name auto-increment handles most common patterns but may need adjustment for unconventional names
- The rollover action is not cancellable once started - consider adding progress indicator for many tickets
