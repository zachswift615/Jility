mod api;
mod error;
mod models;
mod state;
mod websocket;

use anyhow::Result;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    api::api_routes,
    state::{connect_database, AppState},
    websocket::websocket_routes,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "jility_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Jility server...");

    // Connect to database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://.jility/data.db?mode=rwc".to_string());

    tracing::info!("Connecting to database: {}", database_url);
    let db = connect_database(&database_url).await?;

    // Create app state
    let state = AppState::new(db);

    // Build router
    let app = Router::new()
        .merge(api_routes())
        .merge(websocket_routes())
        .layer(
            ServiceBuilder::new()
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any),
                )
                .layer(TraceLayer::new_for_http()),
        )
        .with_state(state);

    // Start server
    let addr = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
