'use client'

import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip'

interface AssigneeAvatarsProps {
  assignees: string[]
  maxVisible?: number
  size?: 'sm' | 'md' | 'lg'
}

function getInitials(email: string): string {
  return email.slice(0, 2).toUpperCase()
}

function getColorFromEmail(email: string): string {
  const colors = [
    'bg-blue-600 dark:bg-blue-500',
    'bg-green-600 dark:bg-green-500',
    'bg-purple-600 dark:bg-purple-500',
    'bg-pink-600 dark:bg-pink-500',
    'bg-yellow-600 dark:bg-yellow-500',
    'bg-indigo-600 dark:bg-indigo-500',
    'bg-red-600 dark:bg-red-500',
    'bg-orange-600 dark:bg-orange-500',
  ]
  const hash = email.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0)
  return colors[hash % colors.length]
}

export function AssigneeAvatars({
  assignees,
  maxVisible = 3,
  size = 'md',
}: AssigneeAvatarsProps) {
  const sizeClasses = {
    sm: 'h-6 w-6 text-xs',
    md: 'h-8 w-8 text-sm',
    lg: 'h-10 w-10 text-base',
  }

  const visible = assignees.slice(0, maxVisible)
  const remaining = assignees.length - maxVisible

  if (assignees.length === 0) {
    return null
  }

  return (
    <TooltipProvider>
      <div className="flex items-center -space-x-2">
        {visible.map((email, index) => (
          <Tooltip key={index}>
            <TooltipTrigger asChild>
              <Avatar className={`${sizeClasses[size]} border-2 border-background`}>
                <AvatarFallback className={getColorFromEmail(email)}>
                  {getInitials(email)}
                </AvatarFallback>
              </Avatar>
            </TooltipTrigger>
            <TooltipContent>
              <p>{email}</p>
            </TooltipContent>
          </Tooltip>
        ))}
        {remaining > 0 && (
          <Tooltip>
            <TooltipTrigger asChild>
              <Avatar className={`${sizeClasses[size]} border-2 border-background bg-muted`}>
                <AvatarFallback className="text-muted-foreground">
                  +{remaining}
                </AvatarFallback>
              </Avatar>
            </TooltipTrigger>
            <TooltipContent>
              <div className="space-y-1">
                {assignees.slice(maxVisible).map((email, index) => (
                  <p key={index}>{email}</p>
                ))}
              </div>
            </TooltipContent>
          </Tooltip>
        )}
      </div>
    </TooltipProvider>
  )
}
