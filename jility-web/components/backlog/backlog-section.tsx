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
    <div className="border-b border-gray-200 last:border-b-0">
      {/* Section Header */}
      <div
        className="flex items-center justify-between p-3 md:p-4 bg-gray-50 cursor-pointer hover:bg-gray-100 transition-colors"
        onClick={onToggle}
      >
        <div className="flex items-center gap-2 md:gap-3">
          {expanded ? (
            <ChevronDown className="h-4 w-4 text-gray-500" />
          ) : (
            <ChevronRight className="h-4 w-4 text-gray-500" />
          )}
          <span className="font-semibold text-sm text-gray-900">{title}</span>
          <span className="inline-flex items-center justify-center px-2 py-0.5 text-xs font-medium bg-gray-200 text-gray-700 rounded-full">
            {count}
          </span>
          {points !== undefined && (
            <span className="text-xs text-gray-500 ml-2">{points} pts</span>
          )}
          {subtitle && (
            <span className="text-xs text-gray-500 ml-2">{subtitle}</span>
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
