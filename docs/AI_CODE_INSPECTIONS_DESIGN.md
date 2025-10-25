# AI Code Inspections - Technical Design Document

**Version:** 1.0
**Status:** Draft
**Last Updated:** 2025-10-25

---

## 1. Overview

This document provides the technical design for the **AI Code Inspections** feature in Jility. It covers architecture, component interactions, data models, implementation details, and deployment strategy.

**Related Documents:**
- [AI_CODE_INSPECTIONS_SPEC.md](./AI_CODE_INSPECTIONS_SPEC.md) - Formal specification

---

## 2. Architecture

### 2.1 System Components

```
┌────────────────────────────────────────────────────────────────┐
│                     Jility Web App (Next.js)                   │
│ ┌────────────────────────────────────────────────────────────┐ │
│ │  Repository Config Page (/projects/:id/settings/ai)       │ │
│ │  - Add/remove GitHub repos                                │ │
│ │  - Configure GitHub access token                          │ │
│ └────────────────────────────────────────────────────────────┘ │
│ ┌────────────────────────────────────────────────────────────┐ │
│ │  Inspection Management Page (/projects/:id/inspections)   │ │
│ │  - View available inspections                             │ │
│ │  - Enable/disable inspections                             │ │
│ │  - Edit prompts                                           │ │
│ │  - Trigger manual runs                                    │ │
│ └────────────────────────────────────────────────────────────┘ │
│ ┌────────────────────────────────────────────────────────────┐ │
│ │  Inspection Results Page (/inspections/:job_id/results)   │ │
│ │  - Preview findings with checkboxes                       │ │
│ │  - View code snippets, severity, reasoning                │ │
│ │  - Bulk create tickets from selected results              │ │
│ └────────────────────────────────────────────────────────────┘ │
└────────────────────────────┬───────────────────────────────────┘
                             │ HTTPS REST API
                             │
┌────────────────────────────▼───────────────────────────────────┐
│                  Jility Server (Rust + Axum)                   │
│ ┌────────────────────────────────────────────────────────────┐ │
│ │  API Routes:                                              │ │
│ │  - GET/POST /api/projects/:id/repositories                │ │
│ │  - GET/POST /api/projects/:id/inspections/configs         │ │
│ │  - POST /api/inspections/trigger                          │ │
│ │  - GET /api/client/poll-jobs (desktop client)             │ │
│ │  - POST /api/client/jobs/:id/claim                        │ │
│ │  - POST /api/client/jobs/:id/results                      │ │
│ │  - GET /api/inspections/:job_id/results                   │ │
│ │  - POST /api/inspections/:job_id/create-tickets           │ │
│ └────────────────────────────────────────────────────────────┘ │
│ ┌────────────────────────────────────────────────────────────┐ │
│ │  Database (PostgreSQL):                                   │ │
│ │  - inspection_configs                                     │ │
│ │  - project_repositories                                   │ │
│ │  - inspection_jobs                                        │ │
│ │  - inspection_results                                     │ │
│ └────────────────────────────────────────────────────────────┘ │
└────────────────────────────┬───────────────────────────────────┘
                             │ Polling (5s interval)
                             │ Job queue via GET /api/client/poll-jobs
┌────────────────────────────▼───────────────────────────────────┐
│            Jility Desktop Client (Rust + Synthia)              │
│ ┌────────────────────────────────────────────────────────────┐ │
│ │  Main Loop:                                               │ │
│ │  1. Poll server for pending inspection jobs               │ │
│ │  2. Claim job (mark as "running")                         │ │
│ │  3. Clone/update repos to ~/.jility/<project>/<repo>      │ │
│ │  4. Execute AI inspection via Synthia agent               │ │
│ │  5. Upload results to server                              │ │
│ │  6. Show system notification                              │ │
│ └────────────────────────────────────────────────────────────┘ │
│ ┌────────────────────────────────────────────────────────────┐ │
│ │  Synthia Agent Core:                                      │ │
│ │  - AgentActor (orchestrates LLM + tools)                  │ │
│ │  - LLMProvider (Anthropic/OpenAI/Ollama)                  │ │
│ │  - ToolRegistry (Read, Grep, Glob, Git, Powertools)       │ │
│ │  - Custom Tool: UploadResultsTool                         │ │
│ │  - ContextManager (auto-compact on token limits)          │ │
│ └────────────────────────────────────────────────────────────┘ │
│ ┌────────────────────────────────────────────────────────────┐ │
│ │  Local Storage (~/.jility/):                              │ │
│ │  - config.toml (LLM provider, API keys)                   │ │
│ │  - <project-name>/                                        │ │
│ │    - <repo-name>/ (cloned git repos)                      │ │
│ └────────────────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────────┘
```

---

## 3. Data Models

### 3.1 Database Schema

#### Table: `inspection_configs`

Stores inspection types and prompts per project.

```sql
CREATE TABLE inspection_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    inspection_type VARCHAR(100) NOT NULL, -- 'security', 'performance', 'tests', etc.
    enabled BOOLEAN DEFAULT true,
    prompt TEXT NOT NULL, -- AI prompt for this inspection
    scope_pattern VARCHAR(500), -- e.g., "src/auth/**,src/api/**"
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(project_id, inspection_type)
);

CREATE INDEX idx_inspection_configs_project ON inspection_configs(project_id);
```

**Example Row:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "project_id": "123e4567-e89b-12d3-a456-426614174000",
  "inspection_type": "security",
  "enabled": true,
  "prompt": "Focus on SQL injection, XSS, auth bypasses, hardcoded secrets...",
  "scope_pattern": "src/**",
  "created_at": "2025-10-25T12:00:00Z"
}
```

---

#### Table: `project_repositories`

Stores GitHub repository configurations per project.

```sql
CREATE TABLE project_repositories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    repo_url VARCHAR(500) NOT NULL, -- https://github.com/user/repo
    branch VARCHAR(100) DEFAULT 'main',
    github_token_encrypted TEXT, -- AES-256 encrypted read-only token
    last_synced_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(project_id, repo_url)
);

CREATE INDEX idx_project_repositories_project ON project_repositories(project_id);
```

**Example Row:**
```json
{
  "id": "660e8400-e29b-41d4-a716-446655440111",
  "project_id": "123e4567-e89b-12d3-a456-426614174000",
  "repo_url": "https://github.com/acme/api",
  "branch": "main",
  "github_token_encrypted": "AES256:base64encodedciphertext...",
  "last_synced_at": "2025-10-25T11:50:00Z"
}
```

---

#### Table: `inspection_jobs`

Queue of inspection jobs for desktop clients to process.

```sql
CREATE TABLE inspection_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    inspection_config_id UUID NOT NULL REFERENCES inspection_configs(id),
    status VARCHAR(50) DEFAULT 'queued', -- queued, running, completed, failed
    triggered_by UUID REFERENCES users(id), -- User who triggered the inspection
    assigned_to_client VARCHAR(100), -- Desktop client ID that claimed the job
    error_message TEXT, -- If status='failed'
    created_at TIMESTAMPTZ DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ
);

CREATE INDEX idx_inspection_jobs_status ON inspection_jobs(status);
CREATE INDEX idx_inspection_jobs_project ON inspection_jobs(project_id);
```

**Example Row:**
```json
{
  "id": "770e8400-e29b-41d4-a716-446655440222",
  "project_id": "123e4567-e89b-12d3-a456-426614174000",
  "inspection_config_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "running",
  "triggered_by": "user-uuid",
  "assigned_to_client": "client-550e8400",
  "created_at": "2025-10-25T12:00:00Z",
  "started_at": "2025-10-25T12:00:05Z"
}
```

---

#### Table: `inspection_results`

Staged tickets awaiting user approval (preview before creation).

```sql
CREATE TABLE inspection_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    inspection_job_id UUID NOT NULL REFERENCES inspection_jobs(id) ON DELETE CASCADE,
    title VARCHAR(200) NOT NULL, -- e.g., "SQL Injection in User Login"
    description TEXT NOT NULL, -- Full explanation with fix suggestions
    severity VARCHAR(20) NOT NULL, -- critical, high, medium, low
    file_path VARCHAR(1000), -- Relative path: src/auth/login.ts
    line_number INTEGER, -- Line where issue starts
    code_snippet TEXT, -- Relevant code (10 lines max)
    ai_reasoning TEXT NOT NULL, -- Why AI flagged this issue
    approved BOOLEAN DEFAULT false, -- User hasn't approved yet
    ticket_id UUID REFERENCES tickets(id), -- After approval, ticket created
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_inspection_results_job ON inspection_results(inspection_job_id);
CREATE INDEX idx_inspection_results_approved ON inspection_results(approved);
```

**Example Row:**
```json
{
  "id": "880e8400-e29b-41d4-a716-446655440333",
  "inspection_job_id": "770e8400-e29b-41d4-a716-446655440222",
  "title": "SQL Injection in User Login",
  "description": "Direct string interpolation in SQL query allows injection attacks. Replace with parameterized queries using prepared statements.",
  "severity": "critical",
  "file_path": "src/auth/login.ts",
  "line_number": 45,
  "code_snippet": "const query = `SELECT * FROM users WHERE email='${email}'`;",
  "ai_reasoning": "User input is directly concatenated into SQL query without sanitization. An attacker could input `' OR '1'='1` to bypass authentication.",
  "approved": false,
  "ticket_id": null,
  "created_at": "2025-10-25T12:05:00Z"
}
```

---

### 3.2 Client Configuration (~/.jility/config.toml)

Desktop client stores LLM provider settings locally.

```toml
[client]
id = "550e8400-e29b-41d4-a716-446655440000" # Generated on install
server_url = "https://api.jility.dev"
api_key = "jility_api_key_from_web_ui" # User generates in settings

[llm]
provider = "anthropic" # anthropic, openai, ollama, lm-studio
model = "claude-3-5-sonnet-20241022"
api_key = "sk-ant-xxxxx" # User's personal API key (not shared with server)
base_url = "" # For ollama/lm-studio: http://localhost:11434

[timeouts]
bash_timeout = 120
git_timeout = 120
powertools_timeout = 60

[ui]
notifications = true # Show system notifications for completed jobs
```

---

## 4. Component Details

### 4.1 Jility Desktop Client

**Technology Stack:**
- **Language:** Rust 2021
- **Agent Framework:** Synthia (from agent-power-tools)
- **HTTP Client:** reqwest
- **Async Runtime:** tokio
- **Git Operations:** git2 or gitoxide
- **System Tray:** tray-icon crate

**Project Structure:**
```
jility-client/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, setup, main loop
│   ├── config.rs            # Load/save ~/.jility/config.toml
│   ├── api_client.rs        # HTTP client for Jility server API
│   ├── git_manager.rs       # Clone/pull repos
│   ├── inspection_runner.rs # Execute inspections via Synthia
│   ├── tools/
│   │   └── upload_results.rs # Custom Synthia tool
│   └── ui/
│       └── tray.rs          # System tray icon + notifications
└── build.rs                 # Build script for bundling
```

**Main Loop (Pseudocode):**
```rust
#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load config
    let config = Config::load_or_init()?; // First run: wizard

    // 2. Create API client
    let api_client = ApiClient::new(&config.server_url, &config.api_key);

    // 3. Initialize system tray
    let tray = SystemTray::new()?;
    tray.set_icon("idle");

    // 4. Create Synthia agent
    let agent = create_inspection_agent(&config)?;

    // 5. Main loop: poll for jobs
    loop {
        match api_client.poll_jobs().await {
            Ok(jobs) if !jobs.is_empty() => {
                for job in jobs {
                    tray.set_icon("running");

                    // Claim job
                    api_client.claim_job(job.id).await?;

                    // Run inspection
                    match run_inspection(&agent, &job).await {
                        Ok(results) => {
                            // Upload results
                            api_client.upload_results(job.id, results).await?;
                            tray.show_notification(&format!(
                                "Inspection complete: {} issues found",
                                results.len()
                            ));
                        }
                        Err(e) => {
                            api_client.mark_job_failed(job.id, &e.to_string()).await?;
                            tray.show_notification(&format!("Inspection failed: {}", e));
                        }
                    }

                    tray.set_icon("idle");
                }
            }
            Ok(_) => {} // No jobs
            Err(e) => {
                eprintln!("Failed to poll jobs: {}", e);
                // Exponential backoff on errors
            }
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
```

---

### 4.2 Synthia Integration

**How Jility Uses Synthia:**

1. **Extract agent core** - Use Synthia's `AgentActor`, `ToolRegistry`, `LLMProvider` as a library
2. **Add custom tool** - `UploadResultsTool` for sending results to Jility server
3. **Run headless** - No TUI, just agent loop

**Custom Tool: UploadResultsTool**

```rust
use synthia::tools::{Tool, ToolResult};
use async_trait::async_trait;
use serde_json::Value;

pub struct UploadResultsTool {
    api_client: Arc<ApiClient>,
    job_id: Uuid,
}

#[async_trait]
impl Tool for UploadResultsTool {
    fn name(&self) -> &str {
        "upload_inspection_results"
    }

    fn description(&self) -> &str {
        "Upload inspection results to Jility server as staged tickets. \
         Call this with a JSON array of findings after analyzing the codebase."
    }

    fn parameters_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "results": {
                    "type": "array",
                    "description": "Array of inspection findings",
                    "items": {
                        "type": "object",
                        "properties": {
                            "title": { "type": "string" },
                            "description": { "type": "string" },
                            "severity": {
                                "type": "string",
                                "enum": ["critical", "high", "medium", "low"]
                            },
                            "file_path": { "type": "string" },
                            "line_number": { "type": "integer" },
                            "code_snippet": { "type": "string" },
                            "reasoning": { "type": "string" }
                        },
                        "required": ["title", "description", "severity", "reasoning"]
                    }
                }
            },
            "required": ["results"]
        })
    }

    async fn execute(&self, params: Value) -> anyhow::Result<ToolResult> {
        let results: Vec<InspectionResult> = serde_json::from_value(
            params["results"].clone()
        )?;

        // Upload to server
        self.api_client.upload_results(self.job_id, results).await?;

        Ok(ToolResult {
            content: format!("Successfully uploaded {} inspection results to Jility server", results.len()),
            is_error: false,
        })
    }
}
```

**Running an Inspection:**

```rust
async fn run_inspection(
    agent: &mut AgentActor,
    job: &InspectionJob,
) -> Result<Vec<InspectionResult>> {
    // 1. Clone/update repositories
    for repo in &job.repositories {
        let local_path = format!(
            "~/.jility/{}/{}",
            job.project_name,
            extract_repo_name(&repo.url)
        );

        if !Path::new(&local_path).exists() {
            git_clone(&repo.url, &repo.github_token, &local_path).await?;
        } else {
            git_pull(&local_path).await?;
        }
    }

    // 2. Build AI prompt
    let prompt = format!(
        "You are analyzing the codebase at ~/.jility/{}/. \n\
         Run a {} inspection. \n\
         {}\n\
         After your analysis, call the `upload_inspection_results` tool with your findings.",
        job.project_name,
        job.inspection_type,
        job.prompt // Inspection-specific instructions
    );

    // 3. Send message to agent
    agent.send_message(prompt).await?;

    // 4. Wait for completion
    // Agent will:
    //   - Use Read/Grep/Glob/Powertools to scan files
    //   - Analyze code for issues
    //   - Call upload_inspection_results tool

    // 5. Results uploaded by tool, return success
    Ok(vec![]) // Actual results already uploaded to server
}
```

---

### 4.3 Server API Implementation

**Technology Stack:**
- **Language:** Rust
- **Web Framework:** Axum
- **Database:** PostgreSQL (via sqlx)
- **Encryption:** AES-256-GCM for GitHub tokens

**Example Route: POST /api/inspections/trigger**

```rust
use axum::{extract::{Path, State}, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
struct TriggerInspectionRequest {
    inspection_config_id: Uuid,
}

#[derive(Serialize)]
struct TriggerInspectionResponse {
    job_id: Uuid,
    status: String,
    created_at: String,
}

async fn trigger_inspection(
    State(db): State<PgPool>,
    Path(project_id): Path<Uuid>,
    Json(req): Json<TriggerInspectionRequest>,
) -> Result<Json<TriggerInspectionResponse>, ApiError> {
    // 1. Verify inspection config exists and belongs to project
    let config = sqlx::query!(
        "SELECT * FROM inspection_configs WHERE id = $1 AND project_id = $2",
        req.inspection_config_id,
        project_id
    )
    .fetch_one(&db)
    .await?;

    // 2. Create job
    let job_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO inspection_jobs (id, project_id, inspection_config_id, status)
         VALUES ($1, $2, $3, 'queued')",
        job_id,
        project_id,
        req.inspection_config_id
    )
    .execute(&db)
    .await?;

    // 3. Return job info
    Ok(Json(TriggerInspectionResponse {
        job_id,
        status: "queued".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    }))
}
```

**Example Route: GET /api/client/poll-jobs**

```rust
#[derive(Serialize)]
struct PollJobsResponse {
    jobs: Vec<InspectionJobDto>,
}

#[derive(Serialize)]
struct InspectionJobDto {
    id: Uuid,
    project_id: Uuid,
    project_name: String,
    inspection_config_id: Uuid,
    inspection_type: String,
    prompt: String,
    repositories: Vec<RepositoryDto>,
}

async fn poll_jobs(
    State(db): State<PgPool>,
    Extension(client_id): Extension<String>, // From auth middleware
) -> Result<Json<PollJobsResponse>, ApiError> {
    // 1. Find all queued jobs
    let jobs = sqlx::query!(
        r#"
        SELECT
            j.id, j.project_id, p.name as project_name,
            j.inspection_config_id, c.inspection_type, c.prompt
        FROM inspection_jobs j
        JOIN projects p ON j.project_id = p.id
        JOIN inspection_configs c ON j.inspection_config_id = c.id
        WHERE j.status = 'queued'
        ORDER BY j.created_at ASC
        LIMIT 10
        "#
    )
    .fetch_all(&db)
    .await?;

    // 2. For each job, fetch repositories
    let mut job_dtos = vec![];
    for job in jobs {
        let repos = sqlx::query!(
            "SELECT repo_url, branch, github_token_encrypted
             FROM project_repositories
             WHERE project_id = $1",
            job.project_id
        )
        .fetch_all(&db)
        .await?;

        job_dtos.push(InspectionJobDto {
            id: job.id,
            project_id: job.project_id,
            project_name: job.project_name,
            inspection_config_id: job.inspection_config_id,
            inspection_type: job.inspection_type,
            prompt: job.prompt,
            repositories: repos.into_iter().map(|r| RepositoryDto {
                repo_url: r.repo_url,
                branch: r.branch,
                github_token: decrypt_token(&r.github_token_encrypted),
            }).collect(),
        });
    }

    Ok(Json(PollJobsResponse { jobs: job_dtos }))
}
```

---

### 4.4 Web UI Implementation

**Technology Stack:**
- **Framework:** Next.js 14 (App Router)
- **Styling:** Tailwind CSS
- **Icons:** Lucide React
- **Forms:** React Hook Form + Zod validation

**Page: Repository Configuration**

```tsx
// app/projects/[id]/settings/ai-inspections/page.tsx

'use client';

import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { Plus, Trash2 } from 'lucide-react';

export default function AIInspectionsSettings({ params }: { params: { id: string } }) {
  const { data: repos, mutate } = useSWR(`/api/projects/${params.id}/repositories`);

  const { register, handleSubmit } = useForm();

  const onAddRepo = async (data: any) => {
    await fetch(`/api/projects/${params.id}/repositories`, {
      method: 'POST',
      body: JSON.stringify(data),
    });
    mutate();
  };

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-6">AI Code Inspections</h1>

      <section className="mb-8">
        <h2 className="text-lg font-semibold mb-4">📦 Repositories</h2>

        {repos?.map((repo: any) => (
          <div key={repo.id} className="border border-border rounded p-4 mb-2">
            <div className="flex justify-between items-center">
              <div>
                <p className="font-mono text-sm">{repo.repo_url}</p>
                <p className="text-xs text-muted-foreground">Branch: {repo.branch}</p>
              </div>
              <button onClick={() => handleDelete(repo.id)} className="text-destructive">
                <Trash2 className="h-4 w-4" />
              </button>
            </div>
          </div>
        ))}

        <button onClick={() => setShowAddForm(true)} className="btn-primary mt-4">
          <Plus className="h-4 w-4 mr-2" />
          Add Repository
        </button>
      </section>

      <section>
        <h2 className="text-lg font-semibold mb-4">⚙️ GitHub Access Token</h2>
        <input
          type="password"
          placeholder="ghp_xxxxxxxxxxxx"
          className="input"
          {...register('github_token')}
        />
        <p className="text-xs text-muted-foreground mt-2">
          ℹ️ Token needs "repo:read" scope only. Generate at{' '}
          <a href="https://github.com/settings/tokens" className="underline">
            github.com/settings/tokens
          </a>
        </p>
      </section>
    </div>
  );
}
```

**Page: Inspection Results**

```tsx
// app/inspections/[job_id]/results/page.tsx

'use client';

import { useState } from 'react';
import { AlertTriangle, CheckSquare, Square } from 'lucide-react';

export default function InspectionResults({ params }: { params: { job_id: string } }) {
  const { data: results } = useSWR(`/api/inspections/${params.job_id}/results`);
  const [selected, setSelected] = useState<Set<string>>(new Set());

  const toggleAll = () => {
    if (selected.size === results.results.length) {
      setSelected(new Set());
    } else {
      setSelected(new Set(results.results.map((r: any) => r.id)));
    }
  };

  const createTickets = async () => {
    await fetch(`/api/inspections/${params.job_id}/create-tickets`, {
      method: 'POST',
      body: JSON.stringify({ result_ids: Array.from(selected) }),
    });
    router.push(`/projects/${results.project_id}/backlog`);
  };

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-2">Security Inspection Results</h1>
      <p className="text-muted-foreground mb-6">
        Completed: {formatDate(results.completed_at)} • Found {results.results.length} issues
      </p>

      <div className="flex gap-4 mb-6">
        <button onClick={toggleAll} className="btn-secondary">
          {selected.size === results.results.length ? 'Deselect All' : 'Select All'}
          ({selected.size})
        </button>
        <button
          onClick={createTickets}
          disabled={selected.size === 0}
          className="btn-primary"
        >
          Create {selected.size} Tickets
        </button>
      </div>

      <div className="space-y-4">
        {results.results.map((result: any) => (
          <div
            key={result.id}
            className="border border-border rounded p-4 hover:bg-muted/50"
          >
            <div className="flex items-start gap-4">
              <button onClick={() => toggleResult(result.id)}>
                {selected.has(result.id) ? (
                  <CheckSquare className="h-5 w-5 text-primary" />
                ) : (
                  <Square className="h-5 w-5" />
                )}
              </button>

              <div className="flex-1">
                <div className="flex items-center gap-2 mb-2">
                  <SeverityBadge severity={result.severity} />
                  <h3 className="font-semibold">{result.title}</h3>
                </div>

                <p className="text-sm text-muted-foreground mb-2">
                  {result.file_path}:{result.line_number}
                </p>

                <pre className="bg-muted p-3 rounded text-xs overflow-x-auto mb-2">
                  <code>{result.code_snippet}</code>
                </pre>

                <details className="text-sm">
                  <summary className="cursor-pointer font-medium mb-2">
                    AI Reasoning
                  </summary>
                  <p className="text-muted-foreground">{result.ai_reasoning}</p>
                </details>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
```

---

## 5. Data Flow

### 5.1 Inspection Execution Flow

```
┌──────────────┐
│   Web UI     │
│  (User)      │
└──────┬───────┘
       │ 1. POST /api/inspections/trigger
       │    { inspection_config_id: "uuid" }
       ▼
┌──────────────┐
│   Server     │
│  (Rust)      │──► 2. INSERT INTO inspection_jobs (status='queued')
└──────┬───────┘
       │ 3. Job in queue
       │
       │ ┌─────────────────────────────────┐
       │ │  Desktop Client (Polling Loop)  │
       │ └─────────────────────────────────┘
       │         │
       │         │ 4. GET /api/client/poll-jobs (every 5s)
       │         ▼
       │ ┌──────────────┐
       └─┤   Server     │──► 5. SELECT * FROM inspection_jobs WHERE status='queued'
         └──────┬───────┘
                │ 6. Returns job details (project, repos, prompt)
                ▼
         ┌──────────────┐
         │Desktop Client│
         └──────┬───────┘
                │ 7. POST /api/client/jobs/:id/claim
                ▼
         ┌──────────────┐
         │   Server     │──► 8. UPDATE inspection_jobs SET status='running'
         └──────────────┘
                │
                │ 9. Client clones repos locally
                ▼
         ┌──────────────┐
         │Desktop Client│──► 10. git clone https://github.com/...
         │(Git Manager) │      to ~/.jility/<project>/<repo>
         └──────┬───────┘
                │ 11. Repos ready
                ▼
         ┌──────────────┐
         │Desktop Client│──► 12. Execute AI inspection via Synthia
         │(Synthia)     │      - Read files (Read tool)
         │              │      - Search patterns (Grep tool)
         │              │      - Navigate code (Powertools tool)
         │              │      - Generate findings
         └──────┬───────┘
                │ 13. Call upload_inspection_results tool
                ▼
         ┌──────────────┐
         │Desktop Client│──► 14. POST /api/client/jobs/:id/results
         │(API Client)  │       { results: [...] }
         └──────┬───────┘
                │
                ▼
         ┌──────────────┐
         │   Server     │──► 15. INSERT INTO inspection_results (12 rows)
         └──────┬───────┘     UPDATE inspection_jobs SET status='completed'
                │
                │ 16. Results stored
                ▼
         ┌──────────────┐
         │   Web UI     │──► 17. User navigates to /inspections/:job_id/results
         │  (User)      │
         └──────┬───────┘
                │ 18. GET /api/inspections/:job_id/results
                ▼
         ┌──────────────┐
         │   Server     │──► 19. SELECT * FROM inspection_results WHERE job_id=...
         └──────┬───────┘
                │ 20. Returns findings with checkboxes
                ▼
         ┌──────────────┐
         │   Web UI     │──► 21. User selects 10 findings, clicks "Create Tickets"
         │  (User)      │
         └──────┬───────┘
                │ 22. POST /api/inspections/:job_id/create-tickets
                │     { result_ids: [10 uuids] }
                ▼
         ┌──────────────┐
         │   Server     │──► 23. For each result_id:
         └──────────────┘       INSERT INTO tickets (title, description, ...)
                                UPDATE inspection_results SET approved=true, ticket_id=...

                          24. Done! 10 tickets in backlog
```

---

## 6. Security Implementation

### 6.1 GitHub Token Encryption

**Algorithm:** AES-256-GCM

**Implementation:**
```rust
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use base64::{Engine as _, engine::general_purpose};

// Encryption key derived from environment variable
const ENCRYPTION_KEY: &str = env!("JILITY_ENCRYPTION_KEY"); // 32-byte hex string

pub fn encrypt_token(plaintext: &str) -> Result<String> {
    let key = Key::from_slice(ENCRYPTION_KEY.as_bytes());
    let cipher = Aes256Gcm::new(key);

    // Generate random nonce
    let nonce = Nonce::from_slice(b"unique nonce"); // Should be random per encryption

    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| anyhow!("Encryption failed: {}", e))?;

    // Prepend nonce to ciphertext
    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);

    // Base64 encode
    Ok(general_purpose::STANDARD.encode(result))
}

pub fn decrypt_token(encrypted: &str) -> Result<String> {
    let key = Key::from_slice(ENCRYPTION_KEY.as_bytes());
    let cipher = Aes256Gcm::new(key);

    // Base64 decode
    let data = general_purpose::STANDARD.decode(encrypted)?;

    // Extract nonce (first 12 bytes) and ciphertext
    let (nonce, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce);

    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| anyhow!("Decryption failed: {}", e))?;

    Ok(String::from_utf8(plaintext)?)
}
```

---

### 6.2 API Authentication

**Desktop Client Authentication:**

```rust
// Client sends API key in Authorization header
let response = client
    .get("https://api.jility.dev/api/client/poll-jobs")
    .header("Authorization", format!("Bearer {}", api_key))
    .send()
    .await?;
```

**Server Middleware:**

```rust
pub async fn auth_middleware(
    State(db): State<PgPool>,
    headers: HeaderMap,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers.get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header.to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify API key
    let client = sqlx::query!("SELECT id FROM api_keys WHERE key = $1", token)
        .fetch_optional(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Inject client_id into request extensions
    req.extensions_mut().insert(client.id);

    Ok(next.run(req).await)
}
```

---

## 7. Testing Strategy

### 7.1 Unit Tests

**Server Tests:**
```rust
#[tokio::test]
async fn test_trigger_inspection() {
    let db = setup_test_db().await;
    let project_id = create_test_project(&db).await;
    let config_id = create_test_inspection_config(&db, project_id).await;

    let response = trigger_inspection(
        State(db.clone()),
        Path(project_id),
        Json(TriggerInspectionRequest { inspection_config_id: config_id }),
    ).await.unwrap();

    assert_eq!(response.status, "queued");

    // Verify job in database
    let job = sqlx::query!("SELECT * FROM inspection_jobs WHERE id = $1", response.job_id)
        .fetch_one(&db)
        .await
        .unwrap();
    assert_eq!(job.status, "queued");
}
```

**Desktop Client Tests:**
```rust
#[tokio::test]
async fn test_git_clone() {
    let temp_dir = tempfile::tempdir().unwrap();
    let local_path = temp_dir.path().join("test-repo");

    git_clone(
        "https://github.com/octocat/Hello-World",
        None, // Public repo, no token
        &local_path,
    ).await.unwrap();

    assert!(local_path.join(".git").exists());
    assert!(local_path.join("README").exists());
}
```

---

### 7.2 Integration Tests

**End-to-End Test:**
```rust
#[tokio::test]
async fn test_full_inspection_flow() {
    // 1. Setup
    let server = spawn_test_server().await;
    let client = spawn_test_client(&server.url).await;

    // 2. Create project + repo config
    let project_id = server.create_project("Test Project").await;
    server.add_repository(project_id, "https://github.com/test/repo").await;

    // 3. Trigger inspection
    let job_id = server.trigger_inspection(project_id, "security").await;

    // 4. Wait for client to process
    tokio::time::sleep(Duration::from_secs(10)).await;

    // 5. Verify results
    let results = server.get_inspection_results(job_id).await;
    assert!(results.len() > 0, "Should find at least 1 issue");

    // 6. Create tickets
    let ticket_ids = server.create_tickets_from_results(
        job_id,
        results.iter().map(|r| r.id).collect(),
    ).await;
    assert_eq!(ticket_ids.len(), results.len());
}
```

---

## 8. Deployment

### 8.1 Desktop Client Distribution

**Build Process:**
```bash
# macOS (ARM64)
cargo build --release --target aarch64-apple-darwin

# macOS (Intel)
cargo build --release --target x86_64-apple-darwin

# Linux (x86_64)
cargo build --release --target x86_64-unknown-linux-gnu

# Windows
cargo build --release --target x86_64-pc-windows-msvc
```

**Distribution:**
- Host binaries on GitHub Releases
- Provide download links in web app: `/settings/desktop-client`
- Auto-update mechanism (check for new versions on startup)

**Installation Flow:**
1. User downloads binary
2. Runs first-time setup wizard
3. Client registers with server (creates API key)
4. Starts background service
5. Shows system tray icon

---

### 8.2 Server Deployment

**Database Migrations:**
```sql
-- migrations/001_create_inspection_tables.sql

CREATE TABLE inspection_configs (...);
CREATE TABLE project_repositories (...);
CREATE TABLE inspection_jobs (...);
CREATE TABLE inspection_results (...);
```

**Run migrations:**
```bash
sqlx migrate run
```

**Environment Variables:**
```bash
DATABASE_URL=postgresql://user:pass@localhost/jility
JILITY_ENCRYPTION_KEY=0123456789abcdef0123456789abcdef  # 32-byte hex
```

---

## 9. Performance Considerations

### 9.1 Polling Optimization

**Problem:** 1000 desktop clients polling every 5s = 200 req/s

**Solution:** Exponential backoff when no jobs
```rust
let mut poll_interval = Duration::from_secs(5);

loop {
    match api_client.poll_jobs().await {
        Ok(jobs) if !jobs.is_empty() => {
            // Process jobs
            poll_interval = Duration::from_secs(5); // Reset
        }
        Ok(_) => {
            // No jobs, increase interval
            poll_interval = (poll_interval * 2).min(Duration::from_secs(60));
        }
        Err(_) => {
            // Error, increase interval
            poll_interval = (poll_interval * 2).min(Duration::from_secs(300));
        }
    }

    tokio::time::sleep(poll_interval).await;
}
```

---

### 9.2 Large Codebase Handling

**Problem:** 10,000+ file repos exceed LLM context limits

**Solution:** Synthia's ContextManager auto-compacts
- Removes old messages when approaching token limit
- Keeps system prompt + recent tool results
- Summarizes earlier conversation

**Alternative (v0.3):** Scope restrictions
- User specifies file patterns: `src/auth/**`, `src/api/**`
- AI only analyzes specified directories

---

## 10. Future Enhancements

### 10.1 WebSocket Communication (v0.2)

Replace polling with persistent WebSocket connection.

**Benefits:**
- Real-time job notifications
- Lower server load
- Better UX (instant feedback)

**Implementation:**
```rust
// Desktop client
let ws = connect_websocket("wss://api.jility.dev/ws").await?;

loop {
    match ws.recv().await {
        WsMessage::InspectionJobReady(job) => {
            run_inspection(job).await?;
        }
        WsMessage::Ping => {
            ws.send(WsMessage::Pong).await?;
        }
    }
}
```

---

### 10.2 Scheduled Inspections (v0.3)

**Cron-style scheduling:**
```sql
CREATE TABLE inspection_schedules (
    id UUID PRIMARY KEY,
    inspection_config_id UUID REFERENCES inspection_configs(id),
    schedule VARCHAR(100), -- "daily", "weekly", "0 0 * * *" (cron)
    last_run_at TIMESTAMPTZ,
    next_run_at TIMESTAMPTZ
);
```

**Server cron job:**
```rust
// Every minute, check for due inspections
async fn schedule_checker(db: PgPool) {
    loop {
        let due_schedules = sqlx::query!(
            "SELECT * FROM inspection_schedules WHERE next_run_at <= NOW()"
        ).fetch_all(&db).await.unwrap();

        for schedule in due_schedules {
            // Create inspection job
            create_inspection_job(schedule.inspection_config_id).await;

            // Update next_run_at
            update_next_run_time(schedule.id).await;
        }

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
```

---

## 11. Revision History

| Version | Date       | Author | Changes                          |
|---------|------------|--------|----------------------------------|
| 1.0     | 2025-10-25 | Zach   | Initial design document          |

---

## 12. Appendices

### Appendix A: Inspection Prompt Examples

See [AI_CODE_INSPECTIONS_SPEC.md](./AI_CODE_INSPECTIONS_SPEC.md) Section 6.

### Appendix B: Database Schema SQL

See Section 3.1 above.

### Appendix C: Desktop Client Configuration

See Section 3.2 above.

---

**End of Document**
