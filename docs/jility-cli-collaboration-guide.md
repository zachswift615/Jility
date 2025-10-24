# Jility CLI Reference - Human & Agent Collaboration

## Overview
The Jility CLI treats humans and agents as equal team members. Both can create tickets, update descriptions, claim work, and collaborate seamlessly.

---

## Assignment & Claiming

### Assign ticket to yourself (human)
```bash
jility ticket assign TASK-123 --to=alice
# or
jility ticket claim TASK-123
```

### Assign ticket to an agent
```bash
jility ticket assign TASK-123 --to=agent-1
```

### Assign to multiple people (pairing)
```bash
jility ticket assign TASK-123 --to=alice,agent-1
```

### Handoff with context
```bash
jility ticket assign TASK-123 --to=agent-1 --message "I've set up the basic structure. Please implement the validation logic."
```

### Unassign ticket
```bash
jility ticket unassign TASK-123
```

### Check who's assigned
```bash
jility ticket show TASK-123 --field=assignees
# Output: alice, agent-1
```

---

## Team Status

### View everyone's current work
```bash
jility team status

# Output:
# 👤 Alice     • TASK-123 (In Progress)  • Active 30s ago
# 👤 Bob       • TASK-125 (Review)       • Active 5m ago
# 🤖 Agent-1   • TASK-124 (In Progress)  • Active 1m ago
# 🤖 Agent-2   • Idle                    • Last active 2h ago
```

### View specific person's tickets
```bash
jility ticket list --assignee=alice
jility ticket list --assignee=agent-1
```

### View your own tickets
```bash
jility ticket list --assignee=me
# or
jility my-tickets
```

### View unassigned tickets
```bash
jility ticket list --unassigned
```

### View tickets by actor type
```bash
jility ticket list --assigned-to=humans
jility ticket list --assigned-to=agents
```

---

## Collaboration Workflows

### Human creates tickets, agents implement
```bash
# Human creates tickets
jility ticket create --title "Add password validation" \
  --description "## Requirements\n- Min 8 chars\n- Must have number\n- Must have special char" \
  --points 3 \
  --labels backend

# Human assigns to agent
jility ticket assign TASK-123 --to=agent-1 --message "Please implement using regex validation"

# Agent works on it (via MCP)
# Agent updates and comments
# Agent marks ready for review
jility ticket move TASK-123 --to=review
jility ticket comment TASK-123 "@alice Ready for your review"

# Human reviews
jility ticket show TASK-123
jility ticket comment TASK-123 "Looks good! Just add tests for edge cases"

# Agent adds tests
# Agent marks done
jility ticket move TASK-123 --to=done
```

### Agent gets stuck, human helps
```bash
# Agent working on ticket
# Agent realizes it needs help
jility ticket comment TASK-123 "@alice I'm not sure how to handle the OAuth flow here. Can you provide guidance?"

# Human sees notification
jility ticket show TASK-123

# Human provides guidance without taking over
jility ticket comment TASK-123 "Check out the implementation in auth.rs:42. Use the same pattern for token refresh."

# Or human takes over temporarily
jility ticket assign TASK-123 --to=alice,agent-1 --message "Let me pair with you on this"
```

### Agent breaks down ticket, human reviews plan
```bash
# Agent analyzes complex ticket
jility ticket show TASK-100

# Agent creates sub-tickets
jility ticket create --title "Add database schema for OAuth" --parent TASK-100 --assignee=agent-1
jility ticket create --title "Implement OAuth endpoints" --parent TASK-100 --assignee=agent-2  
jility ticket create --title "Add OAuth UI components" --parent TASK-100 --assignee=alice

# Human reviews the breakdown
jility ticket list --parent=TASK-100
jility ticket comment TASK-100 "Good breakdown! Let's also add a ticket for error handling"

# Human adds missing ticket
jility ticket create --title "Add OAuth error handling" --parent TASK-100 --assignee=agent-1
```

### Human and agent pair on ticket
```bash
# Assign both
jility ticket assign TASK-123 --to=alice,agent-1

# Both can comment and update
# Human guides strategy
jility ticket comment TASK-123 "Let's use the builder pattern here"

# Agent implements
# Agent links commits
jility ticket link-commit TASK-123 abc123f

# Human reviews in real-time
# Agent updates based on feedback
```

---

## Communication

### Add comment
```bash
jility ticket comment TASK-123 "Great progress! Just needs error handling"
```

### @ mention someone
```bash
jility ticket comment TASK-123 "@agent-1 Please add tests for this"
jility ticket comment TASK-123 "@alice Can you review the API design?"
```

### View conversation
```bash
jility ticket show TASK-123 --with-comments

# or just comments
jility ticket comments TASK-123
```

### Mark ticket as needing human attention
```bash
jility ticket label TASK-123 --add needs-human-review
```

---

## Filtering & Queries

### Show tickets created by agents
```bash
jility ticket list --created-by=agent
```

### Show tickets created by humans
```bash
jility ticket list --created-by=human
```

### Show tickets ready for human review
```bash
jility ticket list --status=review --assigned-to=agents
```

### Show blocked tickets
```bash
jility ticket list --status=blocked
```

### Show tickets needing help
```bash
jility ticket list --label=needs-help
```

---

## Context & History

### View full ticket context (what agents see)
```bash
jility ticket context TASK-123

# Output includes:
# - Ticket details
# - Full description with version history
# - All comments
# - Linked commits
# - Related tickets
# - Dependencies
# - Previous assignees
```

### View assignment history
```bash
jility ticket history TASK-123 --field=assignees

# Output:
# v4 - Assigned to agent-1 by alice (2h ago): "Please implement validation"
# v3 - Assigned to alice by alice (4h ago)
# v2 - Assigned to agent-1 by system (6h ago)
# v1 - Created by agent-1 (8h ago)
```

### View who worked on what
```bash
jility stats --by-person

# Output:
# 👤 Alice
#   - 5 tickets created
#   - 8 tickets completed
#   - 23 story points delivered
#
# 🤖 Agent-1
#   - 12 tickets created
#   - 15 tickets completed
#   - 42 story points delivered
```

---

## Agent-Specific Commands

### For AI agents to identify themselves
```bash
# Agent sets its identity for the session
export JILITY_ACTOR=agent-1
# or
jility config set actor agent-1

# Now all commands are attributed to agent-1
jility ticket create --title "Add validation"
# Created by: agent-1
```

### Agent asks for clarification
```bash
jility ticket comment TASK-123 "@alice I need clarification on the requirements. Should this support OAuth2 or OAuth1?"
jility ticket label TASK-123 --add needs-clarification
```

### Agent marks ready for handoff
```bash
jility ticket move TASK-123 --to=review
jility ticket comment TASK-123 "Implementation complete. @alice please review"
jility ticket assign TASK-123 --add alice  # Keep agent assigned too
```

### Agent detects blockers
```bash
jility ticket move TASK-123 --to=blocked
jility ticket comment TASK-123 "Blocked: Waiting for TASK-120 (database schema) to be completed"
jility ticket add-dependency TASK-123 --depends-on TASK-120
```

---

## Human-Specific Commands

### Human reviews agent work
```bash
# See all tickets ready for review from agents
jility review-queue

# Review specific ticket
jility ticket show TASK-123
jility ticket comment TASK-123 "Looks good! One small request: add error handling for timeout"

# Approve and merge
jility ticket move TASK-123 --to=done
```

### Human provides feedback
```bash
jility ticket comment TASK-123 "Great implementation! For future tickets, please add inline documentation for complex logic"
```

### Human adjusts agent work
```bash
# Human sees agent went in wrong direction
jility ticket comment TASK-123 "Let's change approach: use webhooks instead of polling"
jility ticket label TASK-123 --add approach-change

# Or human takes over
jility ticket assign TASK-123 --to=alice --message "I'll refactor this to use webhooks"
```

---

## Configuration

### Configure team members
```bash
# Initialize team config
jility team init

# Add human
jility team add --name alice --email alice@example.com --type human

# Add agent
jility team add --name agent-1 --type agent

# List team
jility team list

# Output:
# 👤 alice (alice@example.com) - Human
# 👤 bob (bob@example.com) - Human
# 🤖 agent-1 - Agent
# 🤖 agent-2 - Agent
```

### Set notification preferences
```bash
# Human wants notifications when agents @ mention them
jility config set notify.on-mention true

# Human wants summary of agent work daily
jility config set notify.daily-summary true

# Agent should notify human when stuck
jility config set agent.notify-on-block true
```

---

## Quick Reference

| Command | Description |
|---------|-------------|
| `jility ticket assign TASK-X --to=NAME` | Assign to human or agent |
| `jility ticket claim TASK-X` | Assign to yourself |
| `jility ticket assign TASK-X --to=A,B` | Pair assignment |
| `jility my-tickets` | Your assigned tickets |
| `jility team status` | See everyone's work |
| `jility review-queue` | Tickets ready for review |
| `jility ticket comment TASK-X "msg"` | Add comment |
| `jility ticket comment TASK-X "@name msg"` | @ mention someone |
| `jility ticket context TASK-X` | Full context dump |
| `jility ticket list --assigned-to=agents` | Filter by actor type |

---

## Example Workflow: Building a Feature Together

```bash
# Day 1: Human plans, creates tickets
jility ticket create --title "Add user profile feature" \
  --description "## Goal\nAllow users to edit their profile\n\n## Sub-tasks\nWill be broken down" \
  --points 8

# Human asks agent to break it down
jility ticket assign TASK-200 --to=agent-1 --message "Please analyze and create sub-tickets"

# Agent analyzes and creates sub-tickets
# (via MCP in the background)

# Human reviews the plan
jility ticket list --parent=TASK-200
# TASK-201: Add profile API endpoints (agent-1, 3pts)
# TASK-202: Create profile UI component (alice, 3pts)  
# TASK-203: Add profile tests (agent-2, 2pts)

jility ticket comment TASK-200 "Good breakdown! Let's get started"

# Day 2: Parallel work
# Agent-1 works on TASK-201
# Alice works on TASK-202
# Agent-2 works on TASK-203

# Alice needs help with styling
jility ticket comment TASK-202 "@agent-2 Can you help with the CSS layout?"
jility ticket assign TASK-202 --add agent-2

# Both work together
# ...

# Day 3: Review and merge
jility review-queue
# Shows TASK-201, TASK-203 ready for review

jility ticket show TASK-201
jility ticket comment TASK-201 "Great work! Let's merge"
jility ticket move TASK-201 --to=done

# TASK-202 needs small fix
jility ticket comment TASK-202 "Almost there - fix the responsive layout"

# Agent-2 fixes
jility ticket move TASK-202 --to=done

# Feature complete!
jility ticket move TASK-200 --to=done
```

---

## Tips for Effective Human/Agent Collaboration

1. **Be explicit in handoffs** - Provide clear context when assigning work
2. **Use @ mentions** - Call attention to specific people/agents
3. **Review agent work** - Agents benefit from human feedback
4. **Let agents own routine tasks** - Focus your time on creative/strategic work
5. **Pair on complex problems** - Assign both human and agent for collaboration
6. **Provide feedback loops** - Help agents learn by commenting on their work
7. **Use labels** - "needs-human-review", "needs-clarification", etc.
8. **Check the review queue daily** - Stay on top of agent-completed work

---

## MCP Integration

For AI agents using the MCP protocol, all these CLI commands have equivalent MCP methods:

```typescript
// Agent assigns ticket to itself
claim_ticket("TASK-123", "agent-1")

// Agent requests human help  
add_comment("TASK-123", "@alice I need help with the OAuth flow")

// Agent gets full context
get_ticket_context("TASK-123")
// Returns: ticket + all comments + related work + human guidance

// Agent marks ready for human
update_ticket_status("TASK-123", "review")
add_comment("TASK-123", "@alice Ready for your review")
```

The MCP server automatically handles the actor identity and attribution, making it seamless for agents to collaborate with humans through the same interface.
