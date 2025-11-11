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
  project_id: string
}

export interface TicketReference {
  id: string
  number: string
  title: string
  status: string
}

export interface TicketDetails {
  ticket: Ticket
  comments: Comment[]
  dependencies: TicketReference[]
  dependents: TicketReference[]
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
  user_name: string
  changed_at: string
  message?: string
}

export interface Project {
  id: string
  name: string
  description?: string
  key?: string
  color?: string
  ai_planning_enabled: boolean
  auto_link_git: boolean
  require_story_points: boolean
  created_at: string
  updated_at: string
}

export interface CreateProjectRequest {
  workspace_id: string
  name: string
  description?: string
  key?: string
  color?: string
  ai_planning_enabled?: boolean
  auto_link_git?: boolean
  require_story_points?: boolean
}

export interface UpdateProjectRequest {
  name?: string
  description?: string
  key?: string
  color?: string
  ai_planning_enabled?: boolean
  auto_link_git?: boolean
  require_story_points?: boolean
}

export interface CreateTicketRequest {
  project_id: string
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

// Search types
export interface SearchFilters {
  q: string
  status?: string[]
  assignees?: string[]
  labels?: string[]
  created_by?: string
  created_after?: string
  created_before?: string
  updated_after?: string
  updated_before?: string
  min_points?: number
  max_points?: number
  has_comments?: boolean
  has_commits?: boolean
  has_dependencies?: boolean
  epic_id?: string
  parent_id?: string
  project_id?: string
  search_in?: string[]
  limit?: number
  offset?: number
}

export interface SearchResult {
  ticket_id: string
  ticket_number: string
  title: string
  description: string
  status: string
  story_points?: number
  snippet: string
  rank: number
  matched_in: string[]
  assignees: string[]
  labels: string[]
  created_by: string
  created_at: string
  updated_at: string
  parent_id?: string
  epic_id?: string
}

export interface SearchResponse {
  results: SearchResult[]
  total: number
  has_more: boolean
  offset: number
  limit: number
}

export interface SavedView {
  id: string
  user_id: string
  name: string
  description?: string
  filters: SearchFilters
  is_default: boolean
  is_shared: boolean
  created_at: string
  updated_at: string
}

export interface CreateSavedViewRequest {
  name: string
  description?: string
  filters: SearchFilters
  is_default?: boolean
  is_shared?: boolean
}

export interface UpdateSavedViewRequest {
  name?: string
  description?: string
  filters?: SearchFilters
  is_default?: boolean
  is_shared?: boolean
}

export interface WorkspaceMember {
  user_id: string
  email: string
  role: 'admin' | 'member'
  joined_at: string
}

export interface InviteMemberRequest {
  email: string
  role: 'admin' | 'member'
}

export interface InviteResponse {
  invite_id: string
  email: string
  role: 'admin' | 'member'
  token: string
  invite_url: string
  expires_at: string
}

export interface PendingInvite {
  invite_id: string
  email: string
  role: 'admin' | 'member'
  token: string
  invite_url: string
  invited_at: string
  expires_at: string
}

export interface InviteDetails {
  workspace_name: string
  workspace_slug: string
  invited_by_email: string
  role: 'admin' | 'member'
  expires_at: string
  is_expired: boolean
}

export interface WorkspaceResponse {
  id: string
  name: string
  slug: string
  role: 'admin' | 'member'
  created_at: string
}

export interface Sprint {
  id: string
  project_id: string
  name: string
  goal?: string
  status: 'planning' | 'active' | 'completed'
  start_date?: string
  end_date?: string
  created_at: string
  updated_at: string
}

export interface SprintStats {
  total_tickets: number
  total_points: number
  completed_tickets: number
  completed_points: number
  in_progress_tickets: number
  in_progress_points: number
  todo_tickets: number
  todo_points: number
  completion_percentage: number
}

export interface SprintDetails {
  sprint: Sprint
  tickets: Ticket[]
  stats: SprintStats
}

export interface BurndownDataPoint {
  date: string
  ideal: number
  actual: number
}

export interface BurndownData {
  sprint_id: string
  data_points: BurndownDataPoint[]
}

export interface VelocityData {
  sprint_name: string
  completed_points: number
}

export interface SprintHistory {
  sprints: Sprint[]
  velocity_data: VelocityData[]
  average_velocity: number
}

export interface WorkspaceSettings {
  sprint_capacity?: number
}

export interface WorkspaceWithSettings extends WorkspaceResponse {
  settings: WorkspaceSettings
}

export interface EpicProgress {
  total: number
  done: number
  in_progress: number
  todo: number
  blocked: number
  completion_percentage: number
}

export interface Epic {
  id: string
  number: string
  title: string
  description: string
  is_epic: boolean
  epic_color?: string
  progress: EpicProgress
  created_at: string
  updated_at: string
}
