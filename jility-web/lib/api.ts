import type {
  Ticket,
  TicketDetails,
  CreateTicketRequest,
  UpdateTicketRequest,
  Comment,
  Project,
  CreateProjectRequest,
  UpdateProjectRequest,
  TicketFilters,
  LinkedCommit,
  TicketChange,
  SearchFilters,
  SearchResponse,
  SavedView,
  CreateSavedViewRequest,
  UpdateSavedViewRequest,
  WorkspaceMember,
  InviteMemberRequest,
  InviteResponse,
  PendingInvite,
  InviteDetails,
  WorkspaceResponse,
  Sprint,
  SprintDetails,
  SprintStats,
  BurndownData,
  SprintHistory,
} from './types'

const API_BASE = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3900/api'

function getAuthHeaders(): Record<string, string> {
  if (typeof window === 'undefined') return {}
  const token = localStorage.getItem('jility_token')
  return token ? { Authorization: `Bearer ${token}` } : {}
}

async function handleResponse<T>(response: Response): Promise<T> {
  if (!response.ok) {
    // Handle 401 Unauthorized - redirect to login
    if (response.status === 401) {
      // Clear invalid token
      if (typeof window !== 'undefined') {
        localStorage.removeItem('jility_token')
        // Redirect to login page
        window.location.href = '/login'
      }
    }

    const error = await response.json().catch(() => ({ message: 'Unknown error' }))
    throw new Error(error.message || `HTTP ${response.status}`)
  }
  return response.json()
}

export const api = {
  // Projects
  listProjects: async (): Promise<Project[]> => {
    const res = await fetch(`${API_BASE}/projects`)
    return handleResponse<Project[]>(res)
  },

  createProject: async (data: CreateProjectRequest): Promise<Project> => {
    const res = await fetch(`${API_BASE}/projects`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify(data),
    })
    return handleResponse<Project>(res)
  },

  getProject: async (id: string): Promise<Project> => {
    const res = await fetch(`${API_BASE}/projects/${id}`)
    return handleResponse<Project>(res)
  },

  updateProject: async (id: string, data: UpdateProjectRequest): Promise<Project> => {
    const res = await fetch(`${API_BASE}/projects/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify(data),
    })
    return handleResponse<Project>(res)
  },

  deleteProject: async (id: string): Promise<{ success: boolean }> => {
    const res = await fetch(`${API_BASE}/projects/${id}`, {
      method: 'DELETE',
      headers: getAuthHeaders(),
    })
    return handleResponse<{ success: boolean }>(res)
  },

  // Tickets
  listTickets: async (filters?: TicketFilters): Promise<Ticket[]> => {
    const params = new URLSearchParams()
    if (filters?.project_id) params.append('project_id', filters.project_id)
    if (filters?.status) params.append('status', filters.status)
    if (filters?.assignee) params.append('assignee', filters.assignee)

    const res = await fetch(`${API_BASE}/tickets?${params}`)
    return handleResponse<Ticket[]>(res)
  },

  getTicket: async (id: string): Promise<TicketDetails> => {
    const res = await fetch(`${API_BASE}/tickets/${id}`)
    return handleResponse<TicketDetails>(res)
  },

  createTicket: async (ticket: CreateTicketRequest): Promise<Ticket> => {
    const res = await fetch(`${API_BASE}/tickets`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify(ticket),
    })
    return handleResponse<Ticket>(res)
  },

  updateTicket: async (id: string, data: UpdateTicketRequest): Promise<Ticket> => {
    const res = await fetch(`${API_BASE}/tickets/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify(data),
    })
    return handleResponse<Ticket>(res)
  },

  updateTicketStatus: async (id: string, status: string): Promise<Ticket> => {
    const headers = {
      'Content-Type': 'application/json',
      ...getAuthHeaders(),
    }
    console.log('[API] updateTicketStatus - Headers:', headers)
    console.log('[API] updateTicketStatus - Payload:', { id, status })

    const res = await fetch(`${API_BASE}/tickets/${id}/status`, {
      method: 'PATCH',
      headers,
      body: JSON.stringify({ status }),
    })

    console.log('[API] updateTicketStatus - Response status:', res.status)
    return handleResponse<Ticket>(res)
  },

  updateDescription: async (id: string, description: string): Promise<Ticket> => {
    const res = await fetch(`${API_BASE}/tickets/${id}/description`, {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify({ description, operation: 'replace_all' }),
    })
    return handleResponse<Ticket>(res)
  },

  assignTicket: async (id: string, assignee: string): Promise<Ticket> => {
    const res = await fetch(`${API_BASE}/tickets/${id}/assign`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify({ assignee }),
    })
    return handleResponse<Ticket>(res)
  },

  unassignTicket: async (id: string, assignee: string): Promise<Ticket> => {
    const res = await fetch(`${API_BASE}/tickets/${id}/unassign`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify({ assignee }),
    })
    return handleResponse<Ticket>(res)
  },

  deleteTicket: async (id: string): Promise<{ success: boolean }> => {
    const res = await fetch(`${API_BASE}/tickets/${id}`, {
      method: 'DELETE',
    })
    return handleResponse<{ success: boolean }>(res)
  },

  // Comments
  listComments: async (ticketId: string): Promise<Comment[]> => {
    const res = await fetch(`${API_BASE}/tickets/${ticketId}/comments`)
    return handleResponse<Comment[]>(res)
  },

  createComment: async (ticketId: string, content: string): Promise<Comment> => {
    const res = await fetch(`${API_BASE}/tickets/${ticketId}/comments`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify({ content }),
    })
    return handleResponse<Comment>(res)
  },

  updateComment: async (id: string, content: string): Promise<Comment> => {
    const res = await fetch(`${API_BASE}/comments/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify({ content }),
    })
    return handleResponse<Comment>(res)
  },

  deleteComment: async (id: string): Promise<{ success: boolean }> => {
    const res = await fetch(`${API_BASE}/comments/${id}`, {
      method: 'DELETE',
      headers: getAuthHeaders(),
    })
    return handleResponse<{ success: boolean }>(res)
  },

  // Dependencies
  addDependency: async (ticketId: string, dependsOnId: string): Promise<{ success: boolean }> => {
    const res = await fetch(`${API_BASE}/tickets/${ticketId}/dependencies`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify({ depends_on_id: dependsOnId }),
    })
    return handleResponse<{ success: boolean }>(res)
  },

  removeDependency: async (ticketId: string, dependencyId: string): Promise<{ success: boolean }> => {
    const res = await fetch(`${API_BASE}/tickets/${ticketId}/dependencies/${dependencyId}`, {
      method: 'DELETE',
    })
    return handleResponse<{ success: boolean }>(res)
  },

  // Activity
  getActivity: async (ticketId: string): Promise<TicketChange[]> => {
    const res = await fetch(`${API_BASE}/tickets/${ticketId}/activity`)
    return handleResponse<TicketChange[]>(res)
  },

  // Git
  linkCommit: async (
    ticketId: string,
    commitHash: string,
    commitMessage: string
  ): Promise<LinkedCommit> => {
    const res = await fetch(`${API_BASE}/tickets/${ticketId}/commits`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify({ commit_hash: commitHash, commit_message: commitMessage }),
    })
    return handleResponse<LinkedCommit>(res)
  },

  listCommits: async (ticketId: string): Promise<LinkedCommit[]> => {
    const res = await fetch(`${API_BASE}/tickets/${ticketId}/commits`)
    return handleResponse<LinkedCommit[]>(res)
  },

  // Search
  searchTickets: async (query: string, limit?: number): Promise<Ticket[]> => {
    const params = new URLSearchParams({ q: query })
    if (limit) params.append('limit', limit.toString())

    const res = await fetch(`${API_BASE}/search?${params}`)
    return handleResponse<Ticket[]>(res)
  },

  // Advanced search
  advancedSearch: async (filters: SearchFilters): Promise<SearchResponse> => {
    const params = new URLSearchParams({ q: filters.q })

    if (filters.status) filters.status.forEach(s => params.append('status', s))
    if (filters.assignees) filters.assignees.forEach(a => params.append('assignees', a))
    if (filters.labels) filters.labels.forEach(l => params.append('labels', l))
    if (filters.created_by) params.append('created_by', filters.created_by)
    if (filters.created_after) params.append('created_after', filters.created_after)
    if (filters.created_before) params.append('created_before', filters.created_before)
    if (filters.updated_after) params.append('updated_after', filters.updated_after)
    if (filters.updated_before) params.append('updated_before', filters.updated_before)
    if (filters.min_points) params.append('min_points', filters.min_points.toString())
    if (filters.max_points) params.append('max_points', filters.max_points.toString())
    if (filters.has_comments !== undefined) params.append('has_comments', filters.has_comments.toString())
    if (filters.has_commits !== undefined) params.append('has_commits', filters.has_commits.toString())
    if (filters.has_dependencies !== undefined) params.append('has_dependencies', filters.has_dependencies.toString())
    if (filters.epic_id) params.append('epic_id', filters.epic_id)
    if (filters.parent_id) params.append('parent_id', filters.parent_id)
    if (filters.project_id) params.append('project_id', filters.project_id)
    if (filters.search_in) filters.search_in.forEach(s => params.append('search_in', s))
    if (filters.limit) params.append('limit', filters.limit.toString())
    if (filters.offset) params.append('offset', filters.offset.toString())

    const res = await fetch(`${API_BASE}/search?${params}`)
    return handleResponse<SearchResponse>(res)
  },

  // Saved views
  listSavedViews: async (): Promise<SavedView[]> => {
    const res = await fetch(`${API_BASE}/search/views`)
    return handleResponse<SavedView[]>(res)
  },

  getSavedView: async (id: string): Promise<SavedView> => {
    const res = await fetch(`${API_BASE}/search/views/${id}`)
    return handleResponse<SavedView>(res)
  },

  createSavedView: async (data: CreateSavedViewRequest): Promise<SavedView> => {
    const res = await fetch(`${API_BASE}/search/views`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify(data),
    })
    return handleResponse<SavedView>(res)
  },

  updateSavedView: async (id: string, data: UpdateSavedViewRequest): Promise<SavedView> => {
    const res = await fetch(`${API_BASE}/search/views/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify(data),
    })
    return handleResponse<SavedView>(res)
  },

  deleteSavedView: async (id: string): Promise<{ message: string }> => {
    const res = await fetch(`${API_BASE}/search/views/${id}`, {
      method: 'DELETE',
      headers: getAuthHeaders(),
    })
    return handleResponse<{ message: string }>(res)
  },

  // Auth & Profile
  getCurrentUser: async (): Promise<any> => {
    const res = await fetch(`${API_BASE}/auth/me`, {
      headers: getAuthHeaders(),
    })
    return handleResponse<any>(res)
  },

  createApiKey: async (data: { name: string; scopes: string[]; expires_in_days?: number }): Promise<any> => {
    const res = await fetch(`${API_BASE}/auth/api-keys`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify(data),
    })
    return handleResponse<any>(res)
  },

  listApiKeys: async (): Promise<any[]> => {
    const res = await fetch(`${API_BASE}/auth/api-keys`, {
      headers: getAuthHeaders(),
    })
    return handleResponse<any[]>(res)
  },

  revokeApiKey: async (id: string): Promise<{ success: boolean }> => {
    const res = await fetch(`${API_BASE}/auth/api-keys/${id}`, {
      method: 'DELETE',
      headers: getAuthHeaders(),
    })
    return handleResponse<{ success: boolean }>(res)
  },

  listSessions: async (): Promise<any[]> => {
    const res = await fetch(`${API_BASE}/auth/sessions`, {
      headers: getAuthHeaders(),
    })
    return handleResponse<any[]>(res)
  },

  // Workspace member management
  listWorkspaceMembers: async (workspaceSlug: string): Promise<WorkspaceMember[]> => {
    const response = await fetch(`${API_BASE}/workspaces/${workspaceSlug}/members`, {
      headers: {
        Authorization: `Bearer ${localStorage.getItem('jility_token')}`,
      },
    })
    if (!response.ok) {
      throw new Error('Failed to fetch workspace members')
    }
    return response.json()
  },

  inviteWorkspaceMember: async (
    workspaceSlug: string,
    data: InviteMemberRequest
  ): Promise<InviteResponse> => {
    const response = await fetch(`${API_BASE}/workspaces/${workspaceSlug}/invite`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${localStorage.getItem('jility_token')}`,
      },
      body: JSON.stringify(data),
    })
    if (!response.ok) {
      const error = await response.json()
      throw new Error(error.message || 'Failed to invite member')
    }
    return response.json()
  },

  removeWorkspaceMember: async (
    workspaceSlug: string,
    userId: string
  ): Promise<void> => {
    const response = await fetch(
      `${API_BASE}/workspaces/${workspaceSlug}/members/${userId}`,
      {
        method: 'DELETE',
        headers: {
          Authorization: `Bearer ${localStorage.getItem('jility_token')}`,
        },
      }
    )
    if (!response.ok) {
      throw new Error('Failed to remove member')
    }
  },

  listPendingInvites: async (workspaceSlug: string): Promise<PendingInvite[]> => {
    const response = await fetch(`${API_BASE}/workspaces/${workspaceSlug}/invites`, {
      headers: {
        Authorization: `Bearer ${localStorage.getItem('jility_token')}`,
      },
    })
    if (!response.ok) {
      throw new Error('Failed to fetch pending invites')
    }
    return response.json()
  },

  getInviteDetails: async (token: string): Promise<InviteDetails> => {
    const response = await fetch(`${API_BASE}/invites/${token}`)
    if (!response.ok) {
      throw new Error('Failed to fetch invite details')
    }
    return response.json()
  },

  acceptInvite: async (token: string): Promise<WorkspaceResponse> => {
    const response = await fetch(`${API_BASE}/invites/${token}/accept`, {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${localStorage.getItem('jility_token')}`,
      },
    })
    if (!response.ok) {
      const error = await response.json()
      throw new Error(error.message || 'Failed to accept invite')
    }
    return response.json()
  },

  // Helper to get project ID from workspace
  // TODO: Make this more efficient - could cache or get from workspace context
  getProjectByWorkspace: async (workspaceSlug: string): Promise<Project> => {
    const projects = await api.listProjects()
    // For now, assume one project per workspace and get the first one
    // In the future, we should filter by workspace or store workspace-project mapping
    if (projects.length === 0) {
      throw new Error('No project found for workspace')
    }
    return projects[0]
  },

  // Sprint Management
  listSprints: async (workspaceSlug: string, status?: string): Promise<Sprint[]> => {
    const project = await api.getProjectByWorkspace(workspaceSlug)
    const url = status
      ? `${API_BASE}/projects/${project.id}/sprints?status=${status}`
      : `${API_BASE}/projects/${project.id}/sprints`
    const res = await fetch(url, { headers: getAuthHeaders() })
    return handleResponse<Sprint[]>(res)
  },

  createSprint: async (
    workspaceSlug: string,
    data: {
      name: string
      goal?: string
      start_date?: string
      end_date?: string
    }
  ): Promise<Sprint> => {
    const project = await api.getProjectByWorkspace(workspaceSlug)
    const res = await fetch(`${API_BASE}/projects/${project.id}/sprints`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify(data),
    })
    return handleResponse<Sprint>(res)
  },

  getSprint: async (sprintId: string): Promise<SprintDetails> => {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}`, {
      headers: getAuthHeaders(),
    })
    return handleResponse<SprintDetails>(res)
  },

  updateSprint: async (
    sprintId: string,
    data: {
      name?: string
      goal?: string
      start_date?: string
      end_date?: string
    }
  ): Promise<Sprint> => {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify(data),
    })
    return handleResponse<Sprint>(res)
  },

  deleteSprint: async (sprintId: string): Promise<void> => {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}`, {
      method: 'DELETE',
      headers: getAuthHeaders(),
    })
    await handleResponse<void>(res)
  },

  startSprint: async (
    sprintId: string,
    data: {
      start_date: string
      end_date: string
    }
  ): Promise<Sprint> => {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}/start`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify(data),
    })
    return handleResponse<Sprint>(res)
  },

  completeSprint: async (sprintId: string): Promise<Sprint> => {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}/complete`, {
      method: 'POST',
      headers: getAuthHeaders(),
    })
    return handleResponse<Sprint>(res)
  },

  addTicketToSprint: async (sprintId: string, ticketId: string, addedBy: string): Promise<void> => {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}/tickets/${ticketId}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...getAuthHeaders(),
      },
      body: JSON.stringify({ added_by: addedBy }),
    })
    await handleResponse<void>(res)
  },

  removeTicketFromSprint: async (sprintId: string, ticketId: string): Promise<void> => {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}/tickets/${ticketId}`, {
      method: 'DELETE',
      headers: getAuthHeaders(),
    })
    await handleResponse<void>(res)
  },

  getSprintStats: async (sprintId: string): Promise<SprintStats> => {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}/stats`, {
      headers: getAuthHeaders(),
    })
    return handleResponse<SprintStats>(res)
  },

  getBurndownData: async (sprintId: string): Promise<BurndownData> => {
    const res = await fetch(`${API_BASE}/sprints/${sprintId}/burndown`, {
      headers: getAuthHeaders(),
    })
    return handleResponse<BurndownData>(res)
  },

  getSprintHistory: async (workspaceSlug: string): Promise<SprintHistory> => {
    const project = await api.getProjectByWorkspace(workspaceSlug)
    const res = await fetch(`${API_BASE}/projects/${project.id}/sprint-history`, {
      headers: getAuthHeaders(),
    })
    return handleResponse<SprintHistory>(res)
  },
}
