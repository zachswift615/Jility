'use client'

import type { WorkspaceMember } from '@/lib/types'
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Trash2, Loader2 } from 'lucide-react'

interface WorkspaceMemberListProps {
  members: WorkspaceMember[]
  isLoading: boolean
  isAdmin: boolean
  onRemove: (userId: string) => void
}

export function WorkspaceMemberList({
  members,
  isLoading,
  isAdmin,
  onRemove,
}: WorkspaceMemberListProps) {
  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
      </div>
    )
  }

  if (members.length === 0) {
    return (
      <div className="text-center py-12 text-muted-foreground">
        No members found
      </div>
    )
  }

  return (
    <div className="space-y-4">
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
    </div>
  )
}
