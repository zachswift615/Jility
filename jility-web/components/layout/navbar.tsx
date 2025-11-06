'use client'

import { useState } from 'react'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { CommandPalette } from '@/components/command-palette'
import { ProjectSwitcher } from '@/components/projects/project-switcher'
import { ProjectFormDialog } from '@/components/projects/project-form-dialog'
import { WorkspaceSwitcher } from '@/components/workspace-switcher'
import { useProject } from '@/lib/project-context'
import { Button } from '@/components/ui/button'
import { Layers, BarChart3, Boxes, Calendar, Activity, Clock, ChevronDown, ListTodo, Settings } from 'lucide-react'
import { cn } from '@/lib/utils'

export function Navbar() {
  const pathname = usePathname()
  const { currentProject } = useProject()
  const [showProjectSwitcher, setShowProjectSwitcher] = useState(false)
  const [showProjectForm, setShowProjectForm] = useState(false)

  // Get project icon (first 2 letters of name)
  const getProjectIcon = (name: string) => {
    return name.slice(0, 2).toUpperCase()
  }

  const handleCreateNew = () => {
    setShowProjectForm(true)
  }

  const links = [
    { href: '/backlog', label: 'Backlog', icon: ListTodo },
    { href: '/board', label: 'Board', icon: Layers },
    { href: '/sprint/planning', label: 'Sprint Planning', icon: Calendar },
    { href: '/sprint/active', label: 'Active Sprint', icon: Activity },
    { href: '/sprint/history', label: 'History', icon: Clock },
    { href: '/agents', label: 'Agents', icon: Boxes },
  ]

  return (
    <nav className="border-b border-border bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 sticky top-0 z-50">
      <div className="container mx-auto px-4">
        <div className="flex h-14 md:h-16 items-center justify-between">
          <div className="flex items-center gap-4 md:gap-8">
            <Link href="/" className="flex items-center gap-2">
              <div className="rounded-lg bg-primary p-1.5 md:p-2">
                <BarChart3 className="h-4 w-4 md:h-5 md:w-5 text-primary-foreground" />
              </div>
              <span className="text-lg md:text-xl font-bold">Jility</span>
            </Link>

            {/* Workspace Switcher - Desktop */}
            <div className="hidden md:block">
              <WorkspaceSwitcher />
            </div>

            {/* Workspace Switcher - Mobile */}
            <div className="md:hidden">
              <WorkspaceSwitcher />
            </div>

            {/* Desktop nav links (hidden on mobile) */}
            <div className="hidden md:flex items-center gap-1">
              {links.map(({ href, label, icon: Icon }) => (
                <Link
                  key={href}
                  href={href}
                  className={cn(
                    'flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium transition-colors',
                    pathname === href
                      ? 'bg-accent text-accent-foreground'
                      : 'text-muted-foreground hover:text-foreground hover:bg-accent/50'
                  )}
                >
                  <Icon className="h-4 w-4" />
                  {label}
                </Link>
              ))}
            </div>
          </div>

          <div className="flex items-center gap-2 md:gap-4">
            {/* Project Switcher Button or Create Project Button */}
            {currentProject ? (
              <Button
                variant="outline"
                size="sm"
                onClick={() => setShowProjectSwitcher(true)}
                className="flex items-center gap-1 md:gap-2 text-xs md:text-sm"
              >
                <div
                  className="w-4 h-4 md:w-5 md:h-5 rounded flex items-center justify-center text-white font-semibold text-[10px] md:text-xs"
                  style={{
                    backgroundColor: currentProject.color || '#5e6ad2',
                  }}
                >
                  {getProjectIcon(currentProject.name)}
                </div>
                <span className="hidden sm:inline font-medium">{currentProject.name}</span>
                <ChevronDown className="h-3 w-3 md:h-4 md:w-4 opacity-50" />
              </Button>
            ) : (
              <Button
                variant="outline"
                size="sm"
                onClick={handleCreateNew}
                className="flex items-center gap-2 text-xs md:text-sm"
              >
                <span className="font-medium">Create Project</span>
              </Button>
            )}

            <div className="hidden sm:block">
              <CommandPalette />
            </div>
            <Link href="/profile" className="hidden md:block">
              <Button
                variant="ghost"
                size="sm"
                className={cn(
                  'flex items-center gap-2',
                  pathname === '/profile' && 'bg-accent text-accent-foreground'
                )}
              >
                <Settings className="h-4 w-4" />
              </Button>
            </Link>
          </div>
        </div>
      </div>

      {/* Project Switcher Modal */}
      <ProjectSwitcher
        open={showProjectSwitcher}
        onOpenChange={setShowProjectSwitcher}
        onCreateNew={handleCreateNew}
      />

      {/* Project Form Dialog */}
      <ProjectFormDialog
        open={showProjectForm}
        onOpenChange={setShowProjectForm}
      />
    </nav>
  )
}
