use axum::{extract::State, extract::Path, Json};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use uuid::Uuid;
use chrono::Utc;

use crate::{
    error::{ApiError, ApiResult},
    models::{CreateProjectRequest, UpdateProjectRequest, ProjectResponse},
    state::AppState,
};
use jility_core::entities::{project, Project};

pub async fn list_projects(State(state): State<AppState>) -> ApiResult<Json<Vec<ProjectResponse>>> {
    let projects = Project::find()
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    let response = projects
        .into_iter()
        .map(|p| ProjectResponse {
            id: p.id.to_string(),
            name: p.name,
            description: p.description,
            key: p.key,
            color: p.color,
            ai_planning_enabled: p.ai_planning_enabled,
            auto_link_git: p.auto_link_git,
            require_story_points: p.require_story_points,
            created_at: p.created_at.to_rfc3339(),
            updated_at: p.updated_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(response))
}

pub async fn create_project(
    State(state): State<AppState>,
    Json(payload): Json<CreateProjectRequest>,
) -> ApiResult<Json<ProjectResponse>> {
    let now = Utc::now();
    let project_id = Uuid::new_v4();

    let project = project::ActiveModel {
        id: Set(project_id),
        name: Set(payload.name),
        description: Set(payload.description),
        key: Set(payload.key),
        color: Set(payload.color.or(Some("#5e6ad2".to_string()))),
        ai_planning_enabled: Set(payload.ai_planning_enabled),
        auto_link_git: Set(payload.auto_link_git),
        require_story_points: Set(payload.require_story_points),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let result = project
        .insert(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    Ok(Json(ProjectResponse {
        id: result.id.to_string(),
        name: result.name,
        description: result.description,
        key: result.key,
        color: result.color,
        ai_planning_enabled: result.ai_planning_enabled,
        auto_link_git: result.auto_link_git,
        require_story_points: result.require_story_points,
        created_at: result.created_at.to_rfc3339(),
        updated_at: result.updated_at.to_rfc3339(),
    }))
}

pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<ProjectResponse>> {
    let project_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid project ID: {}", id)))?;

    let project = Project::find_by_id(project_id)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Project not found: {}", id)))?;

    Ok(Json(ProjectResponse {
        id: project.id.to_string(),
        name: project.name,
        description: project.description,
        key: project.key,
        color: project.color,
        ai_planning_enabled: project.ai_planning_enabled,
        auto_link_git: project.auto_link_git,
        require_story_points: project.require_story_points,
        created_at: project.created_at.to_rfc3339(),
        updated_at: project.updated_at.to_rfc3339(),
    }))
}

pub async fn update_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProjectRequest>,
) -> ApiResult<Json<ProjectResponse>> {
    let project_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid project ID: {}", id)))?;

    let existing_project = Project::find_by_id(project_id)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Project not found: {}", id)))?;

    let mut active_model: project::ActiveModel = existing_project.into();

    if let Some(name) = payload.name {
        active_model.name = Set(name);
    }
    if payload.description.is_some() {
        active_model.description = Set(payload.description);
    }
    if payload.key.is_some() {
        active_model.key = Set(payload.key);
    }
    if payload.color.is_some() {
        active_model.color = Set(payload.color);
    }
    if let Some(ai_planning) = payload.ai_planning_enabled {
        active_model.ai_planning_enabled = Set(ai_planning);
    }
    if let Some(auto_link) = payload.auto_link_git {
        active_model.auto_link_git = Set(auto_link);
    }
    if let Some(require_points) = payload.require_story_points {
        active_model.require_story_points = Set(require_points);
    }
    active_model.updated_at = Set(Utc::now());

    let result = active_model
        .update(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    Ok(Json(ProjectResponse {
        id: result.id.to_string(),
        name: result.name,
        description: result.description,
        key: result.key,
        color: result.color,
        ai_planning_enabled: result.ai_planning_enabled,
        auto_link_git: result.auto_link_git,
        require_story_points: result.require_story_points,
        created_at: result.created_at.to_rfc3339(),
        updated_at: result.updated_at.to_rfc3339(),
    }))
}

pub async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let project_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid project ID: {}", id)))?;

    let result = Project::delete_by_id(project_id)
        .exec(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    if result.rows_affected == 0 {
        return Err(ApiError::NotFound(format!("Project not found: {}", id)));
    }

    Ok(Json(serde_json::json!({ "success": true })))
}
