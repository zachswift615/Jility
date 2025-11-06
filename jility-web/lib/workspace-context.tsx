'use client'

import { createContext, useContext, useState, useEffect, ReactNode } from 'react'
import { usePathname, useRouter } from 'next/navigation'

interface Workspace {
  id: string
  name: string
  slug: string
  role: 'admin' | 'member'
  created_at: string
}

interface WorkspaceContextType {
  currentWorkspace: Workspace | null
  workspaces: Workspace[]
  isLoading: boolean
  switchWorkspace: (slug: string) => void
  refreshWorkspaces: () => Promise<void>
  createWorkspace: (data: { name: string }) => Promise<Workspace>
}

const WorkspaceContext = createContext<WorkspaceContextType | undefined>(undefined)

export function WorkspaceProvider({ children }: { children: ReactNode }) {
  const [workspaces, setWorkspaces] = useState<Workspace[]>([])
  const [currentWorkspace, setCurrentWorkspace] = useState<Workspace | null>(null)
  const [isLoading, setIsLoading] = useState(true)
  const pathname = usePathname()
  const router = useRouter()

  // Extract workspace slug from URL
  const workspaceSlug = pathname?.match(/^\/w\/([^\/]+)/)?.[1]

  // Fetch user's workspaces
  const fetchWorkspaces = async () => {
    try {
      const response = await fetch('/api/workspaces', {
        credentials: 'include',
      })

      if (!response.ok) {
        throw new Error('Failed to fetch workspaces')
      }

      const data = await response.json()
      setWorkspaces(data)

      // Set current workspace based on URL
      if (workspaceSlug) {
        const current = data.find((w: Workspace) => w.slug === workspaceSlug)
        setCurrentWorkspace(current || null)
      }
    } catch (error) {
      console.error('Failed to fetch workspaces:', error)
    } finally {
      setIsLoading(false)
    }
  }

  useEffect(() => {
    fetchWorkspaces()
  }, [workspaceSlug])

  const switchWorkspace = (slug: string) => {
    const workspace = workspaces.find(w => w.slug === slug)
    if (workspace) {
      setCurrentWorkspace(workspace)
      router.push(`/w/${slug}/board`)
    }
  }

  const refreshWorkspaces = async () => {
    setIsLoading(true)
    await fetchWorkspaces()
  }

  const createWorkspace = async (data: { name: string }) => {
    const response = await fetch('/api/workspaces', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      credentials: 'include',
      body: JSON.stringify(data),
    })

    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}))
      throw new Error(errorData.message || 'Failed to create workspace')
    }

    const workspace = await response.json()

    // Refresh workspaces list to include the new one
    await fetchWorkspaces()

    return workspace
  }

  return (
    <WorkspaceContext.Provider
      value={{
        currentWorkspace,
        workspaces,
        isLoading,
        switchWorkspace,
        refreshWorkspaces,
        createWorkspace,
      }}
    >
      {children}
    </WorkspaceContext.Provider>
  )
}

export function useWorkspace() {
  const context = useContext(WorkspaceContext)
  if (context === undefined) {
    throw new Error('useWorkspace must be used within a WorkspaceProvider')
  }
  return context
}
