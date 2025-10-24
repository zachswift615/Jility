use anyhow::{Context, Result};
use rmcp::ServiceExt;
use std::path::PathBuf;
use tracing::{info, error};

use crate::service::JilityService;

/// Run the MCP server with stdio transport
///
/// This function:
/// 1. Gets the current directory (or looks for .jility/ in project root)
/// 2. Creates the JilityService
/// 3. Starts the MCP server with stdio transport (reads from stdin, writes to stdout)
/// 4. Waits for the service to complete
pub async fn run_mcp_server() -> Result<()> {
    // Initialize tracing for debugging (logs to stderr, not stdout which is used for MCP protocol)
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    info!("Starting Jility MCP server");

    // Get current directory as project root
    let current_dir = std::env::current_dir()
        .context("Failed to get current directory")?;

    info!("Project root: {}", current_dir.display());

    // TODO: Connect to database
    // let db_path = current_dir.join(".jility/data.db");
    // let db = connect_to_database(&db_path).await?;

    // Create the service
    let service = JilityService::new(current_dir)
        .context("Failed to create Jility service")?;

    info!("Jility MCP service created successfully");

    // Start the server with stdio transport
    // This creates a bidirectional channel using stdin/stdout
    let peer = service
        .serve((tokio::io::stdin(), tokio::io::stdout()))
        .await
        .context("Failed to start MCP server")?;

    info!("MCP server started, waiting for requests...");

    // Wait for the service to complete
    // This blocks until the client closes the connection
    match peer.waiting().await {
        Ok(_) => {
            info!("MCP server completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("MCP server error: {}", e);
            Err(anyhow::anyhow!("MCP server error: {}", e))
        }
    }
}

// TODO: Implement database connection
// async fn connect_to_database(db_path: &Path) -> Result<DatabaseConnection> {
//     if !db_path.exists() {
//         return Err(anyhow::anyhow!(
//             "Database not found at {}. Run 'jility init' first.",
//             db_path.display()
//         ));
//     }
//
//     let db_url = format!("sqlite://{}?mode=rwc", db_path.display());
//     let db = Database::connect(&db_url).await?;
//
//     Ok(db)
// }
