use anyhow::{Context, Result};
use jility_core::{Database, Project};
use std::fs;
use std::path::PathBuf;

use crate::output::{print_error, print_success};

pub async fn run() -> Result<()> {
    // Get current directory
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let jility_dir = current_dir.join(".jility");
    
    // Check if already initialized
    if jility_dir.exists() {
        print_error("Jility project already initialized in this directory!");
        anyhow::bail!("Project already initialized");
    }
    
    // Create .jility directory
    fs::create_dir(&jility_dir).context("Failed to create .jility directory")?;
    
    // Create database
    let db_path = jility_dir.join("data.db");
    let db = Database::new(&db_path).context("Failed to initialize database")?;
    
    // Create default project
    let project = Project::new("Default Project".to_string(), "TASK".to_string());
    db.create_project(&project).context("Failed to create default project")?;
    
    print_success("Initialized Jility project!");
    println!("  Database: {}", db_path.display());
    println!("  Project key: TASK");
    println!("\nNext steps:");
    println!("  1. Create a ticket: jility ticket create --title \"My first ticket\"");
    println!("  2. List tickets: jility ticket list");
    
    Ok(())
}
