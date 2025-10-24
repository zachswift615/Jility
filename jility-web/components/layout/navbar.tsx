'use client'

import { useState } from 'react'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { ThemeSwitcher } from '@/components/theme-switcher'
import { CommandPalette } from '@/components/command-palette'
import { ProjectSwitcher } from '@/components/projects/project-switcher'
import { ProjectFormDialog } from '@/components/projects/project-form-dialog'
import { useProject } from '@/lib/project-context'
import { Button } from '@/components/ui/button'
import { Layers, BarChart3, Boxes, Calendar, Activity, Clock, ChevronDown } from 'lucide-react'
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
    { href: '/board', label: 'Board', icon: Layers },
    { href: '/sprint/planning', label: 'Sprint Planning', icon: Calendar },
    { href: '/sprint/active', label: 'Active Sprint', icon: Activity },
    { href: '/sprint/history', label: 'History', icon: Clock },
    { href: '/agents', label: 'Agents', icon: Boxes },
  ]

  return (
    <nav className="border-b border-border bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 sticky top-0 z-50">
      <div className="container mx-auto px-4">
        <div className="flex h-16 items-center justify-between">
          <div className="flex items-center gap-8">
            <Link href="/" className="flex items-center gap-2">
              <div className="rounded-lg bg-primary p-2">
                <BarChart3 className="h-5 w-5 text-primary-foreground" />
              </div>
              <span className="text-xl font-bold">Jility</span>
            </Link>

            <div className="flex items-center gap-1">
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

          <div className="flex items-center gap-4">
            {/* Project Switcher Button or Create Project Button */}
            {currentProject ? (
              <Button
                variant="outline"
                size="sm"
                onClick={() => setShowProjectSwitcher(true)}
                className="flex items-center gap-2"
              >
                <div
                  className="w-5 h-5 rounded flex items-center justify-center text-white font-semibold text-xs"
                  style={{
                    backgroundColor: currentProject.color || '#5e6ad2',
                  }}
                >
                  {getProjectIcon(currentProject.name)}
                </div>
                <span className="font-medium">{currentProject.name}</span>
                <ChevronDown className="h-4 w-4 opacity-50" />
              </Button>
            ) : (
              <Button
                variant="outline"
                size="sm"
                onClick={handleCreateNew}
                className="flex items-center gap-2"
              >
                <span className="font-medium">Create Project</span>
              </Button>
            )}

            <CommandPalette />
            <ThemeSwitcher />
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
