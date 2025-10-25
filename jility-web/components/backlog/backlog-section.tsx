'use client'

import { ChevronDown, ChevronRight } from 'lucide-react'
import { ReactNode } from 'react'

interface BacklogSectionProps {
  title: string
  count: number
  points?: number
  subtitle?: string
  expanded: boolean
  onToggle: () => void
  action?: ReactNode
  children: ReactNode
}

export function BacklogSection({
  title,
  count,
  points,
  subtitle,
  expanded,
  onToggle,
  action,
  children,
}: BacklogSectionProps) {
  return (
    <div className="border-b border-border last:border-b-0">
      {/* Section Header */}
      <div
        className="flex items-center justify-between p-3 md:p-4 bg-muted/50 cursor-pointer hover:bg-muted transition-colors"
        onClick={onToggle}
      >
        <div className="flex items-center gap-2 md:gap-3">
          {expanded ? (
            <ChevronDown className="h-4 w-4 text-muted-foreground" />
          ) : (
            <ChevronRight className="h-4 w-4 text-muted-foreground" />
          )}
          <span className="font-semibold text-sm">{title}</span>
          <span className="inline-flex items-center justify-center px-2 py-0.5 text-xs font-medium bg-secondary text-secondary-foreground rounded-full">
            {count}
          </span>
          {points !== undefined && (
            <span className="text-xs text-muted-foreground ml-2">{points} pts</span>
          )}
          {subtitle && (
            <span className="text-xs text-muted-foreground ml-2">{subtitle}</span>
          )}
        </div>
        {action && (
          <div className="text-xs md:text-sm" onClick={(e) => e.stopPropagation()}>
            {action}
          </div>
        )}
      </div>

      {/* Section Content */}
      {expanded && children}
    </div>
  )
}
