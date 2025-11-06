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

/// Invite member to workspace
pub async fn invite_member(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(workspace_slug): Path<String>,
    Json(payload): Json<InviteMemberRequest>,
) -> ApiResult<StatusCode> {
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
    member_service
        .create_invite(workspace.id, payload.email, invite_role, auth_user.id)
        .await
        .map_err(|e| ApiError::BadRequest(format!("Failed to create invite: {}", e)))?;

    Ok(StatusCode::CREATED)
}
