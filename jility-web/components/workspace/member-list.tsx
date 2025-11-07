'use client'

import { useState } from 'react'
import type { WorkspaceMember, PendingInvite } from '@/lib/types'
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Trash2, Loader2, Copy, Check, Clock } from 'lucide-react'

interface WorkspaceMemberListProps {
  members: WorkspaceMember[]
  pendingInvites: PendingInvite[]
  isLoading: boolean
  isAdmin: boolean
  onRemove: (userId: string) => void
}

export function WorkspaceMemberList({
  members,
  pendingInvites,
  isLoading,
  isAdmin,
  onRemove,
}: WorkspaceMemberListProps) {
  const [copiedInviteId, setCopiedInviteId] = useState<string | null>(null)

  const handleCopyInvite = async (inviteUrl: string, inviteId: string) => {
    await navigator.clipboard.writeText(inviteUrl)
    setCopiedInviteId(inviteId)
    setTimeout(() => setCopiedInviteId(null), 2000)
  }

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
      </div>
    )
  }

  if (members.length === 0 && pendingInvites.length === 0) {
    return (
      <div className="text-center py-12 text-muted-foreground">
        No members found
      </div>
    )
  }

  return (
    <div className="space-y-4">
      {/* Active Members */}
      {members.map((member) => (
        <div
          key={member.user_id}
          className="flex items-center justify-between p-4 border border-border rounded-lg"
        >
          <div className="flex items-center gap-3">
            <Avatar>
              <AvatarFallback>
                {member.email.slice(0, 2).toUpperCase()}
              </AvatarFallback>
            </Avatar>
            <div>
              <div className="font-medium">{member.email}</div>
              <div className="text-sm text-muted-foreground">
                Joined {new Date(member.joined_at).toLocaleDateString()}
              </div>
            </div>
          </div>

          <div className="flex items-center gap-2">
            <Badge variant={member.role === 'admin' ? 'default' : 'secondary'}>
              {member.role}
            </Badge>
            {isAdmin && (
              <Button
                variant="ghost"
                size="sm"
                onClick={() => onRemove(member.user_id)}
              >
                <Trash2 className="h-4 w-4 text-destructive" />
              </Button>
            )}
          </div>
        </div>
      ))}

      {/* Pending Invites */}
      {pendingInvites.map((invite) => {
        const isExpired = new Date(invite.expires_at) < new Date()
        return (
          <div
            key={invite.invite_id}
            className="flex items-center justify-between p-4 border border-border rounded-lg bg-muted/30"
          >
            <div className="flex items-center gap-3">
              <Avatar className="opacity-50">
                <AvatarFallback>
                  <Clock className="h-4 w-4" />
                </AvatarFallback>
              </Avatar>
              <div>
                <div className="font-medium">{invite.email}</div>
                <div className="text-sm text-muted-foreground">
                  Invited {new Date(invite.invited_at).toLocaleDateString()}
                  {isExpired && ' • Expired'}
                </div>
              </div>
            </div>

            <div className="flex items-center gap-2">
              <Badge variant="outline" className="gap-1">
                <Clock className="h-3 w-3" />
                Pending
              </Badge>
              <Badge variant={invite.role === 'admin' ? 'default' : 'secondary'}>
                {invite.role}
              </Badge>
              {isAdmin && (
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => handleCopyInvite(invite.invite_url, invite.invite_id)}
                  title="Copy invite URL"
                >
                  {copiedInviteId === invite.invite_id ? (
                    <Check className="h-4 w-4 text-green-600" />
                  ) : (
                    <Copy className="h-4 w-4" />
                  )}
                </Button>
              )}
            </div>
          </div>
        )
      })}
    </div>
  )
}
