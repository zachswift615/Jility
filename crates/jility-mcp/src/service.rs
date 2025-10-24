use anyhow::Result;
use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ToolsCapability},
    tool, ErrorData as McpError, ServerHandler,
};
use std::path::PathBuf;
use std::sync::Arc;

use crate::params::*;

/// Main service struct for Jility MCP server
#[derive(Clone)]
pub struct JilityService {
    tool_router: ToolRouter<Self>,
    project_root: PathBuf,
    // Database connection will be added later
    // db: Arc<DatabaseConnection>,
}

impl JilityService {
    pub fn new(project_root: PathBuf) -> Result<Self> {
        let tool_router = ToolRouter::new();

        Ok(Self {
            tool_router,
            project_root,
        })
    }

    /// Helper to format success messages
    fn success(message: String) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(message)]))
    }

    /// Helper to format error messages
    fn error(message: String) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::error(vec![Content::text(format!("❌ Error: {}", message))]))
    }
}

/// Tool implementations
#[rmcp::tool_router]
impl JilityService {
    /// Create a new ticket with title, description, and metadata
    #[tool(description = "Create a new ticket in Jility with title, description, story points, assignees, labels, and other metadata. Returns the created ticket ID and number.")]
    async fn create_ticket(
        &self,
        Parameters(params): Parameters<CreateTicketParams>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual database logic
        // For now, return mock data
        let ticket_number = "TASK-1";
        let status = params.status.as_deref().unwrap_or("backlog");

        let assignees_str = params.assignees
            .as_ref()
            .map(|a| a.join(", "))
            .unwrap_or_else(|| "unassigned".to_string());

        let labels_str = params.labels
            .as_ref()
            .map(|l| l.join(", "))
            .unwrap_or_else(|| "none".to_string());

        let story_points_str = params.story_points
            .map(|p| format!("{} pts", p))
            .unwrap_or_else(|| "not estimated".to_string());

        let message = format!(
            "✅ Created ticket {}\n\n\
             **Title:** {}\n\
             **Status:** {}\n\
             **Story Points:** {}\n\
             **Assignees:** {}\n\
             **Labels:** {}\n\n\
             View: `jility ticket show {}`",
            ticket_number,
            params.title,
            status,
            story_points_str,
            assignees_str,
            labels_str,
            ticket_number
        );

        Self::success(message)
    }

    /// Create multiple tickets at once
    #[tool(description = "Create multiple tickets in one operation. Useful when breaking down an epic into sub-tasks. Each ticket can have its own title, description, story points, and metadata.")]
    async fn create_tickets_batch(
        &self,
        Parameters(params): Parameters<CreateTicketsBatchParams>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual database logic
        let count = params.tickets.len();
        let parent_str = params.parent_id
            .as_ref()
            .map(|p| format!(" under {}", p))
            .unwrap_or_default();

        let mut tickets_list = String::new();
        for (i, ticket) in params.tickets.iter().enumerate() {
            let ticket_num = format!("TASK-{}", i + 2);
            let assignee = ticket.assignees
                .as_ref()
                .and_then(|a| a.first())
                .map(|s| s.as_str())
                .unwrap_or("unassigned");
            let points = ticket.story_points.unwrap_or(0);

            tickets_list.push_str(&format!(
                "\n- {}: {} ({} pts) → {}",
                ticket_num, ticket.title, points, assignee
            ));
        }

        let message = format!(
            "✅ Created {} tickets{}\n{}",
            count,
            parent_str,
            tickets_list
        );

        Self::success(message)
    }

    /// Get full ticket context including comments, dependencies, and history
    #[tool(description = "Get complete ticket information including description, status, assignees, comments, dependencies, linked commits, and change history. Returns comprehensive context for AI agents to understand the ticket.")]
    async fn get_ticket(
        &self,
        Parameters(params): Parameters<GetTicketParams>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual database logic
        let ticket_id = &params.ticket_id;

        let ticket_data = serde_json::json!({
            "ticket": {
                "id": "uuid-123",
                "number": ticket_id,
                "title": "Example Ticket",
                "description": "## Context\n\nThis is an example ticket.\n\n## Acceptance Criteria\n\n- [ ] Implement feature\n- [ ] Write tests",
                "status": "in_progress",
                "story_points": 5,
                "assignees": ["agent-1"],
                "labels": ["backend"],
                "created_at": "2024-10-24T00:00:00Z",
                "created_by": "alice"
            },
            "comments": [
                {
                    "author": "alice",
                    "content": "Please make sure to handle edge cases",
                    "created_at": "2024-10-24T01:00:00Z"
                }
            ],
            "dependencies": [],
            "dependents": [],
            "linked_commits": [],
            "recent_changes": [
                {
                    "type": "status_changed",
                    "old_value": "todo",
                    "new_value": "in_progress",
                    "changed_by": "agent-1",
                    "changed_at": "2024-10-24T00:30:00Z"
                }
            ]
        });

        let message = format!(
            "📋 Ticket {}\n\n{}",
            ticket_id,
            serde_json::to_string_pretty(&ticket_data).unwrap()
        );

        Self::success(message)
    }

    /// List tickets with optional filters
    #[tool(description = "Query tickets with filters for status, assignee, labels, parent/epic, or unassigned tickets. Returns a list of matching tickets with their key information.")]
    async fn list_tickets(
        &self,
        Parameters(params): Parameters<ListTicketsParams>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual database logic
        let filters = vec![
            params.status.as_ref().map(|s| format!("status: {:?}", s)),
            params.assignee.as_ref().map(|a| format!("assignee: {}", a)),
            params.labels.as_ref().map(|l| format!("labels: {:?}", l)),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(", ");

        let filters_str = if filters.is_empty() {
            "all tickets".to_string()
        } else {
            format!("filtered by: {}", filters)
        };

        let message = format!(
            "📋 Found 3 tickets ({})\n\n\
             ## In Progress (1)\n\
             - TASK-1: Example ticket (agent-1) • 5 pts\n\n\
             ## Todo (1)\n\
             - TASK-2: Another ticket (unassigned) • 3 pts\n\n\
             ## Backlog (1)\n\
             - TASK-3: Future work (unassigned) • 8 pts",
            filters_str
        );

        Self::success(message)
    }

    /// Claim an unassigned ticket
    #[tool(description = "Agent claims an unassigned ticket, automatically assigning it to themselves and optionally moving it to 'in_progress' status. Include an optional message about starting work.")]
    async fn claim_ticket(
        &self,
        Parameters(params): Parameters<ClaimTicketParams>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual database logic
        let ticket_id = &params.ticket_id;
        let message_str = params.message
            .as_ref()
            .map(|m| format!("\n**Message:** {}", m))
            .unwrap_or_default();

        let message = format!(
            "✅ Claimed {} and assigned to agent-1\n\n\
             **Status:** In Progress (moved from Todo)\n\
             **Story Points:** 3{}\n\n\
             Get context: `jility ticket show {}`",
            ticket_id,
            message_str,
            ticket_id
        );

        Self::success(message)
    }

    /// Update ticket description with precise editing operations
    #[tool(description = "Precisely edit ticket description using operations: replace_all (full replacement), append (add to end), prepend (add to beginning), replace_lines (replace specific lines), replace_section (replace markdown section). Token-efficient for large descriptions.")]
    async fn update_description(
        &self,
        Parameters(params): Parameters<UpdateDescriptionParams>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual database logic
        let operation_str = match params.operation {
            EditOperation::ReplaceAll => "replace_all".to_string(),
            EditOperation::Append => "append".to_string(),
            EditOperation::Prepend => "prepend".to_string(),
            EditOperation::ReplaceLines => format!(
                "replace_lines (lines {}-{})",
                params.start_line.unwrap_or(0),
                params.end_line.unwrap_or(0)
            ),
            EditOperation::ReplaceSection => format!(
                "replace_section ('{}')",
                params.section_header.as_deref().unwrap_or("unknown")
            ),
        };

        let message_str = params.message
            .as_ref()
            .map(|m| format!("\n**Message:** {}", m))
            .unwrap_or_default();

        let message = format!(
            "✅ Updated description for {}\n\n\
             **Operation:** {}\n\
             **Changed by:** agent-1{}\n\n\
             Version history: `jility ticket history {}`",
            params.ticket_id,
            operation_str,
            message_str,
            params.ticket_id
        );

        Self::success(message)
    }

    /// Update ticket status
    #[tool(description = "Move ticket through workflow states: backlog, todo, in_progress, review, done, blocked. Include optional message for context. Validates status transitions.")]
    async fn update_status(
        &self,
        Parameters(params): Parameters<UpdateStatusParams>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual database logic and validation
        let valid_statuses = ["backlog", "todo", "in_progress", "review", "done", "blocked"];

        if !valid_statuses.contains(&params.status.as_str()) {
            return Self::error(format!(
                "Invalid status '{}'. Valid statuses: {}",
                params.status,
                valid_statuses.join(", ")
            ));
        }

        let message = format!(
            "✅ Moved {} to {}\n\n\
             **Previous:** todo\n\
             **Current:** {}\n\
             **Changed by:** agent-1",
            params.ticket_id,
            params.status,
            params.status
        );

        Self::success(message)
    }

    /// Add a comment to a ticket
    #[tool(description = "Add a markdown comment to a ticket. Supports @mentions to notify other team members. Comments are visible in ticket timeline and web UI.")]
    async fn add_comment(
        &self,
        Parameters(params): Parameters<AddCommentParams>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual database logic
        let message = format!(
            "✅ Added comment to {}\n\n\
             **From:** agent-1\n\
             **Content:**\n> {}\n\n\
             View: `jility ticket show {}`",
            params.ticket_id,
            params.content.lines().collect::<Vec<_>>().join("\n> "),
            params.ticket_id
        );

        Self::success(message)
    }

    /// Assign or reassign a ticket
    #[tool(description = "Assign ticket to one or more people (human or agent). Supports pairing by providing multiple assignees. Empty array to unassign. Include optional handoff message.")]
    async fn assign_ticket(
        &self,
        Parameters(params): Parameters<AssignTicketParams>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual database logic
        let assignees_str = if params.assignees.is_empty() {
            "unassigned".to_string()
        } else {
            params.assignees.join(", ")
        };

        let message_str = params.message
            .as_ref()
            .map(|m| format!("\n**Message:** \"{}\"\n\nHandoff message added as comment.", m))
            .unwrap_or_default();

        let message = format!(
            "✅ Reassigned {}\n\n\
             **Previous:** agent-1\n\
             **Current:** {}{}",
            params.ticket_id,
            assignees_str,
            message_str
        );

        Self::success(message)
    }

    /// Link a git commit to a ticket
    #[tool(description = "Link a git commit to a ticket for traceability. Provide commit hash and optional commit message. Linked commits appear in ticket details and timeline.")]
    async fn link_commit(
        &self,
        Parameters(params): Parameters<LinkCommitParams>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual database logic
        let commit_msg = params.commit_message
            .as_ref()
            .map(|m| format!("\n**Message:** {}", m))
            .unwrap_or_default();

        let message = format!(
            "✅ Linked commit to {}\n\n\
             **Commit:** {}{}\n\n\
             View commits: `jility ticket show {}`",
            params.ticket_id,
            params.commit_hash,
            commit_msg,
            params.ticket_id
        );

        Self::success(message)
    }

    /// Add a dependency between tickets
    #[tool(description = "Mark that a ticket depends on another ticket (blocks/blocked-by relationship). Prevents circular dependencies. Useful for tracking task order.")]
    async fn add_dependency(
        &self,
        Parameters(params): Parameters<AddDependencyParams>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual database logic and cycle detection
        let message = format!(
            "✅ Added dependency\n\n\
             {} now depends on {}\n\n\
             {} is blocked until {} is complete.",
            params.ticket_id,
            params.depends_on,
            params.ticket_id,
            params.depends_on
        );

        Self::success(message)
    }

    /// Remove a dependency between tickets
    #[tool(description = "Remove a dependency relationship between two tickets. Unblocks the dependent ticket if this was the last blocking dependency.")]
    async fn remove_dependency(
        &self,
        Parameters(params): Parameters<RemoveDependencyParams>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual database logic
        let message = format!(
            "✅ Removed dependency\n\n\
             {} no longer depends on {}",
            params.ticket_id,
            params.depends_on
        );

        Self::success(message)
    }

    /// Get dependency graph for a ticket
    #[tool(description = "Get full dependency tree showing what blocks this ticket and what this ticket blocks. Returns recursive dependency chain.")]
    async fn get_dependency_graph(
        &self,
        Parameters(params): Parameters<GetDependencyGraphParams>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual database logic
        let graph_data = serde_json::json!({
            "ticket": params.ticket_id,
            "dependencies": [],
            "dependents": [],
            "is_blocked": false,
            "blocking_count": 0
        });

        let message = format!(
            "📊 Dependency graph for {}\n\n{}",
            params.ticket_id,
            serde_json::to_string_pretty(&graph_data).unwrap()
        );

        Self::success(message)
    }

    /// Search tickets by full-text query
    #[tool(description = "Full-text search across ticket titles, descriptions, and comments. Returns matching tickets with snippets showing where the query matched.")]
    async fn search_tickets(
        &self,
        Parameters(params): Parameters<SearchTicketsParams>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual database logic
        let message = format!(
            "🔍 Found 2 tickets matching \"{}\"\n\n\
             1. **TASK-1** • Example ticket (in_progress)\n   \
             > ...{} appears in the description...\n\n\
             2. **TASK-3** • Future work (backlog)\n   \
             > ...related to {}...",
            params.query,
            params.query,
            params.query
        );

        Self::success(message)
    }

    /// List available ticket templates
    #[tool(description = "List all available ticket templates. Templates provide pre-filled ticket structures for common patterns like API endpoints, bugs, features, etc.")]
    async fn list_templates(
        &self,
        Parameters(_params): Parameters<ListTemplatesParams>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual template loading
        let message = "📋 Available Templates\n\n\
            1. **api-endpoint** - REST API endpoint with validation\n\
            2. **database-migration** - Schema migration with rollback\n\
            3. **react-component** - React component with tests\n\
            4. **bug-fix** - Bug investigation and fix\n\
            5. **refactor** - Code refactoring task";

        Self::success(message.to_string())
    }

    /// Create ticket from template
    #[tool(description = "Create a ticket from a template with variable substitution. Provide template name and variables object. Templates speed up ticket creation for common patterns.")]
    async fn create_from_template(
        &self,
        Parameters(params): Parameters<CreateFromTemplateParams>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual template loading and variable substitution
        let assignee_str = params.assignee
            .as_ref()
            .map(|a| format!("**Assignee:** {}\n", a))
            .unwrap_or_default();

        let message = format!(
            "✅ Created TASK-10 from template '{}'\n\n\
             **Title:** Example Ticket\n\
             {}**Variables:** {}\n\n\
             View: `jility ticket show TASK-10`",
            params.template,
            assignee_str,
            params.variables
        );

        Self::success(message)
    }
}

/// Server handler implementation for MCP protocol
#[rmcp::tool_handler]
impl ServerHandler for JilityService {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        rmcp::model::ServerInfo {
            protocol_version: ProtocolVersion::default(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: None,
                }),
                ..Default::default()
            },
            server_info: Implementation {
                name: "jility-mcp".to_string(),
                title: Some("Jility MCP Server".to_string()),
                version: env!("CARGO_PKG_VERSION").to_string(),
                icons: None,
                website_url: Some("https://github.com/yourusername/jility".to_string()),
            },
            instructions: Some(
                "Jility provides AI-native project management for humans and agents working together. \
                 Use create_ticket to add tasks, update_description for precise edits, \
                 get_ticket for full context, list_tickets to query tickets, \
                 and workflow tools like update_status, add_comment, and assign_ticket for collaboration. \
                 For dependencies, use add_dependency and get_dependency_graph.".to_string()
            ),
        }
    }
}
