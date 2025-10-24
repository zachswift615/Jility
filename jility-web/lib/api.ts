import type {
  Ticket,
  TicketDetails,
  CreateTicketRequest,
  UpdateTicketRequest,
  Comment,
  Project,
  TicketFilters,
  LinkedCommit,
  TicketChange,
} from './types'

const API_BASE = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000/api'

async function handleResponse<T>(response: Response): Promise<T> {
  if (!response.ok) {
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

  createProject: async (data: { name: string; description?: string }): Promise<Project> => {
    const res = await fetch(`${API_BASE}/projects`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    })
    return handleResponse<Project>(res)
  },

  getProject: async (id: string): Promise<Project> => {
    const res = await fetch(`${API_BASE}/projects/${id}`)
    return handleResponse<Project>(res)
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
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(ticket),
    })
    return handleResponse<Ticket>(res)
  },

  updateTicket: async (id: string, data: UpdateTicketRequest): Promise<Ticket> => {
    const res = await fetch(`${API_BASE}/tickets/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    })
    return handleResponse<Ticket>(res)
  },

  updateTicketStatus: async (id: string, status: string): Promise<Ticket> => {
    const res = await fetch(`${API_BASE}/tickets/${id}/status`, {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ status }),
    })
    return handleResponse<Ticket>(res)
  },

  updateDescription: async (id: string, description: string): Promise<Ticket> => {
    const res = await fetch(`${API_BASE}/tickets/${id}/description`, {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ description, operation: 'replace_all' }),
    })
    return handleResponse<Ticket>(res)
  },

  assignTicket: async (id: string, assignee: string): Promise<Ticket> => {
    const res = await fetch(`${API_BASE}/tickets/${id}/assign`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ assignee }),
    })
    return handleResponse<Ticket>(res)
  },

  unassignTicket: async (id: string, assignee: string): Promise<Ticket> => {
    const res = await fetch(`${API_BASE}/tickets/${id}/unassign`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
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
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ content }),
    })
    return handleResponse<Comment>(res)
  },

  updateComment: async (id: string, content: string): Promise<Comment> => {
    const res = await fetch(`${API_BASE}/comments/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ content }),
    })
    return handleResponse<Comment>(res)
  },

  deleteComment: async (id: string): Promise<{ success: boolean }> => {
    const res = await fetch(`${API_BASE}/comments/${id}`, {
      method: 'DELETE',
    })
    return handleResponse<{ success: boolean }>(res)
  },

  // Dependencies
  addDependency: async (ticketId: string, dependsOnId: string): Promise<{ success: boolean }> => {
    const res = await fetch(`${API_BASE}/tickets/${ticketId}/dependencies`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
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
      headers: { 'Content-Type': 'application/json' },
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
}
