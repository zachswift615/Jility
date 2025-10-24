use axum::{extract::State, extract::Path, Json};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use uuid::Uuid;
use chrono::Utc;

use crate::{
    error::{ApiError, ApiResult},
    models::{CreateProjectRequest, ProjectResponse},
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
        created_at: project.created_at.to_rfc3339(),
        updated_at: project.updated_at.to_rfc3339(),
    }))
}
