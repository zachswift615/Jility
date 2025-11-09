'use client'

import { useState } from 'react'
import { api } from '@/lib/api'
import type { WorkspaceMember, PendingInvite } from '@/lib/types'
import { WorkspaceMemberList } from '@/components/workspace/member-list'
import { InviteMemberDialog } from '@/components/workspace/invite-member-dialog'
import { Button } from '@/components/ui/button'
import { UserPlus } from 'lucide-react'

interface WorkspaceTabProps {
  slug: string
  members: WorkspaceMember[]
  pendingInvites: PendingInvite[]
  isAdmin: boolean
  onUpdate: () => Promise<void>
}

export function WorkspaceTab({ slug, members, pendingInvites, isAdmin, onUpdate }: WorkspaceTabProps) {
  const [showInviteDialog, setShowInviteDialog] = useState(false)

  const handleInviteMember = async (email: string, role: 'admin' | 'member') => {
    const response = await api.inviteWorkspaceMember(slug, { email, role })
    await onUpdate()
    return response
  }

  const handleRemoveMember = async (userId: string) => {
    if (!confirm('Are you sure you want to remove this member?')) {
      return
    }

    try {
      await api.removeWorkspaceMember(slug, userId)
      await onUpdate()
    } catch (error) {
      console.error('Failed to remove member:', error)
    }
  }

  return (
    <div>
      <div className="bg-card border border-border rounded-lg p-6">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-semibold">Team Members</h2>
          {isAdmin && (
            <Button onClick={() => setShowInviteDialog(true)}>
              <UserPlus className="h-4 w-4 mr-2" />
              Invite Member
            </Button>
          )}
        </div>
        <WorkspaceMemberList
          members={members}
          pendingInvites={pendingInvites}
          isLoading={false}
          isAdmin={isAdmin}
          onRemove={handleRemoveMember}
        />
      </div>

      <InviteMemberDialog
        open={showInviteDialog}
        onOpenChange={setShowInviteDialog}
        onInvite={handleInviteMember}
      />
    </div>
  )
}
