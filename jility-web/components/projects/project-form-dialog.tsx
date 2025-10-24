'use client'

import React, { useState, useEffect } from 'react'
import { useProject } from '@/lib/project-context'
import { Project } from '@/lib/types'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Textarea } from '@/components/ui/textarea'
import { Switch } from '@/components/ui/switch'
import { Loader2 } from 'lucide-react'

interface ProjectFormDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  project?: Project // If provided, edit mode; otherwise create mode
}

const PROJECT_COLORS = [
  { name: 'Purple', value: '#5e6ad2' },
  { name: 'Orange', value: '#f2994a' },
  { name: 'Green', value: '#27ae60' },
  { name: 'Red', value: '#eb5757' },
  { name: 'Violet', value: '#9b59b6' },
  { name: 'Blue', value: '#3498db' },
]

export function ProjectFormDialog({ open, onOpenChange, project }: ProjectFormDialogProps) {
  const { createProject, updateProject } = useProject()
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Form state
  const [name, setName] = useState('')
  const [description, setDescription] = useState('')
  const [key, setKey] = useState('')
  const [color, setColor] = useState(PROJECT_COLORS[0].value)
  const [aiPlanningEnabled, setAiPlanningEnabled] = useState(false)
  const [autoLinkGit, setAutoLinkGit] = useState(false)
  const [requireStoryPoints, setRequireStoryPoints] = useState(false)

  // Load project data when in edit mode
  useEffect(() => {
    if (project) {
      setName(project.name)
      setDescription(project.description || '')
      setKey(project.key || '')
      setColor(project.color || PROJECT_COLORS[0].value)
      setAiPlanningEnabled(project.ai_planning_enabled)
      setAutoLinkGit(project.auto_link_git)
      setRequireStoryPoints(project.require_story_points)
    } else {
      // Reset form when switching to create mode
      setName('')
      setDescription('')
      setKey('')
      setColor(PROJECT_COLORS[0].value)
      setAiPlanningEnabled(false)
      setAutoLinkGit(false)
      setRequireStoryPoints(false)
    }
    setError(null)
  }, [project, open])

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)

    if (!name.trim()) {
      setError('Project name is required')
      return
    }

    setIsSubmitting(true)

    try {
      const data = {
        name: name.trim(),
        description: description.trim() || undefined,
        key: key.trim() || undefined,
        color,
        ai_planning_enabled: aiPlanningEnabled,
        auto_link_git: autoLinkGit,
        require_story_points: requireStoryPoints,
      }

      if (project) {
        await updateProject(project.id, data)
      } else {
        await createProject(data)
      }

      onOpenChange(false)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to save project')
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>{project ? 'Edit Project' : 'Create New Project'}</DialogTitle>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-4">
          {/* Project Name */}
          <div className="space-y-2">
            <Label htmlFor="name">Project Name *</Label>
            <Input
              id="name"
              placeholder="e.g., Mobile App Redesign"
              value={name}
              onChange={e => setName(e.target.value)}
              required
            />
          </div>

          {/* Project Key */}
          <div className="space-y-2">
            <Label htmlFor="key">Project Key</Label>
            <Input
              id="key"
              placeholder="e.g., PROJ (max 10 chars)"
              value={key}
              onChange={e => setKey(e.target.value.toUpperCase().slice(0, 10))}
              maxLength={10}
            />
            <p className="text-xs text-muted-foreground">
              Short prefix for ticket IDs (e.g., PROJ-123)
            </p>
          </div>

          {/* Description */}
          <div className="space-y-2">
            <Label htmlFor="description">Description</Label>
            <Textarea
              id="description"
              placeholder="What is this project about?"
              value={description}
              onChange={e => setDescription(e.target.value)}
              rows={3}
            />
          </div>

          {/* Color Picker */}
          <div className="space-y-2">
            <Label>Project Color</Label>
            <div className="flex gap-2">
              {PROJECT_COLORS.map(colorOption => (
                <button
                  key={colorOption.value}
                  type="button"
                  onClick={() => setColor(colorOption.value)}
                  className={`w-10 h-10 rounded-md border-2 transition-all ${
                    color === colorOption.value
                      ? 'border-primary scale-110'
                      : 'border-transparent hover:scale-105'
                  }`}
                  style={{ backgroundColor: colorOption.value }}
                  title={colorOption.name}
                />
              ))}
            </div>
          </div>

          {/* Feature Toggles */}
          <div className="space-y-3 pt-2">
            <div className="flex items-center justify-between">
              <div className="space-y-0.5">
                <Label htmlFor="ai-planning">Enable AI Planning</Label>
                <p className="text-xs text-muted-foreground">
                  Allow AI agents to create and break down tickets
                </p>
              </div>
              <Switch
                id="ai-planning"
                checked={aiPlanningEnabled}
                onCheckedChange={setAiPlanningEnabled}
              />
            </div>

            <div className="flex items-center justify-between">
              <div className="space-y-0.5">
                <Label htmlFor="auto-link">Auto-link Git Commits</Label>
                <p className="text-xs text-muted-foreground">
                  Automatically link commits that mention ticket IDs
                </p>
              </div>
              <Switch
                id="auto-link"
                checked={autoLinkGit}
                onCheckedChange={setAutoLinkGit}
              />
            </div>

            <div className="flex items-center justify-between">
              <div className="space-y-0.5">
                <Label htmlFor="require-points">Require Story Points</Label>
                <p className="text-xs text-muted-foreground">
                  Don't allow moving tickets to In Progress without estimation
                </p>
              </div>
              <Switch
                id="require-points"
                checked={requireStoryPoints}
                onCheckedChange={setRequireStoryPoints}
              />
            </div>
          </div>

          {/* Error Message */}
          {error && (
            <div className="text-sm text-destructive bg-destructive/10 px-3 py-2 rounded-md">
              {error}
            </div>
          )}

          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
              disabled={isSubmitting}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={isSubmitting}>
              {isSubmitting && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
              {project ? 'Update Project' : 'Create Project'}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
