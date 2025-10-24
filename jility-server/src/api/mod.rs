pub mod projects;
pub mod tickets;
pub mod comments;
pub mod dependencies;
pub mod activity;
pub mod search;
pub mod git;

use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};

use crate::state::AppState;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        // Projects
        .route("/api/projects", get(projects::list_projects))
        .route("/api/projects", post(projects::create_project))
        .route("/api/projects/:id", get(projects::get_project))
        // Tickets
        .route("/api/tickets", get(tickets::list_tickets))
        .route("/api/tickets", post(tickets::create_ticket))
        .route("/api/tickets/:id", get(tickets::get_ticket))
        .route("/api/tickets/:id", put(tickets::update_ticket))
        .route("/api/tickets/:id", delete(tickets::delete_ticket))
        .route(
            "/api/tickets/:id/description",
            patch(tickets::update_description),
        )
        .route("/api/tickets/:id/status", patch(tickets::update_status))
        .route("/api/tickets/:id/assign", post(tickets::assign_ticket))
        .route("/api/tickets/:id/unassign", post(tickets::unassign_ticket))
        // Comments
        .route("/api/tickets/:id/comments", get(comments::list_comments))
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
        // Search
        .route("/api/search", get(search::search_tickets))
        // Git Integration
        .route("/api/tickets/:id/commits", post(git::link_commit))
        .route("/api/tickets/:id/commits", get(git::list_commits))
}
