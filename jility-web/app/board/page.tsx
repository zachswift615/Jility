'use client'

import { KanbanBoard } from '@/components/kanban/board'
import { withAuth } from '@/lib/with-auth'

function BoardPage() {
  return (
    <div className="h-[calc(100vh-3.5rem-4rem)] md:h-[calc(100vh-4rem)]">
      {/* 3.5rem = mobile nav (h-14), 4rem = bottom nav on mobile */}
      {/* 4rem = desktop nav (h-16) */}
      <KanbanBoard />
    </div>
  )
}

export default withAuth(BoardPage)
