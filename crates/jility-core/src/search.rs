use crate::entities::{ticket, Ticket};
use crate::error::CoreResult;
use chrono::{DateTime, Utc};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    pub query: String,
    pub status: Option<Vec<String>>,
    pub assignees: Option<Vec<String>>,
    pub labels: Option<Vec<String>>,
    pub created_by: Option<String>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub updated_after: Option<DateTime<Utc>>,
    pub updated_before: Option<DateTime<Utc>>,
    pub min_points: Option<i32>,
    pub max_points: Option<i32>,
    pub has_comments: Option<bool>,
    pub has_commits: Option<bool>,
    pub has_dependencies: Option<bool>,
    pub epic_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub search_in: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub tickets: Vec<TicketSearchResult>,
    pub total: u64,
    pub limit: u64,
    pub offset: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketSearchResult {
    pub id: Uuid,
    pub ticket_number: i32,
    pub title: String,
    pub description: String,
    pub status: String,
    pub story_points: Option<i32>,
    pub project_id: Uuid,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub epic_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
}

pub struct SearchService {
    db: Arc<DatabaseConnection>,
}

impl SearchService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn search_tickets(
        &self,
        filters: SearchFilters,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> CoreResult<SearchResponse> {
        let limit = limit.unwrap_or(50).min(100);
        let offset = offset.unwrap_or(0);

        // Start building the query
        let mut query = Ticket::find();

        // Apply project filter if provided
        if let Some(project_id) = filters.project_id {
            query = query.filter(ticket::Column::ProjectId.eq(project_id));
        }

        // Apply status filter
        if let Some(statuses) = filters.status {
            if !statuses.is_empty() {
                query = query.filter(ticket::Column::Status.is_in(statuses));
            }
        }

        // Apply created_by filter
        if let Some(created_by) = filters.created_by {
            query = query.filter(ticket::Column::CreatedBy.eq(created_by));
        }

        // Apply date filters
        if let Some(created_after) = filters.created_after {
            query = query.filter(ticket::Column::CreatedAt.gte(created_after));
        }
        if let Some(created_before) = filters.created_before {
            query = query.filter(ticket::Column::CreatedAt.lte(created_before));
        }
        if let Some(updated_after) = filters.updated_after {
            query = query.filter(ticket::Column::UpdatedAt.gte(updated_after));
        }
        if let Some(updated_before) = filters.updated_before {
            query = query.filter(ticket::Column::UpdatedAt.lte(updated_before));
        }

        // Apply story points filters
        if let Some(min_points) = filters.min_points {
            query = query.filter(ticket::Column::StoryPoints.gte(min_points));
        }
        if let Some(max_points) = filters.max_points {
            query = query.filter(ticket::Column::StoryPoints.lte(max_points));
        }

        // Apply parent/epic filters
        if let Some(parent_id) = filters.parent_id {
            query = query.filter(ticket::Column::ParentId.eq(parent_id));
        }
        if let Some(epic_id) = filters.epic_id {
            query = query.filter(ticket::Column::EpicId.eq(epic_id));
        }

        // Apply text search if provided
        if !filters.query.is_empty() {
            let search_pattern = format!("%{}%", filters.query);
            query = query.filter(
                ticket::Column::Title
                    .contains(&search_pattern)
                    .or(ticket::Column::Description.contains(&search_pattern)),
            );
        }

        // Order by updated_at descending
        query = query.order_by_desc(ticket::Column::UpdatedAt);

        // Get total count before pagination
        let total = query.clone().count(self.db.as_ref()).await?;

        // Apply pagination
        let tickets = query
            .limit(limit)
            .offset(offset)
            .all(self.db.as_ref())
            .await?;

        // Convert to search results
        let results: Vec<TicketSearchResult> = tickets
            .into_iter()
            .map(|t| TicketSearchResult {
                id: t.id,
                ticket_number: t.ticket_number,
                title: t.title,
                description: t.description,
                status: t.status,
                story_points: t.story_points,
                project_id: t.project_id,
                created_by: t.created_by,
                created_at: t.created_at.to_utc(),
                updated_at: t.updated_at.to_utc(),
                epic_id: t.epic_id,
                parent_id: t.parent_id,
            })
            .collect();

        Ok(SearchResponse {
            tickets: results,
            total,
            limit,
            offset,
        })
    }
}
