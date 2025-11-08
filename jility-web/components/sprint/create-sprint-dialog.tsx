'use client'

import { useState } from 'react'
import { X } from 'lucide-react'
import { api } from '@/lib/api'
import { useWorkspace } from '@/lib/workspace-context'

interface CreateSprintDialogProps {
  isOpen: boolean
  onClose: () => void
  onSuccess: () => void
}

export function CreateSprintDialog({ isOpen, onClose, onSuccess }: CreateSprintDialogProps) {
  const [name, setName] = useState('')
  const [goal, setGoal] = useState('')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')
  const { currentWorkspace } = useWorkspace()

  if (!isOpen) return null

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!currentWorkspace) return

    setLoading(true)
    setError('')

    try {
      await api.createSprint(currentWorkspace.slug, {
        name,
        goal: goal || undefined,
      })
      onSuccess()
      setName('')
      setGoal('')
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create sprint')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="bg-card border-border rounded-lg border p-6 w-full max-w-md">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-bold">Create Sprint</h2>
          <button
            onClick={onClose}
            className="text-muted-foreground hover:text-foreground"
          >
            <X className="h-5 w-5" />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label htmlFor="name" className="block text-sm font-medium mb-1">
              Sprint Name *
            </label>
            <input
              id="name"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full px-3 py-2 bg-background border-input border rounded-md"
              placeholder="Sprint 1"
              required
            />
          </div>

          <div>
            <label htmlFor="goal" className="block text-sm font-medium mb-1">
              Sprint Goal (optional)
            </label>
            <textarea
              id="goal"
              value={goal}
              onChange={(e) => setGoal(e.target.value)}
              className="w-full px-3 py-2 bg-background border-input border rounded-md"
              placeholder="What do we want to achieve?"
              rows={3}
            />
          </div>

          {error && (
            <div className="text-sm text-destructive">{error}</div>
          )}

          <div className="flex gap-2 justify-end">
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 bg-secondary text-secondary-foreground rounded-md"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={loading || !name}
              className="px-4 py-2 bg-primary text-primary-foreground rounded-md disabled:opacity-50"
            >
              {loading ? 'Creating...' : 'Create Sprint'}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}
