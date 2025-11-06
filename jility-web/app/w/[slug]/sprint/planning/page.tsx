'use client'

import { useState, useEffect } from 'react'
import { DndContext, DragEndEvent, DragOverlay } from '@dnd-kit/core'
import { SortableContext, arrayMove } from '@dnd-kit/sortable'
import { calculateCapacity, formatSprintDateRange, getSprintStatusColor } from '@/lib/sprint-utils'

interface Ticket {
  id: string
  number: string
  title: string
  story_points?: number
  status: string
}

interface Sprint {
  id: string
  name: string
  goal?: string
  status: string
  start_date?: string
  end_date?: string
}

export default function SprintPlanningPage() {
  const [sprints, setSprints] = useState<Sprint[]>([])
  const [selectedSprint, setSelectedSprint] = useState<Sprint | null>(null)
  const [sprintTickets, setSprintTickets] = useState<Ticket[]>([])
  const [backlogTickets, setBacklogTickets] = useState<Ticket[]>([])
  const [loading, setLoading] = useState(true)

  const projectId = '550e8400-e29b-41d4-a716-446655440000' // TODO: Get from context/params

  useEffect(() => {
    fetchSprints()
    fetchBacklogTickets()
  }, [])

  useEffect(() => {
    if (selectedSprint) {
      fetchSprintDetails(selectedSprint.id)
    }
  }, [selectedSprint])

  async function fetchSprints() {
    try {
      const res = await fetch(`http://localhost:3001/api/projects/${projectId}/sprints?status=planning`)
      if (res.ok) {
        const data = await res.json()
        setSprints(data)
        if (data.length > 0) {
          setSelectedSprint(data[0])
        }
      }
    } catch (error) {
      console.error('Failed to fetch sprints:', error)
    } finally {
      setLoading(false)
    }
  }

  async function fetchSprintDetails(sprintId: string) {
    try {
      const res = await fetch(`http://localhost:3001/api/sprints/${sprintId}`)
      if (res.ok) {
        const data = await res.json()
        setSprintTickets(data.tickets)
      }
    } catch (error) {
      console.error('Failed to fetch sprint details:', error)
    }
  }

  async function fetchBacklogTickets() {
    try {
      const res = await fetch(`http://localhost:3001/api/tickets?project_id=${projectId}&status=backlog`)
      if (res.ok) {
        const data = await res.json()
        setBacklogTickets(data)
      }
    } catch (error) {
      console.error('Failed to fetch backlog:', error)
    }
  }

  async function addTicketToSprint(ticketId: string) {
    if (!selectedSprint) return

    try {
      const res = await fetch(
        `http://localhost:3001/api/sprints/${selectedSprint.id}/tickets/${ticketId}`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ added_by: 'user' }),
        }
      )

      if (res.ok) {
        // Move ticket from backlog to sprint
        const ticket = backlogTickets.find(t => t.id === ticketId)
        if (ticket) {
          setBacklogTickets(backlogTickets.filter(t => t.id !== ticketId))
          setSprintTickets([...sprintTickets, ticket])
        }
      }
    } catch (error) {
      console.error('Failed to add ticket to sprint:', error)
    }
  }

  async function removeTicketFromSprint(ticketId: string) {
    if (!selectedSprint) return

    try {
      const res = await fetch(
        `http://localhost:3001/api/sprints/${selectedSprint.id}/tickets/${ticketId}`,
        { method: 'DELETE' }
      )

      if (res.ok) {
        // Move ticket from sprint to backlog
        const ticket = sprintTickets.find(t => t.id === ticketId)
        if (ticket) {
          setSprintTickets(sprintTickets.filter(t => t.id !== ticketId))
          setBacklogTickets([...backlogTickets, ticket])
        }
      }
    } catch (error) {
      console.error('Failed to remove ticket from sprint:', error)
    }
  }

  async function startSprint() {
    if (!selectedSprint) return

    const startDate = new Date().toISOString()
    const endDate = new Date(Date.now() + 14 * 24 * 60 * 60 * 1000).toISOString() // 2 weeks

    try {
      const res = await fetch(`http://localhost:3001/api/sprints/${selectedSprint.id}/start`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ start_date: startDate, end_date: endDate }),
      })

      if (res.ok) {
        window.location.href = '/sprint/active'
      }
    } catch (error) {
      console.error('Failed to start sprint:', error)
    }
  }

  const plannedPoints = sprintTickets.reduce((sum, t) => sum + (t.story_points || 0), 0)
  const capacity = calculateCapacity(5, 14) // 5 team members, 14 days
  const capacityPercentage = capacity > 0 ? Math.round((plannedPoints / capacity) * 100) : 0

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-gray-600 dark:text-gray-400">Loading...</div>
      </div>
    )
  }

  if (!selectedSprint) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center">
          <h1 className="text-2xl font-bold mb-4">No Planning Sprint</h1>
          <p className="text-gray-600 dark:text-gray-400">
            Create a new sprint to start planning.
          </p>
        </div>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-8">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h1 className="text-3xl font-bold mb-2">{selectedSprint.name}</h1>
            {selectedSprint.goal && (
              <p className="text-gray-600 dark:text-gray-400">{selectedSprint.goal}</p>
            )}
          </div>
          <button
            onClick={startSprint}
            className="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors"
          >
            Start Sprint
          </button>
        </div>

        {/* Capacity Indicator */}
        <div className="bg-white dark:bg-gray-800 rounded-lg p-6 border border-gray-200 dark:border-gray-700">
          <div className="flex items-center justify-between mb-2">
            <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
              Capacity: {capacity} pts
            </span>
            <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
              Planned: {plannedPoints} pts
            </span>
            <span className={`text-sm font-medium ${
              capacityPercentage > 100 ? 'text-red-600' :
              capacityPercentage > 80 ? 'text-yellow-600' :
              'text-green-600'
            }`}>
              {capacityPercentage}%
            </span>
          </div>
          <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
            <div
              className={`h-2 rounded-full transition-all ${
                capacityPercentage > 100 ? 'bg-red-600' :
                capacityPercentage > 80 ? 'bg-yellow-600' :
                'bg-green-600'
              }`}
              style={{ width: `${Math.min(capacityPercentage, 100)}%` }}
            />
          </div>
        </div>
      </div>

      {/* Drag and Drop Area */}
      <div className="grid grid-cols-2 gap-6">
        {/* Backlog */}
        <div>
          <h2 className="text-xl font-bold mb-4">Backlog</h2>
          <div className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4 min-h-[500px]">
            <p className="text-sm text-gray-500 dark:text-gray-400 mb-4">
              Drag tickets to add to sprint →
            </p>
            <div className="space-y-2">
              {backlogTickets.map(ticket => (
                <div
                  key={ticket.id}
                  onClick={() => addTicketToSprint(ticket.id)}
                  className="p-4 bg-gray-50 dark:bg-gray-700 rounded border border-gray-200 dark:border-gray-600 hover:border-blue-500 cursor-pointer transition-colors"
                >
                  <div className="flex items-center justify-between">
                    <div className="flex-1">
                      <div className="font-medium">{ticket.number}</div>
                      <div className="text-sm text-gray-600 dark:text-gray-400">
                        {ticket.title}
                      </div>
                    </div>
                    {ticket.story_points && (
                      <div className="ml-4 px-2 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded text-sm font-medium">
                        {ticket.story_points} pts
                      </div>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* Sprint */}
        <div>
          <h2 className="text-xl font-bold mb-4">{selectedSprint.name}</h2>
          <div className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4 min-h-[500px]">
            <p className="text-sm text-gray-500 dark:text-gray-400 mb-4">
              ← Drag tickets to remove from sprint
            </p>
            <div className="space-y-2">
              {sprintTickets.map(ticket => (
                <div
                  key={ticket.id}
                  onClick={() => removeTicketFromSprint(ticket.id)}
                  className="p-4 bg-blue-50 dark:bg-blue-900/20 rounded border border-blue-200 dark:border-blue-700 hover:border-red-500 cursor-pointer transition-colors"
                >
                  <div className="flex items-center justify-between">
                    <div className="flex-1">
                      <div className="font-medium">{ticket.number}</div>
                      <div className="text-sm text-gray-600 dark:text-gray-400">
                        {ticket.title}
                      </div>
                    </div>
                    {ticket.story_points && (
                      <div className="ml-4 px-2 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded text-sm font-medium">
                        {ticket.story_points} pts
                      </div>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
