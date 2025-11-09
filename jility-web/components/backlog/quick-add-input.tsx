'use client'

import { useState, KeyboardEvent, useEffect, useRef } from 'react'
import { Plus } from 'lucide-react'

interface QuickAddInputProps {
  onAdd: (title: string) => void
  autoFocus?: boolean
}

export function QuickAddInput({ onAdd, autoFocus }: QuickAddInputProps) {
  const [value, setValue] = useState('')
  const inputRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    if (autoFocus && inputRef.current) {
      inputRef.current.focus()
    }
  }, [autoFocus])

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && value.trim()) {
      onAdd(value.trim())
      setValue('')
    }
  }

  return (
    <div className="px-4 py-3 bg-muted/50 cursor-text" onClick={() => inputRef.current?.focus()}>
      <div className="flex items-center gap-3">
        <Plus className="h-5 w-5 text-muted-foreground" />
        <input
          ref={inputRef}
          id="quick-add-input"
          type="text"
          value={value}
          onChange={(e) => setValue(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Quick add ticket (press Enter to create)..."
          className="flex-1 border-none bg-transparent text-sm outline-none placeholder:text-muted-foreground"
        />
      </div>
    </div>
  )
}
