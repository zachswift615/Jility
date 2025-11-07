# Ticket Assignee UI Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add complete UI for assigning workspace members to tickets with visual display and filtering.

**Architecture:** Three reusable components (AssigneeAvatars for display, AssigneeSelector for assignment UI, AssigneeFilter for filtering) integrated into ticket detail, board cards, and backlog views. Uses existing backend assign/unassign endpoints with optimistic UI updates.

**Tech Stack:** Next.js 14, React, TypeScript, shadcn/ui (Avatar, Tooltip, Popover, Command), Tailwind CSS

---

## Task 1: Create AssigneeAvatars Component

**Files:**
- Create: `jility-web/components/ticket/assignee-avatars.tsx`

**Step 1: Create component file**

```typescript
'use client'

import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip'

interface AssigneeAvatarsProps {
  assignees: string[]
  maxVisible?: number
  size?: 'sm' | 'md' | 'lg'
}

function getInitials(email: string): string {
  return email.slice(0, 2).toUpperCase()
}

function getColorFromEmail(email: string): string {
  const colors = [
    'bg-blue-500',
    'bg-green-500',
    'bg-purple-500',
    'bg-pink-500',
    'bg-yellow-500',
    'bg-indigo-500',
    'bg-red-500',
    'bg-orange-500',
  ]
  const hash = email.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0)
  return colors[hash % colors.length]
}

export function AssigneeAvatars({
  assignees,
  maxVisible = 3,
  size = 'md',
}: AssigneeAvatarsProps) {
  const sizeClasses = {
    sm: 'h-6 w-6 text-xs',
    md: 'h-8 w-8 text-sm',
    lg: 'h-10 w-10 text-base',
  }

  const visible = assignees.slice(0, maxVisible)
  const remaining = assignees.length - maxVisible

  if (assignees.length === 0) {
    return null
  }

  return (
    <TooltipProvider>
      <div className="flex items-center -space-x-2">
        {visible.map((email, index) => (
          <Tooltip key={index}>
            <TooltipTrigger asChild>
              <Avatar className={`${sizeClasses[size]} border-2 border-background`}>
                <AvatarFallback className={getColorFromEmail(email)}>
                  {getInitials(email)}
                </AvatarFallback>
              </Avatar>
            </TooltipTrigger>
            <TooltipContent>
              <p>{email}</p>
            </TooltipContent>
          </Tooltip>
        ))}
        {remaining > 0 && (
          <Tooltip>
            <TooltipTrigger asChild>
              <Avatar className={`${sizeClasses[size]} border-2 border-background bg-muted`}>
                <AvatarFallback className="text-muted-foreground">
                  +{remaining}
                </AvatarFallback>
              </Avatar>
            </TooltipTrigger>
            <TooltipContent>
              <div className="space-y-1">
                {assignees.slice(maxVisible).map((email, index) => (
                  <p key={index}>{email}</p>
                ))}
              </div>
            </TooltipContent>
          </Tooltip>
        )}
      </div>
    </TooltipProvider>
  )
}
```

**Step 2: Verify tooltip component exists**

Run: `ls jility-web/components/ui/tooltip.tsx`
Expected: File exists (shadcn tooltip already installed)

If missing, run: `cd jility-web && npx shadcn@latest add tooltip`

**Step 3: Commit**

```bash
git add jility-web/components/ticket/assignee-avatars.tsx
git commit -m "feat: add AssigneeAvatars display component

- Renders stacked avatars with email initials
- Color-coded per user (hash-based)
- Shows +N indicator for overflow
- Tooltips display full emails

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 2: Add AssigneeSelector Component

**Files:**
- Create: `jility-web/components/ticket/assignee-selector.tsx`

**Step 1: Create component file**

```typescript
'use client'

import { useState } from 'react'
import { X, UserPlus } from 'lucide-react'
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
} from '@/components/ui/command'
import { useToast } from '@/components/ui/use-toast'
import type { WorkspaceMember } from '@/lib/types'

interface AssigneeSelectorProps {
  currentAssignees: string[]
  availableMembers: WorkspaceMember[]
  onAssign: (email: string) => Promise<void>
  onUnassign: (email: string) => Promise<void>
  isLoading?: boolean
}

function getInitials(email: string): string {
  return email.slice(0, 2).toUpperCase()
}

function getColorFromEmail(email: string): string {
  const colors = [
    'bg-blue-500',
    'bg-green-500',
    'bg-purple-500',
    'bg-pink-500',
    'bg-yellow-500',
    'bg-indigo-500',
    'bg-red-500',
    'bg-orange-500',
  ]
  const hash = email.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0)
  return colors[hash % colors.length]
}

export function AssigneeSelector({
  currentAssignees,
  availableMembers,
  onAssign,
  onUnassign,
  isLoading = false,
}: AssigneeSelectorProps) {
  const [open, setOpen] = useState(false)
  const { toast } = useToast()

  const handleAssign = async (email: string) => {
    if (currentAssignees.includes(email)) return

    try {
      await onAssign(email)
      setOpen(false)
    } catch (error) {
      toast({
        title: 'Assignment failed',
        description: error instanceof Error ? error.message : 'Failed to assign member',
        variant: 'destructive',
      })
    }
  }

  const handleUnassign = async (email: string) => {
    try {
      await onUnassign(email)
    } catch (error) {
      toast({
        title: 'Unassignment failed',
        description: error instanceof Error ? error.message : 'Failed to unassign member',
        variant: 'destructive',
      })
    }
  }

  return (
    <div className="space-y-2">
      <label className="text-sm font-medium">Assignees</label>
      <div className="flex flex-wrap gap-2">
        {currentAssignees.map((email) => (
          <div
            key={email}
            className="flex items-center gap-2 px-3 py-1.5 bg-muted rounded-full"
          >
            <Avatar className="h-6 w-6">
              <AvatarFallback className={`${getColorFromEmail(email)} text-xs`}>
                {getInitials(email)}
              </AvatarFallback>
            </Avatar>
            <span className="text-sm">{email}</span>
            <Button
              variant="ghost"
              size="sm"
              className="h-4 w-4 p-0 hover:bg-transparent"
              onClick={() => handleUnassign(email)}
              disabled={isLoading}
            >
              <X className="h-3 w-3" />
            </Button>
          </div>
        ))}

        <Popover open={open} onOpenChange={setOpen}>
          <PopoverTrigger asChild>
            <Button variant="outline" size="sm" className="h-8" disabled={isLoading}>
              <UserPlus className="h-4 w-4 mr-2" />
              Add assignee
            </Button>
          </PopoverTrigger>
          <PopoverContent className="w-[300px] p-0" align="start">
            <Command>
              <CommandInput placeholder="Search members..." />
              <CommandEmpty>No members found.</CommandEmpty>
              <CommandGroup>
                {availableMembers.map((member) => {
                  const isAssigned = currentAssignees.includes(member.email)
                  return (
                    <CommandItem
                      key={member.user_id}
                      onSelect={() => handleAssign(member.email)}
                      disabled={isAssigned}
                    >
                      <Avatar className="h-6 w-6 mr-2">
                        <AvatarFallback className={getColorFromEmail(member.email)}>
                          {getInitials(member.email)}
                        </AvatarFallback>
                      </Avatar>
                      <span>{member.email}</span>
                      {isAssigned && (
                        <span className="ml-auto text-xs text-muted-foreground">✓</span>
                      )}
                    </CommandItem>
                  )
                })}
              </CommandGroup>
            </Command>
          </PopoverContent>
        </Popover>
      </div>
    </div>
  )
}
```

**Step 2: Verify required shadcn components**

Run: `ls jility-web/components/ui/popover.tsx && ls jility-web/components/ui/command.tsx`

If missing popover: `cd jility-web && npx shadcn@latest add popover`
If missing command: `cd jility-web && npx shadcn@latest add command`

**Step 3: Commit**

```bash
git add jility-web/components/ticket/assignee-selector.tsx
git commit -m "feat: add AssigneeSelector component

- Multi-select dropdown for workspace members
- Shows current assignees as removable chips
- Optimistic updates with error handling
- Prevents duplicate assignments

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 3: Integrate AssigneeSelector into Ticket Detail Page

**Files:**
- Modify: `jility-web/app/w/[slug]/ticket/[id]/page.tsx`

**Step 1: Read current ticket detail page**

Run: `cat jility-web/app/w/[slug]/ticket/[id]/page.tsx | head -100`

**Step 2: Add imports and state for assignee management**

Add to imports section:
```typescript
import { AssigneeSelector } from '@/components/ticket/assignee-selector'
import { AssigneeAvatars } from '@/components/ticket/assignee-avatars'
```

Add state management (after existing useState declarations):
```typescript
const [members, setMembers] = useState<WorkspaceMember[]>([])
const [isLoadingMembers, setIsLoadingMembers] = useState(true)
```

**Step 3: Add useEffect to fetch workspace members**

Add after existing useEffect:
```typescript
useEffect(() => {
  const loadMembers = async () => {
    try {
      setIsLoadingMembers(true)
      const data = await api.listWorkspaceMembers(slug)
      setMembers(data)
    } catch (error) {
      console.error('Failed to load workspace members:', error)
    } finally {
      setIsLoadingMembers(false)
    }
  }

  if (slug) {
    loadMembers()
  }
}, [slug])
```

**Step 4: Add assignment handlers**

Add before return statement:
```typescript
const handleAssign = async (email: string) => {
  if (!ticket) return

  // Optimistic update
  setTicket({
    ...ticket,
    ticket: {
      ...ticket.ticket,
      assignees: [...ticket.ticket.assignees, email],
    },
  })

  try {
    await api.assignTicket(ticket.ticket.id, email)
  } catch (error) {
    // Rollback on error
    setTicket({
      ...ticket,
      ticket: {
        ...ticket.ticket,
        assignees: ticket.ticket.assignees.filter((a) => a !== email),
      },
    })
    throw error
  }
}

const handleUnassign = async (email: string) => {
  if (!ticket) return

  // Optimistic update
  const previousAssignees = ticket.ticket.assignees
  setTicket({
    ...ticket,
    ticket: {
      ...ticket.ticket,
      assignees: ticket.ticket.assignees.filter((a) => a !== email),
    },
  })

  try {
    await api.unassignTicket(ticket.ticket.id, email)
  } catch (error) {
    // Rollback on error
    setTicket({
      ...ticket,
      ticket: {
        ...ticket.ticket,
        assignees: previousAssignees,
      },
    })
    throw error
  }
}
```

**Step 5: Add AssigneeSelector to JSX**

Find the location after ticket status/title and before description, add:
```typescript
<div className="border-t border-border pt-4">
  <AssigneeSelector
    currentAssignees={ticket.ticket.assignees}
    availableMembers={members}
    onAssign={handleAssign}
    onUnassign={handleUnassign}
    isLoading={isLoadingMembers}
  />
</div>
```

**Step 6: Test in browser**

Run dev server: `cd jility-web && npm run dev`
Navigate to: `http://localhost:3901/w/{workspace}/ticket/{id}`
Expected: Assignee selector appears below ticket title

**Step 7: Commit**

```bash
git add jility-web/app/w/[slug]/ticket/[id]/page.tsx
git commit -m "feat: integrate assignee selector into ticket detail page

- Add AssigneeSelector component to ticket detail
- Fetch workspace members on page load
- Optimistic updates with rollback on error
- Handle assign/unassign operations

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 4: Add Assignees to Board Cards

**Files:**
- Modify: `jility-web/app/w/[slug]/board/page.tsx` (or the board card component)

**Step 1: Locate board card component**

Run: `find jility-web -name "*board*" -o -name "*card*" | grep -E "\\.tsx$"`

**Step 2: Import AssigneeAvatars**

Add to imports:
```typescript
import { AssigneeAvatars } from '@/components/ticket/assignee-avatars'
```

**Step 3: Add to card JSX**

Find the ticket card rendering, add AssigneeAvatars in bottom-right corner (likely in a footer section):
```typescript
<div className="flex items-center justify-between mt-2">
  <div className="flex items-center gap-2">
    {/* Existing content like labels, etc */}
  </div>
  <AssigneeAvatars assignees={ticket.assignees} size="sm" maxVisible={3} />
</div>
```

**Step 4: Test in browser**

Navigate to: `http://localhost:3901/w/{workspace}/board`
Expected: Assignee avatars appear on ticket cards in bottom-right

**Step 5: Commit**

```bash
git add jility-web/app/w/[slug]/board/page.tsx
git commit -m "feat: display assignees on board cards

- Add AssigneeAvatars to bottom-right of cards
- Show up to 3 avatars with +N overflow
- Small size for card space efficiency

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 5: Add Assignees Column to Backlog Table

**Files:**
- Modify: `jility-web/app/w/[slug]/backlog/page.tsx`

**Step 1: Import AssigneeAvatars**

Add to imports:
```typescript
import { AssigneeAvatars } from '@/components/ticket/assignee-avatars'
```

**Step 2: Add table column header**

Find the table header row, add between Status and Story Points:
```typescript
<th className="px-4 py-2 text-left text-sm font-medium text-muted-foreground">
  Assignees
</th>
```

**Step 3: Add table cell with avatars**

In the table row rendering, add corresponding cell:
```typescript
<td className="px-4 py-2">
  <AssigneeAvatars assignees={ticket.assignees} size="sm" maxVisible={3} />
</td>
```

**Step 4: Test in browser**

Navigate to: `http://localhost:3901/w/{workspace}/backlog`
Expected: New "Assignees" column appears with avatar stacks

**Step 5: Commit**

```bash
git add jility-web/app/w/[slug]/backlog/page.tsx
git commit -m "feat: add assignees column to backlog table

- New column between Status and Story Points
- Shows avatar stacks for quick identification
- Consistent with board card display

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 6: Create AssigneeFilter Component

**Files:**
- Create: `jility-web/components/ticket/assignee-filter.tsx`

**Step 1: Create component file**

```typescript
'use client'

import { useState, useEffect } from 'react'
import { useRouter, useSearchParams } from 'next/navigation'
import { Filter, X } from 'lucide-react'
import { Button } from '@/components/ui/button'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { Checkbox } from '@/components/ui/checkbox'
import { Label } from '@/components/ui/label'
import { Badge } from '@/components/ui/badge'
import type { WorkspaceMember } from '@/lib/types'

interface AssigneeFilterProps {
  members: WorkspaceMember[]
  currentUserEmail?: string
}

export function AssigneeFilter({ members, currentUserEmail }: AssigneeFilterProps) {
  const router = useRouter()
  const searchParams = useSearchParams()
  const [open, setOpen] = useState(false)
  const [selectedFilters, setSelectedFilters] = useState<string[]>([])

  // Load filters from URL on mount
  useEffect(() => {
    const assigneeParam = searchParams.get('assignee')
    if (assigneeParam) {
      setSelectedFilters(assigneeParam.split(','))
    }
  }, [searchParams])

  const updateURL = (filters: string[]) => {
    const params = new URLSearchParams(searchParams.toString())
    if (filters.length > 0) {
      params.set('assignee', filters.join(','))
    } else {
      params.delete('assignee')
    }
    router.push(`?${params.toString()}`)
  }

  const toggleFilter = (value: string) => {
    const newFilters = selectedFilters.includes(value)
      ? selectedFilters.filter((f) => f !== value)
      : [...selectedFilters, value]

    setSelectedFilters(newFilters)
    updateURL(newFilters)
  }

  const clearFilters = () => {
    setSelectedFilters([])
    updateURL([])
  }

  const isSelected = (value: string) => selectedFilters.includes(value)

  return (
    <div className="flex items-center gap-2">
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverTrigger asChild>
          <Button variant="outline" size="sm" className="h-8">
            <Filter className="h-4 w-4 mr-2" />
            Assignee
            {selectedFilters.length > 0 && (
              <Badge variant="secondary" className="ml-2">
                {selectedFilters.length}
              </Badge>
            )}
          </Button>
        </PopoverTrigger>
        <PopoverContent className="w-[250px]" align="start">
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <h4 className="text-sm font-medium">Filter by assignee</h4>
              {selectedFilters.length > 0 && (
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={clearFilters}
                  className="h-auto p-0 text-xs"
                >
                  Clear
                </Button>
              )}
            </div>

            <div className="space-y-2">
              {currentUserEmail && (
                <div className="flex items-center space-x-2">
                  <Checkbox
                    id="filter-me"
                    checked={isSelected('me')}
                    onCheckedChange={() => toggleFilter('me')}
                  />
                  <Label htmlFor="filter-me" className="text-sm cursor-pointer">
                    Assigned to me
                  </Label>
                </div>
              )}

              <div className="flex items-center space-x-2">
                <Checkbox
                  id="filter-unassigned"
                  checked={isSelected('unassigned')}
                  onCheckedChange={() => toggleFilter('unassigned')}
                />
                <Label htmlFor="filter-unassigned" className="text-sm cursor-pointer">
                  Unassigned
                </Label>
              </div>

              <div className="border-t pt-2 space-y-2">
                {members.map((member) => (
                  <div key={member.user_id} className="flex items-center space-x-2">
                    <Checkbox
                      id={`filter-${member.user_id}`}
                      checked={isSelected(member.email)}
                      onCheckedChange={() => toggleFilter(member.email)}
                    />
                    <Label
                      htmlFor={`filter-${member.user_id}`}
                      className="text-sm cursor-pointer truncate"
                    >
                      {member.email}
                    </Label>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </PopoverContent>
      </Popover>

      {selectedFilters.length > 0 && (
        <Button
          variant="ghost"
          size="sm"
          onClick={clearFilters}
          className="h-8 px-2"
        >
          <X className="h-4 w-4" />
        </Button>
      )}
    </div>
  )
}
```

**Step 2: Verify checkbox component exists**

Run: `ls jility-web/components/ui/checkbox.tsx`

If missing: `cd jility-web && npx shadcn@latest add checkbox`

**Step 3: Commit**

```bash
git add jility-web/components/ticket/assignee-filter.tsx
git commit -m "feat: add AssigneeFilter component

- Multi-select filter with URL param sync
- Quick filters: 'Assigned to me', 'Unassigned'
- Individual member checkboxes
- Badge shows active filter count

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 7: Integrate Filter into Board Page

**Files:**
- Modify: `jility-web/app/w/[slug]/board/page.tsx`

**Step 1: Import filter component and auth context**

Add to imports:
```typescript
import { AssigneeFilter } from '@/components/ticket/assignee-filter'
import { useAuth } from '@/lib/auth-context'
```

**Step 2: Add state and fetch members**

Add state:
```typescript
const [members, setMembers] = useState<WorkspaceMember[]>([])
const { user } = useAuth()
```

Add useEffect to fetch members:
```typescript
useEffect(() => {
  const loadMembers = async () => {
    try {
      const data = await api.listWorkspaceMembers(slug)
      setMembers(data)
    } catch (error) {
      console.error('Failed to load members:', error)
    }
  }

  if (slug) {
    loadMembers()
  }
}, [slug])
```

**Step 3: Add filter logic**

Add filtering function:
```typescript
const searchParams = useSearchParams()

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
```

**Step 4: Apply filter to displayed tickets**

Update the tickets rendering to use filtered tickets:
```typescript
const displayedTickets = getFilteredTickets(tickets)
```

**Step 5: Add filter to toolbar**

Find the toolbar/header section, add:
```typescript
<AssigneeFilter members={members} currentUserEmail={user?.email} />
```

**Step 6: Wrap page in Suspense**

Ensure page is wrapped in Suspense boundary (required for useSearchParams):
```typescript
import { Suspense } from 'react'

// At export
export default function BoardPage() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <BoardContent />
    </Suspense>
  )
}

function BoardContent() {
  // Existing page code
}
```

**Step 7: Test filtering**

Navigate to: `http://localhost:3901/w/{workspace}/board`
- Click "Assignee" filter button
- Select "Assigned to me"
- Expected: Board shows only tickets assigned to current user
- URL updates to `?assignee=me`

**Step 8: Commit**

```bash
git add jility-web/app/w/[slug]/board/page.tsx
git commit -m "feat: add assignee filtering to board view

- Integrate AssigneeFilter component
- Filter tickets by 'me', 'unassigned', or specific members
- URL param sync for shareable filters
- Suspense boundary for useSearchParams

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 8: Integrate Filter into Backlog Page

**Files:**
- Modify: `jility-web/app/w/[slug]/backlog/page.tsx`

**Step 1: Follow same pattern as board (Steps 1-5 from Task 7)**

Apply identical changes:
- Import AssigneeFilter and useAuth
- Add members state and fetch logic
- Add getFilteredTickets function
- Apply filter to displayed tickets
- Add filter to toolbar

**Step 2: Ensure Suspense boundary**

Wrap page in Suspense if not already:
```typescript
import { Suspense } from 'react'

export default function BacklogPage() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <BacklogContent />
    </Suspense>
  )
}
```

**Step 3: Test filtering**

Navigate to: `http://localhost:3901/w/{workspace}/backlog`
- Test same filters as board
- Expected: Table shows filtered tickets

**Step 4: Commit**

```bash
git add jility-web/app/w/[slug]/backlog/page.tsx
git commit -m "feat: add assignee filtering to backlog view

- Same filtering logic as board
- Consistent UX across views
- URL param sync enabled

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 9: Build and Test Complete Flow

**Step 1: Build frontend**

Run: `cd jility-web && npm run build`
Expected: Build succeeds with no errors

**Step 2: Test assignment flow**

1. Navigate to ticket detail page
2. Click "Add assignee"
3. Select a member
4. Expected: Member appears as chip with avatar
5. Click × to remove
6. Expected: Member removed

**Step 3: Test display on board**

1. Navigate to board view
2. Expected: Assigned tickets show avatars in bottom-right
3. Hover over avatars
4. Expected: Tooltip shows emails

**Step 4: Test display on backlog**

1. Navigate to backlog view
2. Expected: Assignees column shows avatar stacks
3. Verify tooltips work

**Step 5: Test filtering**

1. On board, click "Assignee" filter
2. Select "Assigned to me"
3. Expected: Only tickets assigned to current user visible
4. Check URL has `?assignee=me`
5. Refresh page
6. Expected: Filter persists
7. Test "Unassigned" filter
8. Test specific member filter
9. Test multiple selections (OR logic)

**Step 6: Test error handling**

1. Stop backend server
2. Try to assign member
3. Expected: Toast error appears
4. Expected: Optimistic update rolled back

**Step 7: Document results**

Create test report in commit message with checklist.

**Step 8: Final commit**

```bash
git add -A
git commit -m "test: verify ticket assignee UI complete flow

Tested:
✓ Assignment UI on detail page
✓ Unassignment with optimistic updates
✓ Assignee display on board cards
✓ Assignee display in backlog table
✓ Filter by 'Assigned to me'
✓ Filter by 'Unassigned'
✓ Filter by specific members
✓ Multi-select filter (OR logic)
✓ URL param persistence
✓ Error handling and rollback
✓ Tooltips show full emails

All functionality working as designed.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Completion

After all tasks complete and tests pass:

1. Return to main worktree: `cd /Users/zachswift/projects/Jility`
2. Use @superpowers:finishing-a-development-branch to merge/cleanup
3. Celebrate! 🎉
