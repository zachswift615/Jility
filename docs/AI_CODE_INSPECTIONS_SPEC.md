# AI Code Inspections - Formal Specification

**Version:** 1.0
**Status:** Draft
**Last Updated:** 2025-10-25

---

## 1. Executive Summary

This document specifies the **AI Code Inspections** feature for Jility, an AI-native project management tool. The feature enables automated code analysis using embedded or cloud-based LLMs to scan codebases, identify issues, and automatically generate comprehensive, actionable tickets.

### Key Capabilities

1. **Repository Integration** - Configure GitHub repos per project, clone locally for analysis
2. **Multi-Provider LLM Support** - Embedded (Ollama), local (LM Studio), or cloud (OpenAI, Anthropic)
3. **Code Inspections** - Automated scans for security, performance, architecture, tests, etc.
4. **Staged Ticket Creation** - Preview inspection results before creating tickets
5. **Privacy-First Architecture** - Code analyzed locally on user's machine, only results uploaded to server

---

## 2. Goals and Non-Goals

### 2.1 Goals

- **Automate code quality analysis** - Replace manual code reviews for common issues
- **Generate actionable tickets** - Each finding includes file path, line number, code snippet, reasoning, and fix suggestions
- **Respect privacy** - Keep proprietary code on user's machine, never upload to Jility servers
- **Multi-provider flexibility** - Support free (Ollama), local (LM Studio), and cloud (OpenAI/Anthropic) LLMs
- **Team collaboration** - Share inspection insights across team via Jility server

### 2.2 Non-Goals (Future Phases)

- ❌ Sprint planning AI (v0.4 - server-side LLM for priority scoring)
- ❌ Chat-based ticket creation (v0.5 - interactive AI assistant)
- ❌ Scheduled inspections (v0.3 - cron-style automation)
- ❌ Custom inspection prompts (v0.3 - user-defined analysis types)

---

## 3. User Stories

### 3.1 Repository Configuration

**As a** project admin
**I want to** configure which GitHub repositories power my Jility project
**So that** the AI can analyze my codebase for issues

**Acceptance Criteria:**
- Can add/remove repository URLs via web UI
- Can specify branch to analyze (default: `main`)
- Can provide read-only GitHub token for private repos
- Repositories stored server-side, cloned client-side

---

### 3.2 Desktop Client Setup

**As a** developer
**I want to** install the Jility desktop client and configure my LLM provider
**So that** code inspections run on my machine without uploading code to servers

**Acceptance Criteria:**
- Can download desktop client binary (macOS, Linux, Windows)
- First-run wizard prompts for:
  - Jility API URL
  - API key (generated in web UI)
  - LLM provider (Anthropic, OpenAI, Ollama, LM Studio)
  - Model name and API key (if cloud provider)
- Configuration saved to `~/.jility/config.toml`
- Client runs as background service (system tray icon)

---

### 3.3 Running Security Inspection (MVP)

**As a** developer
**I want to** trigger a security vulnerability scan on my project
**So that** I can discover SQL injection, XSS, auth bypasses, and hardcoded secrets

**Acceptance Criteria:**
- Can trigger "Security Vulnerabilities" inspection from web UI
- Desktop client receives job notification
- Client clones/updates repositories locally to `~/.jility/<project-name>/<repo-name>`
- AI scans codebase using Read, Grep, Glob, and Powertools
- AI generates structured findings (title, description, severity, file_path, line_number, code_snippet, reasoning)
- Results uploaded to Jility server as staged tickets

---

### 3.4 Reviewing Inspection Results

**As a** developer
**I want to** preview inspection results before creating tickets
**So that** I can reject false positives and approve real issues

**Acceptance Criteria:**
- Web UI shows inspection results page with all findings
- Each finding displays:
  - Title (e.g., "SQL Injection in User Login")
  - Severity badge (critical, high, medium, low)
  - File path and line number
  - Code snippet (syntax highlighted)
  - AI reasoning (why it's an issue, how to fix)
- Can select/deselect findings via checkboxes
- "Create Tickets" button batch-creates tickets for selected findings
- Created tickets appear in project backlog

---

### 3.5 Managing Inspection Configurations

**As a** project admin
**I want to** enable/disable specific inspection types for my project
**So that** I can focus on relevant code quality issues

**Acceptance Criteria:**
- Web UI shows list of available inspection types:
  - ✅ Security Vulnerabilities (MVP)
  - Performance Bottlenecks (v0.3)
  - Missing Tests (v0.3)
  - Architecture Issues (v0.3)
  - Code Quality (v0.3)
- Can enable/disable each inspection type
- Can edit system prompt for each inspection
- Can manually trigger inspection runs via "Run Now" button

---

## 4. System Architecture

### 4.1 Components

```
┌─────────────────────────────────────────┐
│         Jility Web App (Next.js)        │
│  - Repository config UI                 │
│  - Inspection management UI             │
│  - Results preview UI                   │
│  - Ticket creation UI                   │
└──────────────────┬──────────────────────┘
                   │ HTTPS API
┌──────────────────▼──────────────────────┐
│       Jility Server (Rust/Axum)         │
│  - inspection_configs table             │
│  - project_repositories table           │
│  - inspection_jobs table                │
│  - inspection_results table             │
└──────────────────┬──────────────────────┘
                   │ WebSocket/Polling (5s interval)
┌──────────────────▼──────────────────────┐
│  Jility Desktop Client (Rust + Synthia) │
│  - Poll server for inspection jobs      │
│  - Clone/update repos locally           │
│  - Execute AI inspections via Synthia   │
│  - Upload results to server             │
│  - System tray status icon              │
└──────────────────────────────────────────┘
```

### 4.2 Data Storage

**Server-side (PostgreSQL):**
- `inspection_configs` - Inspection types, prompts, enabled status per project
- `project_repositories` - Repository URLs, branches, GitHub tokens (encrypted)
- `inspection_jobs` - Queue of pending/running/completed inspection jobs
- `inspection_results` - Staged tickets awaiting user approval

**Client-side (~/.jility/):**
- `config.toml` - LLM provider, API keys, server URL
- `<project-name>/<repo-name>/` - Cloned repositories (read-only)

---

## 5. API Contracts

### 5.1 Server API Endpoints

#### `GET /api/projects/:id/repositories`
Get configured repositories for a project.

**Response:**
```json
{
  "repositories": [
    {
      "id": "uuid",
      "repo_url": "https://github.com/user/repo",
      "branch": "main",
      "last_synced_at": "2025-10-25T12:00:00Z"
    }
  ]
}
```

---

#### `POST /api/projects/:id/repositories`
Add a repository to a project.

**Request:**
```json
{
  "repo_url": "https://github.com/user/repo",
  "branch": "main",
  "github_token": "ghp_xxxxx" // encrypted at rest
}
```

**Response:**
```json
{
  "id": "uuid",
  "repo_url": "https://github.com/user/repo",
  "branch": "main"
}
```

---

#### `GET /api/projects/:id/inspections/configs`
Get inspection configurations for a project.

**Response:**
```json
{
  "configs": [
    {
      "id": "uuid",
      "inspection_type": "security",
      "enabled": true,
      "prompt": "Focus on SQL injection, XSS, auth bypasses...",
      "scope_pattern": "src/**"
    }
  ]
}
```

---

#### `POST /api/inspections/trigger`
Trigger an inspection run.

**Request:**
```json
{
  "project_id": "uuid",
  "inspection_config_id": "uuid"
}
```

**Response:**
```json
{
  "job_id": "uuid",
  "status": "queued",
  "created_at": "2025-10-25T12:00:00Z"
}
```

---

#### `GET /api/client/poll-jobs` (Desktop Client)
Poll for pending inspection jobs.

**Headers:**
```
Authorization: Bearer <client-api-key>
```

**Response:**
```json
{
  "jobs": [
    {
      "id": "uuid",
      "project_id": "uuid",
      "project_name": "my-project",
      "inspection_config_id": "uuid",
      "inspection_type": "security",
      "prompt": "Scan for security vulnerabilities...",
      "repositories": [
        {
          "repo_url": "https://github.com/user/repo",
          "branch": "main",
          "github_token": "ghp_xxxxx"
        }
      ]
    }
  ]
}
```

---

#### `POST /api/client/jobs/:id/claim` (Desktop Client)
Claim a job for execution.

**Response:**
```json
{
  "status": "running",
  "started_at": "2025-10-25T12:01:00Z"
}
```

---

#### `POST /api/client/jobs/:id/results` (Desktop Client)
Upload inspection results.

**Request:**
```json
{
  "results": [
    {
      "title": "SQL Injection in User Login",
      "description": "Direct string interpolation allows SQL injection. Use parameterized queries.",
      "severity": "critical",
      "file_path": "src/auth/login.ts",
      "line_number": 45,
      "code_snippet": "const query = `SELECT * FROM users WHERE email='${email}'`;",
      "ai_reasoning": "User input directly concatenated into SQL query without sanitization..."
    }
  ]
}
```

**Response:**
```json
{
  "status": "completed",
  "results_count": 12
}
```

---

#### `GET /api/inspections/:job_id/results`
Get inspection results for preview.

**Response:**
```json
{
  "job_id": "uuid",
  "completed_at": "2025-10-25T12:05:00Z",
  "results": [
    {
      "id": "uuid",
      "title": "SQL Injection in User Login",
      "description": "...",
      "severity": "critical",
      "file_path": "src/auth/login.ts",
      "line_number": 45,
      "code_snippet": "...",
      "ai_reasoning": "...",
      "approved": false
    }
  ]
}
```

---

#### `POST /api/inspections/:job_id/create-tickets`
Create tickets from approved results.

**Request:**
```json
{
  "result_ids": ["uuid1", "uuid2", "uuid3"]
}
```

**Response:**
```json
{
  "created_tickets": [
    {
      "result_id": "uuid1",
      "ticket_id": "uuid",
      "ticket_number": "JIL-123"
    }
  ]
}
```

---

## 6. Inspection Prompt Engineering

### 6.1 System Prompt Template

All inspections use this base system prompt:

```markdown
You are a code analysis assistant helping developers improve their codebase.

Your task: Analyze the provided code files for **{INSPECTION_TYPE}** issues.

Output format: JSON array of findings, where each finding has:
- title: Brief description (50 chars max)
- description: Detailed explanation with actionable fix
- severity: "critical" | "high" | "medium" | "low"
- file_path: Relative path to file
- line_number: Line where issue starts (if applicable)
- code_snippet: Relevant code (10 lines max)
- reasoning: Why this is an issue and how to fix it

Example output:
[
  {
    "title": "SQL Injection in User Login",
    "description": "Direct string interpolation in SQL query allows injection attacks. Replace with parameterized queries using prepared statements.",
    "severity": "critical",
    "file_path": "src/auth/login.ts",
    "line_number": 45,
    "code_snippet": "const query = `SELECT * FROM users WHERE email='${email}'`;",
    "reasoning": "User input is directly concatenated into SQL query without sanitization. An attacker could input `' OR '1'='1` to bypass authentication."
  }
]

Rules:
1. Only report REAL issues, not theoretical ones
2. Be specific about the fix (include code examples if possible)
3. Prioritize severity correctly (critical = exploitable, high = likely buggy, medium = tech debt, low = style)
4. Include line numbers when possible
5. Keep descriptions actionable (what to do, not just what's wrong)
```

---

### 6.2 Security Inspection Prompt (MVP)

```markdown
{SYSTEM_PROMPT}

Focus on these security issues:
1. **SQL Injection**: Raw SQL queries with user input
2. **XSS**: Unescaped user input rendered in HTML
3. **Authentication bypasses**: Missing auth checks on sensitive routes
4. **Hardcoded secrets**: API keys, passwords in code
5. **Insecure dependencies**: Known CVEs in package.json/Cargo.toml

Use the following tools to analyze the codebase:
- `read` - Read file contents
- `grep` - Search for patterns (e.g., grep for "SELECT.*FROM" to find SQL queries)
- `glob` - Find files matching patterns (e.g., glob "src/**/*.ts")
- `powertools` - Semantic code navigation (find references, goto definition)

After analysis, call the `upload_inspection_results` tool with your findings in JSON format.
```

---

## 7. MVP Scope (v0.1)

### 7.1 Features

**Included:**
- ✅ Desktop client (Rust + Synthia agent core)
- ✅ Web UI for repository configuration
- ✅ Web UI for inspection management
- ✅ Web UI for staged ticket creation (preview + approve)
- ✅ ONE inspection type: "Security Vulnerabilities"
- ✅ Manual trigger only (no scheduling)
- ✅ ONE LLM provider: Anthropic Claude (for simplicity)
- ✅ Polling-based communication (5s interval)

**Excluded (Future Versions):**
- ❌ Scheduled inspections (v0.3)
- ❌ Custom inspection prompts (v0.3)
- ❌ Multiple LLM providers (v0.2)
- ❌ Sprint planning AI (v0.4)
- ❌ Chat-based ticket creation (v0.5)

### 7.2 Success Metrics

**MVP is successful if:**
1. User can configure 1+ GitHub repos via web UI
2. User can install desktop client and configure Anthropic API key
3. User can trigger security inspection from web UI
4. Desktop client clones repo, runs AI analysis, uploads results
5. User can preview 10+ inspection findings in web UI
6. User can select findings and create tickets (1 ticket per finding)
7. Created tickets appear in project backlog with proper formatting

---

## 8. Future Phases

### 8.1 v0.2 - Multi-Provider Support

**Goal:** Support Ollama (local), OpenAI, and LM Studio

**New Features:**
- Client config UI in web app (or desktop app settings window)
- Provider selection dropdown (Anthropic, OpenAI, Ollama, LM Studio)
- Base URL config for local providers (http://localhost:11434)
- Model selection dropdown (provider-specific)

---

### 8.2 v0.3 - Custom Inspections & Scheduling

**Goal:** User-defined inspection types and automated runs

**New Features:**
- "Create Custom Inspection" button in web UI
- Prompt editor with preview
- Scope configuration (file patterns to analyze)
- Scheduling options: manual, daily, weekly, on-push (webhook)
- Preset inspections for:
  - Performance Bottlenecks
  - Missing Tests
  - Architecture Issues
  - Code Quality (linting/formatting)

---

### 8.3 v0.4 - Sprint Planning AI

**Goal:** AI-powered priority scoring for tickets

**New Features:**
- Server-side LLM for analyzing ticket metadata
- Priority score breakdown (tech debt, business value, blockers, velocity, dependencies)
- Configurable weights UI (sliders for each factor)
- Presets: "Aggressive cleanup", "Feature-first", "Balanced"
- AI-generated explanations for priority scores

---

### 8.4 v0.5 - Chat-Based Ticket Creation

**Goal:** Interactive AI assistant for ticket creation

**New Features:**
- Chat interface in web UI
- User prompt: "I need to add OAuth2 authentication"
- AI scans codebase, proposes ticket breakdown
- Iterative refinement (user can adjust AI suggestions)
- Batch ticket creation from chat conversation

---

## 9. Security & Privacy Considerations

### 9.1 Code Privacy

- ✅ **Code never leaves user's machine** - Only inspection results (findings) uploaded to server
- ✅ **Read-only repo access** - GitHub tokens have `repo:read` scope only
- ✅ **Local cloning** - Repos cloned to `~/.jility/<project>/` on user's disk
- ✅ **User control** - User chooses LLM provider (can use local Ollama for 100% offline)

### 9.2 API Key Security

- ✅ **Client-side storage** - LLM API keys stored in `~/.jility/config.toml` (user's machine)
- ✅ **Server-side encryption** - GitHub tokens encrypted at rest using AES-256
- ✅ **Secure transmission** - All API calls over HTTPS with TLS 1.3
- ✅ **API key rotation** - Users can regenerate Jility API keys in web UI

### 9.3 Threat Model

**Threats Mitigated:**
- ❌ Code theft from Jility servers (code never uploaded)
- ❌ GitHub token theft (encrypted at rest, transmitted over HTTPS)
- ❌ Malicious inspection execution (client can reject/cancel jobs)

**Threats NOT Mitigated:**
- ⚠️ Compromised desktop client (malware could read `~/.jility/config.toml`)
- ⚠️ Malicious LLM provider (cloud providers see prompts + code excerpts)
- ⚠️ Man-in-the-middle attacks (rely on HTTPS/TLS)

---

## 10. Testing & Validation

### 10.1 Unit Tests

**Server:**
- Database schema migrations
- API endpoint handlers
- Encryption/decryption of GitHub tokens

**Desktop Client:**
- Git clone/pull operations
- Job polling logic
- Result upload formatting
- Synthia agent integration

### 10.2 Integration Tests

**End-to-End Flow:**
1. User adds GitHub repo via web UI
2. User triggers security inspection
3. Desktop client receives job notification
4. Client clones repo locally
5. AI analyzes code and generates findings
6. Client uploads results to server
7. User previews results in web UI
8. User creates tickets from findings
9. Tickets appear in backlog

**Expected Outcome:** 10+ tickets created from security scan of a sample vulnerable codebase (e.g., OWASP Juice Shop)

### 10.3 Manual Testing

**Test Cases:**
- Desktop client setup (first-run wizard)
- Repository config (add/remove repos)
- Inspection trigger (manual run)
- Results preview (checkbox selection, "Create Tickets" button)
- Ticket creation (verify formatting, metadata)
- Error handling (network failures, invalid API keys, git clone failures)

---

## 11. Open Questions

### 11.1 Inspection Scope Control

**Question:** Should users manually specify which files to analyze, or let AI figure it out?

**Options:**
- **A) AI figures it out** (MVP) - User provides high-level prompt, AI decides which files to read
- **B) Manual file patterns** (v0.3) - User specifies glob patterns (e.g., `src/auth/**`, `src/api/**`)
- **C) AI-assisted scope** (v0.3) - AI asks user "Should I also check database queries?" before scanning

**Decision:** Start with Option A (AI figures it out) for MVP. Add Options B/C in v0.3.

---

### 11.2 Desktop Client UI

**Question:** Does the desktop client need a UI, or just system tray icon?

**Options:**
- **A) System tray only** (MVP) - Minimal UI, show status icon and notifications
- **B) Settings window** - Small UI for configuring LLM provider
- **C) Full desktop app** - Electron/Tauri app with inspection history, logs, etc.

**Decision:** Option A (system tray) for MVP. All configuration done in web UI. Option B (settings window) in v0.2 if needed.

---

### 11.3 Handling Large Codebases

**Question:** How to handle repos with 10,000+ files that exceed LLM context limits?

**Options:**
- **A) Let AI chunk** - AI reads files incrementally until context limit, then summarizes
- **B) Scope restrictions** - Require user to specify file patterns (e.g., only `src/`)
- **C) Multiple passes** - Run inspection in batches (e.g., analyze `src/auth/`, then `src/api/`)

**Decision:** Option A (let AI chunk) for MVP. Synthia's context manager auto-compacts messages when hitting token limits. Add Options B/C in v0.3 if performance issues arise.

---

## 12. Revision History

| Version | Date       | Author | Changes                          |
|---------|------------|--------|----------------------------------|
| 1.0     | 2025-10-25 | Zach   | Initial specification            |

---

## 13. Approval

**Awaiting approval from:**
- [ ] Product Lead
- [ ] Engineering Lead
- [ ] Security Lead

**Sign-off:**

_Signature:_ ____________________
_Date:_ ____________________
