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
import type { Ticket } from '@/lib/types'
import { api } from '@/lib/api'
import { BacklogToolbar } from './backlog-toolbar'
import { BacklogSection } from './backlog-section'
import { BacklogTicketItem } from './backlog-ticket-item'
import { QuickAddInput } from './quick-add-input'
import { Button } from '@/components/ui/button'
import { Lightbulb, Bot } from 'lucide-react'

type BacklogCategory = 'ready' | 'needs_estimation' | 'ideas'

interface GroupedTickets {
  ready: Ticket[]
  needs_estimation: Ticket[]
  ideas: Ticket[]
}

export function BacklogView() {
  const [tickets, setTickets] = useState<Ticket[]>([])
  const [activeTicket, setActiveTicket] = useState<Ticket | null>(null)
  const [loading, setLoading] = useState(true)
  const [filter, setFilter] = useState<string>('all')
  const [expandedSections, setExpandedSections] = useState<Record<BacklogCategory, boolean>>({
    ready: true,
    needs_estimation: true,
    ideas: false,
  })

  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 8,
      },
    })
  )

  const loadTickets = useCallback(async () => {
    try {
      const data = await api.listTickets({ status: 'backlog' })
      setTickets(data)
    } catch (error) {
      console.error('Failed to load backlog tickets:', error)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    loadTickets()
  }, [loadTickets])

  // Group tickets by category
  const groupedTickets: GroupedTickets = tickets.reduce(
    (acc, ticket) => {
      // Categorize based on ticket properties
      if (ticket.story_points === undefined || ticket.story_points === null) {
        acc.needs_estimation.push(ticket)
      } else if (ticket.assignees.length > 0 || ticket.story_points > 0) {
        acc.ready.push(ticket)
      } else {
        acc.ideas.push(ticket)
      }
      return acc
    },
    { ready: [], needs_estimation: [], ideas: [] } as GroupedTickets
  )

  // Calculate statistics
  const totalPoints = tickets.reduce((sum, t) => sum + (t.story_points || 0), 0)
  const readyPoints = groupedTickets.ready.reduce((sum, t) => sum + (t.story_points || 0), 0)

  const handleDragStart = (event: DragStartEvent) => {
    const ticket = tickets.find((t) => t.id === event.active.id)
    setActiveTicket(ticket || null)
  }

  const handleDragEnd = async (event: DragEndEvent) => {
    setActiveTicket(null)
    // For now, just reorder within the same category
    // In the future, this could move between categories or reorder priority
  }

  const toggleSection = (category: BacklogCategory) => {
    setExpandedSections((prev) => ({
      ...prev,
      [category]: !prev[category],
    }))
  }

  const handleQuickAdd = async (title: string) => {
    try {
      await api.createTicket({
        title,
        description: '',
        status: 'backlog',
      })
      await loadTickets()
    } catch (error) {
      console.error('Failed to create ticket:', error)
    }
  }

  const handleMoveToBoard = async (category: BacklogCategory) => {
    try {
      const ticketsToMove = groupedTickets[category]
      for (const ticket of ticketsToMove) {
        await api.updateTicketStatus(ticket.id, 'todo')
      }
      await loadTickets()
    } catch (error) {
      console.error('Failed to move tickets to board:', error)
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-gray-500">Loading backlog...</div>
      </div>
    )
  }

  return (
    <div className="flex flex-col h-full bg-gray-50 p-6">
      <div className="max-w-7xl w-full mx-auto">
        <BacklogToolbar
          totalItems={tickets.length}
          totalPoints={totalPoints}
          filter={filter}
          onFilterChange={setFilter}
        />

        <DndContext
          sensors={sensors}
          onDragStart={handleDragStart}
          onDragEnd={handleDragEnd}
        >
          <div className="bg-white border border-gray-200 rounded-lg overflow-hidden shadow-sm">
            {/* Ready for Sprint Section */}
            <BacklogSection
              title="Ready for Sprint"
              count={groupedTickets.ready.length}
              points={readyPoints}
              expanded={expandedSections.ready}
              onToggle={() => toggleSection('ready')}
              action={
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => handleMoveToBoard('ready')}
                  disabled={groupedTickets.ready.length === 0}
                >
                  Move to Board
                </Button>
              }
            >
              {expandedSections.ready &&
                groupedTickets.ready.map((ticket) => (
                  <BacklogTicketItem key={ticket.id} ticket={ticket} />
                ))}
            </BacklogSection>

            {/* Needs Estimation Section */}
            <BacklogSection
              title="Needs Estimation"
              count={groupedTickets.needs_estimation.length}
              expanded={expandedSections.needs_estimation}
              onToggle={() => toggleSection('needs_estimation')}
              action={
                <Button
                  variant="outline"
                  size="sm"
                  disabled={groupedTickets.needs_estimation.length === 0}
                  className="flex items-center gap-1"
                >
                  <Bot className="h-4 w-4" />
                  AI Estimate
                </Button>
              }
            >
              {expandedSections.needs_estimation &&
                groupedTickets.needs_estimation.map((ticket) => (
                  <BacklogTicketItem key={ticket.id} ticket={ticket} />
                ))}
            </BacklogSection>

            {/* Ideas / Future Section */}
            <BacklogSection
              title="Ideas / Future"
              count={groupedTickets.ideas.length}
              subtitle="Low priority"
              expanded={expandedSections.ideas}
              onToggle={() => toggleSection('ideas')}
            >
              {expandedSections.ideas &&
                groupedTickets.ideas.map((ticket) => (
                  <BacklogTicketItem key={ticket.id} ticket={ticket} />
                ))}
            </BacklogSection>

            {/* Quick Add Input */}
            <QuickAddInput onAdd={handleQuickAdd} />
          </div>

          <DragOverlay>
            {activeTicket && <BacklogTicketItem ticket={activeTicket} isDragging />}
          </DragOverlay>
        </DndContext>

        {/* Tips Section */}
        <div className="mt-6 bg-blue-50 border border-blue-200 rounded-lg p-4">
          <div className="flex items-center gap-2 font-semibold text-blue-900 mb-2">
            <Lightbulb className="h-4 w-4" />
            Backlog Tips
          </div>
          <ul className="text-sm text-blue-800 space-y-1 ml-6 list-disc">
            <li>Drag tickets to reorder by priority (top = highest priority)</li>
            <li>Use sections to organize tickets by readiness</li>
            <li>Click "Quick Add" or press + anywhere to create tickets fast</li>
            <li>Let AI agents estimate story points for you</li>
            <li>Move tickets to board when ready to start work</li>
          </ul>
        </div>
      </div>
    </div>
  )
}
