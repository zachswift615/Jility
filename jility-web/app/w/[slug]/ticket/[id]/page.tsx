'use client'

import { useEffect, useState } from 'react'
import { useParams, useRouter } from 'next/navigation'
import { api } from '@/lib/api'
import type { TicketDetails } from '@/lib/types'
import { TicketHeader } from '@/components/ticket/ticket-header'
import { TicketDescription } from '@/components/ticket/ticket-description'
import { CommentsSection } from '@/components/ticket/comments-section'
import { ActivityTimeline } from '@/components/ticket/activity-timeline'
import { Button } from '@/components/ui/button'
import { ArrowLeft } from 'lucide-react'

export default function TicketPage() {
  const params = useParams()
  const router = useRouter()
  const ticketId = params.id as string

  const [ticketDetails, setTicketDetails] = useState<TicketDetails | null>(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    loadTicket()
  }, [ticketId])

  const loadTicket = async () => {
    try {
      const data = await api.getTicket(ticketId)
      setTicketDetails(data)
    } catch (error) {
      console.error('Failed to load ticket:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleUpdateTitle = async (title: string) => {
    try {
      await api.updateTicket(ticketId, { title })
      await loadTicket()
    } catch (error) {
      console.error('Failed to update title:', error)
    }
  }

  const handleUpdateDescription = async (description: string) => {
    try {
      await api.updateDescription(ticketId, description)
      await loadTicket()
    } catch (error) {
      console.error('Failed to update description:', error)
    }
  }

  const handleAddComment = async (content: string) => {
    try {
      await api.createComment(ticketId, content)
      await loadTicket()
    } catch (error) {
      console.error('Failed to add comment:', error)
    }
  }

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-muted-foreground">Loading ticket...</div>
      </div>
    )
  }

  if (!ticketDetails) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-muted-foreground">Ticket not found</div>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-3 md:px-6 py-4 md:py-8 max-w-5xl">
      {/* Mobile back button */}
      <button
        onClick={() => router.back()}
        className="flex md:hidden items-center gap-2 text-sm text-muted-foreground mb-4"
      >
        <ArrowLeft className="h-4 w-4" />
        Back
      </button>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <div className="lg:col-span-2 space-y-8">
          <TicketHeader
            ticket={ticketDetails.ticket}
            onUpdateTitle={handleUpdateTitle}
          />

          <TicketDescription
            description={ticketDetails.ticket.description}
            onUpdate={handleUpdateDescription}
          />

          <CommentsSection
            comments={ticketDetails.comments}
            onAddComment={handleAddComment}
          />
        </div>

        <div className="space-y-8">
          <ActivityTimeline changes={ticketDetails.recent_changes} />

          {ticketDetails.linked_commits.length > 0 && (
            <div className="space-y-2">
              <h3 className="text-sm font-semibold">Linked Commits</h3>
              <div className="space-y-2">
                {ticketDetails.linked_commits.map((commit) => (
                  <div
                    key={commit.id}
                    className="text-sm p-2 rounded-md bg-muted"
                  >
                    <div className="font-mono text-xs text-muted-foreground mb-1">
                      {commit.commit_hash.slice(0, 7)}
                    </div>
                    <div>{commit.commit_message}</div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {(ticketDetails.dependencies.length > 0 ||
            ticketDetails.dependents.length > 0) && (
            <div className="space-y-4">
              {ticketDetails.dependencies.length > 0 && (
                <div className="space-y-2">
                  <h3 className="text-sm font-semibold">Depends On</h3>
                  <div className="space-y-1">
                    {ticketDetails.dependencies.map((dep) => (
                      <Button
                        key={dep.id}
                        variant="outline"
                        size="sm"
                        className="w-full justify-start"
                        onClick={() => router.push(`/ticket/${dep.id}`)}
                      >
                        <span className="font-mono text-xs mr-2">
                          {dep.number}
                        </span>
                        {dep.title}
                      </Button>
                    ))}
                  </div>
                </div>
              )}

              {ticketDetails.dependents.length > 0 && (
                <div className="space-y-2">
                  <h3 className="text-sm font-semibold">Blocking</h3>
                  <div className="space-y-1">
                    {ticketDetails.dependents.map((dep) => (
                      <Button
                        key={dep.id}
                        variant="outline"
                        size="sm"
                        className="w-full justify-start"
                        onClick={() => router.push(`/ticket/${dep.id}`)}
                      >
                        <span className="font-mono text-xs mr-2">
                          {dep.number}
                        </span>
                        {dep.title}
                      </Button>
                    ))}
                  </div>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
