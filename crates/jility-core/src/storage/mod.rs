use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;
use uuid::Uuid;

use crate::models::{Comment, DescriptionVersion, Project, Ticket};
use crate::types::{Priority, TicketNumber, TicketStatus};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)
            .context("Failed to open database connection")?;
        
        let db = Database { conn };
        db.initialize()?;
        Ok(db)
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()
            .context("Failed to open in-memory database")?;
        
        let db = Database { conn };
        db.initialize()?;
        Ok(db)
    }

    fn initialize(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                key TEXT NOT NULL UNIQUE,
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tickets (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                sequence_number INTEGER NOT NULL,
                ticket_number TEXT NOT NULL UNIQUE,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                status TEXT NOT NULL,
                priority TEXT NOT NULL,
                story_points INTEGER,
                assignees TEXT NOT NULL,
                labels TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                created_by TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            );

            CREATE TABLE IF NOT EXISTS comments (
                id TEXT PRIMARY KEY,
                ticket_id TEXT NOT NULL,
                author TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (ticket_id) REFERENCES tickets(id)
            );

            CREATE TABLE IF NOT EXISTS description_versions (
                id TEXT PRIMARY KEY,
                ticket_id TEXT NOT NULL,
                content TEXT NOT NULL,
                version INTEGER NOT NULL,
                changed_by TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (ticket_id) REFERENCES tickets(id)
            );

            CREATE INDEX IF NOT EXISTS idx_tickets_project_id ON tickets(project_id);
            CREATE INDEX IF NOT EXISTS idx_tickets_status ON tickets(status);
            CREATE INDEX IF NOT EXISTS idx_comments_ticket_id ON comments(ticket_id);
            CREATE INDEX IF NOT EXISTS idx_description_versions_ticket_id ON description_versions(ticket_id);
            "#
        ).context("Failed to initialize database schema")?;

        Ok(())
    }

    // Project operations
    pub fn create_project(&self, project: &Project) -> Result<()> {
        self.conn.execute(
            "INSERT INTO projects (id, name, key, description, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                project.id.to_string(),
                &project.name,
                &project.key,
                &project.description,
                project.created_at.to_rfc3339(),
                project.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn get_project_by_key(&self, key: &str) -> Result<Option<Project>> {
        let project = self.conn.query_row(
            "SELECT id, name, key, description, created_at, updated_at FROM projects WHERE key = ?1",
            params![key],
            |row| {
                Ok(Project {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    name: row.get(1)?,
                    key: row.get(2)?,
                    description: row.get(3)?,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                        .unwrap()
                        .with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                        .unwrap()
                        .with_timezone(&Utc),
                })
            },
        ).optional()?;
        Ok(project)
    }

    pub fn get_default_project(&self) -> Result<Option<Project>> {
        self.get_project_by_key("TASK")
    }

    // Ticket operations
    pub fn create_ticket(&self, ticket: &Ticket) -> Result<()> {
        self.conn.execute(
            r#"INSERT INTO tickets 
               (id, project_id, sequence_number, ticket_number, title, description, 
                status, priority, story_points, assignees, labels, created_at, updated_at, created_by)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)"#,
            params![
                ticket.id.to_string(),
                ticket.project_id.to_string(),
                ticket.sequence_number,
                ticket.ticket_number.as_str(),
                &ticket.title,
                &ticket.description,
                ticket.status.to_string(),
                ticket.priority.to_string(),
                ticket.story_points,
                serde_json::to_string(&ticket.assignees)?,
                serde_json::to_string(&ticket.labels)?,
                ticket.created_at.to_rfc3339(),
                ticket.updated_at.to_rfc3339(),
                &ticket.created_by,
            ],
        )?;
        Ok(())
    }

    pub fn get_next_sequence_number(&self, project_id: Uuid) -> Result<i32> {
        let max: Option<i32> = self.conn.query_row(
            "SELECT MAX(sequence_number) FROM tickets WHERE project_id = ?1",
            params![project_id.to_string()],
            |row| row.get(0),
        ).optional()?;
        
        Ok(max.unwrap_or(0) + 1)
    }

    pub fn get_ticket_by_number(&self, ticket_number: &str) -> Result<Option<Ticket>> {
        let ticket = self.conn.query_row(
            r#"SELECT id, project_id, sequence_number, ticket_number, title, description,
                      status, priority, story_points, assignees, labels, created_at, updated_at, created_by
               FROM tickets WHERE ticket_number = ?1"#,
            params![ticket_number],
            |row| self.row_to_ticket(row),
        ).optional()?;
        Ok(ticket)
    }

    pub fn list_tickets(&self, project_id: Option<Uuid>, status: Option<&str>) -> Result<Vec<Ticket>> {
        let mut sql = String::from(
            r#"SELECT id, project_id, sequence_number, ticket_number, title, description,
                      status, priority, story_points, assignees, labels, created_at, updated_at, created_by
               FROM tickets WHERE 1=1"#
        );
        
        let mut params: Vec<String> = vec![];
        
        if let Some(pid) = project_id {
            sql.push_str(" AND project_id = ?");
            params.push(pid.to_string());
        }
        
        if let Some(s) = status {
            sql.push_str(" AND status = ?");
            params.push(s.to_string());
        }
        
        sql.push_str(" ORDER BY sequence_number DESC");
        
        let mut stmt = self.conn.prepare(&sql)?;
        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();
        
        let tickets = stmt.query_map(&param_refs[..], |row| self.row_to_ticket(row))?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(tickets)
    }

    pub fn update_ticket(&self, ticket: &Ticket) -> Result<()> {
        self.conn.execute(
            r#"UPDATE tickets SET title = ?1, description = ?2, status = ?3, priority = ?4,
                                 story_points = ?5, assignees = ?6, labels = ?7, updated_at = ?8
               WHERE id = ?9"#,
            params![
                &ticket.title,
                &ticket.description,
                ticket.status.to_string(),
                ticket.priority.to_string(),
                ticket.story_points,
                serde_json::to_string(&ticket.assignees)?,
                serde_json::to_string(&ticket.labels)?,
                ticket.updated_at.to_rfc3339(),
                ticket.id.to_string(),
            ],
        )?;
        Ok(())
    }

    fn row_to_ticket(&self, row: &rusqlite::Row) -> rusqlite::Result<Ticket> {
        Ok(Ticket {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
            project_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
            sequence_number: row.get(2)?,
            ticket_number: TicketNumber::from_string(row.get(3)?),
            title: row.get(4)?,
            description: row.get(5)?,
            status: TicketStatus::from_str(&row.get::<_, String>(6)?).unwrap(),
            priority: Priority::from_str(&row.get::<_, String>(7)?).unwrap(),
            story_points: row.get(8)?,
            assignees: serde_json::from_str(&row.get::<_, String>(9)?).unwrap_or_default(),
            labels: serde_json::from_str(&row.get::<_, String>(10)?).unwrap_or_default(),
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(11)?)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(12)?)
                .unwrap()
                .with_timezone(&Utc),
            created_by: row.get(13)?,
        })
    }

    // Comment operations
    pub fn create_comment(&self, comment: &Comment) -> Result<()> {
        self.conn.execute(
            "INSERT INTO comments (id, ticket_id, author, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                comment.id.to_string(),
                comment.ticket_id.to_string(),
                &comment.author,
                &comment.content,
                comment.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn get_comments(&self, ticket_id: Uuid) -> Result<Vec<Comment>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, ticket_id, author, content, created_at FROM comments WHERE ticket_id = ?1 ORDER BY created_at ASC"
        )?;
        
        let comments = stmt.query_map(params![ticket_id.to_string()], |row| {
            Ok(Comment {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                ticket_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                author: row.get(2)?,
                content: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(comments)
    }

    // Description version operations
    pub fn create_description_version(&self, version: &DescriptionVersion) -> Result<()> {
        self.conn.execute(
            "INSERT INTO description_versions (id, ticket_id, content, version, changed_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                version.id.to_string(),
                version.ticket_id.to_string(),
                &version.content,
                version.version,
                &version.changed_by,
                version.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn get_description_versions(&self, ticket_id: Uuid) -> Result<Vec<DescriptionVersion>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, ticket_id, content, version, changed_by, created_at FROM description_versions WHERE ticket_id = ?1 ORDER BY version DESC"
        )?;
        
        let versions = stmt.query_map(params![ticket_id.to_string()], |row| {
            Ok(DescriptionVersion {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                ticket_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                content: row.get(2)?,
                version: row.get(3)?,
                changed_by: row.get(4)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(versions)
    }
}
