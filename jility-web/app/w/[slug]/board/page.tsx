'use client'

import { useState, useEffect, Suspense } from 'react'
import { useSearchParams } from 'next/navigation'
import { KanbanBoard } from '@/components/kanban/board'
import { MobileFAB } from '@/components/layout/mobile-fab'
import { CreateTicketDialog } from '@/components/ticket/create-ticket-dialog'
import { AssigneeFilter } from '@/components/ticket/assignee-filter'
import { withAuth } from '@/lib/with-auth'
import { useAuth } from '@/lib/auth-context'
import { api } from '@/lib/api'
import { useWorkspace } from '@/lib/workspace-context'
import type { WorkspaceMember, Ticket } from '@/lib/types'

function BoardContent() {
  const [showCreateDialog, setShowCreateDialog] = useState(false)
  const [members, setMembers] = useState<WorkspaceMember[]>([])
  const { user } = useAuth()
  const { currentWorkspace } = useWorkspace()
  const slug = currentWorkspace?.slug || ''
  const searchParams = useSearchParams()

  // Fetch workspace members on mount
  useEffect(() => {
    const loadMembers = async () => {
      try {
        const data = await api.listWorkspaceMembers(slug)
        setMembers(data)
      } catch (error) {
        console.error('Failed to load workspace members:', error)
      }
    }

    if (slug) {
      loadMembers()
    }
  }, [slug])

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

  return (
    <>
      <div className="flex flex-col h-[calc(100vh-3.5rem-4rem)] md:h-[calc(100vh-4rem)]">
        {/* Toolbar with filter */}
        <div className="flex items-center gap-2 px-4 md:px-6 pt-4 pb-2">
          <AssigneeFilter members={members} currentUserEmail={user?.email} />
        </div>

        {/* Board */}
        <div className="flex-1 overflow-hidden">
          <KanbanBoard filterFn={getFilteredTickets} />
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
