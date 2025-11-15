'use client'

import type { TicketStatus } from '@/lib/types'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { getStatusLabel } from '@/lib/utils'

interface StatusSelectorProps {
  currentStatus: TicketStatus
  onStatusChange: (newStatus: TicketStatus) => Promise<void>
  disabled?: boolean
}

const STATUSES: TicketStatus[] = ['backlog', 'todo', 'in_progress', 'review', 'done', 'blocked']

export function StatusSelector({ currentStatus, onStatusChange, disabled = false }: StatusSelectorProps) {
  const handleChange = async (value: string) => {
    await onStatusChange(value as TicketStatus)
  }

  return (
    <Select value={currentStatus} onValueChange={handleChange} disabled={disabled}>
      <SelectTrigger className="flex-1">
        <SelectValue>
          <div className="flex items-center gap-2">
            <div
              className="w-2 h-2 rounded-full flex-shrink-0"
              style={{ backgroundColor: `var(--status-${currentStatus})` }}
            />
            <span>{getStatusLabel(currentStatus)}</span>
          </div>
        </SelectValue>
      </SelectTrigger>
      <SelectContent>
        {STATUSES.map((status) => (
          <SelectItem key={status} value={status}>
            <div className="flex items-center gap-2">
              <div
                className="w-2 h-2 rounded-full flex-shrink-0"
                style={{ backgroundColor: `var(--status-${status})` }}
              />
              <span>{getStatusLabel(status)}</span>
            </div>
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  )
}
