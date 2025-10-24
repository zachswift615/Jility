mod api;
mod auth;
mod error;
mod models;
mod state;
mod websocket;

use anyhow::Result;
use axum::Router;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    api::api_routes,
    auth::auth_middleware,
    state::{connect_database, AppState},
    websocket::websocket_routes,
};
use axum::middleware;

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

    // Run migrations
    tracing::info!("Running database migrations...");
    jility_core::run_migrations(&db).await?;
    tracing::info!("Migrations completed successfully");

    // Get JWT secret from environment
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| {
            tracing::warn!("JWT_SECRET not set, using default (NOT SECURE FOR PRODUCTION)");
            "insecure_default_secret_change_in_production".to_string()
        });

    // Create app state
    let state = AppState::new(db, jwt_secret);

    // Build router
    let (public_routes, protected_routes) = api_routes();

    // Apply state to routers
    let public_with_state = public_routes.with_state(state.clone());
    let protected_with_state = protected_routes
        .with_state(state.clone())
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    // Merge all routes
    let app = Router::new()
        .merge(public_with_state)
        .merge(protected_with_state)
        .merge(websocket_routes().with_state(state))
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Start server
    let addr = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3900".to_string());
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
