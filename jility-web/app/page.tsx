'use client'

import { useEffect, useState } from 'react'
import { useRouter } from 'next/navigation'
import { useAuth } from '@/lib/auth-context'
import { CreateWorkspaceDialog } from '@/components/create-workspace-dialog'
import { Button } from '@/components/ui/button'
import { Loader2, Plus } from 'lucide-react'

interface Workspace {
  id: string
  name: string
  slug: string
  role: 'admin' | 'member'
  created_at: string
}

export default function Home() {
  const { isAuthenticated, isLoading } = useAuth()
  const router = useRouter()
  const [fetchingWorkspaces, setFetchingWorkspaces] = useState(false)
  const [noWorkspaces, setNoWorkspaces] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [showCreateDialog, setShowCreateDialog] = useState(false)

  useEffect(() => {
    async function handleRedirect() {
      // Wait for auth to load
      if (isLoading) return

      // Redirect to login if not authenticated
      if (!isAuthenticated) {
        router.push('/login')
        return
      }

      // Fetch workspaces if authenticated
      setFetchingWorkspaces(true)
      try {
        const token = localStorage.getItem('jility_token')
        if (!token) {
          router.push('/login')
          return
        }

        const response = await fetch('/api/workspaces', {
          headers: { Authorization: `Bearer ${token}` },
        })

        if (response.ok) {
          const workspaces: Workspace[] = await response.json()
          if (workspaces.length > 0) {
            router.push(`/w/${workspaces[0].slug}/board`)
            return
          } else {
            // No workspaces - show create workspace UI
            setNoWorkspaces(true)
          }
        } else if (response.status === 401) {
          // Token invalid, redirect to login
          router.push('/login')
        } else {
          // Other error
          setError('Failed to load workspaces')
        }
      } catch (err) {
        console.error('Error fetching workspaces:', err)
        setError('Failed to load workspaces')
      } finally {
        setFetchingWorkspaces(false)
      }
    }

    handleRedirect()
  }, [isAuthenticated, isLoading, router])

  // Show loading state while checking auth or fetching workspaces
  if (isLoading || fetchingWorkspaces) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-center">
          <Loader2 className="h-8 w-8 animate-spin text-muted-foreground mx-auto" />
          <p className="mt-4 text-muted-foreground">Loading...</p>
        </div>
      </div>
    )
  }

  // Show "no workspaces" state
  if (noWorkspaces) {
    return (
      <>
        <div className="flex items-center justify-center min-h-screen">
          <div className="text-center max-w-md px-4">
            <h1 className="text-2xl font-bold mb-2">Welcome to Jility</h1>
            <p className="text-muted-foreground mb-6">
              You don't have any workspaces yet. Create your first workspace to get started.
            </p>
            <Button onClick={() => setShowCreateDialog(true)} size="lg">
              <Plus className="mr-2 h-5 w-5" />
              Create Workspace
            </Button>
          </div>
        </div>

        <CreateWorkspaceDialog
          open={showCreateDialog}
          onOpenChange={setShowCreateDialog}
        />
      </>
    )
  }

  // Show error state
  if (error) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-center max-w-md px-4">
          <h1 className="text-2xl font-bold mb-2 text-destructive">Error</h1>
          <p className="text-muted-foreground mb-6">{error}</p>
          <button
            onClick={() => window.location.reload()}
            className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:opacity-90"
          >
            Retry
          </button>
        </div>
      </div>
    )
  }

  return null
}
