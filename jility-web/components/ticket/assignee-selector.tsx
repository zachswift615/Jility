'use client'

import { useState } from 'react'
import { X, UserPlus } from 'lucide-react'
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
} from '@/components/ui/command'
import { useToast } from '@/components/ui/use-toast'
import type { WorkspaceMember } from '@/lib/types'

interface AssigneeSelectorProps {
  currentAssignees: string[]
  availableMembers: WorkspaceMember[]
  onAssign: (email: string) => Promise<void>
  onUnassign: (email: string) => Promise<void>
  isLoading?: boolean
}

function getInitials(email: string): string {
  return email.slice(0, 2).toUpperCase()
}

function getColorFromEmail(email: string): string {
  const colors = [
    'bg-blue-500',
    'bg-green-500',
    'bg-purple-500',
    'bg-pink-500',
    'bg-yellow-500',
    'bg-indigo-500',
    'bg-red-500',
    'bg-orange-500',
  ]
  const hash = email.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0)
  return colors[hash % colors.length]
}

export function AssigneeSelector({
  currentAssignees,
  availableMembers,
  onAssign,
  onUnassign,
  isLoading = false,
}: AssigneeSelectorProps) {
  const [open, setOpen] = useState(false)
  const { toast } = useToast()

  const handleAssign = async (email: string) => {
    if (currentAssignees.includes(email)) return

    try {
      await onAssign(email)
      setOpen(false)
    } catch (error) {
      toast({
        title: 'Assignment failed',
        description: error instanceof Error ? error.message : 'Failed to assign member',
        variant: 'destructive',
      })
    }
  }

  const handleUnassign = async (email: string) => {
    try {
      await onUnassign(email)
    } catch (error) {
      toast({
        title: 'Unassignment failed',
        description: error instanceof Error ? error.message : 'Failed to unassign member',
        variant: 'destructive',
      })
    }
  }

  return (
    <div className="space-y-2">
      <label className="text-sm font-medium">Assignees</label>
      <div className="flex flex-wrap gap-2">
        {currentAssignees.map((email) => (
          <div
            key={email}
            className="flex items-center gap-2 px-3 py-1.5 bg-muted rounded-full"
          >
            <Avatar className="h-6 w-6">
              <AvatarFallback className={`${getColorFromEmail(email)} text-xs`}>
                {getInitials(email)}
              </AvatarFallback>
            </Avatar>
            <span className="text-sm">{email}</span>
            <Button
              variant="ghost"
              size="sm"
              className="h-4 w-4 p-0 hover:bg-transparent"
              onClick={() => handleUnassign(email)}
              disabled={isLoading}
            >
              <X className="h-3 w-3" />
            </Button>
          </div>
        ))}

        <Popover open={open} onOpenChange={setOpen}>
          <PopoverTrigger asChild>
            <Button variant="outline" size="sm" className="h-8" disabled={isLoading}>
              <UserPlus className="h-4 w-4 mr-2" />
              Add assignee
            </Button>
          </PopoverTrigger>
          <PopoverContent className="w-[300px] p-0" align="start">
            <Command>
              <CommandInput placeholder="Search members..." />
              <CommandEmpty>No members found.</CommandEmpty>
              <CommandGroup>
                {availableMembers.map((member) => {
                  const isAssigned = currentAssignees.includes(member.email)
                  return (
                    <CommandItem
                      key={member.user_id}
                      onSelect={() => handleAssign(member.email)}
                      disabled={isAssigned}
                    >
                      <Avatar className="h-6 w-6 mr-2">
                        <AvatarFallback className={getColorFromEmail(member.email)}>
                          {getInitials(member.email)}
                        </AvatarFallback>
                      </Avatar>
                      <span>{member.email}</span>
                      {isAssigned && (
                        <span className="ml-auto text-xs text-muted-foreground">✓</span>
                      )}
                    </CommandItem>
                  )
                })}
              </CommandGroup>
            </Command>
          </PopoverContent>
        </Popover>
      </div>
    </div>
  )
}
