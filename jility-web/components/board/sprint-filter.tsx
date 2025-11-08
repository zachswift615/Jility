'use client'

import { useSearchParams, useRouter } from 'next/navigation'
import { useWorkspace } from '@/lib/workspace-context'

interface SprintFilterProps {
  hasActiveSprint: boolean
}

export function SprintFilter({ hasActiveSprint }: SprintFilterProps) {
  const searchParams = useSearchParams()
  const router = useRouter()
  const { currentWorkspace } = useWorkspace()
  const slug = currentWorkspace?.slug || ''

  const showActiveSprint = searchParams.get('sprint') === 'active'

  const toggleFilter = () => {
    const params = new URLSearchParams(searchParams.toString())

    if (showActiveSprint) {
      params.delete('sprint')
    } else {
      params.set('sprint', 'active')
    }

    router.push(`/w/${slug}/board?${params.toString()}`)
  }

  if (!hasActiveSprint) {
    return null
  }

  return (
    <button
      onClick={toggleFilter}
      className={`px-3 py-1.5 rounded-md text-sm font-medium transition-colors ${
        showActiveSprint
          ? 'bg-primary text-primary-foreground'
          : 'bg-secondary text-secondary-foreground hover:bg-secondary/80'
      }`}
    >
      {showActiveSprint ? 'Active Sprint' : 'All Tickets'}
    </button>
  )
}
