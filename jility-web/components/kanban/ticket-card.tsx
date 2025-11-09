'use client'

import { useRouter, useParams, useSearchParams } from 'next/navigation'
import { useSortable } from '@dnd-kit/sortable'
import { CSS } from '@dnd-kit/utilities'
import { GripVertical, MessageSquare, GitCommit, Users, Layers } from 'lucide-react'
import type { Ticket, Epic } from '@/lib/types'
import { Badge } from '@/components/ui/badge'
import { AssigneeAvatars } from '@/components/ticket/assignee-avatars'
import { cn } from '@/lib/utils'

interface TicketCardProps {
  ticket: Ticket
  epics?: Epic[]
}

export function TicketCard({ ticket, epics = [] }: TicketCardProps) {
  const router = useRouter()
  const params = useParams()
  const searchParams = useSearchParams()
  const slug = params.slug as string
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: ticket.id })

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
  }

  const handleClick = (e: React.MouseEvent) => {
    // Don't navigate if currently dragging
    if (isDragging) {
      e.preventDefault()
      return
    }
    router.push(`/w/${slug}/ticket/${ticket.id}`)
  }

  const handleEpicClick = (e: React.MouseEvent, epicId: string) => {
    e.stopPropagation() // Prevent card navigation
    const params = new URLSearchParams(searchParams.toString())
    params.set('epic', epicId)
    router.push(`/w/${slug}/board?${params.toString()}`)
  }

  // Find epic for this ticket
  const ticketEpic = ticket.epic_id ? epics.find(e => e.id === ticket.epic_id) : null

  return (
    <div
      ref={setNodeRef}
      style={style}
      {...attributes}
      {...listeners}
      onClick={handleClick}
      className={cn(
        'group relative cursor-grab active:cursor-grabbing rounded-lg border border-border bg-card p-4 shadow-sm transition-all hover:shadow-md hover:border-primary/50',
        isDragging && 'opacity-50'
      )}
    >
      <div className="flex items-start justify-between gap-2 mb-2">
        <div className="flex items-center gap-2">
          <GripVertical className="h-4 w-4 text-muted-foreground opacity-0 group-hover:opacity-100 transition-opacity" />
          <span className="text-xs text-muted-foreground font-mono">
            {ticket.number}
          </span>
        </div>
        {ticket.story_points && (
          <span className="text-xs text-muted-foreground">
            {ticket.story_points} pts
          </span>
        )}
      </div>

      <h3 className="text-sm font-medium mb-3 line-clamp-2">
        {ticket.title}
      </h3>

      {ticketEpic && (
        <div className="mb-2">
          <Badge
            variant="outline"
            className="text-xs cursor-pointer hover:bg-accent"
            style={{ borderColor: ticketEpic.epic_color || undefined }}
            onClick={(e) => handleEpicClick(e, ticketEpic.id)}
          >
            <Layers className="h-3 w-3 mr-1" />
            {ticketEpic.title}
          </Badge>
        </div>
      )}

      <div className="flex items-center justify-between gap-2">
        <div className="flex items-center gap-2">
          {ticket.labels.length > 0 && (
            <div className="flex gap-1">
              {ticket.labels.slice(0, 2).map((label) => (
                <Badge key={label} variant="secondary" className="text-xs">
                  {label}
                </Badge>
              ))}
              {ticket.labels.length > 2 && (
                <Badge variant="secondary" className="text-xs">
                  +{ticket.labels.length - 2}
                </Badge>
              )}
            </div>
          )}
        </div>

        <AssigneeAvatars assignees={ticket.assignees} size="sm" maxVisible={3} />
      </div>
    </div>
  )
}
