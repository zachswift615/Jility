pub mod activity;
pub mod auth;
pub mod comments;
pub mod dependencies;
pub mod git;
pub mod projects;
pub mod search;
pub mod sprints;
pub mod tickets;
pub mod workspaces;

use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};

use crate::auth::auth_middleware;
use crate::state::AppState;

pub fn api_routes() -> (Router<AppState>, Router<AppState>) {
    // Public routes (no authentication required)
    let public_routes = Router::new()
        // Auth
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        // Public read-only endpoints
        .route("/api/projects", get(projects::list_projects))
        .route("/api/projects/:id", get(projects::get_project))
        .route("/api/tickets", get(tickets::list_tickets))
        .route("/api/tickets/:id", get(tickets::get_ticket))
        .route("/api/tickets/:id/comments", get(comments::list_comments));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        // Workspaces
        .route("/api/workspaces", get(workspaces::list_workspaces))
        .route("/api/workspaces", post(workspaces::create_workspace))
        .route("/api/workspaces/:slug", get(workspaces::get_workspace))
        .route("/api/workspaces/:slug/invite", post(workspaces::invite_member))
        .route("/api/workspaces/:slug/members", get(workspaces::list_members))
        .route("/api/workspaces/:slug/members/:user_id", delete(workspaces::remove_member))
        // Auth - protected
        .route("/api/auth/logout", post(auth::logout))
        .route("/api/auth/me", get(auth::get_me))
        .route("/api/auth/api-keys", post(auth::create_api_key))
        .route("/api/auth/api-keys", get(auth::list_api_keys))
        .route("/api/auth/api-keys/:id", delete(auth::revoke_api_key))
        .route("/api/auth/sessions", get(auth::list_sessions))
        // Projects - write operations
        .route("/api/projects", post(projects::create_project))
        .route("/api/projects/:id", put(projects::update_project))
        .route("/api/projects/:id", delete(projects::delete_project))
        // Tickets - write operations
        .route("/api/tickets", post(tickets::create_ticket))
        .route("/api/tickets/:id", put(tickets::update_ticket))
        .route("/api/tickets/:id", delete(tickets::delete_ticket))
        .route(
            "/api/tickets/:id/description",
            patch(tickets::update_description),
        )
        .route("/api/tickets/:id/status", patch(tickets::update_status))
        .route("/api/tickets/:id/assign", post(tickets::assign_ticket))
        .route("/api/tickets/:id/unassign", post(tickets::unassign_ticket))
        // Comments - write operations
        .route("/api/tickets/:id/comments", post(comments::create_comment))
        .route("/api/comments/:id", put(comments::update_comment))
        .route("/api/comments/:id", delete(comments::delete_comment))
        // Dependencies
        .route(
            "/api/tickets/:id/dependencies",
            post(dependencies::add_dependency),
        )
        .route(
            "/api/tickets/:id/dependencies/:dep_id",
            delete(dependencies::remove_dependency),
        )
        .route(
            "/api/tickets/:id/dependency-graph",
            get(dependencies::get_dependency_graph),
        )
        // Activity & History
        .route("/api/tickets/:id/activity", get(activity::get_activity))
        .route("/api/tickets/:id/history", get(activity::get_history))
        .route(
            "/api/tickets/:id/history/:version",
            get(activity::get_version),
        )
        .route(
            "/api/tickets/:id/revert/:version",
            post(activity::revert_to_version),
        )
        // Search and Saved Views
        .route("/api/search", get(search::search_tickets))
        .route("/api/search/views", get(search::list_saved_views))
        .route("/api/search/views/:id", get(search::get_saved_view))
        .route("/api/search/views", post(search::create_saved_view))
        .route("/api/search/views/:id", put(search::update_saved_view))
        .route("/api/search/views/:id", delete(search::delete_saved_view))
        // Git Integration
        .route("/api/tickets/:id/commits", post(git::link_commit))
        .route("/api/tickets/:id/commits", get(git::list_commits))
        // Sprints
        .route("/api/projects/:project_id/sprints", get(sprints::list_sprints))
        .route("/api/projects/:project_id/sprints", post(sprints::create_sprint))
        .route("/api/sprints/:id", get(sprints::get_sprint))
        .route("/api/sprints/:id", put(sprints::update_sprint))
        .route("/api/sprints/:id", delete(sprints::delete_sprint))
        .route("/api/sprints/:id/start", post(sprints::start_sprint))
        .route("/api/sprints/:id/complete", post(sprints::complete_sprint))
        .route("/api/sprints/:id/tickets/:ticket_id", post(sprints::add_ticket_to_sprint))
        .route("/api/sprints/:id/tickets/:ticket_id", delete(sprints::remove_ticket_from_sprint))
        .route("/api/sprints/:id/stats", get(sprints::get_sprint_stats))
        .route("/api/sprints/:id/burndown", get(sprints::get_burndown))
        .route("/api/projects/:project_id/sprint-history", get(sprints::get_sprint_history));

    // Return both routers - middleware will be applied in main.rs after with_state
    (public_routes, protected_routes)
}
