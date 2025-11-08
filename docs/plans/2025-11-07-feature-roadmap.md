# Jility Feature Roadmap & Implementation Plan

**Created:** 2025-11-07
**Status:** Planning
**Timeline:** 7 weeks (Phases 1-5)

---

## Overview

This document outlines the next major features for Jility, organized by priority and dependencies. Each feature includes detailed implementation steps, file locations, and technical specifications.

### Goals
1. **Enable team collaboration** (Comments)
2. **Support agile planning** (Sprint Planning)
3. **Improve visualization** (Swimlanes, Burndown)
4. **Scale to large projects** (Search, Filters)
5. **Leverage AI** (MCP enhancements, AI breakdown, Git integration)

### Quick Reference
- 🟢 Backend Ready (tables/API exist)
- 🟡 Partial (some infrastructure exists)
- 🔴 New Build (from scratch)

---

## Risk Assessment & Infrastructure Needs

### High-Risk Features (Plan Carefully)

**Global Search (4.1)** ⚠️
- **Risk:** Performance degradation with 10K+ tickets
- **Mitigation:** Implement SQLite FTS5 indexing from the start
- **Fallback:** Consider external search service (Meilisearch/Typesense) if SQLite FTS5 insufficient
- **Decision Point:** Test with 10K+ tickets before production

**AI Epic Breakdown (5.2)** ⚠️
- **Risk:** API costs can escalate quickly
- **Mitigation:**
  - Rate limiting (max 10 requests/hour/user)
  - Cost tracking dashboard
  - User confirmation before API call
- **Decision Point:** Budget $50-100/month for API costs initially

**GitHub Webhooks (5.3)** ⚠️
- **Risk:** Development requires public URL (ngrok/tunneling)
- **Mitigation:**
  - Use ngrok for local dev
  - Document webhook setup clearly
  - Implement webhook signature verification from day 1
- **Security:** NEVER skip signature verification in production

### Missing Infrastructure (Address Before Phase 1)

**Authentication & User Context** 🔧
- How is current user determined in components?
- Where is workspace context stored?
- Document auth flow before starting frontend work

**Error Boundaries** 🔧
- React error boundaries for graceful failures
- Global error handler for API failures
- User-friendly error messages

**Testing Infrastructure** 🔧
- Set up Playwright for integration tests
- Create test data generator (realistic volume)
- CI/CD pipeline for automated testing

**Performance Monitoring** 🔧
- How will you track metrics?
- Set up basic logging (API response times, error rates)
- User session tracking (optional but recommended)

### Pre-Phase 1 Checklist

Before starting any implementation:
- [ ] Create Jility tickets from this roadmap (use `create-roadmap-tickets.ts`)
- [ ] Generate test data (100+ tickets, 5+ users)
- [ ] Document current auth/user context approach
- [ ] Verify WebSocket real-time updates work
- [ ] Set up error boundaries in React app
- [ ] Create integration test template

---

## Phase 1: Core Collaboration (Weeks 1-2)

### 1.1 Comments System ⭐ **START HERE**

**Status:** 🟢 Backend Complete
**Effort:** 2-3 days
**Priority:** Critical

#### Why This First?
- Quick win (backend ready)
- Enables async collaboration
- Table stakes for team adoption
- Builds on existing infrastructure

#### Backend Status ✅
- Table: `comments` (id, ticket_id, author, content, created_at, updated_at)
- API Endpoints:
  - ✅ `POST /api/tickets/:id/comments` (create)
  - ✅ `GET /api/tickets/:id` (returns comments in TicketDetailResponse)
- WebSocket: ✅ Already broadcasting ticket updates

#### Frontend Work Needed

**Files to Modify:**
```
jility-web/
├── components/ticket/
│   ├── comments-section.tsx          [MODIFY] - Hook up to API
│   └── comment-item.tsx              [CREATE] - Individual comment display
├── lib/
│   └── api.ts                        [VERIFY] - Check createComment exists
└── app/w/[slug]/ticket/[id]/page.tsx [VERIFY] - Comments already rendered
```

#### Implementation Steps

**Step 1: Create CommentItem Component** (30 min)
File: `jility-web/components/ticket/comment-item.tsx`

```tsx
'use client'

import { useState } from 'react'
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import { formatDate } from '@/lib/utils'
import { MoreHorizontal, Pencil, Trash2 } from 'lucide-react'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'

interface CommentItemProps {
  comment: {
    id: string
    author: string
    content: string
    created_at: string
    updated_at?: string
  }
  currentUser: string
  onEdit?: (id: string, content: string) => Promise<void>
  onDelete?: (id: string) => Promise<void>
}

export function CommentItem({ comment, currentUser, onEdit, onDelete }: CommentItemProps) {
  const [isEditing, setIsEditing] = useState(false)
  const [editedContent, setEditedContent] = useState(comment.content)
  const [isSaving, setIsSaving] = useState(false)

  const isAuthor = comment.author === currentUser
  const isEdited = comment.updated_at && comment.updated_at !== comment.created_at

  const handleSave = async () => {
    if (!editedContent.trim() || !onEdit) return

    setIsSaving(true)
    try {
      await onEdit(comment.id, editedContent)
      setIsEditing(false)
    } catch (error) {
      console.error('Failed to edit comment:', error)
    } finally {
      setIsSaving(false)
    }
  }

  const handleDelete = async () => {
    if (!onDelete) return
    if (!confirm('Delete this comment?')) return

    try {
      await onDelete(comment.id)
    } catch (error) {
      console.error('Failed to delete comment:', error)
    }
  }

  return (
    <div className="flex gap-3 group">
      <Avatar className="h-8 w-8">
        <AvatarFallback className="text-xs">
          {comment.author.slice(0, 2).toUpperCase()}
        </AvatarFallback>
      </Avatar>

      <div className="flex-1 min-w-0">
        <div className="flex items-baseline gap-2 mb-1">
          <span className="font-medium text-sm">{comment.author}</span>
          <span className="text-xs text-muted-foreground">
            {formatDate(comment.created_at)}
            {isEdited && ' (edited)'}
          </span>
        </div>

        {isEditing ? (
          <div className="space-y-2">
            <Textarea
              value={editedContent}
              onChange={(e) => setEditedContent(e.target.value)}
              className="min-h-20"
            />
            <div className="flex gap-2">
              <Button size="sm" onClick={handleSave} disabled={isSaving}>
                Save
              </Button>
              <Button
                size="sm"
                variant="outline"
                onClick={() => {
                  setEditedContent(comment.content)
                  setIsEditing(false)
                }}
                disabled={isSaving}
              >
                Cancel
              </Button>
            </div>
          </div>
        ) : (
          <div className="prose prose-sm max-w-none">
            <p className="text-sm whitespace-pre-wrap">{comment.content}</p>
          </div>
        )}
      </div>

      {isAuthor && !isEditing && (
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              variant="ghost"
              size="sm"
              className="h-8 w-8 p-0 opacity-0 group-hover:opacity-100"
            >
              <MoreHorizontal className="h-4 w-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem onClick={() => setIsEditing(true)}>
              <Pencil className="h-4 w-4 mr-2" />
              Edit
            </DropdownMenuItem>
            <DropdownMenuItem onClick={handleDelete} className="text-destructive">
              <Trash2 className="h-4 w-4 mr-2" />
              Delete
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      )}
    </div>
  )
}
```

**Step 2: Update CommentsSection Component** (1 hour)
File: `jility-web/components/ticket/comments-section.tsx`

Add these features:
- Map over `comments` array and render `CommentItem` for each
- Handle comment submission via `onAddComment` prop
- Add optimistic UI updates
- Show loading states
- Handle edit/delete (will need new API endpoints)

```tsx
'use client'

import { useState } from 'react'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import { CommentItem } from './comment-item'
import { MessageSquare, Send } from 'lucide-react'

interface Comment {
  id: string
  author: string
  content: string
  created_at: string
  updated_at?: string
}

interface CommentsSectionProps {
  comments: Comment[]
  currentUser?: string
  onAddComment: (content: string) => Promise<void>
  onEditComment?: (id: string, content: string) => Promise<void>
  onDeleteComment?: (id: string) => Promise<void>
}

export function CommentsSection({
  comments,
  currentUser = 'system',
  onAddComment,
  onEditComment,
  onDeleteComment,
}: CommentsSectionProps) {
  const [newComment, setNewComment] = useState('')
  const [isSubmitting, setIsSubmitting] = useState(false)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!newComment.trim()) return

    setIsSubmitting(true)
    try {
      await onAddComment(newComment.trim())
      setNewComment('')
    } catch (error) {
      console.error('Failed to add comment:', error)
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-2">
        <MessageSquare className="h-5 w-5 text-muted-foreground" />
        <h2 className="text-lg font-semibold">
          Comments ({comments.length})
        </h2>
      </div>

      {/* Comment List */}
      <div className="space-y-4">
        {comments.length === 0 ? (
          <p className="text-sm text-muted-foreground">
            No comments yet. Start the conversation!
          </p>
        ) : (
          comments.map((comment) => (
            <CommentItem
              key={comment.id}
              comment={comment}
              currentUser={currentUser}
              onEdit={onEditComment}
              onDelete={onDeleteComment}
            />
          ))
        )}
      </div>

      {/* New Comment Form */}
      <form onSubmit={handleSubmit} className="space-y-3">
        <Textarea
          placeholder="Add a comment..."
          value={newComment}
          onChange={(e) => setNewComment(e.target.value)}
          className="min-h-24"
          disabled={isSubmitting}
        />
        <div className="flex justify-end">
          <Button type="submit" disabled={isSubmitting || !newComment.trim()}>
            {isSubmitting ? (
              'Posting...'
            ) : (
              <>
                <Send className="h-4 w-4 mr-2" />
                Comment
              </>
            )}
          </Button>
        </div>
      </form>
    </div>
  )
}
```

**Step 3: Add Edit/Delete API Endpoints** (Backend - 1 hour)
File: `jility-server/src/api/comments.rs` (CREATE NEW FILE)

```rust
use axum::{extract::{Path, State}, Json};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use uuid::Uuid;
use chrono::Utc;

use crate::{
    error::{ApiError, ApiResult},
    models::{CommentResponse, UpdateCommentRequest},
    state::AppState,
};
use jility_core::entities::{comment, Comment};

/// Update comment
pub async fn update_comment(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateCommentRequest>,
) -> ApiResult<Json<CommentResponse>> {
    let comment_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid comment ID: {}", id)))?;

    let comment = Comment::find_by_id(comment_id)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Comment not found: {}", id)))?;

    let mut comment: comment::ActiveModel = comment.into();
    comment.content = Set(payload.content);
    comment.updated_at = Set(Some(Utc::now()));

    let result = comment
        .update(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    Ok(Json(CommentResponse {
        id: result.id.to_string(),
        ticket_id: result.ticket_id.to_string(),
        author: result.author,
        content: result.content,
        created_at: result.created_at.to_rfc3339(),
        updated_at: result.updated_at.map(|dt| dt.to_rfc3339()),
    }))
}

/// Delete comment
pub async fn delete_comment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let comment_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid comment ID: {}", id)))?;

    Comment::delete_by_id(comment_id)
        .exec(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    Ok(Json(serde_json::json!({ "success": true })))
}
```

**Step 4: Add Routes** (Backend - 5 min)
File: `jility-server/src/api/mod.rs`

```rust
// Add to routes
.route("/api/comments/:id", put(comments::update_comment))
.route("/api/comments/:id", delete(comments::delete_comment))

// Add to module declarations
mod comments;
```

**Step 5: Add Frontend API Functions** (15 min)
File: `jility-web/lib/api.ts`

```typescript
// Add to API object
async updateComment(commentId: string, content: string): Promise<void> {
  const response = await fetch(`${this.baseUrl}/comments/${commentId}`, {
    method: 'PUT',
    headers: this.getHeaders(),
    body: JSON.stringify({ content }),
  })
  if (!response.ok) throw new Error('Failed to update comment')
}

async deleteComment(commentId: string): Promise<void> {
  const response = await fetch(`${this.baseUrl}/comments/${commentId}`, {
    method: 'DELETE',
    headers: this.getHeaders(),
  })
  if (!response.ok) throw new Error('Failed to delete comment')
}
```

**Step 6: Wire Up Ticket Detail Page** (15 min)
File: `jility-web/app/w/[slug]/ticket/[id]/page.tsx`

Add handlers:
```typescript
const handleEditComment = async (id: string, content: string) => {
  try {
    await api.updateComment(id, content)
    await loadTicket()
  } catch (error) {
    console.error('Failed to edit comment:', error)
  }
}

const handleDeleteComment = async (id: string) => {
  try {
    await api.deleteComment(id)
    await loadTicket()
  } catch (error) {
    console.error('Failed to delete comment:', error)
  }
}

// Pass to CommentsSection
<CommentsSection
  comments={ticketDetails.comments}
  currentUser={user?.email} // Get from auth context
  onAddComment={handleAddComment}
  onEditComment={handleEditComment}
  onDeleteComment={handleDeleteComment}
/>
```

#### Testing Checklist
- [ ] Create comment on ticket
- [ ] Comment appears immediately (optimistic update)
- [ ] Edit own comment
- [ ] Delete own comment
- [ ] Cannot edit/delete other users' comments
- [ ] Markdown renders correctly (links, bold, code)
- [ ] @mentions work (if implemented)
- [ ] Long comments wrap properly
- [ ] Mobile responsive

#### Nice-to-Have Enhancements
- Markdown preview tab
- @mention autocomplete
- Reactions (👍, ❤️)
- Comment notifications
- Attachment support

---

## Phase 2: Sprint Planning (Weeks 2-3)

**⚠️ BREAKING DOWN LARGE FEATURE:** Sprint Planning is split into 4 smaller sub-features for easier implementation and testing.

### 2.1 Sprint Backend API ⭐ **START HERE**

**Status:** 🟡 Partial (tables exist, API needs audit)
**Effort:** 1-2 days
**Priority:** High (blocks all other sprint features)

#### Why This First?
- Foundation for all sprint features
- Backend-first approach prevents rework
- Easier to test in isolation
- Tables already exist

#### Backend Status ✅
- Tables: `sprints`, `sprint_tickets` ✅
- Entities: Sprint, SprintTicket ✅
- API: Needs investigation ⚠️

#### Implementation Steps

**Step 1: Investigation** (30 min)
1. Find `jility-server/src/api/sprints.rs` (or check if it exists)
2. List existing endpoints
3. Identify missing endpoints
4. Document current state

**Step 1: Audit Backend API** (1 hour)
File: `jility-server/src/api/sprints.rs`

Check for these endpoints (create if missing):
- `POST /api/projects/:id/sprints` - Create sprint
- `GET /api/projects/:id/sprints` - List sprints
- `GET /api/sprints/:id` - Get sprint details
- `PUT /api/sprints/:id` - Update sprint
- `DELETE /api/sprints/:id` - Delete sprint
- `POST /api/sprints/:id/tickets` - Add ticket to sprint
- `DELETE /api/sprints/:id/tickets/:ticket_id` - Remove from sprint
- `PUT /api/sprints/:id/start` - Start sprint
- `PUT /api/sprints/:id/complete` - Complete sprint

**Step 2: Create Sprint Models** (30 min)
File: `jility-web/lib/types.ts`

```typescript
export interface Sprint {
  id: string
  project_id: string
  name: string
  goal?: string
  start_date?: string
  end_date?: string
  status: 'planning' | 'active' | 'completed'
  created_at: string
  updated_at: string
}

export interface SprintDetails extends Sprint {
  tickets: Ticket[]
  total_points: number
  completed_points: number
}
```

**Step 3: Create Sprint API Functions** (30 min)
File: `jility-web/lib/api.ts`

```typescript
async createSprint(projectId: string, data: {
  name: string
  goal?: string
  start_date?: string
  end_date?: string
}): Promise<Sprint> {
  const response = await fetch(`${this.baseUrl}/projects/${projectId}/sprints`, {
    method: 'POST',
    headers: this.getHeaders(),
    body: JSON.stringify(data),
  })
  if (!response.ok) throw new Error('Failed to create sprint')
  return response.json()
}

async listSprints(projectId: string): Promise<Sprint[]> {
  const response = await fetch(`${this.baseUrl}/projects/${projectId}/sprints`, {
    headers: this.getHeaders(),
  })
  if (!response.ok) throw new Error('Failed to fetch sprints')
  return response.json()
}

async getSprintDetails(sprintId: string): Promise<SprintDetails> {
  const response = await fetch(`${this.baseUrl}/sprints/${sprintId}`, {
    headers: this.getHeaders(),
  })
  if (!response.ok) throw new Error('Failed to fetch sprint')
  return response.json()
}

async addTicketToSprint(sprintId: string, ticketId: string): Promise<void> {
  const response = await fetch(`${this.baseUrl}/sprints/${sprintId}/tickets`, {
    method: 'POST',
    headers: this.getHeaders(),
    body: JSON.stringify({ ticket_id: ticketId }),
  })
  if (!response.ok) throw new Error('Failed to add ticket to sprint')
}

async startSprint(sprintId: string): Promise<Sprint> {
  const response = await fetch(`${this.baseUrl}/sprints/${sprintId}/start`, {
    method: 'PUT',
    headers: this.getHeaders(),
  })
  if (!response.ok) throw new Error('Failed to start sprint')
  return response.json()
}

async completeSprint(sprintId: string): Promise<Sprint> {
  const response = await fetch(`${this.baseUrl}/sprints/${sprintId}/complete`, {
    method: 'PUT',
    headers: this.getHeaders(),
  })
  if (!response.ok) throw new Error('Failed to complete sprint')
  return response.json()
}
```

**Step 4: Create Sprint Dialog** (2 hours)
File: `jility-web/components/sprints/sprint-dialog.tsx`

```tsx
'use client'

import { useState, useEffect } from 'react'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { Label } from '@/components/ui/label'
import { Calendar } from '@/components/ui/calendar'
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover'
import { CalendarIcon } from 'lucide-react'
import { format } from 'date-fns'
import type { Sprint } from '@/lib/types'

interface SprintDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  sprint?: Sprint // If provided, edit mode
  onSave: (data: {
    name: string
    goal?: string
    start_date?: string
    end_date?: string
  }) => Promise<void>
}

export function SprintDialog({ open, onOpenChange, sprint, onSave }: SprintDialogProps) {
  const [name, setName] = useState('')
  const [goal, setGoal] = useState('')
  const [startDate, setStartDate] = useState<Date>()
  const [endDate, setEndDate] = useState<Date>()
  const [isSubmitting, setIsSubmitting] = useState(false)

  useEffect(() => {
    if (sprint) {
      setName(sprint.name)
      setGoal(sprint.goal || '')
      setStartDate(sprint.start_date ? new Date(sprint.start_date) : undefined)
      setEndDate(sprint.end_date ? new Date(sprint.end_date) : undefined)
    } else {
      // Reset for create mode
      setName('')
      setGoal('')
      setStartDate(undefined)
      setEndDate(undefined)
    }
  }, [sprint, open])

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!name.trim()) return

    setIsSubmitting(true)
    try {
      await onSave({
        name: name.trim(),
        goal: goal.trim() || undefined,
        start_date: startDate?.toISOString(),
        end_date: endDate?.toISOString(),
      })
      onOpenChange(false)
    } catch (error) {
      console.error('Failed to save sprint:', error)
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>
            {sprint ? 'Edit Sprint' : 'Create Sprint'}
          </DialogTitle>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="name">Sprint Name *</Label>
            <Input
              id="name"
              placeholder="e.g., Sprint 23"
              value={name}
              onChange={(e) => setName(e.target.value)}
              required
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="goal">Sprint Goal</Label>
            <Textarea
              id="goal"
              placeholder="What do you want to achieve this sprint?"
              value={goal}
              onChange={(e) => setGoal(e.target.value)}
              rows={3}
            />
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label>Start Date</Label>
              <Popover>
                <PopoverTrigger asChild>
                  <Button variant="outline" className="w-full justify-start">
                    <CalendarIcon className="mr-2 h-4 w-4" />
                    {startDate ? format(startDate, 'MMM d, yyyy') : 'Pick date'}
                  </Button>
                </PopoverTrigger>
                <PopoverContent className="w-auto p-0">
                  <Calendar
                    mode="single"
                    selected={startDate}
                    onSelect={setStartDate}
                  />
                </PopoverContent>
              </Popover>
            </div>

            <div className="space-y-2">
              <Label>End Date</Label>
              <Popover>
                <PopoverTrigger asChild>
                  <Button variant="outline" className="w-full justify-start">
                    <CalendarIcon className="mr-2 h-4 w-4" />
                    {endDate ? format(endDate, 'MMM d, yyyy') : 'Pick date'}
                  </Button>
                </PopoverTrigger>
                <PopoverContent className="w-auto p-0">
                  <Calendar
                    mode="single"
                    selected={endDate}
                    onSelect={setEndDate}
                    disabled={(date) => startDate ? date < startDate : false}
                  />
                </PopoverContent>
              </Popover>
            </div>
          </div>

          <div className="flex justify-end gap-2 pt-4">
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={isSubmitting}>
              {sprint ? 'Update' : 'Create'} Sprint
            </Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  )
}
```

**Step 5: Create Sprints List Page** (3 hours)
File: `jility-web/app/w/[slug]/project/[projectId]/sprints/page.tsx`

```tsx
'use client'

import { useState, useEffect } from 'react'
import { useParams, useRouter } from 'next/navigation'
import { api } from '@/lib/api'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { SprintDialog } from '@/components/sprints/sprint-dialog'
import { Plus, Calendar, Target, Play, CheckCircle2 } from 'lucide-react'
import { formatDate } from '@/lib/utils'
import type { Sprint } from '@/lib/types'

export default function SprintsPage() {
  const params = useParams()
  const router = useRouter()
  const projectId = params.projectId as string

  const [sprints, setSprints] = useState<Sprint[]>([])
  const [loading, setLoading] = useState(true)
  const [dialogOpen, setDialogOpen] = useState(false)
  const [editingSprint, setEditingSprint] = useState<Sprint>()

  useEffect(() => {
    loadSprints()
  }, [projectId])

  const loadSprints = async () => {
    try {
      const data = await api.listSprints(projectId)
      setSprints(data)
    } catch (error) {
      console.error('Failed to load sprints:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleCreateSprint = async (data: any) => {
    await api.createSprint(projectId, data)
    await loadSprints()
  }

  const handleStartSprint = async (sprintId: string) => {
    if (!confirm('Start this sprint?')) return
    try {
      await api.startSprint(sprintId)
      await loadSprints()
    } catch (error) {
      console.error('Failed to start sprint:', error)
    }
  }

  const handleCompleteSprint = async (sprintId: string) => {
    if (!confirm('Complete this sprint?')) return
    try {
      await api.completeSprint(sprintId)
      await loadSprints()
    } catch (error) {
      console.error('Failed to complete sprint:', error)
    }
  }

  const activeSprints = sprints.filter(s => s.status === 'active')
  const planningSprints = sprints.filter(s => s.status === 'planning')
  const completedSprints = sprints.filter(s => s.status === 'completed')

  if (loading) return <div>Loading...</div>

  return (
    <div className="container mx-auto px-6 py-8 max-w-6xl">
      <div className="flex justify-between items-center mb-8">
        <h1 className="text-3xl font-bold">Sprints</h1>
        <Button onClick={() => setDialogOpen(true)}>
          <Plus className="h-4 w-4 mr-2" />
          New Sprint
        </Button>
      </div>

      {/* Active Sprint */}
      {activeSprints.length > 0 && (
        <div className="mb-8">
          <h2 className="text-xl font-semibold mb-4">Active Sprint</h2>
          <div className="grid gap-4">
            {activeSprints.map(sprint => (
              <SprintCard
                key={sprint.id}
                sprint={sprint}
                onView={() => router.push(`/w/${params.slug}/sprints/${sprint.id}`)}
                onComplete={() => handleCompleteSprint(sprint.id)}
              />
            ))}
          </div>
        </div>
      )}

      {/* Planning Sprints */}
      {planningSprints.length > 0 && (
        <div className="mb-8">
          <h2 className="text-xl font-semibold mb-4">Planning</h2>
          <div className="grid gap-4">
            {planningSprints.map(sprint => (
              <SprintCard
                key={sprint.id}
                sprint={sprint}
                onView={() => router.push(`/w/${params.slug}/sprints/${sprint.id}`)}
                onStart={() => handleStartSprint(sprint.id)}
              />
            ))}
          </div>
        </div>
      )}

      {/* Completed Sprints */}
      {completedSprints.length > 0 && (
        <div>
          <h2 className="text-xl font-semibold mb-4">Completed</h2>
          <div className="grid gap-4">
            {completedSprints.map(sprint => (
              <SprintCard
                key={sprint.id}
                sprint={sprint}
                onView={() => router.push(`/w/${params.slug}/sprints/${sprint.id}`)}
              />
            ))}
          </div>
        </div>
      )}

      <SprintDialog
        open={dialogOpen}
        onOpenChange={setDialogOpen}
        sprint={editingSprint}
        onSave={handleCreateSprint}
      />
    </div>
  )
}

function SprintCard({ sprint, onView, onStart, onComplete }: {
  sprint: Sprint
  onView: () => void
  onStart?: () => void
  onComplete?: () => void
}) {
  return (
    <Card className="p-6">
      <div className="flex justify-between items-start">
        <div className="flex-1">
          <div className="flex items-center gap-3 mb-2">
            <h3 className="text-lg font-semibold">{sprint.name}</h3>
            <Badge variant={sprint.status as any}>
              {sprint.status}
            </Badge>
          </div>

          {sprint.goal && (
            <div className="flex items-start gap-2 mb-3">
              <Target className="h-4 w-4 mt-0.5 text-muted-foreground" />
              <p className="text-sm text-muted-foreground">{sprint.goal}</p>
            </div>
          )}

          {(sprint.start_date || sprint.end_date) && (
            <div className="flex items-center gap-2 text-sm text-muted-foreground">
              <Calendar className="h-4 w-4" />
              {sprint.start_date && formatDate(sprint.start_date)}
              {sprint.start_date && sprint.end_date && ' - '}
              {sprint.end_date && formatDate(sprint.end_date)}
            </div>
          )}
        </div>

        <div className="flex gap-2">
          <Button variant="outline" onClick={onView}>
            View Details
          </Button>
          {onStart && (
            <Button onClick={onStart}>
              <Play className="h-4 w-4 mr-2" />
              Start Sprint
            </Button>
          )}
          {onComplete && (
            <Button onClick={onComplete}>
              <CheckCircle2 className="h-4 w-4 mr-2" />
              Complete Sprint
            </Button>
          )}
        </div>
      </div>
    </Card>
  )
}
```

**Step 6: Create Sprint Detail Page** (4 hours)
File: `jility-web/app/w/[slug]/sprints/[id]/page.tsx`

Features needed:
- Display sprint info (name, goal, dates, status)
- List all tickets in sprint
- Drag tickets from backlog to add to sprint
- Remove tickets from sprint
- Show capacity (total/completed story points)
- Progress bar
- Action buttons (Start/Complete sprint)

**Step 7: Add Ticket-to-Sprint Actions** (2 hours)
- Add "Add to Sprint" dropdown on ticket cards
- Add "Remove from Sprint" action on sprint detail page
- Update board view to show sprint indicator on tickets

#### Testing Checklist
- [ ] Create new sprint
- [ ] Edit sprint details
- [ ] Add tickets to sprint
- [ ] Remove tickets from sprint
- [ ] Start sprint
- [ ] Complete sprint
- [ ] View sprint history
- [ ] Sprint dates validation (end after start)

---

### 2.2 Sprint Board View

**Status:** 🟡 Partial (board exists, needs sprint filter)
**Effort:** 2 days
**Dependency:** 2.1 must be complete

#### Implementation Steps

**Step 1: Add Sprint Filter to Board** (1 hour)
File: `jility-web/app/w/[slug]/project/[projectId]/board/page.tsx`

Add sprint selector dropdown:
```tsx
<Select value={selectedSprint} onValueChange={setSelectedSprint}>
  <SelectTrigger className="w-48">
    <SelectValue placeholder="All tickets" />
  </SelectTrigger>
  <SelectContent>
    <SelectItem value="all">All tickets</SelectItem>
    <SelectItem value="backlog">Backlog</SelectItem>
    {sprints.map(sprint => (
      <SelectItem key={sprint.id} value={sprint.id}>
        {sprint.name}
      </SelectItem>
    ))}
  </SelectContent>
</Select>
```

**Step 2: Filter Tickets by Sprint** (30 min)
```tsx
const filteredTickets = tickets.filter(ticket => {
  if (selectedSprint === 'all') return true
  if (selectedSprint === 'backlog') return !ticket.sprint_id
  return ticket.sprint_id === selectedSprint
})
```

**Step 3: Add Sprint Indicator to Ticket Cards** (30 min)
Show sprint badge on each ticket card if assigned to sprint

**Step 4: Quick Add to Sprint** (1 hour)
Add sprint dropdown on ticket card for quick assignment

#### Testing Checklist
- [ ] Filter board by sprint
- [ ] View backlog (no sprint)
- [ ] Add ticket to sprint from board
- [ ] Sprint badge displays correctly
- [ ] Mobile responsive

---

## Phase 3: Visual Workflows (Week 4)

### 3.1 Swimlanes for Board ⭐

**Status:** 🔴 New Build
**Effort:** 4-5 days
**Priority:** Medium-High

#### Why Swimlanes?
- Managers love them ("show me what each person is working on")
- Visual grouping improves clarity
- Multiple perspectives on same data
- Differentiating feature vs competitors

#### Grouping Options
- **By Assignee**: See each team member's work across all statuses
- **By Epic**: Track feature progress across columns
- **By Priority**: High/Medium/Low priority grouping
- **By Labels**: Group by tags (Backend, Frontend, Bug, etc.)

#### Implementation Steps

**Step 1: Design Data Structure** (1 hour)

Need to group tickets by selected dimension while preserving column structure:
```typescript
type GroupBy = 'none' | 'assignee' | 'epic' | 'priority' | 'label'

interface Swimlane {
  id: string
  title: string
  tickets: {
    [status: string]: Ticket[]
  }
}
```

**Step 2: Create Grouping Logic** (2 hours)
File: `jility-web/lib/board-utils.ts` (CREATE NEW)

```typescript
export function groupTickets(
  tickets: Ticket[],
  groupBy: GroupBy,
  statuses: string[]
): Swimlane[] {
  if (groupBy === 'none') {
    return [{
      id: 'all',
      title: 'All Tickets',
      tickets: groupByStatus(tickets, statuses)
    }]
  }

  const grouped = new Map<string, Ticket[]>()

  // Group tickets
  tickets.forEach(ticket => {
    const key = getGroupKey(ticket, groupBy)
    if (!grouped.has(key)) {
      grouped.set(key, [])
    }
    grouped.get(key)!.push(ticket)
  })

  // Convert to swimlanes
  const swimlanes: Swimlane[] = []
  grouped.forEach((tickets, key) => {
    swimlanes.push({
      id: key,
      title: getGroupTitle(key, groupBy),
      tickets: groupByStatus(tickets, statuses)
    })
  })

  // Add empty swimlane for unassigned
  if (groupBy === 'assignee') {
    const unassigned = tickets.filter(t => t.assignees.length === 0)
    if (unassigned.length > 0 || true) { // Always show
      swimlanes.push({
        id: 'unassigned',
        title: 'Unassigned',
        tickets: groupByStatus(unassigned, statuses)
      })
    }
  }

  return swimlanes
}

function getGroupKey(ticket: Ticket, groupBy: GroupBy): string {
  switch (groupBy) {
    case 'assignee':
      return ticket.assignees[0] || 'unassigned'
    case 'epic':
      return ticket.epic_id || 'no-epic'
    case 'priority':
      return ticket.priority || 'medium'
    case 'label':
      return ticket.labels[0] || 'no-label'
    default:
      return 'all'
  }
}

function getGroupTitle(key: string, groupBy: GroupBy): string {
  if (key === 'unassigned') return 'Unassigned'
  if (key === 'no-epic') return 'No Epic'
  if (key === 'no-label') return 'No Label'
  // For assignee, epic, label - the key is the actual value
  return key
}

function groupByStatus(
  tickets: Ticket[],
  statuses: string[]
): { [status: string]: Ticket[] } {
  const grouped: { [status: string]: Ticket[] } = {}

  statuses.forEach(status => {
    grouped[status] = tickets.filter(t => t.status === status)
  })

  return grouped
}
```

**Step 3: Create Swimlane Component** (4 hours)
File: `jility-web/components/board/board-swimlanes.tsx` (CREATE NEW)

```tsx
'use client'

import { useState } from 'react'
import { TicketCard } from './ticket-card'
import { Button } from '@/components/ui/button'
import { ChevronDown, ChevronRight } from 'lucide-react'
import type { Ticket } from '@/lib/types'
import type { Swimlane } from '@/lib/board-utils'

interface BoardSwimlanesProps {
  swimlanes: Swimlane[]
  statuses: { value: string; label: string; color: string }[]
  onTicketMove: (ticketId: string, newStatus: string) => void
  onTicketClick: (ticket: Ticket) => void
}

export function BoardSwimlanes({
  swimlanes,
  statuses,
  onTicketMove,
  onTicketClick,
}: BoardSwimlanesProps) {
  const [collapsed, setCollapsed] = useState<Set<string>>(new Set())

  const toggleCollapse = (swimlaneId: string) => {
    setCollapsed(prev => {
      const next = new Set(prev)
      if (next.has(swimlaneId)) {
        next.delete(swimlaneId)
      } else {
        next.add(swimlaneId)
      }
      return next
    })
  }

  return (
    <div className="space-y-4">
      {swimlanes.map(swimlane => {
        const isCollapsed = collapsed.has(swimlane.id)
        const totalTickets = Object.values(swimlane.tickets)
          .flat().length

        return (
          <div key={swimlane.id} className="border rounded-lg">
            {/* Swimlane Header */}
            <div className="bg-muted/50 px-4 py-2 flex items-center justify-between">
              <button
                onClick={() => toggleCollapse(swimlane.id)}
                className="flex items-center gap-2 hover:text-primary"
              >
                {isCollapsed ? (
                  <ChevronRight className="h-4 w-4" />
                ) : (
                  <ChevronDown className="h-4 w-4" />
                )}
                <span className="font-medium">{swimlane.title}</span>
                <span className="text-sm text-muted-foreground">
                  ({totalTickets})
                </span>
              </button>
            </div>

            {/* Swimlane Content */}
            {!isCollapsed && (
              <div className="p-4">
                <div className="grid gap-4" style={{
                  gridTemplateColumns: `repeat(${statuses.length}, minmax(250px, 1fr))`
                }}>
                  {statuses.map(status => (
                    <div key={status.value}>
                      <div className="space-y-2">
                        {swimlane.tickets[status.value]?.map(ticket => (
                          <TicketCard
                            key={ticket.id}
                            ticket={ticket}
                            onClick={() => onTicketClick(ticket)}
                            onStatusChange={(newStatus) =>
                              onTicketMove(ticket.id, newStatus)
                            }
                          />
                        ))}
                        {swimlane.tickets[status.value]?.length === 0 && (
                          <div className="text-center py-4 text-sm text-muted-foreground">
                            No tickets
                          </div>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        )
      })}
    </div>
  )
}
```

**Step 4: Update Board Page** (2 hours)
File: `jility-web/app/w/[slug]/project/[projectId]/board/page.tsx`

Add groupBy selector:
```tsx
const [groupBy, setGroupBy] = useState<GroupBy>('none')

const swimlanes = useMemo(() => {
  return groupTickets(tickets, groupBy, STATUSES)
}, [tickets, groupBy])

// Add UI control
<Select value={groupBy} onValueChange={setGroupBy}>
  <SelectTrigger className="w-48">
    <SelectValue />
  </SelectTrigger>
  <SelectContent>
    <SelectItem value="none">No Grouping</SelectItem>
    <SelectItem value="assignee">Group by Assignee</SelectItem>
    <SelectItem value="epic">Group by Epic</SelectItem>
    <SelectItem value="priority">Group by Priority</SelectItem>
    <SelectItem value="label">Group by Label</SelectItem>
  </SelectContent>
</Select>

// Render swimlanes or regular board
{groupBy === 'none' ? (
  <RegularBoard ... />
) : (
  <BoardSwimlanes
    swimlanes={swimlanes}
    statuses={STATUSES}
    onTicketMove={handleTicketMove}
    onTicketClick={handleTicketClick}
  />
)}
```

**Step 5: Add Drag-and-Drop for Swimlanes** (4 hours)
Use `@dnd-kit/core` library:

```bash
npm install @dnd-kit/core @dnd-kit/sortable
```

Update swimlane component to support:
- Drag tickets between columns within same swimlane
- Drag tickets between swimlanes (changes assignee/epic/etc)
- Visual feedback during drag

**Step 6: Persist User Preference** (1 hour)
Save groupBy preference to localStorage:
```typescript
useEffect(() => {
  localStorage.setItem('board-group-by', groupBy)
}, [groupBy])
```

#### Testing Checklist
- [ ] Group by assignee
- [ ] Group by epic
- [ ] Group by priority
- [ ] Group by label
- [ ] Collapse/expand swimlanes
- [ ] Drag tickets between columns in swimlane
- [ ] Drag tickets between swimlanes
- [ ] Unassigned swimlane shows correctly
- [ ] Preference persists on reload
- [ ] Mobile responsive (horizontal scroll)

---

### 3.2 Burndown Chart

**Status:** 🔴 New Build
**Effort:** 3-4 days
**Dependency:** 2.1 (Sprint Planning)
**Priority:** Medium

#### Why Burndown Charts?
- Managers ask "are we on track?" daily
- Visual progress tracking
- Identify velocity trends
- Predict completion dates

#### Chart Types to Build
1. **Sprint Burndown** - Daily remaining points
2. **Velocity Chart** - Points per sprint over time
3. **Cumulative Flow** - Work in each status over time (optional)

#### Implementation Steps

**Step 1: Install Chart Library** (5 min)
```bash
npm install recharts
```

**Step 2: Create Burndown Data Endpoint** (Backend - 2 hours)
File: `jility-server/src/api/sprints.rs`

```rust
/// Get burndown data for sprint
pub async fn get_sprint_burndown(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<Vec<BurndownPoint>>> {
    let sprint_id = Uuid::parse_str(&id)
        .map_err(|_| ApiError::InvalidInput(format!("Invalid sprint ID: {}", id)))?;

    let sprint = Sprint::find_by_id(sprint_id)
        .one(state.db.as_ref())
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::NotFound(format!("Sprint not found: {}", id)))?;

    // Get all tickets in sprint
    let sprint_tickets = SprintTicket::find()
        .filter(sprint_ticket::Column::SprintId.eq(sprint_id))
        .find_with_related(Ticket)
        .all(state.db.as_ref())
        .await
        .map_err(ApiError::from)?;

    // Calculate total points
    let total_points: i32 = sprint_tickets
        .iter()
        .filter_map(|(_, ticket)| ticket.story_points)
        .sum();

    // Get ticket changes during sprint to calculate daily burndown
    let start_date = sprint.start_date.unwrap_or(sprint.created_at);
    let end_date = sprint.end_date.unwrap_or(Utc::now());

    let mut burndown_points = vec![];
    let mut current_date = start_date.date();
    let end = end_date.date();

    while current_date <= end {
        // Count completed points as of this date
        let completed_points = calculate_completed_points_at_date(
            &state.db,
            &sprint_tickets,
            current_date
        ).await?;

        burndown_points.push(BurndownPoint {
            date: current_date.to_string(),
            remaining: total_points - completed_points,
            ideal: calculate_ideal_remaining(
                total_points,
                start_date.date(),
                end_date.date(),
                current_date
            ),
        });

        current_date = current_date.succ_opt().unwrap();
    }

    Ok(Json(burndown_points))
}

async fn calculate_completed_points_at_date(
    db: &DatabaseConnection,
    sprint_tickets: &[(SprintTicket, Ticket)],
    date: NaiveDate,
) -> Result<i32, ApiError> {
    let mut completed = 0;

    for (_, ticket) in sprint_tickets {
        // Check if ticket was completed by this date
        let completion_change = TicketChange::find()
            .filter(ticket_change::Column::TicketId.eq(ticket.id))
            .filter(ticket_change::Column::ChangeType.eq("status_changed"))
            .filter(ticket_change::Column::NewValue.eq("done"))
            .filter(ticket_change::Column::ChangedAt.lte(date.and_hms_opt(23, 59, 59).unwrap()))
            .one(db)
            .await
            .map_err(ApiError::from)?;

        if completion_change.is_some() {
            completed += ticket.story_points.unwrap_or(0);
        }
    }

    Ok(completed)
}

fn calculate_ideal_remaining(
    total: i32,
    start: NaiveDate,
    end: NaiveDate,
    current: NaiveDate,
) -> i32 {
    let total_days = (end - start).num_days() as f64;
    let days_elapsed = (current - start).num_days() as f64;

    if total_days == 0.0 {
        return 0;
    }

    let progress = days_elapsed / total_days;
    (total as f64 * (1.0 - progress)) as i32
}
```

**Step 3: Add Response Model** (Backend - 10 min)
File: `jility-server/src/models/response.rs`

```rust
#[derive(Debug, Serialize)]
pub struct BurndownPoint {
    pub date: String,
    pub remaining: i32,
    pub ideal: i32,
}
```

**Step 4: Add Route** (Backend - 5 min)
```rust
.route("/api/sprints/:id/burndown", get(sprints::get_sprint_burndown))
```

**Step 5: Create Burndown Chart Component** (Frontend - 3 hours)
File: `jility-web/components/sprints/burndown-chart.tsx`

```tsx
'use client'

import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts'

interface BurndownPoint {
  date: string
  remaining: number
  ideal: number
}

interface BurndownChartProps {
  data: BurndownPoint[]
  totalPoints: number
}

export function BurndownChart({ data, totalPoints }: BurndownChartProps) {
  return (
    <div className="w-full h-96">
      <ResponsiveContainer width="100%" height="100%">
        <LineChart data={data} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis
            dataKey="date"
            tick={{ fontSize: 12 }}
            tickFormatter={(value) => {
              const date = new Date(value)
              return `${date.getMonth() + 1}/${date.getDate()}`
            }}
          />
          <YAxis
            label={{ value: 'Story Points', angle: -90, position: 'insideLeft' }}
          />
          <Tooltip
            labelFormatter={(value) => new Date(value).toLocaleDateString()}
          />
          <Legend />
          <Line
            type="monotone"
            dataKey="ideal"
            stroke="#94a3b8"
            strokeDasharray="5 5"
            name="Ideal Burndown"
          />
          <Line
            type="monotone"
            dataKey="remaining"
            stroke="#3b82f6"
            strokeWidth={2}
            name="Actual Remaining"
          />
        </LineChart>
      </ResponsiveContainer>

      <div className="mt-4 grid grid-cols-3 gap-4 text-center">
        <div>
          <div className="text-sm text-muted-foreground">Total Points</div>
          <div className="text-2xl font-bold">{totalPoints}</div>
        </div>
        <div>
          <div className="text-sm text-muted-foreground">Remaining</div>
          <div className="text-2xl font-bold">
            {data[data.length - 1]?.remaining || 0}
          </div>
        </div>
        <div>
          <div className="text-sm text-muted-foreground">Completed</div>
          <div className="text-2xl font-bold">
            {totalPoints - (data[data.length - 1]?.remaining || 0)}
          </div>
        </div>
      </div>
    </div>
  )
}
```

**Step 6: Add to Sprint Detail Page** (1 hour)
File: `jility-web/app/w/[slug]/sprints/[id]/page.tsx`

```tsx
const [burndownData, setBurndownData] = useState<BurndownPoint[]>([])

useEffect(() => {
  const loadBurndown = async () => {
    const data = await api.getSprintBurndown(sprintId)
    setBurndownData(data)
  }
  if (sprint.status === 'active' || sprint.status === 'completed') {
    loadBurndown()
  }
}, [sprintId, sprint.status])

// Render
{(sprint.status === 'active' || sprint.status === 'completed') && (
  <Card className="p-6">
    <h2 className="text-xl font-semibold mb-4">Burndown Chart</h2>
    <BurndownChart data={burndownData} totalPoints={sprint.total_points} />
  </Card>
)}
```

**Step 7: Create Velocity Chart** (2 hours)
File: `jility-web/components/sprints/velocity-chart.tsx`

Shows points completed per sprint over time (bar chart)

**Step 8: Export Chart as Image** (1 hour)
Add "Export as PNG" button using `html2canvas`

#### Testing Checklist
- [ ] Burndown chart displays for active sprint
- [ ] Ideal line calculates correctly
- [ ] Actual line updates when tickets complete
- [ ] Chart shows for completed sprints
- [ ] Velocity chart shows trend
- [ ] Export as image works
- [ ] Mobile responsive
- [ ] Chart updates in real-time

---

## Phase 4: Search & Discovery (Week 5)

### 4.1 Global Search

**Status:** 🟡 Partial (backend FTS5 schema exists)
**Effort:** 3-4 days
**Priority:** Medium

#### Why Search?
- Essential at 50+ tickets
- Users need quick ticket lookup
- Search by ID, title, description, comments
- Keyboard shortcut (Cmd+K) power users love

#### Backend Infrastructure ✅
- SQLite FTS5 virtual table mentioned in schema
- Need to verify/implement full-text search

#### Implementation Steps

**Step 1: Implement FTS5 Backend** (2 hours)
File: `jility-server/src/search/mod.rs` (CREATE NEW)

```rust
use sea_orm::{DatabaseConnection, Statement, DbBackend};
use serde::Serialize;
use anyhow::Result;

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub ticket_id: String,
    pub ticket_number: String,
    pub title: String,
    pub description: String,
    pub snippet: String,
    pub score: f64,
}

pub async fn search_tickets(
    db: &DatabaseConnection,
    query: &str,
    limit: u64,
) -> Result<Vec<SearchResult>> {
    // SQLite FTS5 search
    let sql = r#"
        SELECT
            t.id,
            t.ticket_number,
            t.title,
            t.description,
            snippet(tickets_fts, 1, '<mark>', '</mark>', '...', 32) as snippet,
            rank
        FROM tickets_fts
        INNER JOIN tickets t ON t.id = tickets_fts.ticket_id
        WHERE tickets_fts MATCH ?
        ORDER BY rank
        LIMIT ?
    "#;

    let results = db
        .query_all(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            vec![query.into(), limit.into()],
        ))
        .await?;

    // Parse results
    let mut search_results = vec![];
    for row in results {
        search_results.push(SearchResult {
            ticket_id: row.try_get("", "id")?,
            ticket_number: row.try_get("", "ticket_number")?,
            title: row.try_get("", "title")?,
            description: row.try_get("", "description")?,
            snippet: row.try_get("", "snippet")?,
            score: row.try_get("", "rank")?,
        });
    }

    Ok(search_results)
}
```

**Step 2: Create Search Endpoint** (30 min)
File: `jility-server/src/api/search.rs` (CREATE NEW)

```rust
use axum::{extract::{Query, State}, Json};
use serde::Deserialize;

use crate::{
    error::ApiResult,
    search::search_tickets,
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default = "default_limit")]
    pub limit: u64,
}

fn default_limit() -> u64 { 20 }

pub async fn search(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> ApiResult<Json<Vec<SearchResult>>> {
    let results = search_tickets(state.db.as_ref(), &query.q, query.limit)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(results))
}
```

**Step 3: Create Search Command** (Frontend - 3 hours)
File: `jility-web/components/search/command-menu.tsx`

```tsx
'use client'

import { useState, useEffect } from 'react'
import { useRouter } from 'next/navigation'
import {
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from '@/components/ui/command'
import { Search, FileText } from 'lucide-react'
import { api } from '@/lib/api'
import { useDebounce } from '@/lib/hooks/use-debounce'

export function CommandMenu() {
  const router = useRouter()
  const [open, setOpen] = useState(false)
  const [query, setQuery] = useState('')
  const [results, setResults] = useState<SearchResult[]>([])
  const [loading, setLoading] = useState(false)

  const debouncedQuery = useDebounce(query, 300)

  // Keyboard shortcut
  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === 'k' && (e.metaKey || e.ctrlKey)) {
        e.preventDefault()
        setOpen((open) => !open)
      }
    }
    document.addEventListener('keydown', down)
    return () => document.removeEventListener('keydown', down)
  }, [])

  // Search
  useEffect(() => {
    if (!debouncedQuery) {
      setResults([])
      return
    }

    const search = async () => {
      setLoading(true)
      try {
        const data = await api.search(debouncedQuery)
        setResults(data)
      } catch (error) {
        console.error('Search failed:', error)
      } finally {
        setLoading(false)
      }
    }

    search()
  }, [debouncedQuery])

  const handleSelect = (ticketId: string) => {
    router.push(`/tickets/${ticketId}`)
    setOpen(false)
    setQuery('')
  }

  return (
    <>
      <Button
        variant="outline"
        className="relative w-64 justify-start text-sm text-muted-foreground"
        onClick={() => setOpen(true)}
      >
        <Search className="mr-2 h-4 w-4" />
        <span>Search tickets...</span>
        <kbd className="pointer-events-none absolute right-2 top-2 hidden h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-xs font-medium opacity-100 sm:flex">
          <span className="text-xs">⌘</span>K
        </kbd>
      </Button>

      <CommandDialog open={open} onOpenChange={setOpen}>
        <CommandInput
          placeholder="Search tickets..."
          value={query}
          onValueChange={setQuery}
        />
        <CommandList>
          <CommandEmpty>
            {loading ? 'Searching...' : 'No results found.'}
          </CommandEmpty>
          {results.length > 0 && (
            <CommandGroup heading="Tickets">
              {results.map((result) => (
                <CommandItem
                  key={result.ticket_id}
                  value={result.ticket_id}
                  onSelect={() => handleSelect(result.ticket_id)}
                >
                  <FileText className="mr-2 h-4 w-4" />
                  <div className="flex-1">
                    <div className="font-medium">
                      {result.ticket_number} - {result.title}
                    </div>
                    <div
                      className="text-sm text-muted-foreground"
                      dangerouslySetInnerHTML={{ __html: result.snippet }}
                    />
                  </div>
                </CommandItem>
              ))}
            </CommandGroup>
          )}
        </CommandList>
      </CommandDialog>
    </>
  )
}
```

**Step 4: Add to Navbar** (15 min)
File: `jility-web/components/layout/navbar.tsx`

Add CommandMenu component to navbar

**Step 5: Create Search Results Page** (2 hours)
File: `jility-web/app/w/[slug]/search/page.tsx`

Full search results page with filters, pagination

#### Testing Checklist
- [ ] Cmd+K / Ctrl+K opens search
- [ ] Search by ticket ID
- [ ] Search by title
- [ ] Search by description
- [ ] Search highlights matches
- [ ] Results clickable
- [ ] Keyboard navigation works
- [ ] Search results page displays

---

### 4.2 Board Filters

**Status:** 🔴 New Build
**Effort:** 2-3 days
**Priority:** Medium

#### Filter Options
- **Quick filters**: My tickets, Unassigned, Blocked
- **Advanced filters**: Assignee, Label, Epic, Sprint, Status
- **Save custom views**: "Frontend Bugs", "Q4 Features"
- **URL sharing**: Share filtered view via URL

#### Implementation Steps

**Step 1: Create Filter Panel Component** (2 hours)
File: `jility-web/components/board/filter-panel.tsx`

```tsx
'use client'

import { useState } from 'react'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Checkbox } from '@/components/ui/checkbox'
import { Label } from '@/components/ui/label'
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from '@/components/ui/sheet'
import { Filter, X } from 'lucide-react'

export interface BoardFilters {
  assignees: string[]
  labels: string[]
  epics: string[]
  statuses: string[]
  sprints: string[]
}

interface FilterPanelProps {
  filters: BoardFilters
  onFiltersChange: (filters: BoardFilters) => void
  availableAssignees: string[]
  availableLabels: string[]
  availableEpics: { id: string; title: string }[]
  availableSprints: { id: string; name: string }[]
}

export function FilterPanel({
  filters,
  onFiltersChange,
  availableAssignees,
  availableLabels,
  availableEpics,
  availableSprints,
}: FilterPanelProps) {
  const activeFiltersCount =
    filters.assignees.length +
    filters.labels.length +
    filters.epics.length +
    filters.statuses.length +
    filters.sprints.length

  const clearFilters = () => {
    onFiltersChange({
      assignees: [],
      labels: [],
      epics: [],
      statuses: [],
      sprints: [],
    })
  }

  const toggleFilter = (
    category: keyof BoardFilters,
    value: string
  ) => {
    const current = filters[category]
    const updated = current.includes(value)
      ? current.filter(v => v !== value)
      : [...current, value]

    onFiltersChange({
      ...filters,
      [category]: updated,
    })
  }

  return (
    <Sheet>
      <SheetTrigger asChild>
        <Button variant="outline" className="relative">
          <Filter className="h-4 w-4 mr-2" />
          Filters
          {activeFiltersCount > 0 && (
            <Badge variant="secondary" className="ml-2">
              {activeFiltersCount}
            </Badge>
          )}
        </Button>
      </SheetTrigger>
      <SheetContent>
        <SheetHeader>
          <SheetTitle>Filter Tickets</SheetTitle>
        </SheetHeader>

        <div className="mt-6 space-y-6">
          {/* Quick Filters */}
          <div>
            <h3 className="font-medium mb-3">Quick Filters</h3>
            <div className="space-y-2">
              <Button
                variant="outline"
                size="sm"
                className="w-full justify-start"
                onClick={() => {
                  // Set to current user
                }}
              >
                My Tickets
              </Button>
              <Button
                variant="outline"
                size="sm"
                className="w-full justify-start"
                onClick={() => {
                  onFiltersChange({
                    ...filters,
                    assignees: ['unassigned'],
                  })
                }}
              >
                Unassigned
              </Button>
              <Button
                variant="outline"
                size="sm"
                className="w-full justify-start"
                onClick={() => {
                  onFiltersChange({
                    ...filters,
                    statuses: ['blocked'],
                  })
                }}
              >
                Blocked
              </Button>
            </div>
          </div>

          {/* Assignees */}
          <div>
            <h3 className="font-medium mb-3">Assignee</h3>
            <div className="space-y-2">
              {availableAssignees.map(assignee => (
                <div key={assignee} className="flex items-center">
                  <Checkbox
                    id={`assignee-${assignee}`}
                    checked={filters.assignees.includes(assignee)}
                    onCheckedChange={() => toggleFilter('assignees', assignee)}
                  />
                  <Label
                    htmlFor={`assignee-${assignee}`}
                    className="ml-2 text-sm cursor-pointer"
                  >
                    {assignee}
                  </Label>
                </div>
              ))}
            </div>
          </div>

          {/* Labels */}
          <div>
            <h3 className="font-medium mb-3">Labels</h3>
            <div className="space-y-2">
              {availableLabels.map(label => (
                <div key={label} className="flex items-center">
                  <Checkbox
                    id={`label-${label}`}
                    checked={filters.labels.includes(label)}
                    onCheckedChange={() => toggleFilter('labels', label)}
                  />
                  <Label
                    htmlFor={`label-${label}`}
                    className="ml-2 text-sm cursor-pointer"
                  >
                    {label}
                  </Label>
                </div>
              ))}
            </div>
          </div>

          {/* Epics */}
          {availableEpics.length > 0 && (
            <div>
              <h3 className="font-medium mb-3">Epics</h3>
              <div className="space-y-2">
                {availableEpics.map(epic => (
                  <div key={epic.id} className="flex items-center">
                    <Checkbox
                      id={`epic-${epic.id}`}
                      checked={filters.epics.includes(epic.id)}
                      onCheckedChange={() => toggleFilter('epics', epic.id)}
                    />
                    <Label
                      htmlFor={`epic-${epic.id}`}
                      className="ml-2 text-sm cursor-pointer"
                    >
                      {epic.title}
                    </Label>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Clear Filters */}
          {activeFiltersCount > 0 && (
            <Button
              variant="outline"
              size="sm"
              className="w-full"
              onClick={clearFilters}
            >
              <X className="h-4 w-4 mr-2" />
              Clear All Filters
            </Button>
          )}
        </div>
      </SheetContent>
    </Sheet>
  )
}
```

**Step 2: Add Filter Logic to Board** (1 hour)
File: `jility-web/app/w/[slug]/project/[projectId]/board/page.tsx`

```tsx
const [filters, setFilters] = useState<BoardFilters>({
  assignees: [],
  labels: [],
  epics: [],
  statuses: [],
  sprints: [],
})

const filteredTickets = useMemo(() => {
  return tickets.filter(ticket => {
    // Filter by assignees
    if (filters.assignees.length > 0) {
      if (filters.assignees.includes('unassigned')) {
        if (ticket.assignees.length > 0) return false
      } else {
        const hasMatch = ticket.assignees.some(a =>
          filters.assignees.includes(a)
        )
        if (!hasMatch) return false
      }
    }

    // Filter by labels
    if (filters.labels.length > 0) {
      const hasMatch = ticket.labels.some(l =>
        filters.labels.includes(l)
      )
      if (!hasMatch) return false
    }

    // Filter by epics
    if (filters.epics.length > 0) {
      if (!ticket.epic_id || !filters.epics.includes(ticket.epic_id)) {
        return false
      }
    }

    // Filter by sprints
    if (filters.sprints.length > 0) {
      if (!ticket.sprint_id || !filters.sprints.includes(ticket.sprint_id)) {
        return false
      }
    }

    return true
  })
}, [tickets, filters])
```

**Step 3: URL State Management** (2 hours)
Sync filters with URL params for shareable links:

```tsx
import { useSearchParams, useRouter, usePathname } from 'next/navigation'

// Load filters from URL on mount
useEffect(() => {
  const params = new URLSearchParams(searchParams)
  setFilters({
    assignees: params.getAll('assignee'),
    labels: params.getAll('label'),
    epics: params.getAll('epic'),
    statuses: params.getAll('status'),
    sprints: params.getAll('sprint'),
  })
}, [searchParams])

// Update URL when filters change
useEffect(() => {
  const params = new URLSearchParams()

  filters.assignees.forEach(a => params.append('assignee', a))
  filters.labels.forEach(l => params.append('label', l))
  filters.epics.forEach(e => params.append('epic', e))
  filters.statuses.forEach(s => params.append('status', s))
  filters.sprints.forEach(s => params.append('sprint', s))

  router.replace(`${pathname}?${params.toString()}`, { scroll: false })
}, [filters])
```

**Step 4: Save Custom Views** (2 hours)
File: `jility-web/components/board/saved-views.tsx`

Allow users to save filter combinations as named views:
- "My Bugs" = assignee:me + label:bug
- "Frontend Sprint 23" = label:frontend + sprint:23

Store in localStorage or user preferences table

#### Testing Checklist
- [ ] Filter by assignee
- [ ] Filter by label
- [ ] Filter by epic
- [ ] Filter by sprint
- [ ] Quick filter: My Tickets
- [ ] Quick filter: Unassigned
- [ ] Quick filter: Blocked
- [ ] Clear all filters
- [ ] URL updates with filters
- [ ] Shareable filter URL works
- [ ] Save custom view
- [ ] Load saved view

---

## Phase 5: AI/Agent Features (Weeks 6-7)

### 5.1 Enhanced MCP Server

**Status:** 🟡 Partial (MCP server exists, read-only?)
**Effort:** 3 days
**Priority:** High (enables other AI features)

#### Current State Investigation
Need to check:
- What MCP endpoints currently exist?
- Can agents create tickets?
- Can agents update tickets?
- What's the authentication model?

#### Goals
1. Add `create_ticket` to MCP
2. Add `update_ticket` to MCP
3. Add `create_tickets_batch` for epic breakdown
4. Add `add_comment` to MCP
5. Add `link_commit` to MCP

#### Implementation Steps

**Step 1: Audit Current MCP Server** (1 hour)
File: `crates/jility-mcp/src/main.rs` (or similar)

Document current capabilities

**Step 2: Add Create Ticket Tool** (2 hours)
```rust
#[derive(Debug, Deserialize)]
struct CreateTicketParams {
    project_id: String,
    title: String,
    description: Option<String>,
    assignees: Option<Vec<String>>,
    labels: Option<Vec<String>>,
    story_points: Option<i32>,
    parent_id: Option<String>,
    epic_id: Option<String>,
}

async fn create_ticket(params: CreateTicketParams) -> Result<Ticket> {
    // Call Jility API to create ticket
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/api/tickets", API_URL))
        .json(&params)
        .send()
        .await?;

    let ticket = response.json().await?;
    Ok(ticket)
}
```

**Step 3: Add Batch Create Tool** (1 hour)
For AI epic breakdown:
```rust
#[derive(Debug, Deserialize)]
struct CreateTicketsBatchParams {
    project_id: String,
    parent_id: Option<String>,
    tickets: Vec<TicketData>,
}

async fn create_tickets_batch(params: CreateTicketsBatchParams) -> Result<Vec<Ticket>> {
    // Create multiple tickets in one call
    // Link them to parent if specified
}
```

**Step 4: Add Update Ticket Tool** (1 hour)
```rust
async fn update_ticket(
    ticket_id: String,
    updates: UpdateTicketParams,
) -> Result<Ticket> {
    // Update ticket fields
}
```

**Step 5: Add Comment Tool** (30 min)
```rust
async fn add_comment(
    ticket_id: String,
    content: String,
) -> Result<Comment> {
    // Add comment to ticket
}
```

**Step 6: Test with Claude Desktop** (1 day)
Create test scenarios:
- "Create a ticket for implementing dark mode"
- "Break down the mobile app ticket into sub-tasks"
- "Add a comment to PROJ-123 saying we need design review"

#### Testing Checklist
- [ ] Create single ticket via MCP
- [ ] Create batch tickets via MCP
- [ ] Update ticket via MCP
- [ ] Add comment via MCP
- [ ] Link commit via MCP
- [ ] Proper error handling
- [ ] Authentication works
- [ ] Test in Claude Desktop

---

### 5.2 AI Epic Breakdown

**Status:** 🔴 New Build
**Effort:** 4-5 days
**Dependency:** 5.1 (MCP create_tickets_batch)
**Priority:** Medium (Signature feature)

#### User Flow
1. User creates epic ticket with description
2. Clicks "AI Breakdown" button
3. AI analyzes epic and suggests 5-10 sub-tasks
4. User reviews suggestions in preview UI
5. User can edit/remove suggestions
6. User clicks "Create Sub-tasks"
7. Sub-tasks created and linked to epic

#### Implementation Steps

**Step 1: Design AI Prompt** (1 hour)
File: `jility-web/lib/ai/epic-breakdown.ts`

```typescript
export function buildEpicBreakdownPrompt(epic: Ticket): string {
  return `You are a project manager breaking down an epic into manageable sub-tasks.

Epic Title: ${epic.title}

Epic Description:
${epic.description}

Please analyze this epic and break it down into 5-10 concrete, actionable sub-tasks. For each sub-task:
1. Write a clear, specific title (what needs to be done)
2. Provide a brief description (acceptance criteria)
3. Estimate story points (1, 2, 3, 5, 8, or 13)
4. Suggest relevant labels (backend, frontend, design, testing, etc.)

Return your response as a JSON array of sub-tasks:
[
  {
    "title": "...",
    "description": "...",
    "story_points": 3,
    "labels": ["backend", "database"]
  },
  ...
]

Focus on tasks that:
- Are independently deliverable
- Have clear completion criteria
- Together accomplish the full epic
- Are sized appropriately (not too big or too small)`
}
```

**Step 2: Create AI Service** (2 hours)
File: `jility-web/lib/ai/service.ts`

```typescript
interface SubTaskSuggestion {
  title: string
  description: string
  story_points?: number
  labels?: string[]
}

export async function getEpicBreakdown(
  epic: Ticket
): Promise<SubTaskSuggestion[]> {
  const prompt = buildEpicBreakdownPrompt(epic)

  // Call your LLM API (OpenAI, Anthropic, etc.)
  const response = await fetch('/api/ai/breakdown', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ prompt }),
  })

  if (!response.ok) {
    throw new Error('Failed to get AI breakdown')
  }

  const data = await response.json()
  return data.suggestions
}
```

**Step 3: Create Backend AI Endpoint** (1 hour)
File: `jility-server/src/api/ai.rs` (CREATE NEW)

```rust
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct BreakdownRequest {
    pub prompt: String,
}

#[derive(Debug, Serialize)]
pub struct BreakdownResponse {
    pub suggestions: Vec<SubTaskSuggestion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubTaskSuggestion {
    pub title: String,
    pub description: String,
    pub story_points: Option<i32>,
    pub labels: Option<Vec<String>>,
}

pub async fn get_breakdown(
    State(state): State<AppState>,
    Json(payload): Json<BreakdownRequest>,
) -> ApiResult<Json<BreakdownResponse>> {
    // Call OpenAI/Anthropic API
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", std::env::var("ANTHROPIC_API_KEY")?)
        .header("anthropic-version", "2023-06-01")
        .json(&serde_json::json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": 4096,
            "messages": [{
                "role": "user",
                "content": payload.prompt
            }]
        }))
        .send()
        .await?;

    let data: serde_json::Value = response.json().await?;
    let content = data["content"][0]["text"].as_str().unwrap();

    // Parse JSON response
    let suggestions: Vec<SubTaskSuggestion> = serde_json::from_str(content)?;

    Ok(Json(BreakdownResponse { suggestions }))
}
```

**Step 4: Create Breakdown Dialog** (4 hours)
File: `jility-web/components/ticket/epic-breakdown-dialog.tsx`

```tsx
'use client'

import { useState } from 'react'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { Badge } from '@/components/ui/badge'
import { Loader2, Sparkles, X, Edit2, Trash2 } from 'lucide-react'
import { getEpicBreakdown } from '@/lib/ai/service'
import type { Ticket } from '@/lib/types'

interface EpicBreakdownDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  epic: Ticket
  onCreateSubTasks: (subTasks: SubTaskData[]) => Promise<void>
}

export function EpicBreakdownDialog({
  open,
  onOpenChange,
  epic,
  onCreateSubTasks,
}: EpicBreakdownDialogProps) {
  const [loading, setLoading] = useState(false)
  const [suggestions, setSuggestions] = useState<SubTaskSuggestion[]>([])
  const [editing, setEditing] = useState<number | null>(null)

  const handleGetSuggestions = async () => {
    setLoading(true)
    try {
      const data = await getEpicBreakdown(epic)
      setSuggestions(data)
    } catch (error) {
      console.error('Failed to get AI breakdown:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleCreate = async () => {
    await onCreateSubTasks(suggestions)
    onOpenChange(false)
  }

  const handleRemove = (index: number) => {
    setSuggestions(prev => prev.filter((_, i) => i !== index))
  }

  const handleEdit = (index: number, field: string, value: any) => {
    setSuggestions(prev => prev.map((s, i) =>
      i === index ? { ...s, [field]: value } : s
    ))
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-4xl max-h-[80vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <Sparkles className="h-5 w-5 text-primary" />
            AI Epic Breakdown
          </DialogTitle>
        </DialogHeader>

        <div className="space-y-4">
          {/* Epic Info */}
          <div className="bg-muted/50 p-4 rounded-lg">
            <h3 className="font-semibold mb-1">{epic.title}</h3>
            <p className="text-sm text-muted-foreground line-clamp-3">
              {epic.description}
            </p>
          </div>

          {/* Generate Button */}
          {suggestions.length === 0 && (
            <div className="text-center py-8">
              <Button
                onClick={handleGetSuggestions}
                disabled={loading}
                size="lg"
              >
                {loading ? (
                  <>
                    <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                    Analyzing Epic...
                  </>
                ) : (
                  <>
                    <Sparkles className="h-4 w-4 mr-2" />
                    Generate Sub-Tasks
                  </>
                )}
              </Button>
              <p className="text-sm text-muted-foreground mt-2">
                AI will analyze your epic and suggest 5-10 sub-tasks
              </p>
            </div>
          )}

          {/* Suggestions */}
          {suggestions.length > 0 && (
            <>
              <div className="flex items-center justify-between">
                <h3 className="font-semibold">
                  Suggested Sub-Tasks ({suggestions.length})
                </h3>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={handleGetSuggestions}
                  disabled={loading}
                >
                  Regenerate
                </Button>
              </div>

              <div className="space-y-3">
                {suggestions.map((suggestion, index) => (
                  <div
                    key={index}
                    className="border rounded-lg p-4 space-y-3"
                  >
                    <div className="flex items-start justify-between gap-2">
                      {editing === index ? (
                        <Input
                          value={suggestion.title}
                          onChange={(e) => handleEdit(index, 'title', e.target.value)}
                          className="flex-1"
                        />
                      ) : (
                        <h4 className="font-medium flex-1">{suggestion.title}</h4>
                      )}
                      <div className="flex gap-1">
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => setEditing(editing === index ? null : index)}
                        >
                          <Edit2 className="h-3 w-3" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => handleRemove(index)}
                        >
                          <Trash2 className="h-3 w-3" />
                        </Button>
                      </div>
                    </div>

                    {editing === index ? (
                      <Textarea
                        value={suggestion.description}
                        onChange={(e) => handleEdit(index, 'description', e.target.value)}
                        rows={3}
                      />
                    ) : (
                      <p className="text-sm text-muted-foreground">
                        {suggestion.description}
                      </p>
                    )}

                    <div className="flex items-center gap-3">
                      {suggestion.story_points && (
                        <Badge variant="secondary">
                          {suggestion.story_points} points
                        </Badge>
                      )}
                      {suggestion.labels?.map(label => (
                        <Badge key={label} variant="outline">
                          {label}
                        </Badge>
                      ))}
                    </div>
                  </div>
                ))}
              </div>

              <div className="flex justify-end gap-2 pt-4 border-t">
                <Button variant="outline" onClick={() => onOpenChange(false)}>
                  Cancel
                </Button>
                <Button onClick={handleCreate}>
                  Create {suggestions.length} Sub-Tasks
                </Button>
              </div>
            </>
          )}
        </div>
      </DialogContent>
    </Dialog>
  )
}
```

**Step 5: Add Button to Ticket Detail** (30 min)
File: `jility-web/app/w/[slug]/ticket/[id]/page.tsx`

```tsx
const [breakdownOpen, setBreakdownOpen] = useState(false)

const handleCreateSubTasks = async (subTasks: SubTaskData[]) => {
  try {
    // Use MCP batch create or loop through API calls
    for (const subTask of subTasks) {
      await api.createTicket({
        ...subTask,
        project_id: ticket.project_id,
        parent_id: ticket.id,
      })
    }
    await loadTicket()
  } catch (error) {
    console.error('Failed to create sub-tasks:', error)
  }
}

// In render
{ticket.epic_id && (
  <Button onClick={() => setBreakdownOpen(true)}>
    <Sparkles className="h-4 w-4 mr-2" />
    AI Breakdown
  </Button>
)}

<EpicBreakdownDialog
  open={breakdownOpen}
  onOpenChange={setBreakdownOpen}
  epic={ticket}
  onCreateSubTasks={handleCreateSubTasks}
/>
```

#### Testing Checklist
- [ ] AI analyzes epic correctly
- [ ] Suggestions are reasonable
- [ ] Edit suggested title/description
- [ ] Remove suggestions
- [ ] Adjust story points
- [ ] Create all sub-tasks
- [ ] Sub-tasks linked to parent
- [ ] Error handling (AI API down)
- [ ] Cost tracking (API usage)

---

### 5.3 Smart Git Integration

**Status:** 🔴 New Build
**Effort:** 4-5 days
**Priority:** Medium

#### Features
1. **Auto-link commits** - Parse commit messages for ticket IDs
2. **Webhook receiver** - GitHub/GitLab webhooks
3. **PR linking** - Show linked PRs on ticket
4. **Auto-status** - Move ticket to "done" when PR merged

#### Implementation Steps

**Step 1: Create Webhook Endpoint** (Backend - 2 hours)
File: `jility-server/src/api/webhooks.rs` (CREATE NEW)

```rust
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Debug, Deserialize)]
pub struct GitHubWebhook {
    pub action: String,
    pub repository: Repository,
    pub pull_request: Option<PullRequest>,
    pub commits: Option<Vec<Commit>>,
}

#[derive(Debug, Deserialize)]
pub struct PullRequest {
    pub number: i32,
    pub title: String,
    pub state: String,
    pub merged: bool,
    pub html_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub message: String,
}

pub async fn github_webhook(
    State(state): State<AppState>,
    Json(payload): Json<GitHubWebhook>,
) -> ApiResult<Json<serde_json::Value>> {
    match payload.action.as_str() {
        "opened" | "synchronize" => {
            // PR opened or updated
            if let Some(pr) = payload.pull_request {
                handle_pr_update(&state, &pr).await?;
            }
        }
        "closed" => {
            // PR closed/merged
            if let Some(pr) = payload.pull_request {
                if pr.merged {
                    handle_pr_merged(&state, &pr).await?;
                }
            }
        }
        "push" => {
            // New commits
            if let Some(commits) = payload.commits {
                for commit in commits {
                    handle_commit(&state, &commit).await?;
                }
            }
        }
        _ => {}
    }

    Ok(Json(serde_json::json!({ "status": "ok" })))
}

async fn handle_commit(
    state: &AppState,
    commit: &Commit,
) -> Result<()> {
    // Parse commit message for ticket IDs
    let ticket_pattern = Regex::new(r"([A-Z]+-\d+)").unwrap();

    for capture in ticket_pattern.captures_iter(&commit.message) {
        let ticket_number = &capture[1];

        // Find ticket by number
        if let Some(ticket) = find_ticket_by_number(state.db.as_ref(), ticket_number).await? {
            // Link commit to ticket
            link_commit_to_ticket(
                state.db.as_ref(),
                ticket.id,
                &commit.sha,
                &commit.message,
            ).await?;
        }
    }

    Ok(())
}

async fn handle_pr_merged(
    state: &AppState,
    pr: &PullRequest,
) -> Result<()> {
    // Parse PR title/description for ticket IDs
    let ticket_pattern = Regex::new(r"([A-Z]+-\d+)").unwrap();

    for capture in ticket_pattern.captures_iter(&pr.title) {
        let ticket_number = &capture[1];

        if let Some(ticket) = find_ticket_by_number(state.db.as_ref(), ticket_number).await? {
            // Move ticket to done
            update_ticket_status(
                state.db.as_ref(),
                ticket.id,
                "done",
                "github-bot",
            ).await?;
        }
    }

    Ok(())
}
```

**Step 2: Add Webhook Route** (5 min)
```rust
.route("/api/webhooks/github", post(webhooks::github_webhook))
```

**Step 3: Create Webhook UI** (Frontend - 1 hour)
File: `jility-web/app/w/[slug]/project/[projectId]/settings/page.tsx`

Show webhook URL and setup instructions:
```
Webhook URL: https://your-jility.com/api/webhooks/github
Events: Push, Pull Request
```

**Step 4: Display Linked PRs** (Frontend - 2 hours)
File: `jility-web/components/ticket/linked-prs.tsx`

```tsx
export function LinkedPRs({ prs }: { prs: PullRequest[] }) {
  return (
    <div className="space-y-2">
      <h3 className="text-sm font-semibold">Linked Pull Requests</h3>
      <div className="space-y-2">
        {prs.map(pr => (
          <a
            key={pr.id}
            href={pr.url}
            target="_blank"
            rel="noopener noreferrer"
            className="block p-3 border rounded-lg hover:bg-muted/50 transition"
          >
            <div className="flex items-center justify-between">
              <span className="font-medium">#{pr.number}: {pr.title}</span>
              <Badge variant={pr.merged ? 'default' : pr.state === 'open' ? 'secondary' : 'outline'}>
                {pr.merged ? 'Merged' : pr.state}
              </Badge>
            </div>
          </a>
        ))}
      </div>
    </div>
  )
}
```

**Step 5: Add to Ticket Detail** (30 min)
Show linked commits and PRs on ticket detail page

**Step 6: Testing** (1 day)
- Set up test GitHub repo
- Configure webhook
- Test commit linking
- Test PR linking
- Test auto-status on merge

#### Testing Checklist
- [ ] Webhook receives GitHub events
- [ ] Commit message parsed for ticket ID
- [ ] Commit linked to ticket
- [ ] PR linked to ticket
- [ ] Ticket status updates on PR merge
- [ ] Works with multiple ticket IDs
- [ ] GitLab webhook support
- [ ] Webhook security (verify signature)

---

## Implementation Timeline

| Week | Phase | Features | Estimated Hours |
|------|-------|----------|----------------|
| 1-2 | Core Collaboration | Comments + Sprint Planning | 60-70 |
| 3-4 | Visual Workflows | Swimlanes + Burndown | 50-60 |
| 5 | Search & Discovery | Search + Filters | 40-50 |
| 6-7 | AI/Agent | MCP + AI Breakdown + Git | 70-80 |

**Total:** ~220-260 hours (~7 weeks at 30-40 hrs/week)

---

## Success Metrics

**Phase 1 (Collaboration)**
- 90%+ of tickets have comments
- Sprints created for next 3 iterations
- Average 5+ comments per ticket

**Phase 2 (Visual)**
- 60%+ users use swimlanes feature
- Burndown charts viewed daily by managers
- Sprint velocity stabilizes after 3 sprints

**Phase 3 (Search)**
- Average search time < 5 seconds
- 70%+ of searches find target ticket
- Custom filters saved by power users

**Phase 4 (AI)**
- 40%+ of epics use AI breakdown
- AI suggestions accepted 80%+ of time
- Git auto-linking catches 90%+ of commits

---

## Notes for Implementation

**General Guidelines:**
1. **Mobile-first**: Design/test on mobile before desktop
2. **Accessibility**: Keyboard navigation, screen readers
3. **Performance**: Lazy load, virtualize lists, debounce searches
4. **Error handling**: Graceful failures, user-friendly messages
5. **Loading states**: Skeleton screens, optimistic updates
6. **Testing**: Unit tests for utils, integration tests for flows

**Backend Standards:**
- Use transactions for multi-step operations
- Record all changes in `ticket_changes` table
- Broadcast WebSocket events for real-time updates
- Return proper HTTP status codes
- Validate all inputs

**Frontend Standards:**
- Use TypeScript strictly (no `any`)
- Follow existing component patterns
- Reuse UI components from shadcn/ui
- Keep components under 300 lines
- Extract logic to custom hooks

**Security:**
- Authenticate all API calls
- Validate webhook signatures
- Rate limit AI API calls
- Sanitize user inputs
- HTTPS only for webhooks

---

## Next Steps

1. **Review & Prioritize**: Go through this plan and adjust priorities based on user feedback
2. **Set Up Environment**: Ensure dev environment has all dependencies
3. **Create Tickets**: Break each phase into individual tickets in Jility (meta!)
4. **Start Phase 1**: Begin with comments system (quick win)
5. **Iterate**: Get user feedback after each phase

---

**Document Version:** 1.0
**Last Updated:** 2025-11-07
**Status:** Ready for Implementation
