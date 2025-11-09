# Agent Power Tools

This project has a powerful code indexing system available via the `powertools` binary.

## Power Tools Commands

The powertools binary is located at: `./powertools-cli/target/release/powertools`

### MCP Server Mode

Powertools can run as an MCP (Model Context Protocol) server, making all commands available as first-class tools in Claude Code:

```bash
# Run as MCP server (communicates via stdio)
./powertools-cli/target/release/powertools --mcp-server
```

**Claude Code Integration:**
To enable MCP integration, create a `.mcp.json` file at your project root:

```json
{
  "mcpServers": {
    "powertools": {
      "command": "powertools",
      "args": ["--mcp-server"]
    }
  }
}
```

**Important:** The file must be named `.mcp.json` (not `mcp_settings.json`) and placed at the project root. This file can be committed to git for team collaboration.

After creating the file and restarting Claude Code, the following tools will be available:

**Core Navigation Tools:**
- `index_project` - Index a project for semantic navigation (auto-installs indexers)
- `goto_definition` - Find where a symbol is defined
- `find_references` - Find all references to a symbol (with pagination)
- `search_ast` - Search for code patterns using tree-sitter queries (with pagination)
- `list_functions` - List all functions in a file or directory (with pagination)
- `list_classes` - List all classes, structs, or interfaces (with pagination)
- `project_stats` - Get codebase statistics

**File Watcher Tools (NEW in v0.2.0):**
- `watcher_start` - Start automatic re-indexing when files change
- `watcher_stop` - Pause automatic re-indexing
- `get_watcher_status` - Check if watcher is running and get project info

**Batch Operations Tools (NEW in v0.3.0, Production-Ready v0.3.1):**
- `batch_replace` - Replace text across multiple files using regex patterns with preview
  - **Production-Tested**: Validated on real projects (TanStack Query, poetry-core, nlohmann/json)
  - **Supports**: Regex capture groups ($1, $2), file glob filtering, risk assessment
  - **Safe by default**: Preview mode prevents accidental mass edits

**Important: The file watcher starts AUTOMATICALLY when the MCP server starts!** This means:
- Indexes stay fresh as the user edits code
- You don't need to manually re-index after file changes
- If you get "symbol not found" errors, the index might be rebuilding (wait 2-5s and retry)

**When to use watcher tools:**
- **Use `watcher_stop`** before bulk operations (e.g., mass file edits, git operations, npm install) to avoid re-index spam
- **Use `watcher_start`** after bulk operations to resume automatic indexing
- **Use `get_watcher_status`** to check if the watcher is running or to show the user what's being monitored
- **DO NOT** manually call `index_project` on every file change - the watcher handles this automatically!

**When to use batch_replace:**
- **ALWAYS use `preview=true` FIRST** - Never apply batch replacements without previewing!
- **Use for repetitive edits** - Replace patterns across multiple files in one operation
- **Supports regex** - Use capture groups like $1, $2 for complex replacements
- **File filtering** - Use `file_pattern` param (e.g., "*.ts", "**/*.rs") to limit scope
- **Examples:**
  - Fix typos across codebase: `batch_replace("recieve", "receive", preview=true)`
  - Update API URLs: `batch_replace("api\\.old\\.com", "api.new.com", file_pattern="**/*.ts", preview=true)`
  - Add optional chaining: `batch_replace("user\\.([a-zA-Z]+)", "user?.$1", file_pattern="**/*.ts", preview=true)`

**Pagination Support (v0.1.3+):**
All MCP tools that return lists support pagination to prevent token limit errors:
- `limit` parameter: Maximum results to return (default: 100)
- `offset` parameter: Number of results to skip (default: 0)
- Response includes: `count` (total), `has_more` (boolean), and result data

Example: On a project with 1,438 functions, `list_functions` with `limit=100` returns only 100 results instead of exceeding token limits.

### Available Commands:

#### Semantic Navigation (SCIP-based)
```bash
# Index a project (auto-detects TypeScript, JavaScript, Python, Rust)
./powertools-cli/target/release/powertools index --auto-install

# Index only specific languages
./powertools-cli/target/release/powertools index --languages typescript python

# Go to definition (returns JSON with file path, line, column)
./powertools-cli/target/release/powertools definition src/file.ts:10:5 --format json -p /path/to/project

# Find all references to a symbol
./powertools-cli/target/release/powertools references myFunction --format json -p /path/to/project

# Include declarations in references
./powertools-cli/target/release/powertools references myFunction --include-declarations --format json
```

**When to use:**
- **Use `index_project`** only when: (1) Starting work on a new project for the first time, OR (2) The watcher is stopped and you need to manually rebuild
- **DO NOT use `index_project`** repeatedly - the file watcher (v0.2.0+) keeps indexes fresh automatically!
- **Use `goto_definition`** when you need to find where a function/variable is defined
- **Use `find_references`** when you need to find all usages of a symbol
- **Note:** If queries fail with "symbol not found", wait 2-5 seconds for the watcher to finish re-indexing, then retry

**Output:** All commands support `--format json` which returns structured data perfect for parsing.

#### Tree-sitter Pattern Matching
```bash
# Search for AST patterns using tree-sitter queries
./powertools-cli/target/release/powertools search-ast "(function_item) @func" -p src/

# Find all functions in a project
./powertools-cli/target/release/powertools functions --format json

# Find all classes/structs
./powertools-cli/target/release/powertools classes --format json

# Get project statistics
./powertools-cli/target/release/powertools stats

# Get help
./powertools-cli/target/release/powertools --help
```

### Example Tree-sitter Patterns:
- Rust functions: `(function_item) @func`
- TypeScript functions: `(function_declaration) @func`
- Python functions: `(function_definition) @func`
- Find async functions: `(async_function) @func`
- Find classes: `(class_declaration) @class`

### Supported Languages:
- **TypeScript**: Full semantic navigation via scip-typescript
- **JavaScript**: Full semantic navigation via scip-typescript (requires tsconfig.json with `allowJs: true`)
- **Python**: Full semantic navigation via scip-python
- **Rust**: Full semantic navigation via rust-analyzer
- **C++**: Full semantic navigation via scip-clang (requires `compile_commands.json`)

**C++ Requirements:**
- Must have `compile_commands.json` (compilation database)
- Generate with CMake: `cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON ..`
- Or use Bear for Make: `bear -- make`
- scip-clang auto-installs to `~/.local/bin`

**Multi-language projects:** Powertools automatically detects and indexes all languages in a project. For example, a project with both TypeScript and Python will generate both `index.typescript.scip` and `index.python.scip`, and queries will search across both.

### Output Formats:
Use `--format json` for structured data that's easy to parse.

## Workshop CLI Integration

This project uses Workshop, a persistent context tool. At the start of each session, Workshop context is automatically loaded. At the end of each session, a summary is automatically saved.

## Workshop Commands

**Use Workshop liberally throughout the session to:**
- Record decisions: `workshop decision "<text>" -r "<reasoning>"`
- Document gotchas: `workshop gotcha "<text>" -t tag1 -t tag2`
- Add notes: `workshop note "<text>"`
- Track preferences: `workshop preference "<text>" --category code_style`
- Manage state: `workshop goal add "<text>"` and `workshop next "<text>"`

**Query context (use these frequently!):**
- `workshop why "<topic>"` - THE KILLER FEATURE! Answers "why did we do X?" - prioritizes decisions with reasoning
- `workshop context` - View session summary
- `workshop search "<query>"` - Find relevant entries
- `workshop recent` - Recent activity
- `workshop summary` - Activity overview
- `workshop sessions` - View past session history
- `workshop session last` - View details of the most recent session

**Important:** Workshop helps maintain continuity across sessions. Document architectural decisions, failed approaches, user preferences, and gotchas as you discover them.

**Best Practice:** When you wonder "why did we choose X?" or "why is this implemented this way?", run `workshop why "X"` first before asking the user!

## Importing Past Sessions

Workshop can import context from past Claude Code sessions stored in JSONL transcript files:

- **When to suggest:** If the user mentions wanting context from previous sessions, or asks "why" questions that might be answered by historical context, suggest running `workshop import --execute`
- **First-time import:** Always ask the user before running import for the first time - it can extract hundreds of entries from historical sessions
- **What it does:** Analyzes JSONL transcripts and automatically extracts decisions, gotchas, and preferences from past conversations
- **Command:** `workshop import --execute` (without --execute it's just a preview)
- **Location:** By default, imports from the current project's JSONL files in `~/.claude/projects/`

**Important:** You have permission to run `workshop import --execute`, but always ask the user first, especially if import has never been run in this project. Let them decide if they want to import historical context.


# Agent Power Tools

**Stop using hand tools for code navigation. Powertools gives Claude IDE-level semantic navigation.**

Grep is a hand tool. Text search is guessing. Pattern matching finds false positives. **Powertools upgrades you to semantic navigation** — the same technology that powers VS Code's "Go to Definition" and "Find All References", now available to Claude Code.

---

## Core Principles

**Precision over pattern matching**
- Know the exact definition location, not just text matches
- Find semantic references, not string occurrences
- Navigate by meaning, not by regex

**Evidence over exploration**
- Jump directly to the source instead of hunting through files
- See all usages instantly instead of searching manually
- Refactor with confidence instead of hoping you found everything

**Speed over thoroughness**
- Index once, navigate infinitely
- Query in milliseconds, not minutes
- Let the compiler-grade indexers do the heavy lifting

**Mandatory when they're the best tool**
- If powertools can do it semantically, use powertools
- If you need cross-file navigation, ALWAYS use semantic tools
- If you're refactoring, ALWAYS preview before applying

---

## MANDATORY Usage Rules

### Code Navigation

**NEVER grep for definitions — ALWAYS use `goto_definition`**
```
❌ grep -r "function myFunc"        # Hand tool: finds comments, strings, false matches
✅ goto_definition("src/file.ts:42:10")  # Power tool: finds THE definition
```

**NEVER search manually for usages — ALWAYS use `find_references`**
```
❌ grep -r "myVariable"              # Hand tool: finds every string match
✅ find_references("myVariable")     # Power tool: finds semantic references only
```

**NEVER pattern match for code structures — ALWAYS use `search_ast`**
```
❌ grep -r "async function"          # Hand tool: misses arrow functions, class methods
✅ search_ast("(async_function) @f") # Power tool: finds ALL async functions by AST
```

### Refactoring Operations

**ALWAYS use `rename_symbol` for renaming across files**
```
❌ batch_replace("oldName", "newName")    # Dangerous: renames strings, comments, everything
✅ rename_symbol(file, line, col, "newName", preview=true)  # Safe: semantic-aware renaming
```

**ALWAYS preview before applying batch operations**
```
✅ batch_replace("pattern", "replacement", preview=true)  # See what changes first
✅ rename_symbol(..., preview=true)                       # Review before execution
✅ inline_variable(..., preview=true)                     # Verify correctness
```

**NEVER manually find-and-replace for refactoring**
- Use `rename_symbol` for renaming variables/functions/classes
- Use `inline_variable` for inlining constants
- Use `batch_replace` only for text patterns (URLs, typos, comments)

### Indexing

**ALWAYS let the file watcher handle re-indexing**
```
❌ index_project() after every file change   # Wastes time, watcher does this automatically
✅ Trust the watcher (starts automatically)  # It re-indexes changed files in background
✅ watcher_stop() before bulk operations     # Pause during git checkout, npm install, etc.
```

---

## Quick Start

### 1. MCP Integration

Create `.mcp.json` at your project root:

```json
{
  "mcpServers": {
    "powertools": {
      "command": "powertools",
      "args": ["--mcp-server"]
    }
  }
}
```

**Important:** File must be named `.mcp.json` (not `mcp_settings.json`) at project root. Commit to git for team collaboration.

### 2. Restart Claude Code

After creating `.mcp.json`, restart Claude Code. The file watcher starts automatically — indexes stay fresh as you code.

---

## Available Powertools

### Semantic Navigation Tools

**Core navigation tools (work across files):**

- **`index_project`** - Build semantic indexes for a project (run once)
  - Auto-installs language indexers (scip-typescript, scip-python, rust-analyzer, scip-clang)
  - Detects all languages automatically
  - Only needed: (1) First time on new project, OR (2) Watcher is stopped

- **`goto_definition`** - Find where a symbol is defined
  - Input: file:line:column location
  - Output: Exact definition location with file path
  - Works across files, modules, packages

- **`find_references`** - Find all references to a symbol
  - Input: Symbol name or file:line:column
  - Output: All semantic usages (not text matches)
  - Supports pagination (limit/offset for large results)

- **`search_ast`** - Search for code patterns using tree-sitter
  - Input: Tree-sitter query (e.g., `(function_declaration) @func`)
  - Output: Structured AST matches
  - Much more precise than regex

- **`list_functions`** - List all functions in a file/directory
  - Extracts function names, signatures, locations
  - Supports pagination

- **`list_classes`** - List all classes/structs/interfaces
  - Finds type definitions across codebase
  - Supports pagination

- **`project_stats`** - Get codebase statistics
  - File counts, line counts, languages detected

### Refactoring Tools (NEW in v0.4.0)

**Production-tested semantic refactoring:**

- **`rename_symbol`** - Rename symbols across entire codebase
  - **Semantic-aware**: Only renames the actual symbol, not strings/comments
  - **Cross-file**: Updates all references in all files
  - **Import-aware**: Updates import statements automatically
  - **Safe**: ALWAYS preview first (preview=true)
  - **Tested**: Production-validated on TypeScript, Rust, Python, C++ projects

  ```python
  # Example: Rename a function across entire codebase
  rename_symbol(
      file="src/utils.ts",
      line=42,
      column=10,
      new_name="processData",
      preview=true  # ALWAYS preview first!
  )
  ```

- **`inline_variable`** - Inline variables by replacing usages
  - **Safe**: Only works on const/immutable variables
  - **Smart**: Checks for side effects before inlining
  - **Preview**: ALWAYS preview first
  - **Limitations**: Currently single-file only (cross-file coming in v0.5.0)

  ```python
  # Example: Inline a constant
  inline_variable(
      file="src/app.ts",
      line=15,
      column=7,
      preview=true  # ALWAYS preview first!
  )
  ```

### Batch Operations Tools (Production-Ready v0.3.1)

**Text-based mass edits:**

- **`batch_replace`** - Replace text across multiple files using regex
  - **Use for**: Typos, URL updates, copyright notices, comment fixes
  - **Don't use for**: Renaming code (use `rename_symbol` instead)
  - **Features**: Regex capture groups ($1, $2), file glob filtering, risk assessment
  - **Safe**: Preview mode prevents accidental mass edits
  - **Tested**: Validated on TanStack Query (18 files, 138 changes), poetry-core (74 files, 589 changes)

  ```python
  # Example: Fix typos across codebase
  batch_replace(
      pattern="recieve",
      replacement="receive",
      preview=true  # ALWAYS preview first!
  )

  # Example: Update API URLs with capture groups
  batch_replace(
      pattern=r"api\.old\.com/([a-z]+)",
      replacement=r"api.new.com/$1",
      file_pattern="**/*.ts",
      preview=true
  )
  ```

### File Watcher Tools (v0.2.0)

**Automatic re-indexing (starts automatically on MCP server start):**

- **`watcher_start`** - Start automatic re-indexing
  - Usually NOT needed (starts automatically)
  - Use after manually stopping watcher

- **`watcher_stop`** - Pause automatic re-indexing
  - Use before: git checkout, npm install, mass file edits
  - Prevents re-index spam during bulk operations

- **`get_watcher_status`** - Check if watcher is running
  - Shows project root being monitored
  - Useful for debugging "symbol not found" errors

**Important:** The watcher starts automatically when MCP server starts. If you get "symbol not found" errors, wait 2-5 seconds for re-indexing to complete, then retry.

---

## Supported Languages

### Full Semantic Navigation (SCIP-based)

These languages have full cross-file semantic support via SCIP indexers:

- **TypeScript** - via scip-typescript (auto-installed)
- **JavaScript** - via scip-typescript (requires tsconfig.json with `allowJs: true`)
- **Python** - via scip-python (auto-installed)
- **Rust** - via rust-analyzer (auto-installed)
- **C++** - via scip-clang (auto-installed, requires `compile_commands.json`)

**C++ Requirements:**
- Must have `compile_commands.json` (compilation database)
- Generate with CMake: `cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON ..`
- Or use Bear for Make: `bear -- make`

**Python Known Issue:**
- scip-python has an upstream bug: test file references are not indexed
- Workaround: Manually update test files when using `rename_symbol`
- See README.md "Known Issues" section for details

### Tree-sitter Only Mode

- **Swift** (NEW in v0.4.0) - Tree-sitter-only mode
  - ✅ Local refactoring: inline_variable, extract method
  - ✅ Function/class finding
  - ✅ AST pattern search
  - ⚠️ Rename symbol: Single file only
  - ❌ Cross-file navigation: goto_definition, find_references (requires SCIP indexer - roadmap for v0.5.0)

**Why tree-sitter-only for Swift?** No official SCIP indexer exists for Swift yet. Tree-sitter enables local refactoring, but not cross-file semantic navigation. See `docs/SWIFT_LANGUAGE_SUPPORT_PLAN.md` for full roadmap.

---

## When to Use Which Tool

### Finding Code

| Task | Hand Tool (❌) | Power Tool (✅) | Why |
|------|---------------|----------------|-----|
| Find definition | `grep -r "function foo"` | `goto_definition(file:line:col)` | Semantic precision vs text matching |
| Find usages | `grep -r "myVar"` | `find_references("myVar")` | Filters out strings/comments |
| Find all functions | `grep -r "function "` | `list_functions()` | Finds ALL functions by AST |
| Find classes | `grep -r "class "` | `list_classes()` | Handles all class-like structures |
| Find async functions | `grep -r "async"` | `search_ast("(async_function) @f")` | Precise AST matching |

### Refactoring Code

| Task | Hand Tool (❌) | Power Tool (✅) | Why |
|------|---------------|----------------|-----|
| Rename variable | `batch_replace("old", "new")` | `rename_symbol(..., preview=true)` | Semantic-aware, safe |
| Inline constant | Manual copy-paste | `inline_variable(..., preview=true)` | Handles all usages |
| Fix typos | Manual search | `batch_replace("recieve", "receive", preview=true)` | Fast, safe with preview |
| Update URLs | Manual editing | `batch_replace(pattern, replacement, preview=true)` | Regex + preview |

### Managing Indexes

| Situation | What to Do | Why |
|-----------|------------|-----|
| First time on project | `index_project(auto_install=true)` | Build indexes once |
| File changed | Nothing (watcher handles it) | Auto re-indexing |
| Before `git checkout` | `watcher_stop()` | Avoid re-index spam |
| After bulk operation | `watcher_start()` | Resume monitoring |
| Symbol not found error | Wait 2-5 seconds, retry | Index rebuilding |

---

## Technical Reference

### CLI Commands (if not using MCP)

The powertools binary is located at: `./powertools-cli/target/release/powertools`

**Semantic Navigation:**
```bash
# Index project (auto-detects all languages)
./powertools-cli/target/release/powertools index --auto-install

# Index specific languages
./powertools-cli/target/release/powertools index --languages typescript python rust

# Go to definition
./powertools-cli/target/release/powertools definition src/file.ts:10:5 --format json

# Find references
./powertools-cli/target/release/powertools references myFunction --format json

# Include declarations
./powertools-cli/target/release/powertools references myFunction --include-declarations --format json
```

**Tree-sitter Pattern Matching:**
```bash
# Search AST patterns
./powertools-cli/target/release/powertools search-ast "(function_item) @func" -p src/

# List functions
./powertools-cli/target/release/powertools functions --format json

# List classes
./powertools-cli/target/release/powertools classes --format json

# Project stats
./powertools-cli/target/release/powertools stats
```

**Example Tree-sitter Patterns:**
- Rust: `(function_item) @func`
- TypeScript: `(function_declaration) @func`
- Python: `(function_definition) @func`
- Swift: `(function_declaration) @func`
- Async functions: `(async_function) @func`
- Classes: `(class_declaration) @class`

**Output Formats:**
All commands support `--format json` for structured data perfect for parsing.

### Pagination

All MCP tools that return lists support pagination to prevent token limit errors:

- `limit` parameter: Maximum results to return (default: 100)
- `offset` parameter: Number of results to skip (default: 0)
- Response includes: `count` (total), `has_more` (boolean), result data

**Example:** On a project with 1,438 functions, `list_functions(limit=100)` returns only 100 results.


# Project Context

## Design Workflow

When working on front-end designs:

1. **Make Changes**: Implement the requested design changes
2. **Visual Validation**: Take a screenshot using Playwright to see the current state
3. **Compare**: Analyze the screenshot against the design requirements
4. **Iterate**: If issues are found, make corrections and repeat

## Playwright Usage

- Use Playwright to capture screenshots after making changes
- Check browser console for errors
- Test responsive designs by emulating different devices
- Navigate through different states of the application

## Design Philosophy

### Vision
Jility is a **lightweight, fast, agent-first** project management tool that makes JIRA seem bloated while being **beautiful enough for design-conscious teams**.

### Core Principles

1. **Mobile-First Design**
   - Design for mobile screens first, then progressively enhance for larger screens
   - Use Tailwind's responsive breakpoints: `sm:`, `md:`, `lg:`, `xl:`
   - Base styles apply to mobile, add breakpoint prefixes for desktop
   - Touch-friendly targets (minimum 44px tap areas)
   - Bottom navigation on mobile, sidebar on desktop

2. **Agent-First Interactions**
   - Optimized for AI coding assistants and automation
   - Clear, semantic component structure for easy agent navigation
   - MCP server integration for programmatic access
   - Ticket metadata designed for agent workflows

3. **Lightweight & Fast**
   - Minimal dependencies
   - Fast page loads and interactions
   - SQLite for local-first development
   - Real-time updates via WebSockets

4. **Beautiful & Consistent**
   - Professional design suitable for design-conscious teams
   - Consistent spacing, typography, and visual hierarchy
   - Thoughtful use of color and iconography
   - Smooth transitions and interactions

### Theme System

**CSS Variables for Light and Dark Modes**

Jility uses CSS custom properties (CSS variables) to implement a theme system that seamlessly adapts between light and dark modes. **Always use theme variables instead of hardcoded colors.**

**Theme Variables** (defined in `jility-web/app/globals.css`):

```css
/* Semantic color tokens */
--background         /* Page background */
--foreground         /* Primary text color */
--card              /* Card/panel backgrounds */
--card-foreground   /* Text on cards */
--primary           /* Brand/accent color */
--primary-foreground /* Text on primary */
--secondary         /* Secondary backgrounds */
--secondary-foreground /* Text on secondary */
--muted             /* Subtle backgrounds */
--muted-foreground  /* Subtle text */
--border            /* Border color */
--input             /* Input borders */
--ring              /* Focus ring color */
--destructive       /* Error/danger color */
```

**Usage in Tailwind CSS:**

```tsx
// ✅ CORRECT - Uses theme variables
<div className="bg-card border-border text-foreground">
<div className="bg-muted text-muted-foreground">
<button className="bg-primary text-primary-foreground">

// ❌ WRONG - Hardcoded colors break dark mode
<div className="bg-white border-gray-200 text-gray-900">
<div className="bg-gray-50 text-gray-600">
<button className="bg-blue-500 text-white">
```

**Available Tailwind Classes:**
- `bg-background`, `bg-foreground`
- `bg-card`, `text-card-foreground`
- `bg-primary`, `text-primary-foreground`
- `bg-secondary`, `text-secondary-foreground`
- `bg-muted`, `text-muted-foreground`
- `bg-accent`, `text-accent-foreground`
- `bg-destructive`, `text-destructive-foreground`
- `border-border`, `border-input`
- `ring-ring`

**Status Colors:**
Custom status colors are also available as CSS variables:
- `--status-backlog`, `--status-todo`, `--status-in-progress`
- `--status-review`, `--status-done`, `--status-blocked`

**Why Theme Variables?**
- **Dark mode support**: Variables automatically change values when `.dark` class is applied
- **Consistency**: Ensures all components use the same color palette
- **Maintainability**: Change theme colors in one place (globals.css)
- **Accessibility**: Proper contrast ratios maintained in both light and dark modes

**Testing Dark Mode:**
Always test components in both light and dark modes. Use the theme toggle in the navbar to switch modes during development.

### Icon System

- **Library**: [Lucide React](https://lucide.dev)
- **No emojis**: All icons use Lucide components for consistency and accessibility
- **Standard sizes**: `h-4 w-4` (16px), `h-5 w-5` (20px), `h-6 w-6` (24px)
- **Semantic naming**: Choose icons that clearly represent their function

### Typography

- **Font**: System font stack for optimal performance
- **Headings**: Clear hierarchy using Tailwind's text size utilities
- **Body text**: `text-sm` for dense information, `text-base` for content
- **Monospace**: Used for ticket IDs, code, and technical data

### Spacing & Layout

- **Responsive padding**: `p-3 md:p-6` (less padding on mobile)
- **Consistent gaps**: Use Tailwind's gap utilities (`gap-2`, `gap-4`, etc.)
- **Grid & Flexbox**: Prefer CSS Grid for 2D layouts, Flexbox for 1D
- **Max widths**: Use `max-w-7xl` for content containers to prevent ultra-wide layouts

---

# Jility MCP Tools

Jility provides an MCP (Model Context Protocol) server that allows Claude Code to interact with tickets programmatically. This enables AI-assisted project management workflows.

## Available Tools

The Jility MCP server is configured in `.mcp.json` and provides the following tools:

### Ticket Management
- **`mcp__jility__create_ticket`** - Create a new ticket
  - Parameters: `title`, `description`, `status`, `story_points`, `labels`, `assignees`, `epic_id`, `parent_id`
  - Returns: Created ticket with ID and number (e.g., JIL-42)

- **`mcp__jility__list_tickets`** - List tickets with optional filters
  - Parameters: `status` (array), `assignee`, `labels`, `limit`, `offset`
  - Returns: Array of tickets matching filters

- **`mcp__jility__get_ticket`** - Get full ticket details
  - Parameters: `ticket_id`
  - Returns: Ticket with comments, dependencies, commits, history

- **`mcp__jility__update_status`** - Update ticket status
  - Parameters: `ticket_id`, `status` (backlog, todo, in_progress, review, done, blocked)

- **`mcp__jility__update_description`** - Update ticket description
  - Parameters: `ticket_id`, `content`, `operation` (replace_all, append, prepend, replace_lines, replace_section)

### Collaboration
- **`mcp__jility__get_comments`** - Get all comments for a ticket
  - Parameters: `ticket_id` (supports both UUID and ticket number format like JIL-42)
  - Returns: Array of comments with author, content, timestamps
  - **Use case:** Agents read comment threads before starting work on a ticket

- **`mcp__jility__add_comment`** - Add a comment to a ticket
  - Parameters: `ticket_id`, `content`
  - Supports `@mentions` for notifications

- **`mcp__jility__assign_ticket`** - Assign ticket to team members
  - Parameters: `ticket_id`, `assignees` (array, empty array to unassign)
  - Supports pair programming (multiple assignees)

### Workflow
- **`mcp__jility__claim_ticket`** - Claim an unassigned ticket
  - Parameters: `ticket_id`
  - Auto-assigns to agent and moves to `in_progress`

- **`mcp__jility__delete_ticket`** - Soft delete a ticket
  - Parameters: `ticket_id` (supports both UUID and ticket number format like JIL-42)
  - **Soft delete:** Ticket is marked as deleted but preserved in database for audit trail
  - Returns: Success confirmation
  - **Use case:** Clean up test tickets, mistakes, or duplicates

### Dependencies
- **`mcp__jility__add_dependency`** - Mark ticket dependency
  - Parameters: `ticket_id`, `depends_on` (blocker ticket ID)

- **`mcp__jility__remove_dependency`** - Remove dependency

- **`mcp__jility__get_dependency_graph`** - Get full dependency tree

### Batch Operations
- **`mcp__jility__create_tickets_batch`** - Create multiple tickets at once
  - Useful for breaking down epics into sub-tasks
  - Parameters: `tickets` (array), `parent_id` (optional)

### Search
- **`mcp__jility__search_tickets`** - Full-text search across tickets
  - Parameters: `query`, `limit`
  - Searches titles, descriptions, and comments

### Templates
- **`mcp__jility__list_templates`** - List available ticket templates

- **`mcp__jility__create_from_template`** - Create ticket from template
  - Parameters: `template`, `variables` (for substitution)

### Epic Management
- **`mcp__jility__create_epic`** - Create a new epic
  - Parameters: `title`, `description`, `epic_color` (optional hex color like "#3b82f6")
  - Returns: Created epic with ID and number (e.g., JIL-42)
  - **Use case:** Organize related tickets into themed work packages

- **`mcp__jility__list_epics`** - List all epics with progress tracking
  - Parameters: None
  - Returns: Array of epics with progress statistics (total/done/in_progress/todo counts, completion percentage)
  - **Use case:** View project organization and epic completion status

**Assigning Tickets to Epics:**
When creating or updating tickets, use the `parent_epic_id` parameter to link tickets to epics. Use `mcp__jility__create_ticket()` with the `parent_epic_id` field or `mcp__jility__list_tickets()` with the `epic_id` filter.

### Git Integration
- **`mcp__jility__link_commit`** - Link a git commit to a ticket
  - Parameters: `ticket_id`, `commit_hash`, `commit_message`

## Usage Examples

### Creating a Feature Ticket
```typescript
mcp__jility__create_ticket({
  title: "Add dark mode toggle",
  description: "Implement theme switcher in navbar using Tailwind dark mode",
  status: "backlog",
  story_points: 3,
  labels: ["feature", "frontend", "ui"]
})
```

### Planning with Epics
```typescript
// Create an epic to organize related work
const epic = await mcp__jility__create_epic({
  title: "User Authentication System",
  description: "Complete user authentication with login, registration, and password reset",
  epic_color: "#3b82f6"
})

// Create tickets for the epic
await mcp__jility__create_tickets_batch({
  parent_id: epic.id,
  tickets: [
    { title: "Design auth database schema", story_points: 2, labels: ["backend", "database"] },
    { title: "Implement JWT authentication", story_points: 5, labels: ["backend"] },
    { title: "Build login UI", story_points: 3, labels: ["frontend", "ui"] },
    { title: "Add password reset flow", story_points: 3, labels: ["backend", "frontend"] }
  ]
})

// View epic progress
const epics = await mcp__jility__list_epics()
// Shows: "User Authentication System: 0/4 tasks completed (0%)"

// Filter tickets by epic
const authTickets = await mcp__jility__list_tickets({
  epic_id: epic.id
})
```

### Working on a Ticket
```typescript
// Claim ticket
await mcp__jility__claim_ticket({ ticket_id: "ticket-id" })

// Add progress update
await mcp__jility__add_comment({
  ticket_id: "ticket-id",
  content: "Implemented the UI components. Moving to backend integration."
})

// Mark complete
await mcp__jility__update_status({
  ticket_id: "ticket-id",
  status: "done"
})
```

## Best Practices

1. **Always set story points** when creating tickets - helps with sprint planning
2. **Use descriptive titles** - should be clear without reading the description
3. **Add labels** for categorization - use consistent label names (frontend, backend, bug, feature, etc.)
4. **Organize with epics** - group related tickets into epics for better project organization
5. **Link related tickets** - use dependencies to track blockers
6. **Update status regularly** - keep the board accurate
7. **Add comments for context** - explain decisions and progress
8. **Use batch operations** when creating multiple related tickets
9. **Check epic progress** - use `list_epics()` to track completion of themed work packages

---

# Story Point Estimation

Jility uses the **Practical Fibonacci** sequence for story point estimation. Story points measure effort and complexity, not time.

## Practical Fibonacci Scale

**0 - No Points**
- No effort is required, or there is effort but no business value delivered
- Example: Behavioral changes from Scrum Retrospective

**1 - Extra Small**
- Developers feel they understand most requirements and consider it relatively easy
- Probably the smallest item in the Sprint
- Most likely completed in one day

**2 - Small**
- A little bit of thought, effort, or problem-solving is required
- Developers have done this a lot and have confidence in the requirements
- Or, it sounds extra small, but hedge the bet just a bit

**3 - Average**
- Developers have done this a lot; they know what needs to be done
- May have a few extra steps, but that's it
- Unlikely to need research

**5 - Large**
- Complex work, or developers don't do this very often
- Most developers will need assistance from someone else on the team
- Probably one of the largest items that can be completed within a Sprint

**8 - Extra Large**
- Going to take significant time and research
- Probably needs more than one developer to complete within two weeks
- Developers need to make several assumptions that increase the risk
- Could affect getting it Done

**13 - Warning!**
- Complex piece of work with a lot of unknowns
- Requires multiple assumptions to size
- **Too much to complete in one Sprint**
- **Should be split into multiple items** that can be completed independently

**21 - Hazard!**
- Reflects too much complexity to be done within one Sprint
- **Needs to be refined more**
- Large size indicates more risk, assumptions, and dependencies

**? - Danger!**
- As a developer, we don't want to do this work the way it's currently written
- Very complex and cannot be completed in the timeframe of an iteration/Sprint
- Requirements are so fuzzy that it's rife with danger
- **Needs clarification and breakdown before estimation**

## Estimation Guidelines

1. **Compare, don't estimate in hours** - Story points are relative, not absolute
2. **Team consensus** - Use planning poker for collaborative estimation
3. **Consider complexity, not just time** - Account for unknowns and risks
4. **Split large items** - Anything 13+ should be broken down
5. **Velocity emerges over time** - Track completed points per sprint to predict capacity
6. **Re-estimate if needed** - If work uncovers more complexity, it's okay to adjust

## Red Flags

- **13 or higher**: Too big for one sprint - break it down
- **Multiple 8s in a sprint**: Risk of overcommitment
- **Lots of ?s**: Requirements need clarification
- **Wide estimation variance**: Team doesn't understand the work the same way

## Typical Sprint Capacity

Based on Jility's sprint planning:
- Default capacity: **40-70 story points** per 2-week sprint
- Configurable per workspace (see JIL-23, JIL-26)
- Should be based on team's **historical velocity** from completed sprints