use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{Priority, TicketNumber, TicketStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub key: String, // e.g., "TASK"
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(name: String, key: String) -> Self {
        let now = Utc::now();
        Project {
            id: Uuid::new_v4(),
            name,
            key,
            description: None,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: Uuid,
    pub project_id: Uuid,
    pub ticket_number: TicketNumber,
    pub sequence_number: i32,
    pub title: String,
    pub description: String,
    pub status: TicketStatus,
    pub priority: Priority,
    pub story_points: Option<i32>,
    pub assignees: Vec<String>,
    pub labels: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
}

impl Ticket {
    pub fn new(
        project_id: Uuid,
        project_key: &str,
        sequence_number: i32,
        title: String,
        created_by: String,
    ) -> Self {
        let now = Utc::now();
        Ticket {
            id: Uuid::new_v4(),
            project_id,
            ticket_number: TicketNumber::new(project_key, sequence_number),
            sequence_number,
            title,
            description: String::new(),
            status: TicketStatus::Todo,
            priority: Priority::Medium,
            story_points: None,
            assignees: vec![],
            labels: vec![],
            created_at: now,
            updated_at: now,
            created_by,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub author: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl Comment {
    pub fn new(ticket_id: Uuid, author: String, content: String) -> Self {
        Comment {
            id: Uuid::new_v4(),
            ticket_id,
            author,
            content,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescriptionVersion {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub content: String,
    pub version: i32,
    pub changed_by: String,
    pub created_at: DateTime<Utc>,
}

impl DescriptionVersion {
    pub fn new(ticket_id: Uuid, content: String, version: i32, changed_by: String) -> Self {
        DescriptionVersion {
            id: Uuid::new_v4(),
            ticket_id,
            content,
            version,
            changed_by,
            created_at: Utc::now(),
        }
    }
}
