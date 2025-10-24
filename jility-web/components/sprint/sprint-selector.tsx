'use client'

import { useState, useEffect } from 'react'

interface Sprint {
  id: string
  name: string
  status: string
  start_date?: string
  end_date?: string
}

interface SprintSelectorProps {
  projectId: string
  onSprintChange?: (sprintId: string | null) => void
  className?: string
}

export function SprintSelector({ projectId, onSprintChange, className = '' }: SprintSelectorProps) {
  const [sprints, setSprints] = useState<Sprint[]>([])
  const [selectedSprint, setSelectedSprint] = useState<string | null>(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    fetchSprints()
  }, [projectId])

  async function fetchSprints() {
    try {
      const res = await fetch(`http://localhost:3001/api/projects/${projectId}/sprints`)
      if (res.ok) {
        const data = await res.json()
        setSprints(data)

        // Auto-select active sprint
        const activeSprint = data.find((s: Sprint) => s.status === 'active')
        if (activeSprint) {
          setSelectedSprint(activeSprint.id)
          onSprintChange?.(activeSprint.id)
        }
      }
    } catch (error) {
      console.error('Failed to fetch sprints:', error)
    } finally {
      setLoading(false)
    }
  }

  function handleSprintChange(sprintId: string) {
    const id = sprintId === 'all' ? null : sprintId
    setSelectedSprint(id)
    onSprintChange?.(id)
  }

  if (loading) {
    return (
      <div className={`animate-pulse bg-gray-200 dark:bg-gray-700 h-10 rounded ${className}`} />
    )
  }

  return (
    <select
      value={selectedSprint || 'all'}
      onChange={(e) => handleSprintChange(e.target.value)}
      className={`px-3 py-2 rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500 ${className}`}
    >
      <option value="all">All Sprints</option>
      {sprints.map(sprint => (
        <option key={sprint.id} value={sprint.id}>
          {sprint.name} ({sprint.status})
        </option>
      ))}
    </select>
  )
}
