# Workspace Multi-Tenancy Design

**Date:** November 6, 2025
**Status:** Approved
**Goal:** Add workspace-based multi-tenancy to Jility for team collaboration

## Overview

Transform Jility from single-user to multi-tenant workspace model where:
- Users can create and join multiple workspaces
- Each workspace contains projects and tickets
- Workspace admins control membership and permissions
- All members see all projects within their workspace

**Design Philosophy:** Cleaner, faster JIRA alternative with minimal complexity.

## 1. Data Model & Entities

### New Tables

#### `workspace`
Core container for team collaboration.

```sql
CREATE TABLE workspace (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,  -- URL-friendly, e.g., "my-workspace"
    created_by_user_id UUID NOT NULL REFERENCES user(id),
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
```

**Business Rules:**
- Slug must be unique globally
- Slug auto-generated from name, editable
- Created by user is automatically added as admin member

#### `workspace_member`
Links users to workspaces with roles.

```sql
CREATE TABLE workspace_member (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspace(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES user(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL CHECK (role IN ('admin', 'member')),
    invited_by_user_id UUID REFERENCES user(id),  -- NULL for workspace creator
    invited_at TIMESTAMP,
    joined_at TIMESTAMP NOT NULL,
    UNIQUE(workspace_id, user_id)  -- User cannot be member twice
);
```

**Business Rules:**
- Each workspace must have at least one admin
- Users can be members of multiple workspaces
- Last admin cannot leave or be demoted without promoting another admin

#### `workspace_invite`
Tracks pending email invitations.

```sql
CREATE TABLE workspace_invite (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspace(id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL CHECK (role IN ('admin', 'member')),
    invited_by_user_id UUID NOT NULL REFERENCES user(id),
    token VARCHAR(255) UNIQUE NOT NULL,  -- Magic link token
    expires_at TIMESTAMP NOT NULL,
    accepted_at TIMESTAMP,  -- NULL if pending
    created_at TIMESTAMP NOT NULL
);
```

**Business Rules:**
- Invites expire after 7 days
- Token is UUID, used in magic link URL
- Cannot invite same email twice to same workspace (pending or accepted)
- Accepting invite creates workspace_member record and sets accepted_at

### Modified Tables

#### `project`
Add workspace ownership.

```sql
ALTER TABLE project
ADD COLUMN workspace_id UUID NOT NULL REFERENCES workspace(id) ON DELETE CASCADE;

CREATE INDEX idx_project_workspace ON project(workspace_id);
```

**Business Rules:**
- Every project belongs to exactly one workspace
- All workspace members can see all projects
- Projects cannot be moved between workspaces (future feature)

### Entity Relationships

```
User (1) ──< (N) WorkspaceMember (N) >── (1) Workspace
                                              │
                                              └──< (N) Project ──< (N) Ticket
User (1) ──< (N) WorkspaceInvite >── (1) Workspace
```

### Auto-Creation on Signup

When a user registers:
1. Create user record
2. Auto-create workspace:
   - `name`: "{username}'s Workspace"
   - `slug`: generate from username (e.g., "zachs-workspace")
   - `created_by_user_id`: new user ID
3. Create workspace_member:
   - `user_id`: new user
   - `workspace_id`: new workspace
   - `role`: "admin"
   - `joined_at`: now

## 2. URL Structure & Routing

### URL Pattern

All workspace-specific routes prefixed with `/w/:workspace_slug/`:

**Public Routes:**
```
/login
/register
/reset-password
/invite/:token  -- Accept workspace invite
```

**Workspace Routes:**
```
/w/:workspace_slug/board              -- Kanban board (main view)
/w/:workspace_slug/projects           -- Project list
/w/:workspace_slug/projects/new       -- Create project
/w/:workspace_slug/projects/:id       -- Project details
/w/:workspace_slug/tickets            -- All tickets view
/w/:workspace_slug/settings           -- Workspace settings (admin only)
/w/:workspace_slug/members            -- Member management (admin only)
```

**Post-Login Redirect:**
```
1. Get user's workspaces (ordered by last_accessed or created_at)
2. Redirect to /w/{first-workspace-slug}/board
3. If no workspaces (edge case), create default workspace first
```

### Workspace Switcher UI

**Location:** Navbar (top-left)

**Features:**
- Dropdown showing current workspace name
- List of user's workspaces with role badges (Admin/Member)
- Visual indicator for current workspace (checkmark)
- "+ Create New Workspace" option at bottom
- Workspace count display: "3 workspaces"
- Keyboard shortcut: Cmd/Ctrl + K to open switcher

**Switching Behavior:**
- Click workspace → Navigate to `/w/{new-workspace-slug}/board`
- Recently accessed workspaces appear at top

### Backend Middleware

**Workspace Context Extraction:**

```rust
// Middleware extracts workspace from URL and validates access
struct WorkspaceContext {
    workspace_id: Uuid,
    workspace_slug: String,
    user_role: WorkspaceRole,  // admin or member
}

async fn workspace_middleware(
    Path(workspace_slug): Path<String>,
    Extension(user): Extension<AuthUser>,
) -> Result<WorkspaceContext> {
    // 1. Lookup workspace by slug
    // 2. Verify user is member of workspace
    // 3. Get user's role in workspace
    // 4. Return 403 if not a member
    // 5. Inject WorkspaceContext into request
}
```

**Query Scoping:**
- All database queries automatically filter by `workspace_id`
- Controllers access `workspace_id` from context
- Prevents cross-workspace data leakage

### Frontend Route Guards

```typescript
// All /w/* routes require authentication
<Route path="/w/:workspaceSlug/*" element={<RequireAuth />}>
  <Route path="board" element={<BoardPage />} />
  <Route path="projects" element={<ProjectsPage />} />

  // Admin-only routes
  <Route path="settings" element={<RequireAdmin><SettingsPage /></RequireAdmin>} />
  <Route path="members" element={<RequireAdmin><MembersPage /></RequireAdmin>} />
</Route>
```

## 3. Permissions & Authorization

### Role-Based Access Control (RBAC)

Two roles only for simplicity:

```rust
enum WorkspaceRole {
    Admin,   // Full control of workspace
    Member   // Full ticket access, no workspace management
}
```

### Permission Matrix

| Capability | Admin | Member |
|-----------|-------|--------|
| **Workspace Management** |
| Invite members | ✅ | ❌ |
| Remove members | ✅ | ❌ |
| Change member roles | ✅ | ❌ |
| Edit workspace settings (name, slug) | ✅ | ❌ |
| Delete workspace | ✅ | ❌ |
| Access billing settings | ✅ | ❌ |
| **Project Management** |
| Create projects | ✅ | ✅ |
| Edit projects | ✅ | ✅ |
| Delete projects | ✅ | ✅ |
| **Ticket Management** |
| Create tickets | ✅ | ✅ |
| Edit tickets | ✅ | ✅ |
| Delete tickets | ✅ | ✅ |
| Assign tickets | ✅ | ✅ |
| Move ticket status | ✅ | ✅ |
| Add comments | ✅ | ✅ |
| **Other** |
| View workspace members | ✅ | ✅ |
| Leave workspace | ✅ | ✅ |

### Authorization Checks

**Middleware Pattern:**

```rust
// Require any workspace member
async fn require_workspace_member(
    workspace_slug: &str,
    user_id: Uuid
) -> Result<WorkspaceContext>

// Require workspace admin
async fn require_workspace_admin(
    workspace_slug: &str,
    user_id: Uuid
) -> Result<WorkspaceContext>
```

**Controller Usage:**

```rust
// Admin-only endpoint
async fn delete_workspace(
    ctx: Extension<WorkspaceContext>,  // Role guaranteed to be Admin by middleware
) -> Result<()> {
    // Delete logic
}
```

### Edge Cases

| Scenario | Behavior |
|----------|----------|
| User accesses workspace they're not member of | Return 403 Forbidden |
| Last admin tries to leave workspace | Error: "Must promote another admin first" |
| Last admin tries to demote self | Error: "Cannot demote last admin" |
| Admin removes user from workspace | User loses access immediately, redirected to another workspace |
| Invited user already has account | Auto-join on login if invite valid |
| Invited email doesn't have account | Prompt to register, then auto-join |

## 4. Invite Flow & Member Management

### Email-Based Invite Flow

**Step 1: Admin Sends Invites**

UI: `/w/{workspace-slug}/members` page
- Admin enters email addresses (comma-separated for bulk)
- Selects role: Admin or Member (defaults to Member)
- Clicks "Send Invites"

Backend creates:
```rust
workspace_invite {
    workspace_id: current_workspace,
    email: invitee_email,
    role: selected_role,
    invited_by_user_id: current_user,
    token: Uuid::new_v4(),  // Random UUID
    expires_at: now + 7 days,
    created_at: now
}
```

**Step 2: Email Sent**

```
Subject: [Admin Name] invited you to [Workspace Name] on Jility

Body:
You've been invited to join [Workspace Name] as a [Member/Admin].

[Admin Name] thinks you'd be a great addition to the team.

[Accept Invitation Button] → https://jility.app/invite/{token}

This invitation expires in 7 days.
```

**Step 3: User Clicks Invite Link**

Route: `/invite/:token`

**Case A - User Not Logged In:**
```
1. Redirect to /login?invite_token={token}
2. After successful login:
   - Verify token still valid (not expired)
   - Create workspace_member record
   - Mark invite as accepted (set accepted_at)
   - Redirect to /w/{workspace-slug}/board
```

**Case B - User Has No Account:**
```
1. Redirect to /register?invite_token={token}&email={prefilled}
2. Pre-fill email in registration form
3. After successful registration:
   - Verify token still valid
   - Create workspace_member record
   - Mark invite as accepted
   - Redirect to /w/{workspace-slug}/board
```

**Case C - User Already Logged In:**
```
1. Show "Accept Invite" page:
   - Workspace name and description
   - "You've been invited as a [Role]"
   - Current user's email
2. User clicks "Join Workspace" button
3. Create workspace_member record
4. Mark invite as accepted
5. Redirect to /w/{workspace-slug}/board
```

**Invite Validation:**
```rust
// Check invite validity
fn validate_invite(token: &str) -> Result<WorkspaceInvite> {
    let invite = find_invite_by_token(token)?;

    if invite.accepted_at.is_some() {
        return Err("Invite already accepted");
    }

    if invite.expires_at < now() {
        return Err("Invite expired");
    }

    // Check if user already member
    if is_member(invite.workspace_id, current_user.id) {
        return Err("Already a member of this workspace");
    }

    Ok(invite)
}
```

### Member Management UI

**Admin View:** `/w/{workspace-slug}/members`

**Section 1: Active Members**

Table with columns:
- **Avatar + Name** (with "You" badge for current user)
- **Email**
- **Role** (badge: Admin or Member)
- **Joined Date**
- **Actions** (dropdown):
  - "Change to Admin" / "Change to Member"
  - "Remove from Workspace"

**Section 2: Pending Invites**

Table with columns:
- **Email**
- **Role** (what they'll be invited as)
- **Invited By** (admin name)
- **Sent Date**
- **Status** (e.g., "Expires in 3 days" or "Expired")
- **Actions**:
  - "Resend Invite"
  - "Cancel Invite"

**Section 3: Invite New Members**

Button at top: "+ Invite Members"

Opens modal:
- **Email addresses:** (textarea, comma-separated)
- **Role:** [Dropdown: Member (default) | Admin]
- **Buttons:** [Cancel] [Send Invites]

**Member View (Non-Admin):**

Read-only view of active members list. Cannot see pending invites or action buttons.

## 5. Migration Strategy

### Clean Slate Approach

**Decision:** Reset existing database since only test accounts exist.

**Migration Steps:**

```sql
-- New migration file: m20251106_add_workspaces.rs

1. Create workspace table
2. Create workspace_member table
3. Create workspace_invite table
4. Add workspace_id column to project table (NOT NULL)
5. Add foreign key constraints
6. Add indexes for performance
```

**Database Reset:**

Option A (SQLite):
```bash
rm .jility/data.db
cargo run  # Automatically runs migrations on startup
```

Option B (Reset flag):
```bash
cargo run -- --reset-db
```

### Updated Registration Flow

```rust
async fn register_user(email: &str, username: &str, password: &str) -> Result<User> {
    // 1. Create user
    let user = create_user(email, username, password).await?;

    // 2. Auto-create workspace
    let workspace = Workspace {
        name: format!("{}'s Workspace", username),
        slug: generate_slug(username),  // e.g., "zachs-workspace"
        created_by_user_id: user.id,
    };
    let workspace = workspace.insert(&db).await?;

    // 3. Add user as admin member
    let member = WorkspaceMember {
        workspace_id: workspace.id,
        user_id: user.id,
        role: WorkspaceRole::Admin,
        invited_by_user_id: None,  // Self-created
        invited_at: None,
        joined_at: Utc::now(),
    };
    member.insert(&db).await?;

    Ok(user)
}
```

## 6. Workspace Settings & Creation

### Workspace Settings Page

**Route:** `/w/{workspace-slug}/settings` (Admin only)

**Section 1: Workspace Details**

Form fields:
- **Workspace Name:** [Input] (editable)
- **Workspace Slug:** [Input] (editable, validates uniqueness)
- **Created Date:** [Read-only, formatted]
- **Created By:** [Read-only, user's name]

**Actions:**
- [Cancel] [Save Changes] buttons

**Section 2: Danger Zone**

Red-bordered section:
- **Delete Workspace**
  - Button: "Delete Workspace..."
  - Opens confirmation modal
  - Requires typing workspace name to confirm
  - Warning text: "This will permanently delete all projects, tickets, comments, and data. This cannot be undone."
  - Only enabled if user is an admin

**Validation:**
- Slug must be unique across all workspaces
- Slug must be URL-safe (lowercase, hyphens, alphanumeric)
- Name cannot be empty

### Create New Workspace Flow

**Trigger:** User clicks "+ Create Workspace" in workspace switcher

**Modal:**
```
Create New Workspace

Workspace Name: [Input field]
Slug: [Auto-generated, editable]  (e.g., "my-new-workspace")

[Cancel]  [Create Workspace]
```

**Behavior:**
1. Name input triggers slug auto-generation
   - "My New Workspace" → "my-new-workspace"
   - User can manually edit slug
2. Validate slug uniqueness on blur
3. On create:
   - Create workspace record
   - Add user as admin member
   - Redirect to `/w/{new-slug}/board`
4. New workspace starts empty (no projects)

**Slug Generation Rules:**
```rust
fn generate_slug(name: &str) -> String {
    name.to_lowercase()
        .replace(" ", "-")
        .replace(|c: char| !c.is_alphanumeric() && c != '-', "")
        .trim_matches('-')
        .to_string()
}
```

## Implementation Phases

### Phase 1: Database & Models
- Create SeaORM entity models
- Write database migrations
- Add workspace context to auth middleware

### Phase 2: Backend APIs
- Workspace CRUD endpoints
- Member management endpoints
- Invite creation and acceptance endpoints
- Email service for invites

### Phase 3: Frontend Routing
- Update router for `/w/:workspace_slug/` pattern
- Workspace context provider
- Workspace switcher component
- Route guards for admin-only pages

### Phase 4: Member Management UI
- Members page (list, invite, manage)
- Workspace settings page
- Create workspace modal
- Accept invite page

### Phase 5: Testing & Polish
- Unit tests for permissions
- Integration tests for invite flow
- E2E tests for workspace switching
- Performance testing for multi-workspace queries

## Success Criteria

- ✅ Users can create multiple workspaces
- ✅ Users can switch between workspaces via dropdown
- ✅ Admins can invite members via email with magic links
- ✅ Members can only access their workspace's data
- ✅ Role-based permissions enforced (admin vs member)
- ✅ Workspace settings fully functional
- ✅ Clean database migration from single-user to multi-tenant
- ✅ All existing features work within workspace context

## Future Enhancements

Not in initial scope:
- Workspace billing and subscription management
- Per-project permissions (workspace-level is sufficient for MVP)
- Workspace transfer (change owner)
- Workspace activity feed
- Workspace analytics/insights
- Multiple invite methods (shareable links)
- Custom workspace themes/branding
