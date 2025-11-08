'use client'

import { useState, useEffect, useCallback } from 'react'
import { useWorkspace } from './workspace-context'
import { api } from './api'

/**
 * Hook for managing sprint capacity setting.
 *
 * Currently uses localStorage as a temporary solution.
 * TODO: Replace with backend API calls when workspace settings endpoint is ready.
 */
export function useSprintCapacity() {
  const { currentWorkspace } = useWorkspace()
  const [capacity, setCapacity] = useState<number | null>(null)
  const [loading, setLoading] = useState(true)

  const slug = currentWorkspace?.slug || ''

  // Calculate default capacity from team velocity
  const calculateDefaultCapacity = useCallback(async (): Promise<number> => {
    if (!slug) return 40

    try {
      const history = await api.getSprintHistory(slug)
      if (history.average_velocity > 0) {
        // Use average velocity as default capacity
        return Math.round(history.average_velocity)
      }
    } catch (error) {
      console.error('Failed to fetch sprint history:', error)
    }

    // Default to 40 points if no history
    return 40
  }, [slug])

  // Load capacity from localStorage or calculate default
  useEffect(() => {
    const loadCapacity = async () => {
      if (!slug) {
        setLoading(false)
        return
      }

      // Try to get from localStorage first
      const storageKey = `sprint-capacity-${slug}`
      const stored = localStorage.getItem(storageKey)

      if (stored) {
        setCapacity(parseInt(stored, 10))
      } else {
        // Calculate default from team velocity
        const defaultCapacity = await calculateDefaultCapacity()
        setCapacity(defaultCapacity)
      }

      setLoading(false)
    }

    loadCapacity()
  }, [slug, calculateDefaultCapacity])

  // Update capacity
  const updateCapacity = useCallback(
    async (newCapacity: number) => {
      if (!slug) return

      // TODO: Replace with API call when backend is ready
      // await api.updateWorkspaceSettings(slug, { sprint_capacity: newCapacity })

      // For now, use localStorage
      const storageKey = `sprint-capacity-${slug}`
      localStorage.setItem(storageKey, newCapacity.toString())
      setCapacity(newCapacity)
    },
    [slug]
  )

  return {
    capacity,
    updateCapacity,
    loading,
  }
}
