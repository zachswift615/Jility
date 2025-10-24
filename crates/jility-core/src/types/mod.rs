use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TicketStatus {
    Backlog,
    Todo,
    InProgress,
    InReview,
    Done,
    Cancelled,
}

impl fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TicketStatus::Backlog => write!(f, "backlog"),
            TicketStatus::Todo => write!(f, "todo"),
            TicketStatus::InProgress => write!(f, "in-progress"),
            TicketStatus::InReview => write!(f, "in-review"),
            TicketStatus::Done => write!(f, "done"),
            TicketStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl TicketStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "backlog" => Some(TicketStatus::Backlog),
            "todo" => Some(TicketStatus::Todo),
            "in-progress" | "inprogress" | "in_progress" => Some(TicketStatus::InProgress),
            "in-review" | "inreview" | "in_review" => Some(TicketStatus::InReview),
            "done" => Some(TicketStatus::Done),
            "cancelled" | "canceled" => Some(TicketStatus::Cancelled),
            _ => None,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            TicketStatus::Backlog,
            TicketStatus::Todo,
            TicketStatus::InProgress,
            TicketStatus::InReview,
            TicketStatus::Done,
            TicketStatus::Cancelled,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
            Priority::Urgent => write!(f, "urgent"),
        }
    }
}

impl Priority {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "low" => Some(Priority::Low),
            "medium" | "med" => Some(Priority::Medium),
            "high" => Some(Priority::High),
            "urgent" => Some(Priority::Urgent),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketNumber(String);

impl TicketNumber {
    pub fn new(project_key: &str, number: i32) -> Self {
        TicketNumber(format!("{}-{}", project_key, number))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_string(s: String) -> Self {
        TicketNumber(s)
    }
}

impl fmt::Display for TicketNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
