'use client'

import { useState } from 'react'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { MarkdownPreview } from '@/components/ui/markdown-preview'
import { MarkdownHelpPopover } from '@/components/ui/markdown-help-popover'
import { Edit, Check, X } from 'lucide-react'

interface TicketDescriptionProps {
  description: string
  onUpdate?: (description: string) => void
}

export function TicketDescription({ description, onUpdate }: TicketDescriptionProps) {
  const [isEditing, setIsEditing] = useState(false)
  const [value, setValue] = useState(description)
  const [activeTab, setActiveTab] = useState<'write' | 'preview'>('write')

  const handleSave = () => {
    if (value.trim() && value !== description && onUpdate) {
      onUpdate(value.trim())
    }
    setIsEditing(false)
    setActiveTab('write')
  }

  const handleCancel = () => {
    setValue(description)
    setIsEditing(false)
    setActiveTab('write')
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
          <Tabs value={activeTab} onValueChange={(v) => setActiveTab(v as 'write' | 'preview')}>
            <div className="flex items-center justify-between">
              <TabsList className="grid w-full max-w-[400px] grid-cols-2">
                <TabsTrigger value="write">Write</TabsTrigger>
                <TabsTrigger value="preview">Preview</TabsTrigger>
              </TabsList>
              <MarkdownHelpPopover />
            </div>

            <TabsContent value="write" className="mt-4">
              <Textarea
                value={value}
                onChange={(e) => setValue(e.target.value)}
                className="min-h-[300px] font-mono text-sm"
                placeholder="Write your description in Markdown...

**Markdown Tips:**
- **Bold** with **text**
- *Italic* with *text*
- `Code` with backticks
- [Links](url)
- # Headings with #
- - Lists with -
- ```code blocks```"
                autoFocus
              />
            </TabsContent>

            <TabsContent value="preview" className="mt-4">
              <div className="min-h-[300px] p-4 bg-muted rounded-lg border border-border">
                <MarkdownPreview content={value} />
              </div>
            </TabsContent>
          </Tabs>

          <div className="flex gap-2 pt-2">
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
        <div className="p-3 md:p-4 bg-muted rounded-lg border border-border">
          <MarkdownPreview content={description} />
        </div>
      )}
    </div>
  )
}
