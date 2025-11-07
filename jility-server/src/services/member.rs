use anyhow::{anyhow, Result};
use jility_core::entities::{
    user, workspace_invite, workspace_member, WorkspaceInvite, WorkspaceMember, WorkspaceRole,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::api::workspaces::WorkspaceMemberResponse;

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

    /// List workspace members with user details
    pub async fn list_workspace_members(
        &self,
        workspace_id: Uuid,
    ) -> Result<Vec<WorkspaceMemberResponse>> {
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
                        WorkspaceRole::Admin => "admin".to_string(),
                        WorkspaceRole::Member => "member".to_string(),
                    },
                    joined_at: member.joined_at.to_rfc3339(),
                })
            })
            .collect();

        Ok(responses)
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
