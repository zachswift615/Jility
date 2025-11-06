'use client'

import { useState, useEffect } from 'react'
import { BurndownChart } from '@/components/sprint/burndown-chart'
import { calculateDaysRemaining, formatSprintDateRange } from '@/lib/sprint-utils'

interface Sprint {
  id: string
  name: string
  goal?: string
  status: string
  start_date?: string
  end_date?: string
}

interface SprintStats {
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

interface Ticket {
  id: string
  number: string
  title: string
  story_points?: number
  status: string
}

interface BurndownData {
  sprint_id: string
  data_points: Array<{
    date: string
    ideal: number
    actual: number
  }>
}

export default function ActiveSprintPage() {
  const [sprint, setSprint] = useState<Sprint | null>(null)
  const [stats, setStats] = useState<SprintStats | null>(null)
  const [tickets, setTickets] = useState<Ticket[]>([])
  const [burndownData, setBurndownData] = useState<BurndownData | null>(null)
  const [loading, setLoading] = useState(true)

  const projectId = '550e8400-e29b-41d4-a716-446655440000' // TODO: Get from context/params

  useEffect(() => {
    fetchActiveSprint()
  }, [])

  async function fetchActiveSprint() {
    try {
      // Fetch active sprint
      const sprintsRes = await fetch(
        `http://localhost:3001/api/projects/${projectId}/sprints?status=active`
      )
      if (!sprintsRes.ok) throw new Error('Failed to fetch sprints')

      const sprints = await sprintsRes.json()
      if (sprints.length === 0) {
        setLoading(false)
        return
      }

      const activeSprint = sprints[0]
      setSprint(activeSprint)

      // Fetch sprint details
      const detailsRes = await fetch(`http://localhost:3001/api/sprints/${activeSprint.id}`)
      if (detailsRes.ok) {
        const details = await detailsRes.json()
        setStats(details.stats)
        setTickets(details.tickets)
      }

      // Fetch burndown data
      const burndownRes = await fetch(`http://localhost:3001/api/sprints/${activeSprint.id}/burndown`)
      if (burndownRes.ok) {
        const data = await burndownRes.json()
        setBurndownData(data)
      }
    } catch (error) {
      console.error('Error fetching active sprint:', error)
    } finally {
      setLoading(false)
    }
  }

  async function completeSprint() {
    if (!sprint) return

    const confirmed = confirm('Are you sure you want to complete this sprint?')
    if (!confirmed) return

    try {
      const res = await fetch(`http://localhost:3001/api/sprints/${sprint.id}/complete`, {
        method: 'POST',
      })

      if (res.ok) {
        window.location.href = '/sprint/history'
      }
    } catch (error) {
      console.error('Failed to complete sprint:', error)
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-gray-600 dark:text-gray-400">Loading...</div>
      </div>
    )
  }

  if (!sprint) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center">
          <h1 className="text-2xl font-bold mb-4">No Active Sprint</h1>
          <p className="text-gray-600 dark:text-gray-400 mb-6">
            There is no active sprint. Start a sprint from the planning view.
          </p>
          <a
            href="/sprint/planning"
            className="inline-block px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors"
          >
            Go to Sprint Planning
          </a>
        </div>
      </div>
    )
  }

  const daysRemaining = sprint.end_date ? calculateDaysRemaining(sprint.end_date) : 0
  const dateRange = sprint.start_date && sprint.end_date
    ? formatSprintDateRange(sprint.start_date, sprint.end_date)
    : ''

  return (
    <div className="container mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-8">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h1 className="text-3xl font-bold mb-2">{sprint.name}</h1>
            {sprint.goal && (
              <p className="text-gray-600 dark:text-gray-400">{sprint.goal}</p>
            )}
            <p className="text-sm text-gray-500 dark:text-gray-500 mt-1">
              {dateRange} • {daysRemaining > 0 ? `${daysRemaining} days remaining` : 'Sprint ended'}
            </p>
          </div>
          <button
            onClick={completeSprint}
            className="px-6 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg font-medium transition-colors"
          >
            Complete Sprint
          </button>
        </div>

        {/* Progress Bar */}
        {stats && (
          <div className="bg-white dark:bg-gray-800 rounded-lg p-6 border border-gray-200 dark:border-gray-700">
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
                {stats.completed_points}/{stats.total_points} points completed
              </span>
              <span className="text-sm font-medium text-blue-600 dark:text-blue-400">
                {Math.round(stats.completion_percentage)}%
              </span>
            </div>
            <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-3">
              <div
                className="h-3 rounded-full bg-blue-600 transition-all"
                style={{ width: `${Math.min(stats.completion_percentage, 100)}%` }}
              />
            </div>
            <div className="grid grid-cols-4 gap-4 mt-4">
              <div>
                <div className="text-2xl font-bold text-gray-900 dark:text-gray-100">
                  {stats.total_tickets}
                </div>
                <div className="text-sm text-gray-600 dark:text-gray-400">Total Tickets</div>
              </div>
              <div>
                <div className="text-2xl font-bold text-green-600 dark:text-green-400">
                  {stats.completed_tickets}
                </div>
                <div className="text-sm text-gray-600 dark:text-gray-400">Completed</div>
              </div>
              <div>
                <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">
                  {stats.in_progress_tickets}
                </div>
                <div className="text-sm text-gray-600 dark:text-gray-400">In Progress</div>
              </div>
              <div>
                <div className="text-2xl font-bold text-gray-600 dark:text-gray-400">
                  {stats.todo_tickets}
                </div>
                <div className="text-sm text-gray-600 dark:text-gray-400">To Do</div>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Burndown Chart */}
      {burndownData && (
        <div className="mb-8">
          <h2 className="text-2xl font-bold mb-4">Burndown Chart</h2>
          <div className="bg-white dark:bg-gray-800 rounded-lg p-6 border border-gray-200 dark:border-gray-700">
            <BurndownChart data={burndownData} />
          </div>
        </div>
      )}

      {/* Sprint Tickets */}
      <div>
        <h2 className="text-2xl font-bold mb-4">Sprint Tickets</h2>
        <div className="grid grid-cols-3 gap-6">
          {/* To Do */}
          <div>
            <h3 className="font-semibold mb-3 text-gray-700 dark:text-gray-300">
              To Do ({tickets.filter(t => t.status === 'todo' || t.status === 'backlog').length})
            </h3>
            <div className="space-y-2">
              {tickets
                .filter(t => t.status === 'todo' || t.status === 'backlog')
                .map(ticket => (
                  <a
                    key={ticket.id}
                    href={`/ticket/${ticket.id}`}
                    className="block p-4 bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 hover:border-blue-500 transition-colors"
                  >
                    <div className="font-medium text-sm mb-1">{ticket.number}</div>
                    <div className="text-sm text-gray-600 dark:text-gray-400">
                      {ticket.title}
                    </div>
                    {ticket.story_points && (
                      <div className="mt-2 inline-block px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded text-xs">
                        {ticket.story_points} pts
                      </div>
                    )}
                  </a>
                ))}
            </div>
          </div>

          {/* In Progress */}
          <div>
            <h3 className="font-semibold mb-3 text-gray-700 dark:text-gray-300">
              In Progress ({tickets.filter(t => t.status === 'in_progress').length})
            </h3>
            <div className="space-y-2">
              {tickets
                .filter(t => t.status === 'in_progress')
                .map(ticket => (
                  <a
                    key={ticket.id}
                    href={`/ticket/${ticket.id}`}
                    className="block p-4 bg-white dark:bg-gray-800 rounded border border-blue-200 dark:border-blue-700 hover:border-blue-500 transition-colors"
                  >
                    <div className="font-medium text-sm mb-1">{ticket.number}</div>
                    <div className="text-sm text-gray-600 dark:text-gray-400">
                      {ticket.title}
                    </div>
                    {ticket.story_points && (
                      <div className="mt-2 inline-block px-2 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded text-xs">
                        {ticket.story_points} pts
                      </div>
                    )}
                  </a>
                ))}
            </div>
          </div>

          {/* Done */}
          <div>
            <h3 className="font-semibold mb-3 text-gray-700 dark:text-gray-300">
              Done ({tickets.filter(t => t.status === 'done').length})
            </h3>
            <div className="space-y-2">
              {tickets
                .filter(t => t.status === 'done')
                .map(ticket => (
                  <a
                    key={ticket.id}
                    href={`/ticket/${ticket.id}`}
                    className="block p-4 bg-white dark:bg-gray-800 rounded border border-green-200 dark:border-green-700 hover:border-green-500 transition-colors"
                  >
                    <div className="font-medium text-sm mb-1">{ticket.number}</div>
                    <div className="text-sm text-gray-600 dark:text-gray-400">
                      {ticket.title}
                    </div>
                    {ticket.story_points && (
                      <div className="mt-2 inline-block px-2 py-1 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded text-xs">
                        {ticket.story_points} pts
                      </div>
                    )}
                  </a>
                ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
