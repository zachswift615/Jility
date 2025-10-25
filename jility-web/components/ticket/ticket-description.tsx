'use client'

import { useState } from 'react'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import rehypeHighlight from 'rehype-highlight'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import { Edit, Check, X } from 'lucide-react'
import 'highlight.js/styles/github-dark.css'

interface TicketDescriptionProps {
  description: string
  onUpdate?: (description: string) => void
}

export function TicketDescription({ description, onUpdate }: TicketDescriptionProps) {
  const [isEditing, setIsEditing] = useState(false)
  const [value, setValue] = useState(description)

  const handleSave = () => {
    if (value.trim() && value !== description && onUpdate) {
      onUpdate(value.trim())
    }
    setIsEditing(false)
  }

  const handleCancel = () => {
    setValue(description)
    setIsEditing(false)
  }

  return (
    <div className="mt-4 md:mt-6 space-y-3 md:space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-base md:text-lg font-semibold">Description</h2>
        {!isEditing && onUpdate && (
          <Button
            variant="ghost"
            size="sm"
            onClick={() => setIsEditing(true)}
          >
            <Edit className="h-4 w-4 md:mr-2" />
            <span className="hidden md:inline">Edit</span>
          </Button>
        )}
      </div>

      {isEditing ? (
        <div className="space-y-2">
          <Textarea
            value={value}
            onChange={(e) => setValue(e.target.value)}
            className="min-h-[200px] font-mono text-sm"
            placeholder="Enter description (Markdown supported)..."
          />
          <div className="flex gap-2">
            <Button size="sm" onClick={handleSave}>
              <Check className="h-4 w-4 mr-2" />
              Save
            </Button>
            <Button size="sm" variant="outline" onClick={handleCancel}>
              <X className="h-4 w-4 mr-2" />
              Cancel
            </Button>
          </div>
        </div>
      ) : (
        <div className="prose prose-sm md:prose max-w-none dark:prose-invert p-3 md:p-4 bg-muted rounded-lg">
          <ReactMarkdown
            remarkPlugins={[remarkGfm]}
            rehypePlugins={[rehypeHighlight]}
            className="text-foreground"
          >
            {description || '*No description provided*'}
          </ReactMarkdown>
        </div>
      )}
    </div>
  )
}
