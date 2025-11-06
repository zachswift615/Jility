'use client'

import { useState, useEffect } from 'react'
import { formatSprintDateRange, getSprintStatusColor } from '@/lib/sprint-utils'

interface Sprint {
  id: string
  name: string
  goal?: string
  status: string
  start_date?: string
  end_date?: string
  created_at: string
}

interface VelocityData {
  sprint_name: string
  completed_points: number
}

interface SprintHistoryData {
  sprints: Sprint[]
  velocity_data: VelocityData[]
  average_velocity: number
}

export default function SprintHistoryPage() {
  const [historyData, setHistoryData] = useState<SprintHistoryData | null>(null)
  const [loading, setLoading] = useState(true)

  const projectId = '550e8400-e29b-41d4-a716-446655440000' // TODO: Get from context/params

  useEffect(() => {
    fetchSprintHistory()
  }, [])

  async function fetchSprintHistory() {
    try {
      const res = await fetch(`http://localhost:3001/api/projects/${projectId}/sprint-history`)
      if (res.ok) {
        const data = await res.json()
        setHistoryData(data)
      }
    } catch (error) {
      console.error('Error fetching sprint history:', error)
    } finally {
      setLoading(false)
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-gray-600 dark:text-gray-400">Loading...</div>
      </div>
    )
  }

  if (!historyData || historyData.sprints.length === 0) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center">
          <h1 className="text-2xl font-bold mb-4">No Sprint History</h1>
          <p className="text-gray-600 dark:text-gray-400">
            No sprints have been completed yet.
          </p>
        </div>
      </div>
    )
  }

  const maxVelocity = Math.max(...historyData.velocity_data.map(d => d.completed_points))

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-3xl font-bold mb-8">Sprint History</h1>

      {/* Velocity Trend Chart */}
      <div className="mb-12">
        <h2 className="text-2xl font-bold mb-4">Velocity Trend</h2>
        <div className="bg-white dark:bg-gray-800 rounded-lg p-6 border border-gray-200 dark:border-gray-700">
          <div className="mb-4">
            <span className="text-lg font-semibold text-gray-700 dark:text-gray-300">
              Average Velocity: {Math.round(historyData.average_velocity)} points/sprint
            </span>
          </div>

          {/* Simple bar chart */}
          <div className="space-y-3">
            {historyData.velocity_data.map((data, index) => (
              <div key={index}>
                <div className="flex items-center justify-between mb-1">
                  <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
                    {data.sprint_name}
                  </span>
                  <span className="text-sm text-gray-600 dark:text-gray-400">
                    {data.completed_points} pts
                  </span>
                </div>
                <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-6">
                  <div
                    className="h-6 rounded-full bg-blue-600 flex items-center justify-end px-2"
                    style={{ width: `${(data.completed_points / maxVelocity) * 100}%` }}
                  >
                    <span className="text-xs text-white font-medium">
                      {data.completed_points}
                    </span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Sprint List */}
      <div>
        <h2 className="text-2xl font-bold mb-4">Completed Sprints</h2>
        <div className="space-y-4">
          {historyData.sprints.map((sprint, index) => {
            const velocityData = historyData.velocity_data.find(
              v => v.sprint_name === sprint.name
            )

            return (
              <a
                key={sprint.id}
                href={`/sprint/${sprint.id}`}
                className="block bg-white dark:bg-gray-800 rounded-lg p-6 border border-gray-200 dark:border-gray-700 hover:border-blue-500 transition-colors"
              >
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <div className="flex items-center gap-3 mb-2">
                      <h3 className="text-xl font-bold">{sprint.name}</h3>
                      <span className={`px-2 py-1 rounded text-xs font-medium ${getSprintStatusColor(sprint.status)}`}>
                        {sprint.status}
                      </span>
                    </div>

                    {sprint.goal && (
                      <p className="text-gray-600 dark:text-gray-400 mb-3">
                        Goal: {sprint.goal}
                      </p>
                    )}

                    <div className="flex items-center gap-6 text-sm text-gray-600 dark:text-gray-400">
                      {sprint.start_date && sprint.end_date && (
                        <div>
                          Completed {formatSprintDateRange(sprint.start_date, sprint.end_date)}
                        </div>
                      )}
                      {velocityData && (
                        <div>
                          {velocityData.completed_points} points completed
                        </div>
                      )}
                    </div>
                  </div>

                  {velocityData && (
                    <div className="ml-6">
                      <div className="text-3xl font-bold text-blue-600 dark:text-blue-400">
                        {velocityData.completed_points}
                      </div>
                      <div className="text-sm text-gray-600 dark:text-gray-400">
                        points
                      </div>
                    </div>
                  )}
                </div>
              </a>
            )
          })}
        </div>
      </div>
    </div>
  )
}
