use anyhow::{anyhow, Context, Result};
use bcrypt::{hash, DEFAULT_COST};
use colored::Colorize;
use jility_core::entities::{user, User};
use sea_orm::{ColumnTrait, Database, EntityTrait, QueryFilter, Set};
use std::env;
use std::io::{self, Write as _};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // Get username or email from command line args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let username_or_email = &args[1];

    // Find the database
    let db_path = find_database()?;
    let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

    println!("{}", format!("📁 Using database: {}", db_path.display()).dimmed());

    // Connect to database
    let db = Database::connect(&db_url).await
        .context("Failed to connect to database")?;

    // Find the user
    let user_model = User::find()
        .filter(
            user::Column::Username.eq(username_or_email)
            .or(user::Column::Email.eq(username_or_email))
        )
        .one(&db)
        .await
        .context("Database error")?
        .ok_or_else(|| anyhow!("User not found: {}", username_or_email))?;

    println!("{}", format!("Found user: {} ({})", user_model.username, user_model.email).dimmed());

    // Prompt for password
    print!("{}", "Enter new password: ".yellow());
    io::stdout().flush()?;

    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim().to_string();

    if password.is_empty() {
        return Err(anyhow!("Password cannot be empty"));
    }

    print!("{}", "Confirm password: ".yellow());
    io::stdout().flush()?;

    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;
    let confirm = confirm.trim();

    if password != confirm {
        return Err(anyhow!("Passwords do not match"));
    }

    // Hash the password
    let password_hash = hash(password, DEFAULT_COST)
        .context("Failed to hash password")?;

    // Update the password in the database
    let mut user_active: user::ActiveModel = user_model.into();
    user_active.password_hash = Set(password_hash);
    user_active.updated_at = Set(chrono::Utc::now().fixed_offset());

    User::update(user_active)
        .exec(&db)
        .await
        .context("Failed to update password")?;

    println!("{}", "✓ Password reset successfully!".green().bold());

    Ok(())
}

fn print_usage() {
    println!("{}", "Jility Password Reset Tool".bold());
    println!("\n{}", "Usage:".bold());
    println!("  jility-reset-password <username_or_email>");
    println!("\n{}", "Example:".bold());
    println!("  jility-reset-password myuser");
    println!("  jility-reset-password user@example.com");
}

/// Find the Jility database file
fn find_database() -> Result<PathBuf> {
    // Try .jility/data.db in current directory
    let local_db = PathBuf::from(".jility/data.db");
    if local_db.exists() {
        return Ok(local_db);
    }

    // Try home directory
    if let Some(home) = dirs::home_dir() {
        let home_db = home.join(".jility/data.db");
        if home_db.exists() {
            return Ok(home_db);
        }
    }

    Err(anyhow!(
        "Could not find Jility database. Make sure you're in a Jility project directory or have initialized Jility."
    ))
}
