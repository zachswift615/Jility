'use client'

import { useState, useEffect, useCallback } from 'react'
import { Calendar, Target, TrendingUp } from 'lucide-react'
import { withAuth } from '@/lib/with-auth'
import { useWorkspace } from '@/lib/workspace-context'
import { useAuth } from '@/lib/auth-context'
import { api } from '@/lib/api'
import type { Sprint, SprintDetails } from '@/lib/types'
import { CompleteSprintDialog } from '@/components/sprint/complete-sprint-dialog'

function ActiveSprintContent() {
  const [sprint, setSprint] = useState<SprintDetails | null>(null)
  const [loading, setLoading] = useState(true)
  const [showCompleteDialog, setShowCompleteDialog] = useState(false)
  const { currentWorkspace } = useWorkspace()
  const { user } = useAuth()
  const slug = currentWorkspace?.slug || ''

  const fetchActiveSprint = useCallback(async () => {
    if (!slug) return
    try {
      const sprints = await api.listSprints(slug, 'active')
      if (sprints.length > 0) {
        const details = await api.getSprint(sprints[0].id)
        setSprint(details)
      }
    } catch (error) {
      console.error('Failed to fetch active sprint:', error)
    } finally {
      setLoading(false)
    }
  }, [slug])

  useEffect(() => {
    fetchActiveSprint()
  }, [fetchActiveSprint])

  async function handleCompleteClick() {
    setShowCompleteDialog(true)
  }

  async function handleCompleteConfirm(action: 'rollover' | 'backlog' | 'keep') {
    if (!sprint || !slug || !user) return

    const incompleteTickets = sprint.tickets.filter(t => t.status !== 'done')

    try {
      if (action === 'rollover' && incompleteTickets.length > 0) {
        // Generate next sprint name using improved logic from Task 4
        const currentName = sprint.sprint.name
        let nextName = 'Sprint 1'

        // Try to extract number from current sprint name
        const match = currentName.match(/(\d+)/)
        if (match) {
          const num = parseInt(match[1])
          const prefix = currentName.substring(0, match.index)
          const suffix = currentName.substring(match.index! + match[1].length)
          nextName = `${prefix}${num + 1}${suffix}`
        } else {
          // No number found, append " 2" to current name
          nextName = `${currentName} 2`
        }

        // Create next sprint
        const nextSprint = await api.createSprint(slug, {
          name: nextName,
          goal: sprint.sprint.goal, // Copy goal from current sprint
        })

        // Move incomplete tickets to next sprint
        for (const ticket of incompleteTickets) {
          await api.removeTicketFromSprint(sprint.sprint.id, ticket.id)
          await api.addTicketToSprint(nextSprint.id, ticket.id, user.email)
        }
      } else if (action === 'backlog' && incompleteTickets.length > 0) {
        // Remove from sprint and set to backlog status
        for (const ticket of incompleteTickets) {
          await api.removeTicketFromSprint(sprint.sprint.id, ticket.id)
          await api.updateTicketStatus(ticket.id, 'backlog')
        }
      }
      // If 'keep', we don't do anything with the incomplete tickets

      // Complete the sprint
      await api.completeSprint(sprint.sprint.id)

      // Redirect to history
      window.location.href = `/w/${slug}/sprint/history`
    } catch (error) {
      console.error('Failed to complete sprint:', error)
      throw error
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-muted-foreground">Loading...</div>
      </div>
    )
  }

  if (!sprint) {
    return (
      <div className="container mx-auto px-3 md:px-6 py-4 md:py-8">
        <div className="text-center py-12">
          <h1 className="text-2xl md:text-3xl font-bold mb-4">No Active Sprint</h1>
          <p className="text-muted-foreground mb-6">
            Start a sprint from the planning page to begin tracking progress.
          </p>
          <a
            href={`/w/${slug}/sprint/planning`}
            className="inline-block px-6 py-3 bg-primary text-primary-foreground rounded-lg hover:opacity-90"
          >
            Go to Sprint Planning
          </a>
        </div>
      </div>
    )
  }

  const { stats } = sprint
  const daysRemaining = sprint.sprint.end_date
    ? Math.max(0, Math.ceil((new Date(sprint.sprint.end_date).getTime() - Date.now()) / (1000 * 60 * 60 * 24)))
    : null

  return (
    <div className="container mx-auto px-3 md:px-6 py-4 md:py-8">
      {/* Header */}
      <div className="mb-6 md:mb-8">
        <div className="flex flex-col md:flex-row md:items-center md:justify-between gap-4 mb-6">
          <div>
            <h1 className="text-2xl md:text-3xl font-bold mb-2">{sprint.sprint.name}</h1>
            {sprint.sprint.goal && (
              <p className="text-muted-foreground">{sprint.sprint.goal}</p>
            )}
          </div>
          <button
            onClick={handleCompleteClick}
            className="px-6 py-2 bg-primary text-primary-foreground rounded-lg hover:opacity-90 self-start md:self-auto"
          >
            Complete Sprint
          </button>
        </div>

        {/* Sprint Info Cards */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          {/* Days Remaining */}
          {daysRemaining !== null && (
            <div className="bg-card border-border border rounded-lg p-4">
              <div className="flex items-center gap-2 text-muted-foreground mb-2">
                <Calendar className="h-4 w-4" />
                <span className="text-sm font-medium">Days Remaining</span>
              </div>
              <div className="text-2xl font-bold">{daysRemaining}</div>
            </div>
          )}

          {/* Sprint Goal */}
          {sprint.sprint.goal && (
            <div className="bg-card border-border border rounded-lg p-4">
              <div className="flex items-center gap-2 text-muted-foreground mb-2">
                <Target className="h-4 w-4" />
                <span className="text-sm font-medium">Sprint Goal</span>
              </div>
              <div className="text-sm line-clamp-2">{sprint.sprint.goal}</div>
            </div>
          )}

          {/* Completion */}
          <div className="bg-card border-border border rounded-lg p-4">
            <div className="flex items-center gap-2 text-muted-foreground mb-2">
              <TrendingUp className="h-4 w-4" />
              <span className="text-sm font-medium">Completion</span>
            </div>
            <div className="text-2xl font-bold">
              {Math.round(stats.completion_percentage)}%
            </div>
            <div className="text-sm text-muted-foreground mt-1">
              {stats.completed_points}/{stats.total_points} pts
            </div>
          </div>

          {/* Tickets */}
          <div className="bg-card border-border border rounded-lg p-4">
            <div className="flex items-center gap-2 text-muted-foreground mb-2">
              <span className="text-sm font-medium">Tickets</span>
            </div>
            <div className="text-2xl font-bold">{stats.total_tickets}</div>
            <div className="text-sm text-muted-foreground mt-1">
              {stats.completed_tickets} done
            </div>
          </div>
        </div>
      </div>

      {/* Progress Bar */}
      <div className="bg-card border-border border rounded-lg p-4 md:p-6 mb-6">
        <h3 className="font-semibold mb-4">Sprint Progress</h3>
        <div className="space-y-3">
          <div>
            <div className="flex justify-between text-sm mb-1">
              <span>Completed</span>
              <span className="font-medium">{stats.completed_points} pts</span>
            </div>
            <div className="w-full bg-secondary rounded-full h-2">
              <div
                className="bg-green-600 h-2 rounded-full transition-all"
                style={{ width: `${stats.total_points > 0 ? (stats.completed_points / stats.total_points) * 100 : 0}%` }}
              />
            </div>
          </div>
          <div>
            <div className="flex justify-between text-sm mb-1">
              <span>In Progress</span>
              <span className="font-medium">{stats.in_progress_points} pts</span>
            </div>
            <div className="w-full bg-secondary rounded-full h-2">
              <div
                className="bg-yellow-600 h-2 rounded-full transition-all"
                style={{ width: `${stats.total_points > 0 ? (stats.in_progress_points / stats.total_points) * 100 : 0}%` }}
              />
            </div>
          </div>
          <div>
            <div className="flex justify-between text-sm mb-1">
              <span>To Do</span>
              <span className="font-medium">{stats.todo_points} pts</span>
            </div>
            <div className="w-full bg-secondary rounded-full h-2">
              <div
                className="bg-muted h-2 rounded-full transition-all"
                style={{ width: `${stats.total_points > 0 ? (stats.todo_points / stats.total_points) * 100 : 0}%` }}
              />
            </div>
          </div>
        </div>
      </div>

      {/* Tickets List */}
      <div className="bg-card border-border border rounded-lg p-4 md:p-6">
        <h3 className="font-semibold mb-4">Sprint Tickets ({sprint.tickets.length})</h3>
        <div className="space-y-2">
          {sprint.tickets.map(ticket => (
            <a
              key={ticket.id}
              href={`/w/${slug}/ticket/${ticket.id}`}
              className="block p-4 bg-secondary rounded border-border border hover:border-primary transition-colors"
            >
              <div className="flex items-center justify-between">
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-3">
                    <span className="font-medium text-sm">{ticket.number}</span>
                    <span className={`px-2 py-0.5 rounded text-xs font-medium ${
                      ticket.status === 'done' ? 'bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200' :
                      ticket.status === 'in_progress' ? 'bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200' :
                      'bg-muted text-muted-foreground'
                    }`}>
                      {ticket.status}
                    </span>
                  </div>
                  <div className="text-sm mt-1 truncate">{ticket.title}</div>
                </div>
                {ticket.story_points && (
                  <div className="ml-4 px-2 py-1 bg-primary/10 text-primary rounded text-sm font-medium flex-shrink-0">
                    {ticket.story_points} pts
                  </div>
                )}
              </div>
            </a>
          ))}
        </div>
      </div>

      <CompleteSprintDialog
        isOpen={showCompleteDialog}
        onClose={() => setShowCompleteDialog(false)}
        onConfirm={handleCompleteConfirm}
        incompleteTickets={sprint?.tickets.filter(t => t.status !== 'done') || []}
        sprintName={sprint?.sprint.name || ''}
      />
    </div>
  )
}

export default withAuth(ActiveSprintContent)
