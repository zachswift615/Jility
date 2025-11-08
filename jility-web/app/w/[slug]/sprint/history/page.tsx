'use client'

import { useState, useEffect, useCallback } from 'react'
import { Calendar, TrendingUp } from 'lucide-react'
import { withAuth } from '@/lib/with-auth'
import { useWorkspace } from '@/lib/workspace-context'
import { api } from '@/lib/api'
import type { SprintHistory } from '@/lib/types'

function SprintHistoryContent() {
  const [history, setHistory] = useState<SprintHistory | null>(null)
  const [loading, setLoading] = useState(true)
  const { currentWorkspace } = useWorkspace()
  const slug = currentWorkspace?.slug || ''

  const fetchHistory = useCallback(async () => {
    if (!slug) return
    try {
      const data = await api.getSprintHistory(slug)
      setHistory(data)
    } catch (error) {
      console.error('Failed to fetch sprint history:', error)
    } finally {
      setLoading(false)
    }
  }, [slug])

  useEffect(() => {
    fetchHistory()
  }, [fetchHistory])

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-muted-foreground">Loading...</div>
      </div>
    )
  }

  if (!history || history.sprints.length === 0) {
    return (
      <div className="container mx-auto px-3 md:px-6 py-4 md:py-8">
        <div className="text-center py-12">
          <h1 className="text-2xl md:text-3xl font-bold mb-4">No Sprint History</h1>
          <p className="text-muted-foreground">
            Complete some sprints to see your team's velocity and performance.
          </p>
        </div>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-3 md:px-6 py-4 md:py-8">
      {/* Header */}
      <div className="mb-6 md:mb-8">
        <h1 className="text-2xl md:text-3xl font-bold mb-2">Sprint History</h1>
        <p className="text-muted-foreground">
          View completed sprints and team velocity over time
        </p>
      </div>

      {/* Velocity Summary */}
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-6 md:mb-8">
        <div className="bg-card border-border border rounded-lg p-4 md:p-6">
          <div className="flex items-center gap-2 text-muted-foreground mb-2">
            <TrendingUp className="h-5 w-5" />
            <span className="font-medium">Average Velocity</span>
          </div>
          <div className="text-3xl md:text-4xl font-bold">
            {Math.round(history.average_velocity)}
          </div>
          <div className="text-sm text-muted-foreground mt-1">
            points per sprint
          </div>
        </div>

        <div className="bg-card border-border border rounded-lg p-4 md:p-6">
          <div className="flex items-center gap-2 text-muted-foreground mb-2">
            <Calendar className="h-5 w-5" />
            <span className="font-medium">Completed Sprints</span>
          </div>
          <div className="text-3xl md:text-4xl font-bold">
            {history.sprints.length}
          </div>
          <div className="text-sm text-muted-foreground mt-1">
            total sprints
          </div>
        </div>
      </div>

      {/* Velocity Chart (Simple Bar Chart) */}
      <div className="bg-card border-border border rounded-lg p-4 md:p-6 mb-6 md:mb-8">
        <h3 className="font-semibold mb-4">Velocity Trend</h3>
        <div className="space-y-3">
          {history.velocity_data.slice().reverse().map((sprint, index) => (
            <div key={index}>
              <div className="flex justify-between text-sm mb-1">
                <span className="font-medium truncate pr-2">{sprint.sprint_name}</span>
                <span className="font-medium flex-shrink-0">{sprint.completed_points} pts</span>
              </div>
              <div className="w-full bg-secondary rounded-full h-6">
                <div
                  className="bg-primary h-6 rounded-full transition-all flex items-center px-2"
                  style={{
                    width: `${history.average_velocity > 0
                      ? Math.min((sprint.completed_points / (history.average_velocity * 1.5)) * 100, 100)
                      : 0}%`,
                    minWidth: sprint.completed_points > 0 ? '3rem' : '0'
                  }}
                >
                  {sprint.completed_points > 0 && (
                    <span className="text-xs font-medium text-primary-foreground">
                      {sprint.completed_points}
                    </span>
                  )}
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Completed Sprints List */}
      <div className="bg-card border-border border rounded-lg p-4 md:p-6">
        <h3 className="font-semibold mb-4">Completed Sprints</h3>
        <div className="space-y-3">
          {history.sprints.map((sprint) => {
            const velocity = history.velocity_data.find(v => v.sprint_name === sprint.name)
            return (
              <div
                key={sprint.id}
                className="p-4 bg-secondary rounded border-border border"
              >
                <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2">
                  <div className="flex-1 min-w-0">
                    <h4 className="font-medium">{sprint.name}</h4>
                    {sprint.goal && (
                      <p className="text-sm text-muted-foreground mt-1 line-clamp-2">
                        {sprint.goal}
                      </p>
                    )}
                    {sprint.start_date && sprint.end_date && (
                      <p className="text-xs text-muted-foreground mt-2">
                        {new Date(sprint.start_date).toLocaleDateString()} - {new Date(sprint.end_date).toLocaleDateString()}
                      </p>
                    )}
                  </div>
                  <div className="flex items-center gap-4 flex-shrink-0">
                    {velocity && (
                      <div className="text-right">
                        <div className="text-sm text-muted-foreground">Velocity</div>
                        <div className="text-lg font-bold">{velocity.completed_points} pts</div>
                      </div>
                    )}
                  </div>
                </div>
              </div>
            )
          })}
        </div>
      </div>
    </div>
  )
}

export default withAuth(SprintHistoryContent)
