'use client'

import { useState } from 'react'
import { X } from 'lucide-react'
import type { Ticket } from '@/lib/types'

interface CompleteSprintDialogProps {
  isOpen: boolean
  onClose: () => void
  onConfirm: (action: 'rollover' | 'backlog' | 'keep') => Promise<void>
  incompleteTickets: Ticket[]
  sprintName: string
}

export function CompleteSprintDialog({
  isOpen,
  onClose,
  onConfirm,
  incompleteTickets,
  sprintName,
}: CompleteSprintDialogProps) {
  const [selectedAction, setSelectedAction] = useState<'rollover' | 'backlog' | 'keep'>('rollover')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')

  if (!isOpen) return null

  const handleConfirm = async () => {
    setLoading(true)
    setError('')

    try {
      await onConfirm(selectedAction)
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to complete sprint')
    } finally {
      setLoading(false)
    }
  }

  const incompleteCount = incompleteTickets.length
  const incompletePoints = incompleteTickets.reduce((sum, t) => sum + (t.story_points || 0), 0)

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4">
      <div className="bg-card border-border rounded-lg border p-6 w-full max-w-lg">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-bold">Complete Sprint: {sprintName}</h2>
          <button
            onClick={onClose}
            className="text-muted-foreground hover:text-foreground"
            disabled={loading}
          >
            <X className="h-5 w-5" />
          </button>
        </div>

        {incompleteCount > 0 ? (
          <>
            <div className="mb-6">
              <p className="text-sm text-muted-foreground mb-4">
                This sprint has <span className="font-semibold text-foreground">{incompleteCount} incomplete tickets</span> ({incompletePoints} points).
                What would you like to do with them?
              </p>

              <div className="space-y-3">
                {/* Option 1: Roll to next sprint */}
                <label className="flex items-start gap-3 p-3 rounded-lg border border-border cursor-pointer hover:bg-accent/50 transition-colors">
                  <input
                    type="radio"
                    name="action"
                    value="rollover"
                    checked={selectedAction === 'rollover'}
                    onChange={(e) => setSelectedAction(e.target.value as 'rollover')}
                    className="mt-1"
                  />
                  <div className="flex-1">
                    <div className="font-medium">Roll over to next sprint</div>
                    <div className="text-sm text-muted-foreground">
                      Create a new sprint and move incomplete tickets to it
                    </div>
                  </div>
                </label>

                {/* Option 2: Return to backlog */}
                <label className="flex items-start gap-3 p-3 rounded-lg border border-border cursor-pointer hover:bg-accent/50 transition-colors">
                  <input
                    type="radio"
                    name="action"
                    value="backlog"
                    checked={selectedAction === 'backlog'}
                    onChange={(e) => setSelectedAction(e.target.value as 'backlog')}
                    className="mt-1"
                  />
                  <div className="flex-1">
                    <div className="font-medium">Return to backlog</div>
                    <div className="text-sm text-muted-foreground">
                      Remove tickets from sprint and move to backlog status
                    </div>
                  </div>
                </label>

                {/* Option 3: Keep in sprint */}
                <label className="flex items-start gap-3 p-3 rounded-lg border border-border cursor-pointer hover:bg-accent/50 transition-colors">
                  <input
                    type="radio"
                    name="action"
                    value="keep"
                    checked={selectedAction === 'keep'}
                    onChange={(e) => setSelectedAction(e.target.value as 'keep')}
                    className="mt-1"
                  />
                  <div className="flex-1">
                    <div className="font-medium">Keep in this sprint</div>
                    <div className="text-sm text-muted-foreground">
                      Mark sprint as complete but leave incomplete tickets as-is
                    </div>
                  </div>
                </label>
              </div>
            </div>

            {error && (
              <div className="mb-4 text-sm text-destructive">{error}</div>
            )}

            <div className="flex gap-2 justify-end">
              <button
                type="button"
                onClick={onClose}
                disabled={loading}
                className="px-4 py-2 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 disabled:opacity-50"
              >
                Cancel
              </button>
              <button
                onClick={handleConfirm}
                disabled={loading}
                className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:opacity-90 disabled:opacity-50"
              >
                {loading ? 'Completing...' : 'Complete Sprint'}
              </button>
            </div>
          </>
        ) : (
          <>
            <p className="text-sm text-muted-foreground mb-6">
              All tickets in this sprint are complete. Ready to finish?
            </p>

            {error && (
              <div className="mb-4 text-sm text-destructive">{error}</div>
            )}

            <div className="flex gap-2 justify-end">
              <button
                type="button"
                onClick={onClose}
                disabled={loading}
                className="px-4 py-2 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 disabled:opacity-50"
              >
                Cancel
              </button>
              <button
                onClick={handleConfirm}
                disabled={loading}
                className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:opacity-90 disabled:opacity-50"
              >
                {loading ? 'Completing...' : 'Complete Sprint'}
              </button>
            </div>
          </>
        )}
      </div>
    </div>
  )
}
