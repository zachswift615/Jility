# Sprint Planning Fix Implementation Plan

> **Status:** ✅ COMPLETE (JIL-7)

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix the existing Sprint Planning, Active Sprint, and History pages to work with the current workspace/auth system and add a Create Sprint dialog.

**Architecture:** The backend API is fully implemented. Frontend needs to switch from hardcoded ports/IDs to using workspace context and the centralized API client. All three sprint pages currently reference wrong ports and lack auth/workspace integration.

**Tech Stack:** Next.js 14, TypeScript, React, Tailwind CSS, existing API client pattern from board/settings pages

---

## Current State Analysis

**Backend:** ✅ Fully implemented
- All sprint CRUD endpoints exist and are registered
- Tables: `sprints`, `sprint_tickets`
- 12 endpoints including list, create, update, delete, start, complete, add/remove tickets, stats, burndown, history

**Frontend:** ❌ Broken
- Three pages exist but use wrong API URL (localhost:3001 vs 3900)
- No authentication headers
- Hardcoded project IDs instead of workspace context
- Missing Create Sprint dialog

**Pattern to Follow:** `jility-web/app/w/[slug]/board/page.tsx`
- Uses `useWorkspace()` for context
- Uses `useAuth()` for user
- Uses `api.*()` functions from `lib/api.ts`
- Wrapped with `withAuth()` HOC

---

## Task 1: Add Sprint Type Definitions

**Files:**
- Modify: `jility-web/lib/types.ts` (append to file)

**Step 1: Add Sprint interfaces**

Add these types to the end of `jility-web/lib/types.ts`:

```typescript
export interface Sprint {
  id: string
  project_id: string
  name: string
  goal?: string
  status: 'planning' | 'active' | 'completed'
  start_date?: string
  end_date?: string
  created_at: string
  updated_at: string
}

export interface SprintStats {
  total_tickets: number
  total_points: number
  completed_tickets: number
  completed_points: number
  in_progress_tickets: number
  in_progress_points: number
  todo_tickets: number
  todo_points: number
  completion_percentage: number
}

export interface SprintDetails {
  sprint: Sprint
  tickets: Ticket[]
  stats: SprintStats
}

export interface BurndownDataPoint {
  date: string
  ideal: number
  actual: number
}

export interface BurndownData {
  sprint_id: string
  data_points: BurndownDataPoint[]
}

export interface VelocityData {
  sprint_name: string
  completed_points: number
}

export interface SprintHistory {
  sprints: Sprint[]
  velocity_data: VelocityData[]
  average_velocity: number
}
```

**Step 2: Commit**

```bash
git add jility-web/lib/types.ts
git commit -m "feat: add Sprint type definitions"
```

---

## Task 2: Add Sprint API Client Functions

**Files:**
- Modify: `jility-web/lib/api.ts` (append to API class)

**Step 1: Read existing API client pattern**

Check `jility-web/lib/api.ts` lines 1-50 to understand the pattern:
- Uses `API_BASE` constant
- Has `getHeaders()` method for auth
- Methods throw on non-ok responses
- Returns typed JSON data

**Step 2: Add sprint API methods**

Add these methods to the `API` class in `jility-web/lib/api.ts` (before the closing brace):

```typescript
  // Sprint Management
  async listSprints(workspace: string, status?: string): Promise<Sprint[]> {
    const project = await this.getProjectByWorkspace(workspace)
    const url = status
      ? `${API_BASE}/projects/${project.id}/sprints?status=${status}`
      : `${API_BASE}/projects/${project.id}/sprints`
    const res = await fetch(url, { headers: this.getHeaders() })
    if (!res.ok) throw new Error(`Failed to list sprints: ${res.statusText}`)
    return res.json()
  }

  async createSprint(workspace: string, data: {
    name: string
    goal?: string
    start_date?: string
    end_date?: string
  }): Promise<Sprint> {
    const project = await this.getProjectByWorkspace(workspace)
    const res = await fetch(`${API_BASE}/projects/${project.id}/sprints`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify(data),
    })
    if (!res.ok) throw new Error(`Failed to create sprint: ${res.statusText}`)
    return res.json()
  }

  async getSprint(sprintId: string): Promise<SprintDetails> {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}`, {
      headers: this.getHeaders(),
    })
    if (!res.ok) throw new Error(`Failed to get sprint: ${res.statusText}`)
    return res.json()
  }

  async updateSprint(sprintId: string, data: {
    name?: string
    goal?: string
    start_date?: string
    end_date?: string
  }): Promise<Sprint> {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}`, {
      method: 'PUT',
      headers: this.getHeaders(),
      body: JSON.stringify(data),
    })
    if (!res.ok) throw new Error(`Failed to update sprint: ${res.statusText}`)
    return res.json()
  }

  async deleteSprint(sprintId: string): Promise<void> {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}`, {
      method: 'DELETE',
      headers: this.getHeaders(),
    })
    if (!res.ok) throw new Error(`Failed to delete sprint: ${res.statusText}`)
  }

  async startSprint(sprintId: string, data: {
    start_date: string
    end_date: string
  }): Promise<Sprint> {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}/start`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify(data),
    })
    if (!res.ok) throw new Error(`Failed to start sprint: ${res.statusText}`)
    return res.json()
  }

  async completeSprint(sprintId: string): Promise<Sprint> {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}/complete`, {
      method: 'POST',
      headers: this.getHeaders(),
    })
    if (!res.ok) throw new Error(`Failed to complete sprint: ${res.statusText}`)
    return res.json()
  }

  async addTicketToSprint(sprintId: string, ticketId: string, addedBy: string): Promise<void> {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}/tickets/${ticketId}`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ added_by: addedBy }),
    })
    if (!res.ok) throw new Error(`Failed to add ticket to sprint: ${res.statusText}`)
  }

  async removeTicketFromSprint(sprintId: string, ticketId: string): Promise<void> {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}/tickets/${ticketId}`, {
      method: 'DELETE',
      headers: this.getHeaders(),
    })
    if (!res.ok) throw new Error(`Failed to remove ticket from sprint: ${res.statusText}`)
  }

  async getSprintStats(sprintId: string): Promise<SprintStats> {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}/stats`, {
      headers: this.getHeaders(),
    })
    if (!res.ok) throw new Error(`Failed to get sprint stats: ${res.statusText}`)
    return res.json()
  }

  async getBurndownData(sprintId: string): Promise<BurndownData> {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}/burndown`, {
      headers: this.getHeaders(),
    })
    if (!res.ok) throw new Error(`Failed to get burndown data: ${res.statusText}`)
    return res.json()
  }

  async getSprintHistory(workspace: string): Promise<SprintHistory> {
    const project = await this.getProjectByWorkspace(workspace)
    const res = await fetch(`${API_BASE}/projects/${project.id}/sprint-history`, {
      headers: this.getHeaders(),
    })
    if (!res.ok) throw new Error(`Failed to get sprint history: ${res.statusText}`)
    return res.json()
  }
```

**Step 3: Add Sprint imports to types.ts import**

Update the import at the top of `jility-web/lib/api.ts`:

```typescript
import type {
  User,
  Project,
  Workspace,
  WorkspaceMember,
  Ticket,
  Comment,
  Sprint,
  SprintDetails,
  SprintStats,
  BurndownData,
  SprintHistory
} from './types'
```

**Step 4: Commit**

```bash
git add jility-web/lib/api.ts
git commit -m "feat: add Sprint API client methods"
```

---

## Task 3: Create Sprint Dialog Component

**Files:**
- Create: `jility-web/components/sprint/create-sprint-dialog.tsx`

**Step 1: Create the component**

Create file `jility-web/components/sprint/create-sprint-dialog.tsx`:

```typescript
'use client'

import { useState } from 'react'
import { X } from 'lucide-react'
import { api } from '@/lib/api'
import { useWorkspace } from '@/lib/workspace-context'

interface CreateSprintDialogProps {
  isOpen: boolean
  onClose: () => void
  onSuccess: () => void
}

export function CreateSprintDialog({ isOpen, onClose, onSuccess }: CreateSprintDialogProps) {
  const [name, setName] = useState('')
  const [goal, setGoal] = useState('')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')
  const { currentWorkspace } = useWorkspace()

  if (!isOpen) return null

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!currentWorkspace) return

    setLoading(true)
    setError('')

    try {
      await api.createSprint(currentWorkspace.slug, {
        name,
        goal: goal || undefined,
      })
      onSuccess()
      setName('')
      setGoal('')
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create sprint')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="bg-card border-border rounded-lg border p-6 w-full max-w-md">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-bold">Create Sprint</h2>
          <button
            onClick={onClose}
            className="text-muted-foreground hover:text-foreground"
          >
            <X className="h-5 w-5" />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label htmlFor="name" className="block text-sm font-medium mb-1">
              Sprint Name *
            </label>
            <input
              id="name"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full px-3 py-2 bg-background border-input border rounded-md"
              placeholder="Sprint 1"
              required
            />
          </div>

          <div>
            <label htmlFor="goal" className="block text-sm font-medium mb-1">
              Sprint Goal (optional)
            </label>
            <textarea
              id="goal"
              value={goal}
              onChange={(e) => setGoal(e.target.value)}
              className="w-full px-3 py-2 bg-background border-input border rounded-md"
              placeholder="What do we want to achieve?"
              rows={3}
            />
          </div>

          {error && (
            <div className="text-sm text-destructive">{error}</div>
          )}

          <div className="flex gap-2 justify-end">
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 bg-secondary text-secondary-foreground rounded-md"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={loading || !name}
              className="px-4 py-2 bg-primary text-primary-foreground rounded-md disabled:opacity-50"
            >
              {loading ? 'Creating...' : 'Create Sprint'}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}
```

**Step 2: Commit**

```bash
git add jility-web/components/sprint/create-sprint-dialog.tsx
git commit -m "feat: create Sprint dialog component"
```

---

## Task 4: Fix Sprint Planning Page

**Files:**
- Modify: `jility-web/app/w/[slug]/sprint/planning/page.tsx` (complete rewrite)

**Step 1: Replace the entire file**

Replace `jility-web/app/w/[slug]/sprint/planning/page.tsx` with:

```typescript
'use client'

import { useState, useEffect, useCallback } from 'react'
import { Plus } from 'lucide-react'
import { withAuth } from '@/lib/with-auth'
import { useAuth } from '@/lib/auth-context'
import { useWorkspace } from '@/lib/workspace-context'
import { api } from '@/lib/api'
import { CreateSprintDialog } from '@/components/sprint/create-sprint-dialog'
import type { Sprint, Ticket } from '@/lib/types'

function SprintPlanningContent() {
  const [sprints, setSprints] = useState<Sprint[]>([])
  const [selectedSprint, setSelectedSprint] = useState<Sprint | null>(null)
  const [sprintTickets, setSprintTickets] = useState<Ticket[]>([])
  const [backlogTickets, setBacklogTickets] = useState<Ticket[]>([])
  const [loading, setLoading] = useState(true)
  const [showCreateDialog, setShowCreateDialog] = useState(false)
  const { user } = useAuth()
  const { currentWorkspace } = useWorkspace()
  const slug = currentWorkspace?.slug || ''

  const fetchSprints = useCallback(async () => {
    if (!slug) return
    try {
      const data = await api.listSprints(slug, 'planning')
      setSprints(data)
      if (data.length > 0 && !selectedSprint) {
        setSelectedSprint(data[0])
      }
    } catch (error) {
      console.error('Failed to fetch sprints:', error)
    } finally {
      setLoading(false)
    }
  }, [slug, selectedSprint])

  const fetchSprintDetails = useCallback(async (sprintId: string) => {
    try {
      const data = await api.getSprint(sprintId)
      setSprintTickets(data.tickets)
    } catch (error) {
      console.error('Failed to fetch sprint details:', error)
    }
  }, [])

  const fetchBacklogTickets = useCallback(async () => {
    if (!slug) return
    try {
      const data = await api.listTickets(slug, { status: 'backlog' })
      setBacklogTickets(data)
    } catch (error) {
      console.error('Failed to fetch backlog:', error)
    }
  }, [slug])

  useEffect(() => {
    fetchSprints()
    fetchBacklogTickets()
  }, [fetchSprints, fetchBacklogTickets])

  useEffect(() => {
    if (selectedSprint) {
      fetchSprintDetails(selectedSprint.id)
    }
  }, [selectedSprint, fetchSprintDetails])

  async function addTicketToSprint(ticketId: string) {
    if (!selectedSprint || !user) return

    try {
      await api.addTicketToSprint(selectedSprint.id, ticketId, user.email)
      // Move ticket from backlog to sprint
      const ticket = backlogTickets.find(t => t.id === ticketId)
      if (ticket) {
        setBacklogTickets(prev => prev.filter(t => t.id !== ticketId))
        setSprintTickets(prev => [...prev, ticket])
      }
    } catch (error) {
      console.error('Failed to add ticket to sprint:', error)
    }
  }

  async function removeTicketFromSprint(ticketId: string) {
    if (!selectedSprint) return

    try {
      await api.removeTicketFromSprint(selectedSprint.id, ticketId)
      // Move ticket from sprint to backlog
      const ticket = sprintTickets.find(t => t.id === ticketId)
      if (ticket) {
        setSprintTickets(prev => prev.filter(t => t.id !== ticketId))
        setBacklogTickets(prev => [...prev, ticket])
      }
    } catch (error) {
      console.error('Failed to remove ticket from sprint:', error)
    }
  }

  async function startSprint() {
    if (!selectedSprint || !slug) return

    const startDate = new Date().toISOString()
    const endDate = new Date(Date.now() + 14 * 24 * 60 * 60 * 1000).toISOString()

    try {
      await api.startSprint(selectedSprint.id, { start_date: startDate, end_date: endDate })
      window.location.href = `/w/${slug}/sprint/active`
    } catch (error) {
      console.error('Failed to start sprint:', error)
    }
  }

  const plannedPoints = sprintTickets.reduce((sum, t) => sum + (t.story_points || 0), 0)
  const capacity = 70 // TODO: Make configurable
  const capacityPercentage = capacity > 0 ? Math.round((plannedPoints / capacity) * 100) : 0

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-muted-foreground">Loading...</div>
      </div>
    )
  }

  return (
    <>
      <div className="container mx-auto px-3 md:px-6 py-4 md:py-8">
        {/* Header */}
        <div className="mb-6 md:mb-8">
          <div className="flex items-center justify-between mb-4">
            <h1 className="text-2xl md:text-3xl font-bold">Sprint Planning</h1>
            <button
              onClick={() => setShowCreateDialog(true)}
              className="flex items-center gap-2 px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:opacity-90 transition-opacity"
            >
              <Plus className="h-4 w-4" />
              <span className="hidden sm:inline">New Sprint</span>
            </button>
          </div>

          {sprints.length === 0 ? (
            <div className="text-center py-12">
              <p className="text-muted-foreground mb-4">No planning sprints yet</p>
              <button
                onClick={() => setShowCreateDialog(true)}
                className="px-6 py-3 bg-primary text-primary-foreground rounded-lg hover:opacity-90"
              >
                Create Your First Sprint
              </button>
            </div>
          ) : selectedSprint && (
            <>
              <div className="flex items-center justify-between mb-4">
                <div>
                  <h2 className="text-xl md:text-2xl font-bold">{selectedSprint.name}</h2>
                  {selectedSprint.goal && (
                    <p className="text-muted-foreground">{selectedSprint.goal}</p>
                  )}
                </div>
                <button
                  onClick={startSprint}
                  disabled={sprintTickets.length === 0}
                  className="px-6 py-2 bg-primary text-primary-foreground rounded-lg hover:opacity-90 disabled:opacity-50 disabled:cursor-not-allowed transition-opacity"
                >
                  Start Sprint
                </button>
              </div>

              {/* Capacity Indicator */}
              <div className="bg-card rounded-lg p-4 md:p-6 border-border border">
                <div className="flex items-center justify-between mb-2 text-sm md:text-base">
                  <span className="font-medium">Capacity: {capacity} pts</span>
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
            </>
          )}
        </div>

        {/* Drag and Drop Area */}
        {selectedSprint && (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 md:gap-6">
            {/* Backlog */}
            <div>
              <h3 className="text-lg md:text-xl font-bold mb-4">Backlog ({backlogTickets.length})</h3>
              <div className="bg-card rounded-lg border-border border p-4 min-h-[400px]">
                <p className="text-sm text-muted-foreground mb-4">
                  Click tickets to add to sprint →
                </p>
                <div className="space-y-2">
                  {backlogTickets.map(ticket => (
                    <div
                      key={ticket.id}
                      onClick={() => addTicketToSprint(ticket.id)}
                      className="p-3 md:p-4 bg-secondary rounded border-border border hover:border-primary cursor-pointer transition-colors"
                    >
                      <div className="flex items-center justify-between">
                        <div className="flex-1 min-w-0">
                          <div className="font-medium text-sm">{ticket.number}</div>
                          <div className="text-sm text-muted-foreground truncate">
                            {ticket.title}
                          </div>
                        </div>
                        {ticket.story_points && (
                          <div className="ml-4 px-2 py-1 bg-primary/10 text-primary rounded text-sm font-medium flex-shrink-0">
                            {ticket.story_points} pts
                          </div>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>

            {/* Sprint */}
            <div>
              <h3 className="text-lg md:text-xl font-bold mb-4">{selectedSprint.name} ({sprintTickets.length})</h3>
              <div className="bg-card rounded-lg border-border border p-4 min-h-[400px]">
                <p className="text-sm text-muted-foreground mb-4">
                  ← Click tickets to remove from sprint
                </p>
                <div className="space-y-2">
                  {sprintTickets.map(ticket => (
                    <div
                      key={ticket.id}
                      onClick={() => removeTicketFromSprint(ticket.id)}
                      className="p-3 md:p-4 bg-primary/5 rounded border-primary/20 border hover:border-destructive cursor-pointer transition-colors"
                    >
                      <div className="flex items-center justify-between">
                        <div className="flex-1 min-w-0">
                          <div className="font-medium text-sm">{ticket.number}</div>
                          <div className="text-sm text-muted-foreground truncate">
                            {ticket.title}
                          </div>
                        </div>
                        {ticket.story_points && (
                          <div className="ml-4 px-2 py-1 bg-primary/10 text-primary rounded text-sm font-medium flex-shrink-0">
                            {ticket.story_points} pts
                          </div>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </div>
        )}
      </div>

      <CreateSprintDialog
        isOpen={showCreateDialog}
        onClose={() => setShowCreateDialog(false)}
        onSuccess={fetchSprints}
      />
    </>
  )
}

export default withAuth(SprintPlanningContent)
```

**Step 2: Commit**

```bash
git add jility-web/app/w/[slug]/sprint/planning/page.tsx
git commit -m "fix: update Sprint Planning page to use workspace context and auth"
```

---

## Task 5: Fix Active Sprint Page

**Files:**
- Modify: `jility-web/app/w/[slug]/sprint/active/page.tsx` (complete rewrite)

**Step 1: Replace the entire file**

Replace `jility-web/app/w/[slug]/sprint/active/page.tsx` with:

```typescript
'use client'

import { useState, useEffect, useCallback } from 'react'
import { Calendar, Target, TrendingUp } from 'lucide-react'
import { withAuth } from '@/lib/with-auth'
import { useWorkspace } from '@/lib/workspace-context'
import { api } from '@/lib/api'
import type { Sprint, SprintDetails } from '@/lib/types'

function ActiveSprintContent() {
  const [sprint, setSprint] = useState<SprintDetails | null>(null)
  const [loading, setLoading] = useState(true)
  const { currentWorkspace } = useWorkspace()
  const slug = currentWorkspace?.slug || ''

  const fetchActiveSprint = useCallback(async () => {
    if (!slug) return
    try {
      const sprints = await api.listSprints(slug, 'active')
      if (sprints.length > 0) {
        const details = await api.getSprint(sprints[0].id)
        setSprint(details)
      }
    } catch (error) {
      console.error('Failed to fetch active sprint:', error)
    } finally {
      setLoading(false)
    }
  }, [slug])

  useEffect(() => {
    fetchActiveSprint()
  }, [fetchActiveSprint])

  async function completeSprint() {
    if (!sprint || !slug) return

    if (!confirm('Are you sure you want to complete this sprint?')) return

    try {
      await api.completeSprint(sprint.sprint.id)
      window.location.href = `/w/${slug}/sprint/history`
    } catch (error) {
      console.error('Failed to complete sprint:', error)
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-muted-foreground">Loading...</div>
      </div>
    )
  }

  if (!sprint) {
    return (
      <div className="container mx-auto px-3 md:px-6 py-4 md:py-8">
        <div className="text-center py-12">
          <h1 className="text-2xl md:text-3xl font-bold mb-4">No Active Sprint</h1>
          <p className="text-muted-foreground mb-6">
            Start a sprint from the planning page to begin tracking progress.
          </p>
          <a
            href={`/w/${slug}/sprint/planning`}
            className="inline-block px-6 py-3 bg-primary text-primary-foreground rounded-lg hover:opacity-90"
          >
            Go to Sprint Planning
          </a>
        </div>
      </div>
    )
  }

  const { stats } = sprint
  const daysRemaining = sprint.sprint.end_date
    ? Math.max(0, Math.ceil((new Date(sprint.sprint.end_date).getTime() - Date.now()) / (1000 * 60 * 60 * 24)))
    : null

  return (
    <div className="container mx-auto px-3 md:px-6 py-4 md:py-8">
      {/* Header */}
      <div className="mb-6 md:mb-8">
        <div className="flex flex-col md:flex-row md:items-center md:justify-between gap-4 mb-6">
          <div>
            <h1 className="text-2xl md:text-3xl font-bold mb-2">{sprint.sprint.name}</h1>
            {sprint.sprint.goal && (
              <p className="text-muted-foreground">{sprint.sprint.goal}</p>
            )}
          </div>
          <button
            onClick={completeSprint}
            className="px-6 py-2 bg-primary text-primary-foreground rounded-lg hover:opacity-90 self-start md:self-auto"
          >
            Complete Sprint
          </button>
        </div>

        {/* Sprint Info Cards */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          {/* Days Remaining */}
          {daysRemaining !== null && (
            <div className="bg-card border-border border rounded-lg p-4">
              <div className="flex items-center gap-2 text-muted-foreground mb-2">
                <Calendar className="h-4 w-4" />
                <span className="text-sm font-medium">Days Remaining</span>
              </div>
              <div className="text-2xl font-bold">{daysRemaining}</div>
            </div>
          )}

          {/* Sprint Goal */}
          {sprint.sprint.goal && (
            <div className="bg-card border-border border rounded-lg p-4">
              <div className="flex items-center gap-2 text-muted-foreground mb-2">
                <Target className="h-4 w-4" />
                <span className="text-sm font-medium">Sprint Goal</span>
              </div>
              <div className="text-sm line-clamp-2">{sprint.sprint.goal}</div>
            </div>
          )}

          {/* Completion */}
          <div className="bg-card border-border border rounded-lg p-4">
            <div className="flex items-center gap-2 text-muted-foreground mb-2">
              <TrendingUp className="h-4 w-4" />
              <span className="text-sm font-medium">Completion</span>
            </div>
            <div className="text-2xl font-bold">
              {Math.round(stats.completion_percentage)}%
            </div>
            <div className="text-sm text-muted-foreground mt-1">
              {stats.completed_points}/{stats.total_points} pts
            </div>
          </div>

          {/* Tickets */}
          <div className="bg-card border-border border rounded-lg p-4">
            <div className="flex items-center gap-2 text-muted-foreground mb-2">
              <span className="text-sm font-medium">Tickets</span>
            </div>
            <div className="text-2xl font-bold">{stats.total_tickets}</div>
            <div className="text-sm text-muted-foreground mt-1">
              {stats.completed_tickets} done
            </div>
          </div>
        </div>
      </div>

      {/* Progress Bar */}
      <div className="bg-card border-border border rounded-lg p-4 md:p-6 mb-6">
        <h3 className="font-semibold mb-4">Sprint Progress</h3>
        <div className="space-y-3">
          <div>
            <div className="flex justify-between text-sm mb-1">
              <span>Completed</span>
              <span className="font-medium">{stats.completed_points} pts</span>
            </div>
            <div className="w-full bg-secondary rounded-full h-2">
              <div
                className="bg-green-600 h-2 rounded-full transition-all"
                style={{ width: `${stats.total_points > 0 ? (stats.completed_points / stats.total_points) * 100 : 0}%` }}
              />
            </div>
          </div>
          <div>
            <div className="flex justify-between text-sm mb-1">
              <span>In Progress</span>
              <span className="font-medium">{stats.in_progress_points} pts</span>
            </div>
            <div className="w-full bg-secondary rounded-full h-2">
              <div
                className="bg-yellow-600 h-2 rounded-full transition-all"
                style={{ width: `${stats.total_points > 0 ? (stats.in_progress_points / stats.total_points) * 100 : 0}%` }}
              />
            </div>
          </div>
          <div>
            <div className="flex justify-between text-sm mb-1">
              <span>To Do</span>
              <span className="font-medium">{stats.todo_points} pts</span>
            </div>
            <div className="w-full bg-secondary rounded-full h-2">
              <div
                className="bg-muted h-2 rounded-full transition-all"
                style={{ width: `${stats.total_points > 0 ? (stats.todo_points / stats.total_points) * 100 : 0}%` }}
              />
            </div>
          </div>
        </div>
      </div>

      {/* Tickets List */}
      <div className="bg-card border-border border rounded-lg p-4 md:p-6">
        <h3 className="font-semibold mb-4">Sprint Tickets ({sprint.tickets.length})</h3>
        <div className="space-y-2">
          {sprint.tickets.map(ticket => (
            <a
              key={ticket.id}
              href={`/w/${slug}/ticket/${ticket.id}`}
              className="block p-4 bg-secondary rounded border-border border hover:border-primary transition-colors"
            >
              <div className="flex items-center justify-between">
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-3">
                    <span className="font-medium text-sm">{ticket.number}</span>
                    <span className={`px-2 py-0.5 rounded text-xs font-medium ${
                      ticket.status === 'done' ? 'bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200' :
                      ticket.status === 'in_progress' ? 'bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200' :
                      'bg-muted text-muted-foreground'
                    }`}>
                      {ticket.status}
                    </span>
                  </div>
                  <div className="text-sm mt-1 truncate">{ticket.title}</div>
                </div>
                {ticket.story_points && (
                  <div className="ml-4 px-2 py-1 bg-primary/10 text-primary rounded text-sm font-medium flex-shrink-0">
                    {ticket.story_points} pts
                  </div>
                )}
              </div>
            </a>
          ))}
        </div>
      </div>
    </div>
  )
}

export default withAuth(ActiveSprintContent)
```

**Step 2: Commit**

```bash
git add jility-web/app/w/[slug]/sprint/active/page.tsx
git commit -m "fix: update Active Sprint page to use workspace context and auth"
```

---

## Task 6: Fix Sprint History Page

**Files:**
- Modify: `jility-web/app/w/[slug]/sprint/history/page.tsx` (complete rewrite)

**Step 1: Replace the entire file**

Replace `jility-web/app/w/[slug]/sprint/history/page.tsx` with:

```typescript
'use client'

import { useState, useEffect, useCallback } from 'react'
import { Calendar, TrendingUp } from 'lucide-react'
import { withAuth } from '@/lib/with-auth'
import { useWorkspace } from '@/lib/workspace-context'
import { api } from '@/lib/api'
import type { SprintHistory } from '@/lib/types'

function SprintHistoryContent() {
  const [history, setHistory] = useState<SprintHistory | null>(null)
  const [loading, setLoading] = useState(true)
  const { currentWorkspace } = useWorkspace()
  const slug = currentWorkspace?.slug || ''

  const fetchHistory = useCallback(async () => {
    if (!slug) return
    try {
      const data = await api.getSprintHistory(slug)
      setHistory(data)
    } catch (error) {
      console.error('Failed to fetch sprint history:', error)
    } finally {
      setLoading(false)
    }
  }, [slug])

  useEffect(() => {
    fetchHistory()
  }, [fetchHistory])

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-muted-foreground">Loading...</div>
      </div>
    )
  }

  if (!history || history.sprints.length === 0) {
    return (
      <div className="container mx-auto px-3 md:px-6 py-4 md:py-8">
        <div className="text-center py-12">
          <h1 className="text-2xl md:text-3xl font-bold mb-4">No Sprint History</h1>
          <p className="text-muted-foreground">
            Complete some sprints to see your team's velocity and performance.
          </p>
        </div>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-3 md:px-6 py-4 md:py-8">
      {/* Header */}
      <div className="mb-6 md:mb-8">
        <h1 className="text-2xl md:text-3xl font-bold mb-2">Sprint History</h1>
        <p className="text-muted-foreground">
          View completed sprints and team velocity over time
        </p>
      </div>

      {/* Velocity Summary */}
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-6 md:mb-8">
        <div className="bg-card border-border border rounded-lg p-4 md:p-6">
          <div className="flex items-center gap-2 text-muted-foreground mb-2">
            <TrendingUp className="h-5 w-5" />
            <span className="font-medium">Average Velocity</span>
          </div>
          <div className="text-3xl md:text-4xl font-bold">
            {Math.round(history.average_velocity)}
          </div>
          <div className="text-sm text-muted-foreground mt-1">
            points per sprint
          </div>
        </div>

        <div className="bg-card border-border border rounded-lg p-4 md:p-6">
          <div className="flex items-center gap-2 text-muted-foreground mb-2">
            <Calendar className="h-5 w-5" />
            <span className="font-medium">Completed Sprints</span>
          </div>
          <div className="text-3xl md:text-4xl font-bold">
            {history.sprints.length}
          </div>
          <div className="text-sm text-muted-foreground mt-1">
            total sprints
          </div>
        </div>
      </div>

      {/* Velocity Chart (Simple Bar Chart) */}
      <div className="bg-card border-border border rounded-lg p-4 md:p-6 mb-6 md:mb-8">
        <h3 className="font-semibold mb-4">Velocity Trend</h3>
        <div className="space-y-3">
          {history.velocity_data.slice().reverse().map((sprint, index) => (
            <div key={index}>
              <div className="flex justify-between text-sm mb-1">
                <span className="font-medium truncate pr-2">{sprint.sprint_name}</span>
                <span className="font-medium flex-shrink-0">{sprint.completed_points} pts</span>
              </div>
              <div className="w-full bg-secondary rounded-full h-6">
                <div
                  className="bg-primary h-6 rounded-full transition-all flex items-center px-2"
                  style={{
                    width: `${history.average_velocity > 0
                      ? Math.min((sprint.completed_points / (history.average_velocity * 1.5)) * 100, 100)
                      : 0}%`,
                    minWidth: sprint.completed_points > 0 ? '3rem' : '0'
                  }}
                >
                  {sprint.completed_points > 0 && (
                    <span className="text-xs font-medium text-primary-foreground">
                      {sprint.completed_points}
                    </span>
                  )}
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Completed Sprints List */}
      <div className="bg-card border-border border rounded-lg p-4 md:p-6">
        <h3 className="font-semibold mb-4">Completed Sprints</h3>
        <div className="space-y-3">
          {history.sprints.map((sprint) => {
            const velocity = history.velocity_data.find(v => v.sprint_name === sprint.name)
            return (
              <div
                key={sprint.id}
                className="p-4 bg-secondary rounded border-border border"
              >
                <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2">
                  <div className="flex-1 min-w-0">
                    <h4 className="font-medium">{sprint.name}</h4>
                    {sprint.goal && (
                      <p className="text-sm text-muted-foreground mt-1 line-clamp-2">
                        {sprint.goal}
                      </p>
                    )}
                    {sprint.start_date && sprint.end_date && (
                      <p className="text-xs text-muted-foreground mt-2">
                        {new Date(sprint.start_date).toLocaleDateString()} - {new Date(sprint.end_date).toLocaleDateString()}
                      </p>
                    )}
                  </div>
                  <div className="flex items-center gap-4 flex-shrink-0">
                    {velocity && (
                      <div className="text-right">
                        <div className="text-sm text-muted-foreground">Velocity</div>
                        <div className="text-lg font-bold">{velocity.completed_points} pts</div>
                      </div>
                    )}
                  </div>
                </div>
              </div>
            )
          })}
        </div>
      </div>
    </div>
  )
}

export default withAuth(SprintHistoryContent)
```

**Step 2: Commit**

```bash
git add jility-web/app/w/[slug]/sprint/history/page.tsx
git commit -m "fix: update Sprint History page to use workspace context and auth"
```

---

## Task 7: Manual Testing

**Test Checklist:**

1. **Create Sprint**
   - Navigate to `/w/[workspace]/sprint/planning`
   - Click "New Sprint" button
   - Fill in sprint name and goal
   - Verify sprint appears in planning list

2. **Add Tickets to Sprint**
   - Ensure backlog has tickets (create some if needed)
   - Click tickets in backlog column
   - Verify they move to sprint column
   - Check capacity indicator updates

3. **Start Sprint**
   - Click "Start Sprint" button
   - Verify redirect to Active Sprint page
   - Check sprint shows correct info

4. **View Active Sprint**
   - Navigate to `/w/[workspace]/sprint/active`
   - Verify sprint details, stats, and tickets display
   - Check progress bars show correct percentages

5. **Complete Sprint**
   - On Active Sprint page, click "Complete Sprint"
   - Confirm the action
   - Verify redirect to History page

6. **View Sprint History**
   - Navigate to `/w/[workspace]/sprint/history`
   - Verify completed sprint appears
   - Check velocity chart displays correctly
   - Verify average velocity calculation

**Commands:**

```bash
# Start dev server if not running
npm run dev

# Test in browser:
# 1. http://localhost:3901/w/[workspace]/sprint/planning
# 2. http://localhost:3901/w/[workspace]/sprint/active
# 3. http://localhost:3901/w/[workspace]/sprint/history
```

**Expected Results:**
- All pages load without errors
- Authentication works (redirects to login if not authenticated)
- Workspace context is correct (uses current workspace slug)
- All API calls use correct backend URL (localhost:3900)
- Sprint CRUD operations work end-to-end

---

## Verification

After completing all tasks:

1. ✅ Sprint type definitions added
2. ✅ Sprint API client methods implemented
3. ✅ Create Sprint dialog component works
4. ✅ Sprint Planning page uses workspace context and auth
5. ✅ Active Sprint page uses workspace context and auth
6. ✅ Sprint History page uses workspace context and auth
7. ✅ All pages tested manually

**Final commit:**

```bash
git add -A
git commit -m "feat: fix Sprint Planning feature (JIL-7)

- Add Sprint type definitions and API client methods
- Create Sprint dialog component
- Fix Planning/Active/History pages to use workspace context
- Update all pages to use correct API URL and auth headers
- Test complete sprint workflow end-to-end"
```

---

## Notes for Engineer

**Backend is ready:** All 12 sprint endpoints are implemented and working. No backend changes needed.

**Pattern to follow:** Look at `app/w/[slug]/board/page.tsx` for reference on how to use `useWorkspace()`, `useAuth()`, and `api.*()` calls.

**Mobile-first:** All pages use responsive Tailwind classes (`px-3 md:px-6`, `text-2xl md:text-3xl`, etc.)

**Theme variables:** Use `bg-card`, `text-foreground`, `border-border`, etc. - never hardcoded colors

**API client pattern:**
- Always use `api.*()` methods, never raw fetch calls
- Let API client handle auth headers and base URL
- Handle errors with try/catch and console.error

**Testing:** Use your browser's dev tools to verify:
- Network tab: API calls go to `localhost:3900`
- Console: No errors
- Application tab: Auth cookies present
