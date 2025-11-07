'use client'

import { useState, useEffect } from 'react'
import { useRouter, useSearchParams } from 'next/navigation'
import { Filter, X } from 'lucide-react'
import { Button } from '@/components/ui/button'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { Checkbox } from '@/components/ui/checkbox'
import { Label } from '@/components/ui/label'
import { Badge } from '@/components/ui/badge'
import type { WorkspaceMember } from '@/lib/types'

interface AssigneeFilterProps {
  members: WorkspaceMember[]
  currentUserEmail?: string
}

export function AssigneeFilter({ members, currentUserEmail }: AssigneeFilterProps) {
  const router = useRouter()
  const searchParams = useSearchParams()
  const [open, setOpen] = useState(false)
  const [selectedFilters, setSelectedFilters] = useState<string[]>([])

  // Load filters from URL on mount
  useEffect(() => {
    const assigneeParam = searchParams.get('assignee')
    if (assigneeParam) {
      setSelectedFilters(assigneeParam.split(','))
    }
  }, [searchParams])

  const updateURL = (filters: string[]) => {
    const params = new URLSearchParams(searchParams.toString())
    if (filters.length > 0) {
      params.set('assignee', filters.join(','))
    } else {
      params.delete('assignee')
    }
    router.push(`?${params.toString()}`)
  }

  const toggleFilter = (value: string) => {
    const newFilters = selectedFilters.includes(value)
      ? selectedFilters.filter((f) => f !== value)
      : [...selectedFilters, value]

    setSelectedFilters(newFilters)
    updateURL(newFilters)
  }

  const clearFilters = () => {
    setSelectedFilters([])
    updateURL([])
  }

  const isSelected = (value: string) => selectedFilters.includes(value)

  return (
    <div className="flex items-center gap-2">
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverTrigger asChild>
          <Button variant="outline" size="sm" className="h-8">
            <Filter className="h-4 w-4 mr-2" />
            Assignee
            {selectedFilters.length > 0 && (
              <Badge variant="secondary" className="ml-2">
                {selectedFilters.length}
              </Badge>
            )}
          </Button>
        </PopoverTrigger>
        <PopoverContent className="w-[250px]" align="start">
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <h4 className="text-sm font-medium">Filter by assignee</h4>
              {selectedFilters.length > 0 && (
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={clearFilters}
                  className="h-auto p-0 text-xs"
                >
                  Clear
                </Button>
              )}
            </div>

            <div className="space-y-2">
              {currentUserEmail && (
                <div className="flex items-center space-x-2">
                  <Checkbox
                    id="filter-me"
                    checked={isSelected('me')}
                    onCheckedChange={() => toggleFilter('me')}
                  />
                  <Label htmlFor="filter-me" className="text-sm cursor-pointer">
                    Assigned to me
                  </Label>
                </div>
              )}

              <div className="flex items-center space-x-2">
                <Checkbox
                  id="filter-unassigned"
                  checked={isSelected('unassigned')}
                  onCheckedChange={() => toggleFilter('unassigned')}
                />
                <Label htmlFor="filter-unassigned" className="text-sm cursor-pointer">
                  Unassigned
                </Label>
              </div>

              <div className="border-t pt-2 space-y-2">
                {members.map((member) => (
                  <div key={member.user_id} className="flex items-center space-x-2">
                    <Checkbox
                      id={`filter-${member.user_id}`}
                      checked={isSelected(member.email)}
                      onCheckedChange={() => toggleFilter(member.email)}
                    />
                    <Label
                      htmlFor={`filter-${member.user_id}`}
                      className="text-sm cursor-pointer truncate"
                    >
                      {member.email}
                    </Label>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </PopoverContent>
      </Popover>

      {selectedFilters.length > 0 && (
        <Button
          variant="ghost"
          size="sm"
          onClick={clearFilters}
          className="h-8 px-2"
        >
          <X className="h-4 w-4" />
        </Button>
      )}
    </div>
  )
}
