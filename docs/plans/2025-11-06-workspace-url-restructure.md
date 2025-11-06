# Workspace URL Restructure Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Restructure Next.js routes to use workspace-based URLs (`/w/[slug]/...`) and add workspace switcher UI.

**Architecture:** Move all app routes into `/app/w/[slug]/` dynamic route segment. Workspace context extracts slug from URL pathname. Navigation components use workspace slug for routing. Add workspace switcher dropdown in navbar.

**Tech Stack:** Next.js 14 App Router, React Context, TypeScript, Radix UI dropdown

---

## Task 1: Create workspace dynamic route structure

**Files:**
- Create: `jility-web/app/w/[slug]/layout.tsx`
- Reference: `jility-web/lib/workspace-context.tsx` (already exists)

**Step 1: Create workspace layout with slug parameter**

Create `jility-web/app/w/[slug]/layout.tsx`:

```typescript
'use client'

import { ReactNode } from 'react'
import { useParams } from 'next/navigation'

interface WorkspaceLayoutProps {
  children: ReactNode
}

export default function WorkspaceLayout({ children }: WorkspaceLayoutProps) {
  const params = useParams()
  const slug = params.slug as string

  // The WorkspaceProvider in root layout will handle loading workspace data
  // This layout just ensures we're in a workspace context
  return <>{children}</>
}
```

**Step 2: Verify the file compiles**

Run: `cd jility-web && npm run build`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add jility-web/app/w/\[slug\]/layout.tsx
git commit -m "feat: add workspace dynamic route layout"
```

---

## Task 2: Move board page to workspace route

**Files:**
- Move: `jility-web/app/board/page.tsx` → `jility-web/app/w/[slug]/board/page.tsx`
- Verify: No changes to file content needed

**Step 1: Create directory and move file**

```bash
mkdir -p jility-web/app/w/\[slug\]/board
mv jility-web/app/board/page.tsx jility-web/app/w/\[slug\]/board/page.tsx
rmdir jility-web/app/board
```

**Step 2: Verify the file still compiles**

Run: `cd jility-web && npm run build`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add jility-web/app/w/\[slug\]/board/page.tsx
git add jility-web/app/board/
git commit -m "feat: move board page to workspace route"
```

---

## Task 3: Move backlog page to workspace route

**Files:**
- Move: `jility-web/app/backlog/page.tsx` → `jility-web/app/w/[slug]/backlog/page.tsx`

**Step 1: Create directory and move file**

```bash
mkdir -p jility-web/app/w/\[slug\]/backlog
mv jility-web/app/backlog/page.tsx jility-web/app/w/\[slug\]/backlog/page.tsx
rmdir jility-web/app/backlog
```

**Step 2: Verify the file still compiles**

Run: `cd jility-web && npm run build`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add jility-web/app/w/\[slug\]/backlog/page.tsx
git add jility-web/app/backlog/
git commit -m "feat: move backlog page to workspace route"
```

---

## Task 4: Move sprint pages to workspace route

**Files:**
- Move: `jility-web/app/sprint/` → `jility-web/app/w/[slug]/sprint/`
- Contains: `active/page.tsx`, `history/page.tsx`, `planning/page.tsx`

**Step 1: Create directory and move files**

```bash
mkdir -p jility-web/app/w/\[slug\]/sprint
mv jility-web/app/sprint/active jility-web/app/w/\[slug\]/sprint/active
mv jility-web/app/sprint/history jility-web/app/w/\[slug\]/sprint/history
mv jility-web/app/sprint/planning jility-web/app/w/\[slug\]/sprint/planning
rmdir jility-web/app/sprint
```

**Step 2: Verify the files still compile**

Run: `cd jility-web && npm run build`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add jility-web/app/w/\[slug\]/sprint/
git add jility-web/app/sprint/
git commit -m "feat: move sprint pages to workspace route"
```

---

## Task 5: Move ticket, search, agents, and profile pages

**Files:**
- Move: `jility-web/app/ticket/[id]/page.tsx` → `jility-web/app/w/[slug]/ticket/[id]/page.tsx`
- Move: `jility-web/app/search/page.tsx` → `jility-web/app/w/[slug]/search/page.tsx`
- Move: `jility-web/app/agents/page.tsx` → `jility-web/app/w/[slug]/agents/page.tsx`
- Move: `jility-web/app/profile/page.tsx` → `jility-web/app/w/[slug]/profile/page.tsx`

**Step 1: Move ticket page (dynamic route)**

```bash
mkdir -p jility-web/app/w/\[slug\]/ticket/\[id\]
mv jility-web/app/ticket/\[id\]/page.tsx jility-web/app/w/\[slug\]/ticket/\[id\]/page.tsx
rm -rf jility-web/app/ticket
```

**Step 2: Move search page**

```bash
mkdir -p jility-web/app/w/\[slug\]/search
mv jility-web/app/search/page.tsx jility-web/app/w/\[slug\]/search/page.tsx
rmdir jility-web/app/search
```

**Step 3: Move agents page**

```bash
mkdir -p jility-web/app/w/\[slug\]/agents
mv jility-web/app/agents/page.tsx jility-web/app/w/\[slug\]/agents/page.tsx
rmdir jility-web/app/agents
```

**Step 4: Move profile page**

```bash
mkdir -p jility-web/app/w/\[slug\]/profile
mv jility-web/app/profile/page.tsx jility-web/app/w/\[slug\]/profile/page.tsx
rmdir jility-web/app/profile
```

**Step 5: Verify everything compiles**

Run: `cd jility-web && npm run build`
Expected: Build succeeds

**Step 6: Commit**

```bash
git add jility-web/app/w/\[slug\]/
git add jility-web/app/ticket/ jility-web/app/search/ jility-web/app/agents/ jility-web/app/profile/
git commit -m "feat: move ticket, search, agents, and profile pages to workspace route"
```

---

## Task 6: Create workspace switcher component

**Files:**
- Create: `jility-web/components/workspace-switcher.tsx`
- Reference: `jility-web/lib/workspace-context.tsx`
- Reference: `jility-web/components/ui/dropdown-menu.tsx`

**Step 1: Create workspace switcher component**

Create `jility-web/components/workspace-switcher.tsx`:

```typescript
'use client'

import { useWorkspace } from '@/lib/workspace-context'
import { Check, ChevronsUpDown, Plus } from 'lucide-react'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Button } from '@/components/ui/button'

export function WorkspaceSwitcher() {
  const { currentWorkspace, workspaces, switchWorkspace } = useWorkspace()

  if (!currentWorkspace) {
    return null
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button
          variant="outline"
          className="w-[200px] justify-between"
        >
          <span className="truncate">{currentWorkspace.name}</span>
          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-[200px]" align="start">
        <DropdownMenuLabel>Workspaces</DropdownMenuLabel>
        <DropdownMenuSeparator />
        {workspaces.map((workspace) => (
          <DropdownMenuItem
            key={workspace.id}
            onClick={() => switchWorkspace(workspace.slug)}
            className="cursor-pointer"
          >
            <Check
              className={`mr-2 h-4 w-4 ${
                workspace.id === currentWorkspace.id
                  ? 'opacity-100'
                  : 'opacity-0'
              }`}
            />
            <span className="truncate">{workspace.name}</span>
          </DropdownMenuItem>
        ))}
        <DropdownMenuSeparator />
        <DropdownMenuItem className="cursor-pointer">
          <Plus className="mr-2 h-4 w-4" />
          Create workspace
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
```

**Step 2: Verify the file compiles**

Run: `cd jility-web && npm run build`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add jility-web/components/workspace-switcher.tsx
git commit -m "feat: add workspace switcher component"
```

---

## Task 7: Update desktop navbar to include workspace switcher

**Files:**
- Modify: `jility-web/components/layout/desktop-navbar.tsx`
- Add import and render WorkspaceSwitcher

**Step 1: Read current navbar to understand structure**

Read: `jility-web/components/layout/desktop-navbar.tsx`

**Step 2: Add workspace switcher to navbar**

In `jility-web/components/layout/desktop-navbar.tsx`, add import at top:

```typescript
import { WorkspaceSwitcher } from '@/components/workspace-switcher'
```

Then add the WorkspaceSwitcher component in the navbar, typically near the top or left side of the header. Find the appropriate location (likely after the logo or brand name) and add:

```typescript
<WorkspaceSwitcher />
```

**Step 3: Verify it renders**

Run: `cd jility-web && npm run build`
Expected: Build succeeds

**Step 4: Commit**

```bash
git add jility-web/components/layout/desktop-navbar.tsx
git commit -m "feat: add workspace switcher to desktop navbar"
```

---

## Task 8: Update mobile navbar to include workspace switcher

**Files:**
- Modify: `jility-web/components/layout/mobile-navbar.tsx`
- Add WorkspaceSwitcher

**Step 1: Read current mobile navbar**

Read: `jility-web/components/layout/mobile-navbar.tsx`

**Step 2: Add workspace switcher to mobile navbar**

In `jility-web/components/layout/mobile-navbar.tsx`, add import:

```typescript
import { WorkspaceSwitcher } from '@/components/workspace-switcher'
```

Add the component in an appropriate location (typically in a menu or at the top).

**Step 3: Verify it compiles**

Run: `cd jility-web && npm run build`
Expected: Build succeeds

**Step 4: Commit**

```bash
git add jility-web/components/layout/mobile-navbar.tsx
git commit -m "feat: add workspace switcher to mobile navbar"
```

---

## Task 9: Update navigation links to use workspace slug

**Files:**
- Modify: `jility-web/components/layout/desktop-navbar.tsx`
- Modify: `jility-web/components/layout/mobile-navbar.tsx`
- Update all hardcoded `/board`, `/backlog`, etc. to `/w/${slug}/board`

**Step 1: Update desktop navbar links**

In `jility-web/components/layout/desktop-navbar.tsx`:

Add at top of component:
```typescript
const { currentWorkspace } = useWorkspace()
const slug = currentWorkspace?.slug || ''
```

Update all navigation href attributes from:
- `/board` → `` `/w/${slug}/board` ``
- `/backlog` → `` `/w/${slug}/backlog` ``
- `/sprint/active` → `` `/w/${slug}/sprint/active` ``
- `/sprint/planning` → `` `/w/${slug}/sprint/planning` ``
- `/sprint/history` → `` `/w/${slug}/sprint/history` ``
- `/search` → `` `/w/${slug}/search` ``
- `/agents` → `` `/w/${slug}/agents` ``
- `/profile` → `` `/w/${slug}/profile` ``

**Step 2: Update mobile navbar links**

Apply the same changes to `jility-web/components/layout/mobile-navbar.tsx`.

**Step 3: Verify compilation**

Run: `cd jility-web && npm run build`
Expected: Build succeeds

**Step 4: Commit**

```bash
git add jility-web/components/layout/desktop-navbar.tsx jility-web/components/layout/mobile-navbar.tsx
git commit -m "feat: update navigation links to use workspace slug"
```

---

## Task 10: Update workspace context switchWorkspace function

**Files:**
- Modify: `jility-web/lib/workspace-context.tsx:64-70`

**Step 1: Fix switchWorkspace to use new URL pattern**

In `jility-web/lib/workspace-context.tsx`, update the `switchWorkspace` function:

```typescript
const switchWorkspace = (slug: string) => {
  const workspace = workspaces.find(w => w.slug === slug)
  if (workspace) {
    setCurrentWorkspace(workspace)
    // Redirect to the board page of the new workspace
    router.push(`/w/${slug}/board`)
  }
}
```

**Step 2: Verify compilation**

Run: `cd jility-web && npm run build`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add jility-web/lib/workspace-context.tsx
git commit -m "fix: update switchWorkspace to use new URL pattern"
```

---

## Task 11: Update login redirect to workspace URL

**Files:**
- Modify: `jility-web/app/login/page.tsx`
- Redirect to `/w/${workspace-slug}/board` after login

**Step 1: Read current login page**

Read: `jility-web/app/login/page.tsx`

**Step 2: Update login redirect**

Find the successful login redirect logic. After successful login and receiving user data, fetch the user's workspaces and redirect to their first workspace:

```typescript
// After successful login
const workspacesResponse = await fetch('/api/workspaces', {
  credentials: 'include',
})
const workspaces = await workspacesResponse.json()

if (workspaces.length > 0) {
  router.push(`/w/${workspaces[0].slug}/board`)
} else {
  // No workspaces - this shouldn't happen if signup creates default workspace
  router.push('/create-workspace')
}
```

**Step 3: Verify compilation**

Run: `cd jility-web && npm run build`
Expected: Build succeeds

**Step 4: Commit**

```bash
git add jility-web/app/login/page.tsx
git commit -m "feat: redirect to workspace URL after login"
```

---

## Task 12: Verify signup creates default workspace and redirects

**Files:**
- Verify: Backend creates workspace on signup (already implemented)
- Modify: `jility-web/app/register/page.tsx`

**Step 1: Read register page**

Read: `jility-web/app/register/page.tsx`

**Step 2: Update register redirect**

Find the successful registration redirect. After signup, fetch workspaces and redirect:

```typescript
// After successful registration
const workspacesResponse = await fetch('/api/workspaces', {
  credentials: 'include',
})
const workspaces = await workspacesResponse.json()

if (workspaces.length > 0) {
  router.push(`/w/${workspaces[0].slug}/board`)
} else {
  // Fallback - shouldn't happen
  router.push('/login')
}
```

**Step 3: Verify compilation**

Run: `cd jility-web && npm run build`
Expected: Build succeeds

**Step 4: Commit**

```bash
git add jility-web/app/register/page.tsx
git commit -m "feat: redirect to workspace URL after registration"
```

---

## Task 13: Update root page to redirect to workspace

**Files:**
- Modify: `jility-web/app/page.tsx`
- Add redirect logic to workspace

**Step 1: Update root page to redirect authenticated users**

Replace content of `jility-web/app/page.tsx`:

```typescript
'use client'

import { useEffect } from 'react'
import { useRouter } from 'next/navigation'
import { Loader2 } from 'lucide-react'

export default function HomePage() {
  const router = useRouter()

  useEffect(() => {
    async function redirectToWorkspace() {
      try {
        // Check if user is logged in and has workspaces
        const response = await fetch('/api/workspaces', {
          credentials: 'include',
        })

        if (response.ok) {
          const workspaces = await response.json()
          if (workspaces.length > 0) {
            // Redirect to first workspace
            router.push(`/w/${workspaces[0].slug}/board`)
            return
          }
        }

        // Not logged in or no workspaces, go to login
        router.push('/login')
      } catch (error) {
        // Error fetching, assume not logged in
        router.push('/login')
      }
    }

    redirectToWorkspace()
  }, [router])

  return (
    <div className="flex h-screen items-center justify-center">
      <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
    </div>
  )
}
```

**Step 2: Verify compilation**

Run: `cd jility-web && npm run build`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add jility-web/app/page.tsx
git commit -m "feat: redirect root page to workspace or login"
```

---

## Task 14: Add create workspace dialog

**Files:**
- Create: `jility-web/components/workspaces/create-workspace-dialog.tsx`

**Step 1: Create workspace creation dialog**

Create `jility-web/components/workspaces/create-workspace-dialog.tsx`:

```typescript
'use client'

import { useState } from 'react'
import { useRouter } from 'next/navigation'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Loader2 } from 'lucide-react'

interface CreateWorkspaceDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onSuccess?: () => void
}

export function CreateWorkspaceDialog({
  open,
  onOpenChange,
  onSuccess,
}: CreateWorkspaceDialogProps) {
  const [name, setName] = useState('')
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const router = useRouter()

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)

    if (!name.trim()) {
      setError('Workspace name is required')
      return
    }

    setIsSubmitting(true)

    try {
      const response = await fetch('/api/workspaces', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify({ name: name.trim() }),
      })

      if (!response.ok) {
        throw new Error('Failed to create workspace')
      }

      const workspace = await response.json()
      onOpenChange(false)

      if (onSuccess) {
        onSuccess()
      }

      // Redirect to new workspace
      router.push(`/w/${workspace.slug}/board`)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create workspace')
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>Create Workspace</DialogTitle>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="name">Workspace Name *</Label>
            <Input
              id="name"
              placeholder="e.g., My Team"
              value={name}
              onChange={e => setName(e.target.value)}
              required
            />
          </div>

          {error && (
            <div className="text-sm text-destructive bg-destructive/10 px-3 py-2 rounded-md">
              {error}
            </div>
          )}

          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
              disabled={isSubmitting}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={isSubmitting}>
              {isSubmitting && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
              Create Workspace
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
```

**Step 2: Verify compilation**

Run: `cd jility-web && npm run build`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add jility-web/components/workspaces/create-workspace-dialog.tsx
git commit -m "feat: add create workspace dialog component"
```

---

## Task 15: Connect create workspace dialog to workspace switcher

**Files:**
- Modify: `jility-web/components/workspace-switcher.tsx`

**Step 1: Add dialog state and import**

In `jility-web/components/workspace-switcher.tsx`, add at top:

```typescript
import { useState } from 'react'
import { CreateWorkspaceDialog } from '@/components/workspaces/create-workspace-dialog'
```

Add state inside component:
```typescript
const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false)
```

**Step 2: Update "Create workspace" menu item**

Change the "Create workspace" DropdownMenuItem to:

```typescript
<DropdownMenuItem
  className="cursor-pointer"
  onClick={() => setIsCreateDialogOpen(true)}
>
  <Plus className="mr-2 h-4 w-4" />
  Create workspace
</DropdownMenuItem>
```

**Step 3: Add dialog component at end of return**

After the DropdownMenuContent closing tag, add:

```typescript
<CreateWorkspaceDialog
  open={isCreateDialogOpen}
  onOpenChange={setIsCreateDialogOpen}
  onSuccess={() => refreshWorkspaces()}
/>
```

Import refreshWorkspaces from useWorkspace:
```typescript
const { currentWorkspace, workspaces, switchWorkspace, refreshWorkspaces } = useWorkspace()
```

**Step 4: Verify compilation**

Run: `cd jility-web && npm run build`
Expected: Build succeeds

**Step 5: Commit**

```bash
git add jility-web/components/workspace-switcher.tsx
git commit -m "feat: wire up create workspace dialog to switcher"
```

---

## Task 16: Build and test in Docker

**Files:**
- Test: All changes work together

**Step 1: Build Docker images**

```bash
task build
```

Expected: Both frontend and backend build successfully

**Step 2: Restart Docker containers**

```bash
docker-compose down && docker-compose up -d
```

**Step 3: Test the flow**

Manual testing checklist:
1. Navigate to http://localhost:3901
2. Should redirect to login
3. Login with existing user
4. Should redirect to `/w/zacharyswift2s-workspace/board`
5. Workspace switcher should appear in navbar
6. Click workspace switcher - should show current workspace with checkmark
7. Try navigating to different pages - URLs should include `/w/[slug]/`
8. Click "Create workspace" in switcher
9. Create a new workspace
10. Should redirect to new workspace
11. Switch back to original workspace using switcher
12. Try creating a project - should now work (workspace_id is available)

**Step 4: Document any issues found**

If any issues are found, create follow-up tasks to fix them.

**Step 5: Commit if any fixes were made**

```bash
git add .
git commit -m "fix: [describe any fixes made during testing]"
```

---

## Task 17: Push changes to GitHub

**Files:**
- All modified and new files

**Step 1: Review all changes**

```bash
git log --oneline origin/main..HEAD
git diff origin/main..HEAD --stat
```

**Step 2: Push to GitHub**

```bash
git push origin main
```

Expected: All commits pushed successfully

---

## Success Criteria

- ✅ All routes accessible at `/w/[slug]/...` pattern
- ✅ Workspace switcher visible in navbar (desktop and mobile)
- ✅ Login redirects to workspace URL
- ✅ Registration redirects to workspace URL
- ✅ Root page redirects appropriately
- ✅ Can create new workspaces via switcher
- ✅ Can switch between workspaces
- ✅ Project creation works (workspace_id available from context)
- ✅ All navigation links use workspace slug
- ✅ Docker build succeeds
- ✅ Changes pushed to GitHub

## Notes

- All pages remain functionally the same, just moved to workspace-scoped URLs
- The workspace context already extracts the slug from the URL pathname, so it should work automatically once routes are moved
- The backend already has all workspace APIs implemented
- Creating a workspace from the UI will hit the existing POST `/api/workspaces` endpoint
