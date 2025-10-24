'use client'

import { useEffect, useState, useCallback } from 'react'
import { useRouter } from 'next/navigation'
import { Command } from 'cmdk'
import { Search, Plus, FileText, Users } from 'lucide-react'
import { api } from '@/lib/api'
import type { Ticket } from '@/lib/types'
import { Badge } from '@/components/ui/badge'
import { getStatusLabel } from '@/lib/utils'
import { cn } from '@/lib/utils'

export function CommandPalette() {
  const [open, setOpen] = useState(false)
  const [search, setSearch] = useState('')
  const [tickets, setTickets] = useState<Ticket[]>([])
  const [loading, setLoading] = useState(false)
  const router = useRouter()

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === 'k' && (e.metaKey || e.ctrlKey)) {
        e.preventDefault()
        setOpen((open) => !open)
      }
    }

    document.addEventListener('keydown', down)
    return () => document.removeEventListener('keydown', down)
  }, [])

  useEffect(() => {
    if (search.length > 0) {
      setLoading(true)
      api
        .searchTickets(search, 10)
        .then(setTickets)
        .catch(console.error)
        .finally(() => setLoading(false))
    } else {
      setTickets([])
    }
  }, [search])

  const handleSelect = useCallback(
    (callback: () => void) => {
      setOpen(false)
      callback()
    },
    []
  )

  return (
    <>
      <button
        onClick={() => setOpen(true)}
        className="flex items-center gap-2 px-3 py-2 text-sm text-muted-foreground border border-border rounded-md hover:bg-accent hover:text-accent-foreground transition-colors"
      >
        <Search className="h-4 w-4" />
        <span>Search...</span>
        <kbd className="pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border border-border bg-muted px-1.5 font-mono text-[10px] font-medium opacity-100">
          <span className="text-xs">⌘</span>K
        </kbd>
      </button>

      <Command.Dialog
        open={open}
        onOpenChange={setOpen}
        label="Command Menu"
        className={cn(
          'fixed inset-0 z-50',
          open ? 'animate-in fade-in-0' : 'animate-out fade-out-0'
        )}
      >
        <div className="fixed inset-0 bg-background/80 backdrop-blur-sm" onClick={() => setOpen(false)} />
        <div className="fixed left-[50%] top-[50%] z-50 w-full max-w-2xl translate-x-[-50%] translate-y-[-50%]">
          <Command className="overflow-hidden rounded-lg border border-border bg-card shadow-lg">
            <div className="flex items-center border-b border-border px-3">
              <Search className="mr-2 h-4 w-4 shrink-0 opacity-50" />
              <Command.Input
                value={search}
                onValueChange={setSearch}
                placeholder="Type a command or search..."
                className="flex h-12 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50"
              />
            </div>
            <Command.List className="max-h-[400px] overflow-y-auto overflow-x-hidden p-2">
              <Command.Empty className="py-6 text-center text-sm text-muted-foreground">
                {loading ? 'Searching...' : 'No results found.'}
              </Command.Empty>

              <Command.Group heading="Actions">
                <Command.Item
                  onSelect={() => handleSelect(() => router.push('/board?create=true'))}
                  className="flex items-center gap-2 px-2 py-3 rounded-md cursor-pointer hover:bg-accent aria-selected:bg-accent"
                >
                  <Plus className="h-4 w-4" />
                  <span>Create Ticket</span>
                </Command.Item>
              </Command.Group>

              {tickets.length > 0 && (
                <Command.Group heading="Tickets">
                  {tickets.map((ticket) => (
                    <Command.Item
                      key={ticket.id}
                      value={ticket.number}
                      onSelect={() => handleSelect(() => router.push(`/ticket/${ticket.id}`))}
                      className="flex items-center justify-between gap-2 px-2 py-3 rounded-md cursor-pointer hover:bg-accent aria-selected:bg-accent"
                    >
                      <div className="flex items-center gap-3 flex-1 min-w-0">
                        <FileText className="h-4 w-4 shrink-0" />
                        <div className="flex flex-col gap-1 flex-1 min-w-0">
                          <div className="flex items-center gap-2">
                            <span className="text-xs text-muted-foreground font-mono">
                              {ticket.number}
                            </span>
                            <span className="truncate">{ticket.title}</span>
                          </div>
                        </div>
                      </div>
                      <Badge variant={ticket.status as any} className="shrink-0">
                        {getStatusLabel(ticket.status)}
                      </Badge>
                    </Command.Item>
                  ))}
                </Command.Group>
              )}
            </Command.List>
          </Command>
        </div>
      </Command.Dialog>
    </>
  )
}
