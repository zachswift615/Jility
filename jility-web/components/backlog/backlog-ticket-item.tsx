'use client'

import { useDraggable } from '@dnd-kit/core'
import { useRouter } from 'next/navigation'
import type { Ticket } from '@/lib/types'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { GripVertical, Edit, MoreHorizontal, BarChart3, User, MessageSquare, Bot, Link2 } from 'lucide-react'
import { cn } from '@/lib/utils'
import { AssigneeAvatars } from '@/components/ticket/assignee-avatars'

interface BacklogTicketItemProps {
  ticket: Ticket
  isDragging?: boolean
}

export function BacklogTicketItem({ ticket, isDragging = false }: BacklogTicketItemProps) {
  const router = useRouter()
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: ticket.id,
  })

  const style = transform
    ? {
        transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
      }
    : undefined

  const getLabelColor = (label: string): string => {
    const colors: Record<string, string> = {
      backend: 'bg-purple-100 text-purple-700 border-purple-200',
      frontend: 'bg-blue-100 text-blue-700 border-blue-200',
      human: 'bg-green-100 text-green-700 border-green-200',
      agent: 'bg-orange-100 text-orange-700 border-orange-200',
    }
    return colors[label.toLowerCase()] || 'bg-gray-100 text-gray-700 border-gray-200'
  }

  const handleClick = () => {
    router.push(`/ticket/${ticket.id}`)
  }

  return (
    <div
      ref={setNodeRef}
      style={style}
      className={cn(
        'group p-3 md:p-4 border-b border-border last:border-b-0 cursor-move transition-colors hover:bg-muted/50',
        isDragging && 'opacity-50 shadow-lg bg-card'
      )}
    >
      <div className="flex items-start gap-3">
        {/* Drag Handle */}
        <div
          {...listeners}
          {...attributes}
          className="cursor-grab active:cursor-grabbing text-muted-foreground hover:text-foreground mt-1"
        >
          <GripVertical className="h-4 w-4" />
        </div>

        {/* Content */}
        <div className="flex-1 min-w-0">
          {/* Ticket ID and Labels */}
          <div className="flex items-center gap-2 mb-1.5">
            <span className="text-xs font-mono text-muted-foreground">{ticket.number}</span>
            {ticket.labels.map((label) => (
              <Badge
                key={label}
                variant="outline"
                className={cn('text-xs', getLabelColor(label))}
              >
                {label}
              </Badge>
            ))}
          </div>

          {/* Title */}
          <div
            className="text-sm font-medium mb-2 cursor-pointer hover:text-primary"
            onClick={handleClick}
          >
            {ticket.title}
          </div>

          {/* Metadata */}
          <div className="flex items-center gap-4 text-xs text-muted-foreground">
            {/* Story Points */}
            <div className="flex items-center gap-1">
              <BarChart3 className="h-3.5 w-3.5" />
              {ticket.story_points !== undefined && ticket.story_points !== null ? (
                <span>{ticket.story_points} pts</span>
              ) : (
                <span className="text-orange-500 font-semibold">?</span>
              )}
            </div>

            {/* Assignees */}
            {ticket.assignees.length > 0 ? (
              <AssigneeAvatars assignees={ticket.assignees} size="sm" maxVisible={3} />
            ) : (
              <div className="flex items-center gap-1">
                <User className="h-3.5 w-3.5" />
                <span>Unassigned</span>
              </div>
            )}

            {/* Comments count (placeholder) */}
            {Math.random() > 0.7 && (
              <div className="flex items-center gap-1">
                <MessageSquare className="h-3.5 w-3.5" />
                <span>{Math.floor(Math.random() * 5) + 1} comments</span>
              </div>
            )}

            {/* Links count (placeholder) */}
            {Math.random() > 0.8 && (
              <div className="flex items-center gap-1">
                <Link2 className="h-3.5 w-3.5" />
                <span>{Math.floor(Math.random() * 3) + 1} links</span>
              </div>
            )}
          </div>
        </div>

        {/* Action Buttons */}
        <div className="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
          <Button
            variant="ghost"
            size="sm"
            className="h-8 w-8 p-0"
            onClick={handleClick}
          >
            <Edit className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="sm" className="h-8 w-8 p-0">
            <MoreHorizontal className="h-4 w-4" />
          </Button>
        </div>
      </div>
    </div>
  )
}
