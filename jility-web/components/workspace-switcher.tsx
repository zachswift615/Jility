'use client'

import { useState } from 'react'
import { useWorkspace } from '@/lib/workspace-context'
import { Check, ChevronsUpDown, Plus } from 'lucide-react'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Button } from '@/components/ui/button'
import { CreateWorkspaceDialog } from './create-workspace-dialog'

export function WorkspaceSwitcher() {
  const { currentWorkspace, workspaces, switchWorkspace } = useWorkspace()
  const [showCreateDialog, setShowCreateDialog] = useState(false)

  if (!currentWorkspace) {
    return null
  }

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            variant="outline"
            className="min-w-[120px] max-w-[200px] h-11 md:h-auto justify-between"
          >
            <span className="truncate">{currentWorkspace.name}</span>
            <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent className="w-[200px]" align="start">
          <DropdownMenuLabel>Workspaces</DropdownMenuLabel>
          <DropdownMenuSeparator />
          {workspaces.map((workspace) => (
            <DropdownMenuItem
              key={workspace.id}
              onClick={() => switchWorkspace(workspace.slug)}
              className="cursor-pointer"
            >
              <Check
                className={`mr-2 h-4 w-4 ${
                  workspace.id === currentWorkspace.id
                    ? 'opacity-100'
                    : 'opacity-0'
                }`}
              />
              <span className="truncate">{workspace.name}</span>
            </DropdownMenuItem>
          ))}
          <DropdownMenuSeparator />
          <DropdownMenuItem
            onClick={() => setShowCreateDialog(true)}
            className="cursor-pointer"
          >
            <Plus className="mr-2 h-4 w-4" />
            Create workspace
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      <CreateWorkspaceDialog
        open={showCreateDialog}
        onOpenChange={setShowCreateDialog}
      />
    </>
  )
}
