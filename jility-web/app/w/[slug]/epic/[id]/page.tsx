'use client'

import { useState, useEffect } from 'react'
import { useParams, useRouter } from 'next/navigation'
import { KanbanBoard } from '@/components/kanban/board'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { withAuth } from '@/lib/with-auth'
import { api } from '@/lib/api'
import type { Epic, Ticket } from '@/lib/types'
import { ArrowLeft, Layers } from 'lucide-react'

function EpicDetailPage() {
  const params = useParams()
  const router = useRouter()
  const slug = params.slug as string
  const epicId = params.id as string

  const [epic, setEpic] = useState<Epic | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    const fetchEpic = async () => {
      try {
        setLoading(true)
        setError(null)
        const epicData = await api.getEpic(epicId)
        setEpic(epicData)
      } catch (err) {
        console.error('Failed to fetch epic:', err)
        setError(err instanceof Error ? err.message : 'Failed to load epic')
      } finally {
        setLoading(false)
      }
    }

    if (epicId) {
      fetchEpic()
    }
  }, [epicId])

  // Filter function to show only tickets belonging to this epic
  const filterEpicTickets = (tickets: Ticket[]) => {
    return tickets.filter(ticket => ticket.epic_id === epicId)
  }

  if (loading) {
    return (
      <div className="container mx-auto px-3 md:px-6 py-6">
        <div className="flex items-center justify-center h-64">
          <p className="text-muted-foreground">Loading epic...</p>
        </div>
      </div>
    )
  }

  if (error || !epic) {
    return (
      <div className="container mx-auto px-3 md:px-6 py-6">
        <div className="flex flex-col items-center justify-center h-64 gap-4">
          <p className="text-destructive">{error || 'Epic not found'}</p>
          <Button onClick={() => router.push(`/w/${slug}/epics`)}>
            Back to Epics
          </Button>
        </div>
      </div>
    )
  }

  const { progress } = epic
  const completionPercentage = progress.completion_percentage

  return (
    <div className="flex flex-col h-[calc(100vh-3.5rem-4rem)] md:h-[calc(100vh-4rem)]">
      {/* Epic Header */}
      <div className="px-4 md:px-6 pt-4 pb-2 border-b border-border">
        <div className="flex items-start gap-3 mb-4">
          <Button
            variant="ghost"
            size="sm"
            onClick={() => router.push(`/w/${slug}/epics`)}
            className="mt-1"
          >
            <ArrowLeft className="h-4 w-4" />
          </Button>
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 mb-1">
              <Layers className="h-5 w-5 text-muted-foreground flex-shrink-0" />
              <Badge variant="secondary" className="text-xs">
                {epic.number}
              </Badge>
              {epic.epic_color && (
                <div
                  className="w-3 h-3 rounded-full flex-shrink-0"
                  style={{ backgroundColor: epic.epic_color }}
                  title="Epic color"
                />
              )}
            </div>
            <h1 className="text-xl md:text-2xl font-bold break-words">
              {epic.title}
            </h1>
            {epic.description && (
              <p className="text-sm md:text-base text-muted-foreground mt-2 break-words">
                {epic.description}
              </p>
            )}
          </div>
        </div>

        {/* Progress Visualization */}
        <div className="space-y-3">
          {/* Progress Bar */}
          <div className="space-y-1">
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">Progress</span>
              <span className="font-medium">
                {progress.done} of {progress.total} tasks completed ({completionPercentage}%)
              </span>
            </div>
            <div className="w-full bg-secondary rounded-full h-2.5 overflow-hidden">
              <div
                className="bg-primary h-full transition-all duration-300"
                style={{ width: `${completionPercentage}%` }}
              />
            </div>
          </div>

          {/* Status Breakdown */}
          <div className="flex flex-wrap gap-3 text-sm">
            {progress.done > 0 && (
              <div className="flex items-center gap-1.5">
                <div className="w-3 h-3 rounded-full bg-[hsl(var(--status-done))]" />
                <span className="text-muted-foreground">{progress.done} done</span>
              </div>
            )}
            {progress.in_progress > 0 && (
              <div className="flex items-center gap-1.5">
                <div className="w-3 h-3 rounded-full bg-[hsl(var(--status-in-progress))]" />
                <span className="text-muted-foreground">{progress.in_progress} in progress</span>
              </div>
            )}
            {progress.todo > 0 && (
              <div className="flex items-center gap-1.5">
                <div className="w-3 h-3 rounded-full bg-[hsl(var(--status-todo))]" />
                <span className="text-muted-foreground">{progress.todo} todo</span>
              </div>
            )}
            {progress.blocked > 0 && (
              <div className="flex items-center gap-1.5">
                <div className="w-3 h-3 rounded-full bg-[hsl(var(--status-blocked))]" />
                <span className="text-muted-foreground">{progress.blocked} blocked</span>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Epic Kanban Board */}
      <div className="flex-1 overflow-hidden">
        <KanbanBoard filterFn={filterEpicTickets} />
      </div>
    </div>
  )
}

export default withAuth(EpicDetailPage)
