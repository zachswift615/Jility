'use client'

import { useState } from 'react'
import { Check, ChevronsUpDown, Plus } from 'lucide-react'
import { useWorkspace } from '@/lib/workspace-context'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Button } from '@/components/ui/button'

export function WorkspaceSwitcher() {
  const { currentWorkspace, workspaces, switchWorkspace, isLoading } = useWorkspace()
  const [isOpen, setIsOpen] = useState(false)

  if (isLoading) {
    return (
      <Button variant="outline" disabled className="w-48">
        Loading...
      </Button>
    )
  }

  return (
    <DropdownMenu open={isOpen} onOpenChange={setIsOpen}>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" role="combobox" aria-expanded={isOpen} className="w-48 justify-between">
          {currentWorkspace?.name || 'Select workspace'}
          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-48">
        <DropdownMenuLabel>Workspaces</DropdownMenuLabel>
        <DropdownMenuSeparator />
        {workspaces.map((workspace) => (
          <DropdownMenuItem
            key={workspace.id}
            onSelect={() => {
              switchWorkspace(workspace.slug)
              setIsOpen(false)
            }}
          >
            <div className="flex items-center justify-between w-full">
              <span>{workspace.name}</span>
              {currentWorkspace?.id === workspace.id && <Check className="h-4 w-4" />}
            </div>
            {workspace.role === 'admin' && (
              <span className="ml-2 text-xs text-muted-foreground">(Admin)</span>
            )}
          </DropdownMenuItem>
        ))}
        <DropdownMenuSeparator />
        <DropdownMenuItem>
          <Plus className="mr-2 h-4 w-4" />
          <span>Create Workspace</span>
        </DropdownMenuItem>
        <DropdownMenuSeparator />
        <div className="px-2 py-1.5 text-xs text-muted-foreground">
          {workspaces.length} {workspaces.length === 1 ? 'workspace' : 'workspaces'}
        </div>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
