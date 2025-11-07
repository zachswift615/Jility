# Ticket Assignee UI Design

**Date:** 2025-11-07
**Status:** Approved

## Overview

Add complete UI for assigning workspace members to tickets, displaying assignees visually, and filtering tickets by assignee.

## Requirements

- Support multiple assignees per ticket
- Assignment UI on ticket detail page
- Display assignees on board cards, backlog rows, and ticket detail
- Filter tickets by assignee on board and backlog views
- Backend endpoints already exist (assign/unassign)

## Architecture

### Components

1. **AssigneeSelector** (Ticket Detail Page)
   - Multi-select interface for managing ticket assignees
   - Displays current assignees as removable avatar chips
   - Add member button opens dropdown of available workspace members
   - Real-time API updates with optimistic UI

2. **AssigneeAvatars** (Reusable Display Component)
   - Circular avatars with email-based initials
   - Horizontal stack with slight overlap
   - Shows "+N" indicator if more than 3 assignees
   - Tooltip displays full email on hover
   - Used on: board cards, backlog rows, ticket header

3. **AssigneeFilter** (Board & Backlog Pages)
   - Multi-select filter dropdown
   - Quick filters: "Assigned to me", "Unassigned"
   - Individual workspace member selection
   - Client-side filtering
   - URL param persistence

### Data Flow

1. Ticket detail page fetches workspace members on mount
2. Assignment/unassignment triggers API call → local state update
3. Board/backlog use existing ticket data (assignees array already included)
4. Filter component reads from URL params, filters displayed tickets

## UI/UX Specification

### Ticket Detail Page

**Location:** Below ticket title/status, before description

**Layout:**
- Horizontal section with "Assignees" label
- Current assignees shown as avatar chips (avatar + email + × remove button)
- "+ Add assignee" button opens member dropdown
- Dropdown shows: avatar, email, checkmark if already assigned

**Interaction:**
- Click member in dropdown → immediate assignment (optimistic update)
- Click × on chip → immediate unassignment (optimistic update)
- Error → toast notification + rollback

### Board Card Display

**Location:** Bottom-right corner of card

**Layout:**
- Avatar stack (max 3 visible)
- "+N" badge if more assignees
- Small size (h-6 w-6)
- Tooltip on hover shows all emails

### Backlog Table Display

**Location:** New "Assignees" column between "Status" and "Story Points"

**Layout:**
- Horizontal avatar stack
- Same styling as board cards
- Tooltip on hover

### Filter UI

**Location:** Toolbar next to search/view controls

**Options:**
- "Assigned to me" (quick filter using current user)
- "Unassigned" (tickets with no assignees)
- Individual workspace members (multi-select checkboxes)
- "Clear filters" option

**Logic:**
- Multiple selections = OR logic (show tickets assigned to ANY selected member)
- Syncs with URL search params: `?assignee=user1@email.com,user2@email.com`
- Empty filter = show all tickets

## Implementation Details

### AssigneeSelector Component

**Dependencies:**
- `api.listWorkspaceMembers(workspaceSlug)` - fetch available members
- `api.assignTicket(ticketId, email)` - add assignee
- `api.unassignTicket(ticketId, email)` - remove assignee

**State:**
- Current assignees (from ticket data)
- Available members (from workspace)
- Loading/error states

**Behavior:**
- Optimistic updates for instant feedback
- Rollback on API error
- Prevent duplicate assignments (UI-level check)

### AssigneeAvatars Component

**Props:**
```typescript
interface AssigneeAvatarsProps {
  assignees: string[]
  maxVisible?: number  // default: 3
  size?: 'sm' | 'md' | 'lg'  // default: 'md'
}
```

**Avatar Generation:**
- Fallback: First 2 characters of email, uppercase
- Background color: Hash email for consistent color per user
- Uses shadcn Avatar + Tooltip components

### AssigneeFilter Component

**State Management:**
- Reads current user from auth context for "Assigned to me"
- Syncs filter selections with URL search params
- Applies filter to ticket list client-side

**Filter Logic:**
```typescript
const filteredTickets = tickets.filter(ticket => {
  if (selectedFilters.length === 0) return true
  if (selectedFilters.includes('unassigned')) {
    return ticket.assignees.length === 0
  }
  if (selectedFilters.includes('me')) {
    return ticket.assignees.includes(currentUser.email)
  }
  return ticket.assignees.some(a => selectedFilters.includes(a))
})
```

## Error Handling

**Assignment Failures:**
- Toast error notification
- Revert optimistic update
- Log error for debugging

**Member List Fetch Failures:**
- Show error state with retry button
- Fallback: disable assignment UI, show read-only assignees

**Network Errors:**
- Graceful degradation: show existing assignees, disable modification
- Clear error messaging to user

**Edge Cases:**
- User removes themselves → allowed (valid action)
- Last assignee removed → ticket becomes unassigned (valid state)
- Workspace member removed → assignee email persists (historical record)
- Duplicate assignment attempt → backend idempotent (no error)

## Testing Checklist

- [ ] Assign single member to ticket
- [ ] Assign multiple members to ticket
- [ ] Remove assignee from ticket
- [ ] Remove all assignees (unassigned state)
- [ ] Assignees display on board cards
- [ ] Assignees display in backlog table
- [ ] Filter by "Assigned to me"
- [ ] Filter by "Unassigned"
- [ ] Filter by specific member
- [ ] Filter by multiple members (OR logic)
- [ ] Clear filters
- [ ] URL params persist filter state
- [ ] Error handling: assignment fails
- [ ] Error handling: member list fails to load
- [ ] Avatar fallback renders correctly
- [ ] Tooltip shows full email on hover
- [ ] Optimistic updates work smoothly

## Future Enhancements (Out of Scope)

- Backend filtering (for large datasets)
- Quick-assign from board cards (inline dropdown)
- Assignee notification system
- Workload balancing view
- Assignee performance metrics
