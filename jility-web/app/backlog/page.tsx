'use client'

import { BacklogView } from '@/components/backlog/backlog-view'
import { withAuth } from '@/lib/with-auth'

function BacklogPage() {
  return (
    <div className="h-[calc(100vh-4rem)]">
      <BacklogView />
    </div>
  )
}

export default withAuth(BacklogPage)
