'use client'

import { useEffect, useState, useCallback } from 'react'
import { useParams, useRouter } from 'next/navigation'
import { api } from '@/lib/api'
import type { TicketDetails, WorkspaceMember, WebSocketMessage, Epic, TicketStatus } from '@/lib/types'
import { TicketHeader } from '@/components/ticket/ticket-header'
import { TicketDescription } from '@/components/ticket/ticket-description'
import { CommentsSection } from '@/components/ticket/comments-section'
import { ActivityTimeline } from '@/components/ticket/activity-timeline'
import { AssigneeSelector } from '@/components/ticket/assignee-selector'
import { StatusSelector } from '@/components/ticket/status-selector'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { useToast } from '@/hooks/use-toast'
import { useWebSocket } from '@/lib/websocket'
import { ArrowLeft, Trash2, Layers, Edit2, X, Check } from 'lucide-react'
import { useAuth } from '@/lib/auth-context'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { getStatusLabel } from '@/lib/utils'
import Link from 'next/link'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog'

export default function TicketPage() {
  const params = useParams()
  const router = useRouter()
  const { user } = useAuth()
  const { toast } = useToast()
  const slug = params.slug as string
  const ticketId = params.id as string

  const [ticketDetails, setTicketDetails] = useState<TicketDetails | null>(null)
  const [loading, setLoading] = useState(true)
  const [members, setMembers] = useState<WorkspaceMember[]>([])
  const [isLoadingMembers, setIsLoadingMembers] = useState(true)
  const [epic, setEpic] = useState<Epic | null>(null)
  const [epics, setEpics] = useState<Epic[]>([])
  const [isEditingEpic, setIsEditingEpic] = useState(false)
  const [selectedEpicId, setSelectedEpicId] = useState<string>('none')

  // WebSocket handler for real-time updates
  const handleWebSocketMessage = useCallback((message: WebSocketMessage) => {
    // Refresh ticket when a comment is added to this ticket
    if (message.type === 'comment_added' && message.ticket_id === ticketId) {
      loadTicket()
    }
  }, [ticketId])

  useWebSocket(handleWebSocketMessage)

  useEffect(() => {
    loadTicket()
  }, [ticketId])

  useEffect(() => {
    const loadMembers = async () => {
      try {
        setIsLoadingMembers(true)
        const data = await api.listWorkspaceMembers(slug)
        setMembers(data)
      } catch (error) {
        console.error('Failed to load workspace members:', error)
      } finally {
        setIsLoadingMembers(false)
      }
    }

    if (slug) {
      loadMembers()
    }
  }, [slug])

  useEffect(() => {
    const loadEpics = async () => {
      if (!ticketDetails?.ticket.project_id) return
      try {
        const epicsList = await api.listEpics(ticketDetails.ticket.project_id)
        setEpics(epicsList)
      } catch (error) {
        console.error('Failed to load epics:', error)
      }
    }

    if (ticketDetails?.ticket.project_id) {
      loadEpics()
    }
  }, [ticketDetails?.ticket.project_id])

  const loadTicket = async () => {
    try {
      const data = await api.getTicket(ticketId)
      setTicketDetails(data)
      setSelectedEpicId(data.ticket.epic_id || 'none')

      // Fetch epic if ticket belongs to one
      if (data.ticket.epic_id) {
        try {
          const epicData = await api.getEpic(data.ticket.epic_id)
          setEpic(epicData)
        } catch (error) {
          console.error('Failed to load epic:', error)
          setEpic(null)
        }
      } else {
        setEpic(null)
      }
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

  const handleUpdateStoryPoints = async (story_points: number | undefined) => {
    try {
      await api.updateTicket(ticketId, { story_points })
      await loadTicket()
    } catch (error) {
      console.error('Failed to update story points:', error)
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

  const handleEditComment = async (id: string, content: string) => {
    try {
      await api.updateComment(id, content)
      await loadTicket()  // Refresh to get updated comment
    } catch (error) {
      console.error('Failed to edit comment:', error)
      toast({
        title: 'Failed to edit comment',
        description: 'Please try again',
        variant: 'destructive',
      })
      throw error
    }
  }

  const handleDeleteComment = async (id: string) => {
    try {
      await api.deleteComment(id)
      await loadTicket()  // Refresh to remove deleted comment
    } catch (error) {
      console.error('Failed to delete comment:', error)
      toast({
        title: 'Failed to delete comment',
        description: 'Please try again',
        variant: 'destructive',
      })
      throw error
    }
  }

  const handleAssign = async (email: string) => {
    if (!ticketDetails) return

    // Optimistic update
    setTicketDetails({
      ...ticketDetails,
      ticket: {
        ...ticketDetails.ticket,
        assignees: [...ticketDetails.ticket.assignees, email],
      },
    })

    try {
      await api.assignTicket(ticketDetails.ticket.id, email)
    } catch (error) {
      // Rollback on error
      setTicketDetails({
        ...ticketDetails,
        ticket: {
          ...ticketDetails.ticket,
          assignees: ticketDetails.ticket.assignees.filter((a) => a !== email),
        },
      })
      throw error
    }
  }

  const handleUnassign = async (email: string) => {
    if (!ticketDetails) return

    // Optimistic update
    const previousAssignees = ticketDetails.ticket.assignees
    setTicketDetails({
      ...ticketDetails,
      ticket: {
        ...ticketDetails.ticket,
        assignees: ticketDetails.ticket.assignees.filter((a) => a !== email),
      },
    })

    try {
      await api.unassignTicket(ticketDetails.ticket.id, email)
    } catch (error) {
      // Rollback on error
      setTicketDetails({
        ...ticketDetails,
        ticket: {
          ...ticketDetails.ticket,
          assignees: previousAssignees,
        },
      })
      throw error
    }
  }

  const handleDelete = async () => {
    try {
      await api.deleteTicket(ticketId)
      toast({
        title: 'Ticket deleted',
        description: 'The ticket has been deleted successfully.',
      })
      // Navigate back to board or backlog
      router.push(`/w/${slug}/board`)
    } catch (error) {
      console.error('Failed to delete ticket:', error)
      toast({
        title: 'Failed to delete ticket',
        description: 'Please try again',
        variant: 'destructive',
      })
    }
  }

  const handleUpdateEpic = async () => {
    try {
      await api.updateTicket(ticketId, {
        epic_id: selectedEpicId === 'none' ? undefined : selectedEpicId,
      })
      toast({
        title: 'Epic updated',
        description: 'The ticket epic has been updated successfully.',
      })
      await loadTicket()
      setIsEditingEpic(false)
    } catch (error) {
      console.error('Failed to update epic:', error)
      toast({
        title: 'Failed to update epic',
        description: 'Please try again',
        variant: 'destructive',
      })
    }
  }

  const handleStatusChange = async (newStatus: TicketStatus) => {
    if (!ticketDetails) return

    // Optimistic update
    const previousStatus = ticketDetails.ticket.status
    setTicketDetails({
      ...ticketDetails,
      ticket: { ...ticketDetails.ticket, status: newStatus },
    })

    try {
      await api.updateTicketStatus(ticketId, newStatus)
      toast({
        title: `Status updated to ${getStatusLabel(newStatus)}`,
      })
      await loadTicket() // Refresh activity timeline
    } catch (error) {
      // Rollback on error
      setTicketDetails({
        ...ticketDetails,
        ticket: { ...ticketDetails.ticket, status: previousStatus },
      })
      toast({
        title: 'Failed to update status',
        description: 'Please try again',
        variant: 'destructive',
      })
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
          <div>
            <TicketHeader
              ticket={ticketDetails.ticket}
              onUpdateTitle={handleUpdateTitle}
              onUpdateStoryPoints={handleUpdateStoryPoints}
            />

            {/* Epic Badge */}
            <div className="mt-4">
              {!isEditingEpic && epic && (
                <div className="flex items-center gap-2">
                  <Link href={`/w/${slug}/epic/${epic.id}`}>
                    <Badge
                      variant="outline"
                      className="cursor-pointer hover:bg-accent"
                      style={{ borderColor: epic.epic_color }}
                    >
                      <Layers className="h-3 w-3 mr-1" />
                      JIL-{epic.number}: {epic.title}
                    </Badge>
                  </Link>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => setIsEditingEpic(true)}
                    className="h-6 px-2"
                  >
                    <Edit2 className="h-3 w-3" />
                  </Button>
                </div>
              )}

              {!isEditingEpic && !epic && (
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => setIsEditingEpic(true)}
                >
                  <Layers className="h-4 w-4 mr-2" />
                  Add to Epic
                </Button>
              )}

              {isEditingEpic && (
                <div className="flex items-center gap-2">
                  <Select value={selectedEpicId} onValueChange={setSelectedEpicId}>
                    <SelectTrigger className="w-[300px]">
                      <SelectValue placeholder="Select epic" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="none">None</SelectItem>
                      {epics.map((e) => (
                        <SelectItem key={e.id} value={e.id}>
                          <div className="flex items-center gap-2">
                            {e.epic_color && (
                              <div
                                className="w-3 h-3 rounded-full flex-shrink-0"
                                style={{ backgroundColor: e.epic_color }}
                              />
                            )}
                            <span>JIL-{e.number}: {e.title}</span>
                          </div>
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <Button
                    variant="default"
                    size="sm"
                    onClick={handleUpdateEpic}
                  >
                    <Check className="h-4 w-4" />
                  </Button>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => {
                      setIsEditingEpic(false)
                      setSelectedEpicId(ticketDetails.ticket.epic_id || 'none')
                    }}
                  >
                    <X className="h-4 w-4" />
                  </Button>
                </div>
              )}
            </div>
          </div>

          <div className="border-t border-border pt-4">
            <AssigneeSelector
              currentAssignees={ticketDetails.ticket.assignees}
              availableMembers={members}
              onAssign={handleAssign}
              onUnassign={handleUnassign}
              isLoading={isLoadingMembers}
            />
          </div>

          <TicketDescription
            description={ticketDetails.ticket.description}
            onUpdate={handleUpdateDescription}
          />

          <CommentsSection
            comments={ticketDetails.comments}
            currentUser={user?.email || 'system'}
            onAddComment={handleAddComment}
            onEditComment={handleEditComment}
            onDeleteComment={handleDeleteComment}
          />
        </div>

        <div className="space-y-8">
          {/* Status selector and delete button */}
          <div className="border-b border-border pb-4">
            <div className="flex items-center gap-2">
              <StatusSelector
                currentStatus={ticketDetails.ticket.status}
                onStatusChange={handleStatusChange}
              />
              <AlertDialog>
                <AlertDialogTrigger asChild>
                  <Button variant="destructive" size="icon" title="Delete ticket">
                    <Trash2 className="h-4 w-4" />
                  </Button>
                </AlertDialogTrigger>
                <AlertDialogContent>
                  <AlertDialogHeader>
                    <AlertDialogTitle>Are you sure?</AlertDialogTitle>
                    <AlertDialogDescription>
                      This will permanently delete ticket {ticketDetails.ticket.number}. This action cannot be undone.
                    </AlertDialogDescription>
                  </AlertDialogHeader>
                  <AlertDialogFooter>
                    <AlertDialogCancel>Cancel</AlertDialogCancel>
                    <AlertDialogAction onClick={handleDelete} className="bg-destructive text-destructive-foreground hover:bg-destructive/90">
                      Delete
                    </AlertDialogAction>
                  </AlertDialogFooter>
                </AlertDialogContent>
              </AlertDialog>
            </div>
          </div>

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
                        onClick={() => router.push(`/w/${slug}/ticket/${dep.id}`)}
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
                        onClick={() => router.push(`/w/${slug}/ticket/${dep.id}`)}
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
