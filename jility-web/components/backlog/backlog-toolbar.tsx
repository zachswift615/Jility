'use client'

import { Button } from '@/components/ui/button'
import { Zap, Bot } from 'lucide-react'

interface BacklogToolbarProps {
  totalItems: number
  totalPoints: number
  filter: string
  onFilterChange: (filter: string) => void
}

export function BacklogToolbar({
  totalItems,
  totalPoints,
  filter,
  onFilterChange,
}: BacklogToolbarProps) {
  return (
    <div className="flex flex-col sm:flex-row gap-3 sm:gap-4 sm:items-center sm:justify-between mb-5 p-4 bg-muted border border-border rounded-lg">
      <div className="flex flex-wrap items-center gap-3">
        <Button variant="outline" size="sm" className="gap-2">
          <Zap className="h-4 w-4" />
          Quick Add
        </Button>
        <Button variant="outline" size="sm" className="gap-2">
          <Bot className="h-4 w-4" />
          AI Planning
        </Button>
        <div className="border-l border-border h-6 mx-1" />
        <select
          value={filter}
          onChange={(e) => onFilterChange(e.target.value)}
          className="px-3 py-1.5 border border-border rounded-md text-sm bg-card cursor-pointer focus:outline-none focus:ring-2 focus:ring-ring"
        >
          <option value="all">All Issues</option>
          <option value="unestimated">Unestimated</option>
          <option value="ready">Ready for Sprint</option>
          <option value="needs_breakdown">Needs Breakdown</option>
        </select>
      </div>
      <div className="flex items-center gap-2 text-sm text-muted-foreground justify-start sm:justify-end">
        <span className="font-semibold">{totalItems} items</span>
        <span>•</span>
        <span>{totalPoints} story points</span>
      </div>
    </div>
  )
}
