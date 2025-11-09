'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { Kanban, Layers, ListTodo, Settings } from 'lucide-react'
import { cn } from '@/lib/utils'
import { useWorkspace } from '@/lib/workspace-context'

export function MobileBottomNav() {
  const pathname = usePathname()
  const { currentWorkspace } = useWorkspace()
  const slug = currentWorkspace?.slug || ''

  const navItems = [
    { href: `/w/${slug}/board`, label: 'Board', icon: Kanban },
    { href: `/w/${slug}/epics`, label: 'Epics', icon: Layers },
    { href: `/w/${slug}/backlog`, label: 'Backlog', icon: ListTodo },
    { href: `/w/${slug}/profile`, label: 'Settings', icon: Settings },
  ]

  return (
    <nav className="fixed bottom-0 left-0 right-0 z-50 bg-background border-t border-border md:hidden">
      <div className="flex justify-around items-center h-16 pb-safe">
        {navItems.map(({ href, label, icon: Icon }) => {
          const isActive = pathname === href || pathname === href.replace(`/w/${slug}`, '')
          return (
            <Link
              key={href}
              href={href}
              className={cn(
                'flex flex-col items-center justify-center gap-1 px-4 py-2 flex-1',
                'transition-colors',
                isActive
                  ? 'text-primary'
                  : 'text-muted-foreground hover:text-foreground'
              )}
            >
              <Icon className="h-5 w-5" />
              <span className="text-xs font-medium">{label}</span>
            </Link>
          )
        })}
      </div>
    </nav>
  )
}
