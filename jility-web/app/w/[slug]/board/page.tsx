'use client'

import { useState, useEffect, useCallback, Suspense } from 'react'
import { useSearchParams } from 'next/navigation'
import { KanbanBoard } from '@/components/kanban/board'
import { MobileFAB } from '@/components/layout/mobile-fab'
import { CreateTicketDialog } from '@/components/ticket/create-ticket-dialog'
import { AssigneeFilter } from '@/components/ticket/assignee-filter'
import { withAuth } from '@/lib/with-auth'
import { useAuth } from '@/lib/auth-context'
import { api } from '@/lib/api'
import { useWorkspace } from '@/lib/workspace-context'
import type { WorkspaceMember, Ticket, Sprint } from '@/lib/types'
import { SprintFilter } from '@/components/board/sprint-filter'

function BoardContent() {
  const [showCreateDialog, setShowCreateDialog] = useState(false)
  const [members, setMembers] = useState<WorkspaceMember[]>([])
  const [activeSprint, setActiveSprint] = useState<Sprint | null>(null)
  const [activeSprintTicketIds, setActiveSprintTicketIds] = useState<Set<string>>(new Set())
  const { user } = useAuth()
  const { currentWorkspace } = useWorkspace()
  const slug = currentWorkspace?.slug || ''
  const searchParams = useSearchParams()

  // Fetch active sprint and its tickets
  const loadActiveSprint = useCallback(async () => {
    console.log('[DEBUG] loadActiveSprint called, slug:', slug)
    if (!slug) {
      console.log('[DEBUG] No slug, returning early')
      return
    }
    try {
      console.log('[DEBUG] Fetching active sprints for slug:', slug)
      const sprints = await api.listSprints(slug, 'active')
      console.log('[DEBUG] Active sprints response:', sprints)
      if (sprints.length > 0) {
        const sprint = sprints[0]
        console.log('[DEBUG] Setting active sprint:', sprint)
        setActiveSprint(sprint)

        // Fetch sprint details to get ticket IDs
        console.log('[DEBUG] Fetching sprint details for ID:', sprint.id)
        const sprintDetails = await api.getSprint(sprint.id)
        console.log('[DEBUG] Sprint details:', sprintDetails)
        const ticketIds = new Set(sprintDetails.tickets.map(t => t.id))
        console.log('[DEBUG] Ticket IDs in sprint:', Array.from(ticketIds))
        setActiveSprintTicketIds(ticketIds)
      } else {
        console.log('[DEBUG] No active sprints found')
      }
    } catch (error) {
      console.error('[DEBUG] Failed to load active sprint:', error)
    }
  }, [slug])

  // Fetch workspace members on mount
  const loadMembers = useCallback(async () => {
    if (!slug) return
    try {
      const data = await api.listWorkspaceMembers(slug)
      setMembers(data)
    } catch (error) {
      console.error('Failed to load workspace members:', error)
    }
  }, [slug])

  useEffect(() => {
    loadMembers()
    loadActiveSprint()
  }, [loadMembers, loadActiveSprint])

  // Filter tickets by assignee
  const getFilteredTickets = (tickets: Ticket[]) => {
    const assigneeParam = searchParams.get('assignee')
    if (!assigneeParam) return tickets

    const filters = assigneeParam.split(',')

    return tickets.filter((ticket) => {
      // Check for "me" filter
      if (filters.includes('me') && user?.email) {
        if (ticket.assignees.includes(user.email)) return true
      }

      // Check for "unassigned" filter
      if (filters.includes('unassigned')) {
        if (ticket.assignees.length === 0) return true
      }

      // Check for specific member emails
      const memberFilters = filters.filter((f) => f !== 'me' && f !== 'unassigned')
      if (memberFilters.length > 0) {
        if (ticket.assignees.some((a) => memberFilters.includes(a))) return true
      }

      return false
    })
  }

  // Filter tickets by sprint
  const getSprintFilteredTickets = (tickets: Ticket[]) => {
    const showActiveSprint = searchParams.get('sprint') === 'active'

    if (!showActiveSprint || !activeSprint || activeSprintTicketIds.size === 0) {
      return tickets
    }

    return tickets.filter((ticket) => activeSprintTicketIds.has(ticket.id))
  }

  // Combine both filters
  const applyAllFilters = (tickets: Ticket[]) => {
    let filtered = tickets

    // First apply assignee filter
    const assigneeParam = searchParams.get('assignee')
    if (assigneeParam) {
      filtered = getFilteredTickets(filtered)
    }

    // Then apply sprint filter
    filtered = getSprintFilteredTickets(filtered)

    return filtered
  }

  return (
    <>
      <div className="flex flex-col h-[calc(100vh-3.5rem-4rem)] md:h-[calc(100vh-4rem)]">
        {/* Toolbar with filters */}
        <div className="flex items-center gap-2 px-4 md:px-6 pt-4 pb-2">
          <SprintFilter hasActiveSprint={!!activeSprint} />
          <AssigneeFilter members={members} currentUserEmail={user?.email} />
        </div>

        {/* Board */}
        <div className="flex-1 overflow-hidden">
          <KanbanBoard filterFn={applyAllFilters} />
        </div>
      </div>

      <MobileFAB onClick={() => setShowCreateDialog(true)} />

      <CreateTicketDialog
        open={showCreateDialog}
        onClose={() => setShowCreateDialog(false)}
      />
    </>
  )
}

function BoardPage() {
  return (
    <Suspense fallback={<div className="flex items-center justify-center h-full"><div className="text-muted-foreground">Loading...</div></div>}>
      <BoardContent />
    </Suspense>
  )
}

export default withAuth(BoardPage)
