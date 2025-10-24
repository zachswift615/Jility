'use client'

import Link from 'next/link'
import { useSortable } from '@dnd-kit/sortable'
import { CSS } from '@dnd-kit/utilities'
import { GripVertical, MessageSquare, GitCommit, Users } from 'lucide-react'
import type { Ticket } from '@/lib/types'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { cn } from '@/lib/utils'

interface TicketCardProps {
  ticket: Ticket
}

export function TicketCard({ ticket }: TicketCardProps) {
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

  return (
    <div
      ref={setNodeRef}
      style={style}
      className={cn(
        'group relative',
        isDragging && 'opacity-50'
      )}
    >
      <Link
        href={`/ticket/${ticket.id}`}
        className="block rounded-lg border border-border bg-card p-4 shadow-sm transition-all hover:shadow-md hover:border-primary/50"
      >
        <div className="flex items-start justify-between gap-2 mb-2">
          <div className="flex items-center gap-2">
            <button
              className="cursor-grab active:cursor-grabbing opacity-0 group-hover:opacity-100 transition-opacity"
              {...attributes}
              {...listeners}
            >
              <GripVertical className="h-4 w-4 text-muted-foreground" />
            </button>
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

          <div className="flex items-center gap-2">
            {ticket.assignees.length > 0 && (
              <div className="flex -space-x-2">
                {ticket.assignees.slice(0, 3).map((assignee) => (
                  <Avatar key={assignee} className="h-6 w-6 border-2 border-card">
                    <AvatarFallback className="text-xs">
                      {assignee.slice(0, 2).toUpperCase()}
                    </AvatarFallback>
                  </Avatar>
                ))}
                {ticket.assignees.length > 3 && (
                  <Avatar className="h-6 w-6 border-2 border-card">
                    <AvatarFallback className="text-xs">
                      +{ticket.assignees.length - 3}
                    </AvatarFallback>
                  </Avatar>
                )}
              </div>
            )}
          </div>
        </div>
      </Link>
    </div>
  )
}
