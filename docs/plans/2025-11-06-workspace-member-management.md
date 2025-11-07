# Workspace Member Management UI

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add UI for workspace owners/admins to invite, list, and remove team members

**Architecture:** Backend already has workspace_member and workspace_invite tables from migration. Need to add API endpoints for listing members and removing members, then create frontend settings page with member list and invite form.

**Tech Stack:**
- Backend: Rust/Axum, SeaORM, SQLite
- Frontend: Next.js 14, React, TypeScript, shadcn/ui
- Existing: `invite_member` endpoint already implemented

---

## Task 1: Add Backend Endpoint - List Workspace Members

**Files:**
- Modify: `jility-server/src/api/workspaces.rs:~140` (add after invite_member)
- Modify: `jility-server/src/api/mod.rs:40` (add route)

**Step 1: Add list members endpoint to workspaces.rs**

Add after the `invite_member` function:

```rust
#[derive(Serialize)]
pub struct WorkspaceMemberResponse {
    pub user_id: String,
    pub email: String,
    pub role: String,
    pub joined_at: String,
}

/// List workspace members
pub async fn list_members(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(workspace_slug): Path<String>,
) -> ApiResult<Json<Vec<WorkspaceMemberResponse>>> {
    let workspace_service = WorkspaceService::new(state.db.as_ref().clone());
    let member_service = MemberService::new(state.db.as_ref().clone());

    // Get workspace
    let workspace = workspace_service
        .get_workspace_by_slug(&workspace_slug)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch workspace: {}", e)))?
        .ok_or_else(|| ApiError::NotFound("Workspace not found".to_string()))?;

    // Check if user is a member
    let is_member = workspace_service
        .is_member(workspace.id, auth_user.id)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to check membership: {}", e)))?;

    if !is_member {
        return Err(ApiError::Unauthorized(
            "You are not a member of this workspace".to_string(),
        ));
    }

    // Get all members
    let members = member_service
        .list_workspace_members(workspace.id)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch members: {}", e)))?;

    Ok(Json(members))
}
```

**Step 2: Add route to mod.rs**

In `jility-server/src/api/mod.rs`, add after line 40:

```rust
.route("/api/workspaces/:slug/members", get(workspaces::list_members))
```

**Step 3: Add list_workspace_members to MemberService**

Modify: `jility-server/src/services/member.rs`

Add method to MemberService:

```rust
pub async fn list_workspace_members(
    &self,
    workspace_id: Uuid,
) -> Result<Vec<WorkspaceMemberResponse>, anyhow::Error> {
    use jility_core::entities::{user, workspace_member};
    use sea_orm::{EntityTrait, JoinType, QuerySelect, RelationTrait};

    let members = workspace_member::Entity::find()
        .filter(workspace_member::Column::WorkspaceId.eq(workspace_id))
        .find_also_related(user::Entity)
        .all(&self.db)
        .await?;

    let responses = members
        .into_iter()
        .filter_map(|(member, user_opt)| {
            user_opt.map(|user| WorkspaceMemberResponse {
                user_id: member.user_id.to_string(),
                email: user.email,
                role: match member.role {
                    jility_core::entities::WorkspaceRole::Admin => "admin".to_string(),
                    jility_core::entities::WorkspaceRole::Member => "member".to_string(),
                },
                joined_at: member.joined_at.to_rfc3339(),
            })
        })
        .collect();

    Ok(responses)
}
```

**Step 4: Build and test**

```bash
cd jility-server
cargo build
cargo test
```

Expected: Build succeeds, tests pass

**Step 5: Commit**

```bash
git add jility-server/src/api/workspaces.rs jility-server/src/api/mod.rs jility-server/src/services/member.rs
git commit -m "feat: add list workspace members endpoint"
```

---

## Task 2: Add Backend Endpoint - Remove Workspace Member

**Files:**
- Modify: `jility-server/src/api/workspaces.rs:~180` (add after list_members)
- Modify: `jility-server/src/api/mod.rs:41` (add route)

**Step 1: Add remove member endpoint**

Add to `jility-server/src/api/workspaces.rs`:

```rust
/// Remove workspace member
pub async fn remove_member(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path((workspace_slug, user_id)): Path<(String, String)>,
) -> ApiResult<StatusCode> {
    let workspace_service = WorkspaceService::new(state.db.as_ref().clone());
    let member_service = MemberService::new(state.db.as_ref().clone());

    // Parse user_id
    let target_user_id = Uuid::parse_str(&user_id)
        .map_err(|_| ApiError::BadRequest("Invalid user ID".to_string()))?;

    // Get workspace
    let workspace = workspace_service
        .get_workspace_by_slug(&workspace_slug)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch workspace: {}", e)))?
        .ok_or_else(|| ApiError::NotFound("Workspace not found".to_string()))?;

    // Check if requesting user is admin
    let role = workspace_service
        .get_user_role(workspace.id, auth_user.id)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch role: {}", e)))?
        .ok_or_else(|| ApiError::Unauthorized("Not a member".to_string()))?;

    if role != WorkspaceRole::Admin {
        return Err(ApiError::Unauthorized(
            "Only admins can remove members".to_string(),
        ));
    }

    // Don't allow removing yourself
    if target_user_id == auth_user.id {
        return Err(ApiError::BadRequest(
            "Cannot remove yourself from workspace".to_string(),
        ));
    }

    // Remove member
    member_service
        .remove_member(workspace.id, target_user_id)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to remove member: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}
```

**Step 2: Add route to mod.rs**

In `jility-server/src/api/mod.rs`, add after the members list route:

```rust
.route("/api/workspaces/:slug/members/:user_id", delete(workspaces::remove_member))
```

**Step 3: Add remove_member to MemberService**

In `jility-server/src/services/member.rs`:

```rust
pub async fn remove_member(
    &self,
    workspace_id: Uuid,
    user_id: Uuid,
) -> Result<(), anyhow::Error> {
    use jility_core::entities::workspace_member;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    workspace_member::Entity::delete_many()
        .filter(workspace_member::Column::WorkspaceId.eq(workspace_id))
        .filter(workspace_member::Column::UserId.eq(user_id))
        .exec(&self.db)
        .await?;

    Ok(())
}
```

**Step 4: Build and test**

```bash
cd jility-server
cargo build
cargo test
```

**Step 5: Commit**

```bash
git add jility-server/src/api/workspaces.rs jility-server/src/api/mod.rs jility-server/src/services/member.rs
git commit -m "feat: add remove workspace member endpoint"
```

---

## Task 3: Create Frontend API Helper Functions

**Files:**
- Modify: `jility-web/lib/api.ts:~300` (add workspace member functions)

**Step 1: Add TypeScript types**

Add to `jility-web/lib/types.ts`:

```typescript
export interface WorkspaceMember {
  user_id: string
  email: string
  role: 'admin' | 'member'
  joined_at: string
}

export interface InviteMemberRequest {
  email: string
  role: 'admin' | 'member'
}
```

**Step 2: Add API functions to api.ts**

```typescript
// Workspace member management
listWorkspaceMembers: async (workspaceSlug: string): Promise<WorkspaceMember[]> => {
  const response = await fetch(`${API_BASE}/workspaces/${workspaceSlug}/members`, {
    headers: {
      Authorization: `Bearer ${localStorage.getItem('jility_token')}`,
    },
  })
  if (!response.ok) {
    throw new Error('Failed to fetch workspace members')
  }
  return response.json()
},

inviteWorkspaceMember: async (
  workspaceSlug: string,
  data: InviteMemberRequest
): Promise<void> => {
  const response = await fetch(`${API_BASE}/workspaces/${workspaceSlug}/invite`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${localStorage.getItem('jility_token')}`,
    },
    body: JSON.stringify(data),
  })
  if (!response.ok) {
    const error = await response.json()
    throw new Error(error.message || 'Failed to invite member')
  }
},

removeWorkspaceMember: async (
  workspaceSlug: string,
  userId: string
): Promise<void> => {
  const response = await fetch(
    `${API_BASE}/workspaces/${workspaceSlug}/members/${userId}`,
    {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${localStorage.getItem('jility_token')}`,
      },
    }
  )
  if (!response.ok) {
    throw new Error('Failed to remove member')
  }
},
```

**Step 3: Build**

```bash
cd jility-web
npm run build
```

**Step 4: Commit**

```bash
git add jility-web/lib/api.ts jility-web/lib/types.ts
git commit -m "feat: add workspace member API functions"
```

---

## Task 4: Create Workspace Settings Page

**Files:**
- Create: `jility-web/app/w/[slug]/settings/page.tsx`

**Step 1: Create settings page**

```typescript
'use client'

import { useEffect, useState } from 'react'
import { useParams } from 'next/navigation'
import { useWorkspace } from '@/lib/workspace-context'
import { api } from '@/lib/api'
import type { WorkspaceMember } from '@/lib/types'
import { WorkspaceMemberList } from '@/components/workspace/member-list'
import { InviteMemberDialog } from '@/components/workspace/invite-member-dialog'
import { Button } from '@/components/ui/button'
import { UserPlus } from 'lucide-react'

export default function WorkspaceSettingsPage() {
  const params = useParams()
  const slug = params.slug as string
  const { currentWorkspace } = useWorkspace()
  const [members, setMembers] = useState<WorkspaceMember[]>([])
  const [isLoading, setIsLoading] = useState(true)
  const [showInviteDialog, setShowInviteDialog] = useState(false)

  useEffect(() => {
    if (slug) {
      loadMembers()
    }
  }, [slug])

  const loadMembers = async () => {
    try {
      setIsLoading(true)
      const data = await api.listWorkspaceMembers(slug)
      setMembers(data)
    } catch (error) {
      console.error('Failed to load members:', error)
    } finally {
      setIsLoading(false)
    }
  }

  const handleInviteMember = async (email: string, role: 'admin' | 'member') => {
    try {
      await api.inviteWorkspaceMember(slug, { email, role })
      await loadMembers()
      setShowInviteDialog(false)
    } catch (error) {
      console.error('Failed to invite member:', error)
      throw error
    }
  }

  const handleRemoveMember = async (userId: string) => {
    if (!confirm('Are you sure you want to remove this member?')) {
      return
    }

    try {
      await api.removeWorkspaceMember(slug, userId)
      await loadMembers()
    } catch (error) {
      console.error('Failed to remove member:', error)
    }
  }

  const isAdmin = currentWorkspace?.role === 'admin'

  return (
    <div className="container max-w-4xl py-8">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-3xl font-bold">Workspace Settings</h1>
          <p className="text-muted-foreground mt-1">
            Manage your workspace members and settings
          </p>
        </div>
        {isAdmin && (
          <Button onClick={() => setShowInviteDialog(true)}>
            <UserPlus className="h-4 w-4 mr-2" />
            Invite Member
          </Button>
        )}
      </div>

      <div className="bg-card border border-border rounded-lg p-6">
        <h2 className="text-xl font-semibold mb-4">Team Members</h2>
        <WorkspaceMemberList
          members={members}
          isLoading={isLoading}
          isAdmin={isAdmin}
          onRemove={handleRemoveMember}
        />
      </div>

      <InviteMemberDialog
        open={showInviteDialog}
        onOpenChange={setShowInviteDialog}
        onInvite={handleInviteMember}
      />
    </div>
  )
}
```

**Step 2: Build**

```bash
npm run build
```

Expected: Build succeeds

**Step 3: Commit**

```bash
git add jility-web/app/w/[slug]/settings/page.tsx
git commit -m "feat: add workspace settings page"
```

---

## Task 5: Create Member List Component

**Files:**
- Create: `jility-web/components/workspace/member-list.tsx`

**Step 1: Create member list component**

```typescript
'use client'

import type { WorkspaceMember } from '@/lib/types'
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Trash2, Loader2 } from 'lucide-react'

interface WorkspaceMemberListProps {
  members: WorkspaceMember[]
  isLoading: boolean
  isAdmin: boolean
  onRemove: (userId: string) => void
}

export function WorkspaceMemberList({
  members,
  isLoading,
  isAdmin,
  onRemove,
}: WorkspaceMemberListProps) {
  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
      </div>
    )
  }

  if (members.length === 0) {
    return (
      <div className="text-center py-12 text-muted-foreground">
        No members found
      </div>
    )
  }

  return (
    <div className="space-y-4">
      {members.map((member) => (
        <div
          key={member.user_id}
          className="flex items-center justify-between p-4 border border-border rounded-lg"
        >
          <div className="flex items-center gap-3">
            <Avatar>
              <AvatarFallback>
                {member.email.slice(0, 2).toUpperCase()}
              </AvatarFallback>
            </Avatar>
            <div>
              <div className="font-medium">{member.email}</div>
              <div className="text-sm text-muted-foreground">
                Joined {new Date(member.joined_at).toLocaleDateString()}
              </div>
            </div>
          </div>

          <div className="flex items-center gap-2">
            <Badge variant={member.role === 'admin' ? 'default' : 'secondary'}>
              {member.role}
            </Badge>
            {isAdmin && (
              <Button
                variant="ghost"
                size="sm"
                onClick={() => onRemove(member.user_id)}
              >
                <Trash2 className="h-4 w-4 text-destructive" />
              </Button>
            )}
          </div>
        </div>
      ))}
    </div>
  )
}
```

**Step 2: Build and commit**

```bash
npm run build
git add jility-web/components/workspace/member-list.tsx
git commit -m "feat: add workspace member list component"
```

---

## Task 6: Create Invite Member Dialog

**Files:**
- Create: `jility-web/components/workspace/invite-member-dialog.tsx`

**Step 1: Create invite dialog component**

```typescript
'use client'

import { useState } from 'react'
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Loader2 } from 'lucide-react'

interface InviteMemberDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onInvite: (email: string, role: 'admin' | 'member') => Promise<void>
}

export function InviteMemberDialog({
  open,
  onOpenChange,
  onInvite,
}: InviteMemberDialogProps) {
  const [email, setEmail] = useState('')
  const [role, setRole] = useState<'admin' | 'member'>('member')
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)

    if (!email.trim() || !email.includes('@')) {
      setError('Please enter a valid email address')
      return
    }

    setIsSubmitting(true)

    try {
      await onInvite(email.trim(), role)
      setEmail('')
      setRole('member')
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to invite member')
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>Invite Team Member</DialogTitle>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="email">Email Address *</Label>
            <Input
              id="email"
              type="email"
              placeholder="colleague@example.com"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="role">Role</Label>
            <Select value={role} onValueChange={(v) => setRole(v as 'admin' | 'member')}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="member">Member</SelectItem>
                <SelectItem value="admin">Admin</SelectItem>
              </SelectContent>
            </Select>
            <p className="text-xs text-muted-foreground">
              Admins can invite and remove members
            </p>
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
              Send Invite
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
```

**Step 2: Build and commit**

```bash
npm run build
git add jility-web/components/workspace/invite-member-dialog.tsx
git commit -m "feat: add invite member dialog component"
```

---

## Task 7: Add Settings Link to Navigation

**Files:**
- Modify: `jility-web/components/layout/navbar.tsx:~35`

**Step 1: Add settings link**

In the `links` array, add after the "Agents" link:

```typescript
{ href: `/w/${slug}/settings`, label: 'Settings', icon: Settings },
```

Add the Settings import at the top:

```typescript
import { Settings, /* other imports */ } from 'lucide-react'
```

**Step 2: Build, test, and commit**

```bash
npm run build
git add jility-web/components/layout/navbar.tsx
git commit -m "feat: add settings link to navigation"
```

---

## Task 8: Test in Docker

**Step 1: Rebuild and start containers**

```bash
docker-compose down
docker-compose up --build -d
```

**Step 2: Manual testing checklist**

- [ ] Navigate to `/w/{workspace-slug}/settings`
- [ ] See list of workspace members
- [ ] Invite a new member (admin only)
- [ ] Remove a member (admin only)
- [ ] Verify non-admins can't see invite/remove buttons

**Step 3: Final commit**

```bash
git add -A
git commit -m "feat: workspace member management complete"
git push
```
