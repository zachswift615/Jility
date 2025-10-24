'use client'

import { useState, useEffect, useCallback } from 'react'
import { useSearchParams, useRouter } from 'next/navigation'
import {
  DndContext,
  DragEndEvent,
  DragOverlay,
  PointerSensor,
  useSensor,
  useSensors,
  DragStartEvent,
} from '@dnd-kit/core'
import type { Ticket, TicketStatus, WebSocketMessage } from '@/lib/types'
import { api } from '@/lib/api'
import { useWebSocket } from '@/lib/websocket'
import { Column } from './column'
import { TicketCard } from './ticket-card'
import { CreateTicketDialog } from '../ticket/create-ticket-dialog'

const STATUSES: TicketStatus[] = ['backlog', 'todo', 'in_progress', 'review', 'done', 'blocked']

export function KanbanBoard() {
  const searchParams = useSearchParams()
  const router = useRouter()
  const [tickets, setTickets] = useState<Ticket[]>([])
  const [activeTicket, setActiveTicket] = useState<Ticket | null>(null)
  const [loading, setLoading] = useState(true)
  const [showCreateDialog, setShowCreateDialog] = useState(false)

  // Check for create parameter
  useEffect(() => {
    if (searchParams.get('create') === 'true') {
      setShowCreateDialog(true)
    }
  }, [searchParams])

  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 8,
      },
    })
  )

  const loadTickets = useCallback(async () => {
    try {
      const data = await api.listTickets()
      setTickets(data)
    } catch (error) {
      console.error('Failed to load tickets:', error)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    loadTickets()
  }, [loadTickets])

  const handleWebSocketMessage = useCallback((message: WebSocketMessage) => {
    console.log('WebSocket message:', message)

    switch (message.type) {
      case 'ticket_created':
        if (message.ticket) {
          setTickets((prev) => [...prev, message.ticket!])
        }
        break

      case 'ticket_updated':
        if (message.ticket) {
          setTickets((prev) =>
            prev.map((t) => (t.id === message.ticket!.id ? message.ticket! : t))
          )
        }
        break

      case 'status_changed':
        if (message.ticket_id && message.new_status) {
          setTickets((prev) =>
            prev.map((t) =>
              t.id === message.ticket_id
                ? { ...t, status: message.new_status! }
                : t
            )
          )
        }
        break
    }
  }, [])

  useWebSocket(handleWebSocketMessage)

  const handleDragStart = (event: DragStartEvent) => {
    const ticket = tickets.find((t) => t.id === event.active.id)
    setActiveTicket(ticket || null)
  }

  const handleDragEnd = async (event: DragEndEvent) => {
    const { active, over } = event
    setActiveTicket(null)

    console.log('[DragEnd] Event:', { activeId: active.id, overId: over?.id })

    if (!over) {
      console.log('[DragEnd] No drop target, aborting')
      return
    }

    const ticketId = active.id as string
    const newStatus = over.id as TicketStatus

    const ticket = tickets.find((t) => t.id === ticketId)
    console.log('[DragEnd] Ticket:', ticket, 'New status:', newStatus)

    if (!ticket || ticket.status === newStatus) {
      console.log('[DragEnd] Skipping update - ticket not found or same status')
      return
    }

    console.log('[DragEnd] Updating ticket status from', ticket.status, 'to', newStatus)

    // Optimistic update
    setTickets((prev) =>
      prev.map((t) => (t.id === ticketId ? { ...t, status: newStatus } : t))
    )

    try {
      const result = await api.updateTicketStatus(ticketId, newStatus)
      console.log('[DragEnd] Update successful:', result)
    } catch (error) {
      console.error('[DragEnd] Failed to update ticket status:', error)
      // Revert on error
      loadTickets()
    }
  }

  const handleCloseDialog = () => {
    setShowCreateDialog(false)
    // Remove create param from URL
    router.push('/board')
  }

  const handleTicketCreated = () => {
    loadTickets()
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-muted-foreground">Loading tickets...</div>
      </div>
    )
  }

  return (
    <>
      <DndContext
        sensors={sensors}
        onDragStart={handleDragStart}
        onDragEnd={handleDragEnd}
      >
        <div className="flex gap-4 overflow-x-auto pb-4 px-6 pt-6 h-full">
          {STATUSES.map((status) => (
            <Column
              key={status}
              status={status}
              tickets={tickets.filter((t) => t.status === status)}
            />
          ))}
        </div>

        <DragOverlay>
          {activeTicket && <TicketCard ticket={activeTicket} />}
        </DragOverlay>
      </DndContext>

      <CreateTicketDialog
        open={showCreateDialog}
        onClose={handleCloseDialog}
        onCreated={handleTicketCreated}
      />
    </>
  )
}
