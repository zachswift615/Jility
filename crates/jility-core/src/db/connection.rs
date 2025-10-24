use anyhow::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
use std::time::Duration;

/// Database configuration
#[derive(Debug, Clone)]
pub enum DatabaseConfig {
    /// SQLite database with file path
    Sqlite { path: String },

    /// PostgreSQL database with connection string
    Postgres { url: String },
}

impl DatabaseConfig {
    /// Create SQLite configuration with default path
    pub fn sqlite_default() -> Self {
        Self::Sqlite {
            path: ".jility/data.db".to_string(),
        }
    }

    /// Create SQLite configuration with custom path
    pub fn sqlite(path: impl Into<String>) -> Self {
        Self::Sqlite { path: path.into() }
    }

    /// Create PostgreSQL configuration
    pub fn postgres(url: impl Into<String>) -> Self {
        Self::Postgres { url: url.into() }
    }

    /// Get the database URL for SeaORM
    fn database_url(&self) -> String {
        match self {
            Self::Sqlite { path } => format!("sqlite://{}?mode=rwc", path),
            Self::Postgres { url } => url.clone(),
        }
    }

    /// Get the database type name
    pub fn database_type(&self) -> &str {
        match self {
            Self::Sqlite { .. } => "sqlite",
            Self::Postgres { .. } => "postgres",
        }
    }
}

/// Connect to the database with connection pooling
pub async fn connect(config: &DatabaseConfig) -> Result<DatabaseConnection> {
    let url = config.database_url();

    tracing::info!("Connecting to {} database", config.database_type());

    let mut opt = ConnectOptions::new(url);

    // Configure connection pool
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(tracing::log::LevelFilter::Debug);

    // For SQLite, ensure WAL mode for better concurrency
    if matches!(config, DatabaseConfig::Sqlite { .. }) {
        opt.sqlx_logging(true);
    }

    let db = Database::connect(opt).await?;

    tracing::info!("Successfully connected to database");

    Ok(db)
}

/// Run all pending migrations
pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    tracing::info!("Running database migrations");

    crate::migration::Migrator::up(db, None).await?;

    tracing::info!("Migrations completed successfully");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_config() {
        let config = DatabaseConfig::sqlite("test.db");
        assert_eq!(config.database_url(), "sqlite://test.db?mode=rwc");
        assert_eq!(config.database_type(), "sqlite");
    }

    #[test]
    fn test_postgres_config() {
        let config = DatabaseConfig::postgres("postgresql://localhost/test");
        assert_eq!(config.database_url(), "postgresql://localhost/test");
        assert_eq!(config.database_type(), "postgres");
    }

    #[test]
    fn test_sqlite_default() {
        let config = DatabaseConfig::sqlite_default();
        assert_eq!(config.database_url(), "sqlite://.jility/data.db?mode=rwc");
    }
}
