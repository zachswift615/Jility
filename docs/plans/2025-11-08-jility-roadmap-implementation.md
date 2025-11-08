# Jility Feature Roadmap - Complete Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement features task-by-task.

**Goal:** Implement the complete Jility feature roadmap from Phase 1 through Phase 5, transforming Jility from a basic ticket tracker into a full-featured, AI-native project management tool.

**Status:**
- ✅ Phase 1: Core Collaboration (JIL-5) - 1.1 Comments System (JIL-6) (COMPLETE)
- ✅ Phase 2: Sprint Planning (JIL-7, JIL-22, JIL-23, JIL-24) - Sprint Planning Fix, Sprint Rollover, Configurable Capacity, Active Sprint Filter (COMPLETE)
- 📋 Phase 3: Visual Workflows (JIL-8) - PLANNED
- 📋 Phase 4: Search & Discovery (JIL-11) - PLANNED
- 📋 Phase 5: AI/Agent Features (JIL-14) - PLANNED

---

# Phase 3: Visual Workflows

## 3.1 Swimlanes for Board (JIL-9) - 5 Story Points

### Overview
Add grouping capability to the board view - group tickets by assignee, epic, label, or priority with collapsible swimlanes.

### Current State
- Board exists at `jility-web/app/w/[slug]/board/page.tsx`
- Uses simple column-based Kanban layout
- No grouping capability

### Architecture
- Add grouping selector dropdown in board header
- Group tickets client-side by selected field
- Render swimlane headers with collapse/expand
- Preserve group preference in localStorage
- Maintain existing drag-and-drop within groups

### Files to Modify
1. `jility-web/app/w/[slug]/board/page.tsx` - Add grouping logic
2. `jility-web/components/kanban/board.tsx` - Support swimlane rendering
3. `jility-web/components/kanban/swimlane.tsx` - New component
4. `jility-web/lib/board-utils.ts` - Grouping logic (new file)

### Implementation Steps

**Task 1: Add Grouping Utility Functions**

Create `jility-web/lib/board-utils.ts`:

```typescript
import type { Ticket } from './types'

export type GroupBy = 'none' | 'assignee' | 'epic' | 'label' | 'priority'

export interface TicketGroup {
  key: string
  label: string
  tickets: Ticket[]
  count: number
}

export function groupTickets(tickets: Ticket[], groupBy: GroupBy): TicketGroup[] {
  if (groupBy === 'none') {
    return [{ key: 'all', label: 'All Tickets', tickets, count: tickets.length }]
  }

  const groups = new Map<string, Ticket[]>()

  tickets.forEach(ticket => {
    let keys: string[] = []

    switch (groupBy) {
      case 'assignee':
        keys = ticket.assignees.length > 0 ? ticket.assignees : ['unassigned']
        break
      case 'epic':
        keys = [ticket.epic_id || 'no-epic']
        break
      case 'label':
        keys = ticket.labels.length > 0 ? ticket.labels : ['no-label']
        break
      case 'priority':
        keys = [ticket.priority || 'no-priority']
        break
    }

    keys.forEach(key => {
      if (!groups.has(key)) groups.set(key, [])
      groups.get(key)!.push(ticket)
    })
  })

  return Array.from(groups.entries())
    .map(([key, tickets]) => ({
      key,
      label: formatGroupLabel(key, groupBy),
      tickets,
      count: tickets.length,
    }))
    .sort((a, b) => b.count - a.count) // Sort by ticket count descending
}

function formatGroupLabel(key: string, groupBy: GroupBy): string {
  if (key === 'unassigned') return 'Unassigned'
  if (key === 'no-epic') return 'No Epic'
  if (key === 'no-label') return 'No Label'
  if (key === 'no-priority') return 'No Priority'
  return key
}

export function saveGroupPreference(groupBy: GroupBy) {
  localStorage.setItem('board-group-by', groupBy)
}

export function loadGroupPreference(): GroupBy {
  return (localStorage.getItem('board-group-by') as GroupBy) || 'none'
}
```

Commit: `git commit -m "feat: add ticket grouping utilities for swimlanes"`

**Task 2: Create Swimlane Component**

Create `jility-web/components/kanban/swimlane.tsx`:

```typescript
'use client'

import { useState } from 'react'
import { ChevronDown, ChevronRight } from 'lucide-react'
import { TicketCard } from './ticket-card'
import type { Ticket } from '@/lib/types'

interface SwimlaneProps {
  label: string
  tickets: Ticket[]
  status: string
  onTicketClick: (ticket: Ticket) => void
}

export function Swimlane({ label, tickets, status, onTicketClick }: SwimlaneProps) {
  const [isCollapsed, setIsCollapsed] = useState(false)

  return (
    <div className="border-b border-border last:border-b-0">
      {/* Swimlane Header */}
      <button
        onClick={() => setIsCollapsed(!isCollapsed)}
        className="w-full flex items-center gap-2 p-3 bg-muted/50 hover:bg-muted transition-colors"
      >
        {isCollapsed ? (
          <ChevronRight className="h-4 w-4" />
        ) : (
          <ChevronDown className="h-4 w-4" />
        )}
        <span className="font-medium">{label}</span>
        <span className="ml-auto text-sm text-muted-foreground">
          {tickets.length}
        </span>
      </button>

      {/* Swimlane Content */}
      {!isCollapsed && (
        <div className="p-2 space-y-2 min-h-[100px]">
          {tickets
            .filter(t => t.status === status)
            .map(ticket => (
              <TicketCard
                key={ticket.id}
                ticket={ticket}
                onClick={() => onTicketClick(ticket)}
              />
            ))}
        </div>
      )}
    </div>
  )
}
```

Commit: `git commit -m "feat: create Swimlane component with collapse/expand"`

**Task 3: Add Group Selector to Board**

Modify `jility-web/app/w/[slug]/board/page.tsx` - add this near the header:

```typescript
import { useState, useEffect } from 'react'
import { groupTickets, saveGroupPreference, loadGroupPreference, type GroupBy } from '@/lib/board-utils'

// Inside component:
const [groupBy, setGroupBy] = useState<GroupBy>('none')

useEffect(() => {
  setGroupBy(loadGroupPreference())
}, [])

const handleGroupChange = (newGroupBy: GroupBy) => {
  setGroupBy(newGroupBy)
  saveGroupPreference(newGroupBy)
}

// Add to JSX before the board:
<div className="mb-4 flex items-center gap-2">
  <label className="text-sm font-medium">Group by:</label>
  <select
    value={groupBy}
    onChange={(e) => handleGroupChange(e.target.value as GroupBy)}
    className="px-3 py-1 bg-card border-border border rounded-md text-sm"
  >
    <option value="none">None</option>
    <option value="assignee">Assignee</option>
    <option value="epic">Epic</option>
    <option value="label">Label</option>
    <option value="priority">Priority</option>
  </select>
</div>
```

Commit: `git commit -m "feat: add group selector dropdown to board"`

**Task 4: Render Swimlanes**

Update the board rendering logic to use swimlanes when grouped:

```typescript
// Group tickets
const groups = groupTickets(allTickets, groupBy)

// Render swimlanes
{groupBy !== 'none' ? (
  <div className="grid grid-cols-5 gap-4">
    {['backlog', 'todo', 'in_progress', 'review', 'done'].map(status => (
      <div key={status} className="flex flex-col">
        <h3 className="font-semibold mb-2">{formatStatus(status)}</h3>
        <div className="space-y-2">
          {groups.map(group => (
            <Swimlane
              key={group.key}
              label={group.label}
              tickets={group.tickets}
              status={status}
              onTicketClick={handleTicketClick}
            />
          ))}
        </div>
      </div>
    ))}
  </div>
) : (
  // Original board rendering
  <KanbanBoard tickets={allTickets} />
)}
```

Commit: `git commit -m "feat: render board with swimlanes when grouped"`

**Testing:**
1. Navigate to board
2. Select "Group by: Assignee"
3. Verify tickets grouped by assignee
4. Click swimlane header to collapse/expand
5. Refresh page - verify grouping persists
6. Test drag-and-drop still works
7. Test all grouping options

---

## 3.2 Burndown Chart (JIL-10) - 5 Story Points

### Overview
Add burndown chart to active sprint view showing ideal vs. actual work remaining over time.

### Dependencies
- ✅ Backend burndown endpoint exists (`/api/sprints/:id/burndown`)
- ✅ Active sprint page exists
- ❌ Need chart library

### Architecture
- Use Recharts for visualization (already in Next.js ecosystem)
- Fetch burndown data from backend
- Display on Active Sprint page
- Show ideal line (linear) vs actual line (real progress)

### Files to Modify
1. `jility-web/package.json` - Add recharts dependency
2. `jility-web/app/w/[slug]/sprint/active/page.tsx` - Add chart
3. `jility-web/components/sprint/burndown-chart.tsx` - New component

### Implementation Steps

**Task 1: Install Recharts**

```bash
cd jility-web
npm install recharts
```

Commit: `git commit -m "deps: add recharts for burndown chart"`

**Task 2: Create Burndown Chart Component**

Create `jility-web/components/sprint/burndown-chart.tsx`:

```typescript
'use client'

import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts'
import type { BurndownData } from '@/lib/types'

interface BurndownChartProps {
  data: BurndownData
}

export function BurndownChart({ data }: BurndownChartProps) {
  const chartData = data.data_points.map(point => ({
    date: new Date(point.date).toLocaleDateString('en-US', { month: 'short', day: 'numeric' }),
    'Ideal': point.ideal,
    'Actual': point.actual,
  }))

  return (
    <div className="bg-card border-border border rounded-lg p-4 md:p-6">
      <h3 className="font-semibold mb-4">Burndown Chart</h3>
      <ResponsiveContainer width="100%" height={300}>
        <LineChart data={chartData}>
          <CartesianGrid strokeDasharray="3 3" className="stroke-border" />
          <XAxis
            dataKey="date"
            className="text-muted-foreground"
            tick={{ fontSize: 12 }}
          />
          <YAxis
            label={{ value: 'Story Points', angle: -90, position: 'insideLeft' }}
            className="text-muted-foreground"
            tick={{ fontSize: 12 }}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: 'hsl(var(--card))',
              border: '1px solid hsl(var(--border))',
              borderRadius: '0.5rem',
            }}
          />
          <Legend />
          <Line
            type="monotone"
            dataKey="Ideal"
            stroke="hsl(var(--muted-foreground))"
            strokeWidth={2}
            strokeDasharray="5 5"
            dot={false}
          />
          <Line
            type="monotone"
            dataKey="Actual"
            stroke="hsl(var(--primary))"
            strokeWidth={2}
            dot={{ fill: 'hsl(var(--primary))' }}
          />
        </LineChart>
      </ResponsiveContainer>
      <p className="text-sm text-muted-foreground mt-4">
        The ideal line shows expected progress. The actual line shows real progress based on completed tickets.
      </p>
    </div>
  )
}
```

Commit: `git commit -m "feat: create BurndownChart component"`

**Task 3: Add Chart to Active Sprint Page**

Modify `jility-web/app/w/[slug]/sprint/active/page.tsx`:

```typescript
import { BurndownChart } from '@/components/sprint/burndown-chart'
import type { BurndownData } from '@/lib/types'

// Add state:
const [burndownData, setBurndownData] = useState<BurndownData | null>(null)

// Add fetch function:
const fetchBurndown = useCallback(async (sprintId: string) => {
  try {
    const data = await api.getBurndownData(sprintId)
    setBurndownData(data)
  } catch (error) {
    console.error('Failed to fetch burndown data:', error)
  }
}, [])

// Call in useEffect after fetching sprint:
useEffect(() => {
  if (sprint) {
    fetchBurndown(sprint.sprint.id)
  }
}, [sprint, fetchBurndown])

// Add to JSX after progress bars:
{burndownData && <BurndownChart data={burndownData} />}
```

Commit: `git commit -m "feat: add burndown chart to active sprint page"`

**Testing:**
1. Start a sprint with tickets
2. Navigate to Active Sprint page
3. Verify burndown chart displays
4. Check ideal line is straight diagonal
5. Verify actual line reflects current progress
6. Complete some tickets, refresh, verify chart updates
7. Test on mobile - chart should be responsive

---

# Phase 4: Search & Discovery

## 4.1 Global Search (JIL-12) - 8 Story Points

### Overview
Enhance search with full-text capabilities, keyboard shortcuts (Cmd+K), and advanced filtering.

### Current State
- ✅ Backend has FTS5 full-text search for tickets
- ✅ Search endpoint exists: `/api/tickets/search`
- ❌ Frontend has basic search in navbar (limited)

### Architecture
- Command palette style (Cmd+K to open)
- Search dialog with filters sidebar
- Debounced search (300ms)
- Highlight matches in results
- Recent searches in localStorage

### Files to Modify
1. `jility-web/components/search/search-dialog.tsx` - New component
2. `jility-web/components/layout/navbar.tsx` - Add Cmd+K trigger
3. `jility-web/lib/api.ts` - Add search method
4. `jility-web/lib/types.ts` - Add SearchResult type

### Implementation Steps

**Task 1: Add Search Types and API**

Add to `jility-web/lib/types.ts`:

```typescript
export interface SearchResult {
  ticket: Ticket
  highlight: string
  score: number
}

export interface SearchFilters {
  status?: string[]
  assignee?: string
  labels?: string[]
  dateFrom?: string
  dateTo?: string
  minPoints?: number
  maxPoints?: number
}
```

Add to `jility-web/lib/api.ts`:

```typescript
async searchTickets(
  workspace: string,
  query: string,
  filters?: SearchFilters
): Promise<SearchResult[]> {
  const project = await this.getProjectByWorkspace(workspace)
  const params = new URLSearchParams({
    project_id: project.id,
    q: query,
    ...filters,
  })
  const res = await fetch(`${API_BASE}/tickets/search?${params}`, {
    headers: this.getHeaders(),
  })
  if (!res.ok) throw new Error(`Search failed: ${res.statusText}`)
  return res.json()
}
```

Commit: `git commit -m "feat: add search types and API method"`

**Task 2: Create Search Dialog Component**

Create `jility-web/components/search/search-dialog.tsx`:

```typescript
'use client'

import { useState, useEffect, useCallback } from 'react'
import { Search, X, Filter } from 'lucide-react'
import { useWorkspace } from '@/lib/workspace-context'
import { api } from '@/lib/api'
import type { SearchResult, SearchFilters } from '@/lib/types'
import { useDebounce } from '@/lib/use-debounce'

interface SearchDialogProps {
  isOpen: boolean
  onClose: () => void
}

export function SearchDialog({ isOpen, onClose }: SearchDialogProps) {
  const [query, setQuery] = useState('')
  const [results, setResults] = useState<SearchResult[]>([])
  const [loading, setLoading] = useState(false)
  const [showFilters, setShowFilters] = useState(false)
  const [filters, setFilters] = useState<SearchFilters>({})
  const debouncedQuery = useDebounce(query, 300)
  const { currentWorkspace } = useWorkspace()
  const slug = currentWorkspace?.slug || ''

  const performSearch = useCallback(async () => {
    if (!slug || !debouncedQuery || debouncedQuery.length < 2) {
      setResults([])
      return
    }

    setLoading(true)
    try {
      const data = await api.searchTickets(slug, debouncedQuery, filters)
      setResults(data)
      saveRecentSearch(debouncedQuery)
    } catch (error) {
      console.error('Search failed:', error)
    } finally {
      setLoading(false)
    }
  }, [slug, debouncedQuery, filters])

  useEffect(() => {
    performSearch()
  }, [performSearch])

  useEffect(() => {
    if (!isOpen) {
      setQuery('')
      setResults([])
      setFilters({})
    }
  }, [isOpen])

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose()
    }
    if (isOpen) {
      document.addEventListener('keydown', handleKeyDown)
      return () => document.removeEventListener('keydown', handleKeyDown)
    }
  }, [isOpen, onClose])

  if (!isOpen) return null

  return (
    <div className="fixed inset-0 z-50 flex items-start justify-center pt-[10vh] bg-black/50">
      <div className="bg-card border-border border rounded-lg w-full max-w-3xl max-h-[80vh] flex flex-col">
        {/* Search Input */}
        <div className="flex items-center gap-2 p-4 border-b border-border">
          <Search className="h-5 w-5 text-muted-foreground" />
          <input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Search tickets..."
            className="flex-1 bg-transparent outline-none"
            autoFocus
          />
          <button
            onClick={() => setShowFilters(!showFilters)}
            className="p-2 hover:bg-muted rounded-md"
          >
            <Filter className="h-4 w-4" />
          </button>
          <button onClick={onClose} className="p-2 hover:bg-muted rounded-md">
            <X className="h-4 w-4" />
          </button>
        </div>

        {/* Filters (collapsible) */}
        {showFilters && (
          <div className="p-4 border-b border-border bg-muted/50">
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="text-sm font-medium mb-1 block">Status</label>
                <select
                  className="w-full px-3 py-2 bg-background border-input border rounded-md text-sm"
                  onChange={(e) => setFilters({ ...filters, status: [e.target.value] })}
                >
                  <option value="">All</option>
                  <option value="backlog">Backlog</option>
                  <option value="todo">To Do</option>
                  <option value="in_progress">In Progress</option>
                  <option value="review">Review</option>
                  <option value="done">Done</option>
                </select>
              </div>
              {/* Add more filter fields as needed */}
            </div>
          </div>
        )}

        {/* Results */}
        <div className="flex-1 overflow-y-auto p-2">
          {loading && (
            <div className="text-center py-8 text-muted-foreground">
              Searching...
            </div>
          )}
          {!loading && query && results.length === 0 && (
            <div className="text-center py-8 text-muted-foreground">
              No results found for "{query}"
            </div>
          )}
          {!loading && results.map((result) => (
            <a
              key={result.ticket.id}
              href={`/w/${slug}/ticket/${result.ticket.id}`}
              className="block p-3 hover:bg-muted rounded-md mb-2"
              onClick={onClose}
            >
              <div className="flex items-center gap-2 mb-1">
                <span className="font-medium text-sm">{result.ticket.number}</span>
                <span className="text-xs text-muted-foreground">{result.ticket.status}</span>
              </div>
              <div className="font-medium mb-1">{result.ticket.title}</div>
              {result.highlight && (
                <div
                  className="text-sm text-muted-foreground line-clamp-2"
                  dangerouslySetInnerHTML={{ __html: result.highlight }}
                />
              )}
            </a>
          ))}
        </div>

        {/* Footer */}
        <div className="p-3 border-t border-border text-xs text-muted-foreground flex items-center justify-between">
          <span>Press <kbd className="px-1.5 py-0.5 bg-muted rounded">ESC</kbd> to close</span>
          <span>{results.length} results</span>
        </div>
      </div>
    </div>
  )
}

function saveRecentSearch(query: string) {
  const recent = JSON.parse(localStorage.getItem('recent-searches') || '[]')
  const updated = [query, ...recent.filter((q: string) => q !== query)].slice(0, 10)
  localStorage.setItem('recent-searches', JSON.stringify(updated))
}
```

Commit: `git commit -m "feat: create SearchDialog with filters and keyboard shortcuts"`

**Task 3: Add Cmd+K Trigger**

Modify `jility-web/components/layout/navbar.tsx`:

```typescript
import { Search } from 'lucide-react'
import { useState, useEffect } from 'react'
import { SearchDialog } from '@/components/search/search-dialog'

// Inside component:
const [searchOpen, setSearchOpen] = useState(false)

useEffect(() => {
  const handleKeyDown = (e: KeyboardEvent) => {
    if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
      e.preventDefault()
      setSearchOpen(true)
    }
  }
  document.addEventListener('keydown', handleKeyDown)
  return () => document.removeEventListener('keydown', handleKeyDown)
}, [])

// Replace existing search button with:
<button
  onClick={() => setSearchOpen(true)}
  className="flex items-center gap-2 px-3 py-2 bg-muted rounded-md hover:bg-muted/80"
>
  <Search className="h-4 w-4" />
  <span className="hidden md:inline text-sm text-muted-foreground">Search...</span>
  <kbd className="hidden md:inline px-1.5 py-0.5 text-xs bg-background rounded">
    ⌘K
  </kbd>
</button>

<SearchDialog isOpen={searchOpen} onClose={() => setSearchOpen(false)} />
```

Commit: `git commit -m "feat: add Cmd+K global search trigger"`

**Task 4: Create useDebounce Hook**

Create `jility-web/lib/use-debounce.ts`:

```typescript
import { useState, useEffect } from 'react'

export function useDebounce<T>(value: T, delay: number): T {
  const [debouncedValue, setDebouncedValue] = useState(value)

  useEffect(() => {
    const timer = setTimeout(() => setDebouncedValue(value), delay)
    return () => clearTimeout(timer)
  }, [value, delay])

  return debouncedValue
}
```

Commit: `git commit -m "feat: add useDebounce hook for search"`

**Testing:**
1. Press Cmd+K (or Ctrl+K on Windows)
2. Type a search query
3. Verify results appear after 300ms debounce
4. Click a result - verify navigation
5. Test filters
6. Verify recent searches saved
7. Test ESC to close
8. Test on mobile

---

## 4.2 Board Filters (JIL-13) - 5 Story Points

### Overview
Add quick filter toolbar to board for filtering by assignee, label, epic, story points.

### Current State
- Board has basic assignee filter via URL params
- Need UI for filter controls
- Need multi-select filters

### Architecture
- Filter toolbar above board
- Client-side filtering for speed
- URL params for sharing filtered views
- Save custom filter presets

### Implementation Steps

**Task 1: Create Filter Toolbar Component**

Create `jility-web/components/board/filter-toolbar.tsx`:

```typescript
'use client'

import { X } from 'lucide-react'
import type { Ticket } from '@/lib/types'

export interface BoardFilters {
  assignees: string[]
  labels: string[]
  epic?: string
  minPoints?: number
  maxPoints?: number
}

interface FilterToolbarProps {
  filters: BoardFilters
  onChange: (filters: BoardFilters) => void
  allTickets: Ticket[]
}

export function FilterToolbar({ filters, onChange, allTickets }: FilterToolbarProps) {
  // Extract unique values
  const allAssignees = Array.from(new Set(allTickets.flatMap(t => t.assignees)))
  const allLabels = Array.from(new Set(allTickets.flatMap(t => t.labels)))
  const allEpics = Array.from(new Set(allTickets.filter(t => t.epic_id).map(t => t.epic_id!)))

  const hasActiveFilters =
    filters.assignees.length > 0 ||
    filters.labels.length > 0 ||
    filters.epic ||
    filters.minPoints ||
    filters.maxPoints

  return (
    <div className="bg-card border-border border rounded-lg p-4 mb-4">
      <div className="flex items-center gap-4 flex-wrap">
        {/* Assignee Filter */}
        <div>
          <label className="text-sm font-medium mb-1 block">Assignees</label>
          <select
            multiple
            className="px-3 py-2 bg-background border-input border rounded-md text-sm"
            value={filters.assignees}
            onChange={(e) => {
              const selected = Array.from(e.target.selectedOptions, option => option.value)
              onChange({ ...filters, assignees: selected })
            }}
          >
            {allAssignees.map(assignee => (
              <option key={assignee} value={assignee}>{assignee}</option>
            ))}
          </select>
        </div>

        {/* Label Filter */}
        <div>
          <label className="text-sm font-medium mb-1 block">Labels</label>
          <select
            multiple
            className="px-3 py-2 bg-background border-input border rounded-md text-sm"
            value={filters.labels}
            onChange={(e) => {
              const selected = Array.from(e.target.selectedOptions, option => option.value)
              onChange({ ...filters, labels: selected })
            }}
          >
            {allLabels.map(label => (
              <option key={label} value={label}>{label}</option>
            ))}
          </select>
        </div>

        {/* Epic Filter */}
        <div>
          <label className="text-sm font-medium mb-1 block">Epic</label>
          <select
            className="px-3 py-2 bg-background border-input border rounded-md text-sm"
            value={filters.epic || ''}
            onChange={(e) => onChange({ ...filters, epic: e.target.value || undefined })}
          >
            <option value="">All</option>
            {allEpics.map(epic => (
              <option key={epic} value={epic}>{epic}</option>
            ))}
          </select>
        </div>

        {/* Story Points Range */}
        <div>
          <label className="text-sm font-medium mb-1 block">Story Points</label>
          <div className="flex items-center gap-2">
            <input
              type="number"
              placeholder="Min"
              className="w-20 px-2 py-1 bg-background border-input border rounded-md text-sm"
              value={filters.minPoints || ''}
              onChange={(e) => onChange({
                ...filters,
                minPoints: e.target.value ? parseInt(e.target.value) : undefined
              })}
            />
            <span className="text-muted-foreground">to</span>
            <input
              type="number"
              placeholder="Max"
              className="w-20 px-2 py-1 bg-background border-input border rounded-md text-sm"
              value={filters.maxPoints || ''}
              onChange={(e) => onChange({
                ...filters,
                maxPoints: e.target.value ? parseInt(e.target.value) : undefined
              })}
            />
          </div>
        </div>

        {/* Clear Filters */}
        {hasActiveFilters && (
          <button
            onClick={() => onChange({ assignees: [], labels: [] })}
            className="flex items-center gap-1 px-3 py-2 bg-destructive/10 text-destructive rounded-md hover:bg-destructive/20 text-sm"
          >
            <X className="h-4 w-4" />
            Clear Filters
          </button>
        )}
      </div>

      {/* Active Filters Display */}
      {hasActiveFilters && (
        <div className="flex items-center gap-2 mt-3 flex-wrap">
          <span className="text-sm text-muted-foreground">Active filters:</span>
          {filters.assignees.map(assignee => (
            <span key={assignee} className="px-2 py-1 bg-primary/10 text-primary rounded text-xs">
              {assignee}
            </span>
          ))}
          {filters.labels.map(label => (
            <span key={label} className="px-2 py-1 bg-primary/10 text-primary rounded text-xs">
              {label}
            </span>
          ))}
          {filters.epic && (
            <span className="px-2 py-1 bg-primary/10 text-primary rounded text-xs">
              Epic: {filters.epic}
            </span>
          )}
          {(filters.minPoints || filters.maxPoints) && (
            <span className="px-2 py-1 bg-primary/10 text-primary rounded text-xs">
              Points: {filters.minPoints || 0} - {filters.maxPoints || '∞'}
            </span>
          )}
        </div>
      )}
    </div>
  )
}

export function applyFilters(tickets: Ticket[], filters: BoardFilters): Ticket[] {
  return tickets.filter(ticket => {
    // Assignee filter
    if (filters.assignees.length > 0) {
      const hasMatch = ticket.assignees.some(a => filters.assignees.includes(a))
      if (!hasMatch) return false
    }

    // Label filter
    if (filters.labels.length > 0) {
      const hasMatch = ticket.labels.some(l => filters.labels.includes(l))
      if (!hasMatch) return false
    }

    // Epic filter
    if (filters.epic && ticket.epic_id !== filters.epic) {
      return false
    }

    // Story points filter
    if (filters.minPoints && (ticket.story_points || 0) < filters.minPoints) {
      return false
    }
    if (filters.maxPoints && (ticket.story_points || 0) > filters.maxPoints) {
      return false
    }

    return true
  })
}
```

Commit: `git commit -m "feat: create board filter toolbar with multi-select"`

**Task 2: Integrate Filters into Board**

Modify `jility-web/app/w/[slug]/board/page.tsx`:

```typescript
import { FilterToolbar, applyFilters, type BoardFilters } from '@/components/board/filter-toolbar'

// Add state:
const [filters, setFilters] = useState<BoardFilters>({ assignees: [], labels: [] })

// Apply filters before rendering:
const filteredTickets = applyFilters(allTickets, filters)

// Add to JSX:
<FilterToolbar
  filters={filters}
  onChange={setFilters}
  allTickets={allTickets}
/>

<KanbanBoard tickets={filteredTickets} />
```

Commit: `git commit -m "feat: integrate filter toolbar into board"`

**Task 3: Save Filter Presets**

Add preset save/load functionality:

```typescript
function saveFilterPreset(name: string, filters: BoardFilters) {
  const presets = JSON.parse(localStorage.getItem('filter-presets') || '{}')
  presets[name] = filters
  localStorage.setItem('filter-presets', JSON.stringify(presets))
}

function loadFilterPresets(): Record<string, BoardFilters> {
  return JSON.parse(localStorage.getItem('filter-presets') || '{}')
}

// Add UI for saving/loading presets
<button onClick={() => {
  const name = prompt('Name this filter preset:')
  if (name) saveFilterPreset(name, filters)
}}>
  Save Filter
</button>

<select onChange={(e) => {
  const presets = loadFilterPresets()
  setFilters(presets[e.target.value] || { assignees: [], labels: [] })
}}>
  <option value="">Load Preset...</option>
  {Object.keys(loadFilterPresets()).map(name => (
    <option key={name} value={name}>{name}</option>
  ))}
</select>
```

Commit: `git commit -m "feat: add filter preset save/load"`

**Testing:**
1. Navigate to board
2. Select assignee filters
3. Verify tickets filtered correctly
4. Add label filters
5. Test story points range
6. Clear all filters
7. Save a filter preset
8. Load the preset - verify it works
9. Test on mobile

---

# Phase 5: AI/Agent Features

## 5.1 Enhanced MCP Server (JIL-15) - 8 Story Points

### Overview
Expand MCP server with bulk operations, smart queries, and workflow automation.

### Current State
- ✅ Basic MCP exists: `jility-mcp/`
- ✅ Has: create_ticket, update_ticket, list_tickets, add_comment, update_status
- ❌ Missing: bulk operations, smart queries, context awareness

### Architecture
- Add batch endpoints to backend
- Enhance MCP tools with bulk variants
- Add query helpers for common questions
- Implement workflow triggers

### Files to Modify
1. `jility-server/src/api/tickets.rs` - Add batch endpoint
2. `jility-mcp/src/tools/tickets.rs` - Add bulk tools
3. `jility-mcp/src/tools/queries.rs` - New file for smart queries
4. `jility-mcp/src/context.rs` - New file for context tracking

### Implementation Steps

**Task 1: Add Batch Ticket Creation to Backend**

Modify `jility-server/src/api/tickets.rs`:

```rust
pub async fn create_tickets_batch(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(payload): Json<Vec<CreateTicketRequest>>,
) -> ApiResult<Json<Vec<TicketResponse>>> {
    let mut created = Vec::new();

    for req in payload {
        let ticket = create_single_ticket(state.clone(), auth_user.clone(), req).await?;
        created.push(ticket);
    }

    Ok(Json(created))
}
```

Register route in `mod.rs`:
```rust
.route("/api/tickets/batch", post(tickets::create_tickets_batch))
```

Commit: `git commit -m "feat: add batch ticket creation endpoint"`

**Task 2: Add Bulk MCP Tools**

Create `jility-mcp/src/tools/bulk.rs`:

```rust
use crate::JilityServer;
use serde_json::{json, Value};

pub struct BulkCreateTickets;

impl Tool for BulkCreateTickets {
    fn name(&self) -> &str {
        "create_tickets_batch"
    }

    fn description(&self) -> &str {
        "Create multiple tickets at once. Useful when breaking down an epic."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "tickets": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "title": { "type": "string" },
                            "description": { "type": "string" },
                            "story_points": { "type": "integer" },
                            "labels": { "type": "array", "items": { "type": "string" } },
                        },
                        "required": ["title"]
                    }
                }
            },
            "required": ["tickets"]
        })
    }

    async fn execute(&self, params: Value, server: &JilityServer) -> Result<Value> {
        let tickets = params["tickets"].as_array()
            .ok_or_else(|| anyhow!("tickets must be an array"))?;

        let response = server.client
            .post(&format!("{}/tickets/batch", server.api_base))
            .json(&tickets)
            .send()
            .await?;

        let created: Vec<Ticket> = response.json().await?;
        Ok(json!({
            "created_count": created.len(),
            "tickets": created
        }))
    }
}
```

Commit: `git commit -m "feat: add bulk ticket creation MCP tool"`

**Task 3: Add Smart Query Tools**

Create `jility-mcp/src/tools/queries.rs`:

```rust
pub struct FindBlockingTickets;

impl Tool for FindBlockingTickets {
    fn name(&self) -> &str {
        "find_blocking_tickets"
    }

    fn description(&self) -> &str {
        "Find all tickets that are blocking a specific ticket"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "ticket_id": { "type": "string", "description": "Ticket ID to check" }
            },
            "required": ["ticket_id"]
        })
    }

    async fn execute(&self, params: Value, server: &JilityServer) -> Result<Value> {
        let ticket_id = params["ticket_id"].as_str()
            .ok_or_else(|| anyhow!("ticket_id required"))?;

        // Get ticket dependencies
        let deps = server.get_dependencies(ticket_id).await?;

        // Filter to only blockers
        let blockers: Vec<_> = deps.into_iter()
            .filter(|d| d.status != "done")
            .collect();

        Ok(json!({
            "ticket_id": ticket_id,
            "blocking_tickets": blockers,
            "blocked": !blockers.is_empty()
        }))
    }
}

pub struct WhatIsReadyForReview;

impl Tool for WhatIsReadyForReview {
    fn name(&self) -> &str {
        "what_is_ready_for_review"
    }

    fn description(&self) -> &str {
        "Find all tickets in review status that need attention"
    }

    async fn execute(&self, _params: Value, server: &JilityServer) -> Result<Value> {
        let tickets = server.list_tickets(Some("review")).await?;

        Ok(json!({
            "count": tickets.len(),
            "tickets": tickets
        }))
    }
}
```

Commit: `git commit -m "feat: add smart query MCP tools"`

**Task 4: Add Context Tracking**

Create `jility-mcp/src/context.rs`:

```rust
use std::collections::VecDeque;

pub struct ConversationContext {
    recent_tickets: VecDeque<String>,
    max_size: usize,
}

impl ConversationContext {
    pub fn new() -> Self {
        Self {
            recent_tickets: VecDeque::new(),
            max_size: 10,
        }
    }

    pub fn remember_ticket(&mut self, ticket_id: String) {
        if self.recent_tickets.len() >= self.max_size {
            self.recent_tickets.pop_front();
        }
        self.recent_tickets.push_back(ticket_id);
    }

    pub fn get_recent(&self) -> Vec<String> {
        self.recent_tickets.iter().cloned().collect()
    }

    pub fn suggest_related(&self, current: &str) -> Vec<String> {
        // Simple implementation: return recent tickets
        self.recent_tickets
            .iter()
            .filter(|id| *id != current)
            .take(3)
            .cloned()
            .collect()
    }
}
```

Commit: `git commit -m "feat: add conversation context tracking"`

**Testing:**
1. Test batch create: `create_tickets_batch` with 5 tickets
2. Test smart query: `find_blocking_tickets` for a blocked ticket
3. Test context: Create tickets, verify recent list updates
4. Test `what_is_ready_for_review`

---

## 5.2 AI Epic Breakdown (JIL-16) - 5 Story Points

### Overview
AI assistant breaks down epic descriptions into implementable sub-tickets.

### Architecture
- Add "Generate Sub-tickets" button to epic detail view
- Call OpenAI/Claude API with epic description
- Parse structured JSON response
- Show review dialog before creating tickets

### Files to Create
1. `jility-server/src/ai/mod.rs` - AI integration module
2. `jility-server/src/ai/epic_breakdown.rs` - Epic breakdown logic
3. `jility-web/components/ticket/generate-subtasks-dialog.tsx` - UI
4. `jility-web/app/w/[slug]/ticket/[id]/page.tsx` - Add button

### Implementation Steps

**Task 1: Add AI Client to Backend**

Add dependency to `jility-server/Cargo.toml`:
```toml
[dependencies]
async-openai = "0.17"
```

Create `jility-server/src/ai/mod.rs`:
```rust
pub mod epic_breakdown;

use async_openai::{Client, types::*};

pub struct AIClient {
    client: Client<OpenAIConfig>,
}

impl AIClient {
    pub fn new(api_key: String) -> Self {
        let config = OpenAIConfig::new().with_api_key(api_key);
        Self {
            client: Client::with_config(config),
        }
    }
}
```

Commit: `git commit -m "feat: add OpenAI client setup"`

**Task 2: Implement Epic Breakdown**

Create `jility-server/src/ai/epic_breakdown.rs`:
```rust
use super::AIClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubTicketSuggestion {
    pub title: String,
    pub description: String,
    pub story_points: Option<i32>,
    pub labels: Vec<String>,
}

impl AIClient {
    pub async fn break_down_epic(&self, epic_title: &str, epic_description: &str) -> Result<Vec<SubTicketSuggestion>> {
        let prompt = format!(r#"
You are a project manager breaking down an epic into implementable sub-tickets.

Epic: {}
Description: {}

Generate 3-8 sub-tickets. For each, provide:
- title: Brief, actionable title
- description: What needs to be done
- story_points: Estimated effort (1, 2, 3, 5, or 8)
- labels: Relevant tags (e.g., "frontend", "backend", "database")

Return ONLY valid JSON array of objects with these exact keys.
"#, epic_title, epic_description);

        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4")
            .messages(vec![
                ChatCompletionRequestMessage::System(
                    ChatCompletionRequestSystemMessage {
                        content: ChatCompletionRequestSystemMessageContent::Text(
                            "You are a helpful project planning assistant.".to_string()
                        ),
                        ..Default::default()
                    }
                ),
                ChatCompletionRequestMessage::User(
                    ChatCompletionRequestUserMessage {
                        content: ChatCompletionRequestUserMessageContent::Text(prompt),
                        ..Default::default()
                    }
                )
            ])
            .build()?;

        let response = self.client.chat().create(request).await?;
        let content = response.choices[0].message.content
            .as_ref()
            .ok_or_else(|| anyhow!("No response content"))?;

        let suggestions: Vec<SubTicketSuggestion> = serde_json::from_str(content)?;
        Ok(suggestions)
    }
}
```

Commit: `git commit -m "feat: implement AI epic breakdown logic"`

**Task 3: Add API Endpoint**

Add to `jility-server/src/api/tickets.rs`:
```rust
pub async fn generate_subtasks(
    State(state): State<AppState>,
    Path(ticket_id): Path<String>,
) -> ApiResult<Json<Vec<SubTicketSuggestion>>> {
    let ticket = get_ticket_by_id(&ticket_id, &state).await?;

    let suggestions = state.ai_client
        .break_down_epic(&ticket.title, &ticket.description.unwrap_or_default())
        .await
        .map_err(|e| ApiError::Internal(format!("AI error: {}", e)))?;

    Ok(Json(suggestions))
}
```

Register: `.route("/api/tickets/:id/generate-subtasks", post(tickets::generate_subtasks))`

Commit: `git commit -m "feat: add generate subtasks API endpoint"`

**Task 4: Create Frontend Dialog**

Create `jility-web/components/ticket/generate-subtasks-dialog.tsx`:
```typescript
'use client'

import { useState } from 'react'
import { Sparkles, Check } from 'lucide-react'
import { api } from '@/lib/api'

interface SubtaskSuggestion {
  title: string
  description: string
  story_points?: number
  labels: string[]
}

interface GenerateSubtasksDialogProps {
  ticketId: string
  onSuccess: () => void
}

export function GenerateSubtasksDialog({ ticketId, onSuccess }: GenerateSubtasksDialogProps) {
  const [loading, setLoading] = useState(false)
  const [suggestions, setSuggestions] = useState<SubtaskSuggestion[]>([])
  const [selected, setSelected] = useState<Set<number>>(new Set())

  async function generate() {
    setLoading(true)
    try {
      const data = await api.generateSubtasks(ticketId)
      setSuggestions(data)
      setSelected(new Set(data.map((_, i) => i))) // Select all by default
    } catch (error) {
      console.error('Failed to generate subtasks:', error)
    } finally {
      setLoading(false)
    }
  }

  async function createSelected() {
    const toCreate = suggestions.filter((_, i) => selected.has(i))

    for (const suggestion of toCreate) {
      await api.createTicket(workspace, {
        ...suggestion,
        parent_id: ticketId,
      })
    }

    onSuccess()
  }

  return (
    <div className="space-y-4">
      {suggestions.length === 0 ? (
        <button
          onClick={generate}
          disabled={loading}
          className="flex items-center gap-2 px-4 py-2 bg-primary text-primary-foreground rounded-lg"
        >
          <Sparkles className="h-4 w-4" />
          {loading ? 'Generating...' : 'Generate Sub-tickets with AI'}
        </button>
      ) : (
        <>
          <h3 className="font-semibold">Review AI Suggestions</h3>
          <div className="space-y-2">
            {suggestions.map((suggestion, i) => (
              <label
                key={i}
                className="flex items-start gap-3 p-3 bg-card border-border border rounded-lg cursor-pointer hover:bg-muted"
              >
                <input
                  type="checkbox"
                  checked={selected.has(i)}
                  onChange={(e) => {
                    const newSelected = new Set(selected)
                    if (e.target.checked) {
                      newSelected.add(i)
                    } else {
                      newSelected.delete(i)
                    }
                    setSelected(newSelected)
                  }}
                  className="mt-1"
                />
                <div className="flex-1">
                  <div className="font-medium">{suggestion.title}</div>
                  <div className="text-sm text-muted-foreground mt-1">
                    {suggestion.description}
                  </div>
                  <div className="flex gap-2 mt-2">
                    {suggestion.story_points && (
                      <span className="text-xs px-2 py-1 bg-primary/10 text-primary rounded">
                        {suggestion.story_points} pts
                      </span>
                    )}
                    {suggestion.labels.map(label => (
                      <span key={label} className="text-xs px-2 py-1 bg-muted rounded">
                        {label}
                      </span>
                    ))}
                  </div>
                </div>
              </label>
            ))}
          </div>
          <div className="flex gap-2 justify-end">
            <button
              onClick={() => setSuggestions([])}
              className="px-4 py-2 bg-secondary text-secondary-foreground rounded-lg"
            >
              Cancel
            </button>
            <button
              onClick={createSelected}
              disabled={selected.size === 0}
              className="px-4 py-2 bg-primary text-primary-foreground rounded-lg disabled:opacity-50"
            >
              Create {selected.size} Sub-tickets
            </button>
          </div>
        </>
      )}
    </div>
  )
}
```

Commit: `git commit -m "feat: create AI subtask generation dialog"`

**Testing:**
1. Create an epic ticket with detailed description
2. Click "Generate Sub-tickets with AI"
3. Verify AI generates 3-8 suggestions
4. Review suggestions for quality
5. Uncheck some, keep others
6. Click "Create" - verify tickets created with parent_id
7. Check that labels and story points are set

---

## 5.3 Smart Git Integration (JIL-17) - 8 Story Points

### Overview
Auto-link commits to tickets via webhooks, parse ticket IDs from commit messages, track PR status.

### Architecture
- GitHub webhook endpoint receives push events
- Parse commit messages for ticket references (JIL-123, #123)
- Auto-link commits to tickets
- Track PR status and auto-update ticket on merge

### Files to Create
1. `jility-server/src/webhooks/mod.rs` - Webhook handler
2. `jility-server/src/webhooks/github.rs` - GitHub webhook
3. `jility-server/src/git/parser.rs` - Commit message parser
4. `jility-web/components/ticket/git-links.tsx` - Show linked commits

### Implementation Steps

**Task 1: Add Webhook Endpoint**

Create `jility-server/src/webhooks/github.rs`:
```rust
use axum::{Json, extract::State};
use hmac::{Hmac, Mac};
use sha2::Sha256;

#[derive(Debug, Deserialize)]
pub struct GitHubPushEvent {
    pub commits: Vec<Commit>,
    pub repository: Repository,
}

#[derive(Debug, Deserialize)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub author: Author,
}

pub async fn github_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<GitHubPushEvent>,
) -> ApiResult<Json<()>> {
    // Verify webhook signature
    verify_signature(&headers, &payload, &state.webhook_secret)?;

    // Process each commit
    for commit in payload.commits {
        let ticket_ids = parse_ticket_references(&commit.message);

        for ticket_id in ticket_ids {
            link_commit_to_ticket(
                &state,
                &ticket_id,
                &commit.id,
                &commit.message,
                &commit.author.name,
            ).await?;
        }
    }

    Ok(Json(()))
}

fn verify_signature(headers: &HeaderMap, payload: &GitHubPushEvent, secret: &str) -> Result<()> {
    let signature = headers
        .get("X-Hub-Signature-256")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| anyhow!("Missing signature"))?;

    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())?;
    mac.update(serde_json::to_string(payload)?.as_bytes());

    let expected = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));

    if signature != expected {
        return Err(anyhow!("Invalid signature"));
    }

    Ok(())
}
```

Register: `.route("/webhooks/github", post(webhooks::github_webhook))`

Commit: `git commit -m "feat: add GitHub webhook endpoint"`

**Task 2: Implement Commit Parser**

Create `jility-server/src/git/parser.rs`:
```rust
use regex::Regex;

pub fn parse_ticket_references(message: &str) -> Vec<String> {
    let mut refs = Vec::new();

    // Match JIL-123 format
    let re1 = Regex::new(r"(JIL-\d+)").unwrap();
    for cap in re1.captures_iter(message) {
        refs.push(cap[1].to_string());
    }

    // Match #123 format
    let re2 = Regex::new(r"#(\d+)").unwrap();
    for cap in re2.captures_iter(message) {
        refs.push(format!("JIL-{}", &cap[1]));
    }

    // Match "fixes JIL-123" or "closes #45"
    let re3 = Regex::new(r"(?:fix(?:es)?|close(?:s)?|resolve(?:s)?)\s+(JIL-\d+|#\d+)").unwrap();
    for cap in re3.captures_iter(message) {
        let ticket_ref = if cap[1].starts_with('#') {
            format!("JIL-{}", &cap[1][1..])
        } else {
            cap[1].to_string()
        };
        refs.push(ticket_ref);
    }

    refs.into_iter().collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ticket_references() {
        let msg = "fix: resolve JIL-123 and closes #45";
        let refs = parse_ticket_references(msg);
        assert!(refs.contains(&"JIL-123".to_string()));
        assert!(refs.contains(&"JIL-45".to_string()));
    }
}
```

Commit: `git commit -m "feat: implement commit message parser"`

**Task 3: Link Commits to Tickets**

Add to `jility-server/src/api/tickets.rs`:
```rust
async fn link_commit_to_ticket(
    state: &AppState,
    ticket_number: &str,
    commit_hash: &str,
    commit_message: &str,
    author: &str,
) -> Result<()> {
    // Find ticket by number
    let ticket = Ticket::find()
        .filter(ticket::Column::Number.eq(ticket_number))
        .one(state.db.as_ref())
        .await?
        .ok_or_else(|| anyhow!("Ticket {} not found", ticket_number))?;

    // Create commit link
    let link = commit_link::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(ticket.id),
        commit_hash: Set(commit_hash.to_string()),
        commit_message: Set(Some(commit_message.to_string())),
        linked_at: Set(Utc::now()),
        linked_by: Set(author.to_string()),
    };

    link.insert(state.db.as_ref()).await?;

    // Auto-transition if commit message indicates completion
    if commit_message.to_lowercase().contains("fixes") ||
       commit_message.to_lowercase().contains("closes") {
        auto_transition_ticket(&ticket, state).await?;
    }

    Ok(())
}

async fn auto_transition_ticket(ticket: &Ticket, state: &AppState) -> Result<()> {
    // If ticket is in progress, move to review
    if ticket.status == "in_progress" {
        let mut active: ticket::ActiveModel = ticket.clone().into();
        active.status = Set("review".to_string());
        active.update(state.db.as_ref()).await?;
    }
    Ok(())
}
```

Commit: `git commit -m "feat: auto-link commits and transition tickets"`

**Task 4: Show Linked Commits in UI**

Create `jility-web/components/ticket/git-links.tsx`:
```typescript
'use client'

import { GitCommit, ExternalLink } from 'lucide-react'

interface GitLink {
  id: string
  commit_hash: string
  commit_message?: string
  linked_at: string
  linked_by: string
}

interface GitLinksProps {
  links: GitLink[]
  repoUrl?: string
}

export function GitLinks({ links, repoUrl }: GitLinksProps) {
  if (links.length === 0) return null

  return (
    <div className="bg-card border-border border rounded-lg p-4">
      <div className="flex items-center gap-2 mb-3">
        <GitCommit className="h-5 w-5" />
        <h3 className="font-semibold">Linked Commits ({links.length})</h3>
      </div>
      <div className="space-y-2">
        {links.map(link => (
          <div key={link.id} className="flex items-start gap-3 p-3 bg-secondary rounded-lg">
            <code className="text-xs font-mono bg-muted px-2 py-1 rounded">
              {link.commit_hash.substring(0, 7)}
            </code>
            <div className="flex-1 min-w-0">
              <div className="text-sm truncate">{link.commit_message}</div>
              <div className="text-xs text-muted-foreground mt-1">
                {link.linked_by} • {new Date(link.linked_at).toLocaleDateString()}
              </div>
            </div>
            {repoUrl && (
              <a
                href={`${repoUrl}/commit/${link.commit_hash}`}
                target="_blank"
                rel="noopener noreferrer"
                className="text-primary hover:underline flex-shrink-0"
              >
                <ExternalLink className="h-4 w-4" />
              </a>
            )}
          </div>
        ))}
      </div>
    </div>
  )
}
```

Commit: `git commit -m "feat: create GitLinks component"`

**Task 5: GitHub Webhook Setup Documentation**

Create `docs/github-webhook-setup.md`:
```markdown
# GitHub Webhook Setup

## 1. Generate Webhook Secret

```bash
openssl rand -hex 32
```

Add to `.env`:
```
GITHUB_WEBHOOK_SECRET=your_secret_here
```

## 2. Configure GitHub Webhook

1. Go to your repo → Settings → Webhooks
2. Click "Add webhook"
3. Payload URL: `https://your-domain.com/webhooks/github`
4. Content type: `application/json`
5. Secret: (paste your secret)
6. Events: Select "Push events"
7. Active: ✓
8. Click "Add webhook"

## 3. Test

```bash
git commit -m "fix: resolve JIL-123 authentication bug"
git push
```

Check Jility - JIL-123 should have linked commit.
```

Commit: `git commit -m "docs: add GitHub webhook setup guide"`

**Testing:**
1. Set up webhook secret in .env
2. Configure GitHub webhook (or test with curl)
3. Make commit: `git commit -m "fix: JIL-123 broken link"`
4. Push to GitHub
5. Check ticket JIL-123 - verify commit linked
6. Test auto-transition: commit with "fixes JIL-456"
7. Verify ticket moved to review

---

## Final Integration Testing

After all features implemented, test the complete workflow:

1. **Phase 3 Integration**
   - Create sprint with swimlanes enabled
   - Group by assignee
   - View burndown chart

2. **Phase 4 Integration**
   - Press Cmd+K
   - Search for tickets
   - Apply filters to board
   - Save filter preset

3. **Phase 5 Integration**
   - Create epic via MCP: `create_ticket`
   - Generate subtasks via AI
   - Make commits with ticket refs
   - Verify auto-linking works
   - Check ticket auto-transitions

---

## Deployment Checklist

- [ ] All TypeScript compiles without errors
- [ ] All Rust compiles without errors
- [ ] Backend migrations run successfully
- [ ] Frontend builds successfully
- [ ] All manual tests pass
- [ ] Environment variables documented
- [ ] GitHub webhook configured
- [ ] OpenAI API key set (for Phase 5.2)

---

## Notes

**Backend:** Most backend work is already done. Main additions are:
- Batch endpoints for Phase 5.1
- AI integration for Phase 5.2
- Webhook handler for Phase 5.3

**Frontend:** Largest workload. Each feature needs:
- TypeScript components
- Responsive design (mobile-first)
- Theme variable usage (no hardcoded colors)
- Error handling
- Loading states

**Testing:** Each feature should be tested:
1. Desktop browser
2. Mobile browser
3. Dark mode
4. Light mode
5. Error cases

**Commits:** Frequent, atomic commits following conventional commit format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation
- `deps:` for dependencies
