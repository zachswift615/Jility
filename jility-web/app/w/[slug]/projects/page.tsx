'use client'

import { useEffect, useState } from 'react'
import { api } from '@/lib/api'
import type { Project } from '@/lib/types'
import { Button } from '@/components/ui/button'
import { Copy, Check, Settings, Plus } from 'lucide-react'
import { withAuth } from '@/lib/with-auth'

function ProjectsPage() {
  const [projects, setProjects] = useState<Project[]>([])
  const [isLoading, setIsLoading] = useState(true)
  const [copiedId, setCopiedId] = useState<string | null>(null)

  useEffect(() => {
    loadProjects()
  }, [])

  const loadProjects = async () => {
    try {
      setIsLoading(true)
      const data = await api.listProjects()
      setProjects(data)
    } catch (error) {
      console.error('Failed to load projects:', error)
    } finally {
      setIsLoading(false)
    }
  }

  const handleCopyProjectId = async (projectId: string) => {
    await navigator.clipboard.writeText(projectId)
    setCopiedId(projectId)
    setTimeout(() => setCopiedId(null), 2000)
  }

  return (
    <div className="container max-w-6xl py-8">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-3xl font-bold">Projects</h1>
          <p className="text-muted-foreground mt-1">
            Manage your projects and view their configuration details
          </p>
        </div>
        <Button disabled>
          <Plus className="h-4 w-4 mr-2" />
          New Project
        </Button>
      </div>

      {isLoading ? (
        <div className="bg-card border border-border rounded-lg p-8 text-center">
          <div className="text-muted-foreground">Loading projects...</div>
        </div>
      ) : projects.length === 0 ? (
        <div className="bg-card border border-border rounded-lg p-8 text-center">
          <div className="text-muted-foreground">No projects found.</div>
        </div>
      ) : (
        <div className="space-y-4">
          {projects.map((project) => (
            <div
              key={project.id}
              className="bg-card border border-border rounded-lg p-6 hover:border-primary/50 transition-colors"
            >
              <div className="flex items-start justify-between mb-4">
                <div>
                  <h2 className="text-xl font-semibold">{project.name}</h2>
                  {project.description && (
                    <p className="text-sm text-muted-foreground mt-1">
                      {project.description}
                    </p>
                  )}
                </div>
                <Button variant="outline" size="sm" disabled>
                  <Settings className="h-4 w-4 mr-2" />
                  Settings
                </Button>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {/* Project ID */}
                <div>
                  <label className="text-xs font-medium text-muted-foreground uppercase tracking-wide">
                    Project ID (UUID)
                  </label>
                  <div className="mt-2 flex items-center gap-2">
                    <code className="flex-1 bg-muted px-3 py-2 rounded text-xs font-mono break-all">
                      {project.id}
                    </code>
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => handleCopyProjectId(project.id)}
                      className="shrink-0"
                    >
                      {copiedId === project.id ? (
                        <Check className="h-4 w-4" />
                      ) : (
                        <Copy className="h-4 w-4" />
                      )}
                    </Button>
                  </div>
                </div>

                {/* Created Date */}
                <div>
                  <label className="text-xs font-medium text-muted-foreground uppercase tracking-wide">
                    Created
                  </label>
                  <div className="mt-2 text-sm">
                    {new Date(project.created_at).toLocaleDateString('en-US', {
                      year: 'numeric',
                      month: 'long',
                      day: 'numeric',
                    })}
                  </div>
                </div>
              </div>

              {/* MCP Configuration Hint */}
              <div className="mt-4 bg-muted/50 rounded px-3 py-2 text-xs text-muted-foreground">
                💡 Use this UUID in your <code className="bg-background px-1 py-0.5 rounded">.mcp.json</code> file under{' '}
                <code className="bg-background px-1 py-0.5 rounded">JILITY_PROJECT_ID</code>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Info Section */}
      <div className="mt-8 bg-muted border border-border rounded-lg p-6">
        <h3 className="font-semibold mb-2">About Projects</h3>
        <ul className="text-sm text-muted-foreground space-y-2">
          <li>• Projects organize your tickets, sprints, and team collaboration</li>
          <li>• Each workspace is associated with a project</li>
          <li>• Use the Project ID (UUID) when configuring Claude Code's MCP integration</li>
          <li>• Project settings and creation features coming soon</li>
        </ul>
      </div>
    </div>
  )
}

export default withAuth(ProjectsPage)
