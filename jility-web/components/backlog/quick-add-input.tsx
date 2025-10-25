'use client'

import { useState, KeyboardEvent } from 'react'
import { Plus } from 'lucide-react'

interface QuickAddInputProps {
  onAdd: (title: string) => void
}

export function QuickAddInput({ onAdd }: QuickAddInputProps) {
  const [value, setValue] = useState('')

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && value.trim()) {
      onAdd(value.trim())
      setValue('')
    }
  }

  return (
    <div className="px-4 py-3 bg-muted/50 cursor-text" onClick={() => document.getElementById('quick-add-input')?.focus()}>
      <div className="flex items-center gap-3">
        <Plus className="h-5 w-5 text-muted-foreground" />
        <input
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
