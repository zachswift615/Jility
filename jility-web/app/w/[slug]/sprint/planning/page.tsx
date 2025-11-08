'use client'

import { useState, useEffect, useCallback } from 'react'
import { Plus, Info } from 'lucide-react'
import { withAuth } from '@/lib/with-auth'
import { useAuth } from '@/lib/auth-context'
import { useWorkspace } from '@/lib/workspace-context'
import { api } from '@/lib/api'
import { CreateSprintDialog } from '@/components/sprint/create-sprint-dialog'
import { useSprintCapacity } from '@/lib/use-sprint-capacity'
import { CapacityEditor } from '@/components/sprint/capacity-editor'
import type { Sprint, Ticket } from '@/lib/types'

function SprintPlanningContent() {
  const [sprints, setSprints] = useState<Sprint[]>([])
  const [selectedSprint, setSelectedSprint] = useState<Sprint | null>(null)
  const [sprintTickets, setSprintTickets] = useState<Ticket[]>([])
  const [backlogTickets, setBacklogTickets] = useState<Ticket[]>([])
  const [loading, setLoading] = useState(true)
  const [showCreateDialog, setShowCreateDialog] = useState(false)
  const { user } = useAuth()
  const { currentWorkspace } = useWorkspace()
  const slug = currentWorkspace?.slug || ''

  const fetchSprints = useCallback(async () => {
    if (!slug) return
    try {
      const data = await api.listSprints(slug, 'planning')
      setSprints(data)
      if (data.length > 0 && !selectedSprint) {
        setSelectedSprint(data[0])
      }
    } catch (error) {
      console.error('Failed to fetch sprints:', error)
    } finally {
      setLoading(false)
    }
  }, [slug, selectedSprint])

  const fetchSprintDetails = useCallback(async (sprintId: string) => {
    try {
      const data = await api.getSprint(sprintId)
      setSprintTickets(data.tickets)
    } catch (error) {
      console.error('Failed to fetch sprint details:', error)
    }
  }, [])

  const fetchBacklogTickets = useCallback(async () => {
    if (!slug) return
    try {
      const data = await api.listTickets({ status: 'backlog' })
      setBacklogTickets(data)
    } catch (error) {
      console.error('Failed to fetch backlog:', error)
    }
  }, [slug])

  useEffect(() => {
    fetchSprints()
    fetchBacklogTickets()
  }, [fetchSprints, fetchBacklogTickets])

  useEffect(() => {
    if (selectedSprint) {
      fetchSprintDetails(selectedSprint.id)
    }
  }, [selectedSprint, fetchSprintDetails])

  async function addTicketToSprint(ticketId: string) {
    if (!selectedSprint || !user) return

    try {
      await api.addTicketToSprint(selectedSprint.id, ticketId, user.email)
      // Move ticket from backlog to sprint
      const ticket = backlogTickets.find(t => t.id === ticketId)
      if (ticket) {
        setBacklogTickets(prev => prev.filter(t => t.id !== ticketId))
        setSprintTickets(prev => [...prev, ticket])
      }
    } catch (error) {
      console.error('Failed to add ticket to sprint:', error)
    }
  }

  async function removeTicketFromSprint(ticketId: string) {
    if (!selectedSprint) return

    try {
      await api.removeTicketFromSprint(selectedSprint.id, ticketId)
      // Move ticket from sprint to backlog
      const ticket = sprintTickets.find(t => t.id === ticketId)
      if (ticket) {
        setSprintTickets(prev => prev.filter(t => t.id !== ticketId))
        setBacklogTickets(prev => [...prev, ticket])
      }
    } catch (error) {
      console.error('Failed to remove ticket from sprint:', error)
    }
  }

  async function startSprint() {
    if (!selectedSprint || !slug) return

    const startDate = new Date().toISOString()
    const endDate = new Date(Date.now() + 14 * 24 * 60 * 60 * 1000).toISOString()

    try {
      await api.startSprint(selectedSprint.id, { start_date: startDate, end_date: endDate })
      window.location.href = `/w/${slug}/sprint/active`
    } catch (error) {
      console.error('Failed to start sprint:', error)
    }
  }

  const plannedPoints = sprintTickets.reduce((sum, t) => sum + (t.story_points || 0), 0)
  const { capacity: workspaceCapacity, updateCapacity, loading: capacityLoading } = useSprintCapacity()
  const capacity = workspaceCapacity || 40
  const capacityPercentage = capacity > 0 ? Math.round((plannedPoints / capacity) * 100) : 0

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-muted-foreground">Loading...</div>
      </div>
    )
  }

  return (
    <>
      <div className="container mx-auto px-3 md:px-6 py-4 md:py-8">
        {/* Header */}
        <div className="mb-6 md:mb-8">
          <div className="flex items-center justify-between mb-4">
            <h1 className="text-2xl md:text-3xl font-bold">Sprint Planning</h1>
            <button
              onClick={() => setShowCreateDialog(true)}
              className="flex items-center gap-2 px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:opacity-90 transition-opacity"
            >
              <Plus className="h-4 w-4" />
              <span className="hidden sm:inline">New Sprint</span>
            </button>
          </div>

          {sprints.length === 0 ? (
            <div className="text-center py-12">
              <p className="text-muted-foreground mb-4">No planning sprints yet</p>
              <button
                onClick={() => setShowCreateDialog(true)}
                className="px-6 py-3 bg-primary text-primary-foreground rounded-lg hover:opacity-90"
              >
                Create Your First Sprint
              </button>
            </div>
          ) : selectedSprint && (
            <>
              <div className="flex items-center justify-between mb-4">
                <div>
                  <h2 className="text-xl md:text-2xl font-bold">{selectedSprint.name}</h2>
                  {selectedSprint.goal && (
                    <p className="text-muted-foreground">{selectedSprint.goal}</p>
                  )}
                </div>
                <button
                  onClick={startSprint}
                  disabled={sprintTickets.length === 0}
                  className="px-6 py-2 bg-primary text-primary-foreground rounded-lg hover:opacity-90 disabled:opacity-50 disabled:cursor-not-allowed transition-opacity"
                >
                  Start Sprint
                </button>
              </div>

              {/* Capacity Indicator */}
              <div className="bg-card rounded-lg p-4 md:p-6 border-border border">
                <div className="flex items-center justify-between mb-2 text-sm md:text-base">
                  <div className="flex items-center gap-2">
                    <span className="font-medium">
                      Capacity:{' '}
                      {capacityLoading ? (
                        <span className="text-muted-foreground">Loading...</span>
                      ) : (
                        <CapacityEditor capacity={capacity} onSave={updateCapacity} />
                      )}
                    </span>
                    <div className="group relative">
                      <Info className="h-4 w-4 text-muted-foreground cursor-help" />
                      <div className="absolute left-0 top-6 w-64 p-2 bg-popover text-popover-foreground border border-border rounded shadow-lg opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-10">
                        <p className="text-xs">
                          Sprint capacity is the target amount of work (in story points) your team can complete in one sprint.
                          {workspaceCapacity && workspaceCapacity !== 40 && (
                            <> Defaults to your team&apos;s average velocity from past sprints.</>
                          )}
                        </p>
                      </div>
                    </div>
                  </div>
                  <span className="font-medium">Planned: {plannedPoints} pts</span>
                  <span className={`font-medium ${
                    capacityPercentage > 100 ? 'text-destructive' :
                    capacityPercentage > 80 ? 'text-yellow-600' :
                    'text-green-600'
                  }`}>
                    {capacityPercentage}%
                  </span>
                </div>
                <div className="w-full bg-secondary rounded-full h-2">
                  <div
                    className={`h-2 rounded-full transition-all ${
                      capacityPercentage > 100 ? 'bg-destructive' :
                      capacityPercentage > 80 ? 'bg-yellow-600' :
                      'bg-green-600'
                    }`}
                    style={{ width: `${Math.min(capacityPercentage, 100)}%` }}
                  />
                </div>
              </div>
            </>
          )}
        </div>

        {/* Drag and Drop Area */}
        {selectedSprint && (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 md:gap-6">
            {/* Backlog */}
            <div>
              <h3 className="text-lg md:text-xl font-bold mb-4">Backlog ({backlogTickets.length})</h3>
              <div className="bg-card rounded-lg border-border border p-4 min-h-[400px]">
                <p className="text-sm text-muted-foreground mb-4">
                  Click tickets to add to sprint →
                </p>
                <div className="space-y-2">
                  {backlogTickets.map(ticket => (
                    <div
                      key={ticket.id}
                      onClick={() => addTicketToSprint(ticket.id)}
                      className="p-3 md:p-4 bg-secondary rounded border-border border hover:border-primary cursor-pointer transition-colors"
                    >
                      <div className="flex items-center justify-between">
                        <div className="flex-1 min-w-0">
                          <div className="font-medium text-sm">{ticket.number}</div>
                          <div className="text-sm text-muted-foreground truncate">
                            {ticket.title}
                          </div>
                        </div>
                        {ticket.story_points && (
                          <div className="ml-4 px-2 py-1 bg-primary/10 text-primary rounded text-sm font-medium flex-shrink-0">
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
              <h3 className="text-lg md:text-xl font-bold mb-4">{selectedSprint.name} ({sprintTickets.length})</h3>
              <div className="bg-card rounded-lg border-border border p-4 min-h-[400px]">
                <p className="text-sm text-muted-foreground mb-4">
                  ← Click tickets to remove from sprint
                </p>
                <div className="space-y-2">
                  {sprintTickets.map(ticket => (
                    <div
                      key={ticket.id}
                      onClick={() => removeTicketFromSprint(ticket.id)}
                      className="p-3 md:p-4 bg-primary/5 rounded border-primary/20 border hover:border-destructive cursor-pointer transition-colors"
                    >
                      <div className="flex items-center justify-between">
                        <div className="flex-1 min-w-0">
                          <div className="font-medium text-sm">{ticket.number}</div>
                          <div className="text-sm text-muted-foreground truncate">
                            {ticket.title}
                          </div>
                        </div>
                        {ticket.story_points && (
                          <div className="ml-4 px-2 py-1 bg-primary/10 text-primary rounded text-sm font-medium flex-shrink-0">
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
        )}
      </div>

      <CreateSprintDialog
        isOpen={showCreateDialog}
        onClose={() => setShowCreateDialog(false)}
        onSuccess={fetchSprints}
      />
    </>
  )
}

export default withAuth(SprintPlanningContent)
