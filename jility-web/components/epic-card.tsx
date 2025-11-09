'use client'

import { useRouter, useParams } from 'next/navigation'
import type { Epic } from '@/lib/types'
import { Progress } from '@/components/ui/progress'
import { cn } from '@/lib/utils'

interface EpicCardProps {
  epic: Epic
}

export function EpicCard({ epic }: EpicCardProps) {
  const router = useRouter()
  const params = useParams()
  const slug = params.slug as string

  const handleClick = () => {
    router.push(`/w/${slug}/epic/${epic.id}`)
  }

  return (
    <div
      onClick={handleClick}
      className={cn(
        'group cursor-pointer rounded-lg border border-border bg-card p-6 shadow-sm transition-all hover:shadow-md hover:border-primary/50'
      )}
    >
      {/* Epic Color Indicator */}
      {epic.epic_color && (
        <div
          className="h-1 w-full rounded-t-lg -mt-6 -mx-6 mb-4"
          style={{ backgroundColor: epic.epic_color }}
        />
      )}

      {/* Header */}
      <div className="flex items-start justify-between gap-2 mb-3">
        <div className="flex items-center gap-2">
          <span className="text-xs text-muted-foreground font-mono">
            {epic.number}
          </span>
        </div>
      </div>

      {/* Title */}
      <h3 className="text-lg font-semibold mb-2 line-clamp-2">
        {epic.title}
      </h3>

      {/* Description */}
      {epic.description && (
        <p className="text-sm text-muted-foreground mb-4 line-clamp-2">
          {epic.description}
        </p>
      )}

      {/* Progress Bar */}
      <div className="mb-3">
        <Progress value={epic.progress.completion_percentage} className="h-2" />
      </div>

      {/* Progress Stats */}
      <div className="flex items-center justify-between text-sm">
        <span className="text-muted-foreground">
          {epic.progress.done} of {epic.progress.total} tasks completed
        </span>
        <span className="font-medium text-primary">
          {Math.round(epic.progress.completion_percentage)}%
        </span>
      </div>

      {/* Status Breakdown */}
      <div className="flex gap-4 mt-3 text-xs text-muted-foreground">
        {epic.progress.in_progress > 0 && (
          <span>{epic.progress.in_progress} in progress</span>
        )}
        {epic.progress.todo > 0 && (
          <span>{epic.progress.todo} todo</span>
        )}
        {epic.progress.blocked > 0 && (
          <span className="text-destructive">{epic.progress.blocked} blocked</span>
        )}
      </div>
    </div>
  )
}
