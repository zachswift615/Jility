use crate::error::JilityError;
use crate::search::types::{SearchFilters, SearchResponse, SearchResult};
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, FromQueryResult, Statement};
use std::sync::Arc;

pub struct SearchService {
    db: Arc<DatabaseConnection>,
}

impl SearchService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// Search tickets with full-text search and filters
    pub async fn search_tickets(
        &self,
        filters: SearchFilters,
        limit: u64,
        offset: u64,
    ) -> Result<SearchResponse, JilityError> {
        let backend = self.db.get_database_backend();

        let results = match backend {
            DatabaseBackend::Sqlite => self.search_sqlite(&filters, limit, offset).await?,
            DatabaseBackend::Postgres => self.search_postgres(&filters, limit, offset).await?,
            _ => {
                return Err(JilityError::DatabaseError(
                    "Unsupported database backend".to_string(),
                ))
            }
        };

        let total = results.len();
        let has_more = total == limit as usize;

        Ok(SearchResponse {
            results,
            total,
            has_more,
            offset,
            limit,
        })
    }

    /// SQLite FTS5 implementation
    async fn search_sqlite(
        &self,
        filters: &SearchFilters,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<SearchResult>, JilityError> {
        // Build WHERE clause for metadata filters
        let mut where_clauses = Vec::new();
        let mut params: Vec<sea_orm::Value> = Vec::new();
        let mut param_idx = 1;

        // Add project filter if specified
        if let Some(project_id) = filters.project_id {
            where_clauses.push(format!("t.project_id = ?{}", param_idx));
            params.push(project_id.to_string().into());
            param_idx += 1;
        }

        // Add status filter
        if let Some(ref statuses) = filters.status {
            if !statuses.is_empty() {
                let placeholders: Vec<String> = (0..statuses.len())
                    .map(|i| format!("?{}", param_idx + i))
                    .collect();
                where_clauses.push(format!("t.status IN ({})", placeholders.join(", ")));
                for status in statuses {
                    params.push(status.clone().into());
                    param_idx += 1;
                }
            }
        }

        // Add created_by filter
        if let Some(ref created_by) = filters.created_by {
            where_clauses.push(format!("t.created_by = ?{}", param_idx));
            params.push(created_by.clone().into());
            param_idx += 1;
        }

        // Add date filters
        if let Some(created_after) = filters.created_after {
            where_clauses.push(format!("t.created_at >= ?{}", param_idx));
            params.push(created_after.to_rfc3339().into());
            param_idx += 1;
        }

        if let Some(created_before) = filters.created_before {
            where_clauses.push(format!("t.created_at <= ?{}", param_idx));
            params.push(created_before.to_rfc3339().into());
            param_idx += 1;
        }

        if let Some(updated_after) = filters.updated_after {
            where_clauses.push(format!("t.updated_at >= ?{}", param_idx));
            params.push(updated_after.to_rfc3339().into());
            param_idx += 1;
        }

        if let Some(updated_before) = filters.updated_before {
            where_clauses.push(format!("t.updated_at <= ?{}", param_idx));
            params.push(updated_before.to_rfc3339().into());
            param_idx += 1;
        }

        // Add story points filters
        if let Some(min_points) = filters.min_points {
            where_clauses.push(format!("t.story_points >= ?{}", param_idx));
            params.push(min_points.into());
            param_idx += 1;
        }

        if let Some(max_points) = filters.max_points {
            where_clauses.push(format!("t.story_points <= ?{}", param_idx));
            params.push(max_points.into());
            param_idx += 1;
        }

        // Add epic/parent filters
        if let Some(epic_id) = filters.epic_id {
            where_clauses.push(format!("t.epic_id = ?{}", param_idx));
            params.push(epic_id.to_string().into());
            param_idx += 1;
        }

        if let Some(parent_id) = filters.parent_id {
            where_clauses.push(format!("t.parent_id = ?{}", param_idx));
            params.push(parent_id.to_string().into());
            param_idx += 1;
        }

        // Build metadata WHERE clause
        let metadata_where = if where_clauses.is_empty() {
            String::new()
        } else {
            format!("AND {}", where_clauses.join(" AND "))
        };

        // Prepare FTS query (escape special characters)
        let fts_query = self.escape_fts_query(&filters.query);

        // Build the main query
        let sql = format!(
            r#"
            WITH ticket_matches AS (
                SELECT
                    fts.ticket_id,
                    snippet(tickets_fts, 2, '<mark>', '</mark>', '...', 50) as snippet,
                    bm25(tickets_fts) as rank,
                    'title,description' as matched_in
                FROM tickets_fts fts
                WHERE tickets_fts MATCH ?{}
            ),
            comment_matches AS (
                SELECT
                    fts.ticket_id,
                    snippet(comments_fts, 3, '<mark>', '</mark>', '...', 50) as snippet,
                    bm25(comments_fts) as rank,
                    'comments' as matched_in
                FROM comments_fts fts
                WHERE comments_fts MATCH ?{}
            ),
            all_matches AS (
                SELECT * FROM ticket_matches
                UNION ALL
                SELECT * FROM comment_matches
            ),
            ranked_tickets AS (
                SELECT
                    am.ticket_id,
                    am.snippet,
                    MIN(am.rank) as best_rank,
                    GROUP_CONCAT(DISTINCT am.matched_in) as matched_in
                FROM all_matches am
                GROUP BY am.ticket_id
            )
            SELECT
                t.id as ticket_id,
                t.ticket_number,
                t.title,
                t.description,
                t.status,
                t.story_points,
                rt.snippet,
                rt.best_rank as rank,
                rt.matched_in,
                t.created_by,
                t.created_at,
                t.updated_at,
                t.parent_id,
                t.epic_id
            FROM ranked_tickets rt
            INNER JOIN tickets t ON t.id = rt.ticket_id
            WHERE 1=1 {}
            ORDER BY rt.best_rank
            LIMIT ?{} OFFSET ?{}
            "#,
            param_idx,
            param_idx + 1,
            metadata_where,
            param_idx + 2,
            param_idx + 3
        );

        // Add query parameters in order
        let mut final_params = vec![fts_query.clone().into(), fts_query.into()];
        final_params.extend(params);
        final_params.push((limit as i64).into());
        final_params.push((offset as i64).into());

        // Execute query
        let stmt = Statement::from_sql_and_values(DatabaseBackend::Sqlite, sql, final_params);

        #[derive(Debug, FromQueryResult)]
        struct QueryRow {
            ticket_id: String,
            ticket_number: i32,
            title: String,
            description: String,
            status: String,
            story_points: Option<i32>,
            snippet: String,
            rank: f64,
            matched_in: String,
            created_by: String,
            created_at: String,
            updated_at: String,
            parent_id: Option<String>,
            epic_id: Option<String>,
        }

        let rows = QueryRow::find_by_statement(stmt)
            .all(self.db.as_ref())
            .await
            .map_err(|e| JilityError::DatabaseError(e.to_string()))?;

        // Convert to SearchResult
        let mut results = Vec::new();
        for row in rows {
            let ticket_id = uuid::Uuid::parse_str(&row.ticket_id)
                .map_err(|e| JilityError::DatabaseError(e.to_string()))?;

            results.push(SearchResult {
                ticket_id,
                ticket_number: format!("TASK-{}", row.ticket_number),
                title: row.title,
                description: row.description,
                status: row.status,
                story_points: row.story_points,
                snippet: row.snippet,
                rank: row.rank,
                matched_in: row.matched_in.split(',').map(String::from).collect(),
                assignees: Vec::new(), // Will be populated separately if needed
                labels: Vec::new(),    // Will be populated separately if needed
                created_by: row.created_by,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .map_err(|e| JilityError::DatabaseError(e.to_string()))?
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                    .map_err(|e| JilityError::DatabaseError(e.to_string()))?
                    .with_timezone(&chrono::Utc),
                parent_id: row
                    .parent_id
                    .and_then(|id| uuid::Uuid::parse_str(&id).ok()),
                epic_id: row.epic_id.and_then(|id| uuid::Uuid::parse_str(&id).ok()),
            });
        }

        Ok(results)
    }

    /// PostgreSQL full-text search implementation
    async fn search_postgres(
        &self,
        filters: &SearchFilters,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<SearchResult>, JilityError> {
        // Build WHERE clause for metadata filters
        let mut where_clauses = Vec::new();
        let mut params: Vec<sea_orm::Value> = Vec::new();
        let mut param_idx = 2; // $1 is reserved for the search query

        // Add project filter if specified
        if let Some(project_id) = filters.project_id {
            where_clauses.push(format!("t.project_id = ${}", param_idx));
            params.push(project_id.to_string().into());
            param_idx += 1;
        }

        // Add status filter
        if let Some(ref statuses) = filters.status {
            if !statuses.is_empty() {
                let placeholders: Vec<String> = (0..statuses.len())
                    .map(|i| format!("${}", param_idx + i))
                    .collect();
                where_clauses.push(format!("t.status = ANY(ARRAY[{}])", placeholders.join(", ")));
                for status in statuses {
                    params.push(status.clone().into());
                    param_idx += 1;
                }
            }
        }

        // Add created_by filter
        if let Some(ref created_by) = filters.created_by {
            where_clauses.push(format!("t.created_by = ${}", param_idx));
            params.push(created_by.clone().into());
            param_idx += 1;
        }

        // Add date filters
        if let Some(created_after) = filters.created_after {
            where_clauses.push(format!("t.created_at >= ${}", param_idx));
            params.push(created_after.to_rfc3339().into());
            param_idx += 1;
        }

        if let Some(created_before) = filters.created_before {
            where_clauses.push(format!("t.created_at <= ${}", param_idx));
            params.push(created_before.to_rfc3339().into());
            param_idx += 1;
        }

        if let Some(updated_after) = filters.updated_after {
            where_clauses.push(format!("t.updated_at >= ${}", param_idx));
            params.push(updated_after.to_rfc3339().into());
            param_idx += 1;
        }

        if let Some(updated_before) = filters.updated_before {
            where_clauses.push(format!("t.updated_at <= ${}", param_idx));
            params.push(updated_before.to_rfc3339().into());
            param_idx += 1;
        }

        // Add story points filters
        if let Some(min_points) = filters.min_points {
            where_clauses.push(format!("t.story_points >= ${}", param_idx));
            params.push(min_points.into());
            param_idx += 1;
        }

        if let Some(max_points) = filters.max_points {
            where_clauses.push(format!("t.story_points <= ${}", param_idx));
            params.push(max_points.into());
            param_idx += 1;
        }

        // Add epic/parent filters
        if let Some(epic_id) = filters.epic_id {
            where_clauses.push(format!("t.epic_id = ${}", param_idx));
            params.push(epic_id.to_string().into());
            param_idx += 1;
        }

        if let Some(parent_id) = filters.parent_id {
            where_clauses.push(format!("t.parent_id = ${}", param_idx));
            params.push(parent_id.to_string().into());
            param_idx += 1;
        }

        // Build metadata WHERE clause
        let metadata_where = if where_clauses.is_empty() {
            String::new()
        } else {
            format!("AND {}", where_clauses.join(" AND "))
        };

        // Build the main query using PostgreSQL full-text search
        let sql = format!(
            r#"
            WITH ticket_matches AS (
                SELECT
                    t.id as ticket_id,
                    ts_headline('english', t.description, query, 'MaxWords=50, MinWords=25') as snippet,
                    ts_rank(t.search_vector, query) as rank,
                    'title,description' as matched_in
                FROM tickets t, plainto_tsquery('english', $1) query
                WHERE t.search_vector @@ query
            ),
            comment_matches AS (
                SELECT
                    c.ticket_id,
                    ts_headline('english', c.content, query, 'MaxWords=50, MinWords=25') as snippet,
                    ts_rank(c.search_vector, query) as rank,
                    'comments' as matched_in
                FROM comments c, plainto_tsquery('english', $1) query
                WHERE c.search_vector @@ query
            ),
            all_matches AS (
                SELECT * FROM ticket_matches
                UNION ALL
                SELECT * FROM comment_matches
            ),
            ranked_tickets AS (
                SELECT
                    am.ticket_id,
                    am.snippet,
                    MAX(am.rank) as best_rank,
                    STRING_AGG(DISTINCT am.matched_in, ',') as matched_in
                FROM all_matches am
                GROUP BY am.ticket_id, am.snippet
            )
            SELECT
                t.id::text as ticket_id,
                t.ticket_number,
                t.title,
                t.description,
                t.status,
                t.story_points,
                rt.snippet,
                rt.best_rank as rank,
                rt.matched_in,
                t.created_by,
                t.created_at::text,
                t.updated_at::text,
                t.parent_id::text,
                t.epic_id::text
            FROM ranked_tickets rt
            INNER JOIN tickets t ON t.id = rt.ticket_id
            WHERE 1=1 {}
            ORDER BY rt.best_rank DESC
            LIMIT ${} OFFSET ${}
            "#,
            metadata_where,
            param_idx,
            param_idx + 1
        );

        // Prepare parameters
        let mut final_params = vec![filters.query.clone().into()];
        final_params.extend(params);
        final_params.push((limit as i64).into());
        final_params.push((offset as i64).into());

        // Execute query
        let stmt = Statement::from_sql_and_values(DatabaseBackend::Postgres, sql, final_params);

        #[derive(Debug, FromQueryResult)]
        struct QueryRow {
            ticket_id: String,
            ticket_number: i32,
            title: String,
            description: String,
            status: String,
            story_points: Option<i32>,
            snippet: String,
            rank: f64,
            matched_in: String,
            created_by: String,
            created_at: String,
            updated_at: String,
            parent_id: Option<String>,
            epic_id: Option<String>,
        }

        let rows = QueryRow::find_by_statement(stmt)
            .all(self.db.as_ref())
            .await
            .map_err(|e| JilityError::DatabaseError(e.to_string()))?;

        // Convert to SearchResult
        let mut results = Vec::new();
        for row in rows {
            let ticket_id = uuid::Uuid::parse_str(&row.ticket_id)
                .map_err(|e| JilityError::DatabaseError(e.to_string()))?;

            results.push(SearchResult {
                ticket_id,
                ticket_number: format!("TASK-{}", row.ticket_number),
                title: row.title,
                description: row.description,
                status: row.status,
                story_points: row.story_points,
                snippet: row.snippet,
                rank: row.rank,
                matched_in: row.matched_in.split(',').map(String::from).collect(),
                assignees: Vec::new(),
                labels: Vec::new(),
                created_by: row.created_by,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .map_err(|e| JilityError::DatabaseError(e.to_string()))?
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                    .map_err(|e| JilityError::DatabaseError(e.to_string()))?
                    .with_timezone(&chrono::Utc),
                parent_id: row
                    .parent_id
                    .and_then(|id| uuid::Uuid::parse_str(&id).ok()),
                epic_id: row.epic_id.and_then(|id| uuid::Uuid::parse_str(&id).ok()),
            });
        }

        Ok(results)
    }

    /// Escape special FTS5 characters in search query
    fn escape_fts_query(&self, query: &str) -> String {
        // FTS5 special characters: " * ( ) AND OR NOT
        // For now, just wrap in quotes for phrase search
        format!("\"{}\"", query.replace('"', ""))
    }
}
