use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::services::{MemberService, WorkspaceService};
use crate::state::AppState;
use jility_core::entities::{WorkspaceRole};

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
pub struct InviteMemberRequest {
    pub email: String,
    pub role: String,
}

/// Get user's workspaces
pub async fn list_workspaces(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> ApiResult<Json<Vec<WorkspaceResponse>>> {
    let workspace_service = WorkspaceService::new(state.db.as_ref().clone());

    let workspaces = workspace_service
        .get_user_workspaces(auth_user.id)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch workspaces: {}", e)))?;

    // Get role for each workspace
    let mut responses = Vec::new();
    for workspace in workspaces {
        let role = workspace_service
            .get_user_role(workspace.id, auth_user.id)
            .await
            .map_err(|e| ApiError::Internal(format!("Failed to fetch role: {}", e)))?
            .unwrap_or(WorkspaceRole::Member);

        responses.push(WorkspaceResponse {
            id: workspace.id.to_string(),
            name: workspace.name,
            slug: workspace.slug,
            role: match role {
                WorkspaceRole::Admin => "admin".to_string(),
                WorkspaceRole::Member => "member".to_string(),
            },
            created_at: workspace.created_at.to_rfc3339(),
        });
    }

    Ok(Json(responses))
}

/// Get workspace by slug
pub async fn get_workspace(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(workspace_slug): Path<String>,
) -> ApiResult<Json<WorkspaceResponse>> {
    let workspace_service = WorkspaceService::new(state.db.as_ref().clone());

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

    // Get user's role
    let role = workspace_service
        .get_user_role(workspace.id, auth_user.id)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch role: {}", e)))?
        .unwrap_or(WorkspaceRole::Member);

    Ok(Json(WorkspaceResponse {
        id: workspace.id.to_string(),
        name: workspace.name,
        slug: workspace.slug,
        role: match role {
            WorkspaceRole::Admin => "admin".to_string(),
            WorkspaceRole::Member => "member".to_string(),
        },
        created_at: workspace.created_at.to_rfc3339(),
    }))
}

/// Create new workspace
pub async fn create_workspace(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(payload): Json<CreateWorkspaceRequest>,
) -> ApiResult<Json<WorkspaceResponse>> {
    let workspace_service = WorkspaceService::new(state.db.as_ref().clone());

    let workspace = workspace_service
        .create_workspace(payload.name, auth_user.id)
        .await
        .map_err(|e| ApiError::BadRequest(format!("Failed to create workspace: {}", e)))?;

    Ok(Json(WorkspaceResponse {
        id: workspace.id.to_string(),
        name: workspace.name,
        slug: workspace.slug,
        role: "admin".to_string(),
        created_at: workspace.created_at.to_rfc3339(),
    }))
}

#[derive(Serialize)]
pub struct InviteResponse {
    pub invite_id: String,
    pub email: String,
    pub role: String,
    pub token: String,
    pub invite_url: String,
    pub expires_at: String,
}

/// Invite member to workspace
pub async fn invite_member(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(workspace_slug): Path<String>,
    Json(payload): Json<InviteMemberRequest>,
) -> ApiResult<Json<InviteResponse>> {
    let workspace_service = WorkspaceService::new(state.db.as_ref().clone());
    let member_service = MemberService::new(state.db.as_ref().clone());

    // Get workspace
    let workspace = workspace_service
        .get_workspace_by_slug(&workspace_slug)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch workspace: {}", e)))?
        .ok_or_else(|| ApiError::NotFound("Workspace not found".to_string()))?;

    // Check if user is admin
    let role = workspace_service
        .get_user_role(workspace.id, auth_user.id)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch role: {}", e)))?
        .ok_or_else(|| ApiError::Unauthorized("Not a member of this workspace".to_string()))?;

    if role != WorkspaceRole::Admin {
        return Err(ApiError::Unauthorized(
            "Only admins can invite members".to_string(),
        ));
    }

    // Parse role
    let invite_role = match payload.role.as_str() {
        "admin" => WorkspaceRole::Admin,
        "member" => WorkspaceRole::Member,
        _ => return Err(ApiError::BadRequest("Invalid role".to_string())),
    };

    // Create invite
    let invite = member_service
        .create_invite(workspace.id, payload.email, invite_role, auth_user.id)
        .await
        .map_err(|e| ApiError::BadRequest(format!("Failed to create invite: {}", e)))?;

    let invite_url = format!("http://localhost:3901/invite/{}", invite.token);

    Ok(Json(InviteResponse {
        invite_id: invite.id.to_string(),
        email: invite.email,
        role: match invite.role {
            WorkspaceRole::Admin => "admin".to_string(),
            WorkspaceRole::Member => "member".to_string(),
        },
        token: invite.token,
        invite_url,
        expires_at: invite.expires_at.to_rfc3339(),
    }))
}

#[derive(Serialize)]
pub struct PendingInviteResponse {
    pub invite_id: String,
    pub email: String,
    pub role: String,
    pub token: String,
    pub invite_url: String,
    pub invited_at: String,
    pub expires_at: String,
}

/// List pending invites for workspace
pub async fn list_pending_invites(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(workspace_slug): Path<String>,
) -> ApiResult<Json<Vec<PendingInviteResponse>>> {
    let workspace_service = WorkspaceService::new(state.db.as_ref().clone());
    let member_service = MemberService::new(state.db.as_ref().clone());

    // Get workspace
    let workspace = workspace_service
        .get_workspace_by_slug(&workspace_slug)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch workspace: {}", e)))?
        .ok_or_else(|| ApiError::NotFound("Workspace not found".to_string()))?;

    // Check if user is admin
    let role = workspace_service
        .get_user_role(workspace.id, auth_user.id)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch role: {}", e)))?
        .ok_or_else(|| ApiError::Unauthorized("Not a member".to_string()))?;

    if role != WorkspaceRole::Admin {
        return Err(ApiError::Unauthorized(
            "Only admins can view pending invites".to_string(),
        ));
    }

    // Get pending invites
    let invites = member_service
        .get_pending_invites(workspace.id)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch invites: {}", e)))?;

    let responses = invites
        .into_iter()
        .map(|invite| PendingInviteResponse {
            invite_id: invite.id.to_string(),
            email: invite.email,
            role: match invite.role {
                WorkspaceRole::Admin => "admin".to_string(),
                WorkspaceRole::Member => "member".to_string(),
            },
            token: invite.token.clone(),
            invite_url: format!("http://localhost:3901/invite/{}", invite.token),
            invited_at: invite.created_at.to_rfc3339(),
            expires_at: invite.expires_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(responses))
}

#[derive(Serialize)]
pub struct InviteDetailsResponse {
    pub workspace_name: String,
    pub workspace_slug: String,
    pub invited_by_email: String,
    pub role: String,
    pub expires_at: String,
    pub is_expired: bool,
}

/// Get invite details (public endpoint)
pub async fn get_invite_details(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> ApiResult<Json<InviteDetailsResponse>> {
    use jility_core::entities::user;

    let member_service = MemberService::new(state.db.as_ref().clone());
    let workspace_service = WorkspaceService::new(state.db.as_ref().clone());

    // Get invite
    let invite = member_service
        .get_invite_by_token(&token)
        .await
        .map_err(|_| ApiError::NotFound("Invite not found".to_string()))?;

    // Check if expired
    let is_expired = invite.expires_at < chrono::Utc::now().fixed_offset();

    // Get workspace
    let workspace = workspace_service
        .get_workspace_by_id(invite.workspace_id)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch workspace: {}", e)))?
        .ok_or_else(|| ApiError::NotFound("Workspace not found".to_string()))?;

    // Get inviter email
    use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
    let inviter = user::Entity::find()
        .filter(user::Column::Id.eq(invite.invited_by_user_id))
        .one(state.db.as_ref())
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch inviter: {}", e)))?
        .ok_or_else(|| ApiError::Internal("Inviter not found".to_string()))?;

    Ok(Json(InviteDetailsResponse {
        workspace_name: workspace.name,
        workspace_slug: workspace.slug,
        invited_by_email: inviter.email,
        role: match invite.role {
            WorkspaceRole::Admin => "admin".to_string(),
            WorkspaceRole::Member => "member".to_string(),
        },
        expires_at: invite.expires_at.to_rfc3339(),
        is_expired,
    }))
}

/// Accept workspace invite
pub async fn accept_invite(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(token): Path<String>,
) -> ApiResult<Json<WorkspaceResponse>> {
    let member_service = MemberService::new(state.db.as_ref().clone());
    let workspace_service = WorkspaceService::new(state.db.as_ref().clone());

    // Accept invite (validates token, expiry, etc.)
    let workspace_id = member_service
        .accept_invite(&token, auth_user.id)
        .await
        .map_err(|e| ApiError::BadRequest(format!("Failed to accept invite: {}", e)))?;

    // Get workspace details
    let workspace = workspace_service
        .get_workspace_by_id(workspace_id)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch workspace: {}", e)))?
        .ok_or_else(|| ApiError::Internal("Workspace not found after accept".to_string()))?;

    // Get user's role in the workspace
    let role = workspace_service
        .get_user_role(workspace_id, auth_user.id)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch role: {}", e)))?
        .unwrap_or(WorkspaceRole::Member);

    Ok(Json(WorkspaceResponse {
        id: workspace.id.to_string(),
        name: workspace.name,
        slug: workspace.slug,
        role: match role {
            WorkspaceRole::Admin => "admin".to_string(),
            WorkspaceRole::Member => "member".to_string(),
        },
        created_at: workspace.created_at.to_rfc3339(),
    }))
}

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
