use colored::*;
use jility_core::{Comment, Ticket, TicketStatus};
use tabled::{
    builder::Builder,
    settings::{object::Rows, Alignment, Modify, Style},
};

pub fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message);
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

pub fn format_status(status: &TicketStatus) -> ColoredString {
    match status {
        TicketStatus::Backlog => "BACKLOG".dimmed(),
        TicketStatus::Todo => "TODO".cyan(),
        TicketStatus::InProgress => "IN PROGRESS".yellow(),
        TicketStatus::InReview => "IN REVIEW".magenta(),
        TicketStatus::Done => "DONE".green(),
        TicketStatus::Cancelled => "CANCELLED".red(),
    }
}

pub fn print_ticket_table(tickets: &[Ticket]) {
    if tickets.is_empty() {
        println!("{}", "No tickets found.".dimmed());
        return;
    }

    let mut builder = Builder::default();
    
    // Header
    builder.push_record(vec!["ID", "Title", "Status", "Assignees", "Points"]);
    
    // Rows
    for ticket in tickets {
        builder.push_record(vec![
            ticket.ticket_number.as_str(),
            &ticket.title,
            &ticket.status.to_string(),
            &ticket.assignees.join(", "),
            &ticket.story_points.map(|p| p.to_string()).unwrap_or_default(),
        ]);
    }
    
    let mut table = builder.build();
    table.with(Style::rounded());
    table.with(Modify::new(Rows::first()).with(Alignment::center()));
    
    println!("{}", table);
}

pub fn print_ticket_details(ticket: &Ticket, comments: &[Comment]) {
    println!("\n{}", "━".repeat(80).dimmed());
    println!(
        "{} {}",
        ticket.ticket_number.as_str().bold(),
        ticket.title.bold()
    );
    println!("{}", "━".repeat(80).dimmed());
    
    println!("\n{}", "Status:".bold());
    println!("  {}", format_status(&ticket.status));
    
    println!("\n{}", "Priority:".bold());
    println!("  {}", ticket.priority.to_string().to_uppercase());
    
    if let Some(points) = ticket.story_points {
        println!("\n{}", "Story Points:".bold());
        println!("  {}", points);
    }
    
    if !ticket.assignees.is_empty() {
        println!("\n{}", "Assignees:".bold());
        for assignee in &ticket.assignees {
            println!("  • {}", assignee);
        }
    }
    
    if !ticket.labels.is_empty() {
        println!("\n{}", "Labels:".bold());
        for label in &ticket.labels {
            println!("  • {}", label.cyan());
        }
    }
    
    if !ticket.description.is_empty() {
        println!("\n{}", "Description:".bold());
        println!("{}", "─".repeat(80).dimmed());
        println!("{}", ticket.description);
        println!("{}", "─".repeat(80).dimmed());
    }
    
    if !comments.is_empty() {
        println!("\n{}", "Comments:".bold());
        println!("{}", "─".repeat(80).dimmed());
        for comment in comments {
            println!(
                "\n{} • {}",
                comment.author.cyan(),
                comment.created_at.format("%Y-%m-%d %H:%M").to_string().dimmed()
            );
            println!("{}", comment.content);
        }
        println!("{}", "─".repeat(80).dimmed());
    }
    
    println!("\n{}", "Metadata:".bold());
    println!("  Created by: {}", ticket.created_by);
    println!("  Created at: {}", ticket.created_at.format("%Y-%m-%d %H:%M"));
    println!("  Updated at: {}", ticket.updated_at.format("%Y-%m-%d %H:%M"));
    
    println!("{}\n", "━".repeat(80).dimmed());
}

pub fn print_json<T: serde::Serialize>(data: &T) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    println!("{}", json);
    Ok(())
}
