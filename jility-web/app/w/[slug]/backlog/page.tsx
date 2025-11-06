'use client'

import { useState } from 'react'
import { BacklogView } from '@/components/backlog/backlog-view'
import { MobileFAB } from '@/components/layout/mobile-fab'
import { CreateTicketDialog } from '@/components/ticket/create-ticket-dialog'
import { withAuth } from '@/lib/with-auth'

function BacklogPage() {
  const [showCreateDialog, setShowCreateDialog] = useState(false)

  return (
    <>
      <BacklogView />

      <MobileFAB onClick={() => setShowCreateDialog(true)} />

      <CreateTicketDialog
        open={showCreateDialog}
        onClose={() => setShowCreateDialog(false)}
      />
    </>
  )
}

export default withAuth(BacklogPage)
