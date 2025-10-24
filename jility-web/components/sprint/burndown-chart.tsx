'use client'

import { useMemo } from 'react'

export interface BurndownDataPoint {
  date: string
  ideal: number
  actual: number
}

export interface BurndownData {
  sprint_id: string
  data_points: BurndownDataPoint[]
}

interface BurndownChartProps {
  data: BurndownData
}

export function BurndownChart({ data }: BurndownChartProps) {
  const { maxPoints, points } = useMemo(() => {
    const max = Math.max(
      ...data.data_points.map(d => Math.max(d.ideal, d.actual))
    )
    return {
      maxPoints: max,
      points: data.data_points
    }
  }, [data])

  const width = 600
  const height = 300
  const padding = { top: 20, right: 20, bottom: 40, left: 50 }
  const chartWidth = width - padding.left - padding.right
  const chartHeight = height - padding.top - padding.bottom

  const xScale = (index: number) =>
    padding.left + (index / Math.max(points.length - 1, 1)) * chartWidth

  const yScale = (value: number) =>
    padding.top + chartHeight - (value / maxPoints) * chartHeight

  const idealPath = points
    .map((d, i) => `${i === 0 ? 'M' : 'L'} ${xScale(i)} ${yScale(d.ideal)}`)
    .join(' ')

  const actualPath = points
    .map((d, i) => `${i === 0 ? 'M' : 'L'} ${xScale(i)} ${yScale(d.actual)}`)
    .join(' ')

  return (
    <div className="w-full overflow-x-auto">
      <svg
        width={width}
        height={height}
        className="text-gray-700 dark:text-gray-300"
      >
        {/* Grid lines */}
        {[0, 1, 2, 3, 4].map(i => {
          const y = padding.top + (i * chartHeight) / 4
          return (
            <line
              key={i}
              x1={padding.left}
              y1={y}
              x2={width - padding.right}
              y2={y}
              stroke="currentColor"
              strokeOpacity={0.1}
              strokeDasharray="2,2"
            />
          )
        })}

        {/* Y-axis labels */}
        {[0, 1, 2, 3, 4].map(i => {
          const value = Math.round(maxPoints - (i * maxPoints) / 4)
          const y = padding.top + (i * chartHeight) / 4
          return (
            <text
              key={i}
              x={padding.left - 10}
              y={y + 4}
              textAnchor="end"
              fontSize="12"
              fill="currentColor"
              opacity={0.6}
            >
              {value}
            </text>
          )
        })}

        {/* X-axis labels */}
        {points.map((d, i) => {
          if (i % Math.ceil(points.length / 7) !== 0 && i !== points.length - 1) {
            return null
          }
          const date = new Date(d.date)
          const label = `${date.getMonth() + 1}/${date.getDate()}`
          return (
            <text
              key={i}
              x={xScale(i)}
              y={height - padding.bottom + 20}
              textAnchor="middle"
              fontSize="12"
              fill="currentColor"
              opacity={0.6}
            >
              {label}
            </text>
          )
        })}

        {/* Ideal burndown line (dashed) */}
        <path
          d={idealPath}
          fill="none"
          stroke="#94a3b8"
          strokeWidth={2}
          strokeDasharray="5,5"
        />

        {/* Actual burndown line */}
        <path
          d={actualPath}
          fill="none"
          stroke="#3b82f6"
          strokeWidth={3}
        />

        {/* Actual data points */}
        {points.map((d, i) => (
          <circle
            key={i}
            cx={xScale(i)}
            cy={yScale(d.actual)}
            r={4}
            fill="#3b82f6"
            stroke="#ffffff"
            strokeWidth={2}
          />
        ))}

        {/* Axis lines */}
        <line
          x1={padding.left}
          y1={padding.top}
          x2={padding.left}
          y2={height - padding.bottom}
          stroke="currentColor"
          strokeOpacity={0.2}
        />
        <line
          x1={padding.left}
          y1={height - padding.bottom}
          x2={width - padding.right}
          y2={height - padding.bottom}
          stroke="currentColor"
          strokeOpacity={0.2}
        />
      </svg>

      {/* Legend */}
      <div className="flex items-center justify-center gap-6 mt-4">
        <div className="flex items-center gap-2">
          <div className="w-8 h-0.5 bg-gray-400" style={{ borderTop: '2px dashed #94a3b8' }} />
          <span className="text-sm text-gray-600 dark:text-gray-400">Ideal</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-8 h-0.5 bg-blue-500" />
          <span className="text-sm text-gray-600 dark:text-gray-400">Actual</span>
        </div>
      </div>
    </div>
  )
}
