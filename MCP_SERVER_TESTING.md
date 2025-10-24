# Jility MCP Server Testing Guide

## Overview

The Jility MCP server has been implemented and can be tested using JSON-RPC messages via stdio.

## What Was Implemented

### 1. Core Infrastructure
- **JilityService**: Main service struct with tool router
- **Server Runner**: Stdio-based transport using rmcp crate
- **CLI Integration**: `--mcp-server` flag added to jility CLI

### 2. MCP Tools Implemented

#### Ticket Management
- `create_ticket` - Create new ticket with all metadata
- `create_tickets_batch` - Create multiple tickets at once
- `get_ticket` - Get full ticket context (comments, dependencies, history)
- `list_tickets` - Query tickets with filters
- `claim_ticket` - Agent claims an unassigned ticket

#### Description Editing (Killer Feature!)
- `update_description` - Precise editing with operations:
  - `replace_all` - Full replacement
  - `append` - Add to end
  - `prepend` - Add to beginning
  - `replace_lines` - Replace specific line range
  - `replace_section` - Replace markdown section

#### Workflow & Collaboration
- `update_status` - Move ticket through workflow states
- `add_comment` - Add markdown comment (supports @mentions)
- `assign_ticket` - Assign to humans or agents (supports pairing)
- `link_commit` - Link git commit to ticket

#### Dependencies
- `add_dependency` - Mark ticket dependency (blocks/blocked-by)
- `remove_dependency` - Remove dependency
- `get_dependency_graph` - Get full dependency tree

#### Templates & Search
- `list_templates` - Show available ticket templates
- `create_from_template` - Create ticket from template with variables
- `search_tickets` - Full-text search

### 3. Architecture

```
jility-mcp/
├── src/
│   ├── lib.rs        # Module exports
│   ├── main.rs       # Binary entry point
│   ├── server.rs     # MCP server runner with stdio transport
│   ├── service.rs    # JilityService with all tool implementations
│   └── params.rs     # Type-safe parameter structs with JsonSchema
```

## Testing the MCP Server

### 1. Build the Project

```bash
cd /home/user/Jility
cargo build --package jility-mcp
```

### 2. Run as MCP Server

```bash
./target/debug/jility --mcp-server
```

Or:

```bash
cargo run --package jility-cli -- --mcp-server
```

### 3. Manual Testing with JSON-RPC

The MCP server communicates via JSON-RPC over stdio. You can test it by sending JSON messages:

#### Initialize Connection

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{"tools":{}},"clientInfo":{"name":"test-client","version":"1.0.0"}}}' | ./target/debug/jility --mcp-server
```

#### List Available Tools

```bash
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' | ./target/debug/jility --mcp-server
```

#### Create a Ticket

```bash
echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"create_ticket","arguments":{"title":"Test ticket","description":"This is a test","story_points":5,"assignees":["agent-1"],"labels":["backend"]}}}' | ./target/debug/jility --mcp-server
```

#### Get Ticket Details

```bash
echo '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"get_ticket","arguments":{"ticket_id":"TASK-1"}}}' | ./target/debug/jility --mcp-server
```

#### Update Description

```bash
echo '{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"update_description","arguments":{"ticket_id":"TASK-1","operation":"append","content":"\\n## Update\\n\\n- Progress made","message":"Added update"}}}' | ./target/debug/jility --mcp-server
```

### 4. Testing with Claude Code

#### Setup

1. Make sure the `.mcp.json` file exists in your project root:

```json
{
  "mcpServers": {
    "jility": {
      "command": "jility",
      "args": ["--mcp-server"],
      "env": {
        "JILITY_MODE": "local"
      }
    }
  }
}
```

2. Build and install the jility CLI:

```bash
cargo build --release
sudo ln -s /home/user/Jility/target/release/jility /usr/local/bin/jility
```

3. Restart Claude Code to pick up the new MCP server

#### Using in Claude Code

Once configured, you can ask Claude Code to:

```
Create a new ticket titled "Implement authentication" with 5 story points
```

Claude will use the `create_ticket` tool:

```json
{
  "name": "create_ticket",
  "arguments": {
    "title": "Implement authentication",
    "story_points": 5
  }
}
```

## Current Implementation Status

### ✅ Completed

- [x] MCP server infrastructure with rmcp crate
- [x] All 17 core tools implemented
- [x] Type-safe parameter schemas with JsonSchema
- [x] Stdio transport for communication
- [x] CLI integration with --mcp-server flag
- [x] .mcp.json configuration file
- [x] Comprehensive tool descriptions for AI agents
- [x] Error handling with helpful messages

### 🚧 Stub Implementation (To Be Completed)

Currently, all tools return **mock data** instead of interacting with a real database. This allows the MCP server to be tested immediately without requiring the full database layer.

To complete the implementation:

1. **Database Layer**: Implement SeaORM entities and migrations in `jility-core`
2. **Database Connection**: Update `server.rs` to connect to `.jility/data.db`
3. **Tool Logic**: Replace mock responses in `service.rs` with actual database queries
4. **Validation**: Add proper validation for status transitions, dependencies, etc.
5. **Event Sourcing**: Record all changes in `ticket_changes` table

## Next Steps

### Phase 1: Database Integration

```rust
// In server.rs
let db_path = current_dir.join(".jility/data.db");
let db = connect_to_database(&db_path).await?;
let service = JilityService::new(db, current_dir)?;
```

### Phase 2: Real Tool Implementation

Example for `create_ticket`:

```rust
async fn create_ticket(
    &self,
    Parameters(params): Parameters<CreateTicketParams>,
) -> Result<CallToolResult, McpError> {
    // Generate UUID for ticket
    let ticket_id = Uuid::new_v4();

    // Get next ticket number
    let ticket_number = get_next_ticket_number(&self.db, &self.project_id).await?;

    // Insert ticket
    let ticket = ticket::ActiveModel {
        id: Set(ticket_id),
        title: Set(params.title.clone()),
        description: Set(params.description.unwrap_or_default()),
        status: Set(params.status.unwrap_or_else(|| "backlog".to_string())),
        story_points: Set(params.story_points),
        created_at: Set(Utc::now()),
        created_by: Set("agent-1".to_string()), // TODO: Get from context
        ..Default::default()
    };

    let result = ticket.insert(&self.db).await?;

    // Record creation event
    record_change(&self.db, ticket_id, ChangeType::Created, None, None).await?;

    // Format success message
    Self::success(format!("✅ Created ticket {}", result.number))
}
```

### Phase 3: Advanced Features

- Dependency cycle detection
- Full-text search (FTS5 for SQLite)
- Template loading and variable substitution
- Git integration for commit linking

## Architecture Benefits

### 1. Type Safety
All parameters use `schemars::JsonSchema` which generates JSON schemas automatically. Claude Code can validate parameters before calling tools.

### 2. Token Efficiency
The `update_description` tool supports line-based and section-based editing, which is much more token-efficient than sending the entire description.

### 3. Comprehensive Context
The `get_ticket` tool bundles everything an agent needs: ticket data, comments, dependencies, history, and linked commits.

### 4. Event Sourcing
All changes will be tracked in the `ticket_changes` table, providing full auditability and time-travel debugging.

## Troubleshooting

### MCP Server Won't Start

Check logs (goes to stderr):
```bash
RUST_LOG=info ./target/debug/jility --mcp-server
```

### Tools Not Showing in Claude Code

1. Verify `.mcp.json` exists at project root
2. Check that `jility` binary is in PATH
3. Restart Claude Code
4. Check Claude Code MCP server status

### Invalid JSON-RPC Messages

The server expects newline-delimited JSON messages. Each message must be a complete JSON object on a single line.

## Tool Documentation

All tools have comprehensive descriptions that Claude Code can read. To see what each tool does:

```bash
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' | ./target/debug/jility --mcp-server | jq '.result.tools[] | {name: .name, description: .description}'
```

## Example Session

```bash
# Start server
./target/debug/jility --mcp-server

# In another terminal, send commands:

# 1. Initialize
echo '{"jsonrpc":"2.0","id":1,"method":"initialize",...}'

# 2. List tools
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list",...}'

# 3. Create ticket
echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"create_ticket",...}}'

# 4. List tickets
echo '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"list_tickets",...}}'

# 5. Update status
echo '{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"update_status",...}}'
```

## Summary

The Jility MCP server is **fully architected and ready for database integration**. All 17 tools are implemented with:

- ✅ Type-safe parameters
- ✅ JSON schema generation
- ✅ Comprehensive descriptions
- ✅ Error handling
- ✅ User-friendly responses
- ✅ CLI integration
- ✅ Claude Code configuration

The next step is to implement the database layer in `jility-core` and connect it to the MCP tools.
