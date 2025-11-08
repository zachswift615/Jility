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

  // Load capacity from localStorage or calculate default
  useEffect(() => {
    // Track if component is still mounted to prevent state updates after unmount
    let mounted = true

    // Calculate default capacity from team velocity
    const calculateDefaultCapacity = async (): Promise<number> => {
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
    }

    const loadCapacity = async () => {
      if (!slug) {
        if (mounted) setLoading(false)
        return
      }

      // Try to get from localStorage first
      const storageKey = `sprint-capacity-${slug}`
      const stored = localStorage.getItem(storageKey)

      if (stored) {
        const parsed = parseInt(stored, 10)
        // Validate parsed value to ensure it's a valid number
        if (!isNaN(parsed) && parsed > 0) {
          if (mounted) setCapacity(parsed)
        } else {
          // Invalid stored value, calculate default
          const defaultCapacity = await calculateDefaultCapacity()
          if (mounted) setCapacity(defaultCapacity)
        }
      } else {
        // Calculate default from team velocity
        const defaultCapacity = await calculateDefaultCapacity()
        if (mounted) setCapacity(defaultCapacity)
      }

      if (mounted) setLoading(false)
    }

    loadCapacity()

    // Cleanup function to prevent state updates after unmount
    return () => {
      mounted = false
    }
  }, [slug])

  // Update capacity
  const updateCapacity = useCallback(
    async (newCapacity: number) => {
      if (!slug) return

      // TODO: Replace with API call when backend is ready
      // await api.updateWorkspaceSettings(slug, { sprint_capacity: newCapacity })

      // For now, use localStorage with error handling
      const storageKey = `sprint-capacity-${slug}`
      try {
        localStorage.setItem(storageKey, newCapacity.toString())
        setCapacity(newCapacity)
      } catch (error) {
        console.error('Failed to save capacity to localStorage:', error)
        // Still update state even if localStorage fails
        setCapacity(newCapacity)
      }
    },
    [slug]
  )

  return {
    capacity,
    updateCapacity,
    loading,
  }
}
