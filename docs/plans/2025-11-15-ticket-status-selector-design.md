# Ticket Status Selector Design

**Date:** 2025-11-15
**Status:** Approved
**Feature:** Add status change dropdown to ticket detail view

## Overview

Add a status selector to the ticket detail page that allows users to change ticket status directly from the ticket view, similar to JIRA's interface. The selector will be placed in the right sidebar with a color-coded dropdown showing all available statuses.

## User Requirements

1. **Location:** Right sidebar, to the left of the delete button
2. **Interaction:** Always-visible dropdown (not edit mode)
3. **Status transitions:** No restrictions - allow any status to any status
4. **Visual treatment:** Color-coded dropdown with colored dots for each status
5. **Future enhancement:** Configurable status colors in settings (separate feature)

## Design Decisions

### Placement
- **Right sidebar** - next to delete button for easy access to ticket actions
- **Delete button modification** - shrink to icon-only (`<Trash2 />`) to make room
- **Responsive layout:**
  - Mobile: Status selector full-width above delete button
  - Desktop: Status selector and delete button side-by-side

### Interaction Pattern
- **Always-visible dropdown** - no edit mode, always ready to use
- **Immediate save** - status changes on selection (no save/cancel buttons)
- **Optimistic updates** - UI updates immediately, rolls back on error
- **Toast notifications** - confirm success or show errors

### Visual Design
- **Color-coded options** - each status shows a colored dot + label
- **Color system** - uses existing CSS status variables:
  - `--status-backlog`
  - `--status-todo`
  - `--status-in-progress`
  - `--status-review`
  - `--status-done`
  - `--status-blocked`
- **Component** - shadcn Select component for consistency

## Architecture

### Component Structure

```
app/w/[slug]/ticket/[id]/page.tsx
├─ Add handleStatusChange()
├─ Pass status + handler to StatusSelector
└─ Modify delete button to icon-only

components/ticket/status-selector.tsx (NEW)
├─ StatusSelector component
├─ Color-coded Select dropdown
└─ Displays all 6 statuses with dots
```

### API Integration

Uses existing API method:
```typescript
api.updateTicketStatus(ticketId: string, status: string): Promise<Ticket>
```

Endpoint: `PATCH /api/tickets/{id}/status`

### Data Flow

1. User selects new status from dropdown
2. `onStatusChange` handler called with new status
3. Optimistic update - UI shows new status immediately
4. API call to `updateTicketStatus()`
5. On success:
   - Show success toast
   - Reload ticket data (refreshes activity timeline)
6. On error:
   - Rollback to previous status
   - Show error toast

## Implementation Details

### StatusSelector Component

**File:** `components/ticket/status-selector.tsx`

**Props:**
```typescript
interface StatusSelectorProps {
  currentStatus: TicketStatus
  onStatusChange: (newStatus: TicketStatus) => Promise<void>
  disabled?: boolean
}
```

**Key Features:**
- Uses shadcn `Select` component
- Maps over all 6 statuses
- Renders colored dot + label for each option
- Dot color: `style={{backgroundColor: var(--status-${status})}}`
- Label text: uses `getStatusLabel()` helper

### Page Changes

**File:** `app/w/[slug]/ticket/[id]/page.tsx`

**New State Handler:**
```typescript
const handleStatusChange = async (newStatus: TicketStatus) => {
  if (!ticketDetails) return

  // Optimistic update
  const previousStatus = ticketDetails.ticket.status
  setTicketDetails({
    ...ticketDetails,
    ticket: { ...ticketDetails.ticket, status: newStatus }
  })

  try {
    await api.updateTicketStatus(ticketId, newStatus)
    toast({ title: `Status updated to ${getStatusLabel(newStatus)}` })
    await loadTicket() // Refresh activity timeline
  } catch (error) {
    // Rollback on error
    setTicketDetails({
      ...ticketDetails,
      ticket: { ...ticketDetails.ticket, status: previousStatus }
    })
    toast({
      title: 'Failed to update status',
      variant: 'destructive'
    })
  }
}
```

**Sidebar Layout:**
```tsx
<div className="border-b border-border pb-4">
  <div className="flex items-center gap-2">
    <StatusSelector
      currentStatus={ticketDetails.ticket.status}
      onStatusChange={handleStatusChange}
    />
    <AlertDialog>
      <AlertDialogTrigger asChild>
        <Button variant="destructive" size="icon" title="Delete ticket">
          <Trash2 className="h-4 w-4" />
        </Button>
      </AlertDialogTrigger>
      {/* ... rest of delete dialog ... */}
    </AlertDialog>
  </div>
</div>
```

### Delete Button Changes

**Before:**
```tsx
<Button variant="destructive" size="sm" className="w-full">
  <Trash2 className="h-4 w-4 mr-2" />
  Delete Ticket
</Button>
```

**After:**
```tsx
<Button variant="destructive" size="icon" title="Delete ticket">
  <Trash2 className="h-4 w-4" />
</Button>
```

## Error Handling

### Optimistic Updates
- UI updates immediately when user selects new status
- Provides instant feedback for better UX
- Rolls back if API call fails

### API Errors
- Rollback optimistic update
- Show toast: "Failed to update status. Please try again."
- Keep user's selection visible in error toast for context

### Network Errors
- Same rollback behavior as API errors
- Toast includes network error hint if applicable

## Responsive Design

### Mobile Layout
```
┌─────────────────────────────┐
│ [Status Dropdown ▼]         │
│ [Delete Ticket]             │
├─────────────────────────────┤
│ Activity Timeline           │
└─────────────────────────────┘
```

### Desktop Layout
```
┌─────────────────────────────┐
│ [Status Dropdown ▼] [🗑️]   │
├─────────────────────────────┤
│ Activity Timeline           │
└─────────────────────────────┘
```

**Breakpoint:** Use Tailwind `md:` for desktop layout

## Future Enhancements

1. **Configurable Status Colors** (separate feature)
   - Add status color settings to workspace settings
   - Allow admins to customize status colors
   - Store in workspace settings table
   - Update CSS variables dynamically

2. **Status Transition Rules** (if needed)
   - Add optional workflow enforcement
   - Configure allowed transitions per workspace
   - Show warnings for unusual transitions

3. **Keyboard Shortcuts**
   - Quick status change via keyboard
   - e.g., `S` to open status selector

## Testing Checklist

- [ ] Status selector appears in right sidebar
- [ ] Delete button shrinks to icon-only
- [ ] Dropdown shows all 6 statuses with colored dots
- [ ] Selecting new status updates ticket
- [ ] Activity timeline shows status change
- [ ] Toast notification on success
- [ ] Optimistic update works (immediate UI change)
- [ ] Error rollback works (reverts on API failure)
- [ ] Error toast appears on failure
- [ ] Responsive layout works on mobile
- [ ] Works in both light and dark modes
- [ ] Color dots use correct CSS variables
- [ ] Status labels use `getStatusLabel()` formatting

## Dependencies

- Existing: `api.updateTicketStatus()` method ✅
- Existing: `getStatusLabel()` helper ✅
- Existing: CSS status color variables ✅
- Existing: shadcn Select component ✅
- Existing: Toast notification system ✅

## Files to Create/Modify

**New Files:**
- `components/ticket/status-selector.tsx`

**Modified Files:**
- `app/w/[slug]/ticket/[id]/page.tsx`

## Acceptance Criteria

1. ✅ Status selector visible in right sidebar
2. ✅ Delete button is icon-only
3. ✅ Dropdown shows all 6 statuses with colored dots
4. ✅ Changing status updates ticket immediately
5. ✅ Activity timeline reflects status change
6. ✅ Success toast appears on status update
7. ✅ Error handling with rollback works correctly
8. ✅ Responsive design works on mobile and desktop
9. ✅ Works in light and dark modes
10. ✅ No status transition restrictions
