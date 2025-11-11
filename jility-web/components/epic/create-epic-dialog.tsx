'use client'

import { useState } from 'react'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Textarea } from '@/components/ui/textarea'
import { api } from '@/lib/api'
import { Epic } from '@/lib/types'

interface CreateEpicDialogProps {
  isOpen: boolean
  onClose: () => void
  onEpicCreated: (epic: Epic) => void
  projectId: string
}

const PRESET_COLORS = [
  { name: 'Blue', value: '#3b82f6' },
  { name: 'Green', value: '#10b981' },
  { name: 'Orange', value: '#f59e0b' },
  { name: 'Red', value: '#ef4444' },
  { name: 'Purple', value: '#8b5cf6' },
  { name: 'Pink', value: '#ec4899' },
  { name: 'Teal', value: '#14b8a6' },
  { name: 'Indigo', value: '#6366f1' },
]

export function CreateEpicDialog({ isOpen, onClose, onEpicCreated, projectId }: CreateEpicDialogProps) {
  const [title, setTitle] = useState('')
  const [description, setDescription] = useState('')
  const [selectedColor, setSelectedColor] = useState(PRESET_COLORS[0].value)
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleSubmit = async () => {
    if (!title.trim()) {
      setError('Title is required')
      return
    }

    setIsSubmitting(true)
    setError(null)

    try {
      const ticket = await api.createTicket({
        title,
        description: description || '',
        is_epic: true,
        epic_color: selectedColor,
        status: 'backlog',
        project_id: projectId,
      })

      // Build Epic object with progress (new epic has no tickets yet)
      const epic: Epic = {
        ...ticket,
        is_epic: true,
        progress: {
          total: 0,
          done: 0,
          in_progress: 0,
          todo: 0,
          blocked: 0,
          completion_percentage: 0,
        },
      }

      // Reset form
      setTitle('')
      setDescription('')
      setSelectedColor(PRESET_COLORS[0].value)

      onEpicCreated(epic)
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create epic')
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="sm:max-w-[500px]">
        <DialogHeader>
          <DialogTitle>Create Epic</DialogTitle>
        </DialogHeader>

        <div className="space-y-4">
          <div>
            <Label htmlFor="title">Title *</Label>
            <Input
              id="title"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              placeholder="e.g., User Authentication System"
            />
          </div>

          <div>
            <Label htmlFor="description">Description</Label>
            <Textarea
              id="description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="Describe the epic's goals and scope..."
              rows={4}
            />
          </div>

          <div>
            <Label>Epic Color</Label>
            <div className="grid grid-cols-4 gap-2 mt-2">
              {PRESET_COLORS.map(color => (
                <button
                  key={color.value}
                  type="button"
                  onClick={() => setSelectedColor(color.value)}
                  className={`
                    h-10 rounded-md border-2 transition-all
                    ${selectedColor === color.value ? 'border-foreground scale-110' : 'border-border'}
                  `}
                  style={{ backgroundColor: color.value }}
                  title={color.name}
                />
              ))}
            </div>
          </div>

          {error && (
            <div className="text-sm text-destructive">
              {error}
            </div>
          )}

          <div className="flex justify-end gap-2">
            <Button variant="outline" onClick={onClose}>
              Cancel
            </Button>
            <Button onClick={handleSubmit} disabled={isSubmitting}>
              {isSubmitting ? 'Creating...' : 'Create Epic'}
            </Button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  )
}
