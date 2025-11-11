'use client'

import { useState, useEffect } from 'react'
import { useRouter } from 'next/navigation'
import { api } from '@/lib/api'
import { useProject } from '@/lib/project-context'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { MarkdownPreview } from '@/components/ui/markdown-preview'
import { Textarea } from '@/components/ui/textarea'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import type { TicketStatus, Epic } from '@/lib/types'

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
  const [epics, setEpics] = useState<Epic[]>([])
  const [epicId, setEpicId] = useState<string>('none')
  const [formData, setFormData] = useState({
    title: '',
    description: '',
    status: 'backlog' as TicketStatus,
    story_points: undefined as number | undefined,
    project_id: currentProject?.id || '',
  })

  // Update project_id when currentProject changes
  useEffect(() => {
    if (currentProject) {
      setFormData(prev => ({ ...prev, project_id: currentProject.id }))
    }
  }, [currentProject])

  // Fetch epics when project changes
  useEffect(() => {
    const fetchEpics = async () => {
      if (!currentProject?.id) return
      try {
        const epicsList = await api.listEpics(currentProject.id)
        setEpics(epicsList)
      } catch (error) {
        console.error('Failed to fetch epics:', error)
      }
    }
    fetchEpics()
  }, [currentProject])

  // Reset form when dialog opens
  useEffect(() => {
    if (open) {
      setFormData({
        title: '',
        description: '',
        status: 'backlog',
        story_points: undefined,
        project_id: currentProject?.id || '',
      })
      setEpicId('none')
      setError(null)
      setActiveTab('write')
    }
  }, [open, currentProject])

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setLoading(true)
    setError(null)

    try {
      await api.createTicket({
        ...formData,
        epic_id: epicId === 'none' ? undefined : epicId,
      })

      // Reset form state after successful creation
      setFormData({
        title: '',
        description: '',
        status: 'backlog',
        story_points: undefined,
        project_id: currentProject?.id || '',
      })
      setEpicId('none')

      onCreated?.()
      onClose()
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

          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium mb-1">Story Points</label>
              <input
                type="number"
                min="0"
                step="1"
                value={formData.story_points ?? ''}
                onChange={(e) => setFormData({ ...formData, story_points: e.target.value ? Number(e.target.value) : undefined })}
                className="w-full px-3 py-2 border rounded-md bg-background"
                placeholder="Optional"
              />
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
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">Epic</label>
            <Select value={epicId} onValueChange={setEpicId}>
              <SelectTrigger className="w-full">
                <SelectValue placeholder="None" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="none">None</SelectItem>
                {epics.map((epic) => (
                  <SelectItem key={epic.id} value={epic.id}>
                    <div className="flex items-center gap-2">
                      {epic.epic_color && (
                        <div
                          className="w-3 h-3 rounded-full flex-shrink-0"
                          style={{ backgroundColor: epic.epic_color }}
                        />
                      )}
                      <span>JIL-{epic.number}: {epic.title}</span>
                    </div>
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
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
