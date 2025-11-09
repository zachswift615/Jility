'use client'

import { useSearchParams, useRouter } from 'next/navigation'
import { useWorkspace } from '@/lib/workspace-context'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import type { Epic } from '@/lib/types'

interface EpicFilterProps {
  epics: Epic[]
}

export function EpicFilter({ epics }: EpicFilterProps) {
  const searchParams = useSearchParams()
  const router = useRouter()
  const { currentWorkspace } = useWorkspace()
  const slug = currentWorkspace?.slug || ''

  const selectedEpicId = searchParams.get('epic')

  const handleValueChange = (value: string) => {
    const params = new URLSearchParams(searchParams.toString())

    if (value === 'all') {
      params.delete('epic')
    } else {
      params.set('epic', value)
    }

    router.push(`/w/${slug}/board?${params.toString()}`)
  }

  if (epics.length === 0) {
    return null
  }

  return (
    <Select value={selectedEpicId || 'all'} onValueChange={handleValueChange}>
      <SelectTrigger className="w-[200px]">
        <SelectValue placeholder="All tickets" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="all">All tickets</SelectItem>
        <SelectItem value="none">No epic</SelectItem>
        {epics.map((epic) => (
          <SelectItem key={epic.id} value={epic.id}>
            {epic.title}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  )
}
