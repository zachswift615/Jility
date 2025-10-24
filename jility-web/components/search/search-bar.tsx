'use client'

import { useState, useEffect, useRef } from 'react'
import { useRouter } from 'next/navigation'
import { useDebounce } from '@/lib/hooks'
import { api } from '@/lib/api'
import type { SearchResult } from '@/lib/types'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'

export function SearchBar() {
  const router = useRouter()
  const [query, setQuery] = useState('')
  const [isOpen, setIsOpen] = useState(false)
  const [results, setResults] = useState<SearchResult[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const dropdownRef = useRef<HTMLDivElement>(null)

  const debouncedQuery = useDebounce(query, 300)

  // Search when debounced query changes
  useEffect(() => {
    if (debouncedQuery.trim()) {
      setIsLoading(true)
      api
        .advancedSearch({ q: debouncedQuery, limit: 5 })
        .then((res) => {
          setResults(res.results)
          setIsOpen(true)
        })
        .catch((err) => {
          console.error('Search error:', err)
          setResults([])
        })
        .finally(() => {
          setIsLoading(false)
        })
    } else {
      setResults([])
      setIsOpen(false)
    }
  }, [debouncedQuery])

  // Close dropdown when clicking outside
  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false)
      }
    }

    document.addEventListener('mousedown', handleClickOutside)
    return () => document.removeEventListener('mousedown', handleClickOutside)
  }, [])

  const handleResultClick = (ticketId: string) => {
    router.push(`/ticket/${ticketId}`)
    setQuery('')
    setIsOpen(false)
  }

  const handleAdvancedSearch = () => {
    router.push(`/search?q=${encodeURIComponent(query)}`)
    setIsOpen(false)
  }

  return (
    <div className="relative w-full max-w-md" ref={dropdownRef}>
      <Input
        type="text"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        onFocus={() => {
          if (results.length > 0) setIsOpen(true)
        }}
        placeholder="Search tickets..."
        className="w-full"
      />

      {isOpen && results.length > 0 && (
        <div className="absolute z-50 mt-2 w-full rounded-md border bg-popover shadow-lg">
          <div className="max-h-96 overflow-y-auto p-2">
            {results.map((result) => (
              <button
                key={result.ticket_id}
                onClick={() => handleResultClick(result.ticket_id)}
                className="w-full rounded-md p-3 text-left hover:bg-accent transition-colors"
              >
                <div className="flex items-center gap-2">
                  <span className="font-mono text-xs text-muted-foreground">
                    {result.ticket_number}
                  </span>
                  <Badge variant="outline" className="text-xs">
                    {result.status}
                  </Badge>
                </div>
                <div className="mt-1 font-medium">{result.title}</div>
                <div
                  className="mt-1 text-sm text-muted-foreground line-clamp-2"
                  dangerouslySetInnerHTML={{ __html: result.snippet }}
                />
                <div className="mt-2 flex gap-2 text-xs text-muted-foreground">
                  {result.matched_in.map((field) => (
                    <Badge key={field} variant="secondary" className="text-xs">
                      {field}
                    </Badge>
                  ))}
                </div>
              </button>
            ))}
          </div>

          <div className="border-t p-2">
            <button
              onClick={handleAdvancedSearch}
              className="w-full rounded-md p-2 text-sm text-muted-foreground hover:bg-accent transition-colors"
            >
              View all results...
            </button>
          </div>
        </div>
      )}

      {isOpen && isLoading && (
        <div className="absolute z-50 mt-2 w-full rounded-md border bg-popover p-4 text-center text-sm text-muted-foreground shadow-lg">
          Searching...
        </div>
      )}

      {isOpen && !isLoading && query && results.length === 0 && (
        <div className="absolute z-50 mt-2 w-full rounded-md border bg-popover p-4 text-center text-sm text-muted-foreground shadow-lg">
          No results found
        </div>
      )}
    </div>
  )
}
