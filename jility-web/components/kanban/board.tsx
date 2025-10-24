'use client'

import { useState, useEffect, useCallback } from 'react'
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

const STATUSES: TicketStatus[] = ['backlog', 'todo', 'in_progress', 'review', 'done', 'blocked']

export function KanbanBoard() {
  const [tickets, setTickets] = useState<Ticket[]>([])
  const [activeTicket, setActiveTicket] = useState<Ticket | null>(null)
  const [loading, setLoading] = useState(true)

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

    if (!over) return

    const ticketId = active.id as string
    const newStatus = over.id as TicketStatus

    const ticket = tickets.find((t) => t.id === ticketId)
    if (!ticket || ticket.status === newStatus) return

    // Optimistic update
    setTickets((prev) =>
      prev.map((t) => (t.id === ticketId ? { ...t, status: newStatus } : t))
    )

    try {
      await api.updateTicketStatus(ticketId, newStatus)
    } catch (error) {
      console.error('Failed to update ticket status:', error)
      // Revert on error
      loadTickets()
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-muted-foreground">Loading tickets...</div>
      </div>
    )
  }

  return (
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
  )
}
