'use client'

import type { TicketChange } from '@/lib/types'
import { formatDateTime, getStatusLabel } from '@/lib/utils'
import {
  Circle,
  Edit,
  AlertCircle,
  UserPlus,
  UserMinus,
  Tag,
  GitCommit,
  MessageSquare,
} from 'lucide-react'

interface ActivityTimelineProps {
  changes: TicketChange[]
}

function getChangeIcon(changeType: string) {
  switch (changeType) {
    case 'created':
      return <Circle className="h-4 w-4" />
    case 'status_changed':
      return <AlertCircle className="h-4 w-4" />
    case 'title_changed':
    case 'description_changed':
      return <Edit className="h-4 w-4" />
    case 'assignee_added':
      return <UserPlus className="h-4 w-4" />
    case 'assignee_removed':
      return <UserMinus className="h-4 w-4" />
    case 'label_added':
    case 'label_removed':
      return <Tag className="h-4 w-4" />
    case 'commit_linked':
      return <GitCommit className="h-4 w-4" />
    case 'comment_added':
      return <MessageSquare className="h-4 w-4" />
    default:
      return <Circle className="h-4 w-4" />
  }
}

function getChangeDescription(change: TicketChange): string {
  switch (change.change_type) {
    case 'created':
      return 'created this ticket'
    case 'status_changed':
      return `changed status from ${getStatusLabel(change.old_value || '')} to ${getStatusLabel(change.new_value || '')}`
    case 'title_changed':
      return `changed title`
    case 'description_changed':
      return `updated description`
    case 'assignee_added':
      return `assigned ${change.new_value}`
    case 'assignee_removed':
      return `unassigned ${change.old_value}`
    case 'label_added':
      return `added label ${change.new_value}`
    case 'label_removed':
      return `removed label ${change.old_value}`
    case 'story_points_changed':
      return `changed story points from ${change.old_value} to ${change.new_value}`
    case 'commit_linked':
      return `linked commit`
    case 'comment_added':
      return `added a comment`
    default:
      return change.change_type.replace(/_/g, ' ')
  }
}

export function ActivityTimeline({ changes }: ActivityTimelineProps) {
  if (changes.length === 0) {
    return (
      <div className="text-sm text-muted-foreground">
        No activity yet
      </div>
    )
  }

  return (
    <div className="space-y-4">
      <h2 className="text-lg font-semibold">Activity</h2>

      <div className="space-y-4">
        {changes.map((change, index) => (
          <div key={change.id} className="flex gap-3">
            <div className="relative flex flex-col items-center">
              <div className="rounded-full bg-muted p-2 text-muted-foreground">
                {getChangeIcon(change.change_type)}
              </div>
              {index < changes.length - 1 && (
                <div className="w-px h-full bg-border absolute top-8" />
              )}
            </div>

            <div className="flex-1 pb-4">
              <div className="text-sm">
                <span className="font-medium">{change.user_name}</span>
                {' '}
                <span className="text-muted-foreground">
                  {getChangeDescription(change)}
                </span>
              </div>
              <div className="text-xs text-muted-foreground mt-1">
                {formatDateTime(change.changed_at)}
              </div>
              {change.message && (
                <div className="mt-2 text-sm text-muted-foreground bg-muted rounded-md p-2">
                  {change.message}
                </div>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}
