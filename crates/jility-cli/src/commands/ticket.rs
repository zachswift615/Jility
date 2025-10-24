use anyhow::{Context, Result};
use chrono::Utc;
use jility_core::{Comment, Database, DescriptionVersion, Priority, Ticket, TicketStatus};
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempfile::NamedTempFile;

use crate::output::{print_error, print_info, print_json, print_success, print_ticket_details, print_ticket_table};

fn get_db_path() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let jility_dir = current_dir.join(".jility");
    
    if !jility_dir.exists() {
        print_error("Not a Jility project! Run 'jility init' first.");
        anyhow::bail!("Not initialized");
    }
    
    Ok(jility_dir.join("data.db"))
}

fn get_current_user() -> String {
    env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string())
}

pub async fn create(
    title: String,
    description: Option<String>,
    story_points: Option<i32>,
    assignees: Option<String>,
    labels: Option<String>,
    priority: Option<String>,
) -> Result<()> {
    let db_path = get_db_path()?;
    let db = Database::new(&db_path)?;
    
    // Get default project
    let project = db.get_default_project()?
        .context("Default project not found")?;
    
    // Get next sequence number
    let sequence_number = db.get_next_sequence_number(project.id)?;
    
    // Create ticket
    let current_user = get_current_user();
    let mut ticket = Ticket::new(
        project.id,
        &project.key,
        sequence_number,
        title,
        current_user.clone(),
    );
    
    if let Some(desc) = description {
        ticket.description = desc.clone();
        
        // Create initial version
        let version = DescriptionVersion::new(ticket.id, desc, 1, current_user);
        db.create_description_version(&version)?;
    }
    
    if let Some(points) = story_points {
        ticket.story_points = Some(points);
    }
    
    if let Some(assignee_str) = assignees {
        ticket.assignees = assignee_str.split(',').map(|s| s.trim().to_string()).collect();
    }
    
    if let Some(label_str) = labels {
        ticket.labels = label_str.split(',').map(|s| s.trim().to_string()).collect();
    }
    
    if let Some(priority_str) = priority {
        ticket.priority = Priority::from_str(&priority_str)
            .context(format!("Invalid priority: {}", priority_str))?;
    }
    
    db.create_ticket(&ticket)?;
    
    print_success(&format!("Created ticket {}", ticket.ticket_number));
    println!("  Title: {}", ticket.title);
    if !ticket.assignees.is_empty() {
        println!("  Assignees: {}", ticket.assignees.join(", "));
    }
    
    Ok(())
}

pub async fn list(
    status: Option<String>,
    assignee: Option<String>,
    format: String,
) -> Result<()> {
    let db_path = get_db_path()?;
    let db = Database::new(&db_path)?;
    
    let project = db.get_default_project()?
        .context("Default project not found")?;
    
    let status_filter = status.as_deref();
    let mut tickets = db.list_tickets(Some(project.id), status_filter)?;
    
    // Filter by assignee if specified
    if let Some(assignee_filter) = assignee {
        tickets.retain(|t| t.assignees.contains(&assignee_filter));
    }
    
    if format == "json" {
        print_json(&tickets)?;
    } else {
        print_ticket_table(&tickets);
    }
    
    Ok(())
}

pub async fn show(ticket_number: String, format: String) -> Result<()> {
    let db_path = get_db_path()?;
    let db = Database::new(&db_path)?;
    
    let ticket = db.get_ticket_by_number(&ticket_number)?
        .context(format!("Ticket {} not found", ticket_number))?;
    
    let comments = db.get_comments(ticket.id)?;
    
    if format == "json" {
        let data = serde_json::json!({
            "ticket": ticket,
            "comments": comments,
        });
        print_json(&data)?;
    } else {
        print_ticket_details(&ticket, &comments);
    }
    
    Ok(())
}

pub async fn update(
    ticket_number: String,
    title: Option<String>,
    story_points: Option<i32>,
    priority: Option<String>,
    labels: Option<String>,
) -> Result<()> {
    let db_path = get_db_path()?;
    let db = Database::new(&db_path)?;
    
    let mut ticket = db.get_ticket_by_number(&ticket_number)?
        .context(format!("Ticket {} not found", ticket_number))?;
    
    let mut changed = false;
    
    if let Some(new_title) = title {
        ticket.title = new_title;
        changed = true;
    }
    
    if let Some(points) = story_points {
        ticket.story_points = Some(points);
        changed = true;
    }
    
    if let Some(priority_str) = priority {
        ticket.priority = Priority::from_str(&priority_str)
            .context(format!("Invalid priority: {}", priority_str))?;
        changed = true;
    }
    
    if let Some(label_str) = labels {
        ticket.labels = label_str.split(',').map(|s| s.trim().to_string()).collect();
        changed = true;
    }
    
    if changed {
        ticket.updated_at = Utc::now();
        db.update_ticket(&ticket)?;
        print_success(&format!("Updated ticket {}", ticket_number));
    } else {
        print_info("No changes made");
    }
    
    Ok(())
}

pub async fn edit(ticket_number: String) -> Result<()> {
    let db_path = get_db_path()?;
    let db = Database::new(&db_path)?;
    
    let mut ticket = db.get_ticket_by_number(&ticket_number)?
        .context(format!("Ticket {} not found", ticket_number))?;
    
    // Get editor from environment
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    
    // Create temp file with current description
    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(ticket.description.as_bytes())?;
    let temp_path = temp_file.path().to_path_buf();
    
    // Launch editor
    let status = Command::new(&editor)
        .arg(&temp_path)
        .status()
        .context(format!("Failed to launch editor: {}", editor))?;
    
    if !status.success() {
        print_error("Editor exited with error");
        anyhow::bail!("Editor failed");
    }
    
    // Read edited content
    let new_description = fs::read_to_string(&temp_path)?;
    
    // Check if changed
    if new_description.trim() == ticket.description.trim() {
        print_info("No changes made");
        return Ok(());
    }
    
    // Get current version number
    let versions = db.get_description_versions(ticket.id)?;
    let next_version = versions.first().map(|v| v.version + 1).unwrap_or(1);
    
    // Update ticket
    ticket.description = new_description.clone();
    ticket.updated_at = Utc::now();
    db.update_ticket(&ticket)?;
    
    // Save version
    let current_user = get_current_user();
    let version = DescriptionVersion::new(ticket.id, new_description, next_version, current_user);
    db.create_description_version(&version)?;
    
    print_success(&format!("Updated description for {}", ticket_number));
    println!("  Version: {}", next_version);
    
    Ok(())
}

pub async fn move_ticket(ticket_number: String, to: String) -> Result<()> {
    let db_path = get_db_path()?;
    let db = Database::new(&db_path)?;
    
    let mut ticket = db.get_ticket_by_number(&ticket_number)?
        .context(format!("Ticket {} not found", ticket_number))?;
    
    let new_status = TicketStatus::from_str(&to)
        .context(format!("Invalid status: {}", to))?;
    
    let old_status = ticket.status.clone();
    ticket.status = new_status.clone();
    ticket.updated_at = Utc::now();
    
    db.update_ticket(&ticket)?;
    
    print_success(&format!(
        "Moved {} from {} to {}",
        ticket_number,
        old_status.to_string(),
        new_status.to_string()
    ));
    
    Ok(())
}

pub async fn assign(ticket_number: String, to: String, remove: bool) -> Result<()> {
    let db_path = get_db_path()?;
    let db = Database::new(&db_path)?;
    
    let mut ticket = db.get_ticket_by_number(&ticket_number)?
        .context(format!("Ticket {} not found", ticket_number))?;
    
    if remove {
        if let Some(pos) = ticket.assignees.iter().position(|a| a == &to) {
            ticket.assignees.remove(pos);
            ticket.updated_at = Utc::now();
            db.update_ticket(&ticket)?;
            print_success(&format!("Removed {} from {}", to, ticket_number));
        } else {
            print_info(&format!("{} is not assigned to {}", to, ticket_number));
        }
    } else {
        if !ticket.assignees.contains(&to) {
            ticket.assignees.push(to.clone());
            ticket.updated_at = Utc::now();
            db.update_ticket(&ticket)?;
            print_success(&format!("Assigned {} to {}", ticket_number, to));
        } else {
            print_info(&format!("{} is already assigned to {}", to, ticket_number));
        }
    }
    
    Ok(())
}

pub async fn comment(ticket_number: String, text: String) -> Result<()> {
    let db_path = get_db_path()?;
    let db = Database::new(&db_path)?;
    
    let ticket = db.get_ticket_by_number(&ticket_number)?
        .context(format!("Ticket {} not found", ticket_number))?;
    
    let current_user = get_current_user();
    let comment = Comment::new(ticket.id, current_user, text);
    
    db.create_comment(&comment)?;
    
    print_success(&format!("Added comment to {}", ticket_number));
    
    Ok(())
}

pub async fn history(ticket_number: String) -> Result<()> {
    let db_path = get_db_path()?;
    let db = Database::new(&db_path)?;
    
    let ticket = db.get_ticket_by_number(&ticket_number)?
        .context(format!("Ticket {} not found", ticket_number))?;
    
    let versions = db.get_description_versions(ticket.id)?;
    
    if versions.is_empty() {
        print_info("No description history found");
        return Ok(());
    }
    
    println!("\n{} Description History for {}", "📜".to_string(), ticket_number);
    println!("{}", "━".repeat(80));
    
    for version in versions {
        println!("\n{} Version {} by {} ({})",
            "•".to_string(),
            version.version,
            version.changed_by,
            version.created_at.format("%Y-%m-%d %H:%M")
        );
        println!("{}", "─".repeat(80));
        println!("{}", version.content);
    }
    
    println!("{}\n", "━".repeat(80));
    
    Ok(())
}
