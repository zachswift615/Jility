'use client'

import React, { useState, useMemo } from 'react'
import { useProject } from '@/lib/project-context'
import { Project } from '@/lib/types'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Check, Search, Plus } from 'lucide-react'
import { formatDistanceToNow } from 'date-fns'

interface ProjectSwitcherProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onCreateNew?: () => void
}

export function ProjectSwitcher({ open, onOpenChange, onCreateNew }: ProjectSwitcherProps) {
  const { projects, currentProject, setCurrentProject } = useProject()
  const [searchQuery, setSearchQuery] = useState('')

  // Filter projects based on search query
  const filteredProjects = useMemo(() => {
    if (!searchQuery.trim()) return projects

    const query = searchQuery.toLowerCase()
    return projects.filter(
      p =>
        p.name.toLowerCase().includes(query) ||
        p.description?.toLowerCase().includes(query) ||
        p.key?.toLowerCase().includes(query)
    )
  }, [projects, searchQuery])

  const handleSelectProject = (project: Project) => {
    setCurrentProject(project)
    onOpenChange(false)
    setSearchQuery('') // Reset search on close
  }

  const handleCreateNew = () => {
    onOpenChange(false)
    setSearchQuery('')
    onCreateNew?.()
  }

  // Get project icon (first 2 letters of name)
  const getProjectIcon = (project: Project) => {
    return project.name.slice(0, 2).toUpperCase()
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md p-0">
        <DialogHeader className="px-6 pt-6 pb-4 border-b">
          <DialogTitle>Switch Project</DialogTitle>
        </DialogHeader>

        {/* Search Bar */}
        <div className="px-6 py-4">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
            <Input
              placeholder="Search projects..."
              value={searchQuery}
              onChange={e => setSearchQuery(e.target.value)}
              className="pl-10"
              autoFocus
            />
          </div>
        </div>

        {/* Projects List */}
        <div className="max-h-[400px] overflow-y-auto">
          {filteredProjects.length === 0 ? (
            <div className="px-6 py-8 text-center text-muted-foreground">
              {searchQuery ? 'No projects found' : 'No projects yet'}
            </div>
          ) : (
            <div className="px-2">
              <div className="text-xs font-semibold text-muted-foreground px-4 py-2">
                RECENT PROJECTS
              </div>
              {filteredProjects.map(project => (
                <button
                  key={project.id}
                  onClick={() => handleSelectProject(project)}
                  className="w-full flex items-center gap-3 px-4 py-3 hover:bg-accent rounded-md transition-colors text-left"
                >
                  {/* Project Icon */}
                  <div
                    className="w-8 h-8 rounded flex items-center justify-center text-white font-semibold text-sm flex-shrink-0"
                    style={{
                      backgroundColor: project.color || '#5e6ad2',
                    }}
                  >
                    {getProjectIcon(project)}
                  </div>

                  {/* Project Info */}
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <span className="font-medium truncate">{project.name}</span>
                      {currentProject?.id === project.id && (
                        <Check className="h-4 w-4 text-primary flex-shrink-0" />
                      )}
                    </div>
                    <div className="text-xs text-muted-foreground">
                      Updated {formatDistanceToNow(new Date(project.updated_at), { addSuffix: true })}
                    </div>
                  </div>
                </button>
              ))}
            </div>
          )}
        </div>

        {/* Create New Project Button */}
        <div className="px-4 py-4 border-t">
          <Button
            variant="outline"
            className="w-full justify-start"
            onClick={handleCreateNew}
          >
            <Plus className="h-4 w-4 mr-2" />
            Create New Project
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  )
}
