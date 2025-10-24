'use client'

import { KanbanBoard } from '@/components/kanban/board'
import { withAuth } from '@/lib/with-auth'

function BoardPage() {
  return (
    <div className="h-[calc(100vh-4rem)]">
      <KanbanBoard />
    </div>
  )
}

export default withAuth(BoardPage)
