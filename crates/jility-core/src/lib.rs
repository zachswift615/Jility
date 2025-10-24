//! Jility Core - Shared domain models and database layer
//!
//! This crate provides the core functionality for Jility, including:
//! - Database entity models (projects, tickets, sprints, etc.)
//! - SeaORM migrations
//! - Database connection management
//! - Business logic and domain types

pub mod db;
pub mod entities;
pub mod error;
pub mod migration;
pub mod search;

// Re-export commonly used types
pub use db::{connect, run_migrations, DatabaseConfig};
pub use entities::*;
pub use error::{CoreError, CoreResult};
pub use sea_orm;

// Re-export UUID and Chrono types for convenience
pub use chrono::{DateTime, Utc};
pub use uuid::Uuid;
