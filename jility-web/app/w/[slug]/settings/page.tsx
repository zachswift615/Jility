'use client'

import { useEffect, useState } from 'react'
import { useParams } from 'next/navigation'
import { useWorkspace } from '@/lib/workspace-context'
import { api } from '@/lib/api'
import type { WorkspaceMember } from '@/lib/types'
import { WorkspaceMemberList } from '@/components/workspace/member-list'
import { InviteMemberDialog } from '@/components/workspace/invite-member-dialog'
import { Button } from '@/components/ui/button'
import { UserPlus } from 'lucide-react'

export default function WorkspaceSettingsPage() {
  const params = useParams()
  const slug = params.slug as string
  const { currentWorkspace } = useWorkspace()
  const [members, setMembers] = useState<WorkspaceMember[]>([])
  const [isLoading, setIsLoading] = useState(true)
  const [showInviteDialog, setShowInviteDialog] = useState(false)

  useEffect(() => {
    if (slug) {
      loadMembers()
    }
  }, [slug])

  const loadMembers = async () => {
    try {
      setIsLoading(true)
      const data = await api.listWorkspaceMembers(slug)
      setMembers(data)
    } catch (error) {
      console.error('Failed to load members:', error)
    } finally {
      setIsLoading(false)
    }
  }

  const handleInviteMember = async (email: string, role: 'admin' | 'member') => {
    try {
      await api.inviteWorkspaceMember(slug, { email, role })
      await loadMembers()
      setShowInviteDialog(false)
    } catch (error) {
      console.error('Failed to invite member:', error)
      throw error
    }
  }

  const handleRemoveMember = async (userId: string) => {
    if (!confirm('Are you sure you want to remove this member?')) {
      return
    }

    try {
      await api.removeWorkspaceMember(slug, userId)
      await loadMembers()
    } catch (error) {
      console.error('Failed to remove member:', error)
    }
  }

  const isAdmin = currentWorkspace?.role === 'admin'

  return (
    <div className="container max-w-4xl py-8">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-3xl font-bold">Workspace Settings</h1>
          <p className="text-muted-foreground mt-1">
            Manage your workspace members and settings
          </p>
        </div>
        {isAdmin && (
          <Button onClick={() => setShowInviteDialog(true)}>
            <UserPlus className="h-4 w-4 mr-2" />
            Invite Member
          </Button>
        )}
      </div>

      <div className="bg-card border border-border rounded-lg p-6">
        <h2 className="text-xl font-semibold mb-4">Team Members</h2>
        <WorkspaceMemberList
          members={members}
          isLoading={isLoading}
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
