'use client'

import { useState, useEffect } from 'react'
import { useParams } from 'next/navigation'
import { EpicCard } from '@/components/epic-card'
import { MobileFAB } from '@/components/layout/mobile-fab'
import { CreateEpicDialog } from '@/components/epic/create-epic-dialog'
import { Button } from '@/components/ui/button'
import { withAuth } from '@/lib/with-auth'
import { useProject } from '@/lib/project-context'
import type { Epic } from '@/lib/types'
import { Layers, Plus } from 'lucide-react'

function EpicsPage() {
  const params = useParams()
  const slug = params.slug as string
  const { currentProject } = useProject()
  const [epics, setEpics] = useState<Epic[]>([])
  const [loading, setLoading] = useState(true)
  const [isCreateOpen, setIsCreateOpen] = useState(false)

  useEffect(() => {
    if (!currentProject?.id) return

    const fetchEpics = async () => {
      try {
        setLoading(true)
        const response = await fetch(
          `/api/epics?project_id=${currentProject.id}`,
          {
            credentials: 'include',
          }
        )

        if (response.ok) {
          const data = await response.json()
          setEpics(data)
        }
      } catch (error) {
        console.error('Failed to fetch epics:', error)
      } finally {
        setLoading(false)
      }
    }

    fetchEpics()
  }, [currentProject?.id])

  const handleEpicCreated = (newEpic: Epic) => {
    setEpics(prev => [newEpic, ...prev])
  }

  if (loading) {
    return (
      <div className="container mx-auto px-3 md:px-6 py-6">
        <div className="flex items-center justify-center h-64">
          <p className="text-muted-foreground">Loading epics...</p>
        </div>
      </div>
    )
  }

  return (
    <>
      <div className="container mx-auto px-3 md:px-6 py-6">
        {/* Header */}
        <div className="mb-6 flex items-start justify-between">
          <div>
            <h1 className="text-2xl md:text-3xl font-bold mb-2">Epics</h1>
            <p className="text-sm md:text-base text-muted-foreground">
              Track progress across large features and initiatives
            </p>
          </div>
          <Button
            onClick={() => setIsCreateOpen(true)}
            className="hidden md:flex"
          >
            <Plus className="h-4 w-4 mr-2" />
            Create Epic
          </Button>
        </div>

        {/* Epics Grid */}
        {epics.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-12 px-4">
            <div className="rounded-full bg-muted p-6 mb-4">
              <Layers className="h-12 w-12 text-muted-foreground" />
            </div>
            <h2 className="text-xl font-semibold mb-2">No epics yet</h2>
            <p className="text-muted-foreground text-center max-w-md">
              Create your first epic to organize work into larger features and track progress across multiple tickets.
            </p>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 md:gap-6">
            {epics.map((epic) => (
              <EpicCard key={epic.id} epic={epic} />
            ))}
          </div>
        )}
      </div>

      <MobileFAB onClick={() => setIsCreateOpen(true)} />

      <CreateEpicDialog
        isOpen={isCreateOpen}
        onClose={() => setIsCreateOpen(false)}
        onEpicCreated={handleEpicCreated}
        projectId={currentProject?.id || ''}
      />
    </>
  )
}

export default withAuth(EpicsPage)
