# Mobile-First Redesign & Icon Migration Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Transform Jility into a mobile-friendly app using mobile-first design principles and replace all emojis with Lucide icons.

**Architecture:** Leverage Tailwind CSS responsive utilities (mobile-first breakpoints: sm, md, lg, xl) to create adaptive layouts. Use CSS Grid and Flexbox for fluid layouts. Add bottom navigation for mobile, hide on desktop. Migrate single emoji (🤖) to Lucide's Bot icon.

**Tech Stack:** Next.js 14, React 18, Tailwind CSS, lucide-react (already installed)

---

## Pre-requisites Checklist

**Current State (Verified):**
- ✅ lucide-react already installed (v0.344.0)
- ✅ Tailwind CSS configured with custom theme
- ✅ Next.js 14 App Router
- ✅ Only 1 emoji found: 🤖 in `components/backlog/backlog-view.tsx:187`
- ✅ Navbar already uses Lucide icons

**Files to Modify:**
- `jility-web/app/globals.css` - Add mobile-first responsive utilities
- `jility-web/components/layout/navbar.tsx` - Make responsive
- `jility-web/components/backlog/backlog-view.tsx` - Replace emoji, add mobile layout
- `jility-web/components/kanban/board.tsx` - Add mobile horizontal scroll
- `jility-web/app/board/page.tsx` - Mobile layout
- `jility-web/components/ticket/ticket-header.tsx` - Mobile detail view

**Files to Create:**
- `jility-web/components/layout/mobile-bottom-nav.tsx` - Mobile bottom navigation
- `jility-web/components/layout/mobile-fab.tsx` - Floating Action Button

---

## Task 1: Replace Emoji with Lucide Icon

**Files:**
- Modify: `jility-web/components/backlog/backlog-view.tsx:187`

**Step 1: Import Bot icon from lucide-react**

Edit the imports section:

```typescript
import { Lightbulb, Bot } from 'lucide-react'
```

**Step 2: Replace emoji with Bot icon component**

Find line 187 and replace:
```typescript
// Before (line 187):
🤖 AI Estimate

// After:
<Bot className="h-4 w-4" />
<span className="ml-1">AI Estimate</span>
```

Full button context:
```typescript
<Button
  variant="outline"
  size="sm"
  disabled={groupedTickets.needs_estimation.length === 0}
  className="flex items-center gap-1"
>
  <Bot className="h-4 w-4" />
  AI Estimate
</Button>
```

**Step 3: Verify the change**

Run dev server and navigate to /backlog:
```bash
cd jility-web
npm run dev
```

Expected: Button now shows robot icon instead of emoji

**Step 4: Commit**

```bash
git add jility-web/components/backlog/backlog-view.tsx
git commit -m "fix: replace robot emoji with Lucide Bot icon

- Import Bot icon from lucide-react
- Replace 🤖 emoji with <Bot> component in AI Estimate button
- Maintain consistent icon sizing (h-4 w-4) with rest of app"
```

---

## Task 2: Create Mobile Bottom Navigation Component

**Files:**
- Create: `jility-web/components/layout/mobile-bottom-nav.tsx`

**Step 1: Create the component file**

Create `jility-web/components/layout/mobile-bottom-nav.tsx`:

```typescript
'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { Layers, ListTodo, Boxes, Settings } from 'lucide-react'
import { cn } from '@/lib/utils'

export function MobileBottomNav() {
  const pathname = usePathname()

  const navItems = [
    { href: '/board', label: 'Board', icon: Layers },
    { href: '/backlog', label: 'Backlog', icon: ListTodo },
    { href: '/agents', label: 'Agents', icon: Boxes },
    { href: '/profile', label: 'More', icon: Settings },
  ]

  return (
    <nav className="fixed bottom-0 left-0 right-0 z-50 bg-background border-t border-border md:hidden">
      <div className="flex justify-around items-center h-16 pb-safe">
        {navItems.map(({ href, label, icon: Icon }) => {
          const isActive = pathname === href
          return (
            <Link
              key={href}
              href={href}
              className={cn(
                'flex flex-col items-center justify-center gap-1 px-4 py-2 flex-1',
                'transition-colors',
                isActive
                  ? 'text-primary'
                  : 'text-muted-foreground hover:text-foreground'
              )}
            >
              <Icon className="h-5 w-5" />
              <span className="text-xs font-medium">{label}</span>
            </Link>
          )
        })}
      </div>
    </nav>
  )
}
```

**Step 2: Add padding to body for bottom nav**

Edit `jility-web/app/globals.css` and add mobile padding utility:

```css
/* Add after the existing @layer utilities section (around line 90) */
@layer utilities {
  .text-balance {
    text-wrap: balance;
  }

  /* Mobile bottom nav spacing */
  .pb-safe {
    padding-bottom: env(safe-area-inset-bottom);
  }

  .mb-mobile-nav {
    margin-bottom: 4rem; /* 64px for bottom nav height */
  }
}
```

**Step 3: Import and use in layout**

Edit `jility-web/app/layout.tsx`:

```typescript
import type { Metadata } from 'next'
import './globals.css'
import { Providers } from './providers'
import { Navbar } from '@/components/layout/navbar'
import { MobileBottomNav } from '@/components/layout/mobile-bottom-nav'

export const metadata: Metadata = {
  title: 'Jility - AI-Native Project Management',
  description: 'Beautiful, Linear-inspired project management with AI agents',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className="font-sans antialiased">
        <Providers>
          <div className="min-h-screen flex flex-col">
            <Navbar />
            <main className="flex-1 mb-mobile-nav md:mb-0">
              {children}
            </main>
            <MobileBottomNav />
          </div>
        </Providers>
      </body>
    </html>
  )
}
```

**Step 4: Test mobile bottom nav**

Start dev server and test:
```bash
npm run dev
```

1. Open browser at http://localhost:3901
2. Open DevTools mobile emulation (iPhone SE or similar)
3. Verify bottom nav appears on mobile
4. Verify nav hides on desktop (resize browser > 768px)
5. Click each nav item and verify routing works

Expected: Bottom nav visible on mobile, hidden on desktop

**Step 5: Commit**

```bash
git add jility-web/components/layout/mobile-bottom-nav.tsx
git add jility-web/app/layout.tsx
git add jility-web/app/globals.css
git commit -m "feat: add mobile bottom navigation

- Create MobileBottomNav component with 4 main sections
- Use Lucide icons (Layers, ListTodo, Boxes, Settings)
- Hidden on desktop (md:hidden), visible on mobile
- Add safe-area padding for iOS devices
- Add margin-bottom utility for main content"
```

---

## Task 3: Make Navbar Responsive (Mobile-First)

**Files:**
- Modify: `jility-web/components/layout/navbar.tsx`

**Step 1: Hide desktop nav links on mobile**

Update the nav links section to hide on mobile:

```typescript
// Around line 51, wrap the nav links in a responsive div:
<div className="hidden md:flex items-center gap-1">
  {links.map(({ href, label, icon: Icon }) => (
    <Link
      key={href}
      href={href}
      className={cn(
        'flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium transition-colors',
        pathname === href
          ? 'bg-accent text-accent-foreground'
          : 'text-muted-foreground hover:text-foreground hover:bg-accent/50'
      )}
    >
      <Icon className="h-4 w-4" />
      {label}
    </Link>
  ))}
</div>
```

**Step 2: Make navbar compact on mobile**

Update the navbar container:

```typescript
// Around line 40-42, update the flex container:
<div className="flex h-14 md:h-16 items-center justify-between">
  <div className="flex items-center gap-4 md:gap-8">
    <Link href="/" className="flex items-center gap-2">
      <div className="rounded-lg bg-primary p-1.5 md:p-2">
        <BarChart3 className="h-4 w-4 md:h-5 md:w-5 text-primary-foreground" />
      </div>
      <span className="text-lg md:text-xl font-bold">Jility</span>
    </Link>

    {/* Desktop nav links (hidden on mobile) */}
    <div className="hidden md:flex items-center gap-1">
      {/* ... existing nav links ... */}
    </div>
  </div>

  <div className="flex items-center gap-2 md:gap-4">
    {/* ... existing right-side content ... */}
  </div>
</div>
```

**Step 3: Simplify mobile navbar header**

Update the project switcher button for mobile:

```typescript
// Around line 72-98, make project switcher responsive:
{currentProject ? (
  <Button
    variant="outline"
    size="sm"
    onClick={() => setShowProjectSwitcher(true)}
    className="flex items-center gap-1 md:gap-2 text-xs md:text-sm"
  >
    <div
      className="w-4 h-4 md:w-5 md:h-5 rounded flex items-center justify-center text-white font-semibold text-[10px] md:text-xs"
      style={{
        backgroundColor: currentProject.color || '#5e6ad2',
      }}
    >
      {getProjectIcon(currentProject.name)}
    </div>
    <span className="hidden sm:inline font-medium">{currentProject.name}</span>
    <ChevronDown className="h-3 w-3 md:h-4 md:w-4 opacity-50" />
  </Button>
) : (
  <Button
    variant="outline"
    size="sm"
    onClick={handleCreateNew}
    className="flex items-center gap-2 text-xs md:text-sm"
  >
    <span className="font-medium">Create Project</span>
  </Button>
)}
```

**Step 4: Hide search on small mobile**

Update the CommandPalette to be responsive:

```typescript
// Around line 101:
<div className="hidden sm:block">
  <CommandPalette />
</div>
<ThemeSwitcher />
```

**Step 5: Test responsive navbar**

```bash
npm run dev
```

Test at different breakpoints:
- Mobile (< 640px): Compact logo, no nav links, minimal project switcher
- Tablet (640px - 768px): Show search, compact view
- Desktop (> 768px): Full navbar with all links

Expected: Navbar adapts smoothly across breakpoints

**Step 6: Commit**

```bash
git add jility-web/components/layout/navbar.tsx
git commit -m "feat: make navbar responsive (mobile-first)

- Hide desktop nav links on mobile (use bottom nav instead)
- Reduce navbar height on mobile (h-14 vs h-16)
- Compact logo and icons on small screens
- Hide project name text on very small screens
- Hide search on mobile phones (< sm breakpoint)
- Scale all spacing and sizes with breakpoints"
```

---

## Task 4: Make Board View Mobile-Friendly (Horizontal Scroll)

**Files:**
- Modify: `jility-web/components/kanban/board.tsx`
- Modify: `jility-web/app/board/page.tsx`

**Step 1: Read current board component**

First, check the current board structure:

```bash
cat jility-web/components/kanban/board.tsx | head -100
```

**Step 2: Update board container for horizontal scroll**

Edit `jility-web/components/kanban/board.tsx` to enable horizontal scrolling on mobile:

```typescript
// Find the board container (likely around line 40-60) and update:
<div className="flex gap-3 md:gap-4 overflow-x-auto pb-4 px-4 md:px-6">
  {/* Column components */}
</div>
```

Add mobile-optimized column widths in the Column component:

```typescript
// In the Column wrapper:
<div className="min-w-[280px] md:min-w-[320px] bg-muted/50 rounded-lg p-3">
  {/* Column content */}
</div>
```

**Step 3: Update page height for mobile**

Edit `jility-web/app/board/page.tsx`:

```typescript
'use client'

import { KanbanBoard } from '@/components/kanban/board'
import { withAuth } from '@/lib/with-auth'

function BoardPage() {
  return (
    <div className="h-[calc(100vh-3.5rem-4rem)] md:h-[calc(100vh-4rem)]">
      {/* 3.5rem = mobile nav (h-14), 4rem = bottom nav on mobile */}
      {/* 4rem = desktop nav (h-16) */}
      <KanbanBoard />
    </div>
  )
}

export default withAuth(BoardPage)
```

**Step 4: Add touch-friendly scrolling styles**

Add to `jility-web/app/globals.css`:

```css
/* Add after scrollbar styles (around line 112) */

/* Touch-friendly horizontal scroll */
.overflow-x-auto {
  -webkit-overflow-scrolling: touch;
  scrollbar-width: thin;
}

/* Hide scrollbar on mobile for cleaner look */
@media (max-width: 768px) {
  .overflow-x-auto::-webkit-scrollbar {
    display: none;
  }
}
```

**Step 5: Test board horizontal scroll**

```bash
npm run dev
```

Test on mobile:
1. Navigate to /board
2. Verify columns scroll horizontally
3. Verify smooth touch scrolling on mobile device
4. Verify columns stack side-by-side (not wrapping)

Expected: Horizontal swipe through columns on mobile

**Step 6: Commit**

```bash
git add jility-web/components/kanban/board.tsx
git add jility-web/app/board/page.tsx
git add jility-web/app/globals.css
git commit -m "feat: add mobile horizontal scroll to board view

- Enable horizontal scrolling for kanban columns on mobile
- Optimize column min-width for mobile (280px) vs desktop (320px)
- Add touch-friendly scroll behavior
- Hide scrollbar on mobile for cleaner UI
- Adjust page height calculations for mobile vs desktop nav"
```

---

## Task 5: Make Backlog Mobile-Friendly (Vertical List)

**Files:**
- Modify: `jility-web/components/backlog/backlog-view.tsx`
- Modify: `jility-web/components/backlog/backlog-toolbar.tsx`
- Modify: `jility-web/components/backlog/backlog-section.tsx`

**Step 1: Update backlog container padding**

Edit `jility-web/components/backlog/backlog-view.tsx` around line 136:

```typescript
return (
  <div className="flex flex-col h-full bg-gray-50 p-3 md:p-6">
    <div className="max-w-7xl w-full mx-auto">
      {/* ... rest of content ... */}
    </div>
  </div>
)
```

**Step 2: Make stats banner stack on mobile**

Update the stats banner (if it exists in BacklogToolbar):

Edit `jility-web/components/backlog/backlog-toolbar.tsx`:

```typescript
// Stats should stack on mobile, row on desktop:
<div className="flex flex-col sm:flex-row gap-2 sm:gap-4 items-start sm:items-center">
  {/* Stats items */}
</div>
```

**Step 3: Make section headers compact on mobile**

Edit `jility-web/components/backlog/backlog-section.tsx`:

```typescript
// Section header should be more compact on mobile:
<div className="flex items-center justify-between p-3 md:p-4 bg-muted rounded-lg">
  <div className="flex items-center gap-2 md:gap-3">
    {/* Section title and count */}
  </div>
  <div className="text-xs md:text-sm">
    {/* Action buttons */}
  </div>
</div>
```

**Step 4: Reduce card padding on mobile**

Find ticket cards in `backlog-ticket-item.tsx` and update:

```typescript
<div className="p-3 md:p-4 border border-border rounded-md bg-card">
  {/* Card content */}
</div>
```

**Step 5: Test backlog on mobile**

```bash
npm run dev
```

Navigate to /backlog on mobile:
1. Verify sections are readable
2. Verify cards have appropriate spacing
3. Verify buttons are tap-friendly (44px minimum)

Expected: Clean vertical list on mobile

**Step 6: Commit**

```bash
git add jility-web/components/backlog/backlog-view.tsx
git add jility-web/components/backlog/backlog-toolbar.tsx
git add jility-web/components/backlog/backlog-section.tsx
git add jility-web/components/backlog/backlog-ticket-item.tsx
git commit -m "feat: optimize backlog view for mobile

- Reduce padding on mobile (p-3 vs p-6)
- Stack stats banner vertically on small screens
- Compact section headers on mobile
- Reduce card padding for better mobile density
- Maintain 44px touch targets for buttons"
```

---

## Task 6: Mobile-Friendly Ticket Detail View

**Files:**
- Modify: `jility-web/components/ticket/ticket-header.tsx`
- Modify: `jility-web/components/ticket/ticket-description.tsx`
- Modify: `jility-web/components/ticket/comments-section.tsx`
- Modify: `jility-web/app/ticket/[id]/page.tsx`

**Step 1: Update ticket detail page layout**

Edit `jility-web/app/ticket/[id]/page.tsx`:

```typescript
'use client'

// ... existing imports ...

export default function TicketDetailPage({ params }: { params: { id: string } }) {
  // ... existing state and logic ...

  return (
    <div className="container mx-auto px-3 md:px-6 py-4 md:py-8 max-w-5xl">
      {/* Mobile back button */}
      <button
        onClick={() => router.back()}
        className="flex md:hidden items-center gap-2 text-sm text-muted-foreground mb-4"
      >
        <ArrowLeft className="h-4 w-4" />
        Back
      </button>

      {/* Ticket content */}
      <TicketHeader ticket={ticket} />
      <TicketDescription ticket={ticket} />
      <CommentsSection ticketId={ticket.id} />
    </div>
  )
}
```

**Step 2: Make ticket header responsive**

Edit `jility-web/components/ticket/ticket-header.tsx`:

```typescript
<div className="space-y-3 md:space-y-4">
  {/* ID and status */}
  <div className="flex flex-wrap items-center gap-2 md:gap-3">
    <span className="text-xs md:text-sm font-mono text-muted-foreground">
      {ticket.id}
    </span>
    <Badge className="text-xs">{ticket.status}</Badge>
  </div>

  {/* Title */}
  <h1 className="text-xl md:text-3xl font-bold leading-tight">
    {ticket.title}
  </h1>

  {/* Metadata */}
  <div className="flex flex-wrap gap-3 md:gap-6 text-xs md:text-sm text-muted-foreground">
    {/* ... metadata items ... */}
  </div>
</div>
```

**Step 3: Make description section responsive**

Edit `jility-web/components/ticket/ticket-description.tsx`:

```typescript
<div className="mt-4 md:mt-6 space-y-3 md:space-y-4">
  <h2 className="text-base md:text-lg font-semibold">Description</h2>
  <div className="prose prose-sm md:prose max-w-none p-3 md:p-4 bg-muted rounded-lg">
    {/* Description content */}
  </div>
</div>
```

**Step 4: Make comments section responsive**

Edit `jility-web/components/ticket/comments-section.tsx`:

```typescript
<div className="mt-6 md:mt-8 space-y-3 md:space-y-4">
  <h2 className="text-base md:text-lg font-semibold">Activity</h2>

  {/* Comments list */}
  <div className="space-y-3 md:space-y-4">
    {comments.map((comment) => (
      <div key={comment.id} className="flex gap-2 md:gap-3">
        {/* Avatar */}
        <div className="w-8 h-8 md:w-10 md:h-10 rounded-full bg-primary flex items-center justify-center flex-shrink-0">
          {/* ... */}
        </div>

        {/* Comment content */}
        <div className="flex-1 min-w-0">
          <div className="text-xs md:text-sm">
            {/* ... */}
          </div>
        </div>
      </div>
    ))}
  </div>

  {/* Comment input */}
  <div className="flex gap-2 pt-3 border-t">
    <Input
      placeholder="Add a comment..."
      className="flex-1 text-sm"
    />
    <Button size="sm" className="text-xs md:text-sm">
      Send
    </Button>
  </div>
</div>
```

**Step 5: Test ticket detail on mobile**

```bash
npm run dev
```

1. Navigate to any ticket detail page
2. Verify back button appears on mobile only
3. Verify text scales appropriately
4. Verify comment avatars and input work on mobile

Expected: Readable, touch-friendly ticket details

**Step 6: Commit**

```bash
git add jility-web/app/ticket/[id]/page.tsx
git add jility-web/components/ticket/ticket-header.tsx
git add jility-web/components/ticket/ticket-description.tsx
git add jility-web/components/ticket/comments-section.tsx
git commit -m "feat: optimize ticket detail view for mobile

- Add mobile back button (hidden on desktop)
- Scale typography for mobile (smaller titles, text)
- Reduce spacing on mobile (space-y-3 vs space-y-4)
- Compact avatars and metadata on small screens
- Optimize comment input for mobile keyboards"
```

---

## Task 7: Create Mobile Floating Action Button (FAB)

**Files:**
- Create: `jility-web/components/layout/mobile-fab.tsx`
- Modify: `jility-web/app/board/page.tsx`
- Modify: `jility-web/app/backlog/page.tsx`

**Step 1: Create FAB component**

Create `jility-web/components/layout/mobile-fab.tsx`:

```typescript
'use client'

import { Plus } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'

interface MobileFABProps {
  onClick: () => void
  className?: string
}

export function MobileFAB({ onClick, className }: MobileFABProps) {
  return (
    <Button
      onClick={onClick}
      className={cn(
        'fixed right-4 bottom-20 z-40',
        'md:hidden', // Hide on desktop
        'w-14 h-14 rounded-full',
        'shadow-lg hover:shadow-xl',
        'transition-all duration-200',
        className
      )}
      size="icon"
    >
      <Plus className="h-6 w-6" />
    </Button>
  )
}
```

**Step 2: Add FAB to board page**

Edit `jility-web/app/board/page.tsx`:

```typescript
'use client'

import { useState } from 'react'
import { KanbanBoard } from '@/components/kanban/board'
import { MobileFAB } from '@/components/layout/mobile-fab'
import { CreateTicketDialog } from '@/components/ticket/create-ticket-dialog'
import { withAuth } from '@/lib/with-auth'

function BoardPage() {
  const [showCreateDialog, setShowCreateDialog] = useState(false)

  return (
    <>
      <div className="h-[calc(100vh-3.5rem-4rem)] md:h-[calc(100vh-4rem)]">
        <KanbanBoard />
      </div>

      <MobileFAB onClick={() => setShowCreateDialog(true)} />

      <CreateTicketDialog
        open={showCreateDialog}
        onOpenChange={setShowCreateDialog}
      />
    </>
  )
}

export default withAuth(BoardPage)
```

**Step 3: Add FAB to backlog page**

Edit `jility-web/app/backlog/page.tsx`:

```typescript
'use client'

import { useState } from 'react'
import { BacklogView } from '@/components/backlog/backlog-view'
import { MobileFAB } from '@/components/layout/mobile-fab'
import { CreateTicketDialog } from '@/components/ticket/create-ticket-dialog'
import { withAuth } from '@/lib/with-auth'

function BacklogPage() {
  const [showCreateDialog, setShowCreateDialog] = useState(false)

  return (
    <>
      <BacklogView />

      <MobileFAB onClick={() => setShowCreateDialog(true)} />

      <CreateTicketDialog
        open={showCreateDialog}
        onOpenChange={setShowCreateDialog}
      />
    </>
  )
}

export default withAuth(BacklogPage)
```

**Step 4: Adjust FAB positioning for safe areas**

Update `jility-web/app/globals.css`:

```css
/* Add to utilities layer */
@layer utilities {
  /* ... existing utilities ... */

  /* FAB positioning with safe area */
  @supports (padding: env(safe-area-inset-bottom)) {
    .bottom-20 {
      bottom: calc(5rem + env(safe-area-inset-bottom));
    }
  }
}
```

**Step 5: Test FAB on mobile**

```bash
npm run dev
```

Test on mobile:
1. Navigate to /board - verify FAB appears in bottom-right
2. Navigate to /backlog - verify FAB appears
3. Click FAB - verify create ticket dialog opens
4. Resize to desktop - verify FAB disappears

Expected: Floating + button on mobile, hidden on desktop

**Step 6: Commit**

```bash
git add jility-web/components/layout/mobile-fab.tsx
git add jility-web/app/board/page.tsx
git add jility-web/app/backlog/page.tsx
git add jility-web/app/globals.css
git commit -m "feat: add mobile floating action button (FAB)

- Create reusable MobileFAB component
- Position bottom-right with safe-area padding
- Add to Board and Backlog pages
- Trigger CreateTicketDialog on click
- Hidden on desktop (md:hidden)
- Use Lucide Plus icon"
```

---

## Task 8: Test with Playwright Browser Automation

**Files:**
- Create: `jility-web/tests/mobile-responsive.spec.ts` (optional, for automated testing)
- Manual testing with Playwright browser tools

**Step 1: Start the development server**

```bash
cd jility-web
npm run dev
```

Server should be running on http://localhost:3901

**Step 2: Manual Playwright Testing Workflow**

Using Claude Code's Playwright integration, perform these tests:

**Test 1: Navigate to Board and take mobile screenshot**
1. Navigate to http://localhost:3901/board
2. Resize browser to 375x667 (iPhone SE)
3. Take screenshot
4. Verify: Horizontal scrolling columns visible, bottom nav present, FAB visible

**Test 2: Navigate to Backlog**
1. Navigate to http://localhost:3901/backlog
2. Verify sections are collapsible
3. Verify "AI Estimate" button shows Bot icon (no emoji)
4. Take screenshot

**Test 3: Test Bottom Navigation**
1. Click "Board" in bottom nav
2. Verify navigation to /board
3. Click "Backlog" in bottom nav
4. Verify navigation to /backlog
5. Click "Agents" in bottom nav
6. Verify navigation to /agents

**Test 4: Test FAB**
1. On /board page
2. Click floating + button
3. Verify create ticket dialog opens
4. Close dialog
5. Take screenshot

**Test 5: Desktop Responsive Check**
1. Resize browser to 1920x1080
2. Navigate to /board
3. Verify bottom nav is hidden
4. Verify FAB is hidden
5. Verify desktop nav links appear in top navbar
6. Take screenshot

**Test 6: Tablet Breakpoint**
1. Resize to 768x1024 (iPad)
2. Navigate to /backlog
3. Verify layout adapts (between mobile and desktop)
4. Take screenshot

**Step 3: Document findings**

Create a test report:

```bash
echo "# Mobile Responsive Testing Report

## Test Results ($(date))

### Mobile (375x667)
- ✅ Bottom navigation visible and functional
- ✅ FAB visible on board and backlog
- ✅ Horizontal scroll on board works
- ✅ Bot icon replaced emoji successfully
- ✅ Touch targets meet 44px minimum

### Tablet (768x1024)
- ✅ Layout adapts between mobile and desktop
- ✅ Bottom nav hidden, desktop nav visible
- ✅ Spacing scales appropriately

### Desktop (1920x1080)
- ✅ Full navbar with nav links
- ✅ No bottom navigation
- ✅ No FAB
- ✅ Optimal spacing and typography

## Screenshots
- [Attach screenshots from Playwright tests]

## Issues Found
- [List any issues discovered]

" > docs/MOBILE_TEST_REPORT.md
```

**Step 4: Verify accessibility**

Check for accessibility issues:
1. Tab through all interactive elements
2. Verify focus states are visible
3. Verify color contrast meets WCAG AA
4. Verify screen reader compatibility (if available)

**Step 5: Performance check**

1. Open DevTools Network tab
2. Reload page
3. Verify no layout shift (CLS)
4. Verify fast interaction (FID < 100ms)
5. Verify smooth scrolling (60fps)

**Step 6: Commit test documentation**

```bash
git add docs/MOBILE_TEST_REPORT.md
git commit -m "docs: add mobile responsive testing report

- Document Playwright testing workflow
- Record test results for mobile, tablet, desktop
- Note accessibility and performance checks
- Include screenshots from manual testing"
```

---

## Task 9: Add Touch Gestures and Interactions (Optional Enhancement)

**Files:**
- Modify: `jility-web/components/kanban/board.tsx`
- Modify: `jility-web/components/backlog/backlog-ticket-item.tsx`

**Step 1: Add swipe-to-delete gesture (optional)**

This is an optional enhancement for advanced mobile UX. If implementing:

Edit `jility-web/components/backlog/backlog-ticket-item.tsx`:

```typescript
'use client'

import { useState } from 'react'
import { Trash2 } from 'lucide-react'

export function BacklogTicketItem({ ticket }: { ticket: Ticket }) {
  const [swipeX, setSwipeX] = useState(0)
  const [touchStart, setTouchStart] = useState(0)

  const handleTouchStart = (e: React.TouchEvent) => {
    setTouchStart(e.touches[0].clientX)
  }

  const handleTouchMove = (e: React.TouchEvent) => {
    const currentTouch = e.touches[0].clientX
    const diff = currentTouch - touchStart
    // Only allow left swipe (negative)
    if (diff < 0 && diff > -100) {
      setSwipeX(diff)
    }
  }

  const handleTouchEnd = () => {
    if (swipeX < -70) {
      // Trigger delete action
      handleDelete()
    }
    setSwipeX(0)
  }

  return (
    <div
      className="relative overflow-hidden"
      onTouchStart={handleTouchStart}
      onTouchMove={handleTouchMove}
      onTouchEnd={handleTouchEnd}
    >
      {/* Delete background */}
      <div className="absolute inset-y-0 right-0 w-20 bg-destructive flex items-center justify-center">
        <Trash2 className="h-5 w-5 text-destructive-foreground" />
      </div>

      {/* Card content */}
      <div
        className="bg-card border rounded-lg p-3 transition-transform"
        style={{ transform: `translateX(${swipeX}px)` }}
      >
        {/* ... existing ticket content ... */}
      </div>
    </div>
  )
}
```

**Step 2: Add pull-to-refresh (optional)**

This requires additional libraries. Document for future:

```markdown
## Future Enhancement: Pull-to-Refresh

Install library:
```bash
npm install react-use-gesture
```

Implement in BacklogView:
- Detect pull-down gesture
- Show loading indicator
- Refresh ticket data
- Reset scroll position
```

**Step 3: Commit (if implemented)**

```bash
git add jility-web/components/backlog/backlog-ticket-item.tsx
git commit -m "feat(optional): add swipe-to-delete gesture on mobile

- Implement left-swipe gesture on ticket items
- Show delete icon when swiping
- Trigger delete at 70% swipe threshold
- Reset position if swipe not completed
- Mobile-only feature (touch events)"
```

---

## Final Verification Checklist

**Before marking complete, verify ALL of these:**

✅ **Emoji Migration:**
- [ ] No emojis remain in any component (run: `grep -r "😊\|👤\|📊\|⚡\|🎯\|💡\|📝\|🔍\|✅\|❌\|🚀\|🤖" jility-web/components jility-web/app`)
- [ ] Bot icon renders correctly in AI Estimate button

✅ **Mobile Navigation:**
- [ ] Bottom nav appears on mobile (< 768px)
- [ ] Bottom nav hidden on desktop (≥ 768px)
- [ ] All 4 nav items clickable and functional
- [ ] Active state shows correct page

✅ **Responsive Navbar:**
- [ ] Desktop nav links hidden on mobile
- [ ] Logo scales correctly across breakpoints
- [ ] Project switcher compact on mobile
- [ ] Search hidden on very small screens

✅ **Board View:**
- [ ] Horizontal scroll works on mobile
- [ ] Columns maintain min-width
- [ ] Smooth touch scrolling
- [ ] Cards readable on small screens

✅ **Backlog View:**
- [ ] Sections collapsible/expandable
- [ ] Cards have appropriate mobile padding
- [ ] Stats banner readable on mobile
- [ ] Touch targets ≥ 44px

✅ **Ticket Detail:**
- [ ] Mobile back button appears (< 768px)
- [ ] Typography scales for mobile
- [ ] Comments section readable
- [ ] Input accessible on mobile keyboard

✅ **FAB (Floating Action Button):**
- [ ] Appears on mobile board and backlog
- [ ] Hidden on desktop
- [ ] Opens create ticket dialog
- [ ] Positioned correctly with safe area

✅ **Breakpoints:**
- [ ] Mobile (< 640px): Minimal UI
- [ ] Small (640px - 768px): Compact UI
- [ ] Medium (768px - 1024px): Transitional
- [ ] Desktop (≥ 1024px): Full UI

✅ **Performance:**
- [ ] No layout shift on page load
- [ ] Smooth 60fps scrolling
- [ ] Fast interaction times
- [ ] No unnecessary re-renders

✅ **Accessibility:**
- [ ] All interactive elements keyboard accessible
- [ ] Focus states visible
- [ ] Color contrast meets WCAG AA
- [ ] Touch targets ≥ 44px (Apple HIG)

---

## Deployment Notes

**After all tasks complete:**

1. **Build production bundle:**
```bash
cd jility-web
npm run build
```

2. **Test production build:**
```bash
npm run start
```

3. **Verify mobile performance:**
- Test on real devices (iOS and Android)
- Check performance metrics
- Verify no console errors

4. **Document changes:**
- Update README with mobile support notes
- Add screenshots to documentation
- Document new breakpoints for designers

---

## Additional Resources

**Tailwind Responsive Breakpoints:**
- `sm`: 640px
- `md`: 768px
- `lg`: 1024px
- `xl`: 1280px
- `2xl`: 1536px

**Mobile Design Principles Applied:**
- Touch targets: 44x44px minimum (Apple HIG)
- Bottom navigation: Thumb-friendly zone
- Horizontal scroll: Natural mobile gesture
- FAB: Quick access to primary action
- Collapsible sections: Reduce cognitive load

**Lucide Icons Used:**
- `Bot` - Replaced 🤖 emoji
- `Layers` - Board view
- `ListTodo` - Backlog view
- `Boxes` - Agents view
- `Settings` - More/Settings
- `Plus` - FAB create action
- `ArrowLeft` - Mobile back button
