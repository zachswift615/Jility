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
