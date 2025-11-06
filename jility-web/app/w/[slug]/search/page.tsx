'use client'

import { useState, useEffect, Suspense } from 'react'
import { useSearchParams, useRouter } from 'next/navigation'
import { api } from '@/lib/api'
import type { SearchFilters, SearchResult, SavedView } from '@/lib/types'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Card } from '@/components/ui/card'

function SearchPageContent() {
  const router = useRouter()
  const searchParams = useSearchParams()
  const initialQuery = searchParams.get('q') || ''

  const [filters, setFilters] = useState<SearchFilters>({
    q: initialQuery,
    limit: 20,
    offset: 0,
  })

  const [results, setResults] = useState<SearchResult[]>([])
  const [total, setTotal] = useState(0)
  const [hasMore, setHasMore] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  const [savedViews, setSavedViews] = useState<SavedView[]>([])
  const [showSaveDialog, setShowSaveDialog] = useState(false)
  const [viewName, setViewName] = useState('')

  // Load saved views
  useEffect(() => {
    api
      .listSavedViews()
      .then(setSavedViews)
      .catch((err) => console.error('Failed to load saved views:', err))
  }, [])

  // Search when filters change
  useEffect(() => {
    if (filters.q.trim()) {
      performSearch()
    }
  }, [filters])

  const performSearch = async () => {
    setIsLoading(true)
    try {
      const response = await api.advancedSearch(filters)
      setResults(response.results)
      setTotal(response.total)
      setHasMore(response.has_more)
    } catch (err) {
      console.error('Search failed:', err)
    } finally {
      setIsLoading(false)
    }
  }

  const updateFilter = <K extends keyof SearchFilters>(key: K, value: SearchFilters[K]) => {
    setFilters((prev) => ({ ...prev, [key]: value, offset: 0 }))
  }

  const addStatusFilter = (status: string) => {
    const currentStatuses = filters.status || []
    if (!currentStatuses.includes(status)) {
      updateFilter('status', [...currentStatuses, status])
    }
  }

  const removeStatusFilter = (status: string) => {
    const currentStatuses = filters.status || []
    updateFilter(
      'status',
      currentStatuses.filter((s) => s !== status)
    )
  }

  const clearFilters = () => {
    setFilters({ q: filters.q, limit: 20, offset: 0 })
  }

  const loadMore = () => {
    setFilters((prev) => ({ ...prev, offset: (prev.offset || 0) + (prev.limit || 20) }))
  }

  const saveCurrentView = async () => {
    if (!viewName.trim()) return

    try {
      await api.createSavedView({
        name: viewName,
        filters,
        is_default: false,
        is_shared: false,
      })

      // Reload saved views
      const views = await api.listSavedViews()
      setSavedViews(views)
      setShowSaveDialog(false)
      setViewName('')
    } catch (err) {
      console.error('Failed to save view:', err)
    }
  }

  const loadSavedView = (view: SavedView) => {
    setFilters(view.filters)
  }

  const statuses = ['backlog', 'todo', 'in_progress', 'review', 'done', 'blocked']

  return (
    <div className="container mx-auto p-6">
      <div className="mb-6">
        <h1 className="text-3xl font-bold">Advanced Search</h1>
        <p className="text-muted-foreground">
          Search and filter tickets with powerful full-text search
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
        {/* Sidebar - Filters & Saved Views */}
        <div className="space-y-6">
          {/* Saved Views */}
          {savedViews.length > 0 && (
            <Card className="p-4">
              <h3 className="font-semibold mb-3">Saved Views</h3>
              <div className="space-y-2">
                {savedViews.map((view) => (
                  <button
                    key={view.id}
                    onClick={() => loadSavedView(view)}
                    className="w-full text-left p-2 rounded hover:bg-accent transition-colors"
                  >
                    <div className="font-medium">{view.name}</div>
                    {view.description && (
                      <div className="text-xs text-muted-foreground">{view.description}</div>
                    )}
                  </button>
                ))}
              </div>
            </Card>
          )}

          {/* Filters */}
          <Card className="p-4">
            <div className="flex items-center justify-between mb-3">
              <h3 className="font-semibold">Filters</h3>
              <Button variant="ghost" size="sm" onClick={clearFilters}>
                Clear
              </Button>
            </div>

            <div className="space-y-4">
              {/* Status Filter */}
              <div>
                <label className="text-sm font-medium mb-2 block">Status</label>
                <div className="flex flex-wrap gap-2">
                  {statuses.map((status) => {
                    const isActive = filters.status?.includes(status)
                    return (
                      <Badge
                        key={status}
                        variant={isActive ? 'default' : 'outline'}
                        className="cursor-pointer"
                        onClick={() =>
                          isActive ? removeStatusFilter(status) : addStatusFilter(status)
                        }
                      >
                        {status.replace('_', ' ')}
                      </Badge>
                    )
                  })}
                </div>
              </div>

              {/* Created By */}
              <div>
                <label className="text-sm font-medium mb-2 block">Created By</label>
                <Input
                  type="text"
                  value={filters.created_by || ''}
                  onChange={(e) => updateFilter('created_by', e.target.value || undefined)}
                  placeholder="username"
                />
              </div>

              {/* Story Points Range */}
              <div>
                <label className="text-sm font-medium mb-2 block">Story Points</label>
                <div className="grid grid-cols-2 gap-2">
                  <Input
                    type="number"
                    value={filters.min_points || ''}
                    onChange={(e) =>
                      updateFilter('min_points', e.target.value ? parseInt(e.target.value) : undefined)
                    }
                    placeholder="Min"
                  />
                  <Input
                    type="number"
                    value={filters.max_points || ''}
                    onChange={(e) =>
                      updateFilter('max_points', e.target.value ? parseInt(e.target.value) : undefined)
                    }
                    placeholder="Max"
                  />
                </div>
              </div>

              {/* Boolean Filters */}
              <div className="space-y-2">
                <label className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    checked={filters.has_comments || false}
                    onChange={(e) => updateFilter('has_comments', e.target.checked || undefined)}
                  />
                  <span className="text-sm">Has comments</span>
                </label>
                <label className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    checked={filters.has_commits || false}
                    onChange={(e) => updateFilter('has_commits', e.target.checked || undefined)}
                  />
                  <span className="text-sm">Has commits</span>
                </label>
                <label className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    checked={filters.has_dependencies || false}
                    onChange={(e) => updateFilter('has_dependencies', e.target.checked || undefined)}
                  />
                  <span className="text-sm">Has dependencies</span>
                </label>
              </div>
            </div>
          </Card>

          {/* Save Current View */}
          <Card className="p-4">
            <h3 className="font-semibold mb-3">Save Current Search</h3>
            {showSaveDialog ? (
              <div className="space-y-2">
                <Input
                  type="text"
                  value={viewName}
                  onChange={(e) => setViewName(e.target.value)}
                  placeholder="View name"
                />
                <div className="flex gap-2">
                  <Button size="sm" onClick={saveCurrentView}>
                    Save
                  </Button>
                  <Button
                    size="sm"
                    variant="ghost"
                    onClick={() => {
                      setShowSaveDialog(false)
                      setViewName('')
                    }}
                  >
                    Cancel
                  </Button>
                </div>
              </div>
            ) : (
              <Button size="sm" onClick={() => setShowSaveDialog(true)}>
                Save as View
              </Button>
            )}
          </Card>
        </div>

        {/* Main Content - Search Results */}
        <div className="lg:col-span-3 space-y-6">
          {/* Search Input */}
          <Card className="p-4">
            <Input
              type="text"
              value={filters.q}
              onChange={(e) => updateFilter('q', e.target.value)}
              placeholder="Search tickets..."
              className="text-lg"
            />
          </Card>

          {/* Applied Filters */}
          {(filters.status || filters.created_by || filters.min_points || filters.max_points) && (
            <div className="flex flex-wrap gap-2">
              {filters.status?.map((status) => (
                <Badge key={status} variant="secondary">
                  Status: {status}
                  <button
                    onClick={() => removeStatusFilter(status)}
                    className="ml-2 hover:text-destructive"
                  >
                    ×
                  </button>
                </Badge>
              ))}
              {filters.created_by && (
                <Badge variant="secondary">
                  Created by: {filters.created_by}
                  <button
                    onClick={() => updateFilter('created_by', undefined)}
                    className="ml-2 hover:text-destructive"
                  >
                    ×
                  </button>
                </Badge>
              )}
            </div>
          )}

          {/* Results Count */}
          {!isLoading && (
            <div className="text-sm text-muted-foreground">
              Found {total} result{total !== 1 ? 's' : ''}
            </div>
          )}

          {/* Loading State */}
          {isLoading && (
            <div className="text-center py-12 text-muted-foreground">Searching...</div>
          )}

          {/* Results */}
          {!isLoading && results.length > 0 && (
            <div className="space-y-4">
              {results.map((result) => (
                <Card
                  key={result.ticket_id}
                  className="p-4 cursor-pointer hover:border-primary transition-colors"
                  onClick={() => router.push(`/ticket/${result.ticket_id}`)}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex-1">
                      <div className="flex items-center gap-2 mb-2">
                        <span className="font-mono text-sm font-medium text-muted-foreground">
                          {result.ticket_number}
                        </span>
                        <Badge variant="outline">{result.status}</Badge>
                        {result.story_points && (
                          <Badge variant="secondary">{result.story_points} pts</Badge>
                        )}
                      </div>

                      <h3 className="font-semibold text-lg mb-2">{result.title}</h3>

                      <div
                        className="text-sm text-muted-foreground line-clamp-2 mb-3"
                        dangerouslySetInnerHTML={{ __html: result.snippet }}
                      />

                      <div className="flex items-center gap-4 text-xs text-muted-foreground">
                        <span>by {result.created_by}</span>
                        <span>
                          {new Date(result.created_at).toLocaleDateString()}
                        </span>
                        <div className="flex gap-2">
                          {result.matched_in.map((field) => (
                            <Badge key={field} variant="secondary" className="text-xs">
                              {field}
                            </Badge>
                          ))}
                        </div>
                      </div>
                    </div>

                    <div className="ml-4 text-sm text-muted-foreground">
                      Score: {result.rank.toFixed(2)}
                    </div>
                  </div>
                </Card>
              ))}
            </div>
          )}

          {/* Load More */}
          {!isLoading && hasMore && (
            <div className="text-center">
              <Button onClick={loadMore}>Load More</Button>
            </div>
          )}

          {/* No Results */}
          {!isLoading && filters.q && results.length === 0 && (
            <div className="text-center py-12 text-muted-foreground">
              No results found for "{filters.q}"
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

export default function SearchPage() {
  return (
    <Suspense fallback={<div className="container mx-auto p-6 text-center">Loading search...</div>}>
      <SearchPageContent />
    </Suspense>
  )
}
