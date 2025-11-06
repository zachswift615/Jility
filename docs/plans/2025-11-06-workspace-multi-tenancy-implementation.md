# Workspace Multi-Tenancy Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add workspace-based multi-tenancy to enable team collaboration with admins and members.

**Architecture:** Workspace-first model with URL-based context (`/w/:slug/`), two-role RBAC (admin/member), email-based invites, auto-workspace creation on signup.

**Tech Stack:** Rust (SeaORM, Axum), Next.js 14, PostgreSQL/SQLite, TypeScript, Tailwind CSS

**Design Doc:** See `docs/plans/2025-11-06-workspace-multi-tenancy-design.md` for full specification.

---

## Phase 1: Database Entities & Migrations

### Task 1.1: Create Workspace Entity

**Files:**
- Create: `crates/jility-core/src/entities/workspace.rs`

**Step 1: Create workspace entity model**

```rust
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "workspace")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub name: String,

    #[sea_orm(unique)]
    pub slug: String,

    pub created_by_user_id: Uuid,

    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::CreatedByUserId",
        to = "super::user::Column::Id"
    )]
    CreatedBy,

    #[sea_orm(has_many = "super::project::Entity")]
    Projects,

    #[sea_orm(has_many = "super::workspace_member::Entity")]
    Members,

    #[sea_orm(has_many = "super::workspace_invite::Entity")]
    Invites,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CreatedBy.def()
    }
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
```

**Step 2: Add workspace to entities/mod.rs**

Edit `crates/jility-core/src/entities/mod.rs`:

Add after line 14 (after `pub mod user;`):
```rust
pub mod workspace;
pub mod workspace_invite;
pub mod workspace_member;
```

Add exports after line 29:
```rust
pub use workspace::Entity as Workspace;
pub use workspace_invite::Entity as WorkspaceInvite;
pub use workspace_member::Entity as WorkspaceMember;

pub use workspace::Model as WorkspaceModel;
pub use workspace_invite::Model as WorkspaceInviteModel;
pub use workspace_member::Model as WorkspaceMemberModel;
pub use workspace_member::WorkspaceRole;
```

**Step 3: Commit**

```bash
git add crates/jility-core/src/entities/workspace.rs crates/jility-core/src/entities/mod.rs
git commit -m "feat(core): add workspace entity model"
```

---

### Task 1.2: Create WorkspaceMember Entity

**Files:**
- Create: `crates/jility-core/src/entities/workspace_member.rs`

**Step 1: Create workspace_member entity**

```rust
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(50))")]
pub enum WorkspaceRole {
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "member")]
    Member,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "workspace_member")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub workspace_id: Uuid,
    pub user_id: Uuid,

    pub role: WorkspaceRole,

    pub invited_by_user_id: Option<Uuid>,
    pub invited_at: Option<DateTimeWithTimeZone>,
    pub joined_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::workspace::Entity",
        from = "Column::WorkspaceId",
        to = "super::workspace::Column::Id",
        on_delete = "Cascade"
    )]
    Workspace,

    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::workspace::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Workspace.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
```

**Step 2: Commit**

```bash
git add crates/jility-core/src/entities/workspace_member.rs
git commit -m "feat(core): add workspace_member entity with role enum"
```

---

### Task 1.3: Create WorkspaceInvite Entity

**Files:**
- Create: `crates/jility-core/src/entities/workspace_invite.rs`

**Step 1: Create workspace_invite entity**

```rust
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "workspace_invite")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub workspace_id: Uuid,

    pub email: String,

    pub role: super::workspace_member::WorkspaceRole,

    pub invited_by_user_id: Uuid,

    #[sea_orm(unique)]
    pub token: String,

    pub expires_at: DateTimeWithTimeZone,
    pub accepted_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::workspace::Entity",
        from = "Column::WorkspaceId",
        to = "super::workspace::Column::Id",
        on_delete = "Cascade"
    )]
    Workspace,

    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::InvitedByUserId",
        to = "super::user::Column::Id"
    )]
    InvitedBy,
}

impl Related<super::workspace::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Workspace.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
```

**Step 2: Commit**

```bash
git add crates/jility-core/src/entities/workspace_invite.rs
git commit -m "feat(core): add workspace_invite entity for email invitations"
```

---

### Task 1.4: Add workspace_id to Project Entity

**Files:**
- Modify: `crates/jility-core/src/entities/project.rs:6-35`

**Step 1: Add workspace_id field to project**

Edit `crates/jility-core/src/entities/project.rs`:

After line 8 (`pub id: Uuid,`), add:
```rust
    pub workspace_id: Uuid,
```

In the Relation enum (after line 37), add:
```rust
    #[sea_orm(
        belongs_to = "super::workspace::Entity",
        from = "Column::WorkspaceId",
        to = "super::workspace::Column::Id",
        on_delete = "Cascade"
    )]
    Workspace,
```

After the existing Related implementations, add:
```rust
impl Related<super::workspace::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Workspace.def()
    }
}
```

**Step 2: Commit**

```bash
git add crates/jility-core/src/entities/project.rs
git commit -m "feat(core): add workspace_id to project entity"
```

---

### Task 1.5: Create Database Migration

**Files:**
- Create: `crates/jility-core/src/migration/m20251106_000001_add_workspaces.rs`
- Modify: `crates/jility-core/src/migration/mod.rs`

**Step 1: Create migration file**

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create workspace table
        manager
            .create_table(
                Table::create()
                    .table(Workspace::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Workspace::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Workspace::Name).string().not_null())
                    .col(ColumnDef::new(Workspace::Slug).string().not_null().unique_key())
                    .col(ColumnDef::new(Workspace::CreatedByUserId).uuid().not_null())
                    .col(ColumnDef::new(Workspace::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Workspace::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Workspace::Table, Workspace::CreatedByUserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create workspace_member table
        manager
            .create_table(
                Table::create()
                    .table(WorkspaceMember::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WorkspaceMember::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(WorkspaceMember::WorkspaceId).uuid().not_null())
                    .col(ColumnDef::new(WorkspaceMember::UserId).uuid().not_null())
                    .col(ColumnDef::new(WorkspaceMember::Role).string_len(50).not_null())
                    .col(ColumnDef::new(WorkspaceMember::InvitedByUserId).uuid())
                    .col(ColumnDef::new(WorkspaceMember::InvitedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(WorkspaceMember::JoinedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(WorkspaceMember::Table, WorkspaceMember::WorkspaceId)
                            .to(Workspace::Table, Workspace::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WorkspaceMember::Table, WorkspaceMember::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Add unique constraint on workspace_id + user_id
        manager
            .create_index(
                Index::create()
                    .table(WorkspaceMember::Table)
                    .name("idx_workspace_member_unique")
                    .col(WorkspaceMember::WorkspaceId)
                    .col(WorkspaceMember::UserId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create workspace_invite table
        manager
            .create_table(
                Table::create()
                    .table(WorkspaceInvite::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WorkspaceInvite::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(WorkspaceInvite::WorkspaceId).uuid().not_null())
                    .col(ColumnDef::new(WorkspaceInvite::Email).string().not_null())
                    .col(ColumnDef::new(WorkspaceInvite::Role).string_len(50).not_null())
                    .col(ColumnDef::new(WorkspaceInvite::InvitedByUserId).uuid().not_null())
                    .col(ColumnDef::new(WorkspaceInvite::Token).string().not_null().unique_key())
                    .col(ColumnDef::new(WorkspaceInvite::ExpiresAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(WorkspaceInvite::AcceptedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(WorkspaceInvite::CreatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(WorkspaceInvite::Table, WorkspaceInvite::WorkspaceId)
                            .to(Workspace::Table, Workspace::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WorkspaceInvite::Table, WorkspaceInvite::InvitedByUserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Add workspace_id to project table
        manager
            .alter_table(
                Table::alter()
                    .table(Project::Table)
                    .add_column(ColumnDef::new(Project::WorkspaceId).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        // Add foreign key for project.workspace_id
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Project::Table, Project::WorkspaceId)
                    .to(Workspace::Table, Workspace::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Create indexes for performance
        manager
            .create_index(
                Index::create()
                    .table(WorkspaceMember::Table)
                    .name("idx_workspace_member_user")
                    .col(WorkspaceMember::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Project::Table)
                    .name("idx_project_workspace")
                    .col(Project::WorkspaceId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WorkspaceInvite::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(WorkspaceMember::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Workspace::Table).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Project::Table)
                    .drop_column(Project::WorkspaceId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Workspace {
    Table,
    Id,
    Name,
    Slug,
    CreatedByUserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum WorkspaceMember {
    Table,
    Id,
    WorkspaceId,
    UserId,
    Role,
    InvitedByUserId,
    InvitedAt,
    JoinedAt,
}

#[derive(Iden)]
enum WorkspaceInvite {
    Table,
    Id,
    WorkspaceId,
    Email,
    Role,
    InvitedByUserId,
    Token,
    ExpiresAt,
    AcceptedAt,
    CreatedAt,
}

#[derive(Iden)]
enum Project {
    Table,
    WorkspaceId,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
```

**Step 2: Register migration in mod.rs**

Edit `crates/jility-core/src/migration/mod.rs`, add after last migration:
```rust
mod m20251106_000001_add_workspaces;
```

In the `impl MigratorTrait` implementation, add to the vec:
```rust
            Box::new(m20251106_000001_add_workspaces::Migration),
```

**Step 3: Test migration**

Run: `cargo build -p jility-core`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add crates/jility-core/src/migration/
git commit -m "feat(migration): add workspace tables and relations"
```

---

## Phase 2: Backend Services & Utilities

### Task 2.1: Create Slug Generation Utility

**Files:**
- Create: `crates/jility-core/src/utils/slug.rs`
- Modify: `crates/jility-core/src/lib.rs`

**Step 1: Write failing test**

Create `crates/jility-core/src/utils/slug.rs`:
```rust
/// Generate URL-friendly slug from a name
pub fn generate_slug(name: &str) -> String {
    name.to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != ' ' && c != '-', "")
        .replace(' ', "-")
        .trim_matches('-')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_slug_basic() {
        assert_eq!(generate_slug("My Workspace"), "my-workspace");
    }

    #[test]
    fn test_generate_slug_special_chars() {
        assert_eq!(generate_slug("John's Team!"), "johns-team");
    }

    #[test]
    fn test_generate_slug_multiple_spaces() {
        assert_eq!(generate_slug("  Many   Spaces  "), "many-spaces");
    }

    #[test]
    fn test_generate_slug_already_slug() {
        assert_eq!(generate_slug("already-a-slug"), "already-a-slug");
    }

    #[test]
    fn test_generate_slug_unicode() {
        assert_eq!(generate_slug("Café Résumé"), "caf-rsum");
    }
}
```

**Step 2: Run tests to verify implementation works**

Run: `cargo test -p jility-core generate_slug`
Expected: All tests PASS

**Step 3: Add utils module to lib.rs**

Edit `crates/jility-core/src/lib.rs`, add after line 13:
```rust
pub mod utils;
```

Also add export:
```rust
pub use utils::slug;
```

**Step 4: Commit**

```bash
git add crates/jility-core/src/utils/ crates/jility-core/src/lib.rs
git commit -m "feat(core): add slug generation utility with tests"
```

---

### Task 2.2: Create Workspace Service

**Files:**
- Create: `crates/jility-server/src/services/workspace.rs`
- Modify: `crates/jility-server/src/services/mod.rs`

**Step 1: Create workspace service**

```rust
use anyhow::{anyhow, Context, Result};
use jility_core::{
    entities::{workspace, workspace_member, Workspace, WorkspaceMember, WorkspaceRole},
    slug::generate_slug,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

pub struct WorkspaceService {
    db: DatabaseConnection,
}

impl WorkspaceService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Create a new workspace
    pub async fn create_workspace(
        &self,
        name: String,
        created_by_user_id: Uuid,
    ) -> Result<workspace::Model> {
        let slug = generate_slug(&name);

        // Check if slug already exists
        let existing = Workspace::find()
            .filter(workspace::Column::Slug.eq(&slug))
            .one(&self.db)
            .await?;

        if existing.is_some() {
            return Err(anyhow!("Workspace slug '{}' already exists", slug));
        }

        let now = chrono::Utc::now().fixed_offset();
        let workspace_id = Uuid::new_v4();

        // Create workspace
        let workspace = workspace::ActiveModel {
            id: Set(workspace_id),
            name: Set(name),
            slug: Set(slug),
            created_by_user_id: Set(created_by_user_id),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let workspace = workspace.insert(&self.db).await?;

        // Add creator as admin member
        let member = workspace_member::ActiveModel {
            id: Set(Uuid::new_v4()),
            workspace_id: Set(workspace_id),
            user_id: Set(created_by_user_id),
            role: Set(WorkspaceRole::Admin),
            invited_by_user_id: Set(None),
            invited_at: Set(None),
            joined_at: Set(now),
        };

        member.insert(&self.db).await?;

        Ok(workspace)
    }

    /// Get workspace by slug
    pub async fn get_workspace_by_slug(&self, slug: &str) -> Result<Option<workspace::Model>> {
        let workspace = Workspace::find()
            .filter(workspace::Column::Slug.eq(slug))
            .one(&self.db)
            .await?;

        Ok(workspace)
    }

    /// Get user's workspaces
    pub async fn get_user_workspaces(&self, user_id: Uuid) -> Result<Vec<workspace::Model>> {
        // Find workspaces where user is a member
        let members = WorkspaceMember::find()
            .filter(workspace_member::Column::UserId.eq(user_id))
            .all(&self.db)
            .await?;

        let workspace_ids: Vec<Uuid> = members.iter().map(|m| m.workspace_id).collect();

        let workspaces = Workspace::find()
            .filter(workspace::Column::Id.is_in(workspace_ids))
            .all(&self.db)
            .await?;

        Ok(workspaces)
    }

    /// Check if user is member of workspace
    pub async fn is_member(&self, workspace_id: Uuid, user_id: Uuid) -> Result<bool> {
        let member = WorkspaceMember::find()
            .filter(workspace_member::Column::WorkspaceId.eq(workspace_id))
            .filter(workspace_member::Column::UserId.eq(user_id))
            .one(&self.db)
            .await?;

        Ok(member.is_some())
    }

    /// Get user's role in workspace
    pub async fn get_user_role(
        &self,
        workspace_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<WorkspaceRole>> {
        let member = WorkspaceMember::find()
            .filter(workspace_member::Column::WorkspaceId.eq(workspace_id))
            .filter(workspace_member::Column::UserId.eq(user_id))
            .one(&self.db)
            .await?;

        Ok(member.map(|m| m.role))
    }
}
```

**Step 2: Add to services mod**

Edit `crates/jility-server/src/services/mod.rs`, add:
```rust
pub mod workspace;
pub use workspace::WorkspaceService;
```

**Step 3: Commit**

```bash
git add crates/jility-server/src/services/
git commit -m "feat(server): add workspace service with CRUD operations"
```

---

### Task 2.3: Update Auth Service for Auto-Workspace Creation

**Files:**
- Modify: `crates/jility-server/src/api/auth.rs` (registration endpoint)

**Step 1: Update register endpoint to create workspace**

Find the `register` handler function in `crates/jility-server/src/api/auth.rs` and update it to create a workspace after user creation:

```rust
// After user is created, add this code:

// Auto-create workspace for new user
let workspace_service = WorkspaceService::new(db.clone());
let workspace_name = format!("{}'s Workspace", username);

match workspace_service.create_workspace(workspace_name, user.id).await {
    Ok(workspace) => {
        tracing::info!("Created workspace {} for user {}", workspace.slug, user.id);
    }
    Err(e) => {
        tracing::error!("Failed to create workspace for user {}: {}", user.id, e);
        // Continue anyway - user can create workspace later
    }
}
```

**Step 2: Commit**

```bash
git add crates/jility-server/src/api/auth.rs
git commit -m "feat(auth): auto-create workspace on user registration"
```

---

## Phase 3: Backend API Endpoints

### Task 3.1: Create Workspace Middleware

**Files:**
- Create: `crates/jility-server/src/middleware/workspace.rs`
- Modify: `crates/jility-server/src/middleware/mod.rs`

**Step 1: Create workspace context extractor**

```rust
use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
    Extension,
};
use jility_core::entities::WorkspaceRole;
use uuid::Uuid;

use crate::{auth::AuthUser, services::WorkspaceService};

/// Workspace context extracted from URL and validated
#[derive(Clone, Debug)]
pub struct WorkspaceContext {
    pub workspace_id: Uuid,
    pub workspace_slug: String,
    pub user_role: WorkspaceRole,
}

#[async_trait]
impl<S> FromRequestParts<S> for WorkspaceContext
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract workspace slug from path
        let path_params = parts
            .extensions
            .get::<axum::extract::MatchedPath>()
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to extract path".to_string(),
            ))?;

        // Get workspace slug from URL (format: /w/:workspace_slug/...)
        let workspace_slug = path_params
            .as_str()
            .split('/')
            .nth(2)
            .ok_or((
                StatusCode::BAD_REQUEST,
                "Workspace slug not found in URL".to_string(),
            ))?
            .to_string();

        // Get authenticated user
        let user = parts
            .extensions
            .get::<AuthUser>()
            .ok_or((StatusCode::UNAUTHORIZED, "Not authenticated".to_string()))?;

        // Get workspace service from extensions
        let workspace_service = parts
            .extensions
            .get::<WorkspaceService>()
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Workspace service not found".to_string(),
            ))?;

        // Get workspace by slug
        let workspace = workspace_service
            .get_workspace_by_slug(&workspace_slug)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database error: {}", e),
                )
            })?
            .ok_or((
                StatusCode::NOT_FOUND,
                format!("Workspace '{}' not found", workspace_slug),
            ))?;

        // Check if user is member
        let is_member = workspace_service
            .is_member(workspace.id, user.user_id)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database error: {}", e),
                )
            })?;

        if !is_member {
            return Err((
                StatusCode::FORBIDDEN,
                "You are not a member of this workspace".to_string(),
            ));
        }

        // Get user's role
        let user_role = workspace_service
            .get_user_role(workspace.id, user.user_id)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database error: {}", e),
                )
            })?
            .ok_or((
                StatusCode::FORBIDDEN,
                "User role not found".to_string(),
            ))?;

        Ok(WorkspaceContext {
            workspace_id: workspace.id,
            workspace_slug: workspace.slug,
            user_role,
        })
    }
}

/// Require admin role in workspace
pub struct RequireAdmin(pub WorkspaceContext);

#[async_trait]
impl<S> FromRequestParts<S> for RequireAdmin
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx = WorkspaceContext::from_request_parts(parts, state).await?;

        if ctx.user_role != WorkspaceRole::Admin {
            return Err((
                StatusCode::FORBIDDEN,
                "Admin role required for this action".to_string(),
            ));
        }

        Ok(RequireAdmin(ctx))
    }
}
```

**Step 2: Add to middleware mod**

Edit `crates/jility-server/src/middleware/mod.rs`:
```rust
pub mod workspace;
pub use workspace::{RequireAdmin, WorkspaceContext};
```

**Step 3: Commit**

```bash
git add crates/jility-server/src/middleware/
git commit -m "feat(middleware): add workspace context extraction and admin guard"
```

---

### Task 3.2: Create Workspace API Endpoints

**Files:**
- Create: `crates/jility-server/src/api/workspaces.rs`
- Modify: `crates/jility-server/src/api/mod.rs`

**Step 1: Create workspace API handlers**

```rust
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth::AuthUser,
    middleware::{RequireAdmin, WorkspaceContext},
    services::WorkspaceService,
};
use jility_core::entities::{workspace, WorkspaceRole};

#[derive(Serialize)]
pub struct WorkspaceResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub role: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
}

/// Get user's workspaces
pub async fn list_workspaces(
    Extension(auth_user): Extension<AuthUser>,
    Extension(workspace_service): Extension<WorkspaceService>,
) -> Result<Json<Vec<WorkspaceResponse>>, (StatusCode, String)> {
    let workspaces = workspace_service
        .get_user_workspaces(auth_user.user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch workspaces: {}", e),
            )
        })?;

    // Get role for each workspace
    let mut responses = Vec::new();
    for workspace in workspaces {
        let role = workspace_service
            .get_user_role(workspace.id, auth_user.user_id)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to fetch role: {}", e),
                )
            })?
            .unwrap_or(WorkspaceRole::Member);

        responses.push(WorkspaceResponse {
            id: workspace.id.to_string(),
            name: workspace.name,
            slug: workspace.slug,
            role: match role {
                WorkspaceRole::Admin => "admin".to_string(),
                WorkspaceRole::Member => "member".to_string(),
            },
            created_at: workspace.created_at.to_string(),
        });
    }

    Ok(Json(responses))
}

/// Get workspace by slug
pub async fn get_workspace(
    workspace_ctx: WorkspaceContext,
    Extension(workspace_service): Extension<WorkspaceService>,
) -> Result<Json<WorkspaceResponse>, (StatusCode, String)> {
    let workspace = workspace_service
        .get_workspace_by_slug(&workspace_ctx.workspace_slug)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch workspace: {}", e),
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "Workspace not found".to_string()))?;

    Ok(Json(WorkspaceResponse {
        id: workspace.id.to_string(),
        name: workspace.name,
        slug: workspace.slug,
        role: match workspace_ctx.user_role {
            WorkspaceRole::Admin => "admin".to_string(),
            WorkspaceRole::Member => "member".to_string(),
        },
        created_at: workspace.created_at.to_string(),
    }))
}

/// Create new workspace
pub async fn create_workspace(
    Extension(auth_user): Extension<AuthUser>,
    Extension(workspace_service): Extension<WorkspaceService>,
    Json(payload): Json<CreateWorkspaceRequest>,
) -> Result<Json<WorkspaceResponse>, (StatusCode, String)> {
    let workspace = workspace_service
        .create_workspace(payload.name, auth_user.user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Failed to create workspace: {}", e),
            )
        })?;

    Ok(Json(WorkspaceResponse {
        id: workspace.id.to_string(),
        name: workspace.name,
        slug: workspace.slug,
        role: "admin".to_string(),
        created_at: workspace.created_at.to_string(),
    }))
}

/// Update workspace (admin only)
pub async fn update_workspace(
    RequireAdmin(workspace_ctx): RequireAdmin,
    Extension(workspace_service): Extension<WorkspaceService>,
    Json(payload): Json<UpdateWorkspaceRequest>,
) -> Result<Json<WorkspaceResponse>, (StatusCode, String)> {
    // TODO: Implement update logic
    Err((
        StatusCode::NOT_IMPLEMENTED,
        "Update workspace not yet implemented".to_string(),
    ))
}

/// Delete workspace (admin only)
pub async fn delete_workspace(
    RequireAdmin(workspace_ctx): RequireAdmin,
    Extension(workspace_service): Extension<WorkspaceService>,
) -> Result<StatusCode, (StatusCode, String)> {
    // TODO: Implement delete logic
    Err((
        StatusCode::NOT_IMPLEMENTED,
        "Delete workspace not yet implemented".to_string(),
    ))
}
```

**Step 2: Add routes to router**

In the main router file (usually `crates/jility-server/src/main.rs` or router setup), add workspace routes:

```rust
use crate::api::workspaces;

// In router setup:
.route("/api/workspaces", get(workspaces::list_workspaces).post(workspaces::create_workspace))
.route("/api/w/:workspace_slug", get(workspaces::get_workspace))
.route("/api/w/:workspace_slug/settings", put(workspaces::update_workspace).delete(workspaces::delete_workspace))
```

**Step 3: Commit**

```bash
git add crates/jility-server/src/api/workspaces.rs
git commit -m "feat(api): add workspace CRUD endpoints with role-based access"
```

---

## Phase 4: Frontend Infrastructure

### Task 4.1: Create Workspace Context Provider

**Files:**
- Create: `jility-web/lib/workspace-context.tsx`

**Step 1: Create workspace context**

```typescript
'use client'

import { createContext, useContext, useState, useEffect, ReactNode } from 'react'
import { usePathname, useRouter } from 'next/navigation'

interface Workspace {
  id: string
  name: string
  slug: string
  role: 'admin' | 'member'
  createdAt: string
}

interface WorkspaceContextType {
  currentWorkspace: Workspace | null
  workspaces: Workspace[]
  isLoading: boolean
  switchWorkspace: (slug: string) => void
  refreshWorkspaces: () => Promise<void>
}

const WorkspaceContext = createContext<WorkspaceContextType | undefined>(undefined)

export function WorkspaceProvider({ children }: { children: ReactNode }) {
  const [workspaces, setWorkspaces] = useState<Workspace[]>([])
  const [currentWorkspace, setCurrentWorkspace] = useState<Workspace | null>(null)
  const [isLoading, setIsLoading] = useState(true)
  const pathname = usePathname()
  const router = useRouter()

  // Extract workspace slug from URL
  const workspaceSlug = pathname?.match(/^\/w\/([^\/]+)/)?.[1]

  // Fetch user's workspaces
  const fetchWorkspaces = async () => {
    try {
      const response = await fetch('/api/workspaces', {
        credentials: 'include',
      })

      if (!response.ok) {
        throw new Error('Failed to fetch workspaces')
      }

      const data = await response.json()
      setWorkspaces(data)

      // Set current workspace based on URL
      if (workspaceSlug) {
        const current = data.find((w: Workspace) => w.slug === workspaceSlug)
        setCurrentWorkspace(current || null)
      }
    } catch (error) {
      console.error('Failed to fetch workspaces:', error)
    } finally {
      setIsLoading(false)
    }
  }

  useEffect(() => {
    fetchWorkspaces()
  }, [workspaceSlug])

  const switchWorkspace = (slug: string) => {
    const workspace = workspaces.find(w => w.slug === slug)
    if (workspace) {
      setCurrentWorkspace(workspace)
      router.push(`/w/${slug}/board`)
    }
  }

  const refreshWorkspaces = async () => {
    setIsLoading(true)
    await fetchWorkspaces()
  }

  return (
    <WorkspaceContext.Provider
      value={{
        currentWorkspace,
        workspaces,
        isLoading,
        switchWorkspace,
        refreshWorkspaces,
      }}
    >
      {children}
    </WorkspaceContext.Provider>
  )
}

export function useWorkspace() {
  const context = useContext(WorkspaceContext)
  if (context === undefined) {
    throw new Error('useWorkspace must be used within a WorkspaceProvider')
  }
  return context
}
```

**Step 2: Commit**

```bash
git add jility-web/lib/workspace-context.tsx
git commit -m "feat(frontend): add workspace context provider"
```

---

### Task 4.2: Create Workspace Switcher Component

**Files:**
- Create: `jility-web/components/workspace/workspace-switcher.tsx`

**Step 1: Create workspace switcher**

```typescript
'use client'

import { useState } from 'react'
import { Check, ChevronsUpDown, Plus } from 'lucide-react'
import { useWorkspace } from '@/lib/workspace-context'
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
  const { currentWorkspace, workspaces, switchWorkspace, isLoading } = useWorkspace()
  const [isOpen, setIsOpen] = useState(false)

  if (isLoading) {
    return (
      <Button variant="outline" disabled className="w-48">
        Loading...
      </Button>
    )
  }

  return (
    <DropdownMenu open={isOpen} onOpenChange={setIsOpen}>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" role="combobox" aria-expanded={isOpen} className="w-48 justify-between">
          {currentWorkspace?.name || 'Select workspace'}
          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-48">
        <DropdownMenuLabel>Workspaces</DropdownMenuLabel>
        <DropdownMenuSeparator />
        {workspaces.map((workspace) => (
          <DropdownMenuItem
            key={workspace.id}
            onSelect={() => {
              switchWorkspace(workspace.slug)
              setIsOpen(false)
            }}
          >
            <div className="flex items-center justify-between w-full">
              <span>{workspace.name}</span>
              {currentWorkspace?.id === workspace.id && <Check className="h-4 w-4" />}
            </div>
            {workspace.role === 'admin' && (
              <span className="ml-2 text-xs text-muted-foreground">(Admin)</span>
            )}
          </DropdownMenuItem>
        ))}
        <DropdownMenuSeparator />
        <DropdownMenuItem>
          <Plus className="mr-2 h-4 w-4" />
          <span>Create Workspace</span>
        </DropdownMenuItem>
        <DropdownMenuSeparator />
        <div className="px-2 py-1.5 text-xs text-muted-foreground">
          {workspaces.length} {workspaces.length === 1 ? 'workspace' : 'workspaces'}
        </div>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
```

**Step 2: Commit**

```bash
git add jility-web/components/workspace/
git commit -m "feat(frontend): add workspace switcher dropdown component"
```

---

### Task 4.3: Update Root Layout with Workspace Provider

**Files:**
- Modify: `jility-web/app/layout.tsx`

**Step 1: Add workspace provider to root layout**

Edit `jility-web/app/layout.tsx`, wrap children with WorkspaceProvider:

```typescript
import { WorkspaceProvider } from '@/lib/workspace-context'

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>
        <WorkspaceProvider>
          {children}
        </WorkspaceProvider>
      </body>
    </html>
  )
}
```

**Step 2: Commit**

```bash
git add jility-web/app/layout.tsx
git commit -m "feat(frontend): add workspace provider to root layout"
```

---

## Phase 5: Workspace-Scoped Routes

### Task 5.1: Create Workspace Layout

**Files:**
- Create: `jility-web/app/w/[workspaceSlug]/layout.tsx`

**Step 1: Create workspace layout with switcher**

```typescript
'use client'

import { useEffect } from 'react'
import { useRouter } from 'next/navigation'
import { useWorkspace } from '@/lib/workspace-context'
import { WorkspaceSwitcher } from '@/components/workspace/workspace-switcher'
import { useAuth } from '@/lib/auth-context'

export default function WorkspaceLayout({
  children,
  params,
}: {
  children: React.ReactNode
  params: { workspaceSlug: string }
}) {
  const { isAuthenticated, isLoading: authLoading } = useAuth()
  const { currentWorkspace, isLoading: workspaceLoading } = useWorkspace()
  const router = useRouter()

  useEffect(() => {
    if (!authLoading && !isAuthenticated) {
      router.push('/login')
    }
  }, [authLoading, isAuthenticated, router])

  useEffect(() => {
    if (!workspaceLoading && !currentWorkspace && !authLoading) {
      // Workspace not found or user not member
      router.push('/login')
    }
  }, [workspaceLoading, currentWorkspace, authLoading, router])

  if (authLoading || workspaceLoading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto"></div>
          <p className="mt-4 text-muted-foreground">Loading workspace...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen">
      <nav className="border-b bg-background">
        <div className="container mx-auto px-4 py-3 flex items-center justify-between">
          <div className="flex items-center gap-4">
            <h1 className="text-xl font-bold">Jility</h1>
            <WorkspaceSwitcher />
          </div>
          {/* Add other nav items here */}
        </div>
      </nav>
      <main>{children}</main>
    </div>
  )
}
```

**Step 2: Commit**

```bash
git add jility-web/app/w/
git commit -m "feat(frontend): add workspace layout with switcher in navbar"
```

---

### Task 5.2: Create Workspace Board Page

**Files:**
- Create: `jility-web/app/w/[workspaceSlug]/board/page.tsx`

**Step 1: Create workspace-scoped board page**

```typescript
'use client'

import { useWorkspace } from '@/lib/workspace-context'
import { KanbanBoard } from '@/components/kanban/board'

export default function WorkspaceBoardPage() {
  const { currentWorkspace, isLoading } = useWorkspace()

  if (isLoading) {
    return <div>Loading...</div>
  }

  if (!currentWorkspace) {
    return <div>Workspace not found</div>
  }

  return (
    <div className="h-full">
      <div className="container mx-auto px-4 py-6">
        <h2 className="text-2xl font-bold mb-4">{currentWorkspace.name}</h2>
        <KanbanBoard workspaceId={currentWorkspace.id} />
      </div>
    </div>
  )
}
```

**Step 2: Update KanbanBoard component to accept workspaceId**

Edit `jility-web/components/kanban/board.tsx` to filter tickets by workspace:

```typescript
interface KanbanBoardProps {
  workspaceId: string
}

export function KanbanBoard({ workspaceId }: KanbanBoardProps) {
  // Update fetch URL to include workspace filtering
  // const url = `/api/w/${workspaceSlug}/tickets`
  // ...existing code...
}
```

**Step 3: Commit**

```bash
git add jility-web/app/w/ jility-web/components/kanban/
git commit -m "feat(frontend): add workspace-scoped board page"
```

---

## Phase 6: Member Management

### Task 6.1: Create Member Service

**Files:**
- Create: `crates/jility-server/src/services/member.rs`
- Modify: `crates/jility-server/src/services/mod.rs`

**Step 1: Create member management service**

```rust
use anyhow::{anyhow, Result};
use jility_core::entities::{
    workspace_invite, workspace_member, WorkspaceInvite, WorkspaceMember, WorkspaceRole,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

pub struct MemberService {
    db: DatabaseConnection,
}

impl MemberService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Get workspace members
    pub async fn get_workspace_members(
        &self,
        workspace_id: Uuid,
    ) -> Result<Vec<workspace_member::Model>> {
        let members = WorkspaceMember::find()
            .filter(workspace_member::Column::WorkspaceId.eq(workspace_id))
            .all(&self.db)
            .await?;

        Ok(members)
    }

    /// Remove member from workspace
    pub async fn remove_member(&self, workspace_id: Uuid, user_id: Uuid) -> Result<()> {
        // Check if this is the last admin
        let admin_count = WorkspaceMember::find()
            .filter(workspace_member::Column::WorkspaceId.eq(workspace_id))
            .filter(workspace_member::Column::Role.eq(WorkspaceRole::Admin))
            .count(&self.db)
            .await?;

        let member = WorkspaceMember::find()
            .filter(workspace_member::Column::WorkspaceId.eq(workspace_id))
            .filter(workspace_member::Column::UserId.eq(user_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow!("Member not found"))?;

        if member.role == WorkspaceRole::Admin && admin_count <= 1 {
            return Err(anyhow!(
                "Cannot remove the last admin. Promote another member first."
            ));
        }

        // Delete member
        WorkspaceMember::delete_many()
            .filter(workspace_member::Column::WorkspaceId.eq(workspace_id))
            .filter(workspace_member::Column::UserId.eq(user_id))
            .exec(&self.db)
            .await?;

        Ok(())
    }

    /// Change member role
    pub async fn change_role(
        &self,
        workspace_id: Uuid,
        user_id: Uuid,
        new_role: WorkspaceRole,
    ) -> Result<()> {
        // If demoting to member, check if this is the last admin
        if new_role == WorkspaceRole::Member {
            let admin_count = WorkspaceMember::find()
                .filter(workspace_member::Column::WorkspaceId.eq(workspace_id))
                .filter(workspace_member::Column::Role.eq(WorkspaceRole::Admin))
                .count(&self.db)
                .await?;

            let current_member = WorkspaceMember::find()
                .filter(workspace_member::Column::WorkspaceId.eq(workspace_id))
                .filter(workspace_member::Column::UserId.eq(user_id))
                .one(&self.db)
                .await?
                .ok_or_else(|| anyhow!("Member not found"))?;

            if current_member.role == WorkspaceRole::Admin && admin_count <= 1 {
                return Err(anyhow!(
                    "Cannot demote the last admin. Promote another member first."
                ));
            }
        }

        // Update role
        let member = WorkspaceMember::find()
            .filter(workspace_member::Column::WorkspaceId.eq(workspace_id))
            .filter(workspace_member::Column::UserId.eq(user_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow!("Member not found"))?;

        let mut member: workspace_member::ActiveModel = member.into();
        member.role = Set(new_role);
        member.update(&self.db).await?;

        Ok(())
    }

    /// Create invite
    pub async fn create_invite(
        &self,
        workspace_id: Uuid,
        email: String,
        role: WorkspaceRole,
        invited_by_user_id: Uuid,
    ) -> Result<workspace_invite::Model> {
        // Check if invite already exists
        let existing = WorkspaceInvite::find()
            .filter(workspace_invite::Column::WorkspaceId.eq(workspace_id))
            .filter(workspace_invite::Column::Email.eq(&email))
            .filter(workspace_invite::Column::AcceptedAt.is_null())
            .one(&self.db)
            .await?;

        if existing.is_some() {
            return Err(anyhow!("Invite already exists for this email"));
        }

        let now = chrono::Utc::now().fixed_offset();
        let expires_at = now + chrono::Duration::days(7);

        let invite = workspace_invite::ActiveModel {
            id: Set(Uuid::new_v4()),
            workspace_id: Set(workspace_id),
            email: Set(email),
            role: Set(role),
            invited_by_user_id: Set(invited_by_user_id),
            token: Set(Uuid::new_v4().to_string()),
            expires_at: Set(expires_at),
            accepted_at: Set(None),
            created_at: Set(now),
        };

        let invite = invite.insert(&self.db).await?;
        Ok(invite)
    }

    /// Get pending invites
    pub async fn get_pending_invites(
        &self,
        workspace_id: Uuid,
    ) -> Result<Vec<workspace_invite::Model>> {
        let invites = WorkspaceInvite::find()
            .filter(workspace_invite::Column::WorkspaceId.eq(workspace_id))
            .filter(workspace_invite::Column::AcceptedAt.is_null())
            .all(&self.db)
            .await?;

        Ok(invites)
    }

    /// Accept invite
    pub async fn accept_invite(&self, token: &str, user_id: Uuid) -> Result<Uuid> {
        let invite = WorkspaceInvite::find()
            .filter(workspace_invite::Column::Token.eq(token))
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow!("Invite not found"))?;

        // Check if expired
        if invite.expires_at < chrono::Utc::now().fixed_offset() {
            return Err(anyhow!("Invite has expired"));
        }

        // Check if already accepted
        if invite.accepted_at.is_some() {
            return Err(anyhow!("Invite has already been accepted"));
        }

        // Check if user is already a member
        let existing_member = WorkspaceMember::find()
            .filter(workspace_member::Column::WorkspaceId.eq(invite.workspace_id))
            .filter(workspace_member::Column::UserId.eq(user_id))
            .one(&self.db)
            .await?;

        if existing_member.is_some() {
            return Err(anyhow!("You are already a member of this workspace"));
        }

        let now = chrono::Utc::now().fixed_offset();

        // Create member
        let member = workspace_member::ActiveModel {
            id: Set(Uuid::new_v4()),
            workspace_id: Set(invite.workspace_id),
            user_id: Set(user_id),
            role: Set(invite.role.clone()),
            invited_by_user_id: Set(Some(invite.invited_by_user_id)),
            invited_at: Set(Some(invite.created_at)),
            joined_at: Set(now),
        };

        member.insert(&self.db).await?;

        // Mark invite as accepted
        let mut invite_active: workspace_invite::ActiveModel = invite.clone().into();
        invite_active.accepted_at = Set(Some(now));
        invite_active.update(&self.db).await?;

        Ok(invite.workspace_id)
    }
}
```

**Step 2: Add to services mod**

Edit `crates/jility-server/src/services/mod.rs`:
```rust
pub mod member;
pub use member::MemberService;
```

**Step 3: Commit**

```bash
git add crates/jility-server/src/services/member.rs crates/jility-server/src/services/mod.rs
git commit -m "feat(server): add member management service with invite system"
```

---

## Implementation Notes

**Total Estimated Time:** 15-20 hours for complete implementation

**Phases:**
- Phase 1 (DB): 2-3 hours
- Phase 2 (Backend Services): 2-3 hours
- Phase 3 (API Endpoints): 2-3 hours
- Phase 4 (Frontend Infrastructure): 2-3 hours
- Phase 5 (Workspace Routes): 3-4 hours
- Phase 6 (Member Management): 3-4 hours

**Testing Strategy:**
- Unit tests for slug generation
- Integration tests for workspace service
- E2E tests for workspace switching
- Manual testing for invite flow

**Deployment Checklist:**
- ✅ Run migrations on production database
- ✅ Update environment variables
- ✅ Test workspace creation flow
- ✅ Test invite email delivery
- ✅ Verify role-based permissions
- ✅ Test workspace switching UX

**Follow-up Tasks (Not in This Plan):**
- Email service for invite notifications
- Workspace settings page UI
- Member management page UI
- Billing integration
- Workspace analytics
- Workspace transfer/ownership change
