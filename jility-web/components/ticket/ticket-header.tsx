'use client'

import { useState } from 'react'
import type { Ticket } from '@/lib/types'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { getStatusLabel, formatDate } from '@/lib/utils'
import { Calendar, User } from 'lucide-react'

interface TicketHeaderProps {
  ticket: Ticket
  onUpdateTitle?: (title: string) => void
}

export function TicketHeader({ ticket, onUpdateTitle }: TicketHeaderProps) {
  const [isEditing, setIsEditing] = useState(false)
  const [title, setTitle] = useState(ticket.title)

  const handleSave = () => {
    if (title.trim() && title !== ticket.title && onUpdateTitle) {
      onUpdateTitle(title.trim())
    }
    setIsEditing(false)
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSave()
    } else if (e.key === 'Escape') {
      setTitle(ticket.title)
      setIsEditing(false)
    }
  }

  return (
    <div className="space-y-4">
      <div className="flex items-start justify-between gap-4">
        <div className="flex-1">
          <div className="flex items-center gap-3 mb-2">
            <span className="text-sm text-muted-foreground font-mono">
              {ticket.number}
            </span>
            <Badge variant={ticket.status as any}>
              {getStatusLabel(ticket.status)}
            </Badge>
            {ticket.story_points && (
              <span className="text-sm text-muted-foreground">
                {ticket.story_points} story points
              </span>
            )}
          </div>

          {isEditing ? (
            <Input
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              onBlur={handleSave}
              onKeyDown={handleKeyDown}
              className="text-2xl font-bold h-auto py-1"
              autoFocus
            />
          ) : (
            <h1
              onClick={() => setIsEditing(true)}
              className="text-2xl font-bold cursor-pointer hover:text-primary transition-colors"
            >
              {ticket.title}
            </h1>
          )}
        </div>
      </div>

      <div className="flex items-center gap-6 text-sm text-muted-foreground">
        <div className="flex items-center gap-2">
          <User className="h-4 w-4" />
          <span>Created by {ticket.created_by}</span>
        </div>
        <div className="flex items-center gap-2">
          <Calendar className="h-4 w-4" />
          <span>{formatDate(ticket.created_at)}</span>
        </div>
      </div>

      {ticket.assignees.length > 0 && (
        <div>
          <h3 className="text-sm font-medium mb-2">Assignees</h3>
          <div className="flex items-center gap-2">
            {ticket.assignees.map((assignee) => (
              <div key={assignee} className="flex items-center gap-2">
                <Avatar className="h-8 w-8">
                  <AvatarFallback className="text-xs">
                    {assignee.slice(0, 2).toUpperCase()}
                  </AvatarFallback>
                </Avatar>
                <span className="text-sm">{assignee}</span>
              </div>
            ))}
          </div>
        </div>
      )}

      {ticket.labels.length > 0 && (
        <div>
          <h3 className="text-sm font-medium mb-2">Labels</h3>
          <div className="flex flex-wrap gap-2">
            {ticket.labels.map((label) => (
              <Badge key={label} variant="secondary">
                {label}
              </Badge>
            ))}
          </div>
        </div>
      )}
    </div>
  )
}
