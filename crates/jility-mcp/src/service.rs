use rmcp::{
    ServerHandler,
    model::{Implementation, InitializeResult, ProtocolVersion, ServerCapabilities, ToolsCapability},
    tool, tool_box,
};
use reqwest::Client;
use serde_json::json;

use crate::params::*;

/// Main service struct for Jility MCP server
#[derive(Clone)]
pub struct JilityService {
    client: Client,
    api_base_url: String,
    auth_token: Option<String>,
    project_id: String,
}

impl JilityService {
    pub fn new() -> anyhow::Result<Self> {
        let api_base_url = std::env::var("JILITY_API_URL")
            .unwrap_or_else(|_| "http://localhost:3900/api".to_string());

        let auth_token = std::env::var("JILITY_API_TOKEN").ok();

        let project_id = std::env::var("JILITY_PROJECT_ID")
            .map_err(|_| anyhow::anyhow!("JILITY_PROJECT_ID environment variable is required"))?;

        tracing::info!("Jility API URL: {}", api_base_url);
        tracing::info!("Jility Project ID: {}", project_id);
        if auth_token.is_some() {
            tracing::info!("Using authentication token");
        } else {
            tracing::warn!("No authentication token configured (set JILITY_API_TOKEN)");
        }

        Ok(Self {
            client: Client::new(),
            api_base_url,
            auth_token,
            project_id,
        })
    }

    /// Build a request with authentication if available
    fn build_request(&self, method: reqwest::Method, url: String) -> reqwest::RequestBuilder {
        let mut request = self.client.request(method, &url);
        if let Some(token) = &self.auth_token {
            // Support both JWT tokens and API keys
            if token.starts_with("jil_") {
                // API key format - use "ApiKey" prefix
                request = request.header("Authorization", format!("ApiKey {}", token));
            } else {
                // JWT token format - use "Bearer" prefix
                request = request.header("Authorization", format!("Bearer {}", token));
            }
        }
        request
    }

    /// Create a new ticket
    #[tool(
        description = "Create a new ticket in Jility with title, description, story points, assignees, labels, and other metadata. Returns the created ticket ID and number."
    )]
    pub async fn create_ticket(
        &self,
        #[tool(param)] title: String,
        #[tool(param)] description: Option<String>,
        #[tool(param)] story_points: Option<i32>,
        #[tool(param)] status: Option<String>,
        #[tool(param)] assignees: Option<Vec<String>>,
        #[tool(param)] labels: Option<Vec<String>>,
        #[tool(param)] parent_id: Option<String>,
        #[tool(param)] epic_id: Option<String>,
    ) -> Result<String, String> {

        let response = self.build_request(
            reqwest::Method::POST,
            format!("{}/tickets", self.api_base_url)
        )
            .json(&json!({
                "project_id": self.project_id,
                "title": title,
                "description": description.unwrap_or_default(),
                "story_points": story_points,
                "status": status.unwrap_or_else(|| "backlog".to_string()),
                "assignees": assignees.unwrap_or_default(),
                "labels": labels.unwrap_or_default(),
                "parent_id": parent_id,
                "epic_id": epic_id,
            }))
            .send()
            .await
            .map_err(|e| format!("Failed to create ticket: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("API error: {}", error_text));
        }

        let ticket: serde_json::Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let ticket_number = ticket["number"].as_str().unwrap_or("UNKNOWN");
        let title = ticket["title"].as_str().unwrap_or("");
        let status = ticket["status"].as_str().unwrap_or("");
        let story_points = ticket["story_points"].as_i64().map(|p| format!("{} pts", p)).unwrap_or_else(|| "not estimated".to_string());

        let assignees_str = ticket["assignees"].as_array()
            .map(|a| a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join(", "))
            .unwrap_or_else(|| "unassigned".to_string());

        let labels_str = ticket["labels"].as_array()
            .map(|l| l.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join(", "))
            .unwrap_or_else(|| "none".to_string());

        Ok(format!(
            "✅ Created ticket {}\n\n\
             **Title:** {}\n\
             **Status:** {}\n\
             **Story Points:** {}\n\
             **Assignees:** {}\n\
             **Labels:** {}\n\n\
             View: `jility ticket show {}`",
            ticket_number, title, status, story_points, assignees_str, labels_str, ticket_number
        ))
    }

    /// Create multiple tickets at once
    #[tool(
        description = "Create multiple tickets in one operation. Useful when breaking down an epic into sub-tasks. Each ticket can have its own title, description, story points, and metadata."
    )]
    pub async fn create_tickets_batch(
        &self,
        #[tool(aggr)] params: CreateTicketsBatchParams,
    ) -> Result<String, String> {
        let mut created_tickets = Vec::new();

        for ticket_params in params.tickets {
            let response = self.build_request(
                reqwest::Method::POST,
                format!("{}/tickets", self.api_base_url)
            )
                .json(&json!({
                    "project_id": self.project_id,
                    "title": ticket_params.title,
                    "description": ticket_params.description.unwrap_or_default(),
                    "story_points": ticket_params.story_points,
                    "status": ticket_params.status.unwrap_or_else(|| "backlog".to_string()),
                    "assignees": ticket_params.assignees.unwrap_or_default(),
                    "labels": ticket_params.labels.unwrap_or_default(),
                    "parent_id": params.parent_id.clone(),
                    "epic_id": ticket_params.epic_id,
                }))
                .send()
                .await
                .map_err(|e| format!("Failed to create ticket: {}", e))?;

            if response.status().is_success() {
                let ticket: serde_json::Value = response.json().await
                    .map_err(|e| format!("Failed to parse response: {}", e))?;
                created_tickets.push(ticket);
            }
        }

        let parent_str = params.parent_id
            .as_ref()
            .map(|p| format!(" under {}", p))
            .unwrap_or_default();

        Ok(format!(
            "✅ Created {} tickets{}\n\n{}",
            created_tickets.len(),
            parent_str,
            created_tickets.iter()
                .map(|t| format!("- {}: {}",
                    t["number"].as_str().unwrap_or("?"),
                    t["title"].as_str().unwrap_or("?")))
                .collect::<Vec<_>>()
                .join("\n")
        ))
    }

    /// Get full ticket context
    #[tool(
        description = "Get full ticket details including description, comments, dependencies, linked commits, and change history. Returns comprehensive context for working on a ticket."
    )]
    pub async fn get_ticket(
        &self,
        #[tool(param)] ticket_id: String,
    ) -> Result<String, String> {

        let response = self.build_request(
            reqwest::Method::GET,
            format!("{}/tickets/{}", self.api_base_url, ticket_id)
        )
            .send()
            .await
            .map_err(|e| format!("Failed to get ticket: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Ticket not found: {}", ticket_id));
        }

        let data: serde_json::Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let ticket = &data["ticket"];

        Ok(format!(
            "📋 Ticket: {}\n\n\
             **Title:** {}\n\
             **Status:** {}\n\
             **Description:**\n{}\n\n\
             **Assignees:** {}\n\
             **Labels:** {}\n\
             **Story Points:** {}\n\n\
             **Comments:** {}\n\
             **Dependencies:** {}\n\
             **Linked Commits:** {}",
            ticket["number"].as_str().unwrap_or("?"),
            ticket["title"].as_str().unwrap_or(""),
            ticket["status"].as_str().unwrap_or(""),
            ticket["description"].as_str().unwrap_or("No description"),
            ticket["assignees"].as_array().map(|a| a.len()).unwrap_or(0),
            ticket["labels"].as_array().map(|l| l.len()).unwrap_or(0),
            ticket["story_points"].as_i64().unwrap_or(0),
            data["comments"].as_array().map(|c| c.len()).unwrap_or(0),
            data["dependencies"].as_array().map(|d| d.len()).unwrap_or(0),
            data["linked_commits"].as_array().map(|l| l.len()).unwrap_or(0),
        ))
    }

    /// Query tickets with filters
    #[tool(
        description = "List tickets with optional filters for status, assignee, labels, etc. Returns a summary of matching tickets."
    )]
    pub async fn list_tickets(
        &self,
        #[tool(param)] status: Option<Vec<String>>,
        #[tool(param)] assignee: Option<String>,
        #[tool(param)] labels: Option<Vec<String>>,
        #[tool(param)] parent_id: Option<String>,
        #[tool(param)] epic_id: Option<String>,
        #[tool(param)] unassigned: Option<bool>,
        #[tool(param)] limit: Option<u64>,
    ) -> Result<String, String> {

        let mut url = format!("{}/tickets", self.api_base_url);
        let mut query_params = Vec::new();

        // Always filter by the configured project ID
        query_params.push(format!("project_id={}", self.project_id));

        if let Some(status) = &status {
            for s in status {
                query_params.push(format!("status[]={}", s));
            }
        }
        if let Some(assignee) = &assignee {
            query_params.push(format!("assignee={}", assignee));
        }

        if !query_params.is_empty() {
            url.push('?');
            url.push_str(&query_params.join("&"));
        }

        let response = self.build_request(
            reqwest::Method::GET,
            url.clone()
        )
            .send()
            .await
            .map_err(|e| format!("Failed to list tickets: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_else(|_| "Unable to read error".to_string());
            return Err(format!("Failed to list tickets (HTTP {}): {} | URL: {}", status, error_body, url));
        }

        let tickets: Vec<serde_json::Value> = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        if tickets.is_empty() {
            return Ok("📋 No tickets found".to_string());
        }

        let mut output = format!("📋 Found {} tickets\n\n", tickets.len());

        for ticket in tickets.iter().take(limit.unwrap_or(50) as usize) {
            output.push_str(&format!(
                "- {} [ID: {}]: {} ({})\n",
                ticket["number"].as_str().unwrap_or("?"),
                ticket["id"].as_str().unwrap_or("?"),
                ticket["title"].as_str().unwrap_or("?"),
                ticket["status"].as_str().unwrap_or("?")
            ));
        }

        Ok(output)
    }

    /// Agent claims an unassigned ticket
    #[tool(
        description = "Claim an unassigned ticket and automatically assign it to the agent making the request. Moves ticket to 'in_progress' status."
    )]
    pub async fn claim_ticket(
        &self,
        #[tool(param)] ticket_id: String,
        #[tool(param)] message: Option<String>,
    ) -> Result<String, String> {

        // Assign to "agent"
        let response = self.build_request(
            reqwest::Method::POST,
            format!("{}/tickets/{}/assign", self.api_base_url, ticket_id)
        )
            .json(&json!({ "assignee": "agent" }))
            .send()
            .await
            .map_err(|e| format!("Failed to claim ticket: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Failed to claim ticket: {}", ticket_id));
        }

        // Update status to in_progress
        let _ = self.build_request(
            reqwest::Method::PATCH,
            format!("{}/tickets/{}/status", self.api_base_url, ticket_id)
        )
            .json(&json!({ "status": "in_progress" }))
            .send()
            .await;

        Ok(format!("✅ Claimed {} and assigned to agent", ticket_id))
    }

    /// Precisely edit ticket description
    #[tool(
        description = "Update ticket description with precise line-based or section-based operations. Supports replace_all, append, prepend, replace_lines, and replace_section operations. This is token-efficient for making surgical edits."
    )]
    pub async fn update_description(
        &self,
        #[tool(aggr)] params: UpdateDescriptionParams,
    ) -> Result<String, String> {

        let response = self.build_request(
            reqwest::Method::PATCH,
            format!("{}/tickets/{}/description", self.api_base_url, params.ticket_id)
        )
            .json(&json!({
                "description": params.content,
                "operation": params.operation.to_string()
            }))
            .send()
            .await
            .map_err(|e| format!("Failed to update description: {}", e))?;

        if !response.status().is_success() {
            return Err("Failed to update description".to_string());
        }

        Ok(format!(
            "✅ Updated description for {}\n**Operation:** {}",
            params.ticket_id, params.operation
        ))
    }

    /// Move ticket through workflow states
    #[tool(
        description = "Update ticket status. Valid statuses: backlog, todo, in_progress, review, done, blocked."
    )]
    pub async fn update_status(
        &self,
        #[tool(param)] ticket_id: String,
        #[tool(param)] status: String,
        #[tool(param)] message: Option<String>,
    ) -> Result<String, String> {

        let response = self.build_request(
            reqwest::Method::PATCH,
            format!("{}/tickets/{}/status", self.api_base_url, ticket_id)
        )
            .json(&json!({ "status": status }))
            .send()
            .await
            .map_err(|e| format!("Failed to update status: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Failed to update status: {}", error_text));
        }

        Ok(format!("✅ Moved {} to {}", ticket_id, status))
    }

    /// Add comment to ticket
    #[tool(
        description = "Add a markdown comment to a ticket. Supports @mentions for notifying team members."
    )]
    pub async fn add_comment(
        &self,
        #[tool(param)] ticket_id: String,
        #[tool(param)] content: String,
    ) -> Result<String, String> {

        let response = self.build_request(
            reqwest::Method::POST,
            format!("{}/tickets/{}/comments", self.api_base_url, ticket_id)
        )
            .json(&json!({
                "author": "agent",
                "content": content
            }))
            .send()
            .await
            .map_err(|e| format!("Failed to add comment: {}", e))?;

        if !response.status().is_success() {
            return Err("Failed to add comment".to_string());
        }

        Ok(format!("✅ Added comment to {}", ticket_id))
    }

    /// Get comments for a ticket
    #[tool(
        description = "Get all comments for a ticket. Returns array of comments with author, timestamp, and content. Useful for reading human discussion before working on a ticket."
    )]
    pub async fn get_comments(
        &self,
        #[tool(param)] ticket_id: String,
    ) -> Result<String, String> {

        let response = self.build_request(
            reqwest::Method::GET,
            format!("{}/tickets/{}/comments", self.api_base_url, ticket_id)
        )
            .send()
            .await
            .map_err(|e| format!("Failed to get comments: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Failed to get comments for ticket: {}", ticket_id));
        }

        let comments: Vec<serde_json::Value> = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        if comments.is_empty() {
            return Ok(format!("💬 No comments on {}", ticket_id));
        }

        let mut output = format!("💬 {} comments on {}\n\n", comments.len(), ticket_id);

        for comment in comments {
            let author = comment["author"].as_str().unwrap_or("unknown");
            let created_at = comment["created_at"].as_str().unwrap_or("");
            let content = comment["content"].as_str().unwrap_or("");

            output.push_str(&format!(
                "**{}** ({})\n{}\n\n---\n\n",
                author, created_at, content
            ));
        }

        Ok(output)
    }

    /// Assign or reassign ticket
    #[tool(
        description = "Assign ticket to one or more people (supports pairing). Pass empty array to unassign. Optionally include a handoff message."
    )]
    pub async fn assign_ticket(
        &self,
        #[tool(param)] ticket_id: String,
        #[tool(param)] assignees: Vec<String>,
        #[tool(param)] message: Option<String>,
    ) -> Result<String, String> {

        for assignee in &assignees {
            let response = self.build_request(
                reqwest::Method::POST,
                format!("{}/tickets/{}/assign", self.api_base_url, ticket_id)
            )
                .json(&json!({ "assignee": assignee }))
                .send()
                .await
                .map_err(|e| format!("Failed to assign ticket: {}", e))?;

            if !response.status().is_success() {
                return Err(format!("Failed to assign to {}", assignee));
            }
        }

        Ok(format!(
            "✅ Assigned {} to {}",
            ticket_id,
            assignees.join(", ")
        ))
    }

    /// Link git commit to ticket
    #[tool(
        description = "Link a git commit to a ticket for traceability. Helps track which commits are associated with which work items."
    )]
    pub async fn link_commit(
        &self,
        #[tool(param)] ticket_id: String,
        #[tool(param)] commit_hash: String,
        #[tool(param)] commit_message: Option<String>,
    ) -> Result<String, String> {

        let response = self.build_request(
            reqwest::Method::POST,
            format!("{}/tickets/{}/commits", self.api_base_url, ticket_id)
        )
            .json(&json!({
                "commit_hash": commit_hash,
                "commit_message": commit_message,
                "linked_by": "agent"
            }))
            .send()
            .await
            .map_err(|e| format!("Failed to link commit: {}", e))?;

        if !response.status().is_success() {
            return Err("Failed to link commit".to_string());
        }

        Ok(format!(
            "✅ Linked commit {} to {}",
            commit_hash, ticket_id
        ))
    }

    /// Add dependency between tickets
    #[tool(
        description = "Mark that one ticket depends on another (blocks/blocked-by relationship). The first ticket cannot be completed until the dependency is done."
    )]
    pub async fn add_dependency(
        &self,
        #[tool(param)] ticket_id: String,
        #[tool(param)] depends_on: String,
    ) -> Result<String, String> {

        let response = self.build_request(
            reqwest::Method::POST,
            format!("{}/tickets/{}/dependencies", self.api_base_url, ticket_id)
        )
            .json(&json!({ "depends_on_id": depends_on }))
            .send()
            .await
            .map_err(|e| format!("Failed to add dependency: {}", e))?;

        if !response.status().is_success() {
            return Err("Failed to add dependency".to_string());
        }

        Ok(format!(
            "✅ Added dependency: {} depends on {}",
            ticket_id, depends_on
        ))
    }

    /// Remove dependency
    #[tool(
        description = "Remove a dependency relationship between two tickets."
    )]
    pub async fn remove_dependency(
        &self,
        #[tool(param)] ticket_id: String,
        #[tool(param)] depends_on: String,
    ) -> Result<String, String> {

        let response = self.build_request(
            reqwest::Method::DELETE,
            format!(
                "{}/tickets/{}/dependencies/{}",
                self.api_base_url, ticket_id, depends_on
            )
        )
            .send()
            .await
            .map_err(|e| format!("Failed to remove dependency: {}", e))?;

        if !response.status().is_success() {
            return Err("Failed to remove dependency".to_string());
        }

        Ok(format!(
            "✅ Removed dependency: {} no longer depends on {}",
            ticket_id, depends_on
        ))
    }

    /// Get full dependency tree
    #[tool(
        description = "Get the complete dependency graph for a ticket, showing what it depends on and what depends on it."
    )]
    pub async fn get_dependency_graph(
        &self,
        #[tool(param)] ticket_id: String,
    ) -> Result<String, String> {

        let response = self.build_request(
            reqwest::Method::GET,
            format!("{}/tickets/{}", self.api_base_url, ticket_id)
        )
            .send()
            .await
            .map_err(|e| format!("Failed to get ticket: {}", e))?;

        if !response.status().is_success() {
            return Err("Ticket not found".to_string());
        }

        let data: serde_json::Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let empty_array = Vec::new();
        let deps = data["dependencies"].as_array().unwrap_or(&empty_array);
        let dependents = data["dependents"].as_array().unwrap_or(&empty_array);

        Ok(format!(
            "📊 Dependency graph for {}\n\n\
             **Dependencies (blocks):** {}\n\
             **Dependents (blocked by):** {}",
            ticket_id,
            deps.len(),
            dependents.len()
        ))
    }

    /// List available templates
    #[tool(
        description = "List all available ticket templates. Templates provide pre-filled structure for common ticket types."
    )]
    pub async fn list_templates(&self) -> Result<String, String> {
        Ok("📋 Available Templates\n\n\
            (Template system not yet implemented on backend)".to_string())
    }

    /// Create ticket from template
    #[tool(
        description = "Create a new ticket from a template with variable substitution. Templates help standardize common ticket types."
    )]
    pub async fn create_from_template(
        &self,
        #[tool(aggr)] params: CreateFromTemplateParams,
    ) -> Result<String, String> {
        Ok(format!(
            "📋 Template creation not yet supported\n\n\
             Requested template: {}",
            params.template
        ))
    }

    /// Search tickets by text
    #[tool(
        description = "Full-text search across ticket titles, descriptions, and comments. Returns matching tickets with context snippets."
    )]
    pub async fn search_tickets(
        &self,
        #[tool(param)] query: String,
        #[tool(param)] limit: Option<u64>,
    ) -> Result<String, String> {

        let response = self.build_request(
            reqwest::Method::GET,
            format!("{}/tickets/search?q={}&limit={}",
                self.api_base_url,
                urlencoding::encode(&query),
                limit.unwrap_or(20)
            )
        )
            .send()
            .await
            .map_err(|e| format!("Failed to search: {}", e))?;

        if !response.status().is_success() {
            return Err("Search failed".to_string());
        }

        let tickets: Vec<serde_json::Value> = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        if tickets.is_empty() {
            return Ok(format!("🔍 No tickets found matching '{}'", query));
        }

        let mut output = format!("🔍 Found {} tickets matching '{}'\n\n", tickets.len(), query);

        for ticket in tickets {
            output.push_str(&format!(
                "- {}: {}\n",
                ticket["number"].as_str().unwrap_or("?"),
                ticket["title"].as_str().unwrap_or("?")
            ));
        }

        Ok(output)
    }

    /// Delete a ticket (soft delete)
    #[tool(
        description = "Delete a ticket by marking it as deleted (soft delete). The ticket will no longer appear in lists or boards, but is preserved in the database for audit trail."
    )]
    pub async fn delete_ticket(
        &self,
        #[tool(param)] ticket_id: String,
    ) -> Result<String, String> {

        let response = self.build_request(
            reqwest::Method::DELETE,
            format!("{}/tickets/{}", self.api_base_url, ticket_id)
        )
            .send()
            .await
            .map_err(|e| format!("Failed to delete ticket: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Failed to delete ticket: {}", error_text));
        }

        Ok(format!("✅ Deleted ticket {}", ticket_id))
    }
}

// Use the tool_box! macro to generate list_tools and call_tool implementations
tool_box!(JilityService {
    create_ticket,
    create_tickets_batch,
    get_ticket,
    list_tickets,
    claim_ticket,
    update_description,
    update_status,
    add_comment,
    get_comments,
    assign_ticket,
    link_commit,
    add_dependency,
    remove_dependency,
    get_dependency_graph,
    list_templates,
    create_from_template,
    search_tickets,
    delete_ticket,
});

impl ServerHandler for JilityService {
    async fn list_tools(
        &self,
        _: rmcp::model::PaginatedRequestParam,
        _: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, rmcp::Error> {
        Ok(rmcp::model::ListToolsResult {
            next_cursor: None,
            tools: tool_box().list(),
        })
    }

    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParam,
        context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::CallToolResult, rmcp::Error> {
        let tool_context = rmcp::handler::server::tool::ToolCallContext::new(self, request, context);
        tool_box().call(tool_context).await
    }

    fn get_info(&self) -> InitializeResult {
        InitializeResult {
            protocol_version: ProtocolVersion::default(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: None,
                }),
                ..Default::default()
            },
            server_info: Implementation {
                name: "jility-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some(
                "Jility provides AI-native project management for humans and agents working together. \
                 Use create_ticket to add tasks, update_description for precise edits, \
                 get_ticket for full context, list_tickets to query tickets, \
                 and workflow tools like update_status, add_comment, and assign_ticket for collaboration. \
                 For dependencies, use add_dependency and get_dependency_graph.\n\n\
                 Backend URL: configured via JILITY_API_URL environment variable (default: http://localhost:3900/api)".to_string()
            ),
        }
    }
}
