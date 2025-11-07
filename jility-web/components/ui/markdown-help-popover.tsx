'use client'

import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { Button } from '@/components/ui/button'
import { HelpCircle } from 'lucide-react'

export function MarkdownHelpPopover() {
  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button variant="ghost" size="sm">
          <HelpCircle className="h-4 w-4 mr-2" />
          Markdown Help
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-80">
        <div className="space-y-3">
          <h4 className="font-semibold">Markdown Formatting</h4>
          <div className="text-sm space-y-2">
            <div>
              <code className="text-xs bg-muted px-1 py-0.5 rounded">
                **bold**
              </code>
              <span className="ml-2">for <strong>bold text</strong></span>
            </div>
            <div>
              <code className="text-xs bg-muted px-1 py-0.5 rounded">
                *italic*
              </code>
              <span className="ml-2">for <em>italic text</em></span>
            </div>
            <div>
              <code className="text-xs bg-muted px-1 py-0.5 rounded">
                `code`
              </code>
              <span className="ml-2">for <code>inline code</code></span>
            </div>
            <div>
              <code className="text-xs bg-muted px-1 py-0.5 rounded">
                # Heading
              </code>
              <span className="ml-2">for headings</span>
            </div>
            <div>
              <code className="text-xs bg-muted px-1 py-0.5 rounded">
                [link](url)
              </code>
              <span className="ml-2">for links</span>
            </div>
            <div>
              <code className="text-xs bg-muted px-1 py-0.5 rounded">
                - item
              </code>
              <span className="ml-2">for bullet lists</span>
            </div>
            <div>
              <code className="text-xs bg-muted px-1 py-0.5 rounded">
                ```code```
              </code>
              <span className="ml-2">for code blocks</span>
            </div>
          </div>
        </div>
      </PopoverContent>
    </Popover>
  )
}
