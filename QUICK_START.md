# Jility CLI - Quick Start Guide

## Installation (After Build Fix)

```bash
cargo build --release
cargo install --path crates/jility-cli
```

## Initialize Project

```bash
cd your-project
jility init
```

## Common Commands

### Create Tickets

```bash
# Simple ticket
jility ticket create --title "Fix login bug"

# Full featured ticket
jility ticket create \
  --title "Implement OAuth" \
  --description "Add Google and GitHub OAuth providers" \
  --story-points 8 \
  --assignees "alice,agent-1" \
  --labels "auth,security" \
  --priority high
```

### View Tickets

```bash
# List all tickets
jility ticket list

# Filter by status
jility ticket list --status in-progress

# Filter by assignee
jility ticket list --assignee alice

# JSON output
jility ticket list --format json
```

### Show Ticket Details

```bash
# Pretty format (default)
jility ticket show TASK-1

# JSON format
jility ticket show TASK-1 --format json
```

### Modify Tickets

```bash
# Update metadata
jility ticket update TASK-1 --title "New title" --story-points 13

# Edit description with $EDITOR
jility ticket edit TASK-1

# Change status
jility ticket move TASK-1 --to in-progress
jility ticket move TASK-1 --to done

# Assign/unassign
jility ticket assign TASK-1 --to alice
jility ticket assign TASK-1 --to agent-1  # Add second assignee
jility ticket assign TASK-1 --to alice --remove  # Remove assignee
```

### Collaboration

```bash
# Add comment
jility ticket comment TASK-1 "This is ready for review"

# View version history
jility ticket history TASK-1
```

## Typical Workflows

### Solo Developer
```bash
jility ticket create --title "Add feature X"
jility ticket show TASK-1
jility ticket move TASK-1 --to in-progress
# ... work on feature ...
jility ticket comment TASK-1 "Implemented core functionality"
jility ticket move TASK-1 --to done
```

### Human + Agent Pairing
```bash
# Create and assign to agent
jility ticket create --title "Write unit tests" --assignees "agent-1"

# Agent implements (via MCP)
jility --mcp-server  # In separate terminal

# Review agent's work
jility ticket show TASK-2
jility ticket comment TASK-2 "Add edge case tests"

# Assign to both for collaboration
jility ticket assign TASK-2 --to alice

# Mark complete
jility ticket move TASK-2 --to done
```

### Team Sprint
```bash
# Create sprint tickets
jility ticket create --title "Feature A" --story-points 5
jility ticket create --title "Feature B" --story-points 8
jility ticket create --title "Fix bugs" --story-points 3

# Assign work
jility ticket assign TASK-1 --to alice
jility ticket assign TASK-2 --to bob
jility ticket assign TASK-3 --to agent-1

# Track progress
jility ticket list --status in-progress
jility ticket list --assignee alice

# Update estimates
jility ticket update TASK-1 --story-points 8  # Took longer than expected
```

## Tips & Tricks

### Environment Variables
```bash
export EDITOR=code  # Use VS Code for editing
export EDITOR=nano  # Use nano instead of vim
```

### Shell Aliases
```bash
alias jt='jility ticket'
alias jtc='jility ticket create'
alias jtl='jility ticket list'
alias jts='jility ticket show'

# Then use:
jtc --title "Quick ticket"
jtl --status todo
jts TASK-1
```

### JSON Processing with jq
```bash
# Get all in-progress ticket IDs
jility ticket list --status in-progress --format json | jq -r '.[] | .ticket_number'

# Count tickets by status
jility ticket list --format json | jq 'group_by(.status) | map({status: .[0].status, count: length})'

# Get unassigned tickets
jility ticket list --format json | jq '.[] | select(.assignees | length == 0)'
```

### Scripting
```bash
# Bulk create tickets from file
while IFS= read -r title; do
  jility ticket create --title "$title"
done < tickets.txt

# Auto-assign based on workload
ASSIGNEE=$(jility ticket list --format json | jq -r '...' )  # Custom logic
jility ticket assign TASK-10 --to "$ASSIGNEE"
```

## Status Values

Valid status values for `--status` and `--to` flags:
- `backlog`
- `todo`
- `in-progress` (or `inprogress`, `in_progress`)
- `in-review` (or `inreview`, `in_review`)
- `done`
- `cancelled` (or `canceled`)

## Priority Values

Valid priority values for `--priority` flag:
- `low`
- `medium` (or `med`)
- `high`
- `urgent`

## File Locations

- Database: `.jility/data.db` (SQLite)
- Config: `.jility/config.toml` (future)
- Temp files: System temp directory (for $EDITOR)

## Help

```bash
jility --help
jility ticket --help
jility ticket create --help
```

---

Happy project managing!
