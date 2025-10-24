/**
 * Sprint utility functions for capacity planning and velocity calculations
 */

export interface Sprint {
  id: string
  project_id: string
  name: string
  goal?: string
  status: 'planning' | 'active' | 'completed'
  start_date?: string
  end_date?: string
  created_at: string
  updated_at: string
}

export interface SprintStats {
  total_tickets: number
  total_points: number
  completed_tickets: number
  completed_points: number
  in_progress_tickets: number
  in_progress_points: number
  todo_tickets: number
  todo_points: number
  completion_percentage: number
}

/**
 * Calculate team capacity for a sprint
 * @param teamMembers - Number of team members
 * @param sprintDays - Number of days in the sprint
 * @param pointsPerDay - Story points per person per day (default: 3)
 * @returns Total capacity in story points
 */
export function calculateCapacity(
  teamMembers: number,
  sprintDays: number,
  pointsPerDay: number = 3
): number {
  return teamMembers * sprintDays * pointsPerDay
}

/**
 * Calculate average velocity from completed sprints
 * @param completedSprints - Array of completed sprints with stats
 * @returns Average velocity (completed points per sprint)
 */
export function calculateVelocity(
  completedSprints: Array<{ completed_points: number }>
): number {
  if (completedSprints.length === 0) return 0

  const totalPoints = completedSprints.reduce(
    (sum, sprint) => sum + sprint.completed_points,
    0
  )

  return Math.round(totalPoints / completedSprints.length)
}

/**
 * Calculate sprint progress percentage
 * @param stats - Sprint statistics
 * @returns Progress percentage (0-100)
 */
export function calculateProgress(stats: SprintStats): number {
  if (stats.total_points === 0) return 0
  return Math.round((stats.completed_points / stats.total_points) * 100)
}

/**
 * Calculate days remaining in sprint
 * @param endDate - Sprint end date
 * @returns Number of days remaining (negative if overdue)
 */
export function calculateDaysRemaining(endDate: string): number {
  const end = new Date(endDate)
  const now = new Date()
  const diff = end.getTime() - now.getTime()
  return Math.ceil(diff / (1000 * 60 * 60 * 24))
}

/**
 * Format sprint date range
 * @param startDate - Sprint start date
 * @param endDate - Sprint end date
 * @returns Formatted date range string
 */
export function formatSprintDateRange(startDate: string, endDate: string): string {
  const start = new Date(startDate)
  const end = new Date(endDate)

  const formatOptions: Intl.DateTimeFormatOptions = {
    month: 'short',
    day: 'numeric'
  }

  return `${start.toLocaleDateString('en-US', formatOptions)} - ${end.toLocaleDateString('en-US', formatOptions)}`
}

/**
 * Get sprint status badge color
 * @param status - Sprint status
 * @returns Tailwind CSS color classes
 */
export function getSprintStatusColor(status: string): string {
  switch (status) {
    case 'planning':
      return 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200'
    case 'active':
      return 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200'
    case 'completed':
      return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
    default:
      return 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200'
  }
}
