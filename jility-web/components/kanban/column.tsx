'use client'

import { useDroppable } from '@dnd-kit/core'
import { SortableContext, verticalListSortingStrategy } from '@dnd-kit/sortable'
import type { Ticket, TicketStatus } from '@/lib/types'
import { TicketCard } from './ticket-card'
import { getStatusLabel, cn } from '@/lib/utils'
import { Plus } from 'lucide-react'

interface ColumnProps {
  status: TicketStatus
  tickets: Ticket[]
  onCreateTicket?: (status: TicketStatus) => void
}

export function Column({ status, tickets, onCreateTicket }: ColumnProps) {
  const { setNodeRef, isOver } = useDroppable({
    id: status,
  })

  const ticketIds = tickets.map((t) => t.id)

  return (
    <div className="flex flex-col w-80 flex-shrink-0">
      <div className="flex items-center justify-between mb-3 px-2">
        <div className="flex items-center gap-2">
          <h2 className="text-sm font-semibold">
            {getStatusLabel(status)}
          </h2>
          <span className="text-xs text-muted-foreground bg-muted px-2 py-0.5 rounded-full">
            {tickets.length}
          </span>
        </div>
        {onCreateTicket && (
          <button
            onClick={() => onCreateTicket(status)}
            className="p-1 rounded-md hover:bg-accent transition-colors"
            title="Add ticket"
          >
            <Plus className="h-4 w-4 text-muted-foreground" />
          </button>
        )}
      </div>

      <div
        ref={setNodeRef}
        className={cn(
          'flex-1 rounded-lg p-2 min-h-[200px] transition-colors',
          isOver && 'bg-accent/50 ring-2 ring-primary/50'
        )}
      >
        <SortableContext items={ticketIds} strategy={verticalListSortingStrategy}>
          <div className="space-y-2">
            {tickets.map((ticket) => (
              <TicketCard key={ticket.id} ticket={ticket} />
            ))}
          </div>
        </SortableContext>
      </div>
    </div>
  )
}
