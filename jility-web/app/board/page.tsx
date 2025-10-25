'use client'

import { useState } from 'react'
import { KanbanBoard } from '@/components/kanban/board'
import { MobileFAB } from '@/components/layout/mobile-fab'
import { CreateTicketDialog } from '@/components/ticket/create-ticket-dialog'
import { withAuth } from '@/lib/with-auth'

function BoardPage() {
  const [showCreateDialog, setShowCreateDialog] = useState(false)

  return (
    <>
      <div className="h-[calc(100vh-3.5rem-4rem)] md:h-[calc(100vh-4rem)]">
        {/* 3.5rem = mobile nav (h-14), 4rem = bottom nav on mobile */}
        {/* 4rem = desktop nav (h-16) */}
        <KanbanBoard />
      </div>

      <MobileFAB onClick={() => setShowCreateDialog(true)} />

      <CreateTicketDialog
        open={showCreateDialog}
        onClose={() => setShowCreateDialog(false)}
      />
    </>
  )
}

export default withAuth(BoardPage)
