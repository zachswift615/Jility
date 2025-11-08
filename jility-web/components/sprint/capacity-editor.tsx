'use client'

import { useState } from 'react'
import { Pencil, Check, X } from 'lucide-react'

interface CapacityEditorProps {
  capacity: number
  onSave: (newCapacity: number) => Promise<void>
}

export function CapacityEditor({ capacity, onSave }: CapacityEditorProps) {
  const [editing, setEditing] = useState(false)
  const [value, setValue] = useState(capacity.toString())
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState('')

  const handleSave = async () => {
    const parsed = parseInt(value, 10)

    if (isNaN(parsed) || parsed < 1) {
      setError('Capacity must be a positive number')
      return
    }

    setSaving(true)
    setError('')

    try {
      await onSave(parsed)
      setEditing(false)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to save')
    } finally {
      setSaving(false)
    }
  }

  const handleCancel = () => {
    setValue(capacity.toString())
    setError('')
    setEditing(false)
  }

  if (!editing) {
    return (
      <button
        onClick={() => setEditing(true)}
        className="inline-flex items-center gap-1 text-sm font-medium hover:text-primary transition-colors"
        title="Edit capacity"
      >
        <span>{capacity} pts</span>
        <Pencil className="h-3 w-3" />
      </button>
    )
  }

  return (
    <div className="inline-flex items-center gap-2">
      <div className="flex flex-col">
        <div className="flex items-center gap-1">
          <input
            type="number"
            value={value}
            onChange={(e) => setValue(e.target.value)}
            className="w-20 px-2 py-1 text-sm bg-background border-input border rounded"
            min="1"
            autoFocus
            onKeyDown={(e) => {
              if (e.key === 'Enter') handleSave()
              if (e.key === 'Escape') handleCancel()
            }}
          />
          <span className="text-sm text-muted-foreground">pts</span>
          <button
            onClick={handleSave}
            disabled={saving}
            className="p-1 text-green-600 hover:bg-green-100 dark:hover:bg-green-900 rounded disabled:opacity-50"
            title="Save"
          >
            <Check className="h-4 w-4" />
          </button>
          <button
            onClick={handleCancel}
            disabled={saving}
            className="p-1 text-destructive hover:bg-destructive/10 rounded disabled:opacity-50"
            title="Cancel"
          >
            <X className="h-4 w-4" />
          </button>
        </div>
        {error && (
          <span className="text-xs text-destructive mt-1">{error}</span>
        )}
      </div>
    </div>
  )
}
