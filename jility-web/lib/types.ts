export type TicketStatus = 'backlog' | 'todo' | 'in_progress' | 'review' | 'done' | 'blocked'

export interface Ticket {
  id: string
  number: string
  title: string
  description: string
  status: TicketStatus
  story_points?: number
  assignees: string[]
  labels: string[]
  created_at: string
  updated_at: string
  created_by: string
  parent_id?: string
  epic_id?: string
}

export interface TicketDetails {
  ticket: Ticket
  comments: Comment[]
  dependencies: Ticket[]
  dependents: Ticket[]
  linked_commits: LinkedCommit[]
  recent_changes: TicketChange[]
}

export interface Comment {
  id: string
  ticket_id: string
  author: string
  content: string
  created_at: string
  updated_at?: string
}

export interface LinkedCommit {
  id: string
  commit_hash: string
  commit_message: string
  linked_at: string
  linked_by: string
}

export interface TicketChange {
  id: string
  change_type: string
  field_name?: string
  old_value?: string
  new_value?: string
  changed_by: string
  changed_at: string
  message?: string
}

export interface Project {
  id: string
  name: string
  description?: string
  created_at: string
  updated_at: string
}

export interface CreateTicketRequest {
  title: string
  description: string
  story_points?: number
  status?: TicketStatus
  assignees?: string[]
  labels?: string[]
  parent_id?: string
  epic_id?: string
}

export interface UpdateTicketRequest {
  title?: string
  story_points?: number
  parent_id?: string
  epic_id?: string
}

export interface WebSocketMessage {
  type: 'ticket_created' | 'ticket_updated' | 'status_changed' | 'comment_added' | 'description_edited'
  ticket?: Ticket
  ticket_id?: string
  old_status?: TicketStatus
  new_status?: TicketStatus
  comment?: Comment
  version?: number
}

export interface TicketFilters {
  project_id?: string
  status?: TicketStatus
  assignee?: string
}
