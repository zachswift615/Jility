'use client'

import { useState, useEffect } from 'react'
import { useRouter } from 'next/navigation'
import { api } from '@/lib/api'
import { useProject } from '@/lib/project-context'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { MarkdownPreview } from '@/components/ui/markdown-preview'
import { Textarea } from '@/components/ui/textarea'
import type { TicketStatus } from '@/lib/types'

interface CreateTicketDialogProps {
  open: boolean
  onClose: () => void
  onCreated?: () => void
}

export function CreateTicketDialog({ open, onClose, onCreated }: CreateTicketDialogProps) {
  const router = useRouter()
  const { currentProject } = useProject()
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [activeTab, setActiveTab] = useState<'write' | 'preview'>('write')
  const [formData, setFormData] = useState({
    title: '',
    description: '',
    status: 'backlog' as TicketStatus,
    project_id: currentProject?.id || '',
  })

  // Update project_id when currentProject changes
  useEffect(() => {
    if (currentProject) {
      setFormData(prev => ({ ...prev, project_id: currentProject.id }))
    }
  }, [currentProject])

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setLoading(true)
    setError(null)

    try {
      await api.createTicket(formData)
      onCreated?.()
      onClose()
      setFormData({ title: '', description: '', status: 'backlog', project_id: '' })
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create ticket')
    } finally {
      setLoading(false)
    }
  }

  if (!open) return null

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-background border rounded-lg p-6 w-full max-w-2xl">
        <h2 className="text-2xl font-bold mb-4">Create New Ticket</h2>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium mb-1">Title</label>
            <input
              type="text"
              value={formData.title}
              onChange={(e) => setFormData({ ...formData, title: e.target.value })}
              className="w-full px-3 py-2 border rounded-md bg-background"
              required
              autoFocus
            />
          </div>

          <div>
            <label className="block text-sm font-medium mb-2">Description</label>
            <Tabs value={activeTab} onValueChange={(v) => setActiveTab(v as 'write' | 'preview')}>
              <TabsList className="grid w-full max-w-[400px] grid-cols-2">
                <TabsTrigger value="write">Write</TabsTrigger>
                <TabsTrigger value="preview">Preview</TabsTrigger>
              </TabsList>

              <TabsContent value="write" className="mt-4">
                <Textarea
                  value={formData.description}
                  onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                  className="min-h-[200px] font-mono text-sm"
                  placeholder="Describe the ticket (Markdown supported)..."
                />
              </TabsContent>

              <TabsContent value="preview" className="mt-4">
                <div className="min-h-[200px] p-4 bg-muted rounded-lg border border-border">
                  <MarkdownPreview content={formData.description} />
                </div>
              </TabsContent>
            </Tabs>
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">Status</label>
            <select
              value={formData.status}
              onChange={(e) => setFormData({ ...formData, status: e.target.value as TicketStatus })}
              className="w-full px-3 py-2 border rounded-md bg-background"
            >
              <option value="backlog">Backlog</option>
              <option value="todo">Todo</option>
              <option value="in_progress">In Progress</option>
              <option value="review">Review</option>
              <option value="done">Done</option>
              <option value="blocked">Blocked</option>
            </select>
          </div>

          {error && (
            <div className="text-red-500 text-sm">{error}</div>
          )}

          <div className="flex gap-2 justify-end">
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 border rounded-md hover:bg-accent"
              disabled={loading}
            >
              Cancel
            </button>
            <button
              type="submit"
              className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
              disabled={loading}
            >
              {loading ? 'Creating...' : 'Create Ticket'}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}
