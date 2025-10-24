use anyhow::Result;
use jility_mcp::run_mcp_server;

#[tokio::main]
async fn main() -> Result<()> {
    run_mcp_server().await
}
