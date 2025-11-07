'use client'

import { useState } from 'react'
import type { Ticket } from '@/lib/types'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'
import { getStatusLabel, formatDate } from '@/lib/utils'
import { Calendar, User } from 'lucide-react'

interface TicketHeaderProps {
  ticket: Ticket
  onUpdateTitle?: (title: string) => void
  onUpdateStoryPoints?: (points: number | undefined) => void
}

export function TicketHeader({ ticket, onUpdateTitle, onUpdateStoryPoints }: TicketHeaderProps) {
  const [isEditingTitle, setIsEditingTitle] = useState(false)
  const [isEditingPoints, setIsEditingPoints] = useState(false)
  const [title, setTitle] = useState(ticket.title)
  const [storyPoints, setStoryPoints] = useState(ticket.story_points)

  const handleSaveTitle = () => {
    if (title.trim() && title !== ticket.title && onUpdateTitle) {
      onUpdateTitle(title.trim())
    }
    setIsEditingTitle(false)
  }

  const handleSavePoints = () => {
    if (storyPoints !== ticket.story_points && onUpdateStoryPoints) {
      onUpdateStoryPoints(storyPoints)
    }
    setIsEditingPoints(false)
  }

  const handleTitleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSaveTitle()
    } else if (e.key === 'Escape') {
      setTitle(ticket.title)
      setIsEditingTitle(false)
    }
  }

  const handlePointsKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSavePoints()
    } else if (e.key === 'Escape') {
      setStoryPoints(ticket.story_points)
      setIsEditingPoints(false)
    }
  }

  return (
    <div className="space-y-3 md:space-y-4">
      <div className="flex items-start justify-between gap-4">
        <div className="flex-1">
          <div className="flex flex-wrap items-center gap-2 md:gap-3 mb-2">
            <span className="text-xs md:text-sm text-muted-foreground font-mono">
              {ticket.number}
            </span>
            <Badge variant={ticket.status as any} className="text-xs">
              {getStatusLabel(ticket.status)}
            </Badge>
            {isEditingPoints ? (
              <Input
                type="number"
                min="0"
                step="1"
                value={storyPoints ?? ''}
                onChange={(e) => setStoryPoints(e.target.value ? Number(e.target.value) : undefined)}
                onBlur={handleSavePoints}
                onKeyDown={handlePointsKeyDown}
                className="w-20 h-6 px-2 text-xs"
                autoFocus
              />
            ) : (
              <span
                onClick={() => onUpdateStoryPoints && setIsEditingPoints(true)}
                className={`text-xs md:text-sm ${onUpdateStoryPoints ? 'cursor-pointer hover:text-primary' : ''} ${ticket.story_points ? 'text-muted-foreground' : 'text-muted-foreground/50'}`}
                title={onUpdateStoryPoints ? 'Click to edit story points' : undefined}
              >
                {ticket.story_points ? `${ticket.story_points} points` : 'No points'}
              </span>
            )}
          </div>

          {isEditingTitle ? (
            <Input
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              onBlur={handleSaveTitle}
              onKeyDown={handleTitleKeyDown}
              className="text-xl md:text-2xl font-bold h-auto py-1"
              autoFocus
            />
          ) : (
            <h1
              onClick={() => setIsEditingTitle(true)}
              className="text-xl md:text-3xl font-bold leading-tight cursor-pointer hover:text-primary transition-colors"
            >
              {ticket.title}
            </h1>
          )}
        </div>
      </div>

      <div className="flex flex-wrap gap-3 md:gap-6 text-xs md:text-sm text-muted-foreground">
        <div className="flex items-center gap-2">
          <User className="h-4 w-4" />
          <span>Created by {ticket.created_by}</span>
        </div>
        <div className="flex items-center gap-2">
          <Calendar className="h-4 w-4" />
          <span>{formatDate(ticket.created_at)}</span>
        </div>
      </div>

      {ticket.labels.length > 0 && (
        <div>
          <h3 className="text-xs md:text-sm font-medium mb-2">Labels</h3>
          <div className="flex flex-wrap gap-2">
            {ticket.labels.map((label) => (
              <Badge key={label} variant="secondary" className="text-xs">
                {label}
              </Badge>
            ))}
          </div>
        </div>
      )}
    </div>
  )
}
